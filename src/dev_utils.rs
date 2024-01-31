use std::{
    ffi::c_void,
    mem::{size_of, size_of_val, zeroed},
    ptr::null_mut,
};

use windows_sys::{
    core::*, Win32::Devices::DeviceAndDriverInstallation::*, Win32::Foundation::*,
    Win32::System::Ioctl::*,
};

use crate::disk::{ioctl, last_error, open};

pub fn enum_dev() {
    unsafe {
        let h_dev_info: HDEVINFO = SetupDiGetClassDevsA(
            null_mut(),
            null_mut(),
            0 as HWND,
            DIGCF_ALLCLASSES | DIGCF_DEVICEINTERFACE,
        );

        if h_dev_info == INVALID_HANDLE_VALUE {
            println!("SetupDiGetClassDevs failed");
            return;
        }

        let guid: *const GUID = &GUID_DEVINTERFACE_STORAGEPORT;
        let mut device_info_data: SP_DEVINFO_DATA = zeroed();
        let mut device_interface_data: SP_DEVICE_INTERFACE_DATA = zeroed();
        device_info_data.cbSize = size_of::<SP_DEVINFO_DATA>() as u32;
        device_interface_data.cbSize = size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;

        let mut index: u32 = 0;
        let mut device_id: [u8; 1024] = [0; 1024];
        let device_interface_detail_data: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A =
            device_id.as_mut_ptr() as *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A;
        (*device_interface_detail_data).cbSize = size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_A>() as u32;

        while SetupDiEnumDeviceInterfaces(
            h_dev_info,
            null_mut(),
            guid,
            index,
            &mut device_interface_data,
        ) != 0
        {
            if SetupDiGetDeviceInterfaceDetailA(
                h_dev_info,
                &mut device_interface_data,
                device_interface_detail_data,
                1024,
                null_mut(),
                &mut device_info_data,
            ) != 0
            {
                let device_path = std::ffi::CStr::from_ptr(
                    (*device_interface_detail_data).DevicePath.as_ptr() as *const i8,
                )
                .to_string_lossy();

                println!("enum Device path: {:?}", device_path);
                let _ = get_dev_inst_interfaces(device_info_data.DevInst);

                let mut parent = std::mem::zeroed();
                if CM_Get_Parent(&mut parent, device_info_data.DevInst, 0) == CR_SUCCESS {
                    let mut device_id_parent: [u8; 1000] = [0; 1000];
                    if CM_Get_Device_IDA(parent, device_id_parent.as_mut_ptr() as *mut u8, 1000, 0)
                        == CR_SUCCESS
                    {
                        let device_id_parent_str =
                            std::ffi::CStr::from_ptr(device_id_parent.as_ptr() as *const i8)
                                .to_string_lossy();
                        println!("Parent Device ID: {}", device_id_parent_str);
                        let _ = get_dev_inst_interfaces(parent);
                    }
                }
            }

            index += 1;
        }

        SetupDiDestroyDeviceInfoList(h_dev_info);
    }
}

pub fn get_dev_inst_interfaces(dev: u32) -> Result<Box<str>, &'static str> {
    unsafe {
        let mut device_id: [u8; 1000] = [0; 1000];
        if CM_Get_Device_IDA(dev, device_id.as_mut_ptr() as *mut u8, 1000, 0) != CR_SUCCESS {
            return Err("CM_Get_Device_IDA fail");
        }

        let guid: *const GUID = &GUID_DEVINTERFACE_STORAGEPORT;
        // Get list size
        let mut iface_list_size = [0u32; 1];
        if CM_Get_Device_Interface_List_SizeA(
            iface_list_size.as_mut_ptr() as *mut u32,
            guid,
            null_mut(),
            CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
        ) != CR_SUCCESS
        {
            return Err("CM_Get_Device_Interface_List_SizeA fail");
        }

        let device_id_str =
            std::ffi::CStr::from_ptr(device_id.as_ptr() as *const i8).to_string_lossy();

        let mut iface_list: [u8; 1000] = [0; 1000];
        if CM_Get_Device_Interface_ListA(
            guid,
            device_id.as_mut_ptr() as *mut u8,
            iface_list.as_mut_ptr() as *mut u8,
            *iface_list_size.as_mut_ptr(),
            CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
        ) != CR_SUCCESS
        {
            return Err("CM_Get_Device_Interface_ListA fail");
        }
        let iface_list_str =
            std::ffi::CStr::from_ptr(iface_list.as_ptr() as *const i8).to_string_lossy();
        println!(
            "GetDevInstInterfaces {} ({}) -> {:?}, {}\n",
            dev, device_id_str, iface_list_size, iface_list_str
        );

        return Ok(iface_list_str.into());
    }
}

