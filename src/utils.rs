use std::net::IpAddr;

use local_ip_address::local_ip;

pub fn connection_string(interface: IpAddr, port: u16) -> String {
    let i = interface.to_string();
    let addr = match i.as_str() {
        "0.0.0.0" => local_ip()
            .map(|a| a.to_string())
            .unwrap_or("127.0.0.1".to_string()),
        _ => i,
    };
    format!(
        "http://{}:{}",
        addr,
        if cfg!(debug_assertions) { 3001 } else { port }
    )
}
