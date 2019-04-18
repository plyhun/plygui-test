use plygui::Window;

use std::sync::{Arc, RwLock, atomic::{Ordering, AtomicBool}};
use std::thread;
use std::time::Duration;

mod lib;

fn main() {
    let feeders = Arc::new(RwLock::new(Vec::new()));
    let feeders2 = feeders.clone();
    let running = Arc::new(AtomicBool::new(true));
    let running2 = running.clone();
    
    thread::spawn(move ||{
        lib::exec(feeders2);
        running2.store(false, Ordering::SeqCst);
    });
    
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(5000));
        
        if feeders.write().unwrap().len() > 0 {
            let _ = feeders.write().unwrap().remove(0).feed((move |w: &mut dyn (Window)| { 
                        w.close(true); 
                        false 
                }).into());
        }
    }
}