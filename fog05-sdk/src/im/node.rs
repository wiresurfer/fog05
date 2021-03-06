/*********************************************************************************
* Copyright (c) 2018,2020 ADLINK Technology Inc.
*
* This program and the accompanying materials are made available under the
* terms of the Eclipse Public License 2.0 which is available at
* http://www.eclipse.org/legal/epl-2.0, or the Apache Software License 2.0
* which is available at https://www.apache.org/licenses/LICENSE-2.0.
*
* SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
* Contributors:
*   ADLINK fog05 team, <fog05@adlink-labs.tech>
*********************************************************************************/

extern crate serde;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

//use pnet::datalink::NetworkInterface; Once they support serde use them...

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum NodeStatusEnum {
    UNKNOWN,
    NOTREADY,
    READY,
}

impl std::fmt::Display for NodeStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NodeStatusEnum::NOTREADY => write!(f, "NotReady"),
            NodeStatusEnum::READY => write!(f, "Ready"),
            NodeStatusEnum::UNKNOWN => write!(f, "Unknown"),
        }
    }
}

// Node Information

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub index: u32,
    pub mac: Option<crate::types::MACAddress>,
    pub ips: Vec<pnet::ipnetwork::IpNetwork>,
    pub flags: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CPUSpec {
    pub model: String,
    pub frequency: u64, //in MHz
    pub arch: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RAMSpec {
    pub size: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskSpec {
    pub local_address: String,
    pub dimension: f64,
    pub mount_point: String,
    pub file_system: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IOSpec {
    pub name: String,
    pub io_type: String,
    pub io_file: String,
    pub available: bool,
}

// #[derive(Serialize,Deserialize,Debug, Clone)]
// pub struct VolatilitySpec {
//     pub avg_availability_minutes : u64,
//     pub quartile_availability_minutes : Vec<u64>
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AcceleratorSpec {
    pub hw_address: String,
    pub name: String,
    pub supported_libraries: Vec<String>,
    pub available: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionSpec {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterfaceConfiguration {
    pub ipv4_address: String,
    pub ipv4_netmask: String,
    pub ipv4_gateway: String,
    pub ipv6_address: String,
    pub ipv6_netmask: String,
    pub ipv6_gateway: Option<String>,
    pub bus_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeInfo {
    pub uuid: Uuid,
    pub name: String,
    pub os: String,
    pub cpu: Vec<CPUSpec>,
    pub ram: RAMSpec,
    pub disks: Vec<DiskSpec>,
    pub io: Vec<IOSpec>,
    pub accelerators: Vec<AcceleratorSpec>,
    pub interfaces: Vec<NetworkInterface>,
    pub position: Option<PositionSpec>,
    pub agent_service_uuid: Uuid,
    //pub volatility : Option<VolatilitySpec>
}

// Node Monitoring

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RAMStatus {
    pub total: f64,
    pub free: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskStatus {
    pub mount_point: String,
    pub total: f64,
    pub free: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeighborPeerInfo {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeighborInfo {
    pub node: NeighborPeerInfo,
    pub port: NeighborPeerInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Neighbor {
    pub src: NeighborInfo,
    pub dst: NeighborInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingInfo {
    pub peer: String,
    pub average: f64,
    pub ip: String,
    pub iface: String,
    pub packet_loss: f64,
    pub packet_sent: u64,
    pub packet_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeartbeatInfo {
    pub nodeid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkInterfaceStatus {
    pub name: String,
    pub index: u32,
    pub mac: Option<crate::types::MACAddress>,
    pub ips: Vec<pnet::ipnetwork::IpNetwork>,
    pub flags: u32,
    pub is_up: bool,
    pub mtu: u64,
    pub speed: u64,
    pub sent_pkts: u64,
    pub recv_pkts: u64,
    pub sent_bytes: u64,
    pub recv_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeStatus {
    pub uuid: Uuid,
    pub status: NodeStatusEnum,
    pub supported_hypervisors: Vec<String>,
    pub interfaces: Vec<NetworkInterfaceStatus>,
    pub ram: RAMStatus,
    pub disk: Vec<DiskStatus>,
    pub neighbors: Vec<Neighbor>,
}
