use std::net::SocketAddr;

use anyhow::Context;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::buffer::{Packet, ProtoBuf};

pub struct Connection {
    stream: TcpStream,
    addr: SocketAddr,
    buffer: ProtoBuf,
}

impl Connection {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        Self {
            stream,
            addr,
            buffer: ProtoBuf::default(),
        }
    }

    async fn read(&mut self) -> anyhow::Result<Vec<Packet>> {
        let mut packets = vec![];
        while let Ok(length) = self.buffer.read_u8().await {
            if length == 0 {
                break;
            }

            let id = self
                .buffer
                .read_u8()
                .await
                .context("unable to read packet's id")?;
            let packet_buffer = self
                .buffer
                .get_sized_protobuf(length as usize)
                .await
                .context("unable to get packet's buffer")?;

            packets.push(Packet {
                id,
                length,
                data: packet_buffer,
            });
        }

        Ok(packets)
    }

    async fn process_packets(&mut self, packets: Vec<Packet>) -> anyhow::Result<()> {
        println!("got packets {:#?}", packets);
        Ok(())
    }

    pub async fn extend_buffer(&mut self) -> anyhow::Result<()> {
        let mut buffer = vec![0; 1024];
        match self.stream.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                self.buffer.extend(buffer.as_slice());
                let packets = self
                    .read()
                    .await
                    .context("unable to read from the buffer")?;

                self.process_packets(packets)
                    .await
                    .context("unable to process the packets")?;
                Ok(())
            }
            Ok(_) => Ok(()), // client got disconnected
            Err(e) => Err(anyhow::anyhow!(e).context("unable to read from the stream")),
        }
    }
}
