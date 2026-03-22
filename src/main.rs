pub const UI_TITLE_NAME: &str = "GridLaunch";
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";
const MINIFIED_HTML_STR: &str = include_str!("../src-ui/dist/index.html");

use gridlaunch::event_handler::{GridLaunchState, handle_event};
use gridlaunch::game_handler::get_valid_game_handlers;
use gridlaunch::gamepad_monitor::gamepad_monitor_worker_main;
use gridlaunch::ipc_handler::ipc_handler;
use gridlaunch::spawner_thread::spawner_thread_main;
use gridlaunch::user_profile::get_all_profiles;
use gridlaunch::utils::ensure_handler_dir_exists;
use gridlaunch::wry_ui_helper::WryWebViewAppBuilder;

fn main() -> Result<(), String> {
    ensure_handler_dir_exists().map_err(|err| err.to_string())?;
    let handlers = get_valid_game_handlers();
    let profiles = get_all_profiles();
    let mut builder = WryWebViewAppBuilder::new()
        .with_title_name(UI_TITLE_NAME)
        .with_ipc_handler(ipc_handler)
        .with_worker_thread(gamepad_monitor_worker_main)
        .with_worker_thread(spawner_thread_main)
        .with_event_handler(handle_event)
        .with_initial_state(GridLaunchState {
            game_handlers: handlers,
            profiles: profiles,
        });
    #[cfg(debug_assertions)]
    {
        builder = builder.with_url(VITE_DEV_LOCALHOST_URL);
    }
    #[cfg(not(debug_assertions))]
    {
        builder = builder.with_html(MINIFIED_HTML_STR);
    }

    let app = builder.build();
    app.run()
}
