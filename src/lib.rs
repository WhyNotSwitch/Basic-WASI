
mod sdk;

#[no_mangle]
pub extern "C" fn start(_resource_id: i32) -> i32 {
    let str = String::from("hello world");
    sdk::log_info(&str);
    return 0;
}

#[no_mangle]
pub extern "C" fn handle_event(_resource_id: i32) -> i32 {
    sdk::log_info(
        &format!("Handler called with event_id: {}", _resource_id));

    let payload = get_data_as_str(_resource_id);
    sdk::log_info(&format!("event data as string: {}", payload));
    return 0;
}

// Returns the event payload as a string
fn get_data_as_str(event_id: i32) -> String {

    return match sdk::get_data(event_id) {
        Some(data) => match String::from_utf8(data) {
            Ok(data) => data,
            _ => String::from("convert from utf8 to string failed")
        },
        _ => String::from("get data from log failed")
    };
}

pub fn main() {}