use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt,
};
use notify::RecommendedWatcher as Watch;
use notify::{Event, RecursiveMode, Watcher};
use std::time::SystemTime;

pub type NotifyReceiver = Receiver<notify::Result<Event>>;

pub fn async_watcher(path: &str) -> notify::Result<(Watch, NotifyReceiver)> {
    let mut debounce = Box::new(SystemTime::now());
    let (mut tx, rx) = channel(10);
    let mut watcher = Watch::new(move |res: notify::Result<Event>| match res {
        Ok(ref e) if e.kind.is_modify() => {
            if debounce.elapsed().unwrap().as_secs() > 1 {
                debounce = Box::new(SystemTime::now());
                futures::executor::block_on(async {
                    tx.send(res).await;
                });
            }
        }
        _ => {}
    })?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    Ok((watcher, rx))
}
