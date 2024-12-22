use crate::dev::disk::{get_physical_drv_number_from_logical_drv, ioctl, open};
use std::ffi::c_void;
use std::io;
use std::ptr;
use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileA, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows_sys::Win32::System::Ioctl::IOCTL_STORAGE_PROTOCOL_COMMAND;
use windows_sys::Win32::System::IO::DeviceIoControl;
use windows_sys::{
    core::*,
    Win32::{
        Devices::{DeviceAndDriverInstallation::*, Properties::*},
        Foundation::*,
        Storage::FileSystem::GetLogicalDriveStringsA,
        System::Ioctl::*,
    },
};

#[repr(C)]
#[derive(Debug, Default)]
struct StorageProtocolCommand {
    version: u32,
    length: u32,
    protocol_type: u32,
    flags: u32,
    return_status: u32,
    error_code: u32,
    command_length: u32,
    error_info_length: u32,
    data_to_device_transfer_length: u32,
    data_from_device_transfer_length: u32,
    timeout_value: u32,
    error_info_offset: u32,
    data_to_device_buffer_offset: u32,
    data_from_device_buffer_offset: u32,
    command_specific: u32,
    reserved0: u32,
    fixed_protocol_return_data: u32,
    reserved1: [u32; 3],
    command: [u8; 1], // Placeholder for ANYSIZE_ARRAY
}

#[repr(C)]
#[derive(Debug, Default)]
struct StorageProtocolSpecificData {
    protocol_type: u32,
    data_type: u32,
    data_offset: u32,
    data_length: u32,
    reserved: [u64; 3],
}

#[repr(C)]
#[derive(Debug, Default)]
struct StorageProtocolDataDescriptor {
    version: u32,
    size: u32,
    protocol_specific_data: StorageProtocolSpecificData,
}

#[repr(C)]
#[derive(Debug, Default)]
struct NvmeCommand {
    opcode: u8,    // 명령 코드
    flags: u8,     // 명령 플래그
    nsid: u32,     // 네임스페이스 ID
    cdw2: u32,     // Command Dword 2
    cdw3: u32,     // Command Dword 3
    metadata: u64, // 메타데이터 주소
    prp1: u64,     // 데이터 버퍼 주소 1
    prp2: u64,     // 데이터 버퍼 주소 2
    cdw10: u32,    // Command Dword 10
    cdw11: u32,    // Command Dword 11
    cdw12: u32,    // Command Dword 12
    cdw13: u32,    // Command Dword 13
    cdw14: u32,    // Command Dword 14
    cdw15: u32,    // Command Dword 15
}

const ProtocolTypeUnknown: u32 = 0;
const ProtocolTypeScsi: u32 = 1;
const ProtocolTypeAta: u32 = 2;
const ProtocolTypeNvme: u32 = 3;
const ProtocolTypeSd: u32 = 4;
const ProtocolTypeUfs: u32 = 5;

struct NvmeDevice {
    handle: HANDLE,
}

impl NvmeDevice {
    fn open(device_path: &str) -> io::Result<Self> {
        let handle = open(device_path, 'w');

        if handle == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self { handle })
        }
    }

    fn send_command(
        &self,
        protocol_command: &mut StorageProtocolCommand,
        nvme_command: &NvmeCommand,
    ) -> io::Result<()> {
        let mut buffer =
            vec![0u8; protocol_command.length as usize + protocol_command.command_length as usize];

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
                ptr::null_mut(),
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

    fn send_protocol_command(
        &self,
        protocol_descriptor: &mut StorageProtocolDataDescriptor,
        output_buffer: &mut [u8],
    ) -> io::Result<()> {
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
                ptr::null_mut(),
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

    let mut protocol_command = StorageProtocolCommand {
        version: 0,
        length: std::mem::size_of::<StorageProtocolCommand>() as u32,
        protocol_type: ProtocolTypeNvme, // ProtocolTypeNvme
        flags: 0,
        command_length: std::mem::size_of::<NvmeCommand>() as u32,
        data_to_device_buffer_offset: std::mem::size_of::<StorageProtocolCommand>() as u32,
        data_from_device_transfer_length: 4096, // Identify command returns 4096 bytes
        data_from_device_buffer_offset: std::mem::size_of::<StorageProtocolCommand>() as u32
            + std::mem::size_of::<NvmeCommand>() as u32,
        timeout_value: 5000,
        ..Default::default()
    };

    let identify_command = NvmeCommand {
        opcode: 0x06, // Identify opcode
        nsid: 0,      // Identify Controller
        cdw10: 1,     // CNS = 1 for Identify Controller
        ..Default::default()
    };

    nvme_device.send_command(&mut protocol_command, &identify_command)
}

fn _send_nvme_get_log_page_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_command = StorageProtocolCommand {
        version: 0,
        length: std::mem::size_of::<StorageProtocolCommand>() as u32,
        protocol_type: ProtocolTypeNvme, // ProtocolTypeNvme
        flags: 0,
        command_length: std::mem::size_of::<NvmeCommand>() as u32,
        data_to_device_buffer_offset: std::mem::size_of::<StorageProtocolCommand>() as u32,
        data_from_device_transfer_length: 4096, // Log page size
        data_from_device_buffer_offset: std::mem::size_of::<StorageProtocolCommand>() as u32
            + std::mem::size_of::<NvmeCommand>() as u32,
        timeout_value: 5000,
        ..Default::default()
    };

    let get_log_page_command = NvmeCommand {
        opcode: 0x02,            // Get Log Page opcode
        nsid: 0,                 // Controller-wide log
        cdw10: (0x02 << 16) | 1, // Log Identifier = 2, Number of Dwords = 1
        ..Default::default()
    };

    nvme_device.send_command(&mut protocol_command, &get_log_page_command)
}

