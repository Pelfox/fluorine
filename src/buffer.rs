use std::io::Cursor;

use anyhow::Context;

// use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use tokio::io::AsyncReadExt;

#[derive(Debug, Default)]
pub struct ProtoBuf {
    cursor: Cursor<Vec<u8>>,
}

impl ProtoBuf {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            cursor: Cursor::new(bytes),
        }
    }

    pub fn extend(&mut self, bytes: &[u8]) {
        self.cursor.get_mut().extend_from_slice(bytes)
    }

    pub async fn read_u8(&mut self) -> anyhow::Result<u8> {
        self.cursor.read_u8().await.context("unable to read u8")
    }

    pub async fn get_sized_vec(&mut self, size: usize) -> anyhow::Result<Vec<u8>> {
        let mut buffer = vec![0u8; size];
        self.cursor
            .read_exact(&mut buffer)
            .await
            .context("unable to get sized vec")?;
        Ok(buffer)
    }

    pub async fn get_sized_protobuf(&mut self, size: usize) -> anyhow::Result<ProtoBuf> {
        let sized_vec = self.get_sized_vec(size).await?;
        Ok(ProtoBuf::new(sized_vec))
    }
}

#[derive(Debug)]
pub struct Packet {
    pub id: u8,
    pub length: u8,
    pub data: ProtoBuf,
}
