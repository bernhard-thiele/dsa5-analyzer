use crate::character::Character;
use eframe::egui;
use egui_extras::{TableBuilder, Column};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum CharacterTab {
    Overview,
    System,
    Skills,
    CombatSkills,
    Magic,
    Karma,
    Items,
}

pub struct CharacterView {
    image_cache: HashMap<String, egui::TextureHandle>,
    selected_tab: CharacterTab,
    load_images: bool,
}

impl CharacterView {
    pub fn new() -> Self {
        Self {
            image_cache: HashMap::new(),
            selected_tab: CharacterTab::Overview,
            load_images: false, // Default to false for faster startup
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, character: &Character) {
        // Tab selection
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.selected_tab, CharacterTab::Overview, "Overview");
            ui.selectable_value(&mut self.selected_tab, CharacterTab::System, "System");
            ui.selectable_value(&mut self.selected_tab, CharacterTab::Skills, "Skills");
            ui.selectable_value(&mut self.selected_tab, CharacterTab::CombatSkills, "Combat Skills");
            ui.selectable_value(&mut self.selected_tab, CharacterTab::Magic, "Magic");
            ui.selectable_value(&mut self.selected_tab, CharacterTab::Karma, "Karma");
            ui.selectable_value(&mut self.selected_tab, CharacterTab::Items, "Items");
        });

        ui.separator();

        // Show content based on selected tab
        match self.selected_tab {
            CharacterTab::Overview => self.show_overview_tab(ui, character),
            CharacterTab::System => self.show_system_tab(ui, character),
            CharacterTab::Skills => self.show_skills_tab(ui, character),
            CharacterTab::CombatSkills => self.show_combat_skills_tab(ui, character),
            CharacterTab::Magic => self.show_magic_tab(ui, character),
            CharacterTab::Karma => self.show_karma_tab(ui, character),
            CharacterTab::Items => self.show_items_tab(ui, character),
        }
    }

    fn show_overview_tab(&mut self, ui: &mut egui::Ui, character: &Character) {
        // Image loading toggle and display
        if character.has_image() {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.load_images, "Load character image");
            });

            if self.load_images {
                self.show_character_image(ui, character);
            } else {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label("🖼 Character image available");
                });
            }
            ui.add_space(10.0);
        }

        ui.heading("📊 Character Information");
        ui.separator();

        // Character info section
        egui::Grid::new("character_info")
            .num_columns(2)
            .spacing([10.0, 5.0])
            .show(ui, |ui| {
                ui.label("Name:");
                ui.label(&character.name);
                ui.end_row();

                ui.label("Type:");
                ui.label(&character.character_type);
                ui.end_row();

                ui.label("Total Items:");
                ui.label(character.items.len().to_string());
                ui.end_row();

                ui.label("Skills:");
                ui.label(character.get_skills().len().to_string());
                ui.end_row();

                ui.label("Combat Skills:");
                ui.label(character.get_combat_skills().len().to_string());
                ui.end_row();

                ui.label("Spells:");
                ui.label(character.get_spells().len().to_string());
                ui.end_row();

                ui.label("Rituals:");
                ui.label(character.get_rituals().len().to_string());
                ui.end_row();

                ui.label("Magic Tricks:");
                ui.label(character.get_magic_tricks().len().to_string());
                ui.end_row();

                ui.label("Liturgies:");
                ui.label(character.get_liturgies().len().to_string());
                ui.end_row();

                ui.label("Ceremonies:");
                ui.label(character.get_ceremonies().len().to_string());
                ui.end_row();

                ui.label("Blessings:");
                ui.label(character.get_blessings().len().to_string());
                ui.end_row();

                ui.label("Advantages:");
                ui.label(character.get_advantages().len().to_string());
                ui.end_row();

                ui.label("Disadvantages:");
                ui.label(character.get_disadvantages().len().to_string());
                ui.end_row();
            });

        // Experience section
        if let Some(system) = &character.system {
            if let Some(details) = &system.details {
                if let Some(experience) = &details.experience {

                    ui.add_space(20.0);
                    ui.heading("🎖 Experience Points");
                    ui.separator();

                    egui::Grid::new("experience_info")
                        .num_columns(2)
                        .spacing([10.0, 5.0])
                        .show(ui, |ui| {
                            ui.label("Total AP:");
                            ui.label(experience.total().to_string());
                            ui.end_row();

                            ui.label("Spent AP (Foundry VTT):");
                            ui.label(experience.spent().to_string());
                            ui.end_row();
                        });
                }
            }
        }

    }


    fn show_system_tab(&mut self, ui: &mut egui::Ui, character: &Character) {

        if let Some(system) = &character.system {
            egui::ScrollArea::vertical()
                .id_salt("system_tab_scroll")  // Changed from "system_scroll"
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    // Characteristics section with better formatting
                    ui.heading("💎 Characteristics");
                    ui.separator();

                    if let Some(characteristics) = &system.characteristics {
                        TableBuilder::new(ui)
                            .id_salt("system_characteristics_table")  // Added unique ID
                            .column(Column::auto().at_least(80.0)) // Characteristic name
                            .column(Column::auto().at_least(40.0))  // Value
                            .header(20.0, |mut header| {
                                header.col(|ui| { ui.strong("Characteristic"); });
                                header.col(|ui| { ui.strong("Value"); });
                            })
                            .body(|mut body| {
                                let char_list = [
                                    ("Mut", &characteristics.mu),
                                    ("Klugheit", &characteristics.kl),
                                    ("Intuition", &characteristics.in_),
                                    ("Charisma", &characteristics.ch),
                                    ("Fingerfertigkeit", &characteristics.ff),
                                    ("Gewandtheit", &characteristics.ge),
                                    ("Konstitution", &characteristics.ko),
                                    ("Körperkraft", &characteristics.kk),
                                ];

                                for (name, char_opt) in char_list {
                                    if let Some(char_value) = char_opt {
                                        body.row(18.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label(format!("{}:", name));
                                            });
                                            row.col(|ui| {
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(&char_value.nominal_value().to_string())
                                                        .strong()
                                                        .color(egui::Color32::from_rgb(70, 130, 180))
                                                ));
                                            });
                                        });
                                    }
                                }
                            });
                    } else {
                        ui.colored_label(egui::Color32::from_rgb(180, 100, 100), "⚠ No characteristics found");
                    }

                    // Status Values section with consistent table approach
                    ui.add_space(20.0);
                    ui.heading("🔋 Status Values");
                    ui.separator();

                    if let Some(status) = &system.status {
                        TableBuilder::new(ui)
                            .id_salt("system_status_values_table")  // Added unique ID
                            .column(Column::auto().at_least(120.0)) // Status type
                            .column(Column::auto().at_least(80.0))  // Property
                            .column(Column::auto().at_least(60.0))  // Value
                            .header(20.0, |mut header| {
                                header.col(|ui| { ui.strong("Status"); });
                                header.col(|ui| { ui.strong("Property"); });
                                header.col(|ui| { ui.strong("Value"); });
                            })
                            .body(|mut body| {
                                // Life Points (LeP)
                                if let Some(wounds) = &status.wounds {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new("💚 Life Points (LeP)")
                                                    .strong()
                                            ));
                                        });
                                        row.col(|ui| {
                                            ui.label("Base Value:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&wounds.base_value().to_string())
                                                    .strong()
                                                    .color(egui::Color32::from_rgb(50, 150, 50))
                                            ));
                                        });
                                    });

                                    body.row(18.0, |mut row| {
                                        row.col(|_ui| {}); // Empty first column
                                        row.col(|ui| {
                                            ui.label("Advances:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&wounds.advances().to_string())
                                                    .color(egui::Color32::from_rgb(50, 150, 50))
                                            ));
                                        });
                                    });

                                    body.row(18.0, |mut row| {
                                        row.col(|_ui| {}); // Empty first column
                                        row.col(|ui| {
                                            ui.label("Modifier:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&wounds.modifier.to_string())
                                                    .color(if wounds.modifier > 0 {
                                                        egui::Color32::from_rgb(50, 150, 50)
                                                    } else if wounds.modifier < 0 {
                                                        egui::Color32::from_rgb(200, 80, 80)
                                                    } else {
                                                        egui::Color32::GRAY
                                                    })
                                            ));
                                        });
                                    });
                                }

                                // Astral Energy (ASP)
                                if let Some(astral_energy) = &status.astralenergy {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new("⚡ Astral Energy (ASP)")
                                                    .strong()
                                            ));
                                        });
                                        row.col(|ui| {
                                            ui.label("Base Value:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&astral_energy.base_value().to_string())
                                                    .strong()
                                                    .color(egui::Color32::from_rgb(100, 149, 237))
                                            ));
                                        });
                                    });

                                    body.row(18.0, |mut row| {
                                        row.col(|_ui| {}); // Empty first column
                                        row.col(|ui| {
                                            ui.label("Advances:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&astral_energy.advances().to_string())
                                                    .color(egui::Color32::from_rgb(50, 150, 50))
                                            ));
                                        });
                                    });

                                    body.row(18.0, |mut row| {
                                        row.col(|_ui| {}); // Empty first column
                                        row.col(|ui| {
                                            ui.label("Permanent Loss:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&astral_energy.permanent_loss().to_string())
                                                    .color(if astral_energy.permanent_loss() > 0 {
                                                        egui::Color32::from_rgb(200, 80, 80)
                                                    } else {
                                                        egui::Color32::GRAY
                                                    })
                                            ));
                                        });
                                    });

                                    body.row(18.0, |mut row| {
                                        row.col(|_ui| {}); // Empty first column
                                        row.col(|ui| {
                                            ui.label("Rebuy Points:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&astral_energy.rebuy_points().to_string())
                                                    .color(if astral_energy.rebuy_points() > 0 {
                                                        egui::Color32::from_rgb(200, 150, 50)
                                                    } else {
                                                        egui::Color32::GRAY
                                                    })
                                            ));
                                        });
                                    });
                                }

                                // Karma Energy (KaP)
                                if let Some(karma_energy) = &status.karmaenergy {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new("🕯 Karma Energy (KaP)")
                                                    .strong()
                                            ));
                                        });
                                        row.col(|ui| {
                                            ui.label("Base Value:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&karma_energy.base_value().to_string())
                                                    .strong()
                                                    .color(egui::Color32::from_rgb(100, 149, 237))
                                            ));
                                        });
                                    });

                                    body.row(18.0, |mut row| {
                                        row.col(|_ui| {}); // Empty first column
                                        row.col(|ui| {
                                            ui.label("Advances:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&karma_energy.advances().to_string())
                                                    .color(egui::Color32::from_rgb(50, 150, 50))
                                            ));
                                        });
                                    });

                                    body.row(18.0, |mut row| {
                                        row.col(|_ui| {}); // Empty first column
                                        row.col(|ui| {
                                            ui.label("Permanent Loss:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&karma_energy.permanent_loss().to_string())
                                                    .color(if karma_energy.permanent_loss() > 0 {
                                                        egui::Color32::from_rgb(200, 80, 80)
                                                    } else {
                                                        egui::Color32::GRAY
                                                    })
                                            ));
                                        });
                                    });

                                    body.row(18.0, |mut row| {
                                        row.col(|_ui| {}); // Empty first column
                                        row.col(|ui| {
                                            ui.label("Rebuy Points:");
                                        });
                                        row.col(|ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&karma_energy.rebuy_points().to_string())
                                                    .color(if karma_energy.rebuy_points() > 0 {
                                                        egui::Color32::from_rgb(200, 150, 50)
                                                    } else {
                                                        egui::Color32::GRAY
                                                    })
                                            ));
                                        });
                                    });
                                }
                            });
                    } else {
                        ui.colored_label(egui::Color32::from_rgb(180, 100, 100), "⚠ No status values found");
                    }

                });
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.colored_label(egui::Color32::from_rgb(150, 150, 150), "📋 No system data available for this character");
            });
        }
    }

    fn show_skills_tab(&mut self, ui: &mut egui::Ui, character: &Character) {
        ui.heading("🎯 Skills");
        ui.separator();

        egui::ScrollArea::vertical()
            .id_salt("skills_scroll")
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                use egui_extras::{TableBuilder, Column};

                TableBuilder::new(ui)
                    .column(Column::auto().at_least(120.0)) // Skill name
                    .column(Column::auto().at_least(90.0))  // Characteristics
                    .column(Column::auto().at_least(40.0))  // StF
                    .column(Column::auto().at_least(40.0))  // Talent Value
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Skill");
                        });
                        header.col(|ui| {
                            ui.strong("Characteristics");
                        });
                        header.col(|ui| {
                            ui.strong("StF");
                        });
                        header.col(|ui| {
                            ui.strong("Value");
                        });
                    })
                    .body(|mut body| {
                        for skill in character.get_skills() {
                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(&skill.name);
                                });
                                row.col(|ui| {
                                    if let Some(characteristics) = skill.system.get_characteristic_values() {
                                        ui.label(format!("{}/{}/{}",
                                                         characteristics.0.to_uppercase(),
                                                         characteristics.1.to_uppercase(),
                                                         characteristics.2.to_uppercase()
                                        ));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(st_f) = skill.system.get_st_f_value() {
                                        ui.label(st_f);
                                    } else {
                                        ui.label("-");
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(talent_value) = skill.system.get_talent_value() {
                                        ui.label(talent_value.to_string());
                                    } else {
                                        ui.label("0");
                                    }
                                });
                            });
                        }
                    });
            });
    }

    fn show_combat_skills_tab(&mut self, ui: &mut egui::Ui, character: &Character) {
        ui.heading("⚔ Combat Skills");
        ui.separator();

        egui::ScrollArea::vertical()
            .id_salt("combat_skills_scroll")
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                use egui_extras::{TableBuilder, Column};

                TableBuilder::new(ui)
                    .column(Column::auto().at_least(120.0)) // Combat skill name
                    .column(Column::auto().at_least(40.0))  // Guidevalue
                    .column(Column::auto().at_least(40.0))  // StF
                    .column(Column::auto().at_least(40.0))  // Talent Value
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Combat Skill");
                        });
                        header.col(|ui| {
                            ui.strong("Guide");
                        });
                        header.col(|ui| {
                            ui.strong("StF");
                        });
                        header.col(|ui| {
                            ui.strong("Value");
                        });
                    })
                    .body(|mut body| {
                        for combat_skill in character.get_combat_skills() {
                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(&combat_skill.name);
                                });
                                row.col(|ui| {
                                    if let Some(guidevalue) = combat_skill.system.get_guidevalue_value() {
                                        ui.label(guidevalue.to_uppercase());
                                    } else {
                                        ui.label("-");
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(st_f) = combat_skill.system.get_st_f_value() {
                                        ui.label(st_f);
                                    } else {
                                        ui.label("-");
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(talent_value) = combat_skill.system.get_talent_value() {
                                        ui.label(talent_value.to_string());
                                    } else {
                                        ui.label("0");
                                    }
                                });
                            });
                        }
                    });
            });
    }

    // show_magic_tab code is a duplicate of most of show_skills_tab. Can we do better?
    fn show_magic_tab(&mut self, ui: &mut egui::Ui, character: &Character) {
        ui.heading("✨ Spells & Rituals");
        ui.separator();

        // First ScrollArea for spells/rituals table
        egui::ScrollArea::vertical()
            .id_salt("spells_scroll")
            .auto_shrink([false; 2])
            .max_height(ui.available_height() * 0.6) // Use 60% of available height
            .show(ui, |ui| {
                use egui_extras::{TableBuilder, Column};

                TableBuilder::new(ui)
                    .column(Column::auto().at_least(120.0)) // Spell/Ritual name
                    .column(Column::auto().at_least(90.0))  // Characteristics
                    .column(Column::auto().at_least(40.0))  // StF
                    .column(Column::auto().at_least(40.0))  // Talent Value
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Spell/Ritual");
                        });
                        header.col(|ui| {
                            ui.strong("Characteristics");
                        });
                        header.col(|ui| {
                            ui.strong("StF");
                        });
                        header.col(|ui| {
                            ui.strong("Value");
                        });
                    })
                    .body(|mut body| {
                        for spell in character.get_spells_and_rituals() {
                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(&spell.name);
                                });
                                row.col(|ui| {
                                    if let Some(characteristics) = spell.system.get_characteristic_values() {
                                        ui.label(format!("{}/{}/{}",
                                                         characteristics.0.to_uppercase(),
                                                         characteristics.1.to_uppercase(),
                                                         characteristics.2.to_uppercase()
                                        ));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(st_f) = spell.system.get_st_f_value() {
                                        ui.label(st_f);
                                    } else {
                                        ui.label("-");
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(talent_value) = spell.system.get_talent_value() {
                                        ui.label(talent_value.to_string());
                                    } else {
                                        ui.label("0");
                                    }
                                });
                            });
                        }
                    });
            });

        // Magic tricks section outside the first scroll area
        ui.add_space(20.0);
        ui.heading("🎪 Magic Tricks");
        ui.separator();

        // Second ScrollArea for magic tricks
        egui::ScrollArea::vertical()
            .id_salt("magictricks_scroll")
            .auto_shrink([false; 2])
            .max_height(ui.available_height()) // Use remaining height
            .show(ui, |ui| {
                let magic_tricks = character.get_magic_tricks();
                if magic_tricks.is_empty() {
                    ui.label("No magic tricks found.");
                } else {
                    for item in magic_tricks {
                        ui.horizontal(|ui| {
                            ui.label(&item.name);
                        });
                    }
                }
            });
    }

    // show_magic_tab code is a duplicate of show_magic_tab. Can we do better?
    fn show_karma_tab(&mut self, ui: &mut egui::Ui, character: &Character) {
        ui.heading("🕯 Liturgies & Ceremonies");
        ui.separator();

        // First ScrollArea for spells/rituals table
        egui::ScrollArea::vertical()
            .id_salt("liturgies_ceremonies_scroll")
            .auto_shrink([false; 2])
            .max_height(ui.available_height() * 0.6) // Use 60% of available height
            .show(ui, |ui| {
                use egui_extras::{TableBuilder, Column};

                TableBuilder::new(ui)
                    .column(Column::auto().at_least(120.0)) // Liturgy/Ceremony name
                    .column(Column::auto().at_least(90.0))  // Characteristics
                    .column(Column::auto().at_least(40.0))  // StF
                    .column(Column::auto().at_least(40.0))  // Talent Value
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Liturgy/Ceremony");
                        });
                        header.col(|ui| {
                            ui.strong("Characteristics");
                        });
                        header.col(|ui| {
                            ui.strong("StF");
                        });
                        header.col(|ui| {
                            ui.strong("Value");
                        });
                    })
                    .body(|mut body| {
                        for item in character.get_liturgies_and_ceremonies() {
                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(&item.name);
                                });
                                row.col(|ui| {
                                    if let Some(characteristics) = item.system.get_characteristic_values() {
                                        ui.label(format!("{}/{}/{}",
                                                         characteristics.0.to_uppercase(),
                                                         characteristics.1.to_uppercase(),
                                                         characteristics.2.to_uppercase()
                                        ));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(st_f) = item.system.get_st_f_value() {
                                        ui.label(st_f);
                                    } else {
                                        ui.label("-");
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(talent_value) = item.system.get_talent_value() {
                                        ui.label(talent_value.to_string());
                                    } else {
                                        ui.label("0");
                                    }
                                });
                            });
                        }
                    });
            });

        // Blessings section outside the first scroll area
        ui.add_space(20.0);
        ui.heading("📚 Blessings");
        ui.separator();

        // Second ScrollArea for blessings
        egui::ScrollArea::vertical()
            .id_salt("blessings_scroll")
            .auto_shrink([false; 2])
            .max_height(ui.available_height()) // Use remaining height
            .show(ui, |ui| {
                let item = character.get_blessings();
                if item.is_empty() {
                    ui.label("No magic tricks found.");
                } else {
                    for item in item {
                        ui.horizontal(|ui| {
                            ui.label(&item.name);
                        });
                    }
                }
            });
    }


    fn show_items_tab(&mut self, ui: &mut egui::Ui, character: &Character) {
        ui.heading("🎒 Items");
        ui.separator();

        egui::ScrollArea::vertical()
            .id_salt("items_scroll")
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for item in &character.items {
                    ui.horizontal(|ui| {
                        ui.label(&item.name);
                        ui.label(format!("[{}]", item.item_type));
                        if let Some(quantity) = item.system.get_quantity() {
                            ui.label(format!("({})", quantity));
                        }
                    });
                }
            });
    }


    fn show_character_image(&mut self, ui: &mut egui::Ui, character: &Character) {
        if let Some(image_url) = character.get_image_url() {
            // Check if image is already cached
            if let Some(texture) = self.image_cache.get(image_url) {
                // Calculate aspect-ratio preserving size
                let image_size = self.calculate_display_size(texture, 120.0);
                ui.horizontal(|ui| {
                    ui.add_space(10.0); // Center the image a bit
                    ui.image((texture.id(), image_size));
                });
            } else {
                // Try to load the image
                match self.load_image_from_url(ui.ctx(), image_url) {
                    Ok(texture) => {
                        let image_size = self.calculate_display_size(&texture, 120.0);
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.image((texture.id(), image_size));
                        });
                        // Cache the loaded image
                        self.image_cache.insert(image_url.to_string(), texture);
                    }
                    Err(_) => {
                        // Show placeholder or error message
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label("🖼️ Character Image (Loading failed)");
                        });
                    }
                }
            }
        }
    }

    fn calculate_display_size(&self, texture: &egui::TextureHandle, target_width: f32) -> egui::Vec2 {
        let original_size = texture.size_vec2();
        let aspect_ratio = original_size.y / original_size.x; // height / width
        let display_height = target_width * aspect_ratio;
        egui::Vec2::new(target_width, display_height)
    }

    fn load_image_from_url(&self, ctx: &egui::Context, url: &str) -> Result<egui::TextureHandle, Box<dyn std::error::Error>> {
        // Download image data
        let response = reqwest::blocking::get(url)?;
        let bytes = response.bytes()?;
        
        // Load image using the image crate
        let image = image::load_from_memory(&bytes)?;
        let rgba_image = image.to_rgba8();
        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
        
        // Create egui texture
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba_image);
        let texture = ctx.load_texture(
            "character_image",
            color_image,
            egui::TextureOptions::default()
        );
        
        Ok(texture)
    }
}