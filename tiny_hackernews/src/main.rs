mod ui;

use eframe::{
    egui::{
        CentralPanel, CtxRef, Hyperlink, ScrollArea, Separator, TextStyle, TopBottomPanel, Ui, Vec2,
    },
    epi::App,
    run_native, NativeOptions,
};

use hackernews_api;
use ui::NewsCardData;

pub fn fetch_news(articles: &mut Vec<NewsCardData>) {
    if let Ok(response) = hackernews_api::get_articles("http://api.hackerwebapp.com/news") {
        let resp_articles = response;
        for a in resp_articles.iter() {
            let news = ui::NewsCardData {
                title: a.title.to_string(),
                url: a.url.to_string(),
            };
            articles.push(news);
        }
    }
}

impl App for ui::TinyHackernews {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        fetch_news(&mut self.articles);
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| self.render_cards(ui));
            render_footer(ctx);
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

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(
                Hyperlink::new("http://api.hackerwebapp.com/news")
                    .text("API Source")
                    .text_style(TextStyle::Monospace),
            );
            ui.add(
                Hyperlink::new("https://github.com/emilk/egui")
                    .text("Made with egui")
                    .text_style(TextStyle::Monospace),
            );
            ui.add_space(10.);
        })
    });
}

fn main() {
    let app = ui::TinyHackernews::new();
    let mut window_option = NativeOptions::default();
    window_option.initial_window_size = Some(Vec2::new(540.00, 960.00));
    run_native(Box::new(app), window_option);
}
