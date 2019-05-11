use std::sync::{Arc, RwLock};

mod lib;

fn main() {
    lib::exec(Arc::new(RwLock::new(Vec::new())));
}

#[cfg(test)]
mod tests {
    use super::lib;
    use plygui::imp::{Tray, Window};
    use plygui::{Application, Closeable};

    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    };
    use std::thread;
    use std::time::Duration;

    #[test]
    fn close_after_5_secs() {
        let feeders: Arc<RwLock<Vec<plygui::callbacks::AsyncFeeder<plygui::callbacks::OnFrame>>>> = Arc::new(RwLock::new(Vec::new()));
        let feeders2 = feeders.clone();
        let running = Arc::new(AtomicBool::new(true));
        let running2 = running.clone();

        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(5000));

                if feeders2.write().unwrap().len() > 0 {
                    let _ = feeders2.write().unwrap().remove(0).feed(
                        (move |w: &mut dyn Application| {
                            for m in w.members_mut() {
                                if let Some(w) = m.as_any_mut().downcast_mut::<Window>() {
                                    w.close(true);
                                } else if let Some(t) = m.as_any_mut().downcast_mut::<Tray>() {
                                    t.close(true);
                                } else {
                                    println!("Unknown member type");
                                }
                            }
                            false
                        })
                        .into(),
                    );
                }

                println!("Still alive");
            }
        });

        lib::exec(feeders);
        running2.store(false, Ordering::SeqCst);
    }
}
