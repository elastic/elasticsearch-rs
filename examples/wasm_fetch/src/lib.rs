use wasm_bindgen::prelude::*;
use serde::Deserialize;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Deserialize)]
pub struct VersionInfo {
    pub number: String,
    pub build_date: String,
    pub build_hash: String,
    pub build_snapshot: bool
}

#[derive(Deserialize)]
pub struct InfoResponse {
    pub name: String,
    pub cluster_name: String,
    pub version: VersionInfo,
    pub tagline: String,
}

#[wasm_bindgen]
pub async fn call_es(url: String, login: String, password: String) -> Result<JsValue, JsError> {
    set_panic_hook();

    web_sys::console::log_1(&"starting".into());

    let url = url::Url::parse(&url)?;
    let pool = elasticsearch::http::transport::SingleNodeConnectionPool::new(url);
    let mut transport = elasticsearch::http::transport::TransportBuilder::new(pool);

    if !login.is_empty() {
        let credentials = elasticsearch::auth::Credentials::Basic(login.into(), password.into());
        transport = transport.auth(credentials)
    }

    let transport = transport.build()?;
    let client = elasticsearch::Elasticsearch::new(transport);

    let response = client.info().send().await?;

    let product_header: &str = response.headers()
        .get("X-Elastic-Product")
        .map(|h| h.to_str().unwrap())
        .unwrap_or("missing product header");

    web_sys::console::log_1(&JsValue::from_str(product_header));

    let info = response.json::<InfoResponse>().await?;

    let result = format!("version: {}, build date: {}", info.version.number, info.version.build_date);

    Ok((&result).into())
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
