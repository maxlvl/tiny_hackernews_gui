use std::borrow::Cow;

use eframe::{
    egui::{CentralPanel, CtxRef, FontDefinitions, FontFamily, ScrollArea, TextStyle, Vec2},
    epi::App,
    run_native, NativeOptions,
};

struct TinyHackernews {
    articles: Vec<NewsCardData>,
}

impl TinyHackernews {
    fn new() -> TinyHackernews {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title {}", a),
            description: format!("description {}", a),
            url: format!("http://example.com/{}", a),
        });

        TinyHackernews {
            articles: Vec::from_iter(iter),
        }
    }

    fn configure_fonts(&self, ctx: &CtxRef) {
        // create font def object
        // load up the font
        // set the size
        // load the font with the context object

        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "SourceSansPro".to_string(),
            Cow::Borrowed(include_bytes!("../../SourceSansPro-Regular.ttf")),
        );
        font_def
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 45.));

        font_def
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 20.));

        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "SourceSansPro".to_string());

        ctx.set_fonts(font_def);
    }
}
struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

impl App for TinyHackernews {
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
            ScrollArea::auto_sized().show(ui, |ui| {
                for a in &self.articles {
                    ui.label(&a.title);
                    ui.label(&a.description);
                    ui.label(&a.url);
                }
            })
        });
    }

    fn name(&self) -> &str {
        "Tiny HackerNews"
    }
}

fn main() {
    let app = TinyHackernews::new();
    let mut window_option = NativeOptions::default();
    window_option.initial_window_size = Some(Vec2::new(540.00, 960.00));
    run_native(Box::new(app), window_option);
}