pub fn get_parent_dev_path(driveno: u32) -> Result<Box<str>, &'static str> {
    if let Ok(devinst) = get_drives_dev_inst_by_disk_number(driveno) {
        unsafe {
            let mut parent = std::mem::zeroed();
            if CM_Get_Parent(&mut parent, devinst, 0) == CR_SUCCESS {
                let mut device_id_parent: [u8; 1000] = [0; 1000];
                if CM_Get_Device_IDA(parent, device_id_parent.as_mut_ptr() as *mut u8, 1000, 0)
                    == CR_SUCCESS
                {
                    let device_id_parent_str =
                        std::ffi::CStr::from_ptr(device_id_parent.as_ptr() as *const i8)
                            .to_string_lossy();
                    println!("Parent Device ID: {}", device_id_parent_str);
                }
                return get_dev_inst_interfaces(parent);
            }
        }
    }
    return Err("Can't get parent's path");
}

pub fn get_drives_dev_inst_by_disk_number(disk_number: u32) -> Result<u32, &'static str> {
    unsafe {
        let guid: *const GUID = &GUID_DEVINTERFACE_DISK;
        let h_dev_info: HDEVINFO = SetupDiGetClassDevsA(
            guid,
            null_mut(),
            0 as HWND,
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
        );

        if h_dev_info == INVALID_HANDLE_VALUE {
            return Err("SetupDiGetClassDevs failed");
        }

        let mut device_info_data: SP_DEVINFO_DATA = zeroed();
        let mut device_interface_data: SP_DEVICE_INTERFACE_DATA = zeroed();
        device_info_data.cbSize = size_of::<SP_DEVINFO_DATA>() as u32;
        device_interface_data.cbSize = size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;

        let mut index: u32 = 0;
        let mut device_id: [u8; 1024] = [0; 1024];
        let device_interface_detail_data: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A =
            device_id.as_mut_ptr() as *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A;
        (*device_interface_detail_data).cbSize = size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_A>() as u32;

        while SetupDiEnumDeviceInterfaces(
            h_dev_info,
            null_mut(),
            guid,
            index,
            &mut device_interface_data,
        ) != 0
        {
            if SetupDiGetDeviceInterfaceDetailA(
                h_dev_info,
                &mut device_interface_data,
                device_interface_detail_data,
                1024,
                null_mut(),
                &mut device_info_data,
            ) != 0
            {
                let device_path = std::ffi::CStr::from_ptr(
                    (*device_interface_detail_data).DevicePath.as_ptr() as *const i8,
                )
                .to_str()
                .unwrap()
                .to_owned();
                println!("Device path: {:?}", device_path);
                let handle = open(&device_path, 'r');
                if handle != INVALID_HANDLE_VALUE {
                    let mut sdn: STORAGE_DEVICE_NUMBER = zeroed();
                    let ret = ioctl(
                        handle,
                        IOCTL_STORAGE_GET_DEVICE_NUMBER,
                        Some((null_mut(), 0)),
                        Some((&mut sdn as *mut _ as *mut c_void, size_of_val(&sdn))),
                    );
                    CloseHandle(handle);
                    match ret {
                        Err(_err) => {
                            println!("DeviceIoControl is failed(Err Code: {})", last_error());
                        }
                        Ok(_ret) => {
                            if disk_number == sdn.DeviceNumber {
                                SetupDiDestroyDeviceInfoList(h_dev_info);
                                return Ok(device_info_data.DevInst);
                            }
                        }
                    }
                }
            }
            index += 1;
        }

        SetupDiDestroyDeviceInfoList(h_dev_info);

        return Err("Can not found devide instance!!");
    }
}
