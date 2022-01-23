/// Logs the given `format!`-style data to the browser's console, or to stdout as usual on the server.
#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! web_log {
    ($format_str:literal $(, $data:expr)*) => {
        $crate::internal::log::log_js_value(
            &$crate::internal::log::JsValue::from(
                format!($format_str $(, $data)*)
            )
        );
    };
}

/// Logs the given `format!`-style data to the browser's console, or to stdout on the server.
#[macro_export]
#[cfg(not(target_arch = "wasm32"))]
macro_rules! web_log {
    ($format_str:literal $(, $data:expr)*) => {
        println!($format_str $(, $data)*)
    };
}
