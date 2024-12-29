use crate::dev::disk::open;
use crate::dev::nvme_define::*;
use std::mem::offset_of;
use std::{ffi::c_void, io, mem::size_of, ptr::null_mut};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Ioctl::*;
use windows_sys::Win32::System::IO::DeviceIoControl;

// To use FIELD_OFFSET macro equivalent in Rust:
// let offset = field_offset::<SomeType, SomeFieldType>(0 as *const SomeType, |s| &s.some_field);
pub struct NvmeDevice {
    handle: HANDLE,
    pub project_type: String, // Example field, adjust as necessary
}

impl NvmeDevice {
    pub fn open(device_path: &str) -> io::Result<Self> {
        let handle = open(device_path, 'w');
        if handle == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self {
                handle,
                project_type: String::new(),
            })
        }
    }

    pub fn get_handle(&self) -> HANDLE {
        self.handle
    }

    pub fn nvme_send_passthrough_command(
        &self,
        direction: u8,
        nvme_command: &NVME_COMMAND,
        data_buffer: &mut [u8],
        return_dw0: &mut u32,
    ) -> io::Result<NVME_COMMAND_STATUS> {
        let command_offset = offset_of!(STORAGE_PROTOCOL_COMMAND, Command);
        let mut buffer: Vec<u8> =
            vec![0; command_offset + size_of::<NVME_COMMAND>() + data_buffer.len()];
        let protocol_command =
            unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROTOCOL_COMMAND) };
        protocol_command.Version = STORAGE_PROTOCOL_STRUCTURE_VERSION;
        protocol_command.Length = size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
        protocol_command.ProtocolType = ProtocolTypeNvme as i32;
        protocol_command.Flags = STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST;
        protocol_command.CommandLength = STORAGE_PROTOCOL_COMMAND_LENGTH_NVME;
        protocol_command.TimeOutValue = 30;

        // protocol_command.ErrorInfoLength = size_of::<NVME_ERROR_INFO_LOG>() as u32;
        protocol_command.ErrorInfoOffset =
            command_offset as u32 + STORAGE_PROTOCOL_COMMAND_LENGTH_NVME;

        match direction {
            1 => {
                protocol_command.DataToDeviceTransferLength = data_buffer.len() as u32;
            }
            2 => {
                protocol_command.DataFromDeviceTransferLength = data_buffer.len() as u32;
            }
            _ => {}
        }
        protocol_command.DataToDeviceBufferOffset =
            protocol_command.ErrorInfoOffset + protocol_command.ErrorInfoLength;
        protocol_command.DataFromDeviceBufferOffset =
            protocol_command.DataToDeviceBufferOffset + protocol_command.DataToDeviceTransferLength;
        protocol_command.CommandSpecific = STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND;

        buffer[command_offset..command_offset + size_of::<NVME_COMMAND>()].copy_from_slice(
            unsafe {
                std::slice::from_raw_parts(
                    nvme_command as *const _ as *const u8,
                    size_of::<NVME_COMMAND>(),
                )
            },
        );
        if direction == 1 && data_buffer.len() > 0 {
            let data_offset = protocol_command.DataToDeviceBufferOffset as usize;
            buffer[data_offset..data_offset + data_buffer.len()].copy_from_slice(data_buffer);
        }

        let mut returned_length = 0;
        let result = unsafe {
            DeviceIoControl(
                self.handle,
                IOCTL_STORAGE_PROTOCOL_COMMAND,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                &mut returned_length,
                null_mut(),
            )
        };

        if result == 0 {
            println!("Error: {:?}", io::Error::last_os_error());
            Err(io::Error::last_os_error())
        } else {
            if direction == 2 && data_buffer.len() > 0 {
                let data_offset = protocol_command.DataFromDeviceBufferOffset as usize;
                data_buffer.copy_from_slice(&buffer[data_offset..data_offset + data_buffer.len()]);
            }
            let ncs = NVME_COMMAND_STATUS::from(protocol_command.ErrorCode as u16);
            Ok(ncs)
        }
    }

    pub fn query(
        &self,
        protocol_data_descr: &mut STORAGE_PROTOCOL_DATA_DESCRIPTOR,
        output_buffer: &mut [u8],
    ) -> io::Result<()> {
        let mut bytes_returned: u32 = 0;
        let success = unsafe {
            DeviceIoControl(
                self.handle,
                IOCTL_STORAGE_QUERY_PROPERTY,
                protocol_data_descr as *mut _ as *mut c_void,
                size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32,
                output_buffer.as_mut_ptr() as *mut c_void,
                output_buffer.len() as u32,
                &mut bytes_returned,
                null_mut(),
            )
        };

        if success == 0 {
            Err(io::Error::last_os_error())
        } else {
            println!(
                "Command executed successfully. Output: {:?}",
                &output_buffer[..bytes_returned as usize]
            );
            Ok(())
        }
    }
}

impl Drop for NvmeDevice {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}
