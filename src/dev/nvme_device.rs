use crate::dev::disk::open;
use crate::dev::nvme_define::{NVME_IDENTIFY_CNS_CODES::*, NVME_LOG_PAGES::*, *};
use std::mem::offset_of;
use std::{ffi::c_void, io, mem::size_of, ptr::null_mut};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Ioctl::*;
use windows_sys::Win32::System::IO::DeviceIoControl;

trait StorageProtocolCommand {
    fn new(&mut self) -> &mut Self;
    fn nvme_command(&mut self, command: &NVME_COMMAND) -> &mut Self;
    fn set_data_in(&mut self, direction: u8, data: &[u8]) -> &mut Self;
    fn get_data(&mut self, data: &mut [u8]) -> &mut Self;
}

impl StorageProtocolCommand for STORAGE_PROTOCOL_COMMAND {
    fn new(&mut self) -> &mut Self {
        self.Version = STORAGE_PROTOCOL_STRUCTURE_VERSION;
        self.Length = size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
        self.ProtocolType = ProtocolTypeNvme as i32;
        self.Flags = STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST;
        self.CommandLength = STORAGE_PROTOCOL_COMMAND_LENGTH_NVME;
        self.TimeOutValue = 30;
        self
    }
    fn nvme_command(&mut self, command: &NVME_COMMAND) -> &mut Self {
        let command_offset = offset_of!(STORAGE_PROTOCOL_COMMAND, Command);
        let command_size = size_of::<NVME_COMMAND>();
        let buffer = self as *mut _ as *mut u8;
        unsafe {
            std::ptr::copy_nonoverlapping(
                command as *const _ as *const u8,
                buffer.add(command_offset),
                command_size,
            );
        };
        self.ErrorInfoOffset = (command_offset + command_size) as u32;
        self.CommandSpecific = STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND;
        self
    }
    fn set_data_in(&mut self, direction: u8, data: &[u8]) -> &mut Self {
        match direction {
            1 => self.DataToDeviceTransferLength = data.len() as u32,
            2 => self.DataFromDeviceTransferLength = data.len() as u32,
            _ => {}
        }
        self.DataToDeviceBufferOffset = self.ErrorInfoOffset + self.ErrorInfoLength;
        self.DataFromDeviceBufferOffset =
            self.DataToDeviceBufferOffset + self.DataToDeviceTransferLength;
        if direction == 1 && !data.is_empty() {
            let data_offset = self.DataToDeviceBufferOffset as usize;
            let buffer = self as *mut _ as *mut u8;
            let buffer_slice =
                unsafe { std::slice::from_raw_parts_mut(buffer, data_offset + data.len()) };
            buffer_slice[data_offset..data_offset + data.len()].copy_from_slice(data);
        }
        self
    }
    fn get_data(&mut self, data: &mut [u8]) -> &mut Self {
        if !data.is_empty() {
            let data_len = self.DataFromDeviceTransferLength as usize;
            let data_offset = self.DataFromDeviceBufferOffset as usize;
            let buffer = unsafe {
                std::slice::from_raw_parts_mut(self as *mut _ as *mut u8, data_offset + data_len)
            };
            data.copy_from_slice(&buffer[data_offset..data_offset + data_len]);
        }
        self
    }
}

trait StorageProtocolSpecificData {
    fn new(data_type: i32, request_value: u32, length: usize) -> Self;
    fn set(&mut self, data_type: i32, value: u32, length: usize) -> &mut Self;
    fn is_valid(&self, length: usize) -> bool;
    fn get_data(&self) -> &[u8];
}

impl StorageProtocolSpecificData for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn new(data_type: i32, request_value: u32, length: usize) -> Self {
        STORAGE_PROTOCOL_SPECIFIC_DATA {
            ProtocolType: ProtocolTypeNvme as i32,
            ProtocolDataOffset: size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32,
            DataType: data_type as u32,
            ProtocolDataRequestValue: request_value,
            ProtocolDataRequestSubValue: 0,
            ProtocolDataRequestSubValue2: 0,
            ProtocolDataRequestSubValue3: 0,
            ProtocolDataRequestSubValue4: 0,
            FixedProtocolReturnData: 0,
            ProtocolDataLength: length as u32,
        }
    }
    fn set(&mut self, data_type: i32, request_value: u32, length: usize) -> &mut Self {
        self.ProtocolType = ProtocolTypeNvme as i32;
        self.ProtocolDataOffset = size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32;
        self.DataType = data_type as u32;
        self.ProtocolDataRequestValue = request_value;
        self.ProtocolDataLength = length as u32;
        self
    }
    fn is_valid(&self, length: usize) -> bool {
        self.ProtocolDataOffset >= size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32
            && self.ProtocolDataLength >= length as u32
    }
    fn get_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const _ as *const u8).add(self.ProtocolDataOffset as usize),
                self.ProtocolDataLength as usize,
            )
        }
    }
}

