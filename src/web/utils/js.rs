use web_sys::console;

pub fn js_log(message: String) {
    console::log_1(&message.into());
}
