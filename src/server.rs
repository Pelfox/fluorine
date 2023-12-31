use std::net::SocketAddr;

use anyhow::Context;
use tokio::net::TcpListener;

use crate::connection::Connection;

pub struct DatabaseServer {
    addr: SocketAddr,
}

impl DatabaseServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn open(&mut self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.addr)
            .await
            .context("unable to start TCP listener")?;

        loop {
            let (stream, addr) = listener
                .accept()
                .await
                .context("unable to accept new connection")?;
            let mut connection = Connection::new(stream, addr);

            loop {
                let size = connection
                    .extend_buffer()
                    .await
                    .context("unable to extend the buffer")?;
                // client has been disconnected, closing the connection
                if size == -1 {
                    break;
                }
            }
        }
    }
}
