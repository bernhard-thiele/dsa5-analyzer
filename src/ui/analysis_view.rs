use crate::character::Character;
use crate::character::ApCalculator;
use eframe::egui;
use egui::Ui;

pub struct ApAnalysis;

impl ApAnalysis {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, character: &Character) {
        ui.heading("🔮 AP Analysis");
        ui.separator();

        let calculated_total_spent_ap = ApCalculator::calculate_total_spent_ap(character);

        let foundry_spent_ap = character.system
            .as_ref()
            .and_then(|s| s.details.as_ref())
            .and_then(|d| d.experience.as_ref())
            .map(|e| e.spent())
            .unwrap_or(0);

        let difference = calculated_total_spent_ap - foundry_spent_ap;

        // AP Comparison Section - Prominent display
        ui.add_space(10.0);

        // Create a visually prominent frame for the comparison
        egui::Frame::NONE
            .fill(ui.style().visuals.faint_bg_color)
            .stroke(egui::Stroke::new(2.0, ui.style().visuals.text_color()))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.heading("🔍 AP Comparison");
                ui.separator();

                egui::Grid::new("ap_comparison")
                    .num_columns(3)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        // Header row
                        ui.strong("Source");
                        ui.strong("Spent AP");
                        ui.strong("Difference");
                        ui.end_row();

                        // Our calculation
                        ui.label("✏ Our Calculation:");
                        ui.strong(format!("{} AP", calculated_total_spent_ap));
                        ui.label("—");
                        ui.end_row();

                        // Foundry VTT calculation
                        ui.label("🎮 Foundry VTT:");
                        ui.label(format!("{} AP", foundry_spent_ap));

