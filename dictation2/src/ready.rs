pub mod event_loop;
pub mod window;

trait Ready<I, O> {
    fn ready(&mut self, input: I) -> O;
}

struct Window;

impl Ready<Surface, ()> for Window {
    fn ready(&mut self, input: Surface) -> () {
        println!("EventLoop is ready");
    }
}
