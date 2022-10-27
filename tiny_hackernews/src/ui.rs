use eframe::egui::{
    self, Button, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout,
    Separator, TextStyle, TopBottomPanel,
};
use std::borrow::Cow;

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

pub(crate) struct TinyHackernews {
    articles: Vec<NewsCardData>,
}

impl TinyHackernews {
    pub fn new() -> TinyHackernews {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title {}", a),
            description: format!("description {}", a),
            url: format!("http://example.com/{}", a),
        });

        TinyHackernews {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
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

    pub fn render_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            // render title
            let title = format!("{}", a.title);
            ui.colored_label(WHITE, title);

            // render desc
            ui.add_space(PADDING);
            let description =
                Label::new(&a.description).text_style(eframe::egui::TextStyle::Button);
            ui.add(description);

            // render url
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);

            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.add(Hyperlink::new(&a.url).text("Read More ⤴"));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
    pub fn render_top_panel(&self, ctx: &CtxRef) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new("📒").text_style(egui::TextStyle::Heading));
                });
                // controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                    let refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                    let theme_btn = ui.add(Button::new("🌙").text_style(egui::TextStyle::Body));
                });
            });
            ui.add_space(10.);
        });
    }
}
