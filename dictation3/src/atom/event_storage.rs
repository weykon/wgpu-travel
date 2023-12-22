use winit::event_loop::EventLoop;

pub struct EventStorage {
    pub event_loop: EventLoop<()>,
}

impl EventStorage {
    pub fn run(
        &mut self,
        mut match_event: impl FnMut(winit::event::Event<()>, &(), &mut winit::event_loop::ControlFlow)
            + 'static,
    ) {
        let event_loop = std::mem::take(&mut self.event_loop);
        event_loop.run(move |event, _, control_flow| {
            match_event(event, &(), control_flow);
        });
    }
}
