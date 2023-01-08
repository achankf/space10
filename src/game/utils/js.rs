use web_sys::console;

pub fn log(message: String) {
    console::log_1(&message.into());
}
