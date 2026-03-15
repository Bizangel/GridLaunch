use super::WryWebViewApp;
use tao::{event::WindowEvent, event_loop::ControlFlow, keyboard::Key};

impl<T: Send + 'static, S: 'static, M: Send + 'static> WryWebViewApp<T, S, M> {
    pub fn handle_window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let key = &event.logical_key;
                match key {
                    Key::Character("i") => {
                        #[cfg(debug_assertions)]
                        {
                            self.webview.open_devtools();
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::CloseRequested => {
                self.workers_stop_signal.request_stop();

                for handle in self.worker_threads.drain(..) {
                    handle.join().unwrap();
                }

                *control_flow = ControlFlow::ExitWithCode(1);
            }
            _ => {}
        }
    }
}
