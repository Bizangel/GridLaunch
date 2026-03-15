pub const UI_TITLE_NAME: &str = "GridLaunch";
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";

use gridlaunch::event_handler::handle_event;
use gridlaunch::gamepad_monitor::gamepad_monitor_worker_main;
use gridlaunch::ipc_handler::ipc_handler;
use gridlaunch::spawner_thread::spawner_thread_main;
use gridlaunch::wry_ui_helper::WryWebViewAppBuilder;

fn main() -> Result<(), String> {
    let mut builder = WryWebViewAppBuilder::new()
        .with_title_name(UI_TITLE_NAME)
        .with_ipc_handler(ipc_handler)
        .with_worker_thread(gamepad_monitor_worker_main)
        .with_worker_thread(spawner_thread_main)
        .with_event_handler(handle_event)
        .with_initial_state(());
    #[cfg(debug_assertions)]
    {
        builder = builder.with_url(VITE_DEV_LOCALHOST_URL);
    }

    let app = builder.build();
    app.run()
}
