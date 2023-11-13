use crate::lexi_type::LexiType;

pub enum Cmd<'a> {
    Ping,
    Pop,
    Deque,
    Keys,
    Values,
    Entries,
    StatsCycles,
    Set(SetCmd<'a>),
    Get(GetCmd<'a>),
    Del(DelCmd<'a>),
    Push(PushCmd),
    Enque(EnqueCmd),
    ClusterNew(ClusterNewCmd<'a>),
    ClusterSet(ClusterSetCmd<'a>),
    ClusterGet(ClusterGetCmd<'a>),
    ClusterDel(ClusterDelCmd<'a>),
    ClusterPush(ClusterPushCmd<'a>),
    ClusterPop(ClusterPopCmd<'a>),
    ClusterKeys(ClusterKeysCmd<'a>),
    ClusterValues(ClusterValuesCmd<'a>),
    ClusterEntries(ClusterEntriesCmd<'a>),
    ClusterDrop(ClusterDropCmd<'a>),
}

pub struct SetCmd<'a>{
    pub key: &'a str,
    pub value: LexiType,
}

pub struct GetCmd<'a> {
    pub key: &'a str,
}

pub type DelCmd<'a> = GetCmd<'a>;

pub struct PushCmd {
    pub value: LexiType,
}

pub type EnqueCmd = PushCmd;

pub struct ClusterNewCmd<'a> {
    pub name: &'a str,
}

pub struct ClusterSetCmd<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub value: LexiType,
}

pub struct ClusterGetCmd<'a> {
    pub name: &'a str,
    pub key: &'a str,
}

pub type ClusterDelCmd<'a> = ClusterGetCmd<'a>;

pub struct ClusterPushCmd<'a> {
    pub name: &'a str,
    pub value: LexiType,
}

pub type ClusterPopCmd<'a> = ClusterNewCmd<'a>;

pub type ClusterKeysCmd<'a> = ClusterNewCmd<'a>;

pub type ClusterValuesCmd<'a> = ClusterNewCmd<'a>;

pub type ClusterEntriesCmd<'a> = ClusterNewCmd<'a>;

pub type ClusterDropCmd<'a> = ClusterNewCmd<'a>;

