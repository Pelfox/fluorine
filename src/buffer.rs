use std::io::Cursor;

use anyhow::Context;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Default)]
pub struct ProtoBuf {
    read_cursor: Cursor<Vec<u8>>,
    pub(crate) write_buffer: Vec<u8>,
}

impl ProtoBuf {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            read_cursor: Cursor::new(bytes),
            write_buffer: vec![],
        }
    }

    pub fn extend(&mut self, bytes: &[u8]) {
        self.read_cursor.get_mut().extend_from_slice(bytes)
    }

    pub async fn read_u8(&mut self) -> anyhow::Result<u8> {
        self.read_cursor
            .read_u8()
            .await
            .context("unable to read u8")
    }

    pub async fn write_u8(&mut self, value: u8) -> anyhow::Result<()> {
        self.write_buffer
            .write_u8(value)
            .await
            .context("unable to write u8")
    }

    pub async fn get_sized_vec(&mut self, size: usize) -> anyhow::Result<Vec<u8>> {
        let mut buffer = vec![0u8; size];
        self.read_cursor
            .read_exact(&mut buffer)
            .await
            .context("unable to get sized vec")?;
        Ok(buffer)
    }

    pub async fn get_sized_protobuf(&mut self, size: usize) -> anyhow::Result<ProtoBuf> {
        let sized_vec = self.get_sized_vec(size).await?;
        Ok(ProtoBuf::new(sized_vec))
    }

    pub async fn read_string(&mut self) -> anyhow::Result<String> {
        let string_length = self
            .read_u8()
            .await
            .context("unable to read string's length")?;
        let mut string_buffer = vec![0u8; string_length as usize];
        self.read_cursor
            .read_exact(&mut string_buffer)
            .await
            .context("unable to fill string's buffer")?;
        String::from_utf8(string_buffer).context("unable to construct string from bytes")
    }

    pub async fn write_string(&mut self, value: String) -> anyhow::Result<()> {
        self.write_u8(value.len() as u8)
            .await
            .context("unable to write string's length")?;
        self.write_buffer.extend(value.as_bytes());
        Ok(())
    }

    pub async fn read_bool(&mut self) -> anyhow::Result<bool> {
        let value = self
            .read_u8()
            .await
            .context("unable to read bool's value")?;
        Ok(value == 1)
    }

    pub async fn write_bool(&mut self, value: bool) -> anyhow::Result<()> {
        self.write_u8(if value { 1 } else { 0 })
            .await
            .context("unable to write bool's value")
    }

    pub async fn read_i64(&mut self) -> anyhow::Result<i64> {
        self.read_cursor
            .read_i64()
            .await
            .context("unable to read i64")
    }

    pub async fn write_i64(&mut self, value: i64) -> anyhow::Result<()> {
        self.write_buffer
            .write_i64(value)
            .await
            .context("unable to write i64")
    }

    pub async fn write_all(&mut self, buffer: ProtoBuf) -> anyhow::Result<()> {
        self.write_buffer.extend(buffer.write_buffer);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Packet {
    pub id: u8,
    pub length: u8,
    pub data: ProtoBuf,
}
