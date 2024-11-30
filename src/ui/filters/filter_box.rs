use crate::models::Audit;
use crate::risk::{RiskCalculator, RiskLevel};
use eframe::egui;
use std::collections::HashSet;

#[derive(Clone)]
pub struct FilterBox {
    pub active_filters: HashSet<RiskLevel>,
    pub active_social_filters: HashSet<String>,
    pub active_audit_filters: HashSet<Audit>,
    pub show_filter_menu: bool,
}

impl FilterBox {
    pub fn new() -> Self {
        Self {
            active_filters: HashSet::new(),
            active_social_filters: HashSet::new(),
            active_audit_filters: HashSet::new(),
            show_filter_menu: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, risk_calculator: &RiskCalculator) -> bool {
        let mut needs_update = false;

        if !self.show_filter_menu {
            return needs_update;
        }

        egui::Window::new("Filters")
            .fixed_size([320.0, 480.0])
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::RIGHT_TOP, [-8.0, 8.0])
            .frame(Self::create_window_frame(ctx))
            .show(ctx, |ui| {
                self.apply_dark_theme(ui);
                needs_update |= self.show_filter_content(ui, risk_calculator);
            });

        needs_update
    }

    fn show_filter_content(&mut self, ui: &mut egui::Ui, risk_calculator: &RiskCalculator) -> bool {
        let mut needs_update = false;
        needs_update |= self.show_filter_section_risk_levels(ui, risk_calculator);
        needs_update |= self.show_filter_section_social_media(ui);
        needs_update |= self.process_bottom_actions(ui);
        needs_update
    }

    fn display_active_count_label(ui: &mut egui::Ui, count: usize) {
        if count > 0 {
            ui.label(
                egui::RichText::new(format!("{} selected", count))
                    .size(12.0)
                    .color(egui::Color32::from_rgb(139, 148, 158)),
            );
        }
    }

    fn show_filter_section_risk_levels(
        &mut self,
        ui: &mut egui::Ui,
        risk_calculator: &RiskCalculator,
    ) -> bool {
        let mut needs_update = false;
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    egui::RichText::new("Risk Levels")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(201, 209, 217)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let active_count = self.active_filters.len();
                    Self::display_active_count_label(ui, active_count);
                });
            });

            ui.add_space(8.0);

            let risk_colors = [
                egui::Color32::from_rgb(87, 171, 90),
                egui::Color32::from_rgb(187, 128, 9),
                egui::Color32::from_rgb(219, 97, 62),
                egui::Color32::from_rgb(219, 68, 55),
            ];

            for (risk_level, base_color) in [
                RiskLevel::Low,
                RiskLevel::Moderate,
                RiskLevel::High,
                RiskLevel::VeryHigh,
            ]
            .iter()
            .zip(risk_colors.iter())
            {
                let text = risk_calculator.get_risk_text(*risk_level);
                let is_active = self.active_filters.contains(risk_level);
                let response = ui.add(egui::SelectableLabel::new(
                    is_active,
                    egui::RichText::new(format!("⬤ {}", text))
                        .color(if is_active {
                            *base_color
                        } else {
                            base_color.linear_multiply(0.5)
                        })
                        .size(14.0),
                ));

                if response.clicked() {
                    if is_active {
                        self.active_filters.remove(risk_level);
                    } else {
                        self.active_filters.insert(*risk_level);
                    }
                    needs_update = true;
                }
            }
        });

        needs_update
    }

    fn show_filter_section_social_media(&mut self, ui: &mut egui::Ui) -> bool {
        let mut needs_update = false;
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    egui::RichText::new("Social Media")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(201, 209, 217)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let active_count = self.active_social_filters.len();
                    Self::display_active_count_label(ui, active_count);
                });
            });

            ui.add_space(8.0);

            let social_options = [
                ("Twitter", "🐦", egui::Color32::from_rgb(29, 155, 240)),
                ("Reddit", "🔵", egui::Color32::from_rgb(255, 88, 62)),
                ("Telegram", "✈️", egui::Color32::from_rgb(34, 158, 217)),
            ];

            for (platform, icon, brand_color) in social_options {
                let is_active = self.active_social_filters.contains(platform);
                let response = ui.add(egui::SelectableLabel::new(
                    is_active,
                    egui::RichText::new(format!("{} {}", icon, platform))
                        .size(14.0)
                        .color(if is_active {
                            brand_color
                        } else {
                            egui::Color32::from_rgb(139, 148, 158)
                        }),
                ));

                if response.clicked() {
                    if is_active {
                        self.active_social_filters.remove(platform);
                    } else {
                        self.active_social_filters.insert(platform.to_string());
                    }
                    needs_update = true;
                }
            }
        });

        needs_update
    }

    fn process_bottom_actions(&mut self, ui: &mut egui::Ui) -> bool {
        let mut needs_update = false;
        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.horizontal(|ui| {
                let total_active = self.active_filters.len() + self.active_social_filters.len();

                if ui
                    .add_enabled(
                        total_active > 0,
                        egui::Button::new(egui::RichText::new("Reset All").size(14.0))
                            .fill(egui::Color32::from_rgb(45, 51, 59))
                            .min_size(egui::vec2(80.0, 32.0)),
                    )
                    .clicked()
                {
                    self.active_filters.clear();
                    self.active_social_filters.clear();
                    self.active_audit_filters.clear();
                    needs_update = true;
                }

                ui.add_space(8.0);

                if ui
                    .add(
                        egui::Button::new(egui::RichText::new("Apply Filters").size(14.0))
                            .fill(if total_active > 0 {
                                egui::Color32::from_rgb(47, 129, 247)
                            } else {
                                egui::Color32::from_rgb(48, 54, 61)
                            })
                            .min_size(egui::vec2(100.0, 32.0)),
                    )
                    .clicked()
                {
                    self.show_filter_menu = false;
                    needs_update = true;
                }
            });

            let total_active = self.active_filters.len() + self.active_social_filters.len();
            if total_active > 0 {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(format!("{} active filters", total_active))
                        .size(13.0)
                        .color(egui::Color32::from_rgb(139, 148, 158)),
                );
            }
        });

        needs_update
    }

    fn create_window_frame(ctx: &egui::Context) -> egui::Frame {
        egui::Frame::window(&ctx.style())
            .inner_margin(16.0)
            .fill(egui::Color32::from_rgb(13, 17, 23))
            .shadow(egui::epaint::Shadow {
                offset: egui::vec2(0.0, 4.0),
                blur: 12.0,
                spread: 2.0,
                color: egui::Color32::from_black_alpha(35),
            })
            .rounding(egui::Rounding::same(8.0))
    }

    fn apply_dark_theme(&self, ui: &mut egui::Ui) {
        let visuals = &mut ui.style_mut().visuals;
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(22, 27, 34);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(33, 38, 45);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(41, 46, 53);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(48, 54, 61);

        let rounding = egui::Rounding::same(4.0);
        visuals.widgets.noninteractive.rounding = rounding;
        visuals.widgets.inactive.rounding = rounding;
        visuals.widgets.active.rounding = rounding;
        visuals.widgets.hovered.rounding = rounding;
    }
}
