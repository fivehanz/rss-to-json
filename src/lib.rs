// use serde_json::json;
use worker::{*, Headers};

mod utils;
mod feed;

const RSS_URL : &str = "https://blog.fivehanz.xyz/rss.xml";

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}



#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/", |_, _| async move { 
            // rss xml to json string
            let feed_json_string = feed::feed_json_string(RSS_URL).await;
            
            // response with the string
            let res = Response::ok(
                feed_json_string
            );

            // set json headers
            let mut headers = Headers::new();
            headers.set("Content-Type", "application/json").unwrap();

            // return self
            Ok(Response::with_headers(res.unwrap(), headers))
        }) 
        .run(req, env)
        .await
}
