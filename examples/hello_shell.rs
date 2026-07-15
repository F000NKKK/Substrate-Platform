//! Smoke test: an empty dockable window with a single panel, proving the
//! shell runs standalone with no product-specific code at all.

use substrate_platform::{DockState, Panel, Shell};

struct HelloPanel;

impl Panel for HelloPanel {
    fn title(&self) -> String {
        "Hello".into()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Substrate Platform is running.");
    }
}

fn main() -> eframe::Result<()> {
    let dock_state = DockState::new(vec![Box::new(HelloPanel) as Box<dyn Panel>]);
    substrate_platform::run("Substrate Platform — hello_shell", Shell::new(dock_state))
}
