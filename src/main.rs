mod errors;
mod math;
mod models;
mod websocket;
mod risk;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use crate::errors::{Result, WebSocketError};
use crate::models::{TokenData, TokenMap};
use eframe::egui;
use crate::risk::{RiskCalculator, RiskLevel};
use crate::websocket::WebSocketServer;
use std::fs::OpenOptions;
use chrono::TimeZone;

const ADDR: &str = "127.0.0.1:8080";
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(PartialEq)]
#[derive(Clone)]
enum SortType {
    Risk,
    Age,
    MarketCap,
    Volume,
    Newest
}


struct TokenApp {
    token_data: Arc<RwLock<TokenMap>>,
    tokens_per_page: usize,
    current_page: usize,
    search_query: String,
    cached_tokens: Vec<(String, TokenData)>,
    last_refresh: Instant,
    risk_calculator: RiskCalculator,
    active_filters: HashSet<RiskLevel>,
    show_filter_menu: bool,
    sort_type: SortType,
    last_update: Instant,
    active_social_filters: HashSet<String>
}

impl Default for TokenApp {
    fn default() -> Self {
        Self {
            token_data: Arc::new(RwLock::new(TokenMap::default())),
            tokens_per_page: 10,
            current_page: 0,
            search_query: String::new(),
            cached_tokens: Vec::new(),
            last_refresh: Instant::now(),
            risk_calculator: RiskCalculator::default(),
            active_filters: HashSet::new(),
            show_filter_menu: false,
            sort_type: SortType::Risk,
            last_update: Instant::now(),
            active_social_filters: HashSet::new()
        }
    }
}

impl TokenApp {
    fn get_filtered_tokens(&self) -> Vec<&(String, TokenData)> {
        self.cached_tokens
            .iter()
            .filter(|(_, token)| {
                let matches_search = if self.search_query.is_empty() {
                    true
                } else {
                    let query = self.search_query.to_lowercase();
                    token.attributes.name.as_ref()
                        .map_or(false, |name| name.to_lowercase().contains(&query))
                        || token.attributes.symbol.as_ref()
                        .map_or(false, |symbol| symbol.to_lowercase().contains(&query))
                };

                let matches_risk = if self.active_filters.is_empty() {
                    true
                } else {
                    let risk_level = self.risk_calculator.calculate_risk(token);
                    self.active_filters.contains(&risk_level)
                };

                // Add social media filter logic here
                let matches_socials = if self.active_social_filters.is_empty() {
                    true
                } else {
                    token.attributes.socials.as_ref().map_or(false, |socials| {
                        self.active_social_filters.iter().all(|filter| {
                            match filter.as_str() {
                                "Twitter" => socials.twitter.as_ref().map_or(false, |url| url.starts_with("https://x.com")),
                                "Reddit" => socials.reddit.is_some(),
                                "Telegram" => socials.telegram.as_ref().map_or(false, |url| url.contains("https://t.me")),
                                _ => false,
                            }
                        })
                    })
                };

                matches_search && matches_risk && matches_socials
            })
            .collect()
    }


