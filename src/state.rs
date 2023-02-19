#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum  State {
    Idle,
    Connect,
    OpenSent,
    OpenConfirm,
    Established,
}