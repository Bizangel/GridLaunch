use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
pub struct StopSignal {
    flag: Arc<AtomicBool>,
}

impl StopSignal {
    pub fn new() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn request_stop(&self) {
        self.flag.store(true, Ordering::Relaxed);
    }

    pub fn requested(&self) -> bool {
        self.flag.load(Ordering::Relaxed)
    }
}
