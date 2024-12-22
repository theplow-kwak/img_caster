use crate::dev::nvme_device::*;
use crate::dev::nvme_structs::{StorageProtocolSpecificDataExt, *};
use std::{ffi::c_void, ptr::null_mut};
use windows_sys::Win32::System::Ioctl::*;
use windows_sys::Win32::System::IO::DeviceIoControl;

pub fn nvme_identify_query(device: &NvmeDevice) -> Result<(), std::io::Error> {
    let mut buffer: Vec<u8> = vec![
        0;
        std::mem::size_of::<STORAGE_PROPERTY_QUERY>()
            - std::mem::size_of::<[u8; 1]>()
            + std::mem::size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>()
            + NVME_MAX_LOG_SIZE
    ];
    let buffer_length = buffer.len() as u32;

    let query = unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROPERTY_QUERY) };
    let protocol_data_descr =
        unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROTOCOL_DATA_DESCRIPTOR) };
    let protocol_data = unsafe {
        &mut *(query.AdditionalParameters.as_mut_ptr() as *mut STORAGE_PROTOCOL_SPECIFIC_DATA)
    };

    query.PropertyId = StorageAdapterProtocolSpecificProperty;
    query.QueryType = PropertyStandardQuery;

    protocol_data.ProtocolType = ProtocolTypeNvme as i32;
    protocol_data.DataType = NVMeDataTypeIdentify as u32;
    protocol_data.ProtocolDataRequestValue = NVME_IDENTIFY_CNS_CONTROLLER;
    protocol_data.ProtocolDataRequestSubValue = 0;
    protocol_data.ProtocolDataOffset = std::mem::size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32;
    protocol_data.ProtocolDataLength = NVME_MAX_LOG_SIZE as u32;

    let mut returned_length = 0;
    let result = unsafe {
        DeviceIoControl(
            device.get_handle(),
            IOCTL_STORAGE_QUERY_PROPERTY as u32,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
            &mut returned_length,
            null_mut(),
        )
    };

    if result == 0 {
        return Err(std::io::Error::last_os_error());
    }

    if protocol_data_descr.Version != std::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
        || protocol_data_descr.Size != std::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Data descriptor header not valid",
        ));
    }

    let protocol_data = &protocol_data_descr.ProtocolSpecificData;

    if protocol_data.ProtocolDataOffset
        < std::mem::size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32
        || protocol_data.ProtocolDataLength < NVME_MAX_LOG_SIZE as u32
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ProtocolData Offset/Length not valid",
        ));
    }

    let identify_controller_data = unsafe {
        &*(buffer
            .as_ptr()
            .add(protocol_data.ProtocolDataOffset as usize)
            as *const NvmeIdentifyControllerData)
    };

    if identify_controller_data.vid == 0 || identify_controller_data.nn == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Identify Controller Data not valid",
        ));
    }

    println!("***Identify Controller Data succeeded***");
    Ok(())
}

fn nvme_get_log_pages(device: &NvmeDevice) -> Result<(), std::io::Error> {
    let mut buffer: Vec<u8> = vec![
        0;
        std::mem::size_of::<StoragePropertyQuery>()
            - std::mem::size_of::<[u8; 1]>()
            + std::mem::size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>()
            + std::mem::size_of::<NVME_HEALTH_INFO_LOG>()
    ];
    let buffer_length = buffer.len() as u32;

    let query = unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROPERTY_QUERY) };
    let protocol_data_descr =
        unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROTOCOL_DATA_DESCRIPTOR) };
    let protocol_data = unsafe {
        &mut *(query.AdditionalParameters.as_mut_ptr() as *mut STORAGE_PROTOCOL_SPECIFIC_DATA)
    };

    query.PropertyId = StorageDeviceProtocolSpecificProperty;
    query.QueryType = PropertyStandardQuery;

    protocol_data.ProtocolType = ProtocolTypeNvme as i32;
    protocol_data.DataType = NVMeDataTypeLogPage as u32;
    protocol_data.ProtocolDataRequestValue = NVME_LOG_PAGE_HEALTH_INFO;
    protocol_data.ProtocolDataRequestSubValue = 0;
    protocol_data.ProtocolDataOffset = std::mem::size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32;
    protocol_data.ProtocolDataLength = std::mem::size_of::<NVME_HEALTH_INFO_LOG>() as u32;

    let mut returned_length = 0;
    let result = unsafe {
        DeviceIoControl(
            device.get_handle(),
            IOCTL_STORAGE_QUERY_PROPERTY,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
            &mut returned_length,
            null_mut(),
        )
    };

    if result == 0 {
        return Err(std::io::Error::last_os_error());
    }

    if protocol_data_descr.Version != std::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
        || protocol_data_descr.Size != std::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Data descriptor header not valid",
        ));
    }

    let protocol_data = &protocol_data_descr.ProtocolSpecificData;

    if protocol_data.ProtocolDataOffset
        < std::mem::size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32
        || protocol_data.ProtocolDataLength < std::mem::size_of::<NVME_HEALTH_INFO_LOG>() as u32
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ProtocolData Offset/Length not valid",
        ));
    }

    let smart_info = unsafe {
        &*(buffer
            .as_ptr()
            .add(protocol_data.ProtocolDataOffset as usize)
            as *const NVME_HEALTH_INFO_LOG)
    };

    println!(
        "SMART/Health Information Log Data - Temperature: {}.",
        ((smart_info.temperature[1] as u32) << 8 | smart_info.temperature[0] as u32) - 273
    );
    println!("***SMART/Health Information Log succeeded***");
    Ok(())
}

fn nvme_set_features(device: &NvmeDevice) -> Result<(), std::io::Error> {
    let buffer_length = std::mem::size_of::<StoragePropertySet>() - std::mem::size_of::<[u8; 1]>()
        + std::mem::size_of::<StorageProtocolSpecificDataExt>()
        + NVME_MAX_LOG_SIZE;
    let mut buffer: Vec<u8> = vec![0; buffer_length as usize];

    let set_property = unsafe { &mut *(buffer.as_mut_ptr() as *mut StoragePropertySet) };
    let protocol_data = unsafe {
        &mut *(set_property.additional_parameters.as_mut_ptr()
            as *mut StorageProtocolSpecificDataExt)
    };

    set_property.property_id = StorageAdapterProtocolSpecificProperty;
    set_property.set_type = PropertyStandardSet;

    protocol_data.protocol_type = ProtocolTypeNvme as i32;
    protocol_data.data_type = NVMeDataTypeFeature as u32;
    protocol_data.protocol_data_value = NVME_FEATURE_HOST_CONTROLLED_THERMAL_MANAGEMENT;
    protocol_data.protocol_data_sub_value = 0;
    protocol_data.protocol_data_sub_value2 = 0;
    protocol_data.protocol_data_sub_value3 = 0;
    protocol_data.protocol_data_sub_value4 = 0;
    protocol_data.protocol_data_sub_value5 = 0;
    protocol_data.protocol_data_offset = 0;
    protocol_data.protocol_data_length = 0;

    let mut returned_length = 0;
    let result = unsafe {
        DeviceIoControl(
            device.get_handle(),
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
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}
