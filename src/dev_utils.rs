use crate::disk::{ioctl, last_error, open};
use sscanf;
use std::{
    ffi::c_void,
    fmt,
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

#[derive(Default)]
struct PciBdf {
    segment: i32,
    bus: i32,
    device: i32,
    function: i32,
}

impl PciBdf {
    pub fn parse(location_info: &str) -> Option<Self> {
        sscanf::sscanf!(location_info, "PCI bus {i32}, device {i32}, function {i32}")
            .ok()
            .map(|(bus, device, function)| Self {
                segment: 0,
                bus,
                device,
                function,
            })
    }
}

impl PartialEq for PciBdf {
    fn eq(&self, other: &Self) -> bool {
        self.segment == other.segment
            && self.bus == other.bus
            && self.device == other.device
            && self.function == other.function
    }
}

impl fmt::Debug for PciBdf {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "PCI bus {:02x}, device {:02x}, function {:02x} (Segment: {})",
            self.bus, self.device, self.function, self.segment
        )
    }
}

impl fmt::Display for PciBdf {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{:04X}:{:02X}:{:02X}:{:02X}",
            self.segment, self.bus, self.device, self.function
        )
    }
}

#[derive(Debug)]
struct DevInstance {
    devinst: u32,
}

impl DevInstance {
    pub fn new(devinst: u32) -> Option<Self> {
        let mut status: u32 = 0;
        let mut problem: u32 = 0;
        if unsafe { CM_Get_DevNode_Status(&mut status, &mut problem, devinst, 0) } == CR_SUCCESS {
            Some(Self { devinst })
        } else {
            None
        }
    }

    pub fn get_device_property(&self, property_key: *const DEVPROPKEY) -> Option<Vec<u16>> {
        let mut buffer: Vec<u16> = vec![0; 260];
        let mut buffer_len: u32 = buffer.len() as u32;
        let mut property_type: u32 = 0; // assuming propertytype is a u32 (adjust if needed)

        if unsafe {
            CM_Get_DevNode_PropertyW(
                self.devinst,
                property_key,
                &mut property_type,
                buffer.as_mut_ptr() as *mut u8,
                &mut buffer_len,
                0,
            )
        } != CR_SUCCESS
        {
            return None;
        }

        let trimed: Vec<u16> = buffer.into_iter().take_while(|&c| c != 0).collect();
        Some(trimed)
    }

    pub fn service(&self) -> Option<String> {
        if let Some(trimed) = self.get_device_property(&DEVPKEY_Device_Service) {
            return Some(String::from_utf16_lossy(&trimed));
        }
        None
    }

    pub fn location_info(&self) -> Option<String> {
        if let Some(trimed) = self.get_device_property(&DEVPKEY_Device_LocationInfo) {
            return Some(String::from_utf16_lossy(&trimed));
        }
        None
    }

    pub fn instance_id(&self) -> Option<Vec<u16>> {
        self.get_device_property(&DEVPKEY_Device_InstanceId)
    }

    pub fn value(&self) -> u32 {
        self.devinst
    }
}

impl fmt::Display for DevInstance {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:02}", self.devinst)
    }
}

#[derive(Debug)]
pub struct PhysicalDisk {
    devinst: DevInstance,
    device_path: String,
    disk_number: i32,
    nsid: i32,
}

impl PhysicalDisk {
    pub fn new(devinst: u32) -> Option<Self> {
        DevInstance::new(devinst).map(|devinst| Self {
            devinst,
            device_path: String::new(),
            disk_number: -1,
            nsid: -1,
        })
    }

    pub fn inspect(&mut self) -> &mut Self {
        if let Some(ref instance_path) = self.devinst.instance_id() {
            self.nsid = String::from_utf16_lossy(instance_path)
                .split('&')
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                + 1;
        }
        self
    }

    pub fn get_interface_path(&mut self) -> &mut Self {
        unsafe {
            if let Some(ref device_id) = self.devinst.instance_id() {
                let guid: *const GUID = &GUID_DEVINTERFACE_DISK;
                let mut iface_list_size: u32 = 0;
                let ret = CM_Get_Device_Interface_List_SizeW(
                    &mut iface_list_size,
                    guid,
                    device_id.as_ptr(),
                    CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
                );
                if ret != CR_SUCCESS {
                    println!(
                        "get size of {:?}: size {} ret {:?}",
                        device_id, iface_list_size, ret
                    );
                    return self;
                }

                let mut iface_list: Vec<u16> = vec![0; iface_list_size as usize];
                if CM_Get_Device_Interface_ListW(
                    guid,
                    device_id.as_ptr(),
                    iface_list.as_mut_ptr(),
                    iface_list_size,
                    CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
                ) != CR_SUCCESS
                {
                    return self;
                }

                let interface: Vec<u16> = iface_list.into_iter().take_while(|&c| c != 0).collect();
                self.device_path = String::from_utf16_lossy(&interface).to_string();
            }
        }
        self
    }

