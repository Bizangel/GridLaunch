use super::WryWebViewApp;
use tao::{event::Event, event_loop::ControlFlow};

impl<T: Send + 'static> WryWebViewApp<T> {
    pub fn run(mut self) -> ! {
        let event_loop = self.event_loop.take().expect("Event loop already consumed");
        event_loop.run(move |event, _, control_flow| {
            self.handle_main_loop_event(event, control_flow);
        });
    }

    fn handle_main_loop_event(&mut self, event: Event<T>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, .. } => self.handle_window_event(&event, control_flow),
            Event::UserEvent(event) => (self.event_handler)(event, self),
            _ => {}
        }
    }
}
