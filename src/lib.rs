
// use core::slice::SlicePattern;

use std::collections::HashMap;

use anyhow::{bail, Result};
use serde_json::Value;
use serde::{Serialize, Deserialize};

use reqwest_wasm::Client;

extern "C" {
    fn ws_log(log_level: i32, ptr: *const u8, size: i32) -> i32;
    fn ws_get_data(event_id: i32, return_ptr: *const *mut u8, return_size: *const i32) -> i32;
    fn ws_set_db(key_ptr: *const u8, key_size: i32, value_ptr: *const u8, value_size: i32) -> i32;
    fn ws_get_db(hey_ptr: *const u8, key_size: i32, return_ptr: *const *mut u8, return_size: &i32) -> i32;
}

#[no_mangle]
pub extern "C" fn alloc(size: i32) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(size as _);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr;
}


#[no_mangle]
pub extern "C" fn start(_resource_id: i32) -> i32 {
    let str = String::from("####### Hello World! #######");
    log_info(&str);
    return 0;
}


#[no_mangle]
pub extern "C" fn handle_read_event(event_id: i32) -> i32 {
    log_info("start from rust");
    log_info(&format!("Handler called with event_id: {}", event_id));

    let data_str = match get_data(event_id) {
        Some(data) => match String::from_utf8(data) {
            Ok(data) => data,
            _ => {
                log_info("failed to convert data to string");
                return -1;
            },
        }
        _ => {
            log_info("failed to get data from event");
            return -1;
        }
    };

    log_info(&format!("data: {}", data_str));
    return 0;
}

#[no_mangle]
pub extern "C" fn handle_set_db_event(event_id: i32) -> i32 {
    log_info(&format!("Handler called with event_id: {}", event_id));
    let data_json: Value = match get_data(event_id) {
        Some(data) => match serde_json::from_slice(data.as_slice()) {
            Ok(value) => value,
            _ => {
                log_info(&"failed to read data json");
                return -1;
            }
        },
        _ => {
            log_info(&"failed to get data");
            return -1;
        }
    };

    match sink_data(&data_json) {
        Ok(()) => 0,
        _ => {
            log_info(&"set to db failed");
            return -1;
        }
    }
}

#[no_mangle]
pub extern "C" fn handle_get_db_event(event_id: i32) -> i32 {
    log_info(&format!("Handler called with event_id: {}", event_id));
    return 0;
}

#[no_mangle]
pub async extern "C" fn handle_confirmation_event(event_id: i32) -> i32 {
    log_info(&format!("Handler called with event_id: {}", event_id));

    let payload = match get_data(event_id) {
        Some(data) => match serde_json::from_slice::<ConfirmationPayload>(data.as_slice()) {
            Ok(data)=> data,
            Err(error) => {
                log_info(&format!("failed to read data json with error {}", error.to_string()));
                return -1;
            }
        },
        None => {
            log_info(&"failed to get data");
            return -1;
        }
    };

    match send_confirmation_check(&payload.confirmation_url, payload.ref_number).await {
        Ok(_data) => log_info(&"confirmed"),
        Err(_error) => log_info(&"failed")
    };

    return 0;
}


pub fn log_info(str: &str) {
    unsafe { ws_log(3, str.as_bytes().as_ptr(), str.len() as _) };
}

pub fn get_data(resource_id: i32) -> Option<Vec<u8>> {
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &(0 as i32);
    match unsafe { ws_get_data(resource_id, data_ptr, data_size) } {
        0 => Some(unsafe { Vec::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => None,
    }
}

pub fn set_db(key: &String, value: Vec<u8>) -> Result<()> {
    match unsafe {
        ws_set_db(
            key.as_ptr(),
            key.len() as _,
            value.as_ptr(),
            value.len() as _,
        )
    } {
        0 => Ok(()),
        _ => bail!("fail to set db"),
    }
}

pub fn get_db(key: &String) -> Option<Vec<u8>> {
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &(0 as i32);
    match unsafe { ws_get_db(key.as_ptr(), key.len() as _, data_ptr, data_size) } {
        0 => Some(unsafe { Vec::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => None,
    }
}

// method to add value to db
// expect data in 
fn sink_data(data: &Value) -> Result<()> {
    let id = data["device_id"].as_str().unwrap().to_string();

    let mut value: Vec<Record> = match get_db(&id) {
        Some(data) => serde_json::from_slice(data.as_slice())?,
        None => vec![]
    };

    value.push(Record {
        device_id: data["device_id"].as_str().unwrap().to_string(),
        signature: data["signature"].as_str().unwrap().to_string(),
    });

    set_db(&id, serde_json::to_string(&value)?.into_bytes())
}

async fn send_confirmation_check(request_url: &String, ref_id: String) -> Result<()> {

    let client = Client::new();
    // let res = client.get(request_url).send().await?;

    let res = client
        .post(request_url)
        // .header("accept", "application/json")
        .body(ref_id)
        .send()
        .await?;
    
    let raw = res.text().await?;

    let data = match serde_json::from_str::<HashMap<String, Value>>(raw.as_str()) {
        Ok(data) => data,
        Err(err) => panic!("failed with {}", err.to_string())
    };
    
    log_info(&data.get("body").unwrap().as_str().unwrap());
    Ok(())
}

// Expected Value format in json
// payload: {
//   device_id: 001, 
//   signature: random_string
// }
#[derive(Serialize, Deserialize, Debug)]
struct Record {
    device_id: String,
    signature: String
}

#[derive(Deserialize, Debug)]
struct ConfirmationPayload {
    confirmation_url: String,
    ref_number: String
}


pub fn main() {}