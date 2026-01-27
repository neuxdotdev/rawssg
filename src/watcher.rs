use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use crate::error::Result;
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<notify::Event>>,
    debounce_time: Duration,
}
impl FileWatcher {
    pub fn new(debounce_ms: u64) -> Result<Self> {
        let (tx, rx) = channel();
        let watcher = RecommendedWatcher::new(
            tx,
            notify::Config::default()
            .with_poll_interval(Duration::from_millis(300))
        )?;
        Ok(Self {
                watcher,
                rx,
                debounce_time: Duration::from_millis(debounce_ms),
            })
        }
        pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
            self.watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
            Ok(())
        }
        pub fn wait_for_changes(&self, callback: impl Fn()) -> Result<()> {
            use std::time::Instant;
            let mut last_event = Instant::now();
            loop {
                if let Ok(event) = self.rx.recv() {
                    match event {
                        Ok(event) => {
                            if Self::should_trigger(&event) {
                                let now = Instant::now();
                                if now.duration_since(last_event) > self.debounce_time {
                                    last_event = now;
                                    callback();
                                }
                            }
                        }
                        Err(e) => eprintln!("Watch error: {:?}", e),
                    }
                }
            }
        }
        fn should_trigger(event: &notify::Event) -> bool {
            matches!(
                event.kind,
                notify::EventKind::Modify(_) |
                notify::EventKind::Create(_) |
                notify::EventKind::Remove(_)
            )
        }
    }