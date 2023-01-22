
#[link(wasm_import_module = "env")]
extern "C" {
    fn ws_log(log_level: i32, ptr: *const u8, size: i32) -> i32;
    fn ws_get_data(event_id: i32, payload_ptr: *const *mut u8, payload_size: *const i32) -> i32;
}

#[no_mangle]
pub extern "C" fn start(_resource_id: i32) -> i32 {
    let str = String::from("hello world");
    log_info(&str);
    return 0;
}

#[no_mangle]
pub extern "C" fn handle_event(_resource_id: i32) -> i32 {
    log_info(
        &format!("Handler called with event_id: {}", _resource_id));

    let payload = get_data_as_str(_resource_id).unwrap();
    log_info(&format!("event data as string: {}", payload));
    return 0;
}

// Returns the event payload as a string
fn get_data_as_str(event_id: i32) -> Option<String> {
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &(3 as i32);
    log_info(&String::from("Before call to get Data"));

    let call: i32 = unsafe { ws_get_data(event_id, data_ptr, data_size) };

    if let 0 = call {
        return  Option::Some(String::from("call successful"));
    } else {
        //   log_info(&String::from("after call to get Data"));
        return Option::Some(String::from("call failed"));
    }
}

// Logs an info message string to the W3bstream console
fn log_info(str: &str) {
    unsafe { ws_log(3, str.as_bytes().as_ptr(), str.len() as _) };
}

pub fn main() {}