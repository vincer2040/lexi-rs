use crate::builder::Builder;
use crate::cmd::{
    ClusterDelCmd, ClusterDropCmd, ClusterEntriesCmd, ClusterGetCmd, ClusterKeysCmd, ClusterNewCmd,
    ClusterPopCmd, ClusterPushCmd, ClusterSetCmd, ClusterValuesCmd, Cmd, DelCmd, EnqueCmd, GetCmd,
    PushCmd, SetCmd,
};
use crate::lexer::Lexer;
use crate::lexi_type::LexiType;
use crate::parser::Parser;

pub struct Multi<'a> {
    cmds: Vec<Cmd<'a>>,
    stream: &'a tokio::net::TcpStream,
}

impl<'a> Multi<'a> {
    pub fn new(stream: &'a tokio::net::TcpStream) -> Self {
        Self {
            cmds: Vec::new(),
            stream,
        }
    }

    pub fn add_ping(mut self) -> Self {
        let cmd = Cmd::Ping;
        self.cmds.push(cmd);
        self
    }

    pub fn add_set(mut self, key: &'a str, value: impl Into<LexiType>) -> Self {
        let cmd = Cmd::Set(SetCmd {
            key,
            value: value.into(),
        });
        self.cmds.push(cmd);
        self
    }

    pub fn add_get(mut self, key: &'a str) -> Self {
        let cmd = Cmd::Get(GetCmd { key });
        self.cmds.push(cmd);
        self
    }

    pub fn add_del(mut self, key: &'a str) -> Self {
        let cmd = Cmd::Del(DelCmd { key });
        self.cmds.push(cmd);
        self
    }

    pub fn add_push(mut self, value: impl Into<LexiType>) -> Self {
        let cmd = Cmd::Push(PushCmd {
            value: value.into(),
        });
        self.cmds.push(cmd);
        self
    }

    pub fn add_pop(mut self) -> Self {
        let cmd = Cmd::Pop;
        self.cmds.push(cmd);
        self
    }

    pub fn add_enque(mut self, value: impl Into<LexiType>) -> Self {
        let cmd = Cmd::Enque(EnqueCmd {
            value: value.into(),
        });
        self.cmds.push(cmd);
        self
    }

    pub fn add_deque(mut self) -> Self {
        let cmd = Cmd::Deque;
        self.cmds.push(cmd);
        self
    }

    pub fn add_keys(mut self) -> Self {
        let cmd = Cmd::Keys;
        self.cmds.push(cmd);
        self
    }

    pub fn add_values(mut self) -> Self {
        let cmd = Cmd::Values;
        self.cmds.push(cmd);
        self
    }

