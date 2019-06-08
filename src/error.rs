use std::{error, io, str};

use jsonrpc_core::{types::error::Error, ErrorCode};
use snafu::Snafu;

pub type BoxError = Box<dyn error::Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum NetworkError {
    #[snafu(display("Failed to add network for {}", ssid))]
    AddWifi { ssid: String },

    #[snafu(display("Could not access IP address for interface: {}", iface))]
    GetIp { iface: String, source: io::Error },

    #[snafu(display("Could not find SSID for interface: {}", iface))]
    GetSsid { iface: String },

    #[snafu(display("No saved networks found for default interface"))]
    ListSavedNetworks,

    #[snafu(display("No networks found in range of interface: {}", iface))]
    ListScanResults { iface: String },

    #[snafu(display("Missing expected parameters: {}", e))]
    MissingParams { e: Error },

    #[snafu(display("No IP found for interface: {}", iface))]
    NoIpFound { iface: String },

    #[snafu(display("Failed to reassociate with WiFi network for interface: {}", iface))]
    ReassociateFailed { iface: String },

    #[snafu(display("Failed to reconnect with WiFi network for interface: {}", iface))]
    ReconnectFailed { iface: String },

    #[snafu(display("Regex command failed"))]
    RegexFailed { source: regex::Error },

    #[snafu(display("Failed to run interface_checker script: {}", source))]
    RunApClientScript { source: io::Error },

    #[snafu(display("Failed to open control interface for wpasupplicant"))]
    WpaCtrlOpen {
        #[snafu(source(from(failure::Error, std::convert::Into::into)))]
        source: BoxError,
    },

    #[snafu(display("Request to wpasupplicant via wpactrl failed"))]
    WpaCtrlRequest {
        #[snafu(source(from(failure::Error, std::convert::Into::into)))]
        source: BoxError,
    },
}

impl From<NetworkError> for Error {
    fn from(err: NetworkError) -> Self {
        match &err {
            NetworkError::AddWifi { ssid } => Error {
                code: ErrorCode::ServerError(-32000),
                message: format!("Failed to add network for {}", ssid),
                data: None,
            },
            NetworkError::GetIp { iface, source } => Error {
                code: ErrorCode::ServerError(-32000),
                message: format!("Failed to retrieve IP address for {}: {}", iface, source),
                data: None,
            },
            NetworkError::GetSsid { iface } => Error {
                code: ErrorCode::ServerError(-32000),
                message: format!(
                    "Failed to retrieve SSID for {}. Interface may not be connected.",
                    iface
                ),
                data: None,
            },
            NetworkError::ListSavedNetworks => Error {
                code: ErrorCode::ServerError(-32000),
                message: "No saved networks found".to_string(),
                data: None,
            },
            NetworkError::ListScanResults { iface } => Error {
                code: ErrorCode::ServerError(-32000),
                message: format!("No networks found in range of interface {}", iface),
                data: None,
            },
            NetworkError::MissingParams { e } => e.clone(),
            NetworkError::NoIpFound { iface } => Error {
                code: ErrorCode::ServerError(-32000),
                message: format!("No IP address found for {}", iface),
                data: None,
            },
            NetworkError::ReassociateFailed { iface } => Error {
                code: ErrorCode::InternalError,
                message: format!("Failed to reassociate with WiFi network for: {}", iface),
                data: None,
            },
            NetworkError::ReconnectFailed { iface } => Error {
                code: ErrorCode::InternalError,
                message: format!("Failed to reconnect with WiFi network for: {}", iface),
                data: None,
            },
            NetworkError::RegexFailed { source } => Error {
                code: ErrorCode::ServerError(-32000),
                message: format!("Regex command error: {}", source),
                data: None,
            },
            NetworkError::RunApClientScript { source } => Error {
                code: ErrorCode::InternalError,
                message: format!("Failed to run interface_checker script: {}", source),
                data: None,
            },
            NetworkError::WpaCtrlOpen { source } => Error {
                code: ErrorCode::ServerError(-32000),
                message: format!(
                    "Failed to open control interface for wpasupplicant: {}",
                    source
                ),
                data: None,
            },
            NetworkError::WpaCtrlRequest { source } => Error {
                code: ErrorCode::ServerError(-32000),
                message: format!("WPA supplicant request failed: {}", source),
                data: None,
            },
        }
    }
}