fn send_nvme_set_features_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_command = StorageProtocolCommand {
        version: 0,
        length: std::mem::size_of::<StorageProtocolCommand>() as u32,
        protocol_type: ProtocolTypeNvme, // ProtocolTypeNvme
        flags: 0,
        command_length: std::mem::size_of::<NvmeCommand>() as u32,
        data_to_device_buffer_offset: std::mem::size_of::<StorageProtocolCommand>() as u32,
        data_from_device_transfer_length: 0,
        data_from_device_buffer_offset: 0,
        timeout_value: 5000,
        ..Default::default()
    };

    let set_features_command = NvmeCommand {
        opcode: 0x09,  // Set Features opcode
        nsid: 0,       // NSID 0 for global features
        cdw10: 0x01,   // Feature Identifier: Arbitrary example
        cdw11: 0xABCD, // Feature Value: Example value
        ..Default::default()
    };

    nvme_device.send_command(&mut protocol_command, &set_features_command)
}

fn send_vendor_specific_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_command = StorageProtocolCommand {
        version: 0,
        length: std::mem::size_of::<StorageProtocolCommand>() as u32,
        protocol_type: ProtocolTypeNvme, // ProtocolTypeNvme
        flags: 0,
        command_length: std::mem::size_of::<NvmeCommand>() as u32,
        data_to_device_buffer_offset: std::mem::size_of::<StorageProtocolCommand>() as u32,
        data_from_device_transfer_length: 0,
        data_from_device_buffer_offset: 0,
        timeout_value: 5000,
        ..Default::default()
    };

    let vendor_specific_command = NvmeCommand {
        opcode: 0xC0,  // Vendor-specific opcode
        nsid: 0,       // Adjust NSID as required
        cdw10: 0x1234, // Vendor-specific data
        cdw11: 0x5678, // Additional vendor-specific data
        ..Default::default()
    };

    nvme_device.send_command(&mut protocol_command, &vendor_specific_command)
}

fn send_nvme_get_features_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_command = StorageProtocolCommand {
        version: 0,
        length: std::mem::size_of::<StorageProtocolCommand>() as u32,
        protocol_type: ProtocolTypeNvme, // ProtocolTypeNvme
        flags: 0,
        command_length: std::mem::size_of::<NvmeCommand>() as u32,
        data_to_device_buffer_offset: std::mem::size_of::<StorageProtocolCommand>() as u32,
        data_from_device_transfer_length: 4096, // Feature data size
        data_from_device_buffer_offset: std::mem::size_of::<StorageProtocolCommand>() as u32
            + std::mem::size_of::<NvmeCommand>() as u32,
        timeout_value: 5000,
        ..Default::default()
    };

    let get_features_command = NvmeCommand {
        opcode: 0x0A, // Get Features opcode
        nsid: 0,      // NSID 0 for global features
        cdw10: 0x01,  // Feature Identifier: Example identifier
        ..Default::default()
    };

    nvme_device.send_command(&mut protocol_command, &get_features_command)
}

fn send_nvme_identify_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_specific_data = StorageProtocolSpecificData {
        protocol_type: ProtocolTypeNvme, // ProtocolTypeNvme
        data_type: 0,                    // NVMe Identify
        data_offset: 0,
        data_length: 4096, // Identify returns 4096 bytes
        ..Default::default()
    };

    let mut protocol_descriptor = StorageProtocolDataDescriptor {
        version: std::mem::size_of::<StorageProtocolDataDescriptor>() as u32,
        size: std::mem::size_of::<StorageProtocolDataDescriptor>() as u32,
        protocol_specific_data,
    };

    let mut output_buffer = vec![0u8; 4096];
    nvme_device.send_protocol_command(&mut protocol_descriptor, &mut output_buffer)
}

fn send_nvme_get_log_page_command(device_path: &str) -> io::Result<()> {
    let nvme_device = NvmeDevice::open(device_path)?;

    let mut protocol_specific_data = StorageProtocolSpecificData {
        protocol_type: ProtocolTypeNvme, // ProtocolTypeNvme
        data_type: 2,                    // NVMe Get Log Page
        data_offset: 0,
        data_length: 4096, // Log page size
        ..Default::default()
    };

    let mut protocol_descriptor = StorageProtocolDataDescriptor {
        version: std::mem::size_of::<StorageProtocolDataDescriptor>() as u32,
        size: std::mem::size_of::<StorageProtocolDataDescriptor>() as u32,
        protocol_specific_data,
    };

    let mut output_buffer = vec![0u8; 4096];
    nvme_device.send_protocol_command(&mut protocol_descriptor, &mut output_buffer)
}
