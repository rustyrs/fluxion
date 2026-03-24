use crate::app::FluxionApp;

pub trait Plugins<Marker> {
    fn add_to_app(self, app: &mut FluxionApp);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub enum PluginsState {
    Adding,
    Ready,
    Finished,
    Cleaned,
}
