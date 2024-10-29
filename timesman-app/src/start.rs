use crate::app::{Event, Pane};
use crate::req::{Requester, Times};
use eframe::egui::ScrollArea;

pub struct StartPane {
    times: Vec<Times>,
    title: String,
}

impl Pane for StartPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        req: &Requester,
    ) -> Event {
        let mut event = Event::Nothing;

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("server");
                ui.separator();
                ui.label(&req.server);
                ui.separator();
                if ui.button("update").clicked() {
                    //TODO:
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("new");
                ui.separator();
                ui.text_edit_singleline(&mut self.title);
                ui.separator();
                if ui.button("new").clicked() {
                    if let Some(newt) = req.create_times(&self.title) {
                        event = Event::OpenTimes(newt);
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height());
            scroll_area.show(ui, |ui| {
                for t in &self.times {
                    ui.horizontal(|ui| {
                        if ui.button(&t.title).clicked() {
                            event = Event::OpenTimes(t.clone());
                        }
                        ui.label(format!("{}", t.created_at));
                    });
                }
            });
        });

        event
    }
}

impl StartPane {
    pub fn new(req: &Requester) -> Self {
        let list = req.get_list().unwrap();
        Self {
            times: list,
            title: "".to_string(),
        }
    }
}