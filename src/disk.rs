use log::{debug, error, info, trace, warn};
use std::io;
use winapi::{
    ctypes::c_void,
    shared::ntdef::LARGE_INTEGER,
    um::{
        errhandlingapi::GetLastError,
        fileapi::{self as fs, CREATE_ALWAYS, OPEN_EXISTING},
        handleapi::INVALID_HANDLE_VALUE,
        ioapiset::DeviceIoControl,
        winbase::{FILE_FLAG_NO_BUFFERING, FILE_FLAG_WRITE_THROUGH},
        winioctl::{DISK_GEOMETRY_EX, IOCTL_DISK_GET_DRIVE_GEOMETRY_EX},
        winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, HANDLE},
    },
};

use std::{
    io::{Read, Write},
    mem::size_of,
    ptr::null_mut,
};

use crate::SECTOR_SIZE;

pub fn last_error() -> u32 {
    unsafe { GetLastError() }
}

#[derive(Copy, Clone, Debug)]
pub struct DiskGeometryEx {
    pub cylinders: i64,
    pub media_type: u32,
    pub tracks_per_cylinder: u32,
    pub sectors_per_track: u32,
    pub bytes_per_sector: u32,
    pub disk_size: i64,
    data: [u8; 1],
}

impl DiskGeometryEx {
    /// Returns the size of the disk in bytes.
    pub fn size(&self) -> u64 {
        self.sectors() * self.bytes_per_sector as u64
    }

    /// Returns the number of sectors of the disk.
    pub fn sectors(&self) -> u64 {
        self.cylinders as u64 * self.tracks_per_cylinder as u64 * self.sectors_per_track as u64
    }
}

impl From<DISK_GEOMETRY_EX> for DiskGeometryEx {
    fn from(geo: DISK_GEOMETRY_EX) -> Self {
        DiskGeometryEx {
            cylinders: unsafe { *geo.Geometry.Cylinders.QuadPart() },
            media_type: geo.Geometry.MediaType,
            tracks_per_cylinder: geo.Geometry.TracksPerCylinder,
            sectors_per_track: geo.Geometry.SectorsPerTrack,
            bytes_per_sector: geo.Geometry.BytesPerSector,
            disk_size: unsafe { *geo.DiskSize.QuadPart() },
            data: geo.Data,
        }
    }
}

fn geometry(drive: &HANDLE) -> usize {
    let mut geo = Default::default();
    let mut bytes_returned = 0u32;
    let geo_ptr: *mut DISK_GEOMETRY_EX = &mut geo;
    let r = unsafe {
        DeviceIoControl(
            *drive,
            IOCTL_DISK_GET_DRIVE_GEOMETRY_EX,
            null_mut(),
            0,
            geo_ptr as *mut c_void,
            size_of::<DISK_GEOMETRY_EX>() as u32,
            &mut bytes_returned,
            null_mut(),
        )
    };
    if r == 0 {
        0 as usize
    } else {
        DiskGeometryEx::from(geo).disk_size as usize
    }
}

fn getfilesize(drive: &HANDLE) -> usize {
    let mut bytes_returned = LARGE_INTEGER::default();
    let r = unsafe { fs::GetFileSizeEx(*drive, &mut bytes_returned) };
    if r == 0 {
        geometry(drive)
    } else {
        unsafe { *bytes_returned.QuadPart() as usize }
    }
}

#[derive(Debug)]
pub struct Disk {
    path: String,
    rw: char,
    handle: HANDLE,
    pub size: usize,
}

impl Disk {
    pub fn open(path: String, rw: char) -> Option<Disk> {
        let handle = unsafe {
            fs::CreateFileA(
                std::ffi::CString::new(path.as_str()).unwrap().as_ptr(),
                if rw == 'w' {
                    GENERIC_WRITE
                } else {
                    GENERIC_READ
                },
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                if rw == 'w' && !path.contains("PhysicalDrive") {
                    CREATE_ALWAYS
                } else {
                    OPEN_EXISTING
                },
                FILE_FLAG_NO_BUFFERING | FILE_FLAG_WRITE_THROUGH,
                null_mut(),
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            warn!("Can't open file!! '{}'", path);
            None
        } else {
            Some(Disk {
                path,
                rw,
                handle,
                size: getfilesize(&handle),
            })
        }
    }

    /// The size of the drive in bytes.
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Read for Disk {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0u32;
        let res = unsafe {
            fs::ReadFile(
                self.handle,
                buf.as_mut_ptr() as *mut c_void,
                buf.len() as u32,
                &mut bytes_read,
                null_mut(),
            )
        };
        if res == 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error code: {:#08x}", last_error()),
            ))
        } else {
            Ok(bytes_read as usize)
        }
    }
}

impl Write for Disk {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() <= 0 {
            return Ok(0);
        }
        let mut len = buf.len() - 1;
        len += SECTOR_SIZE - (len % SECTOR_SIZE);
        let mut bytes_write = 0u32;
        let res = unsafe {
            fs::WriteFile(
                self.handle,
                buf.as_ptr() as *const c_void,
                len as u32,
                &mut bytes_write,
                null_mut(),
            )
        };
        if res == 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error code: {:#08x}", last_error()),
            ))
        } else {
            Ok(bytes_write as usize)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

unsafe impl Send for Disk {}
unsafe impl Sync for Disk {}