    pub fn add_entries(mut self) -> Self {
        let cmd = Cmd::Entries;
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_new(mut self, name: &'a str) -> Self {
        let cmd = Cmd::ClusterNew(ClusterNewCmd { name });
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_set(
        mut self,
        name: &'a str,
        key: &'a str,
        value: impl Into<LexiType>,
    ) -> Self {
        let cmd = Cmd::ClusterSet(ClusterSetCmd {
            name,
            key,
            value: value.into(),
        });
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_get(mut self, name: &'a str, key: &'a str) -> Self {
        let cmd = Cmd::ClusterGet(ClusterGetCmd { name, key });
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_del(mut self, name: &'a str, key: &'a str) -> Self {
        let cmd = Cmd::ClusterDel(ClusterDelCmd { name, key });
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_push(mut self, name: &'a str, value: impl Into<LexiType>) -> Self {
        let cmd = Cmd::ClusterPush(ClusterPushCmd {
            name,
            value: value.into(),
        });
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_pop(mut self, name: &'a str) -> Self {
        let cmd = Cmd::ClusterPop(ClusterPopCmd { name });
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_keys(mut self, name: &'a str) -> Self {
        let cmd = Cmd::ClusterKeys(ClusterKeysCmd { name });
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_values(mut self, name: &'a str) -> Self {
        let cmd = Cmd::ClusterValues(ClusterValuesCmd { name });
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_entries(mut self, name: &'a str) -> Self {
        let cmd = Cmd::ClusterEntries(ClusterEntriesCmd { name });
        self.cmds.push(cmd);
        self
    }

    pub fn add_cluster_drop(mut self, name: &'a str) -> Self {
        let cmd = Cmd::ClusterDrop(ClusterDropCmd { name });
        self.cmds.push(cmd);
        self
    }

    pub async fn run(mut self) -> anyhow::Result<LexiType> {
        let buf = self.build_cmd();
        let mut read_buf = Vec::with_capacity(4096);
        let _ = self.write(buf).await?;
        let _ = self.read(&mut read_buf).await?;
        Self::parse(read_buf)
    }

    fn build_cmd(&self) -> Vec<u8> {
        let mut b = Builder::new();
        for cmd in self.cmds.iter() {
            match cmd {
                Cmd::Ping => b = b.add_ping(),
                Cmd::Pop => b = b.add_bulk("POP"),
                Cmd::Deque => b = b.add_bulk("DEQUE"),
                Cmd::Keys => b = b.add_bulk("KEYS"),
                Cmd::Values => b = b.add_bulk("VALUES"),
                Cmd::Entries => b = b.add_bulk("ENTRIES"),
                Cmd::Set(set_cmd) => {
                    b = b.add_arr(3).add_bulk("SET").add_bulk(set_cmd.key);
                    b = match &set_cmd.value {
                        LexiType::BulkString(bulk) => b.add_bulk(bulk),
                        LexiType::Int(i) => b.add_int(*i),
                        _ => unreachable!(),
                    }
                }
                Cmd::Get(get_cmd) => {
                    b = b.add_arr(2).add_bulk("GET").add_bulk(get_cmd.key);
                }
                Cmd::Del(del_cmd) => {
                    b = b.add_arr(2).add_bulk("DEL").add_bulk(del_cmd.key);
                }
                Cmd::Push(push_cmd) => {
                    b = b.add_arr(2).add_bulk("PUSH");
                    b = match &push_cmd.value {
                        LexiType::BulkString(bulk) => b.add_bulk(bulk),
                        LexiType::Int(i) => b.add_int(*i),
                        _ => unreachable!(),
                    }
                }
                Cmd::Enque(enque_cmd) => {
                    b = b.add_arr(2).add_bulk("ENQUE");
                    b = match &enque_cmd.value {
                        LexiType::BulkString(bulk) => b.add_bulk(bulk),
                        LexiType::Int(i) => b.add_int(*i),
                        _ => unreachable!(),
                    }
                }
                Cmd::ClusterNew(cluster_new_cmd) => {
                    b = b
                        .add_arr(2)
                        .add_bulk("CLUSTER.NEW")
                        .add_bulk(cluster_new_cmd.name);
                }
                Cmd::ClusterSet(cluster_set_cmd) => {
                    b = b
                        .add_arr(4)
                        .add_bulk("CLUSTER.SET")
                        .add_bulk(cluster_set_cmd.name)
                        .add_bulk(cluster_set_cmd.key);
                    b = match &cluster_set_cmd.value {
                        LexiType::BulkString(bulk) => b.add_bulk(bulk),
                        LexiType::Int(i) => b.add_int(*i),
                        _ => unreachable!(),
                    }
                }
                Cmd::ClusterGet(cluster_get_cmd) => {
                    b = b
                        .add_arr(3)
                        .add_bulk("CLUSTER.GET")
                        .add_bulk(cluster_get_cmd.name)
                        .add_bulk(cluster_get_cmd.key);
                }
                Cmd::ClusterDel(cluster_del_cmd) => {
                    b = b
                        .add_arr(3)
                        .add_bulk("CLUSTER.DEL")
                        .add_bulk(cluster_del_cmd.name)
                        .add_bulk(cluster_del_cmd.key);
                }
                Cmd::ClusterPush(cluster_push_cmd) => {
                    b = b
                        .add_arr(3)
                        .add_bulk("CLUSTER.PUSh")
                        .add_bulk(cluster_push_cmd.name);
                    b = match &cluster_push_cmd.value {
                        LexiType::BulkString(bulk) => b.add_bulk(bulk),
                        LexiType::Int(i) => b.add_int(*i),
                        _ => unreachable!(),
                    }
                }
                Cmd::ClusterPop(cluster_pop_cmd) => {
                    b = b
                        .add_arr(2)
                        .add_bulk("CLUSTER.POP")
                        .add_bulk(cluster_pop_cmd.name);
                }
                Cmd::ClusterKeys(cluster_keys_cmd) => {
                    b = b
                        .add_arr(2)
                        .add_bulk("CLUSTER.KEYS")
                        .add_bulk(cluster_keys_cmd.name);
                }
                Cmd::ClusterValues(cluster_values_cmd) => {
                    b = b
                        .add_arr(2)
                        .add_bulk("CLUSTER.VALUES")
                        .add_bulk(cluster_values_cmd.name);
                }
                Cmd::ClusterEntries(cluster_entries_cmd) => {
                    b = b
                        .add_arr(2)
                        .add_bulk("CLUSTER.ENTRIES")
                        .add_bulk(cluster_entries_cmd.name);
                }
                Cmd::ClusterDrop(cluster_drop_cmd) => {
                    b = b
                        .add_arr(2)
                        .add_bulk("CLUSTER.DROP")
                        .add_bulk(cluster_drop_cmd.name);
                }
            };
        }

        b.out()
    }

    fn parse(input: Vec<u8>) -> anyhow::Result<LexiType> {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        p.parse()
    }

    async fn write(&mut self, bytes: Vec<u8>) -> anyhow::Result<usize> {
        let res: usize;
        loop {
            self.stream.writable().await?;
            match self.stream.try_write(&bytes) {
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

        Ok(res)
    }

    async fn read(&mut self, out: &mut Vec<u8>) -> anyhow::Result<usize> {
        let res: usize;
        loop {
            self.stream.readable().await?;
            match self.stream.try_read_buf(out) {
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

        Ok(res)
    }
}
