use anyhow::Result;
use hidapi::{DeviceInfo, HidApi};
use log::{debug, error};
use serde::Deserialize;

use crate::controller::Status;

use super::Controller;

pub const VENDOR_ID_NINTENDO: u16 = 0x057e;
pub const PRODUCT_ID_NINTENDO_PROCON: u16 = 0x2009;
pub const PRODUCT_ID_NINTENDO_JOYCON_L: u16 = 0x2006;
pub const PRODUCT_ID_NINTENDO_JOYCON_R: u16 = 0x2007;

const INPUT_REPORT_SIZE: usize = 362;

#[macro_export]
macro_rules! BIT {
    ($x:expr) => {
        1 << $x
    };
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Deserialize)]
struct InputReport {
    id: u8,
    timer: u8,
    bat_con: u8,
}

pub fn parse_controller_data(device_info: &DeviceInfo, hidapi: &HidApi) -> Result<Controller> {
    let name = match device_info.product_id() {
        PRODUCT_ID_NINTENDO_JOYCON_L => "Joy-Con L",
        PRODUCT_ID_NINTENDO_JOYCON_R => "Joy-Con R",
        PRODUCT_ID_NINTENDO_PROCON => "Pro Controller",
        _ => "Nintendo Controller",
    };
    let mut controller = Controller::from_hidapi(device_info, name, 0, Status::Unknown);

    let device = device_info.open_device(hidapi)?;
    let mut buf = [0u8; INPUT_REPORT_SIZE];
    let _res = match device.read_timeout(&mut buf[..], 1000) {
        Ok(res) => res,
        Err(e) => {
            error!("Error reading from device: {}", e);
            return Ok(controller);
        }
    };

    let input_report: InputReport = bincode::deserialize(&buf[0..3])?;
    let tmp = input_report.bat_con;
    let _host_powered = tmp & BIT!(0) != 0;
    let battery_charging = tmp & BIT!(4) != 0;
    let tmp = tmp >> 5;
    controller.status = if battery_charging {
        Status::Charging
    } else {
        Status::Discharging
    };
    match tmp {
        0 => controller.capacity = 5,
        1 => controller.capacity = 25,
        2 => controller.capacity = 50,
        3 => controller.capacity = 75,
        4 => controller.capacity = 100,
        _ => {
            controller.capacity = 0;
            debug!("Unknown battery status: {}", tmp);
        }
    }

    Ok(controller)
}
