use std::{io::Cursor, net::SocketAddr};

use anyhow::Context;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

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
        for mut packet in packets {
            match packet.id {
                0x0 => {
                    let version = packet.data.read_string().await?;
                    let enable_compression = packet.data.read_bool().await?;
                    let compression_threshold = packet.data.read_i64().await?;
                    println!("Got handshake packet. Client's version: {version}, enable compression: {enable_compression}, threshold: {compression_threshold}");
                    self.write(Packet {
                        id: 0x01,
                        length: 0,
                        data: ProtoBuf::new(vec![]),
                    })
                    .await?;
                }
                _ => {
                    println!("Got unknown packet.")
                }
            }
        }

        Ok(())
    }

    pub async fn extend_buffer(&mut self) -> anyhow::Result<isize> {
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
                Ok(n as isize)
            }
            Ok(_) => Ok(-1), // client has been disconnected, closing the connection
            Err(e) => Err(anyhow::anyhow!(e).context("unable to read from the stream")),
        }
    }

    pub async fn write(&mut self, packet: Packet) -> anyhow::Result<()> {
        let mut buffer = Cursor::new(vec![]);
        buffer
            .write_u8(packet.id)
            .await
            .context("unable to write packet's id")?;
        buffer
            .write_all(&packet.data.write_buffer)
            .await
            .context("unable to write the packet")?;

        let mut packet_buffer = Cursor::new(vec![]);
        packet_buffer
            .write_u8(buffer.get_ref().len() as u8)
            .await
            .context("unable to write packet's length")?;
        packet_buffer
            .write(buffer.get_ref())
            .await
            .context("unable to write packet's buffer")?;

        println!("{:#?}", packet_buffer.get_ref());
        self.stream
            .write_all(packet_buffer.get_ref())
            .await
            .context("unable to write the packet into the stream")?;

        Ok(())
    }
}
