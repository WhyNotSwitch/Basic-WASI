
extern "C" {
    fn ws_get_data(event_id: i32, return_ptr: *const *mut u8, return_size: *const i32) -> i32;
    fn ws_log(log_level: i32, ptr: *const u8, size: i32) -> i32;
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
pub extern "C" fn handle_event(event_id: i32) -> i32 {
    log_info("start from rust");
    log_info(&format!("Handler called with event_id: {}", event_id));

    let data_u8 = match get_data(event_id) {
        Some(data) => data,
        _ => {
            log_info("failed to get data from event");
            return -1;
        }
    };
    
    let data_str = match String::from_utf8(data_u8) {
        Ok(data) => data,
        _ => {
            log_info("failed to convert data to string");
            return -1;
        }
    };

    log_info(&format!("data: {}", data_str));
    log_info(&format!("data: {}", data_str));
    return 0;
}


pub fn get_data(resource_id: i32) -> Option<Vec<u8>> {
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &(0 as i32);
    match unsafe { ws_get_data(resource_id, data_ptr, data_size) } {
        0 => Some(unsafe { Vec::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => None,
    }
}


pub fn log_info(str: &str) {
    unsafe { ws_log(3, str.as_bytes().as_ptr(), str.len() as _) };
}


pub fn main() {}