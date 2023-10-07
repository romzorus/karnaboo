use crate::LocalSystemConfig;
use colored::Colorize;
use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use sysinfo::{CpuExt, DiskExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

pub fn get_local_system_conf() -> LocalSystemConfig {
    // Unique Host ID building
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder
        .add_component(HWIDComponent::CPUID)
        .add_component(HWIDComponent::MacAddress);

    // System information gathering
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut disks_info: Vec<u64> = vec![];
    for disk in sys.disks() {
        disks_info.push(disk.available_space() / 1_073_741_824); // Also converting bytes to gb (1 gb = 1 073 741 824 bytes)
    }

    // Fetch and return the required informations as a LocalSystemConfig struct
    LocalSystemConfig {
        osname: sys.name().unwrap(),
        osversion: sys.os_version().unwrap(),
        hostname: sys.host_name().unwrap(),
        hostid: builder.build("karnaboo").unwrap(),
        disks_infos: disks_info,
    }
}

// If you want this machine to become a DISS or REPS, it needs at least xxx gb of free space
const MIN_FREE_SPACE_FOR_DISS_REPS: u64 = 300; // Unit : GB

pub fn check_request_feasability(role: &String, local_conf: &LocalSystemConfig) -> bool {
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
    }

    // No criteria to check for client role at the moment
    if role == "client" {
        feasability = true;
    }

    feasability
}
