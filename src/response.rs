use serde::Serialize;

use crate::ip::Ip;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct IpData {
    pub ip: Ip,
}

#[derive(Serialize, Debug)]
pub struct SingleIpResponse {
    pub status: String,
    pub data: IpData,
}

#[derive(Serialize, Debug)]
pub struct IpListResponse {
    pub status: String,
    pub results: usize,
    pub ips: Vec<Ip>,
}
