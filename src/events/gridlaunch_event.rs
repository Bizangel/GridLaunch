use crate::events::{fromwebview_event::FromWebViewEvent, towebview_event::ToWebViewEvent};

// Events generated to be handled for the main loop
#[derive(Debug, Clone)]
pub enum GridLaunchEvent {
    FromWebViewEvent(FromWebViewEvent),
    ForwardToWebViewEvent(ToWebViewEvent),
}
