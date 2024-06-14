use worker::*;
use serde::Deserialize;
use serde::Serialize;
use elasticsearch::Elasticsearch;

mod utils;

// Types returned by the 'info' endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    pub number: String,
    pub build_date: String,
    pub build_hash: String,
    pub build_snapshot: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub name: String,
    pub cluster_name: String,
    pub version: VersionInfo,
    pub tagline: String,
}

// Global application context
struct AppContext {
    pub es_client: Elasticsearch,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Fetch secrets and create ES client
    let url =  env.secret("ES_URL")?.to_string();
    let login =  env.secret("ES_LOGIN")?.to_string();
    let password =  env.secret("ES_PASSWORD")?.to_string();

    let router = Router::with_data(AppContext {
        es_client: create_client(url, login, password)
            .map_err(|e| Error::RustError(e.to_string()))?
    });

    // HTTP request router
    router
        .get("/", |_, _| {
            Response::from_html("Hello from Workers! Please go to <a href='/es'>/es</a>")
        })
        .get_async("/es", |mut req, ctx| async move {
            let result = call_es(ctx.data.es_client).await
                .map_err(|e| Error::RustError(e.to_string()))?;
            Response::ok(result)
        })
        .run(req, env)
        .await
}

fn create_client(url: String, login: String, password: String) -> StdResult<Elasticsearch, elasticsearch::Error> {

    let url = url::Url::parse(&url)?;
    let pool = elasticsearch::http::transport::SingleNodeConnectionPool::new(url);
    let credentials = elasticsearch::auth::Credentials::Basic(login.into(), password.into());
    let transport = elasticsearch::http::transport::TransportBuilder::new(pool)
        .auth(credentials)
        .build()?;

    Ok(Elasticsearch::new(transport))
}

pub async fn call_es(client: Elasticsearch) -> std::result::Result<String, elasticsearch::Error> {
    let response = client.info().send().await?;
    let info = response.json::<InfoResponse>().await?;

    Ok(format!("version: {}, build_date: {}", info.version.number, info.version.build_date))
}

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}
