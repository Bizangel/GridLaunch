use crate::{
    events::{gridlaunch_event::GridLaunchEvent, worker_event::GridLaunchWorkerEvent},
    launch::spawn_games_and_wait,
    wry_ui_helper::stop_signal::StopSignal,
};

use crate::events::towebview_event::ToWebViewEvent;

use std::{sync::mpsc::Receiver, thread, time::Duration};
use tao::event_loop::EventLoopProxy;

fn _spawner_thread_main(
    stop_signal: StopSignal,
    rx: Receiver<GridLaunchWorkerEvent>,
    ui_proxy: EventLoopProxy<GridLaunchEvent>,
) -> Result<(), String> {
    while !stop_signal.requested() {
        match rx.try_recv().ok() {
            Some(GridLaunchWorkerEvent::SpawnInstances { request, handler }) => {
                // block spawn games and wait
                spawn_games_and_wait(request, handler);
                // notify UI that we have returned
                let _ = ui_proxy.send_event(GridLaunchEvent::ForwardToWebViewEvent(
                    ToWebViewEvent::LaunchReturned,
                ));
            }
            _ => {}
        }

        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

pub fn spawner_thread_main(
    stop_signal: StopSignal,
    rx: Receiver<GridLaunchWorkerEvent>,
    ui_proxy: EventLoopProxy<GridLaunchEvent>,
) {
    let _ = _spawner_thread_main(stop_signal, rx, ui_proxy);
}
