/// Logs the given `format!`-style data to the browser's console.
#[macro_export]
macro_rules! web_log {
    ($format_str:literal $(, $data:expr)*) => {
        ::web_sys::console::log_1(
            &::wasm_bindgen::JsValue::from(
                format!($format_str $(, $data)*)
            )
        );
    };
}
