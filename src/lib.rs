use worker::*;

mod feed;
mod utils;

const RSS_URL: &str = "https://devblog.hanz.lol/rss.xml";

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

fn get_json_headers() -> Headers {
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json").unwrap();
    headers.set("Access-Control-Allow-Origin", "*").unwrap();
    headers
        .set(
            "Access-Control-Allow-Headers",
            "Origin, Content-Type, Accept",
        )
        .unwrap();

    headers
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .post_async("/", |mut req, _ctx| async move {
            let data: FormData = match req.form_data().await {
                Ok(data) => data,
                Err(_) => return Response::error("Bad Request", 400),
            };

            let url = match data.get("url") {
                Some(FormEntry::Field(s)) => s,
                _ => return Response::error("Bad Request", 400),
            };

            let feed_json_string = feed::feed_json_string(&url).await;
            let res = Response::ok(feed_json_string.unwrap());
            Ok(Response::with_headers(res.unwrap(), get_json_headers()))
        })
        .get_async("/", |_, _| async move {
            let feed_json_string = feed::feed_json_string(&RSS_URL).await;
            let res = Response::ok(feed_json_string.unwrap());
            Ok(Response::with_headers(res.unwrap(), get_json_headers()))
        })
        .run(req, env)
        .await
}
