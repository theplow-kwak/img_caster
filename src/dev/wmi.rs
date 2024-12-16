use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection, WMIDateTime, Variant};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32_DiskDrive {
    DeviceID: String,
    Index: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32_DiskPartition {
    DeviceID: String,
    DiskIndex: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32_LogicalDiskToPartition {
    Antecedent: String,
    Dependent: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32_LogicalDisk {
    DeviceID: String,
}

fn wmi_query() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize COM library
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    // Change this to the target physical disk index
    let target_disk_index: u32 = 0;

    // Query all disk drives
    let disks: Vec<Win32_DiskDrive> = wmi_con.query()?;
    let disk = disks
        .into_iter()
        .find(|d| d.Index == target_disk_index)
        .ok_or("Disk not found")?;
    
    println!("Found Disk: {:?}", disk);

    // Query all partitions associated with the disk
    let partitions: Vec<Win32_DiskPartition> = wmi_con.query()?;
    let associated_partitions: Vec<_> = partitions
        .into_iter()
        .filter(|p| p.DiskIndex == target_disk_index)
        .collect();

    println!("Partitions: {:?}", associated_partitions);

    // Query logical disk to partition mappings
    let mappings: Vec<Win32_LogicalDiskToPartition> = wmi_con.query()?;
    let logical_disks: Vec<_> = associated_partitions
        .iter()
        .flat_map(|partition| {
            mappings.iter().filter_map(move |map| {
                if map.Antecedent.contains(&partition.DeviceID) {
                    // Extract the logical disk (drive letter)
                    let start = map.Dependent.find("DeviceID=")? + 10; // Skip `DeviceID=` + quotes
                    let end = map.Dependent.rfind('"')?;
                    Some(&map.Dependent[start..end])
                } else {
                    None
                }
            })
        })
        .collect();

    println!("Logical Disks: {:?}", logical_disks);

    Ok(())
}

// wmi = "0.7"
// serde = { version = "1.0", features = ["derive"] }
