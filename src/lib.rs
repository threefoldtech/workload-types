use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Workload {
    pub workload_id: i64,
    pub node_id: String,
    pub customer_id: i64,
    
    pub version: i64,
    pub reference: String,
    pub pool_id: i64,
    pub epoch: i64,
    pub description: String,
    pub metadata: String,

    pub data: WorkloadData
}

#[derive(Serialize, Deserialize)]
pub enum WorkloadData {
    Volume(VolumeInformation),
    ZDB(ZDBInformation),
    Container(ContainerInformation),
    K8S(K8SInformation),
    PublicIP(PublicIPInformation),
    Network(NetworkInformation),
    GatewayProxy(GatewayProxyInformation),
    GatewayReverseProxy(GatewayReverseProxyInformation),
    GatewaySubdomain(GatewaySubdomainInformation),
    GatewayDelegate(GatewayDelegateInformation),
    Gateway4To6(Gateway4To6Information)
}

#[derive(Serialize, Deserialize)]
pub struct VolumeInformation {
    pub size: i64,
    pub kind: DiskType,
}

#[derive(Serialize, Deserialize)]
pub struct ZDBInformation {
    pub size: i64,
    pub mode: ZdbMode,
    pub password: String,
    pub disk_type: DiskType,
    pub public: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ContainerInformation {
    pub flist: String,
    pub hub_url: String,
    pub environment: HashMap<String, String>,
    pub secret_environment: HashMap<String, String>,
    pub entrypoint: String,
    pub interactive: bool,
    pub volumes: Vec<ContainerMount>,
    pub network_connections: Vec<NetworkConnection>,
    pub stats: Vec<Stats>,
    pub logs: Vec<Logs>,
    pub capacity: ContainerCapacity
}

#[derive(Serialize, Deserialize)]
pub struct K8SInformation {
    pub size: i64,
    pub cluster_secret: String,
    pub network_id: String,
    pub ipaddress: std::net::IpAddr,
    pub master_ips: Vec<std::net::IpAddr>,
    pub ssh_keys: Vec<String>,
    pub public_ip_wid: i64
}

#[derive(Serialize, Deserialize)]
pub struct PublicIPInformation {
    pub ipaddress: IPNet
}

#[derive(Serialize, Deserialize)]
pub struct NetworkInformation {
    pub name: String,
    pub workload_id: i64,
    pub iprange: IPNet,
    pub network_resources: Vec<NetworkResources>,
    pub farmer_tid: i64
}

#[derive(Serialize, Deserialize)]
pub struct GatewayProxyInformation {
    pub domain: String,
    pub addr: String,
    pub port: u32,
    pub port_tls: u32
}

#[derive(Serialize, Deserialize)]
pub struct GatewayReverseProxyInformation {
    pub domain: String,
    pub secret: String
}

#[derive(Serialize, Deserialize)]
pub struct GatewaySubdomainInformation {
    pub domain: String,
    pub ips: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct GatewayDelegateInformation {
    pub domain: String
}

#[derive(Serialize, Deserialize)]
pub struct Gateway4To6Information {
    pub public_key: String
}

#[derive(Serialize, Deserialize)]
pub struct NetworkResources {
    pub node_id: String,
    pub wireguard_private_key_encrypted: String,
    pub wireguard_public_key: String,
    pub wireguard_listen_port: i64,
    pub iprange: IPNet,
    pub peers: Vec<WireguardPeer>
}

#[derive(Serialize, Deserialize)]
pub struct WireguardPeer {
    pub public_key: String,
    pub endpoint: String,
    pub iprange: IPNet,
    pub allowed_ip_range: Vec<IPNet>
}

#[derive(Serialize, Deserialize)]
pub struct IPNet {
    pub ip: std::net::IpAddr,
    pub mask: Vec<u8>
}

#[derive(Serialize, Deserialize)]
pub struct ContainerMount {
    pub volume_id: String,
    pub mount_point: String
}

#[derive(Serialize, Deserialize)]
pub struct NetworkConnection {
    pub network_id: String,
    pub ipaddress: std::net::IpAddr,
    pub public_ip6: bool,
    pub yggdrasil_ip: bool
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub stats_type: String,
    pub data: Vec<u8>
}

#[derive(Serialize, Deserialize)]
pub struct Logs {
    pub logs_type: String,
    pub data: LogRedis
}

#[derive(Serialize, Deserialize)]
pub struct LogRedis {
    pub stdout: String,
    pub stderr: String,

    pub secret_stdout: String,
    pub secret_stderr: String
}

#[derive(Serialize, Deserialize)]
pub struct ContainerCapacity {
    pub cpu: i64,
    pub memory: i64,
    pub disk_type: DiskType,
    pub disk_size: i64
}

#[derive(Serialize, Deserialize)]
pub enum ZdbMode {
    ZDBModeSeq,
	ZDBModeUser
}

#[derive(Serialize, Deserialize)]
pub enum DiskType {
    SSD,
    HDD
}