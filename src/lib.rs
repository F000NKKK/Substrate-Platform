//! Substrate Platform — a brand-neutral IDE shell core.
//!
//! Provides the pieces every IDE-style desktop tool needs regardless of what
//! it's actually editing: a dockable panel layout ([`Shell`]) and the trait
//! a product plugs its own panels into ([`Panel`]). Nothing here knows about
//! any specific product (no Minecraft/Yog concepts) — that's the whole
//! point: a product like Yog-IDLE depends on this crate the same way any
//! future, unrelated IDE product would.

pub use egui_dock::{DockState, NodeIndex, SurfaceIndex};

/// One dockable panel in the shell. A product implements this per panel
/// kind it wants (a tree view, a property inspector, a canvas, ...).
pub trait Panel {
    /// Tab title shown in the dock.
    fn title(&self) -> String;
    /// Draw this panel's contents into `ui`.
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// A running dock layout of boxed [`Panel`]s. Products build one of these
/// with their own panel implementations and hand it to [`run`].
pub struct Shell {
    pub dock_state: DockState<Box<dyn Panel>>,
}

impl Shell {
    pub fn new(dock_state: DockState<Box<dyn Panel>>) -> Self {
        Self { dock_state }
    }
}

struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = Box<dyn Panel>;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui);
    }
}

impl eframe::App for Shell {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui_dock::DockArea::new(&mut self.dock_state).show_inside(ui, &mut TabViewer);
    }
}

/// Launch a native window running `shell`. Blocks until the window closes.
pub fn run(window_title: &str, shell: Shell) -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        window_title,
        options,
        Box::new(|_cc| Ok(Box::new(shell))),
    )
}
