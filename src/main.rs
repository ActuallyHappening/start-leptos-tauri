use leptos::*;
use leptos_app::app::*;
use leptos_app::app::*;
use log::info;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
	use actix_files::Files;
	use actix_web::*;
	use leptos_actix::{generate_route_list, LeptosRoutes};

	let conf = get_configuration(None).await.unwrap();
	let addr = conf.leptos_options.site_addr;
	// Generate the list of routes in your Leptos App
	let routes = generate_route_list(|cx| view! { cx, <App/> });

	HttpServer::new(move || {
		let leptos_options = &conf.leptos_options;
		let site_root = &leptos_options.site_root;

		App::new()
			.route("/api/{tail:.*}", leptos_actix::handle_server_fns())
			.leptos_routes(
				leptos_options.to_owned(),
				routes.to_owned(),
				|cx| view! { cx, <App/> },
			)
			.service(Files::new("/", site_root))
		//.wrap(middleware::Compress::default())
	})
	.bind(&addr)?
	.run()
	.await
}

#[cfg(not(any(feature = "ssr", feature = "ssg")))]
compile_error!("Must enable either `ssr` or `ssg` feature");

#[cfg(all(feature = "ssr", feature = "ssg"))]
compile_error!("Cannot enable both `ssr` and `ssg` features");

#[cfg(feature = "ssg")]
pub fn main() {
	// a client-side main function is required for using `trunk serve`
	// prefer using `cargo leptos serve` instead
	// to run: `trunk serve --open --features ssg`

	console_error_panic_hook::set_once();
	console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

	info!("Running leptos application as ssg ...");

	#[cfg(feature = "tauri")]
	info!("invoking tauri command from front end ...");
	#[cfg(feature = "tauri")]
	invoke_example_command();

	leptos::mount_to_body(move |cx| {
		// note: for testing it may be preferrable to replace this with a
		// more specific component, although leptos_router should still work
		view! {cx, <App/> }
	});
}

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[cfg(feature = "tauri")]
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = "window.__TAURI__.tauri.invoke", catch)]
	async fn tauri_invoke(cmd: &str, args: js_sys::Object) -> Result<JsValue, js_sys::Error>;
}

#[cfg(feature = "tauri")]
fn invoke_example_command() {
	use js_sys::JsString;

	wasm_bindgen_futures::spawn_local(async {
		let ret = tauri_invoke("command_example", js_sys::Object::new())
			.await
			.expect("No errors from js interop");
		let str: String = JsString::from(ret).into();
		info!("tauri_invoke returned: {}", str);
	})
}
