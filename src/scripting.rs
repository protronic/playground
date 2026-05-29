use instant::Instant;
use rhai::ParseError;
use std::cell::RefCell;
use wasm_bindgen::JsValue;
use web_sys::console;

/// Callbacks that wire the Rhai NUS API to the JavaScript WebBluetooth layer.
pub struct NusCallbacks {
    pub connect: Box<dyn Fn()>,
    pub disconnect: Box<dyn Fn()>,
    pub send: Box<dyn Fn(String)>,
    pub receive: Box<dyn Fn() -> String>,
    pub is_connected: Box<dyn Fn() -> bool>,
}

fn build_engine(
    print_callback: impl Fn(&str) + 'static,
    debug_callback: impl Fn(&str) + 'static,
    led_callback: impl Fn(bool) + 'static,
    nus: Option<NusCallbacks>,
) -> rhai::Engine {
    let mut engine = rhai::Engine::new();
    engine
        .disable_symbol("eval")
        .on_print(move |s| print_callback(s))
        .on_debug(move |s, src, pos| {
            debug_callback(&src.map_or_else(
                || format!("<script>:[{}] {}", pos, s),
                |src| format!("{}:[{}] {}", src, pos, s),
            ))
        });

    let start = Instant::now();
    engine.register_fn("ts", move || -> i64 {
        (start.elapsed().as_secs_f64() * 32768.0) as i64
    });

    engine.register_fn("led", move |on: bool| -> bool {
        led_callback(on);
        on
    });

    if let Some(nus) = nus {
        let connect = nus.connect;
        engine.register_fn("nus_connect", move || connect());

        let disconnect = nus.disconnect;
        engine.register_fn("nus_disconnect", move || disconnect());

        let send = nus.send;
        engine.register_fn("nus_send", move |s: String| send(s));

        let receive = nus.receive;
        engine.register_fn("nus_receive", move || -> String { receive() });

        let is_connected = nus.is_connected;
        engine.register_fn("nus_is_connected", move || -> bool { is_connected() });
    }

    engine
}

pub fn run_script(
    script: &str,
    print_callback: impl Fn(&str) + 'static,
    debug_callback: impl Fn(&str) + 'static,
    progress_callback: impl Fn(u64) + 'static,
    led_callback: impl Fn(bool) + 'static,
) -> Result<String, String> {
    run_script_impl(script, print_callback, debug_callback, progress_callback, led_callback, None)
}

pub fn run_script_with_nus(
    script: &str,
    print_callback: impl Fn(&str) + 'static,
    debug_callback: impl Fn(&str) + 'static,
    progress_callback: impl Fn(u64) + 'static,
    led_callback: impl Fn(bool) + 'static,
    nus: NusCallbacks,
) -> Result<String, String> {
    run_script_impl(script, print_callback, debug_callback, progress_callback, led_callback, Some(nus))
}

fn run_script_impl(
    script: &str,
    print_callback: impl Fn(&str) + 'static,
    debug_callback: impl Fn(&str) + 'static,
    progress_callback: impl Fn(u64) + 'static,
    led_callback: impl Fn(bool) + 'static,
    nus: Option<NusCallbacks>,
) -> Result<String, String> {
    let mut engine = build_engine(print_callback, debug_callback, led_callback, nus);
    let script_ast = engine.compile(&script).map_err(|e| e.to_string())?;

    let interval = RefCell::new(1000);
    let last_instant = RefCell::new(Instant::now());
    engine.on_progress(move |ops| {
        let interval_value = *interval.borrow();
        if ops % interval_value == 0 {
            let mut last_instant = last_instant.borrow_mut();
            let new_instant = Instant::now();
            let duration_msec = new_instant.duration_since(*last_instant).as_millis();
            if duration_msec < 50 {
                interval.replace(interval_value * 10);
            } else if duration_msec >= 100 {
                progress_callback(ops);
                *last_instant = new_instant;
                if duration_msec >= 500 && interval_value > 1 {
                    interval.replace(interval_value / 10);
                }
            }
        }
        None
    });

    let result: rhai::Dynamic = engine.eval_ast(&script_ast).map_err(|e| e.to_string())?;
    Ok(result.to_string())
}

thread_local! {
    static ENGINE_FOR_AST_ONLY: rhai::Engine = {
        let mut engine = rhai::Engine::new();
        engine.set_optimization_level(rhai::OptimizationLevel::None);
        engine
    };
}

pub fn compile_ast(script: &str) -> Result<String, JsValue> {
    ENGINE_FOR_AST_ONLY.with(|engine| {
        let script_ast = engine.compile(&script).map_err(parse_error_to_js)?;
        console::log_1(&JsValue::from_str("Script compiled to AST!"));
        #[allow(deprecated)]
        let statements = script_ast.statements();
        #[allow(deprecated)]
        let module = script_ast.lib();
        let mut s = format!("//This is the Debug representation of the AST.\n\n// Statements:\n{:#?}\n\n// Modules (script-defined functions):\n", statements);
        for f in module.iter_script_fn_info() {
            use std::fmt::Write;
            writeln!(&mut s, "{:#?}", &f).unwrap();
        }
        Ok(s)
    })
}

#[derive(serde::Serialize)]
struct OutParseError {
    message: String,
    line: Option<u32>,
    column: Option<u32>,
}

fn parse_error_to_js(e: ParseError) -> JsValue {
    let ParseError(err, pos) = e;
    let res = OutParseError {
        message: err.to_string(),
        line: pos.line().map(|x| x as u32),
        column: pos.position().map(|x| x as u32),
    };
    serde_wasm_bindgen::to_value(&res).unwrap()
}
