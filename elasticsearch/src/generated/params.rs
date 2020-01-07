// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Bytes {
    #[serde(rename = "b")]
    B,
    #[serde(rename = "k")]
    K,
    #[serde(rename = "kb")]
    Kb,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "mb")]
    Mb,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "gb")]
    Gb,
    #[serde(rename = "t")]
    T,
    #[serde(rename = "tb")]
    Tb,
    #[serde(rename = "p")]
    P,
    #[serde(rename = "pb")]
    Pb,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Conflicts {
    #[serde(rename = "abort")]
    Abort,
    #[serde(rename = "proceed")]
    Proceed,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum DefaultOperator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum ExpandWildcards {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "all")]
    All,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum GroupBy {
    #[serde(rename = "nodes")]
    Nodes,
    #[serde(rename = "parents")]
    Parents,
    #[serde(rename = "none")]
    None,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Health {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Level {
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "indices")]
    Indices,
    #[serde(rename = "shards")]
    Shards,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum OpType {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "create")]
    Create,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Refresh {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "wait_for")]
    WaitFor,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum SearchType {
    #[serde(rename = "query_then_fetch")]
    QueryThenFetch,
    #[serde(rename = "dfs_query_then_fetch")]
    DfsQueryThenFetch,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Size {
    #[serde(rename = "k")]
    K,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "t")]
    T,
    #[serde(rename = "p")]
    P,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum SuggestMode {
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "popular")]
    Popular,
    #[serde(rename = "always")]
    Always,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Time {
    #[serde(rename = "d")]
    D,
    #[serde(rename = "h")]
    H,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "s")]
    S,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "micros")]
    Micros,
    #[serde(rename = "nanos")]
    Nanos,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Type {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "block")]
    Block,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum VersionType {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "external_gte")]
    ExternalGte,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum WaitForEvents {
    #[serde(rename = "immediate")]
    Immediate,
    #[serde(rename = "urgent")]
    Urgent,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "languid")]
    Languid,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum WaitForStatus {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
}