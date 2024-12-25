use crate::dev::nvme_define::*;
use crate::dev::nvme_device::*;
use std::{
    ffi::c_void,
    io,
    mem::{size_of, transmute},
    ptr::null_mut,
};
use windows_sys::Win32::System::Ioctl::*;
use windows_sys::Win32::System::IO::DeviceIoControl;

pub fn nvme_identify_query(device: &NvmeDevice) -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![
        0;
        size_of::<STORAGE_PROPERTY_QUERY>() - size_of::<[u8; 1]>()
            + size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>()
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
    protocol_data.ProtocolDataOffset = size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32;
    protocol_data.ProtocolDataLength = NVME_IDENTIFY_SIZE as u32;

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
        return Err(io::Error::last_os_error());
    }

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

    let identify_controller_data = unsafe {
        let protocol_data_ptr = protocol_data as *const _ as *const u8; // Get raw pointer to u8
        let offset_ptr = protocol_data_ptr.add(protocol_data.ProtocolDataOffset as usize);
        &*(offset_ptr as *const NVME_IDENTIFY_CONTROLLER_DATA)
    };

    if identify_controller_data.VID == 0 || identify_controller_data.NN == 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Identify Controller Data not valid",
        ));
    }

    println!("***Identify Controller Data succeeded***");
    Ok(())
}

pub fn nvme_get_log_pages(device: &NvmeDevice) -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![
        0;
        size_of::<STORAGE_PROPERTY_QUERY>() - size_of::<[u8; 1]>()
            + size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>()
            + size_of::<NVME_HEALTH_INFO_LOG>()
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
    protocol_data.ProtocolDataOffset = size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32;
    protocol_data.ProtocolDataLength = size_of::<NVME_HEALTH_INFO_LOG>() as u32;

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
        return Err(io::Error::last_os_error());
    }

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
        || protocol_data.ProtocolDataLength < size_of::<NVME_HEALTH_INFO_LOG>() as u32
    {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "ProtocolData Offset/Length not valid",
        ));
    }

    let smart_info = unsafe {
        let protocol_data_ptr = protocol_data as *const _ as *const u8; // Get raw pointer to u8
        let offset_ptr = protocol_data_ptr.add(protocol_data.ProtocolDataOffset as usize);
        &*(offset_ptr as *const NVME_HEALTH_INFO_LOG)
    };

    println!(
        "SMART/Health Information Log Data - Temperature: {}.",
        (smart_info.Temperature as u32) - 273
    );
    println!("***SMART/Health Information Log succeeded***");
    Ok(())
}

