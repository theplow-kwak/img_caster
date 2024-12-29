use crate::dev::nvme_define::*;
use crate::dev::nvme_device::*;
use std::mem::offset_of;
use std::{
    ffi::c_void,
    io,
    mem::{size_of, transmute},
    ptr::null_mut,
};
use windows_sys::Win32::System::Ioctl::*;
use windows_sys::Win32::System::IO::DeviceIoControl;

trait StorageProtocolSpecificData {
    fn length(&mut self, length: usize) -> &mut Self;
    fn set_protocol_data_offset(&mut self) -> &mut Self;
    fn request_value(&mut self, value: u32) -> &mut Self;
    fn data_type(&mut self, data_type: i32) -> &mut Self;
    fn set_protocol_type(&mut self) -> &mut Self;
    fn is_valid(&self, length: usize) -> bool;
    fn default(&mut self) -> &mut Self;
    fn data_ptr(&self) -> *const u8;
}

impl StorageProtocolSpecificData for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn default(&mut self) -> &mut Self {
        self.set_protocol_type();
        self.set_protocol_data_offset();
        self
    }

    fn set_protocol_type(&mut self) -> &mut Self {
        self.ProtocolType = ProtocolTypeNvme as i32;
        self
    }

    fn data_type(&mut self, data_type: i32) -> &mut Self {
        self.DataType = data_type as u32;
        self
    }

    fn request_value(&mut self, value: u32) -> &mut Self {
        self.ProtocolDataRequestValue = value;
        self
    }

    fn set_protocol_data_offset(&mut self) -> &mut Self {
        self.ProtocolDataOffset = size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32;
        self
    }

    fn length(&mut self, length: usize) -> &mut Self {
        self.ProtocolDataLength = length as u32;
        self
    }

    fn is_valid(&self, length: usize) -> bool {
        self.ProtocolDataOffset >= size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32
            && self.ProtocolDataLength >= length as u32
    }

    fn data_ptr(&self) -> *const u8 {
        let protocol_data_ptr = self as *const _ as *const u8;
        let offset_ptr = unsafe { protocol_data_ptr.add(self.ProtocolDataOffset as usize) };
        offset_ptr
    }
}

pub fn nvme_identify_query(device: &NvmeDevice) -> io::Result<NVME_IDENTIFY_CONTROLLER_DATA> {
    let data_offset = offset_of!(STORAGE_PROPERTY_QUERY, AdditionalParameters);
    let mut buffer: Vec<u8> =
        vec![0; data_offset + size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() + NVME_MAX_LOG_SIZE];
    let buffer_length = buffer.len() as u32;

    let query_command = unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROPERTY_QUERY) };
    let protocol_data = unsafe {
        &mut *(query_command.AdditionalParameters.as_mut_ptr()
            as *mut STORAGE_PROTOCOL_SPECIFIC_DATA)
    };

    query_command.PropertyId = StorageAdapterProtocolSpecificProperty;
    query_command.QueryType = PropertyStandardQuery;

    protocol_data
        .default()
        .data_type(NVMeDataTypeIdentify)
        .request_value(NVME_IDENTIFY_CNS_CONTROLLER)
        .length(NVME_IDENTIFY_SIZE);

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

    if !protocol_data.is_valid(NVME_MAX_LOG_SIZE) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "ProtocolData Offset/Length not valid",
        ));
    }

    let identify_controller_data =
        unsafe { &*(protocol_data.data_ptr() as *const NVME_IDENTIFY_CONTROLLER_DATA) };

    if identify_controller_data.VID == 0 || identify_controller_data.NN == 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Identify Controller Data not valid",
        ));
    }

    println!("***Identify Controller Data succeeded***");
    Ok(*identify_controller_data)
}

