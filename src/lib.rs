pub use geng_core::*;
pub use geng_debug_overlay::*;
pub use geng_net as net;
pub use geng_ui as ui;

use prelude::*;

pub fn run(geng: Rc<Geng>, state: impl State) {
    let mut state_manager = StateManager::new();
    state_manager.push(Box::new(state));
    let state = DebugOverlay::new(&geng, state_manager);
    geng_core::run(geng, state);
}