    pub fn get_device_number(&mut self) -> &mut Self {
        unsafe {
            let handle = open(&self.device_path, 'r');
            if handle != INVALID_HANDLE_VALUE {
                let mut sdn: STORAGE_DEVICE_NUMBER = zeroed();
                if let Ok(_ret) = ioctl(
                    handle,
                    IOCTL_STORAGE_GET_DEVICE_NUMBER,
                    Some((null_mut(), 0)),
                    Some((&mut sdn as *mut _ as *mut c_void, size_of_val(&sdn))),
                ) {
                    self.disk_number = sdn.DeviceNumber as i32;
                    CloseHandle(handle);
                }
            }
        }
        self
    }
}

impl fmt::Display for PhysicalDisk {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            " L Disk{:02} ({}): nsid {}, path {}",
            self.disk_number, self.devinst, self.nsid, self.device_path
        )
    }
}

#[derive(Debug)]
pub struct NvmeController {
    devinst: DevInstance,
    path: String,
    bdf: PciBdf,
    disks: Vec<PhysicalDisk>,
}

impl NvmeController {
    pub fn new(devinst: u32) -> Option<Self> {
        if let Some(devinst) = DevInstance::new(devinst) {
            match devinst.service() {
                Some(service) if service == "stornvme" => {
                    return Some(Self {
                        devinst,
                        path: String::new(),
                        bdf: Default::default(),
                        disks: vec![],
                    })
                }
                _ => return None,
            }
        }
        None
    }

    pub fn inspect(&mut self) -> &mut Self {
        if let Some(location_info) = self.devinst.location_info() {
            self.bdf = PciBdf::parse(&location_info).unwrap_or_default();
        }
        if let Some(ref instance_path) = self.devinst.instance_id() {
            self.path = String::from_utf16_lossy(instance_path);
        }
        self
    }

    pub fn enum_child_disks(&mut self) -> &mut Self {
        unsafe {
            let mut child = 0;
            let mut result = CM_Get_Child(&mut child, self.devinst.value(), 0);
            while result == CR_SUCCESS {
                if let Some(mut disk) = PhysicalDisk::new(child) {
                    disk.inspect().get_interface_path().get_device_number();
                    self.disks.push(disk);
                }
                result = CM_Get_Sibling(&mut child, child, 0);
            }
        }
        self
    }

    pub fn add_disk(&mut self, disk: PhysicalDisk) {
        self.disks.push(disk);
    }
}

impl fmt::Display for NvmeController {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "DiskController ({}) bdf {} - {}\n",
            self.devinst, self.bdf, self.path
        )?;
        for disk in &self.disks {
            write!(fmt, "{}\n", disk)?;
        }
        Ok(())
    }
}

pub struct NvmeControllerList {
    controllers: Vec<NvmeController>,
}

impl NvmeControllerList {
    pub fn new() -> Option<Self> {
        Some(Self {
            controllers: vec![],
        })
    }

    pub fn enumerate(&mut self) -> &mut Self {
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
                return self;
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
                return self;
            }

            let interfaces: Vec<_> = iface_list
                .split(|&e| e == 0x0)
                .filter(|v| !v.is_empty())
                .collect();

            let mut devinst = 0;
            let mut propertytype: DEVPROPTYPE = 0;
            for interface in interfaces {
                // let iface_list_str = String::from_utf16_lossy(interface);
                // println!("{} {:?}", devinst, iface_list_str);
                let mut current_device: Vec<u16> = vec![0; 1000];
                let mut device_id_size: u32 = 1000;
                if CM_Get_Device_Interface_PropertyW(
                    interface.as_ptr(),
                    &DEVPKEY_Device_InstanceId,
                    &mut propertytype,
                    current_device.as_mut_ptr() as *mut u8,
                    &mut device_id_size,
                    0,
                ) != CR_SUCCESS
                {
                    continue;
                }
                if propertytype != DEVPROP_TYPE_STRING {
                    continue;
                }

                if CM_Locate_DevNodeW(
                    &mut devinst,
                    current_device.as_ptr(),
                    CM_LOCATE_DEVNODE_NORMAL,
                ) != CR_SUCCESS
                {
                    continue;
                }

                if let Some(mut controller) = NvmeController::new(devinst) {
                    controller.inspect().enum_child_disks();
                    self.controllers.push(controller);
                }
            }
        }
        self
    }

    pub fn by_bus(&mut self, bus: u32) -> Option<NvmeController> {
        self.controllers.pop()
    }
}

impl fmt::Display for NvmeControllerList {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        for controller in &self.controllers {
            write!(fmt, "{}\n", controller)?;
        }
        Ok(())
    }
}
