mod ui;

use eframe::{
    egui::{CentralPanel, ScrollArea, Separator, Ui, Vec2},
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
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| self.render_cards(ui))
        });
    }

    fn name(&self) -> &str {
        "Tiny HackerNews"
    }
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Tiny HackerNews");
    });

    ui.add_space(ui::PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

// fn render_footer()_{
// pass

// }

fn main() {
    let app = ui::TinyHackernews::new();
    let mut window_option = NativeOptions::default();
    window_option.initial_window_size = Some(Vec2::new(540.00, 960.00));
    run_native(Box::new(app), window_option);
}
