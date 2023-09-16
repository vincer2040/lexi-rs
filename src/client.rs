use crate::lexi_type::LexiType;
use crate::builder::Builder;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct Client {
    addr: std::net::SocketAddr,
    stream: Option<tokio::net::TcpStream>,
}

impl Client {
    pub fn new(address: &str) -> anyhow::Result<Self> {
        let addr: std::net::SocketAddr = address.parse()?;
        Ok(Client { addr, stream: None, })
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        let socket = tokio::net::TcpSocket::new_v4()?;
        let stream = socket.connect(self.addr).await?;
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn set(&mut self, key: &str, value: impl Into<LexiType>) -> anyhow::Result<LexiType> {
        match value.into() {
            LexiType::BulkString(bulk) => {
                let buf = Builder::new()
                    .add_arr(3)
                    .add_bulk("SET")
                    .add_bulk(key)
                    .add_bulk(&bulk)
                    .out();
                let _ = self.write(buf).await?;
                let mut read_buf = Vec::with_capacity(4096);
                let _ = self.read(&mut read_buf).await?;
                let l = Lexer::new(read_buf);
                let mut p = Parser::new(l);
                let ret = p.parse()?;
                Ok(ret)
            }
            LexiType::Int(int) => {
                let buf = Builder::new()
                    .add_arr(3)
                    .add_bulk("SET")
                    .add_bulk(key)
                    .add_int(int)
                    .out();
                let _ = self.write(buf).await?;
                let mut read_buf = Vec::with_capacity(4096);
                let _ = self.read(&mut read_buf).await?;
                let l = Lexer::new(read_buf);
                let mut p = Parser::new(l);
                let ret = p.parse()?;
                Ok(ret)
            }
            _ => return Err(anyhow::anyhow!("invalid value")),
        }
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
        let l = Lexer::new(read_buf);
        let mut p = Parser::new(l);
        let ret = p.parse()?;
        Ok(ret)
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
        let l = Lexer::new(read_buf);
        let mut p = Parser::new(l);
        let ret = p.parse()?;
        Ok(ret)
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
        let l = Lexer::new(read_buf);
        let mut p = Parser::new(l);
        let ret = p.parse()?;
        Ok(ret)
    }

    pub async fn cluster_set(&mut self, name: &str, key: &str, value: impl Into<LexiType>) -> anyhow::Result<LexiType> {
        match value.into() {
            LexiType::BulkString(v) => {
                let buf = Builder::new()
                    .add_arr(4)
                    .add_bulk("CLUSTER.SET")
                    .add_bulk(name)
                    .add_bulk(key)
                    .add_bulk(&v)
                    .out();
                let mut read_buf = Vec::with_capacity(4096);
                let _ = self.write(buf).await?;
                let _ = self.read(&mut read_buf).await?;
                let l = Lexer::new(read_buf);
                let mut p = Parser::new(l);
                let ret = p.parse()?;
                Ok(ret)
            }
            LexiType::Int(v) => {
                let buf = Builder::new()
                    .add_arr(3)
                    .add_bulk("CLUSTER.SET")
                    .add_bulk(name)
                    .add_bulk(key)
                    .add_int(v)
                    .out();
                let mut read_buf = Vec::with_capacity(4096);
                let _ = self.write(buf).await?;
                let _ = self.read(&mut read_buf).await?;
                let l = Lexer::new(read_buf);
                let mut p = Parser::new(l);
                let ret = p.parse()?;
                Ok(ret)
            }
            _ => Err(anyhow::anyhow!("invalid value"))
        }
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
        let l = Lexer::new(read_buf);
        let mut p = Parser::new(l);
        let ret = p.parse()?;
        Ok(ret)
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
        let l = Lexer::new(read_buf);
        let mut p = Parser::new(l);
        let ret = p.parse()?;
        Ok(ret)
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
        let l = Lexer::new(read_buf);
        let mut p = Parser::new(l);
        let ret = p.parse()?;
        Ok(ret)
    }

    async fn write(&mut self, bytes: Vec<u8>) -> anyhow::Result<usize> {
        let res: usize;
        match &self.stream {
            Some(stream) => {
                loop {
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
                }
            }
            None => return Err(anyhow::anyhow!("no connection to database")),
        }

        Ok(res)
    }

    async fn read(&mut self, out: &mut Vec<u8>) -> anyhow::Result<usize> {
        let res: usize;
        match &self.stream {
            Some(stream) => {
                loop {
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
                }
            }
            None => return Err(anyhow::anyhow!("no connection to the database")),
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn test_set_get_delvalue() -> anyhow::Result<()> {
        let mut client = Client::new("127.0.0.1:6969")?;
        client.connect().await?;
        let mut val = client.set("vince", "is cool").await?;
        let mut exp = LexiType::Simple("OK".to_string());
        assert_eq!(val, exp);
        val = client.get("vince").await?;
        exp = "is cool".into();
        assert_eq!(val, exp);
        exp = LexiType::Simple("OK".to_string());
        val = client.del("vince").await?;
        assert_eq!(exp, val);
        Ok(())
    }

    #[tokio::test]
    async fn test_clusters() -> anyhow::Result<()> {
        let mut client = Client::new("127.0.0.1:6969")?;
        client.connect().await?;
        let mut val = client.cluster_new("test").await?;
        let mut exp = LexiType::Simple("OK".to_string());
        assert_eq!(val, exp);
        val = client.cluster_set("test", "vince", "is cool").await?;
        assert_eq!(val, exp);
        exp = "is cool".into();
        val = client.cluster_get("test", "vince").await?;
        assert_eq!(val, exp);
        val = client.cluster_del("test", "vince").await?;
        exp = LexiType::Simple("OK".to_string());
        assert_eq!(val, exp);
        val = client.cluster_drop("test").await?;
        assert_eq!(val, exp);
        Ok(())
    }
}
