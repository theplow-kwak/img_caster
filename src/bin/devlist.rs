extern crate winapi;

use winapi::shared::devguid::GUID_DEVCLASS_DISKDRIVE;
use winapi::shared::ntdef::NULL;
use winapi::um::cfgmgr32::{
    CM_Get_Device_IDA, CM_Get_Parent, CM_Locate_DevNodeA, CM_Locate_DevNode_ExA, CR_SUCCESS,
};
use winapi::um::setupapi::{DIGCF_ALLCLASSES, HDEVINFO, SPDRP_DEVICEDESC};

fn main() {
    unsafe {
        let mut device_info_set: HDEVINFO = winapi::um::setupapi::SetupDiGetClassDevsA(
            &GUID_DEVCLASS_DISKDRIVE,
            NULL as *const i8,
            NULL as *mut winapi::shared::windef::HWND__,
            DIGCF_ALLCLASSES,
        );

        if device_info_set.is_null() {
            println!("SetupDiGetClassDevs failed");
            return;
        }

        let mut dev_info_data: winapi::um::setupapi::SP_DEVINFO_DATA = std::mem::zeroed();
        dev_info_data.cbSize = std::mem::size_of::<winapi::um::setupapi::SP_DEVINFO_DATA>() as u32;

        let mut index: u32 = 0;
        let mut device_id: [u8; 1000] = [0; 1000];

        while winapi::um::setupapi::SetupDiEnumDeviceInfo(
            device_info_set,
            index,
            &mut dev_info_data,
        ) != 0
        {
            if winapi::um::setupapi::SetupDiGetDeviceInstanceIdA(
                device_info_set,
                &mut dev_info_data,
                device_id.as_mut_ptr() as *mut i8,
                1000,
                NULL as *mut u32,
            ) != 0
            {
                let device_id_str =
                    std::ffi::CStr::from_ptr(device_id.as_ptr() as *const i8).to_string_lossy();
                println!("Device ID: {}", device_id_str);

                let mut parent: winapi::um::cfgmgr32::DEVINST = std::mem::zeroed();
                if winapi::um::cfgmgr32::CM_Get_Parent(&mut parent, dev_info_data.DevInst, 0)
                    == CR_SUCCESS
                {
                    let mut device_id_parent: [u8; 1000] = [0; 1000];
                    if winapi::um::cfgmgr32::CM_Get_Device_IDA(
                        parent,
                        device_id_parent.as_mut_ptr() as *mut i8,
                        1000,
                        0,
                    ) == CR_SUCCESS
                    {
                        let device_id_parent_str =
                            std::ffi::CStr::from_ptr(device_id_parent.as_ptr() as *const i8)
                                .to_string_lossy();
                        println!("Parent Device ID: {}", device_id_parent_str);
                    }
                }
            }

            index += 1;
        }

        winapi::um::setupapi::SetupDiDestroyDeviceInfoList(device_info_set);
    }
}
