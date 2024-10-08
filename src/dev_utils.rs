use crate::disk::{ioctl, last_error, open};
use sscanf;
use std::{
    ffi::c_void,
    mem::{size_of, size_of_val, zeroed},
    ptr::null_mut,
};
use windows_sys::{
    core::*,
    Win32::{
        Devices::{DeviceAndDriverInstallation::*, Properties::*},
        Foundation::*,
        System::Ioctl::*,
    },
};

pub fn enum_dev() {
    unsafe {
        let guid: *const GUID = &GUID_DEVINTERFACE_STORAGEPORT;
        let h_dev_info: HDEVINFO = SetupDiGetClassDevsA(
            guid,
            null_mut(),
            0 as HWND,
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
        );

        if h_dev_info == INVALID_HANDLE_VALUE {
            println!("SetupDiGetClassDevs failed");
            return;
        }

        let mut device_info_data: SP_DEVINFO_DATA = zeroed();
        let mut device_interface_data: SP_DEVICE_INTERFACE_DATA = zeroed();
        device_info_data.cbSize = size_of::<SP_DEVINFO_DATA>() as u32;
        device_interface_data.cbSize = size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;

        let mut index: u32 = 0;
        let mut device_id: Vec<u8> = vec![0; 1024];
        let device_interface_detail_data: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A =
            device_id.as_mut_ptr() as *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A;
        (*device_interface_detail_data).cbSize =
            size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_A>() as u32;

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
                get_dev_inst_interfaces(device_info_data.DevInst).unwrap();

                let mut parent = std::mem::zeroed();
                if CM_Get_Parent(&mut parent, device_info_data.DevInst, 0) == CR_SUCCESS {
                    let mut device_id_parent: Vec<u8> = vec![0; 1000];
                    if CM_Get_Device_IDA(parent, device_id_parent.as_mut_ptr() as *mut u8, 1000, 0)
                        == CR_SUCCESS
                    {
                        let device_id_parent_str =
                            std::ffi::CStr::from_ptr(device_id_parent.as_ptr() as *const i8)
                                .to_string_lossy();
                        println!("Parent Device ID: {}", device_id_parent_str);
                        get_dev_inst_interfaces(parent).unwrap();
                    }
                }
            }

            index += 1;
        }

        SetupDiDestroyDeviceInfoList(h_dev_info);
    }
}

pub fn get_dev_inst_interfaces(devinst: u32) -> Result<Box<str>, &'static str> {
    unsafe {
        let mut device_id: Vec<u8> = vec![0; 1000];
        if CM_Get_Device_IDA(devinst, device_id.as_mut_ptr() as *mut u8, 1000, 0) != CR_SUCCESS {
            return Err("CM_Get_Device_IDA fail");
        }

        let guid: *const GUID = &GUID_DEVINTERFACE_STORAGEPORT;
        // Get list size
        let mut iface_list_size: u32 = 0;
        if CM_Get_Device_Interface_List_SizeA(
            &mut iface_list_size,
            guid,
            device_id.as_mut_ptr() as *mut u8,
            CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
        ) != CR_SUCCESS
        {
            return Err("CM_Get_Device_Interface_List_SizeA fail");
        }

        let mut iface_list: Vec<u8> = vec![0; iface_list_size as usize];
        if CM_Get_Device_Interface_ListA(
            guid,
            device_id.as_mut_ptr() as *mut u8,
            iface_list.as_mut_ptr() as *mut u8,
            iface_list_size,
            CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
        ) != CR_SUCCESS
        {
            return Err("CM_Get_Device_Interface_ListA fail");
        }

        let device_id_str =
            std::ffi::CStr::from_ptr(device_id.as_ptr() as *const i8).to_string_lossy();
        let iface_list_str =
            std::ffi::CStr::from_ptr(iface_list.as_ptr() as *const i8).to_string_lossy();
        println!(
            "GetDevInstInterfaces {} ({}) -> {:?}, {}\n",
            devinst, device_id_str, iface_list_size, iface_list_str
        );

        return Ok(iface_list_str.into());
    }
}

