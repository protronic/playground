use wasm_bindgen::prelude::*;

mod alloc_tracker;
mod cm_rhai_mode;
mod codemirror;
mod playground;
mod scripting;

#[wasm_bindgen]
pub fn run_script(
    script: String,
    print_callback: js_sys::Function,
    debug_callback: js_sys::Function,
    progress_callback: Option<js_sys::Function>,
    led_callback: Option<js_sys::Function>,
) -> Result<String, JsValue> {
    Ok(scripting::run_script(
        &script,
        move |s| {
            let _ = print_callback.call1(&JsValue::null(), &JsValue::from_str(s));
        },
        move |s| {
            let _ = debug_callback.call1(&JsValue::null(), &JsValue::from_str(s));
        },
        move |ops| {
            if let Some(f) = &progress_callback {
                let _ = f.call1(&JsValue::null(), &JsValue::from_f64(ops as f64));
            }
        },
        move |on| {
            if let Some(f) = &led_callback {
                let _ = f.call1(&JsValue::null(), &JsValue::from_bool(on));
            }
        },
    )?)
}

/// Variant of `run_script` that also registers NUS (Nordic UART Service) Rhai
/// functions backed by JavaScript WebBluetooth callbacks.
///
/// The NUS callbacks are called synchronously from Rhai scripts:
/// - `nus_connect_callback()` — initiate a BLE scan/connect (async, fire-and-forget)
/// - `nus_disconnect_callback()` — disconnect (synchronous)
/// - `nus_send_callback(data: string)` — write to the NUS RX characteristic
/// - `nus_receive_callback() -> string` — poll the receive buffer; returns `""` when empty
/// - `nus_is_connected_callback() -> bool` — connection status
#[wasm_bindgen]
pub fn run_script_with_nus(
    script: String,
    print_callback: js_sys::Function,
    debug_callback: js_sys::Function,
    progress_callback: Option<js_sys::Function>,
    led_callback: Option<js_sys::Function>,
    nus_connect_callback: js_sys::Function,
    nus_disconnect_callback: js_sys::Function,
    nus_send_callback: js_sys::Function,
    nus_receive_callback: js_sys::Function,
    nus_is_connected_callback: js_sys::Function,
) -> Result<String, JsValue> {
    let nus = scripting::NusCallbacks {
        connect: Box::new(move || {
            let _ = nus_connect_callback.call0(&JsValue::null());
        }),
        disconnect: Box::new(move || {
            let _ = nus_disconnect_callback.call0(&JsValue::null());
        }),
        send: Box::new(move |s: String| {
            let _ = nus_send_callback.call1(&JsValue::null(), &JsValue::from_str(&s));
        }),
        receive: Box::new(move || {
            nus_receive_callback
                .call0(&JsValue::null())
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_default()
        }),
        is_connected: Box::new(move || {
            nus_is_connected_callback
                .call0(&JsValue::null())
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        }),
    };
    Ok(scripting::run_script_with_nus(
        &script,
        move |s| {
            let _ = print_callback.call1(&JsValue::null(), &JsValue::from_str(s));
        },
        move |s| {
            let _ = debug_callback.call1(&JsValue::null(), &JsValue::from_str(s));
        },
        move |ops| {
            if let Some(f) = &progress_callback {
                let _ = f.call1(&JsValue::null(), &JsValue::from_f64(ops as f64));
            }
        },
        move |on| {
            if let Some(f) = &led_callback {
                let _ = f.call1(&JsValue::null(), &JsValue::from_bool(on));
            }
        },
        nus,
    )?)
}

#[wasm_bindgen]
pub fn compile_script(script: String) -> Result<String, JsValue> {
    Ok(scripting::compile_ast(&script)?)
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
// NOTE: wee_alloc conflicts with alloc_tracker's #[global_allocator]. Don't enable both.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Returns currently live (allocated but not freed) bytes tracked by the custom allocator.
/// Call this before a script run to get the baseline.
#[wasm_bindgen]
pub fn alloc_live_bytes() -> u32 {
    alloc_tracker::live_bytes()
}

/// Returns the peak live bytes since the last `alloc_reset_peak()` call.
/// Subtract the pre-run baseline from this to get peak net allocation of the run.
#[wasm_bindgen]
pub fn alloc_peak_bytes() -> u32 {
    alloc_tracker::peak_bytes()
}

/// Resets the peak counter to current live bytes.
/// Call immediately before each script run.
#[wasm_bindgen]
pub fn alloc_reset_peak() {
    alloc_tracker::reset_peak();
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}
