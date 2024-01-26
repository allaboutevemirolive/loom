use std::env;

pub mod opts;

fn main() {
    let _ = init_nm();
}

#[allow(unused_variables, unused_mut)]
fn init_nm() {
    let _: Vec<String> = env::args().collect();

    let success: bool = true;
    let mut manager: Option<NetworkManager> = None;
}

// Core

pub struct NetworkManager;

pub struct DBusObject;

pub struct ActiveConnection;

pub struct AuditManager;

pub struct VpnConnection;

pub struct ActRequest;

pub struct AuthSubject;

pub struct DBusManager;

pub struct Config;

pub struct ConfigData;

pub struct Device;

pub struct DhcpConfig;

pub struct IPConfig;

pub struct Netns;

pub struct Policy;

pub struct RfkillManager;

pub struct PacrunnerManager;

pub struct SessionMonitor;

pub struct KeepAlive;

pub struct SleepMonitor;

pub struct LldpListener;

pub struct ConfigDeviceStateData;
