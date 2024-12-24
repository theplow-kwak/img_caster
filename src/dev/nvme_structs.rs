use serde::{Deserialize, Serialize};
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

pub const NVME_IDENTIFY_CNS_CONTROLLER: u32 = 1;
pub const NVME_LOG_PAGE_HEALTH_INFO: u32 = 2;
pub const NVME_IDENTIFY_SIZE: usize = 4096;
pub const NVME_MAX_LOG_SIZE: usize = 4096;
pub const NVME_FEATURE_HOST_CONTROLLED_THERMAL_MANAGEMENT: u32 = 0x10;
pub const NVME_FEATURE_TEMPERATURE_THRESHOLD: u32 = 0x11;
pub const NVME_FEATURE_VOLATILE_WRITE_CACHE: u32 = 0x0C;

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
    pub mic: u8,
    pub mdts: u8,
    pub reserved89: [u8; 159],
    pub oacs: u16,
    pub acl: u8,
    pub aerl: u8,
    pub frmw: u8,
    pub lpa: u8,
    pub elpe: u8,
    pub npss: u8,
    pub avscc: u8,
    pub apsta: u8,
    pub wctemp: u8,
    pub cctemp: u8,
    pub mtfa: u16,
    pub hmpre: u8,
    pub hmmin: u8,
    pub tnvmcap: [u8; 16],
    pub unvmcap: [u8; 16],
    pub rpmbs: u32,
    pub edstt: u8,
    pub dsto: u8,
    pub fwug: u8,
    pub ksug: u8,
    pub hctma: u8,
    pub mntmt: u8,
    pub mxtmt: u8,
    pub sanic: u8,
    pub hmminds: u8,
    pub hmmaxd: u8,
    pub nsetidmax: u8,
    pub endgidmax: u16,
    pub anchbak: u32,
    pub rgs: u32,
    pub reserved192: [u8; 40],
    pub nn: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct NVME_HEALTH_INFO_LOG {
    pub critical_warning: u8,
    pub temperature: [u8; 2],
    pub available_spare: u8,
    pub available_spare_threshold: u8,
    pub percentage_used: u8,
    pub reserved81: [u8; 155],
    pub data_units_read: [u8; 16],
    pub data_units_written: [u8; 16],
    pub host_read_commands: [u8; 16],
    pub host_write_commands: [u8; 16],
    pub controller_busy_time: [u8; 16],
    pub power_cycles: [u8; 16],
    pub power_on_hours: [u8; 16],
    pub unsafe_shutdowns: [u8; 16],
    pub media_errors: [u8; 16],
    pub num_err_log_entries: [u8; 16],
}
