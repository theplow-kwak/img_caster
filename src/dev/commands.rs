use crate::dev::disk::open;
use crate::dev::nvme_structs::*;
use std::{ffi::c_void, io, mem::size_of, ptr::null_mut};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Ioctl::*;
use windows_sys::Win32::System::IO::DeviceIoControl;

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

    pub fn pass_through(
        &self,
        protocol_command: &mut STORAGE_PROTOCOL_COMMAND,
        nvme_command: &NvmeCommand,
    ) -> io::Result<()> {
        let mut buffer =
            vec![0u8; protocol_command.Length as usize + protocol_command.CommandLength as usize];

        let command_offset = protocol_command.DataToDeviceBufferOffset as usize;
        buffer[command_offset..command_offset + size_of::<NvmeCommand>()].copy_from_slice(unsafe {
            std::slice::from_raw_parts(
                nvme_command as *const _ as *const u8,
                size_of::<NvmeCommand>(),
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
            println!(
                "Command executed successfully. Output: {:?}",
                &buffer[..bytes_returned as usize]
            );
            Ok(())
        }
    }

    pub fn query(
        &self,
        protocol_descriptor: &mut STORAGE_PROTOCOL_DATA_DESCRIPTOR,
        output_buffer: &mut [u8],
    ) -> io::Result<()> {
        let mut bytes_returned: u32 = 0;
        let success = unsafe {
            DeviceIoControl(
                self.handle,
                IOCTL_STORAGE_QUERY_PROPERTY,
                protocol_descriptor as *mut _ as *mut c_void,
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

fn _send_nvme_identify_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_command = STORAGE_PROTOCOL_COMMAND {
        DataFromDeviceBufferOffset: std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32
            + std::mem::size_of::<NvmeCommand>() as u32,
        TimeOutValue: 5000,
        ..STORAGE_PROTOCOL_COMMAND { .._ } // Initialize remaining fields with "don't care" values
    };
    protocol_command.Version = 0;
    protocol_command.Length = std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
    protocol_command.ProtocolType = ProtocolTypeNvme; // ProtocolTypeNvme
    protocol_command.Flags = 0;
    protocol_command.CommandLength = std::mem::size_of::<NvmeCommand>() as u32;
    protocol_command.DataToDeviceBufferOffset =
        std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
    protocol_command.DataFromDeviceTransferLength = 4096; // Identify command returns 4096 bytes

    let identify_command = NvmeCommand {
        opcode: 0x06, // Identify opcode
        nsid: 0,      // Identify Controller
        cdw10: 1,     // CNS = 1 for Identify Controller
        ..Default::default()
    };

    nvme_device.pass_through(&mut protocol_command, &identify_command)
}

fn send_nvme_set_features_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_command = STORAGE_PROTOCOL_COMMAND {
        DataFromDeviceBufferOffset: std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32
            + std::mem::size_of::<NvmeCommand>() as u32,
        TimeOutValue: 5000,
        ..STORAGE_PROTOCOL_COMMAND { .._ } // Initialize remaining fields with "don't care" values
    };
    protocol_command.Version = 0;
    protocol_command.Length = std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
    protocol_command.ProtocolType = ProtocolTypeNvme; // ProtocolTypeNvme
    protocol_command.Flags = 0;
    protocol_command.CommandLength = std::mem::size_of::<NvmeCommand>() as u32;
    protocol_command.DataToDeviceBufferOffset =
        std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
    protocol_command.DataFromDeviceTransferLength = 0;

    let set_features_command = NvmeCommand {
        opcode: 0x09,  // Set Features opcode
        nsid: 0,       // NSID 0 for global features
        cdw10: 0x01,   // Feature Identifier: Arbitrary example
        cdw11: 0xABCD, // Feature Value: Example value
        ..Default::default()
    };

    nvme_device.pass_through(&mut protocol_command, &set_features_command)
}

fn send_vendor_specific_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_command = STORAGE_PROTOCOL_COMMAND {
        DataFromDeviceBufferOffset: std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32
            + std::mem::size_of::<NvmeCommand>() as u32,
        TimeOutValue: 5000,
        ..STORAGE_PROTOCOL_COMMAND { .._ } // Initialize remaining fields with "don't care" values
    };
    protocol_command.Version = 0;
    protocol_command.Length = std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
    protocol_command.ProtocolType = ProtocolTypeNvme; // ProtocolTypeNvme
    protocol_command.Flags = 0;
    protocol_command.CommandLength = std::mem::size_of::<NvmeCommand>() as u32;
    protocol_command.DataToDeviceBufferOffset =
        std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
    protocol_command.DataFromDeviceTransferLength = 0;

    let vendor_specific_command = NvmeCommand {
        opcode: 0xC0,  // Vendor-specific opcode
        nsid: 0,       // Adjust NSID as required
        cdw10: 0x1234, // Vendor-specific data
        cdw11: 0x5678, // Additional vendor-specific data
        ..Default::default()
    };

    nvme_device.pass_through(&mut protocol_command, &vendor_specific_command)
}

fn send_nvme_get_features_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_command = STORAGE_PROTOCOL_COMMAND {
        DataFromDeviceBufferOffset: std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32
            + std::mem::size_of::<NvmeCommand>() as u32,
        TimeOutValue: 5000,
        ..STORAGE_PROTOCOL_COMMAND { .._ } // Initialize remaining fields with "don't care" values
    };
    protocol_command.Version = 0;
    protocol_command.Length = std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
    protocol_command.ProtocolType = ProtocolTypeNvme; // ProtocolTypeNvme
    protocol_command.Flags = 0;
    protocol_command.CommandLength = std::mem::size_of::<NvmeCommand>() as u32;
    protocol_command.DataToDeviceBufferOffset =
        std::mem::size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
    protocol_command.DataFromDeviceTransferLength = 4096; // Feature data size

    let get_features_command = NvmeCommand {
        opcode: 0x0A, // Get Features opcode
        nsid: 0,      // NSID 0 for global features
        cdw10: 0x01,  // Feature Identifier: Example identifier
        ..Default::default()
    };

    nvme_device.pass_through(&mut protocol_command, &get_features_command)
}

fn send_nvme_identify_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;
    let mut protocol_specific_data = STORAGE_PROTOCOL_SPECIFIC_DATA {
        ProtocolType: ProtocolTypeNvme, // ProtocolTypeNvme
        DataType: 0,                    // NVMe Identify
        ProtocolDataOffset: 0,
        ProtocolDataLength: 4096, // Identify returns 4096 bytes
        ..Default::default()
    };

    let mut protocol_descriptor = STORAGE_PROTOCOL_DATA_DESCRIPTOR {
        Version: std::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32,
        Size: std::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32,
        ProtocolSpecificData: protocol_specific_data,
    };

    let mut output_buffer = vec![0u8; 4096];
    nvme_device.query(&mut protocol_descriptor, &mut output_buffer)
}

fn send_nvme_get_log_page_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_specific_data = STORAGE_PROTOCOL_SPECIFIC_DATA {
        ProtocolType: ProtocolTypeNvme, // ProtocolTypeNvme
        DataType: 2,                    // NVMe Get Log Page
        ProtocolDataOffset: 0,
        ProtocolDataLength: 4096, // Log page size
        ..Default::default()
    };

    let mut protocol_descriptor = STORAGE_PROTOCOL_DATA_DESCRIPTOR {
        Version: std::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32,
        Size: std::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32,
        ProtocolSpecificData: protocol_specific_data,
    };

    let mut output_buffer = vec![0u8; 4096];
    nvme_device.query(&mut protocol_descriptor, &mut output_buffer)
}
