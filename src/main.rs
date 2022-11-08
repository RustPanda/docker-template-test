use std::collections::BTreeMap;

use xitca_web::{
    handler::{handler_service, json::Json},
    route::get,
    App, HttpServer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_listener = std::net::TcpListener::bind("0.0.0.0:8080")?;

    HttpServer::new(|| {
        App::new()
            .at(
                "/",
                get(handler_service(|| async {
                    Json::<_>(std::env::vars().collect::<BTreeMap<String, String>>())
                })),
            )
            .finish()
    })
    .listen(tcp_listener)?
    .run()
    .await
    .map_err(Into::into)
}
