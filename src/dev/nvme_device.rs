use crate::dev::disk::open;
use crate::dev::nvme_structs::*;
use std::ffi::c_void;
use crate::dev::nvme_structs::ProtocolTypeNvme;
use std::io;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::IO::DeviceIoControl;
use windows_sys::Win32::System::Ioctl::*;

pub struct NvmeDevice {
    handle: HANDLE,
}

impl NvmeDevice {
    pub fn open(device_path: &str) -> io::Result<Self> {
        let handle = open(device_path, 'w');

        if handle == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self { handle })
        }
    }

    pub fn get_handle(&self) -> HANDLE {
        self.handle
    }

    fn send_vendor_specific_command(&self, buffer: &mut [u8]) -> Result<(), std::io::Error> {
        let protocol_command =
            unsafe { &mut *(buffer.as_mut_ptr() as *mut StorageProtocolCommand) };
        protocol_command.version = STORAGE_PROTOCOL_STRUCTURE_VERSION;
        protocol_command.length = std::mem::size_of::<StorageProtocolCommand>() as u32;
        protocol_command.protocol_type = ProtocolTypeNvme;
        protocol_command.flags = STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST;
        protocol_command.command_length = STORAGE_PROTOCOL_COMMAND_LENGTH_NVME;
        protocol_command.error_info_length = std::mem::size_of::<NVME_ERROR_INFO_LOG>() as u32;
        protocol_command.data_from_device_transfer_length = 4096;
        protocol_command.timeout_value = 10;
        protocol_command.error_info_offset = std::mem::size_of::<StorageProtocolCommand>() as u32;
        protocol_command.data_from_device_buffer_offset =
            protocol_command.error_info_offset + protocol_command.error_info_length;
        protocol_command.command_specific = STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND;

        let command = unsafe { &mut *(protocol_command.command.as_mut_ptr() as *mut NvmeCommand) };
        command.opcode = 0xFF;
        command.cdw10 = 0; // to_fill_in
        command.cdw12 = 0; // to_fill_in
        command.cdw13 = 0; // to_fill_in

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
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn send_command(&self, protocol_command: &mut StorageProtocolCommand, nvme_command: &NvmeCommand) -> io::Result<()> {
        let mut buffer = vec![0u8;
            protocol_command.length as usize + protocol_command.command_length as usize
        ];

        let command_offset = protocol_command.data_to_device_buffer_offset as usize;
        buffer[command_offset..command_offset + std::mem::size_of::<NvmeCommand>()]
            .copy_from_slice(unsafe {
                std::slice::from_raw_parts(
                    nvme_command as *const _ as *const u8,
                    std::mem::size_of::<NvmeCommand>(),
                )
            });

        let mut bytes_returned: u32 = 0;
        let success = unsafe {
            DeviceIoControl(
                self.handle,
                IOCTL_STORAGE_PROTOCOL_COMMAND,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                &mut bytes_returned,
                null_mut(),
            )
        };

        if success == 0 {
            Err(io::Error::last_os_error())
        } else {
            println!("Command executed successfully. Output: {:?}", &buffer[..bytes_returned as usize]);
            Ok(())
        }
    }

    pub fn send_protocol_command(&self, protocol_descriptor: &mut StorageProtocolDataDescriptor, output_buffer: &mut [u8]) -> io::Result<()> {
        let mut bytes_returned: u32 = 0;
        let success = unsafe {
            DeviceIoControl(
                self.handle,
                IOCTL_STORAGE_QUERY_PROPERTY,
                protocol_descriptor as *mut _ as *mut c_void,
                std::mem::size_of::<StorageProtocolDataDescriptor>() as u32,
                output_buffer.as_mut_ptr() as *mut c_void,
                output_buffer.len() as u32,
                &mut bytes_returned,
                null_mut(),
            )
        };

        if success == 0 {
            Err(io::Error::last_os_error())
        } else {
            println!("Command executed successfully. Output: {:?}", &output_buffer[..bytes_returned as usize]);
            Ok(())
        }
    }
}

impl Drop for NvmeDevice {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}