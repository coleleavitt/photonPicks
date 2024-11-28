//! Main module for the WebSocket server.

mod models;
mod math;
mod errors;

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use eframe::egui;
use crate::models::{TokenData, TokenMap};
use crate::math::{generate_wallet_holdings, collect_recent_trades};
use crate::errors::{Result, WebSocketError};
use futures_util::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};

const ADDR: &str = "127.0.0.1:8080";

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct TokenApp {
    token_data: Arc<RwLock<TokenMap>>,
    tokens_per_page: usize,
    current_page: usize,
    search_query: String,
    cached_tokens: Vec<(String, TokenData)>,
    last_refresh: Instant,
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
        }
    }
}

impl TokenApp {
    fn update_cache(&mut self) {
        let token_map = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.token_data.read().await.clone()
            })
        });

        let mut tokens: Vec<_> = token_map.iter().collect();
        tokens.sort_by(|a, b| {
            let a_risk = a.1.get_concentration_risk(a.1.calculate_adjusted_concentration(
                &generate_wallet_holdings(a.1),
                &collect_recent_trades(a.1),
            ));
            let b_risk = b.1.get_concentration_risk(b.1.calculate_adjusted_concentration(
                &generate_wallet_holdings(b.1),
                &collect_recent_trades(b.1),
            ));

            let risk_value = |risk: &str| -> i32 {
                match risk {
                    "Low Risk" => 0,
                    "Moderate Risk" => 1,
                    "High Risk" => 2,
                    "Very High Risk" => 3,
                    _ => 4,
                }
            };

            let risk_order = risk_value(a_risk).cmp(&risk_value(b_risk));

            if risk_order == std::cmp::Ordering::Equal {
                let a_cap = a.1.attributes.fdv.unwrap_or(0.0);
                let b_cap = b.1.attributes.fdv.unwrap_or(0.0);
                match (a_cap.is_nan(), b_cap.is_nan()) {
                    (true, true) => std::cmp::Ordering::Equal,
                    (true, false) => std::cmp::Ordering::Greater,
                    (false, true) => std::cmp::Ordering::Less,
                    (false, false) => b_cap.total_cmp(&a_cap),
                }
            } else {
                risk_order
            }
        });

        self.cached_tokens = tokens.into_iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
    }

    fn detect_duplicates(&self) {
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();

        for (name, token) in &self.cached_tokens {
            if !seen.insert(name.clone()) {
                duplicates.push((name.clone(), token.attributes.token_address.clone()));
            }
        }

        if !duplicates.is_empty() {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("duplicates.log")
                .unwrap();

            for (name, address) in duplicates {
                writeln!(file, "Duplicate token: {}, Address: {}", name, address.unwrap_or_default()).unwrap();
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
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            ui.heading("Token Monitor");
            ui.style_mut().override_text_style = None;

            // Add search field
            ui.horizontal(|ui| {
                ui.label("Search:");
                if ui.text_edit_singleline(&mut self.search_query).changed() {
                    self.current_page = 0;
                    self.update_cache();
                }
            });

            // Add refresh button
            if ui.button("Refresh").clicked() {
                self.update_cache();
                self.detect_duplicates();
            }

            // Add pagination controls
            ui.horizontal(|ui| {
                ui.label("Tokens per page:");
                ui.add(egui::Slider::new(&mut self.tokens_per_page, 5..=50));

                let filtered_tokens: Vec<_> = self.cached_tokens
                    .iter()
                    .filter(|(_, token)| {
                        if self.search_query.is_empty() {
                            return true;
                        }
                        let query = self.search_query.to_lowercase();
                        token.attributes.name.as_ref().map_or(false, |name|
                            name.to_lowercase().contains(&query)
                        ) ||
                            token.attributes.symbol.as_ref().map_or(false, |symbol|
                                symbol.to_lowercase().contains(&query)
                            )
                    })
                    .collect();

                let total_pages = (filtered_tokens.len() as f32 / self.tokens_per_page as f32).ceil() as usize;
                if ui.button("←").clicked() && self.current_page > 0 {
                    self.current_page -= 1;
                }
                ui.label(format!("Page {} of {}", self.current_page + 1, total_pages));
                if ui.button("→").clicked() && self.current_page < total_pages - 1 {
                    self.current_page += 1;
                }
            });

            let filtered_tokens: Vec<_> = self.cached_tokens
                .iter()
                .filter(|(_, token)| {
                    if self.search_query.is_empty() {
                        return true;
                    }
                    let query = self.search_query.to_lowercase();
                    token.attributes.name.as_ref().map_or(false, |name|
                        name.to_lowercase().contains(&query)
                    ) ||
                        token.attributes.symbol.as_ref().map_or(false, |symbol|
                            symbol.to_lowercase().contains(&query)
                        )
                })
                .collect();

            let start = self.current_page * self.tokens_per_page;
            let end = (start + self.tokens_per_page).min(filtered_tokens.len());

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (_, token) in &filtered_tokens[start..end] {
                    let trades = collect_recent_trades(token);
                    let wallet_holdings = generate_wallet_holdings(token);
                    let hhi = token.calculate_adjusted_concentration(&wallet_holdings, &trades);
                    let risk_level = token.get_concentration_risk(hhi);

                    let frame_color = match risk_level {
                        "Low Risk" => egui::Color32::from_rgb(20, 110, 20),
                        "Moderate Risk" => egui::Color32::from_rgb(110, 110, 20),
                        "High Risk" => egui::Color32::from_rgb(110, 60, 20),
                        "Very High Risk" => egui::Color32::from_rgb(110, 20, 20),
                        _ => ui.visuals().widgets.noninteractive.bg_fill,
                    };

                    egui::Frame::none()
                        .fill(frame_color)
                        .inner_margin(8.0)
                        .rounding(5.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(60)))
                        .show(ui, |ui| {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    if let Some(name) = &token.attributes.name {
                                        ui.heading(egui::RichText::new(name.to_string())
                                            .strong()
                                            .size(20.0));
                                    }

                                    ui.horizontal(|ui| {
                                        if let Some(addr) = &token.attributes.token_address {
                                            ui.label(egui::RichText::new(format!("Address: {}", addr))
                                                .monospace()
                                                .size(14.0));
                                        }
                                    });

                                    ui.add_space(4.0);

                                    ui.horizontal(|ui| {
                                        if let Some(symbol) = &token.attributes.symbol {
                                            ui.label(egui::RichText::new(format!("Symbol: {}", symbol))
                                                .monospace()
                                                .size(14.0));
                                        }
                                        ui.add_space(20.0);
                                        if let Some(price) = token.attributes.price_usd {
                                            ui.label(egui::RichText::new(format!("Price: ${:.8}", price))
                                                .monospace()
                                                .size(14.0));
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        if let Some(market_cap) = token.attributes.fdv {
                                            ui.label(egui::RichText::new(format!("Market Cap: ${:.2}", market_cap))
                                                .monospace()
                                                .size(14.0));
                                        }
                                        ui.add_space(20.0);
                                        if let Some(volume) = token.attributes.volume {
                                            ui.label(egui::RichText::new(format!("Volume: ${:.2}", volume))
                                                .monospace()
                                                .size(14.0));
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        if let Some(holders) = token.attributes.holders_count {
                                            ui.label(egui::RichText::new(format!("Holders: {}", holders))
                                                .monospace()
                                                .size(14.0));
                                        }
                                        ui.add_space(20.0);
                                        ui.label(egui::RichText::new(format!("Concentration: {:.4}", hhi))
                                            .monospace()
                                            .size(14.0));
                                        ui.add_space(20.0);

                                        let risk_color = match risk_level {
                                            "Low Risk" => egui::Color32::from_rgb(100, 255, 100),
                                            "Moderate Risk" => egui::Color32::from_rgb(255, 255, 100),
                                            "High Risk" => egui::Color32::from_rgb(255, 180, 100),
                                            "Very High Risk" => egui::Color32::from_rgb(255, 100, 100),
                                            _ => ui.visuals().text_color(),
                                        };

                                        ui.label(egui::RichText::new(risk_level)
                                            .strong()
                                            .monospace()
                                            .color(risk_color)
                                            .size(14.0));
                                    });
                                });
                            });
                            ui.add_space(4.0);
                        });
                    ui.add_space(4.0);
                }
            });
        });

        ctx.request_repaint();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = TokenApp::default();
    let token_map = Arc::clone(&app.token_data);

    tokio::spawn(async move {
        if let Ok(listener) = TcpListener::bind(ADDR).await {
            println!("WebSocket server listening on ws://{ADDR}");
            while let Ok((stream, _)) = listener.accept().await {
                let token_map = Arc::clone(&token_map);
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, token_map).await {
                        eprintln!("Error handling connection: {e}");
                    }
                });
            }
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::Vec2::new(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Token Monitor",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    ).map_err(|e| WebSocketError::Other(e.to_string()))?;

    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    token_map: Arc<RwLock<TokenMap>>,
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (_write, read) = ws_stream.split();

    read.try_for_each(|msg| {
        let token_map = Arc::clone(&token_map);
        async move {
            match msg {
                Message::Text(text) => {
                    if let Err(e) = process_message(&text, &token_map).await {
                        eprintln!("Error processing message: {e}");
                    }
                }
                Message::Close(_) => return Ok(()),
                _ => {}
            }
            Ok(())
        }
    })
    .await?;

    Ok(())
}

async fn process_message(text: &str, token_map: &Arc<RwLock<TokenMap>>) -> Result<()> {
    let json: serde_json::Value = serde_json::from_str(text)?;
    if let Some(tokens) = json.get("tokens") {
        update_token_map(tokens, token_map).await?;
    }
    Ok(())
}

async fn update_token_map(
    tokens: &serde_json::Value,
    token_map: &Arc<RwLock<TokenMap>>,
) -> Result<()> {
    let tokens_array = tokens.as_array()
        .ok_or_else(|| WebSocketError::TokenParse("Expected array".into()))?;

    for token_value in tokens_array {
        let token: TokenData = serde_json::from_value(token_value.clone())?;
        token_map.write().await.insert(token.id.clone(), token);
    }

    Ok(())
}