use crate::character::Character;
use crate::character::ApCalculator;
use eframe::egui;
use egui::Ui;
use std::f32::consts::PI;

pub struct ApAnalysis;

// Pie chart slice data
#[derive(Debug, Clone)]
struct PieSlice {
    label: String,
    value: i32,
    percentage: f32,
    color: egui::Color32,
    start_angle: f32,
    end_angle: f32,
}

/// Utility function: Normalize to [0, 2π].
fn normalize_angle(angle: f32) -> f32 {
    let normalized = angle % (2.0 * PI);
    if normalized < 0.0 { normalized + 2.0 * PI } else { normalized }
}

impl ApAnalysis {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, character: &Character) {
        ui.heading("🔮 AP Analysis");
        ui.separator();

        // Totally spent AP section
        self.show_total_spent(ui, character);

        ui.add_space(20.0);

        // AP by category section
        self.show_ap_by_category(ui, character);

        // Collapsable detailed AP items section
        self.show_ap_items_details(ui, character);

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

    fn show_total_spent(&self, ui: &mut Ui, character: &Character) {
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
    }

    fn show_ap_by_category(&self, ui: &mut Ui, character: &Character) {
        ui.heading("📊 AP by Category");
        ui.separator();

        let ap_by_category = ApCalculator::get_ap_by_category(character);

        if ap_by_category.is_empty() {
            ui.label("No AP items found.");
            return;
        }

        // Create horizontal layout for pie chart and legend/details
        ui.horizontal(|ui| {
            // Left side: Pie chart
            ui.vertical(|ui| {
                let pie_slices = self.prepare_pie_data(&ap_by_category);
                self.draw_pie_chart(ui, &pie_slices, 120.0);
            });

            ui.add_space(20.0);

            // Right side: Legend and detailed breakdown
            ui.vertical(|ui| {
                ui.heading("📋 Breakdown");
                ui.separator();

                let mut sorted_categories: Vec<_> = ap_by_category.iter().collect();
                // Sort by AP value descending, then by category name ascending for stable ordering
                sorted_categories.sort_by(|a, b| {
                    match b.1.cmp(a.1) {
                        std::cmp::Ordering::Equal => a.0.cmp(b.0), // Secondary sort by name
                        other => other,
                    }
                });

                let total_ap: i32 = sorted_categories.iter().map(|(_, &ap)| ap).sum();

                for (category, &ap_value) in sorted_categories {
                    let percentage = if total_ap > 0 {
                        (ap_value as f32 / total_ap as f32) * 100.0
                    } else {
                        0.0
                    };

                    ui.horizontal(|ui| {
                        // Color indicator (small rectangle)
                        let color = self.get_category_color(category, &ap_by_category);
                        let rect = egui::Rect::from_min_size(
                            ui.cursor().min,
                            egui::Vec2::new(12.0, 12.0)
                        );
                        ui.painter().rect_filled(rect, 2.0, color);
                        ui.add_space(16.0);

                        ui.label(format!("{}:", category));
                        ui.label(format!("{} AP ({:.1}%)", ap_value, percentage));
                    });
                }
            });
        });
    }

    fn prepare_pie_data(&self, ap_by_category: &std::collections::HashMap<String, i32>) -> Vec<PieSlice> {
        let mut sorted_categories: Vec<_> = ap_by_category.iter().collect();
        sorted_categories.sort_by(|a, b| b.1.cmp(a.1)); // Sort by value descending

        let total_ap: i32 = sorted_categories.iter().map(|(_, &ap)| ap).sum();
        if total_ap == 0 {
            return Vec::new();
        }

        let mut slices = Vec::new();
        let mut current_angle = PI / 2.0;   // Start at top (12 o'clock)

        // Take top 3 categories
        let top_categories = sorted_categories.iter().take(3);

        for (i, (category, &ap_value)) in top_categories.enumerate() {
            let percentage = ap_value as f32 / total_ap as f32;
            let angle_size = percentage * 2.0 * PI;

            slices.push(PieSlice {
                label: category.to_string(),
                value: ap_value,
                percentage: percentage * 100.0,
                color: self.get_slice_color(i),
                start_angle: current_angle,
                end_angle: current_angle + angle_size,
            });

            current_angle += angle_size;
        }

        // Calculate remaining AP for "Rest" slice
        let remaining_ap = sorted_categories.iter().skip(3).map(|(_, &ap)| ap).sum();

        if remaining_ap > 0 {
            let percentage = remaining_ap as f32 / total_ap as f32;
            let angle_size = percentage * 2.0 * PI;

            slices.push(PieSlice {
                label: "Others".to_string(),
                value: remaining_ap,
                percentage: percentage * 100.0,
                color: self.get_slice_color(3), // Gray for "Others"
                start_angle: current_angle,
                end_angle: current_angle + angle_size,
            });
        }

        slices
    }

    fn draw_pie_chart(&self, ui: &mut egui::Ui, slices: &[PieSlice], radius: f32) {
        let desired_size = egui::Vec2::splat(radius * 2.2); // Slightly larger for labels
        let (response, painter) = ui.allocate_painter(desired_size, egui::Sense::hover());

        let center = response.rect.center();

        // Draw pie slices
        for slice in slices {
            self.draw_pie_slice(&painter, center, radius, slice);
        }

        // Add hover interactions using the modern approach
        if let Some(hover_pos) = response.hover_pos() {
            if let Some(hovered_slice) = self.get_slice_at_position(center, radius, hover_pos, slices) {
                response.on_hover_text_at_pointer(format!("{}: {} AP ({:.1}%)",
                                                          hovered_slice.label,
                                                          hovered_slice.value,
                                                          hovered_slice.percentage
                ));
            }
        }
    }

