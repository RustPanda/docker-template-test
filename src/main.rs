use std::collections::BTreeMap;

use owo_colors::OwoColorize;
use xitca_web::{
    handler::{handler_service, json::Json},
    route::get,
    App, HttpServer,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_listener = std::net::TcpListener::bind("0.0.0.0:8080")?;

    println!(
        "Server up on: {} 🚀",
        tcp_listener.local_addr().unwrap().green()
    );

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
    .wait()
    .map_err(Into::into)
}