pub fn nvme_get_log_pages(device: &NvmeDevice) -> io::Result<()> {
    let command_offset = offset_of!(STORAGE_PROPERTY_QUERY, AdditionalParameters);
    let mut buffer: Vec<u8> =
        vec![0; command_offset + size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() + NVME_MAX_LOG_SIZE];
    let buffer_length = buffer.len() as u32;

    let query = unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROPERTY_QUERY) };
    let protocol_data_descr =
        unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROTOCOL_DATA_DESCRIPTOR) };
    let protocol_data = unsafe {
        &mut *(query.AdditionalParameters.as_mut_ptr() as *mut STORAGE_PROTOCOL_SPECIFIC_DATA)
    };

    query.PropertyId = StorageDeviceProtocolSpecificProperty;
    query.QueryType = PropertyStandardQuery;

    protocol_data
        .default()
        .data_type(NVMeDataTypeLogPage)
        .request_value(NVME_LOG_PAGE_HEALTH_INFO)
        .length(size_of::<NVME_HEALTH_INFO_LOG>());

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

    if !protocol_data.is_valid(size_of::<NVME_HEALTH_INFO_LOG>()) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "ProtocolData Offset/Length not valid",
        ));
    }

    let smart_info = unsafe { &*(protocol_data.data_ptr() as *const NVME_HEALTH_INFO_LOG) };

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
#[repr(u8)]
#[derive(Debug)]
pub enum NvmeOpcodeType {
    NOBUFFER,
    WRITE,
    READ,
    READWRITE,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum NvmeVscOpcode {
    None = 0x00,
    Write = 0x01,
    Read = 0x02,
}

impl Default for NvmeVscOpcode {
    fn default() -> Self {
        NvmeVscOpcode::None
    }
}

const NVME_DATA_BUFFER_SIZE: usize = 4096; // Example size, adjust as necessary
const VS_STD_NVME_CMD_TYPE_READ: u32 = 0;
const VS_STD_NVME_CMD_TYPE_WRITE: u32 = 1;
const VS_STD_NVME_CMD_TYPE_NON_DATA: u32 = 2;

impl NvmeDevice {
    pub fn nvme_send_vsc2_passthrough_command(
        &self,
        sub_opcode: u32, // Adjust type if necessary
        direction: u8,
        p_param_buf: &mut [u8],
        p_data_buf: &mut [u8],
        p_completion_dw0: Option<&mut u32>,
        nsid: u32, // Adjust type if necessary
    ) -> io::Result<NVME_COMMAND_STATUS> {
        let mut default_completion_dw0 = 0;
        let mut completion_dw0 = p_completion_dw0.unwrap_or(&mut default_completion_dw0);

        let mut nc = NVME_COMMAND::default();
        nc.CDW0.set_OPC(NvmeVscOpcode::Write as u32);
        nc.u.GENERAL.CDW10 = p_param_buf.len() as u32 / size_of::<u32>() as u32;
        nc.u.GENERAL.CDW11 = 0;
        nc.u.GENERAL.CDW12 =
            self.set_vsc_op_code_by_project_type(self.project_type.clone(), sub_opcode); // Adjust return type and function call if necessary
        nc.u.GENERAL.CDW13 = 0;
        nc.u.GENERAL.CDW14 = 0;
        nc.NSID = nsid;

        let result = self.nvme_send_passthrough_command(
            NvmeOpcodeType::WRITE as u8,
            &nc,
            p_param_buf,
            completion_dw0,
        );
        let ncs = match result {
            Ok(ncs) => ncs,
            Err(e) => return Err(e),
        };
        if direction == 0
            || ncs.SCT() != NVME_STATUS_TYPES::NVME_STATUS_TYPE_GENERIC_COMMAND as u16
            || ncs.SC() != NVME_STATUS_GENERIC_COMMAND_CODES::NVME_STATUS_SUCCESS_COMPLETION as u16
        {
            return result;
        }

        // Data phase
        nc.CDW0
            .set_OPC(NvmeVscOpcode::None as u32 | direction as u32);
        nc.u.GENERAL.CDW10 = p_data_buf.len() as u32 / size_of::<u32>() as u32;
        nc.u.GENERAL.CDW11 = 0;
        nc.u.GENERAL.CDW12 =
            self.set_vsc_op_code_by_project_type(self.project_type.clone(), sub_opcode); // Adjust return type and function call if necessary
        nc.u.GENERAL.CDW13 = 0;
        nc.u.GENERAL.CDW14 = 1; // Phase ID

        self.nvme_send_passthrough_command(
            NvmeOpcodeType::NOBUFFER as u8 | direction,
            &nc,
            p_data_buf,
            completion_dw0,
        )
    }

    pub fn nvme_send_vsc_admin_passthrough_command(
        &self,
        p_nc_admin: &NVME_COMMAND,
        p_data_buf: Option<&mut [u8]>,
        p_completion_dw0: Option<&mut u32>,
    ) -> io::Result<NVME_COMMAND_STATUS> {
        let mut opflag = (p_nc_admin.CDW0.OPC() as u8) & 3;
        if p_data_buf.is_none() {
            opflag = 0;
        }
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

        self.nvme_send_vsc2_passthrough_command(
            sub_opcode,
            opflag,
            &mut param_buffer,
            p_data_buf.unwrap_or(&mut []),
            p_completion_dw0,
            0, // Default NSID, adjust if necessary
        )
    }