pub fn get_parent_dev_path(driveno: u32) -> Result<Box<str>, &'static str> {
    if let Ok(devinst) = get_drives_dev_inst_by_disk_number(driveno) {
        unsafe {
            let mut parent = std::mem::zeroed();
            if CM_Get_Parent(&mut parent, devinst, 0) == CR_SUCCESS {
                let mut device_id_parent: Vec<u8> = vec![0; 1000];
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
        let mut device_id: Vec<u8> = vec![0; 1024];
        let device_interface_detail_data: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A =
            device_id.as_mut_ptr() as *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A;
        (*device_interface_detail_data).cbSize =
            size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_A>() as u32;

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

pub fn get_drives_dev_inst_by_bus_number(bus_number: i32) -> Result<Box<str>, &'static str> {
    unsafe {
        let guid: *const GUID = &GUID_DEVINTERFACE_STORAGEPORT;
        let h_dev_info: HDEVINFO = SetupDiGetClassDevsA(
            guid,
            null_mut(),
            0 as HWND,
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
        );

        if h_dev_info == INVALID_HANDLE_VALUE {
            return Err("SetupDiGetClassDevs failed");
        }

        let mut index: u32 = 0;
        let mut device_info_data: SP_DEVINFO_DATA = zeroed();
        let mut device_interface_data: SP_DEVICE_INTERFACE_DATA = zeroed();
        let mut device_interface_detail_data: Vec<u8> = vec![0; 1024];
        let device_interface_detail_data =
            device_interface_detail_data.as_mut_ptr() as *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A;
        device_info_data.cbSize = size_of::<SP_DEVINFO_DATA>() as u32;
        device_interface_data.cbSize = size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;
        (*device_interface_detail_data).cbSize =
            size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_A>() as u32;

        while SetupDiEnumDeviceInterfaces(
            h_dev_info,
            null_mut(),
            guid,
            index,
            &mut device_interface_data,
        ) != 0
        {
            index += 1;
            if SetupDiGetDeviceInterfaceDetailA(
                h_dev_info,
                &mut device_interface_data,
                device_interface_detail_data,
                1024,
                null_mut(),
                &mut device_info_data,
            ) != 0
            {
                let mut deviceinstanceid = vec![0; 260];
                SetupDiGetDeviceInstanceIdA(
                    h_dev_info,
                    &device_info_data,
                    deviceinstanceid.as_mut_ptr(),
                    deviceinstanceid.len() as u32,
                    std::ptr::null_mut(),
                );
                let hdev_inst = device_info_data.DevInst;

                let mut pulstatus = 0;
                let mut pulproblemnumber = 0;
                let cr = CM_Get_DevNode_Status(&mut pulstatus, &mut pulproblemnumber, hdev_inst, 0);
                if cr != CR_SUCCESS {
                    continue;
                }

                let mut buffer = vec![0; 260];
                let mut buffer_len: u32 = buffer.len() as u32;
                CM_Get_DevNode_Registry_PropertyA(
                    hdev_inst,
                    CM_DRP_LOCATION_INFORMATION,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut c_void,
                    &mut buffer_len,
                    0,
                );
                let location_info = String::from_utf8_lossy(&buffer[..(buffer_len - 1) as usize]);
                let bdf =
                    sscanf::sscanf!(location_info, "PCI bus {i32}, device {i32}, function {i32}")
                        .unwrap_or((0, 0, 0));

                let device_path = std::ffi::CStr::from_ptr(
                    (*device_interface_detail_data).DevicePath.as_ptr() as *const i8,
                )
                .to_string_lossy();
                println!(
                    "Device path: {:?} inst {} buffer {} {:?}",
                    device_path,
                    hdev_inst,
                    std::str::from_utf8(&buffer).unwrap(),
                    bdf
                );

                if bdf == (bus_number, 0, 0) {
                    SetupDiDestroyDeviceInfoList(h_dev_info);
                    return Ok(device_path.into());
                }
            }
        }

        SetupDiDestroyDeviceInfoList(h_dev_info);

        return Err("Can not found devide instance!!");
    }
}

pub fn enum_dev_interfaces() -> Result<Box<str>, &'static str> {
    unsafe {
        let guid: *const GUID = &GUID_DEVINTERFACE_STORAGEPORT;
        let mut iface_list_size: u32 = 0;
        if CM_Get_Device_Interface_List_SizeW(
            &mut iface_list_size,
            guid,
            null_mut(),
            CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
        ) != CR_SUCCESS
        {
            return Err("CM_Get_Device_Interface_List_SizeA fail");
        }

        let mut iface_list: Vec<u16> = vec![0; iface_list_size as usize];
        if CM_Get_Device_Interface_ListW(
            guid,
            null_mut(),
            iface_list.as_mut_ptr() as *mut u16,
            iface_list_size,
            CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
        ) != CR_SUCCESS
        {
            return Err("CM_Get_Device_Interface_ListA fail");
        }

        println!("GetDevInstInterfaces {:?}:", iface_list_size);
        let interfaces: Vec<_> = iface_list
            .split(|&e| e == 0x0)
            .filter(|v| !v.is_empty())
            .collect();

        let mut devinst = 0;
        let mut propertytype: DEVPROPTYPE = 0;
        for interface in interfaces {
            let iface_list_str = String::from_utf16_lossy(interface);
            // println!("{} {:?}", devinst, iface_list_str);
            let mut current_device: Vec<u16> = vec![0; 1000];
            let mut device_id_size: u32 = 1000;
            let cr = CM_Get_Device_Interface_PropertyW(
                interface.as_ptr(),
                &DEVPKEY_Device_InstanceId,
                &mut propertytype,
                current_device.as_mut_ptr() as *mut u8,
                &mut device_id_size,
                0,
            );
            if cr != CR_SUCCESS {
                continue;
            }
            if propertytype != DEVPROP_TYPE_STRING {
                continue;
            }

            let cr = CM_Locate_DevNodeW(
                &mut devinst,
                current_device.as_ptr(),
                CM_LOCATE_DEVNODE_NORMAL,
            );
            if cr != CR_SUCCESS {
                continue;
            }

            let mut buffer: Vec<u16> = vec![0; 260];
            let mut buffer_len: u32 = buffer.len() as u32;
            let cr = CM_Get_DevNode_PropertyW(
                devinst,
                &DEVPKEY_Device_LocationInfo,
                &mut propertytype,
                buffer.as_mut_ptr() as *mut u8,
                &mut buffer_len,
                0,
            );
            if cr != CR_SUCCESS {
                continue;
            }
            let strip = buffer.split(|&num| num == 0).next().unwrap();
            let location_info = String::from_utf16_lossy(strip);
            let bdf = sscanf::sscanf!(location_info, "PCI bus {i32}, device {i32}, function {i32}")
                .unwrap_or((0, 0, 0));

            println!(
                "{} {}\n    {} - {:?}\n    {}",
                devinst,
                iface_list_str,
                String::from_utf16_lossy(&current_device),
                bdf,
                location_info
            );
        }

        return Ok("f".into());
    }
}
