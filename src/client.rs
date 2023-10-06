use tokio::io::{AsyncRead, AsyncWrite};

use crate::builder::Builder;
use crate::lexer::Lexer;
use crate::lexi_type::LexiType;
use crate::parser::Parser;

pub trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin {}

impl AsyncReadWrite for tokio::net::TcpStream {}

pub struct Client {
    addr: std::net::SocketAddr,
    stream: Option<tokio::net::TcpStream>,
}

impl Client {
    pub fn new(address: &str) -> anyhow::Result<Self> {
        let addr: std::net::SocketAddr = address.parse()?;
        Ok(Client { addr, stream: None })
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        let socket = tokio::net::TcpSocket::new_v4()?;
        let stream = socket.connect(self.addr).await?;
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn set(&mut self, key: &str, value: impl Into<LexiType>) -> anyhow::Result<LexiType> {
        let buf = Self::build_set_command(key, value)?;
        let _ = self.write(buf).await?;
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn get(&mut self, key: &str) -> anyhow::Result<LexiType> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("GET")
            .add_bulk(key)
            .out();
        let _ = self.write(buf).await?;
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn del(&mut self, key: &str) -> anyhow::Result<LexiType> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("DEL")
            .add_bulk(key)
            .out();
        let _ = self.write(buf).await?;
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn push(&mut self, value: impl Into<LexiType>) -> anyhow::Result<LexiType> {
        let buf = Self::build_push_cmd(value)?;
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn pop(&mut self) -> anyhow::Result<LexiType> {
        let buf = Builder::new().add_bulk("POP").out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn keys(&mut self) -> anyhow::Result<LexiType> {
        let buf = Builder::new().add_bulk("KEYS").out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn values(&mut self) -> anyhow::Result<LexiType> {
        let buf = Builder::new().add_bulk("VALUES").out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn entries(&mut self) -> anyhow::Result<LexiType> {
        let buf = Builder::new().add_bulk("ENTRIES").out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn cluster_new(&mut self, name: &str) -> anyhow::Result<LexiType> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("CLUSTER.NEW")
            .add_bulk(name)
            .out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn cluster_set(
        &mut self,
        name: &str,
        key: &str,
        value: impl Into<LexiType>,
    ) -> anyhow::Result<LexiType> {
        let buf = Self::build_cluster_set_command(name, key, value)?;
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn cluster_get(&mut self, name: &str, key: &str) -> anyhow::Result<LexiType> {
        let buf = Builder::new()
            .add_arr(3)
            .add_bulk("CLUSTER.GET")
            .add_bulk(name)
            .add_bulk(key)
            .out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn cluster_del(&mut self, name: &str, key: &str) -> anyhow::Result<LexiType> {
        let buf = Builder::new()
            .add_arr(3)
            .add_bulk("CLUSTER.DEL")
            .add_bulk(name)
            .add_bulk(key)
            .out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn cluster_keys(&mut self, name: &str) -> anyhow::Result<LexiType> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("CLUSTER.KEYS")
            .add_bulk(name)
            .out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn cluster_values(&mut self, name: &str) -> anyhow::Result<LexiType> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("CLUSTER.VALUES")
            .add_bulk(name)
            .out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn cluster_entries(&mut self, name: &str) -> anyhow::Result<LexiType> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("CLUSTER.ENTRIES")
            .add_bulk(name)
            .out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    pub async fn cluster_drop(&mut self, name: &str) -> anyhow::Result<LexiType> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("CLUSTER.DROP")
            .add_bulk(name)
            .out();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    fn build_set_command(key: &str, value: impl Into<LexiType>) -> anyhow::Result<Vec<u8>> {
        match value.into() {
            LexiType::BulkString(bulk) => {
                let buf = Builder::new()
                    .add_arr(3)
                    .add_bulk("SET")
                    .add_bulk(key)
                    .add_bulk(&bulk)
                    .out();
                Ok(buf)
            }
            LexiType::Int(int) => {
                let buf = Builder::new()
                    .add_arr(3)
                    .add_bulk("SET")
                    .add_bulk(key)
                    .add_int(int)
                    .out();
                Ok(buf)
            }
            _ => return Err(anyhow::anyhow!("invalid value")),
        }
    }

    fn build_push_cmd(value: impl Into<LexiType>) -> anyhow::Result<Vec<u8>> {
        match value.into() {
            LexiType::BulkString(bulk) => {
                let buf = Builder::new()
                    .add_arr(2)
                    .add_bulk("PUSH")
                    .add_bulk(&bulk)
                    .out();
                Ok(buf)
            }
            LexiType::Int(int) => {
                let buf = Builder::new()
                    .add_arr(2)
                    .add_bulk("PUSH")
                    .add_int(int)
                    .out();
                Ok(buf)
            }
            _ => return Err(anyhow::anyhow!("invalid value")),
        }
    }

    fn build_cluster_set_command(
        cluster_name: &str,
        key: &str,
        value: impl Into<LexiType>,
    ) -> anyhow::Result<Vec<u8>> {
        match value.into() {
            LexiType::BulkString(bulk) => {
                let buf = Builder::new()
                    .add_arr(4)
                    .add_bulk("CLUSTER.SET")
                    .add_bulk(cluster_name)
                    .add_bulk(key)
                    .add_bulk(&bulk)
                    .out();
                Ok(buf)
            }
            LexiType::Int(int) => {
                let buf = Builder::new()
                    .add_arr(4)
                    .add_bulk("CLUSTER.SET")
                    .add_bulk(cluster_name)
                    .add_bulk(key)
                    .add_int(int)
                    .out();
                Ok(buf)
            }
            _ => return Err(anyhow::anyhow!("invalid value")),
        }
    }

    fn parse(input: Vec<u8>) -> anyhow::Result<LexiType> {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        p.parse()
    }

    async fn write(&mut self, bytes: Vec<u8>) -> anyhow::Result<usize> {
        let res: usize;
        match &self.stream {
            Some(stream) => loop {
                stream.writable().await?;
                match stream.try_write(&bytes) {
                    Ok(n) => {
                        res = n;
                        break;
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            },
            None => return Err(anyhow::anyhow!("no connection to database")),
        }

        Ok(res)
    }

    async fn read(&mut self, out: &mut Vec<u8>) -> anyhow::Result<usize> {
        let res: usize;
        match &self.stream {
            Some(stream) => loop {
                stream.readable().await?;
                match stream.try_read_buf(out) {
                    Ok(0) => {
                        res = 0;
                        break;
                    }
                    Ok(n) => {
                        res = n;
                        break;
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            },
            None => return Err(anyhow::anyhow!("no connection to the database")),
        }

        Ok(res)
    }
}
