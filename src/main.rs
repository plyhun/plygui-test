use std::sync::{Arc, RwLock};

mod lib;

fn main() {
    lib::exec(Arc::new(RwLock::new(Vec::new())));
}
