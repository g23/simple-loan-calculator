use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn js_log(s: String);
}

#[macro_export]
macro_rules! log {
    ($($args:tt)*) => {{
        use $crate::utils::js_log;
        js_log(format!($($args)*));
    }};
}
