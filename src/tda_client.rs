use std::{convert::Infallible, net::SocketAddr, time::Duration};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use tokio::time::sleep;

pub struct Client {}

impl Client {
    pub async fn auth_server() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 7043));
        let make_svc =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

        let server = Server::bind(&addr)
            .serve(make_svc)
            .with_graceful_shutdown(shutdown_signal());

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

async fn shutdown_signal() {
    sleep(Duration::from_millis(10000)).await;
}