    fn update_cache(&mut self) {
        let token_map = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.token_data.read().await.clone() })
        });

        let mut unique_tokens: HashMap<String, TokenData> = HashMap::new();

        for (_, token) in token_map.iter() {
            if let Some(addr) = &token.attributes.token_address {
                unique_tokens.insert(addr.clone().parse().unwrap(), token.clone());
            }
        }

        let mut tokens: Vec<_> = unique_tokens.into_iter().collect();

        // Sort tokens based on the current sort type
        tokens.sort_by(|a, b| match self.sort_type {
            SortType::Newest => {
                let a_created = a.1.attributes.created_timestamp.unwrap_or(0);
                let b_created = b.1.attributes.created_timestamp.unwrap_or(0);
                b_created.cmp(&a_created) // Newest first
            }
            SortType::Risk => {
                let a_risk = self.risk_calculator.calculate_risk(&a.1);
                let b_risk = self.risk_calculator.calculate_risk(&b.1);
                a_risk.cmp(&b_risk)
            }
            SortType::MarketCap => {
                let a_cap = a.1.attributes.fdv.unwrap_or(0.0);
                let b_cap = b.1.attributes.fdv.unwrap_or(0.0);
                b_cap.partial_cmp(&a_cap).unwrap_or(std::cmp::Ordering::Equal) // Descending order
            }
            SortType::Volume => {
                let a_vol = a.1.attributes.volume.unwrap_or(0.0);
                let b_vol = b.1.attributes.volume.unwrap_or(0.0);
                b_vol.partial_cmp(&a_vol).unwrap_or(std::cmp::Ordering::Equal) // Descending order
            }
            SortType::Age => {
                let a_created = a.1.attributes.created_timestamp.unwrap_or(0);
                let b_created = b.1.attributes.created_timestamp.unwrap_or(0);
                a_created.cmp(&b_created) // Oldest first
            }
        });

        // Update cached_tokens with unique entries
        self.cached_tokens = tokens
            .into_iter()
            .map(|(addr, token)| {
                let display_name = token.attributes.name
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| addr.clone());
                (display_name, token)
            })
            .collect();
    }
    
    fn show_token_details_static(
        ui: &mut egui::Ui,
        token: &TokenData,
        risk_level: RiskLevel,
        risk_calculator: &RiskCalculator,
    ) {
        let risk_color = risk_calculator.get_risk_color(risk_level);

        egui::Frame::none()
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(60)))
            .rounding(8.0)
            .shadow(egui::epaint::Shadow::default())
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;

                ui.horizontal(|ui| {
                    if let Some(name) = &token.attributes.name {
                        ui.heading(egui::RichText::new(name.as_ref()).size(20.0).strong());
                    }
                    if let Some(symbol) = &token.attributes.symbol {
                        ui.label(egui::RichText::new(format!("({})", symbol)).monospace());
                    }
                });

                egui::Grid::new(format!(
                    "token_details_{}_{}_{:?}",
                    token.attributes.token_address.as_deref().unwrap_or("unknown"),
                    token.attributes.symbol.as_deref().unwrap_or(""),
                    ui.next_auto_id()  // Add a unique runtime ID
                ))


                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        // Address with hyperlink and copy functionality
                        ui.label("Address:");
                        if let Some(address) = &token.attributes.token_address {
                            let text = egui::RichText::new(address.as_ref())
                                .monospace()
                                .size(14.0)
                                .color(egui::Color32::from_rgb(100, 150, 255));

                            let response = ui.link(text);

                            // Left click for hyperlink
                            if response.clicked() {
                                let url = format!(
                                    "https://photon-sol.tinyastro.io/en/lp/{}?handle=846942c86d4c86d6797ec",
                                    address
                                );
                                ui.output_mut(|o| o.open_url = Some(egui::output::OpenUrl::new_tab(url)));
                            }

                            // Right click for copy
                            if response.secondary_clicked() {
                                let url = format!(
                                    "https://photon-sol.tinyastro.io/en/lp/{}?handle=846942c86d4c86d6797ec",
                                    address
                                );
                                ui.output_mut(|o| o.copied_text = url.to_string());
                            }

                            // Show tooltip on hover
                            response.on_hover_text("Left click to open in browser\nRight click to copy address");
                        } else {
                            ui.label(egui::RichText::new("N/A").monospace().size(14.0));
                        }
                        ui.end_row();

                        // Age
                        ui.label("Age:");
                        if let Some(created_timestamp) = token.attributes.created_timestamp {
                            // Calculate the elapsed time in seconds since the token was created
                            let now = chrono::Utc::now();
                            let creation_time = chrono::DateTime::from_timestamp(
                                created_timestamp,
                                0
                            ).unwrap_or_else(|| chrono::DateTime::<chrono::Utc>::from_timestamp(0, 0).unwrap());

                            let duration = now.signed_duration_since(creation_time);

                            // Convert duration to its components  
                            let days = duration.num_days();
                            let hours = duration.num_hours() % 24;
                            let minutes = duration.num_minutes() % 60;
                            let seconds = duration.num_seconds() % 60;

                            // Create the formatted age string
                            let age_string = format!("{}d {}h {}m {}s", days, hours, minutes, seconds);
                            ui.label(age_string);
                        } else {
                            ui.label("Unknown");
                        }
                        ui.end_row();



                        // Market Cap
                        ui.label("Market Cap:");
                        ui.label(format!("${:.2}M", token.attributes.fdv.unwrap_or(0.0) / 1_000_000.0));
                        ui.end_row();

                        // Volume
                        ui.label("24h Volume:");
                        ui.label(format!("${:.2}M", token.attributes.volume.unwrap_or(0.0) / 1_000_000.0));
                        ui.end_row();

                        // Holders
                        ui.label("Holders:");
                        ui.label(format!("{}", token.attributes.holders_count.unwrap_or(0)));
                        ui.end_row();

                        // Price
                        ui.label("Price:");
                        ui.label(format!("${:.6}", token.attributes.price_usd.unwrap_or(0.0)));
                        ui.end_row();

                        // Risk Level
                        ui.label("Risk Level:");
                        ui.label(
                            egui::RichText::new(risk_calculator.get_risk_text(risk_level))
                                .color(risk_color)
                                .strong()
                        );
                        ui.end_row();
                    });
            });
    }

    fn detect_duplicates(&self) {
        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();

        for (name, token) in &self.cached_tokens {
            if !seen.insert(name.clone()) {
                duplicates.push((name.clone(), token.attributes.token_address.clone()));
            }
        }

        if !duplicates.is_empty() {
            use std::io::Write;
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("duplicates.log")
                .unwrap();

            for (name, address) in duplicates {
                writeln!(
                    file,
                    "Duplicate token: {}, Address: {}",
                    name,
                    address.unwrap_or_default()
                )
                    .unwrap();
            }
        }
    }
}

