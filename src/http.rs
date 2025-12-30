use embassy_net::Stack;
use log::info;

use embassy_time::Duration;
use picoserve::routing::get;
use picoserve::{Config, Router, Server, Timeouts};

pub struct HttpServer<'a> {
    stack: Stack<'a>,
    port: u16,
}

impl<'a> HttpServer<'a> {
    pub fn new(stack: Stack<'a>, port: u16) -> Self {
        Self { stack, port }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let port = self.port;
        info!("HTTP server listening on port {port}");

        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }));
        let config = Config::new(Timeouts {
            start_read_request: Some(Duration::from_secs(10)),
            persistent_start_read_request: Some(Duration::from_secs(10)),
            read_request: Some(Duration::from_secs(10)),
            write: Some(Duration::from_secs(10)),
        })
        .close_connection_after_response();

        // Embassy net stack: listen for TCP connections
        let mut rx_buffer = [0; 2048];
        let mut tx_buffer = [0; 2048];
        let mut http_buffer = [0u8; 2048];
        let task_id = 1; // Unique task ID for picoserve
        Server::new(&app, &config, &mut http_buffer)
            .listen_and_serve(task_id, self.stack, port, &mut rx_buffer, &mut tx_buffer)
            .await
            .into_never()
    }
}
