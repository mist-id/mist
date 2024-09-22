use derive_more::Display;

#[derive(Display)]
pub(crate) enum Event {
    #[display("mist-redirect-after-auth")]
    Redirect,
}

pub(crate) fn get_event_key(event: &Event, id: &str) -> String {
    format!("{}.{}", event, id)
}