    fn set_vsc_op_code_by_project_type(&self, project_type: String, sub_opcode: u32) -> u32 {
        // Implement logic to determine sub-opcode based on project type
        sub_opcode
    }
}

pub fn print_nvme_identify_controller_data(data: &NVME_IDENTIFY_CONTROLLER_DATA) {
    println!("{:<12} : 0x{:04X}", "vid", data.VID);
    println!("{:<12} : 0x{:04X}", "ssvid", data.SSVID);
    println!("{:<12} : {}", "sn", String::from_utf8_lossy(&data.SN));
    println!("{:<12} : {}", "mn", String::from_utf8_lossy(&data.MN));
    println!("{:<12} : {}", "fr", String::from_utf8_lossy(&data.FR));
    println!("{:<12} : {}", "rab", data.RAB);
    println!("{:<12} : {:?}", "ieee", &data.IEEE);
    println!("{:<12} : {:?}", "cmic", data.CMIC);
    println!("{:<12} : {}", "mdts", data.MDTS);
    println!("{:<12} : {}", "cntlid", data.CNTLID);
    println!("{:<12} : 0x{:08X}", "ver", data.VER);
    println!("{:<12} : {}", "rtd3r", data.RTD3R);
    println!("{:<12} : {}", "rtd3e", data.RTD3E);
    println!("{:<12} : {:?}", "oaes", data.OAES);
    println!("{:<12} : {:?}", "ctratt", data.CTRATT);
    println!("{:<12} : {:?}", "rrls", data.RRLS);
    println!("{:<12} : {}", "cntltype", data.CNTRLTYPE);
    println!("{:<12} : {:?}", "fguid", &data.FGUID);
    println!("{:<12} : {}", "crdt1", data.CRDT1);
    println!("{:<12} : {}", "crdt2", data.CRDT2);
    println!("{:<12} : {}", "crdt3", data.CRDT3);
    println!("{:<12} : {:?}", "oacs", data.OACS);
    println!("{:<12} : {}", "acl", data.ACL);
    println!("{:<12} : {}", "aerl", data.AERL);
    println!("{:<12} : {:?}", "frmw", data.FRMW);
    println!("{:<12} : {:?}", "lpa", data.LPA);
    println!("{:<12} : {}", "elpe", data.ELPE);
    println!("{:<12} : {}", "npss", data.NPSS);
    println!("{:<12} : {:?}", "avscp", data.AVSCC);
    println!("{:<12} : {:?}", "apsta", data.APSTA);
    println!("{:<12} : {}", "wctemp", data.WCTEMP);
    println!("{:<12} : {}", "cctemp", data.CCTEMP);
    println!("{:<12} : {}", "mtfa", data.MTFA);
    println!("{:<12} : {}", "hmpre", data.HMPRE);
    println!("{:<12} : {}", "hmmin", data.HMMIN);
    println!("{:<12} : {:?}", "tnvmcap", &data.TNVMCAP);
    println!("{:<12} : {:?}", "unvmcap", &data.UNVMCAP);
    println!("{:<12} : {:?}", "rpmbs", data.RPMBS);
    println!("{:<12} : {}", "edstt", data.EDSTT);
    println!("{:<12} : {}", "dsto", data.DSTO);
    println!("{:<12} : {}", "fwug", data.FWUG);
    println!("{:<12} : {}", "kas", data.KAS);
    println!("{:<12} : {:?}", "hctma", data.HCTMA);
    println!("{:<12} : {}", "mntmt", data.MNTMT);
    println!("{:<12} : {}", "mxtmt", data.MXTMT);
    println!("{:<12} : {:?}", "sanicap", data.SANICAP);
    println!("{:<12} : {}", "hmminds", data.HMMINDS);
    println!("{:<12} : {}", "hmmaxd", data.HMMAXD);
    println!("{:<12} : {}", "nsetidmax", data.NSETIDMAX);
    println!("{:<12} : {}", "endgidmax", data.ENDGIDMAX);
    println!("{:<12} : {}", "anatt", data.ANATT);
    println!("{:<12} : {:?}", "anacap", data.ANACAP);
    println!("{:<12} : {}", "anagrpmax", data.ANAGRPMAX);
    println!("{:<12} : {}", "nanagrpid", data.NANAGRPID);
    println!("{:<12} : {}", "pels", data.PELS);
    println!("{:<12} : {:?}", "sqes", data.SQES);
    println!("{:<12} : {:?}", "cqes", data.CQES);
    println!("{:<12} : {}", "maxcmd", data.MAXCMD);
    println!("{:<12} : {}", "nn", data.NN);
    println!("{:<12} : {:?}", "oncs", data.ONCS);
    println!("{:<12} : {:?}", "fuses", data.FUSES);
    println!("{:<12} : {:?}", "fna", data.FNA);
    println!("{:<12} : {:?}", "vwc", data.VWC);
    println!("{:<12} : {}", "awun", data.AWUN);
    println!("{:<12} : {}", "awupf", data.AWUPF);
    println!("{:<12} : {:?}", "nvscss", data.NVSCC);
    println!("{:<12} : {:?}", "nwpc", data.NWPC);
    println!("{:<12} : {}", "acwu", data.ACWU);
    println!("{:<12} : {:?}", "sgls", data.SGLS);
    println!("{:<12} : {}", "mnan", data.MNAN);
    println!(
        "{:<12} : {}",
        "subnqn",
        String::from_utf8_lossy(&data.SUBNQN)
    );
    // Power State Descriptors are not printed here for brevity.
    // Vendor Specific fields are also not printed here for brevity.
}
