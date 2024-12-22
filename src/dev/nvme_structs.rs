use windows_sys::Win32::System::Ioctl::*;

#[repr(C)]
#[derive(Debug)]
pub struct NVME_ERROR_INFO_LOG {
    pub error_count: u64,
    pub sqid: u16,
    pub cmdid: u16,
    pub status_field: u16,
    pub parameter_error_location: u16,
    pub lba: u64,
    pub nsid: u32,
    pub vs: u8,
    pub reserved: [u8; 35],
}

impl Default for NVME_ERROR_INFO_LOG {
    fn default() -> Self {
        NVME_ERROR_INFO_LOG {
            error_count: 0,
            sqid: 0,
            cmdid: 0,
            status_field: 0,
            parameter_error_location: 0,
            lba: 0,
            nsid: 0,
            vs: 0,
            reserved: [0; 35],
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct NvmeCommand {
    pub opcode: u8,
    pub flags: u8,
    pub nsid: u32,
    pub cdw2: u32,
    pub cdw3: u32,
    pub metadata: u64,
    pub prp1: u64,
    pub prp2: u64,
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct NVME_HEALTH_INFO_LOG {
    pub temperature: [u8; 2],
    // Add other fields as necessary
}

#[repr(C)]
#[derive(Debug)]
pub struct NvmeIdentifyControllerData {
    pub vid: u16,
    pub ssvid: u16,
    pub sn: [u8; 20],
    pub mn: [u8; 40],
    pub fr: [u8; 8],
    pub rab: u8,
    pub ieee: [u8; 3],
    pub cmic: u8,
    pub mdts: u8,
    pub cntlid: u16,
    pub ver: u32,
    pub rtd3r: u32,
    pub rtd3e: u32,
    pub oaes: u32,
    pub ctratt: u32,
    pub reserved: [u8; 156],
    pub nn: u32,
    pub reserved2: [u8; 4],
    pub fguid: [u8; 16],
    pub reserved3: [u8; 112],
    pub reserved4: [u8; 896],
    pub reserved5: [u8; 256],
}

impl Default for NvmeIdentifyControllerData {
    fn default() -> Self {
        NvmeIdentifyControllerData {
            vid: 0,
            ssvid: 0,
            sn: [0; 20],
            mn: [0; 40],
            fr: [0; 8],
            rab: 0,
            ieee: [0; 3],
            cmic: 0,
            mdts: 0,
            cntlid: 0,
            ver: 0,
            rtd3r: 0,
            rtd3e: 0,
            oaes: 0,
            ctratt: 0,
            reserved: [0; 156],
            nn: 0,
            reserved2: [0; 4],
            fguid: [0; 16],
            reserved3: [0; 112],
            reserved4: [0; 896],
            reserved5: [0; 256],
        }
    }
}

pub const NVME_IDENTIFY_CNS_CONTROLLER: u32 = 1;
pub const NVME_LOG_PAGE_HEALTH_INFO: u32 = 2;
pub const NVME_MAX_LOG_SIZE: usize = 4096;
pub const NVME_FEATURE_HOST_CONTROLLED_THERMAL_MANAGEMENT: u32 = 0x10;
pub const NVME_FEATURE_TEMPERATURE_THRESHOLD: u32 = 0x11;