                        // Display difference with color coding
                        if difference == 0 {
                            ui.colored_label(egui::Color32::DARK_GREEN, "✅ Perfect match!");
                        } else if difference > 0 {
                            ui.colored_label(egui::Color32::DARK_RED, format!("⚠ Foundry shows {} AP less", difference));
                        } else {
                            ui.colored_label(egui::Color32::DARK_RED, format!("⚠ Foundry shows {} AP more", difference));
                        }
                        ui.end_row();
                    });

                // Add explanation if there's a significant difference
                if difference.abs() > 0 {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.small(if difference > 0 {
                        "💡 Our calculation includes more AP costs than Foundry VTT recorded"
                    } else {
                        "💡 Foundry VTT shows higher AP costs than our calculation found"
                    });
                }
            });

        ui.add_space(20.0);

        // AP by category section
        ui.heading("📊 AP by Category");
        ui.separator();

        let ap_by_category = ApCalculator::get_ap_by_category(character);

        if ap_by_category.is_empty() {
            ui.label("No AP items found.");
        } else {
            let mut sorted_categories: Vec<_> = ap_by_category.iter().collect();
            // Sort by AP value descending, then by category name ascending for stable ordering
            sorted_categories.sort_by(|a, b| {
                match b.1.cmp(a.1) {
                    std::cmp::Ordering::Equal => a.0.cmp(b.0), // Secondary sort by name
                    other => other,
                }
            });

            for (category, ap_value) in sorted_categories {
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", category));
                    ui.label(format!("{}", ap_value));
                });
            }
        }

        // Detailed AP items section
        ui.add_space(15.0);
        ui.collapsing("📝 AP Items Details", |ui| {

            let ap_items = ApCalculator::get_ap_items_breakdown(character);

            // Use the remaining available height for the scroll area
            let available_height = ui.available_height();
            egui::ScrollArea::vertical()
                .id_salt("ap_items_scroll")
                .max_height(available_height - 20.0) // Leave some padding
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    use egui_extras::{TableBuilder, Column};

                    if ap_items.is_empty() {
                        ui.label("No AP items found.");
                    } else {
                        TableBuilder::new(ui)
                            .column(Column::auto().at_least(140.0)) // Item name
                            .column(Column::auto().at_least(80.0))  // Raw AP value
                            .column(Column::auto().at_least(40.0))  // Step
                            .column(Column::auto().at_least(100.0)) // Calculation
                            .column(Column::auto().at_least(60.0))  // Final cost
                            .column(Column::auto().at_least(140.0)) // Comment
                            .header(20.0, |mut header| {
                                header.col(|ui| { ui.strong("Item"); });
                                header.col(|ui| { ui.strong("Raw AP Value"); });
                                header.col(|ui| { ui.strong("Step"); });
                                header.col(|ui| { ui.strong("Calculation"); });
                                header.col(|ui| { ui.strong("Final Cost"); });
                                header.col(|ui| { ui.strong("Comment"); });
                            })
                            .body(|mut body| {
                                for item in ap_items {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(&item.name);
                                        });
                                        row.col(|ui| {
                                            ui.label(&item.raw_value);
                                        });
                                        row.col(|ui| {
                                            ui.label(item.step.unwrap_or(1).to_string());
                                        });
                                        row.col(|ui| {
                                            ui.label(&item.calculation);
                                        });
                                        row.col(|ui| {
                                            if item.was_excluded {
                                                ui.colored_label(egui::Color32::GRAY, format!("({} AP - excluded)", item.ap_cost));
                                            } else {
                                                ui.label(format!("{} AP", item.ap_cost));
                                            }
                                        });
                                        row.col(|ui| {
                                            if item.was_excluded {
                                                ui.colored_label(egui::Color32::RED, "(duplicate, lower step)");
                                            }
                                            ui.label(format!("[{}]", item.item_type));
                                        });
                                    });
                                }
                            });
                    }
                });
        });

        // Add debug info
        ui.add_space(15.0);
        self.show_debug_characteristics_breakdown(ui, character);
        self.show_debug_skills_breakdown(ui, character);
        self.show_debug_combat_skills_breakdown(ui, character);
        self.show_debug_spells_rituals_breakdown(ui, character);
        self.show_debug_liturgies_ceremonies_breakdown(ui, character);
        self.show_debug_item_info(ui, character);

        ui.add_space(15.0);


    }

    fn show_debug_skills_breakdown(&self, ui: &mut Ui, character: &Character) {
        let skills_ap = ApCalculator::calculate_skills_ap(character);
        if skills_ap > 0 {
            ui.collapsing("Debug: Skills AP Breakdown", |ui| {
                let skills_breakdown = ApCalculator::get_skills_ap_breakdown(character);

                egui::ScrollArea::vertical()
                    .id_salt("skills_ap_scroll")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (name, talent_value, stf, ap_cost) in skills_breakdown {
                            ui.horizontal(|ui| {
                                ui.label(&name);
                                ui.label(format!("({}×{} = {})", talent_value, stf, ap_cost));
                            });
                        }
                    });
            });
        }
    }

    fn show_debug_combat_skills_breakdown(&self, ui: &mut Ui, character: &Character) {
        let combat_skills_ap = ApCalculator::calculate_combat_skills_ap(character);
        if combat_skills_ap > 0 {
            ui.collapsing("Debug: Combat Skills AP Breakdown", |ui| {
                let combat_skills_breakdown = ApCalculator::get_combat_skills_ap_breakdown(character);

                egui::ScrollArea::vertical()
                    .id_salt("combat_skills_ap_scroll")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (name, talent_value, stf, ap_cost) in combat_skills_breakdown {
                            ui.horizontal(|ui| {
                                ui.label(&name);
                                ui.label(format!("({}×{} = {})", talent_value, stf, ap_cost));
                            });
                        }
                    });
            });
        }
    }

    fn show_debug_characteristics_breakdown(&self, ui: &mut Ui, character: &Character) {
        let characteristics_ap = ApCalculator::calculate_characteristics_ap(character);
        if characteristics_ap > 0 {
            ui.collapsing("Debug: Characteristics AP Breakdown", |ui| {
                let characteristics_breakdown = ApCalculator::get_characteristics_ap_breakdown(character);

                egui::ScrollArea::vertical()
                    .id_salt("characteristics_ap_scroll")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (name, value, ap_cost) in characteristics_breakdown {
                            ui.horizontal(|ui| {
                                ui.label(&name);
                                ui.label(format!("(Value: {}, Cost: {} AP)", value, ap_cost));
                            });
                        }
                    });
            });
        }
    }
    
    fn show_debug_spells_rituals_breakdown(&self, ui: &mut Ui, character: &Character) {
        let spells_rituals_ap = ApCalculator::calculate_spells_and_rituals_ap(character);
        if spells_rituals_ap > 0 {
            ui.collapsing("Debug: Spells & Rituals AP Breakdown", |ui| {
                let spells_rituals_breakdown = ApCalculator::get_spells_and_rituals_ap_breakdown(character);

                egui::ScrollArea::vertical()
                    .id_salt("spells_rituals_ap_scroll")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (name, talent_value, stf, ap_cost) in spells_rituals_breakdown {
                            ui.horizontal(|ui| {
                                ui.label(&name);
                                ui.label(format!("({}×{} = {} AP)", talent_value, stf, ap_cost));
                            });
                        }
                    });
            });
        }
    }

    fn show_debug_liturgies_ceremonies_breakdown(&self, ui: &mut Ui, character: &Character) {
        let liturgies_ceremonies_ap = ApCalculator::calculate_liturgies_and_ceremonies_ap(character);
        if liturgies_ceremonies_ap > 0 {
            ui.collapsing("Debug: Liturgies & Ceremonies AP Breakdown", |ui| {
                let liturgies_ceremonies_breakdown = ApCalculator::get_liturgies_and_ceremonies_ap_breakdown(character);

                egui::ScrollArea::vertical()
                    .id_salt("liturgies_ceremonies_ap_scroll")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (name, talent_value, stf, ap_cost) in liturgies_ceremonies_breakdown {
                            ui.horizontal(|ui| {
                                ui.label(&name);
                                ui.label(format!("({}×{} = {} AP)", talent_value, stf, ap_cost));
                            });
                        }
                    });
            });
        }
    }


    fn show_debug_item_info(&self, ui: &mut egui::Ui, character: &Character) {
        ui.collapsing("Debug: Item Type Analysis", |ui| {
            // Count all item types
            let mut all_types = std::collections::HashMap::new();
            let mut types_with_ap = std::collections::HashMap::new();

            for item in &character.items {
                *all_types.entry(item.item_type.clone()).or_insert(0) += 1;

                if item.system.get_ap_value().is_some() {
                    *types_with_ap.entry(item.item_type.clone()).or_insert(0) += 1;
                }
            }

            ui.label("All item types in character:");

            // Sort the types to prevent flickering
            let mut sorted_types: Vec<_> = all_types.iter().collect();
            sorted_types.sort_by(|a, b| a.0.cmp(b.0));

            for (item_type, count) in sorted_types {
                let has_ap = types_with_ap.contains_key(item_type);
                ui.horizontal(|ui| {
                    ui.label(format!("  {}: {} items", item_type, count));
                    if has_ap {
                        ui.label("(has AP values)");
                    } else {
                        ui.label("(no AP values)");
                    }
                });
            }
        });
    }

}