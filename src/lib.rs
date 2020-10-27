#[no_mangle]
pub extern "C" fn on_load() {
    let _ = std::panic::take_hook();
}
