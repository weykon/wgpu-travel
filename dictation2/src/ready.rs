use crate::process_atom::Process;


pub trait Ready {
    fn ready(&self);
}

impl Process for dyn Ready {
    fn process(&self) {
        // 实现 config 的处理逻辑
    }
}