// To use FIELD_OFFSET macro equivalent in Rust:
// let offset = field_offset::<SomeType, SomeFieldType>(0 as *const SomeType, |s| &s.some_field);
pub struct InboxDriver {
    handle: HANDLE,
}

impl InboxDriver {
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

    pub fn nvme_send_passthrough_command(
        &self,
        direction: u8,
        nvme_command: &NVME_COMMAND,
        data_buffer: &mut [u8],
        return_dw0: &mut u32,
    ) -> io::Result<NVME_COMMAND_STATUS> {
        let command_offset = offset_of!(STORAGE_PROTOCOL_COMMAND, Command);
        let buffer_size = command_offset + size_of::<NVME_COMMAND>() + data_buffer.len();
        let mut buffer = vec![0; buffer_size];
        let protocol_command =
            unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROTOCOL_COMMAND) };
        protocol_command
            .new()
            .nvme_command(nvme_command)
            .set_data_in(direction, data_buffer);

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
            Err(io::Error::last_os_error())
        } else {
            if direction == 2 {
                protocol_command.get_data(data_buffer);
            }
            *return_dw0 = protocol_command.FixedProtocolReturnData;
            let ncs = NVME_COMMAND_STATUS::from(protocol_command.ErrorCode as u16);
            Ok(ncs)
        }
    }

    pub fn nvme_send_query_command(
        &self,
        property_id: i32,
        protocol_data: &STORAGE_PROTOCOL_SPECIFIC_DATA,
        data_length: usize,
    ) -> io::Result<&[u8]> {
        let data_offset = offset_of!(STORAGE_PROPERTY_QUERY, AdditionalParameters);
        let query_size = data_offset + size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() + data_length;
        let mut buffer = vec![0u8; query_size];
        let propert_query = unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROPERTY_QUERY) };
        propert_query.PropertyId = property_id;
        propert_query.QueryType = PropertyStandardQuery;

        let protocol_specific_data_ptr =
            unsafe { buffer.as_mut_ptr().add(data_offset) as *mut STORAGE_PROTOCOL_SPECIFIC_DATA };
        unsafe {
            std::ptr::copy_nonoverlapping(protocol_data, protocol_specific_data_ptr, 1);
        }
        let mut returned_length = 0;
        let result = unsafe {
            DeviceIoControl(
                self.get_handle(),
                IOCTL_STORAGE_QUERY_PROPERTY,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                &mut returned_length,
                null_mut(),
            )
        };

        if result == 0 {
            return Err(io::Error::last_os_error());
        }

        let data_descriptor =
            unsafe { &*(buffer.as_ptr() as *const STORAGE_PROTOCOL_DATA_DESCRIPTOR) };
        if data_descriptor.Version != size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
            || data_descriptor.Size != size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Data descriptor header not valid",
            ));
        }

        let protocol_specific_data = &data_descriptor.ProtocolSpecificData;
        if !protocol_specific_data.is_valid(NVME_IDENTIFY_SIZE) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "ProtocolData Offset/Length not valid",
            ));
        }

        Ok(protocol_specific_data.get_data())
    }

    pub fn nvme_identify_controller(&self) -> io::Result<NVME_IDENTIFY_CONTROLLER_DATA> {
        let protocol_specific_data = STORAGE_PROTOCOL_SPECIFIC_DATA::new(
            NVMeDataTypeIdentify,
            NVME_IDENTIFY_CNS_CONTROLLER as u32,
            NVME_IDENTIFY_SIZE,
        );

        if let Ok(identify_controller_data_bytes) = self.nvme_send_query_command(
            StorageAdapterProtocolSpecificProperty,
            &protocol_specific_data,
            NVME_IDENTIFY_SIZE,
        ) {
            let identify_controller_data = unsafe {
                *(identify_controller_data_bytes.as_ptr() as *const NVME_IDENTIFY_CONTROLLER_DATA)
            };
            if identify_controller_data.VID == 0 || identify_controller_data.NN == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Identify Controller Data not valid",
                ));
            }
            Ok(identify_controller_data)
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn nvme_identify_namespace(&self) -> io::Result<NVME_IDENTIFY_NAMESPACE_DATA> {
        let protocol_specific_data = STORAGE_PROTOCOL_SPECIFIC_DATA::new(
            NVMeDataTypeIdentify,
            NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE as u32,
            NVME_IDENTIFY_SIZE,
        );

        if let Ok(data_bytes) = self.nvme_send_query_command(
            StorageAdapterProtocolSpecificProperty,
            &protocol_specific_data,
            NVME_IDENTIFY_SIZE,
        ) {
            let data = unsafe { *(data_bytes.as_ptr() as *const NVME_IDENTIFY_NAMESPACE_DATA) };
            Ok(data)
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn nvme_get_log_pages(&self) -> io::Result<NVME_HEALTH_INFO_LOG> {
        let protocol_specific_data = STORAGE_PROTOCOL_SPECIFIC_DATA::new(
            NVMeDataTypeLogPage,
            NVME_LOG_PAGE_HEALTH_INFO as u32,
            size_of::<NVME_HEALTH_INFO_LOG>(),
        );
        if let Ok(data_bytes) = self.nvme_send_query_command(
            StorageDeviceProtocolSpecificProperty,
            &protocol_specific_data,
            size_of::<NVME_HEALTH_INFO_LOG>(),
        ) {
            let data = unsafe { *(data_bytes.as_ptr() as *const NVME_HEALTH_INFO_LOG) };
            Ok(data)
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn nvme_get_feature(&self) -> io::Result<()> {
        const BUFFER_LENGTH: usize = size_of::<STORAGE_PROPERTY_QUERY>() - size_of::<[u8; 1]>()
            + size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>()
            + NVME_MAX_LOG_SIZE;
        let mut buffer: Vec<u8> = vec![0; BUFFER_LENGTH];

        let query = unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROPERTY_QUERY) };
        let protocol_data = unsafe {
            &mut *(query.AdditionalParameters.as_mut_ptr() as *mut STORAGE_PROTOCOL_SPECIFIC_DATA)
        };

        query.PropertyId = StorageDeviceProtocolSpecificProperty;
        query.QueryType = PropertyStandardQuery;

        protocol_data.ProtocolType = ProtocolTypeNvme as i32;
        protocol_data.DataType = NVMeDataTypeFeature as u32;
        protocol_data.ProtocolDataRequestValue = NVME_FEATURE_VOLATILE_WRITE_CACHE;
        protocol_data.ProtocolDataRequestSubValue = 0;
        protocol_data.ProtocolDataOffset = size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32;
        protocol_data.ProtocolDataLength = NVME_MAX_LOG_SIZE as u32;

        let mut returned_length = 0;
        let result = unsafe {
            DeviceIoControl(
                self.get_handle(),
                IOCTL_STORAGE_QUERY_PROPERTY,
                buffer.as_mut_ptr() as *mut c_void,
                BUFFER_LENGTH as u32,
                buffer.as_mut_ptr() as *mut c_void,
                BUFFER_LENGTH as u32,
                &mut returned_length,
                null_mut(),
            )
        };

        if result == 0 {
            return Err(io::Error::last_os_error());
        }

        let protocol_data_descr =
            unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROTOCOL_DATA_DESCRIPTOR) };

        if protocol_data_descr.Version != size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
            || protocol_data_descr.Size != size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Data descriptor header not valid",
            ));
        }

        let protocol_data = &protocol_data_descr.ProtocolSpecificData;

        if protocol_data.ProtocolDataOffset < size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32
            || protocol_data.ProtocolDataLength < NVME_MAX_LOG_SIZE as u32
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "ProtocolData Offset/Length not valid",
            ));
        }

        println!(
            "Volatile Cache: {:x}",
            protocol_data.FixedProtocolReturnData
        );

        println!("***Get Feature - Volatile Cache succeeded***");
        Ok(())
    }

    pub fn nvme_set_features(&self) -> io::Result<()> {
        let buffer_length = size_of::<STORAGE_PROPERTY_SET>() - size_of::<[u8; 1]>()
            + size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA_EXT>()
            + NVME_MAX_LOG_SIZE;
        let mut buffer: Vec<u8> = vec![0; buffer_length as usize];

        let set_property = unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROPERTY_SET) };
        let protocol_data = unsafe {
            &mut *(set_property.AdditionalParameters.as_mut_ptr()
                as *mut STORAGE_PROTOCOL_SPECIFIC_DATA_EXT)
        };

        set_property.PropertyId = StorageAdapterProtocolSpecificProperty;
        set_property.SetType = PropertyStandardSet;

        protocol_data.ProtocolType = ProtocolTypeNvme as i32;
        protocol_data.DataType = NVMeDataTypeFeature as u32;
        protocol_data.ProtocolDataValue = NVME_FEATURE_HOST_CONTROLLED_THERMAL_MANAGEMENT;
        protocol_data.ProtocolDataSubValue = 0;
        protocol_data.ProtocolDataSubValue2 = 0;
        protocol_data.ProtocolDataSubValue3 = 0;
        protocol_data.ProtocolDataSubValue4 = 0;
        protocol_data.ProtocolDataSubValue5 = 0;
        protocol_data.ProtocolDataOffset = 0;
        protocol_data.ProtocolDataLength = 0;

        let mut returned_length = 0;
        let result = unsafe {
            DeviceIoControl(
                self.get_handle(),
                IOCTL_STORAGE_SET_PROPERTY,
                buffer.as_mut_ptr() as *mut c_void,
                buffer_length as u32,
                buffer.as_mut_ptr() as *mut c_void,
                buffer_length as u32,
                &mut returned_length,
                null_mut(),
            )
        };

        if result == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for InboxDriver {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}
