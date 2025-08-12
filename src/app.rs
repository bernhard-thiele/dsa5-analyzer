use crate::character::Character;
use crate::ui::{FileDialog, CharacterView, ApAnalysis};
use eframe::egui;
use std::path::PathBuf;

pub struct App {
    file_dialog: FileDialog,
    character_view: CharacterView,
    ap_analysis: ApAnalysis,
    current_character: Option<Character>,
    selected_file: Option<PathBuf>,
}

impl App {
    pub fn new() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            character_view: CharacterView::new(),
            ap_analysis: ApAnalysis::new(),
            current_character: None,
            selected_file: None,
        }
    }

    fn load_character(&mut self, path: &PathBuf) -> anyhow::Result<()> {
        let character = Character::from_file(path)?;
        self.current_character = Some(character);
        self.selected_file = Some(path.clone());
        Ok(())
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with file operations
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Load Character File").clicked() {
                    if let Some(path) = self.file_dialog.open_file() {
                        match self.load_character(&path) {
                            Ok(_) => {
                                println!("Successfully loaded character from: {:?}", path);
                            }
                            Err(e) => {
                                eprintln!("Error loading character: {}", e);
                            }
                        }
                    }
                }

                if let Some(ref path) = self.selected_file {
                    ui.label(format!("Loaded: {}", path.file_name().unwrap_or_default().to_string_lossy()));
                }
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.current_character {
                Some(character) => {
                    // Use SidePanel for left panel with fixed width
                    egui::SidePanel::left("character_panel")
                        .show_inside(ui, |ui| {
                            self.character_view.show(ui, character);
                        });

                    // The remaining space will be used for AP analysis
                    egui::CentralPanel::default()
                        .show_inside(ui, |ui| {
                            self.ap_analysis.show(ui, character);
                        });
                }
                None => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(200.0);
                        ui.heading("DSA Character Analyzer");
                        ui.add_space(20.0);
                        ui.label("Click 'Load Character File' to start analyzing a character sheet.");
                    });
                }
            }
        });
    }
}