use crate::packets::{
    open::OpenMessage, keepalive::KeepaliveMessage, update::UpdateMessage
};

/// BGPのRFC内8.1
/// (https://datatracker.ietf.org/doc/html/rfc4271#section-8.1)
/// 定義されているEvent)を表す列挙型です
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
    ManualStart,
    // 正常系しか実装しない本実装では別のEventとして扱う意味がないため、
    // TcpConnectionConfirmedはTcpCrAckedも兼ねている。
    TcpConnectionConfirmed,
    BgpOpen(OpenMessage),
    // MsgはMessageの省略形。BGPのRFC内での定義に従っている。
    KeepAliveMsg(KeepaliveMessage),
    // BGPのRFC内での定義に従っている。
    UpdateMsg(UpdateMessage),
    // StateがEstablishedに遷移したことを表す。
    // 存在するほうが実装が楽なので追加した本実装オリジナルのイベント
    Established,
    // LocRib / AdjRibOut / AdjRibInが変わった時のイベント。
    // 存在するほうが楽なので追加した。
    LocRibChanged,
    AdjRibOutChanged,
    AdjRibInChanged,
}