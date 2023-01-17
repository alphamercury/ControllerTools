use anyhow::Result;
use hidapi::{DeviceInfo, HidApi};
// use log::debug;
// use serde::{Deserialize, Serialize};

// use evdev::{
//     Device,
//     InputEvent,
//     // InputEventType, InputProperty, InputPropertyType, InputPropertyValue,
// };
// use evdev_rs::{Device, InputEvent};

use super::Controller;

pub const MS_VENDOR_ID: u16 = 0x045e;
pub const MS_VENDOR_ID_STR: &str = "045e";

// Xbox One S controller
pub const XBOX_CONTROLLER_USB_PRODUCT_ID: u16 = 0x02ea; // 746
pub const XBOX_CONTROLLER_USB_PRODUCT_ID_STR: &str = "02ea"; // 746
pub const XBOX_CONTROLLER_PRODUCT_ID: u16 = 0x02df; // 765

// Xbox Wireless Controller (model 1914)
pub const XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID: u16 = 0x0b12; // 2834
pub const XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID_STR: &str = "0b12"; // 2834
pub const XBOX_WIRELESS_CONTROLLER_BT_PRODUCT_ID: u16 = 0x0b13; // 2835

// pub const XBOX_ONE_REPORT_BT_SIZE: usize = 64;

pub fn get_xbox_controller(product_id: u16, bluetooth: bool) -> Result<Controller> {
    let controller = Controller {
        name: if product_id == XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID
            || product_id == XBOX_WIRELESS_CONTROLLER_BT_PRODUCT_ID
        {
            "Xbox Series X/S".to_string()
        } else {
            "Xbox One S".to_string()
        },
        product_id: product_id,
        vendor_id: MS_VENDOR_ID,
        capacity: if bluetooth { 0 } else { 100 }, // for now for USB, "fake" it and set capacity to 100 as charging
        status: if bluetooth {
            "unknown".to_string()
        } else {
            // for now for USB, "fake" it and set status to charging since it's plugged in
            "charging".to_string()
        },
        bluetooth,
        hid_device_path: "unknown".to_string(),
    };

    Ok(controller)
}

pub fn parse_xbox_controller_data(
    device_info: &DeviceInfo,
    _hidapi: &HidApi,
) -> Result<Controller> {
    let bluetooth = device_info.interface_number() == -1;
    // let device = device_info.open_device(hidapi)?;

    // println!("sneding feature report");
    // let mut buffer = [0; 64];
    // buffer[0] = 0x05;
    // match device.send_feature_report(&buffer) {
    //     Ok(_) => println!("feature report request sent"),
    //     Err(e) => println!("Failed to get report {:?}", e),
    // }
    // // println!("Send report response: {:?}", send_report_res);

    // let mut status_buffer = [0; 64];
    // let res = device.get_feature_report(&mut status_buffer)?;
    // println!("Get report response: {:?}", res);

    // TODO Read data from device_info to maybe get battery data?
    // so far we couldn't figure out how
    // let mut buf = [0u8; XBOX_ONE_REPORT_BT_SIZE];
    // let res = device.read(&mut buf[..])?;
    let controller = Controller {
        name: if device_info.product_id() == XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID
            || device_info.product_id() == XBOX_WIRELESS_CONTROLLER_BT_PRODUCT_ID
        {
            "Xbox Series X/S".to_string()
        } else {
            "Xbox One S".to_string()
        },
        product_id: device_info.product_id(),
        vendor_id: device_info.vendor_id(),
        capacity: 0,
        status: "unknown".to_string(),
        bluetooth,
        hid_device_path: "unknown".to_string(),
    };

    // let device = Device::new_from_path("/dev/input/event20")?;
    // evdev_rs::AbsInfo::
    // device.has_event_type(ev_type)
    // device.grab(grab)
    // for prop in device.properties().iter() {
    //     println!("Property: {:?}", prop)
    // }
    // let battery_status = device
    //     .properties
    //     .get(&InputProperty::new(
    //         InputPropertyType::BatteryPercent,
    //         InputPropertyValue::Integer(0),
    //     ))
    //     .unwrap();
    // println!("Battery status is {}%", battery_status.value);

    Ok(controller)
}
