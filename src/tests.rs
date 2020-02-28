mod lib;

use plygui::{Application, FindBy};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use std::thread;
use std::time::Duration;

pub fn main() {
    close_after_5_secs();
}

fn close_after_5_secs() {
    let feeders: Arc<RwLock<Vec<plygui::callbacks::AsyncFeeder<plygui::callbacks::OnFrame>>>> = Arc::new(RwLock::new(Vec::new()));
    let feeders2 = feeders.clone();
    let running = Arc::new(AtomicBool::new(true));
    let running2 = running.clone();

    thread::spawn(move || {
        while running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(5000));
            
            if feeders2.write().unwrap().len() > 0 {
                let _ = feeders2.write().unwrap()[0].feed(
                    (move |w: &mut dyn Application| {
                        let id = w.roots_mut().next().map(|r| r.id());
                        if let Some(id) = id {
                            w.close_root(FindBy::Id(id), true);
                        }
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
