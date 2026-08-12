use super::LaserSink;

pub struct LaserExtractor {
    lasersink: LaserSink,
}

impl Default for LaserExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl LaserExtractor {
    pub fn new() -> Self {
        let lasersink = LaserSink::init();
        LaserExtractor { lasersink }
    }

    pub fn register_extractor(&self) {
        self.lasersink.register_proc_macro_hook();
    }

    pub fn un_register_extractor(&self) {
        self.lasersink.unregister_proc_macro_hook();
    }

    pub fn is_extractor_active(&self) -> bool {
        self.lasersink.is_active()
    }

    pub fn toggle_extractor(&self) {
        self.lasersink.toggle_active();
    }
}
