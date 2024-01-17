use tokio::net::{TcpSocket, TcpStream};

use crate::{builder::Builder, lexi_data::LexiData, parser::Parser};

pub struct Client {
    addr: std::net::SocketAddr,
    stream: Option<TcpStream>,
}

impl Client {
    pub fn new(address: &str) -> anyhow::Result<Self> {
        let addr = address.parse()?;
        Ok(Self { addr, stream: None })
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        let socket = TcpSocket::new_v4()?;
        let stream = socket.connect(self.addr).await?;
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn ping(&mut self) -> anyhow::Result<LexiData> {
        let buf = Builder::new().add_ping().out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn auth(&mut self, username: &str, password: &str) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("AUTH")
            .add_bulk(username)
            .add_bulk(password)
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn keys(&mut self) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_bulk("KEYS")
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn set(
        &mut self,
        key: impl Into<LexiData>,
        value: impl Into<LexiData>,
    ) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_arr(3)
            .add_bulk("SET")
            .add_impl_lexi_data(key)
            .add_impl_lexi_data(value)
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn get(&mut self, key: impl Into<LexiData>) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("GET")
            .add_impl_lexi_data(key)
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn del(&mut self, key: impl Into<LexiData>) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("DEL")
            .add_impl_lexi_data(key)
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn push(&mut self, value: impl Into<LexiData>) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("PUSH")
            .add_impl_lexi_data(value)
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn pop(&mut self) -> anyhow::Result<LexiData> {
        let buf = Builder::new().add_bulk("POP").out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn enque(&mut self, value: impl Into<LexiData>) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("ENQUE")
            .add_impl_lexi_data(value)
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn deque(&mut self) -> anyhow::Result<LexiData> {
        let buf = Builder::new().add_bulk("DEQUE").out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn zset(&mut self, value: impl Into<LexiData>) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("ZSET")
            .add_impl_lexi_data(value)
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn zhas(&mut self, value: impl Into<LexiData>) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("ZHAS")
            .add_impl_lexi_data(value)
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    pub async fn zdel(&mut self, value: impl Into<LexiData>) -> anyhow::Result<LexiData> {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("ZDEL")
            .add_impl_lexi_data(value)
            .out();
        let bytes = self.send_and_read(&buf).await?;
        Self::parse(&bytes)
    }

    fn parse(buf: &Vec<u8>) -> anyhow::Result<LexiData> {
        let mut p = Parser::new(buf);
        p.parse()
    }

    async fn send_and_read(&mut self, buf: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        self.send(&buf).await?;
        self.read().await
    }

    async fn send(&mut self, buf: &Vec<u8>) -> anyhow::Result<()> {
        match &self.stream {
            Some(stream) => loop {
                stream.writable().await?;
                match stream.try_write(buf) {
                    Ok(_) => break,
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                    Err(e) => return Err(e.into()),
                }
            },
            None => return Err(anyhow::anyhow!("not connnected")),
        }
        Ok(())
    }

    async fn read(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(4096);
        match &self.stream {
            Some(stream) => loop {
                stream.readable().await?;
                match stream.try_read_buf(&mut buf) {
                    Ok(_) => break,
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                    Err(e) => return Err(e.into()),
                }
            },
            None => unreachable!(),
        }
        Ok(buf)
    }
}
