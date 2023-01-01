/// Logs the given `format!`-style data to the browser's console, or to stdout
/// as usual on the server.
#[macro_export]
#[cfg(client)]
macro_rules! web_log {
    ($format_str:literal $(, $data:expr)*) => {
        $crate::log::log_js_value(
            &$crate::log::JsValue::from(
                format!($format_str $(, $data)*)
            )
        );
    };
}

/// Logs the given `format!`-style data to the browser's console, or to stdout
/// on the server.
#[macro_export]
#[cfg(engine)]
macro_rules! web_log {
    ($format_str:literal $(, $data:expr)*) => {
        println!($format_str $(, $data)*)
    };
}
