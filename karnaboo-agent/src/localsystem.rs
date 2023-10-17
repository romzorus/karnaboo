use colored::Colorize;
use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use sysinfo::{DiskExt, System, SystemExt};
use std::process::exit;

pub fn get_local_system_conf() -> LocalSystemConfig {
    // Unique Host ID building
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder
        .add_component(HWIDComponent::CPUID)
        .add_component(HWIDComponent::MacAddress)
        .add_component(HWIDComponent::MachineName)
        .add_component(HWIDComponent::Username);

    // System information gathering
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut disks_info: Vec<u64> = vec![];
    for disk in sys.disks() {
        disks_info.push(disk.available_space() / 1_073_741_824); // Also converting bytes to gb (1 gb = 1 073 741 824 bytes)
    }

    println!("Local config :");
    println!("  - OS name : {}", sys.name().unwrap());
    println!("  - OS version : {}", sys.os_version().unwrap());
    println!("  - Hostname : {}", sys.host_name().unwrap());
    println!("  - Host key : {}", builder.build("karnaboo").unwrap());

    // Fetch and return the required informations as a LocalSystemConfig struct
    LocalSystemConfig {
        osname: sys.name().unwrap(),
        osversion: sys.os_version().unwrap(),
        hostname: sys.host_name().unwrap(),
        _key: builder.build("karnaboo").unwrap(),
        disks_infos: disks_info,
    }
}

// If you want this machine to become a DISS or REPS, it needs at least xxx gb of free space
const MIN_FREE_SPACE_FOR_DISS_REPS: u64 = 5; // Unit : GB

pub fn check_request_feasability(role: &String, local_conf: &LocalSystemConfig) {
    // For now, only the available space is watched
    // More criterias may come in the future
    let mut feasability = false;

    // Checking that available space is enough for the DISS and REPS roles
    let disks_infos = local_conf.disks_infos.clone();
    if ["diss", "reps"].contains(&role.as_str()) {
        for free_space in disks_infos.into_iter() {
            if free_space >= MIN_FREE_SPACE_FOR_DISS_REPS {
                feasability = true;
            }
        }
        if !feasability {
            println!(
                "{}",
                format!("Not enough available space for the {} role.", role)
                    .bold()
                    .red()
            );
            println!(
                "{}",
                format!(
                    "You need a disk with at least {} gb free space.",
                    MIN_FREE_SPACE_FOR_DISS_REPS
                )
                .bold()
                .red()
            );
        }
    } else if role == "client" {
        // No criteria to check for client role at the moment
        feasability = true;
    } else {
        println!("Unable to check feasability. Incorrect role value.");
    }

    // After checking, final result
    if feasability {
        println!("Your system is compatible with your request.");
    } else {
        println!("The requirements are not met for your request.");
        exit(1);
    }

}

#[derive(Debug)]
pub struct LocalSystemConfig {
    pub osname: String,
    pub osversion: String,
    pub hostname: String,
    pub _key: String,
    pub disks_infos: Vec<u64>, // Only stores the free space of each disk (unit : gb)
}
