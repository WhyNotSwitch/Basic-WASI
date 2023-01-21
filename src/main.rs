
#[link(wasm_import_module = "env")]
extern "C" {
    fn ws_log(log_level: i32, ptr: *const u8, size: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn start(resource_id: i32) -> i32 {
    let log_level_info: i32 = 3;
    let str = String::from("hello world");
    unsafe { ws_log(log_level_info, str.as_bytes().as_ptr(), str.len() as _) };
    return 0;
}

pub fn main() {}