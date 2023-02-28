use std::str::FromStr;
use std::sync::Arc;
use std::env;

use bgprust::peer::Peer;
use bgprust::config::Config;
use bgprust::routing::LocRib;

use tokio::time::{sleep, Duration};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = 
        env::args()
        .skip(1)
        .fold("".to_owned(), |mut acc, s| {
            acc += &(s.to_owned() + " ");
            acc
        });
    let config = config.trim_end();
    let configs = vec![Config::from_str(&config).unwrap()];

    let loc_rib = Arc::new(Mutex::new(
        LocRib::new(&configs[0])
            .await
            .expect("LocRibの生成に失敗しました。"),
    ));
    let mut peers: Vec<Peer> = configs
        .into_iter()
        .map(|c| Peer::new(c, Arc::clone(&loc_rib)))
        .collect();
    for peer in &mut peers {
        peer.start();
    }

    let mut handles = vec![];
    for mut peer in peers {
        let handle = tokio::spawn(async move {
            loop {
                peer.next().await;
                sleep(Duration::from_secs_f32(0.1)).await;
            }
        });
        handles.push(handle);
    }

    // main関数が勝手に終了しないようにしている。
    for handle in handles {
        handle.await;
    }

}