pub fn nvme_get_feature(device: &NvmeDevice) -> io::Result<()> {
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
            device.get_handle(),
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

pub fn nvme_set_features(device: &NvmeDevice) -> io::Result<()> {
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
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

// Example Enum Definitions (actual values and types may vary)
#[derive(Debug)]
pub enum NvmeOpcodeType {
    Write,
    NoBuffer,
    // Add more variants as needed
}

#[derive(Debug, Copy, Clone)]
pub enum NvmeVscOpcode {
    Write,
    Read,
    None,
    // Add more variants as needed
}

impl Default for NvmeVscOpcode {
    fn default() -> Self {
        NvmeVscOpcode::Write
    }
}

#[derive(Debug, PartialEq)]
pub enum NvmeStatusType {
    GenericCommand,
    // Add more variants as needed
}

#[derive(Debug, PartialEq)]
pub enum NvmeStatus {
    SuccessCompletion,
    // Add more variants as needed
}
impl Default for NvmeStatusType {
    fn default() -> Self {
        NvmeStatusType::GenericCommand // Example
    }
}

impl Default for NvmeStatus {
    fn default() -> Self {
        NvmeStatus::SuccessCompletion // Example
    }
}

// Example Struct Definitions (actual fields may vary)
#[derive(Debug, Default)]
pub struct NvmeDisk {
    pub project_type: String, // Example field, adjust as necessary
                              // Add more fields as needed
}

const NVME_DATA_BUFFER_SIZE: usize = 4096; // Example size, adjust as necessary
const VS_STD_NVME_CMD_TYPE_READ: u32 = 0;
const VS_STD_NVME_CMD_TYPE_WRITE: u32 = 1;
const VS_STD_NVME_CMD_TYPE_NON_DATA: u32 = 2;

pub fn nvme_send_vsc2_passthrough_command(
    p_nd: &NvmeDisk,
    sub_opcode: u32, // Adjust type if necessary
    direction: i32,
    p_param_buf: &[u8],
    p_data_buf: &[u8],
    p_ncs: Option<&mut NVME_COMMAND_STATUS>,
    p_completion_dw0: Option<&mut u32>,
    nsid: u32, // Adjust type if necessary
) -> io::Result<()> {
    let mut default_ncs = NVME_COMMAND_STATUS::default();
    let mut ncs = p_ncs.unwrap_or(&mut default_ncs);
    let mut default_completion_dw0 = 0;
    let mut completion_dw0 = p_completion_dw0.unwrap_or(&mut default_completion_dw0);

    let mut nc = NVME_COMMAND::default();
    nc.CDW0.OPC = NvmeVscOpcode::Write as u8;
    nc.u.GENERAL.CDW10 = p_param_buf.len() as u32 / size_of::<u32>() as u32;
    nc.u.GENERAL.CDW11 = 0;
    nc.u.GENERAL.CDW12 = set_vsc_op_code_by_project_type(p_nd.project_type.clone(), sub_opcode); // Adjust return type and function call if necessary
    nc.u.GENERAL.CDW13 = 0;
    nc.u.GENERAL.CDW14 = 0;
    nc.NSID = nsid;

    let err = nvme_send_passthrough_command(
        p_nd,
        NvmeOpcodeType::Write,
        &nc,
        p_param_buf,
        ncs,
        completion_dw0,
    );
    if err.is_err() || direction == 0 {
        return err;
    }
    if ncs.SCT != NvmeStatusType::GenericCommand as u16
        || ncs.SC != NvmeStatus::SuccessCompletion as u16
    {
        return Err(io::Error::new(io::ErrorKind::Other, "Not Supported"));
    }

    // Data phase
    nc.CDW0.OPC = match direction {
        1 => NvmeVscOpcode::Write as u8,
        2 => NvmeVscOpcode::Read as u8, // Adjust based on actual logic
        _ => return Err(io::Error::new(io::ErrorKind::Other, "Not Supported")),
    };
    nc.u.GENERAL.CDW10 = p_data_buf.len() as u32 / size_of::<u32>() as u32;
    nc.u.GENERAL.CDW14 = 1; // Phase ID

    nvme_send_passthrough_command(
        p_nd,
        NvmeOpcodeType::NoBuffer,
        &nc,
        p_data_buf,
        ncs,
        completion_dw0,
    )
}

pub fn nvme_send_vsc_admin_passthrough_command(
    p_nd: &NvmeDisk,
    p_nc_admin: &NVME_COMMAND,
    p_data_buf: Option<&[u8]>,
    p_ncs: Option<&mut NVME_COMMAND_STATUS>,
    p_completion_dw0: Option<&mut u32>,
) -> io::Result<()> {
    let mut opflag = (p_nc_admin.CDW0.OPC as i32) & 3;
    let sub_opcode = match opflag {
        0 => VS_STD_NVME_CMD_TYPE_NON_DATA, // Adjust based on actual enum or constant
        1 => VS_STD_NVME_CMD_TYPE_WRITE,
        2 => VS_STD_NVME_CMD_TYPE_READ,
        _ => return Err(io::Error::new(io::ErrorKind::Other, "Not Supported")),
    };

    let mut param_buffer = [0u8; NVME_DATA_BUFFER_SIZE];
    let command_bytes = unsafe {
        std::slice::from_raw_parts(
            p_nc_admin as *const NVME_COMMAND as *const u8,
            size_of::<NVME_COMMAND>(),
        )
    };
    param_buffer[..command_bytes.len()].copy_from_slice(command_bytes);

    if p_data_buf.is_none() {
        opflag = 0;
    }

    nvme_send_vsc2_passthrough_command(
        p_nd,
        sub_opcode,
        opflag,
        &param_buffer,
        p_data_buf.unwrap_or(&[]),
        p_ncs,
        p_completion_dw0,
        0, // Default NSID, adjust if necessary
    )
}

// Example function, adjust based on actual implementation
fn set_vsc_op_code_by_project_type(project_type: String, sub_opcode: u32) -> u32 {
    // Implement logic to determine sub-opcode based on project type
    sub_opcode
}

// Example function, adjust based on actual implementation
fn nvme_send_passthrough_command(
    _p_nd: &NvmeDisk,
    _opcode_type: NvmeOpcodeType,
    _nc: &NVME_COMMAND,
    _buffer: &[u8],
    _p_ncs: &mut NVME_COMMAND_STATUS,
    _p_completion_dw0: &mut u32,
) -> io::Result<()> {
    // Implement the actual logic for sending the passthrough command
    Ok(())
}
