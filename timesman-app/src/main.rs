use eframe::egui::{self, FontData, FontDefinitions, FontFamily, ScrollArea};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RequestData {
    comment: String,
}

#[derive(Deserialize, Debug)]
struct Comments {
    id: i64,
    comment: String,
    created_at: chrono::NaiveDateTime,
}

struct TimesManApp {
    input_text: String,
    server: String,
    list: Vec<Comments>,
}

impl TimesManApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "ja".to_owned(),
            FontData::from_static(include_bytes!(
                "../fonts/ja/NotoSansJP-VariableFont_wght.ttf"
            )),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "ja".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        Self::default()
    }

    fn post(&mut self, text: &String) {
        let client = reqwest::blocking::Client::new();
        let data = RequestData {
            comment: text.clone(),
        };

        let url = self.server.clone() + "/append";

        let res = client.post(url).json(&data).send();

        let com = Comments {
            id: 0,
            comment: text.clone(),
            created_at: chrono::Local::now().naive_local(),
        };

        self.list.push(com);
    }

    fn get_list(server: &String) -> Vec<Comments> {
        let url = server.clone() + "/list";
        reqwest::blocking::get(url)
            .unwrap()
            .json::<Vec<Comments>>()
            .unwrap()
    }
}

impl Default for TimesManApp {
    fn default() -> Self {
        let server = "http://localhost:8080";
        Self {
            input_text: "".to_owned(),
            server: server.to_string(),
            list: TimesManApp::get_list(&server.to_string()),
        }
    }
}

impl eframe::App for TimesManApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("title").show(ctx, |ui| {
                ui.heading("TimesMan");
            });

            // bottom
            egui::TopBottomPanel::bottom("textbox").show(ctx, |ui| {
                egui::TextEdit::multiline(&mut self.input_text)
                    .hint_text("Type something!")
                    .desired_width(f32::INFINITY)
                    .show(ui);
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                // middle
                let scroll_area = ScrollArea::vertical()
                    .auto_shrink(false)
                    .max_height(ui.available_height())
                    .stick_to_bottom(true);
                scroll_area.show(ui, |ui| {
                    ui.vertical(|ui| {
                        for comment in &self.list {
                            ui.horizontal(|ui| {
                                ui.label(comment.created_at.format("%Y-%m-%d %H:%M").to_string());
                                ui.separator();
                                ui.label(&comment.comment);
                            });
                        }
                    });
                });
            });

            if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::Enter)) {
                if self.input_text.is_empty() {
                    return;
                }

                let text = self.input_text.clone();

                self.post(&text);
                self.input_text.clear();
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TimesMan",
        options,
        Box::new(|cc| Ok(Box::<TimesManApp>::new(TimesManApp::new(cc)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_test() {
        let data = RequestData {
            comment: "hello".to_string(),
        };
        let j = serde_json::to_string(&data).unwrap();
        println!("{}", j);
        assert_eq!(j, r#"{"comment":"hello"}"#);
    }
}
