#[derive(Debug, Clone, PartialEq)]
pub enum ScriptType {
    P2PK,
    P2PKH,
    P2SH,
    P2MS,
    Return,
    Unknown,
}