use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt,
};
use notify::{Event, RecommendedWatcher, Watcher};

pub type NotifyReceiver = Receiver<notify::Result<Event>>;

pub fn async_watcher() -> notify::Result<(RecommendedWatcher, NotifyReceiver)> {
    let (mut tx, rx) = channel(1);
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await;
        })
    })?;

    Ok((watcher, rx))
}