    fn draw_pie_slice(&self, painter: &egui::Painter, center: egui::Pos2, radius: f32, slice: &PieSlice) {
        let num_segments = ((slice.end_angle - slice.start_angle).abs() * radius / 2.0).max(8.0) as usize;

        let mut points = vec![center]; // Center point

        // Generate points along the arc
        for i in 0..=num_segments {
            let t = i as f32 / num_segments as f32;
            let angle = slice.start_angle + t * (slice.end_angle - slice.start_angle);
            let x = center.x + radius * angle.cos();
            let y = center.y - radius * angle.sin(); // -r*sin(a) because y-axis is flipped in screen coordinates
            points.push(egui::Pos2::new(x, y));
        }

        // Draw filled slice
        painter.add(egui::Shape::convex_polygon(
            points.clone(),
            slice.color,
            egui::Stroke::new(1.5, egui::Color32::WHITE), // White border between slices
        ));
    }

    fn get_slice_at_position<'a>(&self, center: egui::Pos2, radius: f32, pos: egui::Pos2, slices: &'a [PieSlice]) -> Option<&'a PieSlice> {
        let dx = pos.x - center.x;
        let dy = pos.y - center.y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Check if point is within circle
        if distance > radius {
            return None;
        }

        // egui's coordinate system: (0,0) is top-left, y increases downward
        let angle = (-dy).atan2(dx); // Note: -dy because y-axis is flipped in screen coordinates


        // Normalize to [0, 2π]
        let normalized_angle = normalize_angle(angle);

        // Debug: uncomment to see what's happening
        // println!("Mouse pos: ({:.1}, {:.1}), center: ({:.1}, {:.1})", pos.x, pos.y, center.x, center.y);
        // println!("Calculated angle: {:.3} ({:.1}°)", normalized_angle, normalized_angle.to_degrees());

        // Find which slice contains this angle
        for slice in slices {
            let slice_start_angle = normalize_angle(slice.start_angle);
            let slice_end_angle = normalize_angle(slice.end_angle);

            // Debug: uncomment to see slice angles
            // println!("Slice '{}': {:.3}-{:.3} ({:.1}°-{:.1}°)",
            //     slice.label, slice_start_angle, slice_end_angle,
            //     slice_start_angle.to_degrees(), slice_end_angle.to_degrees());

            if slice_start_angle <= slice_end_angle {
                if normalized_angle >= slice_start_angle && normalized_angle <= slice_end_angle {
                    return Some(slice);
                }
            } else {
                // Handle wrap-around case
                if normalized_angle >= slice_start_angle || normalized_angle <= slice_end_angle {
                    return Some(slice);
                }
            }
        }

        None
    }

    fn get_slice_color(&self, index: usize) -> egui::Color32 {
        match index {
            0 => egui::Color32::from_rgb(52, 152, 219),   // Blue
            1 => egui::Color32::from_rgb(46, 204, 113),   // Green
            2 => egui::Color32::from_rgb(231, 76, 60),    // Red
            3 => egui::Color32::from_rgb(149, 165, 166),  // Gray for "Others"
            _ => egui::Color32::from_rgb(127, 140, 141),  // Darker gray fallback
        }
    }

    fn get_category_color(&self, category: &str, ap_by_category: &std::collections::HashMap<String, i32>) -> egui::Color32 {
        let mut sorted_categories: Vec<_> = ap_by_category.iter().collect();
        sorted_categories.sort_by(|a, b| b.1.cmp(a.1));

        // Find the index of this category in the sorted list
        for (i, (cat_name, _)) in sorted_categories.iter().enumerate() {
            if *cat_name == category {
                return if i < 3 {
                    self.get_slice_color(i)
                } else {
                    self.get_slice_color(3) // "Others" color
                };
            }
        }

        self.get_slice_color(4) // Fallback
    }

    // OLD Method
    // fn show_ap_by_category(&self, ui: &mut Ui, character: &Character) {
    //     ui.heading("📊 AP by Category");
    //     ui.separator();
    //
    //     let ap_by_category = ApCalculator::get_ap_by_category(character);
    //
    //     if ap_by_category.is_empty() {
    //         ui.label("No AP items found.");
    //     } else {
    //         let mut sorted_categories: Vec<_> = ap_by_category.iter().collect();
    //         // Sort by AP value descending, then by category name ascending for stable ordering
    //         sorted_categories.sort_by(|a, b| {
    //             match b.1.cmp(a.1) {
    //                 std::cmp::Ordering::Equal => a.0.cmp(b.0), // Secondary sort by name
    //                 other => other,
    //             }
    //         });
    //
    //         for (category, ap_value) in sorted_categories {
    //             ui.horizontal(|ui| {
    //                 ui.label(format!("{}:", category));
    //                 ui.label(format!("{}", ap_value));
    //             });
    //         }
    //     }
    // }

    fn show_ap_items_details(&self, ui: &mut Ui, character: &Character) {
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