use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt};
use machineid_rs::{IdBuilder, Encryption, HWIDComponent};
use crate::LocalSystemConfig;


pub fn get_local_system_conf() -> LocalSystemConfig {
    // Unique Host ID building
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder.add_component(HWIDComponent::CPUID).add_component(HWIDComponent::MacAddress);
    
    // System information gathering
    let mut sys = System::new_all();
    sys.refresh_all();

    // Fetch and return the required informations as a LocalSystemConfig struct
    LocalSystemConfig {
        osname: sys.name().unwrap(),
        osversion: sys.os_version().unwrap(),
        hostname: sys.host_name().unwrap(),
        hostid: builder.build("karnaboo").unwrap()
    }
}