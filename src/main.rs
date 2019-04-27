use plygui::Window;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use std::thread;
use std::time::Duration;

mod lib;

fn main() {
    let feeders: Arc<RwLock<Vec<plygui::callbacks::AsyncFeeder<plygui::callbacks::OnFrame>>>> = Arc::new(RwLock::new(Vec::new()));
    let feeders2 = feeders.clone();
    let running = Arc::new(AtomicBool::new(true));
    let running2 = running.clone();

    thread::spawn(move || {
        while running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(5000));

            if feeders2.write().unwrap().len() > 0 {
                let _ = feeders2.write().unwrap().remove(0).feed(
                    (move |w: &mut dyn (Window)| {
                        w.close(true);
                        false
                    })
                    .into(),
                );
            }
        }
    });

    lib::exec(feeders);
    running2.store(false, Ordering::SeqCst);
}
