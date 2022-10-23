mod ui;

use eframe::{
    egui::{CentralPanel, ScrollArea, Vec2},
    epi::App,
    run_native, NativeOptions,
};

impl App for ui::TinyHackernews {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| self.render_cards(ui))
        });
    }

    fn name(&self) -> &str {
        "Tiny HackerNews"
    }
}

fn main() {
    let app = ui::TinyHackernews::new();
    let mut window_option = NativeOptions::default();
    window_option.initial_window_size = Some(Vec2::new(540.00, 960.00));
    run_native(Box::new(app), window_option);
}
