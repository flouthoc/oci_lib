use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::serialize;

impl Spec {
    pub fn load(path: &str) -> Result<Spec, serialize::SerializeError> {
        serialize::deserialize(path)    
    }
    pub fn save(&self, path: &str) -> Result<(), serialize::SerializeError> {
        serialize::serialize(self, path)    
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
	#[serde(rename = "ociVersion")]
	#[serde(default)]
    pub version: String,
	#[serde(default)]
    pub process: Option<Process>,
	#[serde(default)]
    pub root: Option<Root>,
	#[serde(default)]
    pub hostname: Option<String>,
	#[serde(default)]
	pub mounts: Option<Vec<Mount>>,
	#[serde(default)]
	pub hooks: Option<Hooks>,
	#[serde(default)]
	pub annotations: Option<HashMap<String, String>>,
	#[serde(default)]
	pub linux: Option<Linux>,
	#[serde(default)]
    pub vm: Option<VM>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    #[serde(rename = "apparmorProfile")]
	#[serde(default)]
    pub apparmor_profile: Option<String>,
    pub args: Vec<String>,
	#[serde(default)]
    pub capabilities: Option<LinuxCapabilities>,
    #[serde(rename = "commandLine")]
	pub command_line: Option<String>,
    #[serde(rename = "consoleSize")]
	#[serde(default)]
    pub console_size: Option<ConsoleSize>,
    pub cwd: String,
    #[serde(default)]
    pub env: Option<Vec<String>>,
    #[serde(rename = "noNewPrivileges")]
    pub no_new_privileges: Option<bool>,
    #[serde(rename = "oomScoreAdj")]
    pub oom_score_adj: Option<i64>,
    #[serde(default)]
    pub rlimits: Option<Vec<POSIXRlimit>>,
    #[serde(rename = "selinuxLabel")]
    pub selinux_label: Option<String>,
    #[serde(default)]
    pub terminal: Option<bool>,
    #[serde(default)]
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxCapabilities {
    #[serde(default)]
    pub ambient: Option<Vec<String>>,
    #[serde(default)]
    pub bounding: Option<Vec<String>>,
    #[serde(default)]
    pub effective: Option<Vec<String>>,
    #[serde(default)]
    pub inheritable: Option<Vec<String>>,
    #[serde(default)]
    pub permitted: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsoleSize {
    pub height: i64,
    pub width: i64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "additionalGids")]
    #[serde(default)]
    pub additional_gids: Option<Vec<i64>>,
    #[serde(default)]
    pub gid: Option<i64>,
    #[serde(default)]
    pub uid: Option<i64>,
    #[serde(default)]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub path: String,
    #[serde(default)]
    pub readonly: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    pub destination: String,
    #[serde(default)]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub mount_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hooks {
    #[serde(default)]
    pub poststart: Option<Vec<Hook>>,
    #[serde(default)]
    pub poststop: Option<Vec<Hook>>,
    #[serde(default)]
    pub prestart: Option<Vec<Hook>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hook {
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub env: Option<Vec<String>>,
    pub path: String,
    #[serde(default)]
    pub timeout: Option<i64>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Linux {
    #[serde(rename = "cgroupsPath")]
    pub cgroups_path: Option<String>,
    pub devices: Option<Vec<LinuxDevice>>,
    #[serde(rename = "gidMappings")]
    pub gid_mappings: Option<Vec<LinuxIdMapping>>,
    #[serde(rename = "intelRdt")]
    pub intel_rdt: Option<LinuxIntelRdt>,
    #[serde(rename = "maskedPaths")]
    pub masked_paths: Option<Vec<String>>,
    #[serde(rename = "mountLabel")]
    pub mount_label: Option<String>,
    #[serde(default)]
    pub namespaces: Option<Vec<LinuxNamespace>>,
    #[serde(rename = "readonlyPaths")]
    pub readonly_paths: Option<Vec<String>>,
    pub resources: Option<LinuxResources>,
    #[serde(rename = "rootfsPropagation")]
    pub rootfs_propagation: Option<RootfsPropagation>,
    pub seccomp: Option<LinuxSeccomp>,
    pub sysctl: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "uidMappings")]
    pub uid_mappings: Option<Vec<LinuxIdMapping>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RootfsPropagation {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "slave")]
    Slave,
    #[serde(rename = "unbindable")]
    Unbindable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxNamespace {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(rename = "type")]
    pub namespace_reference_type: LinuxNamespaceType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinuxNamespaceType {
    #[serde(rename = "cgroup")]
    Cgroup,
    #[serde(rename = "ipc")]
    Ipc,
    #[serde(rename = "mount")]
    Mount,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "pid")]
    Pid,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "uts")]
    Uts,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxIdMapping {
    #[serde(rename = "containerID")]
    pub container_id: i64,
    #[serde(rename = "hostID")]
    pub host_id: i64,
    pub size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct POSIXRlimit {
    pub hard: i64,
    pub soft: i64,
    #[serde(rename = "type")]
    pub rlimit_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxHugepageLimit {
    pub limit: i64,
    #[serde(rename = "pageSize")]
    pub page_size: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxNetworkInterfacePriority {
    pub name: String,
    pub priority: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxBlockIo {
    #[serde(rename = "leafWeight")]
    #[serde(default)]
    pub leaf_weight: Option<i64>,
    #[serde(rename = "throttleReadBpsDevice")]
    #[serde(default)]
    pub throttle_read_bps_device: Option<Vec<LinuxBlockIoDeviceThrottle>>,
    #[serde(rename = "throttleReadIOPSDevice")]
    #[serde(default)]
    pub throttle_read_iops_device: Option<Vec<LinuxBlockIoDeviceThrottle>>,
    #[serde(rename = "throttleWriteBpsDevice")]
    #[serde(default)]
    pub throttle_write_bps_device: Option<Vec<LinuxBlockIoDeviceThrottle>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    #[serde(default)]
    pub throttle_write_iops_device: Option<Vec<LinuxBlockIoDeviceThrottle>>,
    #[serde(default)]
    pub weight: Option<i64>,
    #[serde(rename = "weightDevice")]
    #[serde(default)]
    pub weight_device: Option<Vec<LinuxBlockIoDeviceWeight>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxMemory {
    #[serde(rename = "disableOOMKiller")]
    #[serde(default)]
    pub disable_oom_killer: Option<bool>,
    #[serde(default)]
    pub kernel: Option<i64>,
    #[serde(rename = "kernelTCP")]
    #[serde(default)]
    pub kernel_tcp: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub reservation: Option<i64>,
    #[serde(default)]
    pub swap: Option<i64>,
    #[serde(default)]
    pub swappiness: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxBlockIoDeviceThrottle {
    pub major: i64,
    pub minor: i64,
    #[serde(default)]
    pub rate: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxBlockIoDeviceWeight {
    pub major: i64,
    pub minor: i64,
    #[serde(rename = "leafWeight")]
    #[serde(default)]
    pub leaf_weight: Option<i64>,
    #[serde(default)]
    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxCpu {
    #[serde(default)]
    pub cpus: Option<String>,
    #[serde(default)]
    pub mems: Option<String>,
    #[serde(default)]
    pub period: Option<i64>,
    #[serde(default)]
    pub quota: Option<i64>,
    #[serde(rename = "realtimePeriod")]
    #[serde(default)]
    pub realtime_period: Option<i64>,
    #[serde(rename = "realtimeRuntime")]
    #[serde(default)]
    pub realtime_runtime: Option<i64>,
    #[serde(default)]
    pub shares: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxPids {
    pub limit: i64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxResourcesNetwork {
    #[serde(rename = "classID")]
    #[serde(default)]
    pub class_id: Option<i64>,
    #[serde(default)]
    pub priorities: Option<Vec<LinuxInterfacePriority>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxInterfacePriority {
    pub name: String,
    pub priority: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxRdma {
    #[serde(rename = "hcaHandles")]
    #[serde(default)]
    pub hca_handles: Option<i64>,
    #[serde(rename = "hcaObjects")]
    #[serde(default)]
    pub hca_objects: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxResources {
    #[serde(rename = "blockIO")]
    #[serde(default)]
    pub block_io: Option<LinuxBlockIo>,
    #[serde(default)]
    pub cpu: Option<LinuxCpu>,
    #[serde(default)]
    pub devices: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(rename = "hugepageLimits")]
    #[serde(default)]
    pub hugepage_limits: Option<Vec<LinuxHugepageLimit>>,
    #[serde(default)]
    pub memory: Option<LinuxMemory>,
    #[serde(default)]
    pub network: Option<LinuxResourcesNetwork>,
    #[serde(default)]
    pub pids: Option<LinuxPids>,
    #[serde(default)]
    pub rdma: Option<HashMap<String, LinuxRdma>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxDevice {
    #[serde(rename = "fileMode")]
    #[serde(default)]
    pub file_mode: Option<i64>,
    #[serde(default)]
    pub gid: Option<i64>,
    #[serde(default)]
    pub major: Option<i64>,
    #[serde(default)]
    pub minor: Option<i64>,
    pub path: String,
    #[serde(rename = "type")]
    pub device_type: String,
    #[serde(default)]
    pub uid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxDeviceCgroup {
    #[serde(default)]
    pub access: Option<String>,
    pub allow: bool,
    #[serde(default)]
    pub major: Option<i64>,
    #[serde(default)]
    pub minor: Option<i64>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub device_cgroup_type: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct VM {
    #[serde(default)]
    pub hypervisor: Option<VMHypervisor>,
    #[serde(default)]
    pub image: Option<VMImage>,
    pub kernel: VMKernel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VMHypervisor {
    #[serde(default)]
    pub parameters: Option<Vec<String>>,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VMImage {
    pub format: RootImageFormat,
    pub path: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RootImageFormat {
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "vdi")]
    Vdi,
    #[serde(rename = "vhd")]
    Vhd,
    #[serde(rename = "vmdk")]
    Vmdk,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VMKernel {
    #[serde(default)]
    pub initrd: Option<String>,
    #[serde(default)]
    pub parameters: Option<Vec<String>>,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxSeccomp {
    #[serde(default)]
    pub architectures: Option<Vec<LinuxSeccompArch>>,
    #[serde(rename = "defaultAction")]
    pub default_action: LinuxSeccompAction,
    #[serde(default)]
    pub syscalls: Option<Vec<LinuxSyscall>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxSyscall {
    pub action: LinuxSeccompAction,
    #[serde(default)]
    pub args: Option<Vec<LinuxSyscallArg>>,
    pub names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxSyscallArg {
    pub index: i64,
    pub op: LinuxSeccompOperators,
    pub value: i64,
    #[serde(rename = "valueTwo")]
    #[serde(default)]
    pub value_two: Option<i64>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinuxSeccompArch {
    #[serde(rename = "SCMP_ARCH_AARCH64")]
    ScmpArchAarch64,
    #[serde(rename = "SCMP_ARCH_ARM")]
    ScmpArchArm,
    #[serde(rename = "SCMP_ARCH_MIPS")]
    ScmpArchMips,
    #[serde(rename = "SCMP_ARCH_MIPS64")]
    ScmpArchMips64,
    #[serde(rename = "SCMP_ARCH_MIPS64N32")]
    ScmpArchMips64N32,
    #[serde(rename = "SCMP_ARCH_MIPSEL")]
    ScmpArchMipsel,
    #[serde(rename = "SCMP_ARCH_MIPSEL64")]
    ScmpArchMipsel64,
    #[serde(rename = "SCMP_ARCH_MIPSEL64N32")]
    ScmpArchMipsel64N32,
    #[serde(rename = "SCMP_ARCH_PARISC")]
    ScmpArchParisc,
    #[serde(rename = "SCMP_ARCH_PARISC64")]
    ScmpArchParisc64,
    #[serde(rename = "SCMP_ARCH_PPC")]
    ScmpArchPpc,
    #[serde(rename = "SCMP_ARCH_PPC64")]
    ScmpArchPpc64,
    #[serde(rename = "SCMP_ARCH_PPC64LE")]
    ScmpArchPpc64Le,
    #[serde(rename = "SCMP_ARCH_S390")]
    ScmpArchS390,
    #[serde(rename = "SCMP_ARCH_S390X")]
    ScmpArchS390X,
    #[serde(rename = "SCMP_ARCH_X32")]
    ScmpArchX32,
    #[serde(rename = "SCMP_ARCH_X86")]
    ScmpArchX86,
    #[serde(rename = "SCMP_ARCH_X86_64")]
    ScmpArchX8664,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinuxSeccompAction {
    #[serde(rename = "SCMP_ACT_ALLOW")]
    ScmpActAllow,
    #[serde(rename = "SCMP_ACT_ERRNO")]
    ScmpActErrno,
    #[serde(rename = "SCMP_ACT_KILL")]
    ScmpActKill,
    #[serde(rename = "SCMP_ACT_TRACE")]
    ScmpActTrace,
    #[serde(rename = "SCMP_ACT_TRAP")]
    ScmpActTrap,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinuxSeccompOperators {
    #[serde(rename = "SCMP_CMP_EQ")]
    ScmpCmpEq,
    #[serde(rename = "SCMP_CMP_GE")]
    ScmpCmpGe,
    #[serde(rename = "SCMP_CMP_GT")]
    ScmpCmpGt,
    #[serde(rename = "SCMP_CMP_LE")]
    ScmpCmpLe,
    #[serde(rename = "SCMP_CMP_LT")]
    ScmpCmpLt,
    #[serde(rename = "SCMP_CMP_MASKED_EQ")]
    ScmpCmpMaskedEq,
    #[serde(rename = "SCMP_CMP_NE")]
    ScmpCmpNe,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxIntelRdt {
    #[serde(rename = "closID")]
    pub clos_id: Option<String>,
    #[serde(rename = "l3CacheSchema")]
    pub l3_cache_schema: Option<String>,
    #[serde(rename = "memBwSchema")]
    pub mem_bw_schema: Option<String>,
}