impl eframe::App for TokenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.cached_tokens.is_empty() || self.last_refresh.elapsed() > Duration::from_secs(5) {
            self.update_cache();
            self.detect_duplicates();
            self.last_refresh = Instant::now();
            self.last_update = Instant::now();
        }

        let filtered_tokens = self.get_filtered_tokens();
        let total_pages = (filtered_tokens.len() as f32 / self.tokens_per_page as f32).ceil() as usize;

        let mut tokens_per_page = self.tokens_per_page;
        let mut current_page = self.current_page;
        let mut search_query = self.search_query.clone();
        let mut active_filters = self.active_filters.clone();
        let mut show_filter_menu = self.show_filter_menu;
        let mut sort_type = self.sort_type.clone();
        let mut active_social_filters = self.active_social_filters.clone(); // Make a local copy
        let risk_calculator = &self.risk_calculator;

        let needs_cache_update = egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            ui.heading("Token Monitor");
            ui.style_mut().override_text_style = None;

            let mut needs_update = false;

            ui.horizontal(|ui| {
                if ui.selectable_value(&mut sort_type, SortType::Risk, "Risk").clicked() {
                    needs_update = true;
                }
                if ui.selectable_value(&mut sort_type, SortType::Newest, "Newest Coins").clicked() {
                    needs_update = true;
                }
                if ui.selectable_value(&mut sort_type, SortType::MarketCap, "Market Cap").clicked() {
                    needs_update = true;
                }
                if ui.selectable_value(&mut sort_type, SortType::Volume, "Volume").clicked() {
                    needs_update = true;
                }
                if ui.selectable_value(&mut sort_type, SortType::Age, "Age").clicked() {
                    needs_update = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Search:");
                if ui.text_edit_singleline(&mut search_query).changed() {
                    current_page = 0;
                    needs_update = true;
                }
                ui.add(egui::Label::new("🔍"))
                    .on_hover_text("Search tokens by name or symbol");
            });

            ui.horizontal(|ui| {
                if ui.button("🔍 Filters").clicked() {
                    show_filter_menu = !show_filter_menu;
                }

                for risk_level in [RiskLevel::Low, RiskLevel::Moderate, RiskLevel::High, RiskLevel::VeryHigh] {
                    if active_filters.contains(&risk_level) {
                        let chip = egui::Frame::none()
                            .fill(risk_calculator.get_risk_color(risk_level))
                            .rounding(16.0)
                            .inner_margin(egui::vec2(8.0, 4.0));

                        chip.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(risk_calculator.get_risk_text(risk_level))
                                        .color(egui::Color32::WHITE)
                                );
                                if ui.button("✕").clicked() {
                                    active_filters.remove(&risk_level);
                                    needs_update = true;
                                }
                            });
                        });
                    }
                }
            });
            
            if show_filter_menu {
                egui::Window::new("Filters")
                    .fixed_size([300.0, 400.0])
                    .show(ui.ctx(), |ui| {
                        ui.heading("Risk Levels");
                        ui.add_space(8.0);

                        // Risk Level Filters
                        for risk_level in [RiskLevel::Low, RiskLevel::Moderate, RiskLevel::High, RiskLevel::VeryHigh] {
                            let text = risk_calculator.get_risk_text(risk_level);
                            let color = risk_calculator.get_risk_color(risk_level);
                            if ui.add(egui::SelectableLabel::new(
                                active_filters.contains(&risk_level),
                                egui::RichText::new(text).color(color).strong()
                            )).clicked() {
                                if active_filters.contains(&risk_level) {
                                    active_filters.remove(&risk_level);
                                } else {
                                    active_filters.insert(risk_level);
                                }
                                needs_update = true;
                            }
                        }

                        ui.add_space(16.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Social Media Filters
                        ui.heading("Social Media");
                        ui.add_space(8.0);

                        let social_options = [
                            ("Twitter", "🐦", "Filter tokens with Twitter presence"),
                            ("Reddit", "🔵", "Filter tokens with Reddit presence"),
                            ("Telegram", "✈️", "Filter tokens with Telegram presence")
                        ];

                        for (platform, icon, tooltip) in social_options {
                            let is_active = active_social_filters.contains(platform);
                            let response = ui.add(
                                egui::SelectableLabel::new(
                                    is_active,
                                    egui::RichText::new(format!("{} {}", icon, platform))
                                        .strong()
                                        .color(
                                            if is_active {
                                                egui::Color32::from_rgb(100, 150, 255)
                                            } else {
                                                ui.style().visuals.text_color()
                                            }
                                        )
                                )
                            );

                            if response.clicked() {
                                if is_active {
                                    active_social_filters.remove(platform);
                                } else {
                                    active_social_filters.insert(platform.to_string());
                                }
                                needs_update = true;
                            }

                            response.on_hover_text(tooltip);
                        }

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Filter Actions
                        ui.horizontal(|ui| {
                            if ui.button("Clear All").clicked() {
                                active_filters.clear();
                                active_social_filters.clear();
                                needs_update = true;
                            }

                            if ui.button("Apply").clicked() {
                                show_filter_menu = false;
                                needs_update = true;
                            }
                        });
                    });
            }


            ui.horizontal(|ui| {
                ui.label("Tokens per page:");
                ui.add(egui::Slider::new(&mut tokens_per_page, 5..=50));
                if ui.button("←").clicked() && current_page > 0 {
                    current_page -= 1;
                }
                ui.label(format!("Page {} of {}", current_page + 1, total_pages));
                if ui.button("→").clicked() && current_page < total_pages - 1 {
                    current_page += 1;
                }
            });

            let start = current_page * tokens_per_page;
            let end = (start + tokens_per_page).min(filtered_tokens.len());

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (_name, token) in &filtered_tokens[start..end] {
                    let risk_level = risk_calculator.calculate_risk(token);
                    TokenApp::show_token_details_static(ui, token, risk_level, risk_calculator);
                }
            });

            needs_update
        }).inner;

        self.sort_type = sort_type;
        self.tokens_per_page = tokens_per_page;
        self.current_page = current_page;
        self.search_query = search_query;
        self.active_filters = active_filters;
        self.active_social_filters = active_social_filters; // Apply changes to active social filters
        self.show_filter_menu = show_filter_menu;

        if needs_cache_update {
            self.update_cache();
            self.detect_duplicates();
        }

        ctx.request_repaint();
    }
}



#[tokio::main]
async fn main() -> Result<()> {
    let app = TokenApp::default();
    let token_map = Arc::clone(&app.token_data);

    // Create and spawn WebSocket server
    let ws_server = WebSocketServer::new(ADDR.to_string(), token_map);

    tokio::spawn(async move {
        if let Err(e) = ws_server.run().await {
            eprintln!("WebSocket server error: {e}");
        }
    });

    // Configure and run UI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::Vec2::new(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native("Token Monitor", options, Box::new(|_cc| Ok(Box::new(app))))
        .map_err(|e| WebSocketError::Other(e.to_string()))?;

    Ok(())
}