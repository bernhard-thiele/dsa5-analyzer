use crate::character::Character;
use std::collections::HashMap;

pub struct ApCalculator;

#[derive(Debug, Clone)]
pub struct ApItem {
    pub name: String,
    pub item_type: String,
    pub raw_value: String,
    pub step: Option<i32>,
    pub calculation: String,
    pub ap_cost: i32,
    pub was_excluded: bool,
}

#[derive(Debug, Clone)]
enum ApValueParseResult {
    SingleValue(i32),
    MultipleValues(Vec<i32>),
    ParseError,
}

impl ApCalculator {
    pub fn calculate_total_spent_ap(character: &Character) -> i32 {
        let processed_items = Self::apply_special_rules(character);
        let items_total: i32 = processed_items.iter()
            .filter(|item| !item.was_excluded)
            .map(|item| item.ap_cost)
            .sum();
        
        let skills_ap = Self::calculate_skills_ap(character);
        let combat_skills_ap = Self::calculate_combat_skills_ap(character);
        let spells_rituals_ap = Self::calculate_spells_and_rituals_ap(character);
        let magic_tricks_ap = Self::calculate_magic_tricks_ap(character);
        let liturgies_ceremonies_ap = Self::calculate_liturgies_and_ceremonies_ap(character);
        let blessings_ap = Self::calculate_blessings_ap(character);
        let characteristics_ap = Self::calculate_characteristics_ap(character);
        let energy_values_ap = Self::calculate_energy_values_ap(character);

        items_total + skills_ap + combat_skills_ap + spells_rituals_ap + magic_tricks_ap
            + liturgies_ceremonies_ap + blessings_ap + characteristics_ap + energy_values_ap
    }

    pub fn get_ap_by_category(character: &Character) -> HashMap<String, i32> {
        let mut categories = HashMap::new();

        let processed_items = Self::apply_special_rules(character);
        for item in processed_items.iter().filter(|item| !item.was_excluded) {
            if item.ap_cost != 0 {
                *categories.entry(item.item_type.clone()).or_insert(0) += item.ap_cost;
            }
        }

        // Add skills AP calculation
        let skills_ap = Self::calculate_skills_ap(character);
        if skills_ap > 0 {
            categories.insert("Skills".to_string(), skills_ap);
        }

        // Add combat skills AP calculation
        let combat_skills_ap = Self::calculate_combat_skills_ap(character);
        if combat_skills_ap > 0 {
            categories.insert("Combat Skills".to_string(), combat_skills_ap);
        }

        // Add spells & rituals AP calculation
        let spells_rituals_ap = Self::calculate_spells_and_rituals_ap(character);
        if spells_rituals_ap > 0 {
            categories.insert("Spells/Rituals".to_string(), spells_rituals_ap);
        }

        // Add magic tricks AP calculation
        let magic_tricks_ap = Self::calculate_magic_tricks_ap(character);
        if magic_tricks_ap > 0 {
            categories.insert("Magic Tricks".to_string(), magic_tricks_ap);
        }

        // Add liturgies & ceremonies AP calculation
        let liturgies_ceremonies_ap = Self::calculate_liturgies_and_ceremonies_ap(character);
        if liturgies_ceremonies_ap > 0 {
            categories.insert("Liturgies/Ceremonies".to_string(), liturgies_ceremonies_ap);
        }

        // Add blessings AP calculation
        let blessings_ap = Self::calculate_blessings_ap(character);
        if blessings_ap > 0 {
            categories.insert("Blessings".to_string(), blessings_ap);
        }

        let energy_values_ap = Self::calculate_energy_values_ap(character);
        if energy_values_ap > 0 {
            categories.insert("Energies (LeP/AsP/KaP)".to_string(), energy_values_ap);
        }

        // Add characteristics
        let characteristics_ap = Self::calculate_characteristics_ap(character);
        if characteristics_ap > 0 {
            categories.insert("Characteristics".to_string(), characteristics_ap);
        }

        categories
    }

    /// Apply special DSA rules for duplicate advantages/disadvantages
    fn apply_special_rules(character: &Character) -> Vec<ApItem> {
        let mut result = Vec::new();
        let mut duplicate_groups: HashMap<String, Vec<ApItem>> = HashMap::new();

        // Define which items have the "highest step only" rule
        let highest_step_only_items = [
            "Prinzipientreue",
            "Verpflichtungen",
            // Add more items that follow this rule as needed
        ];

        // Process all AP items
        for item in character.get_ap_items() {
            let raw_ap_value = item.system.get_ap_value().unwrap_or_default();
            // TODO unify step or step_option
            let step_option = item.system.get_step_value()
                .and_then(|s| s.parse::<i32>().ok());
            let step = item.system.get_step_value()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(1);
            let calculated_cost = Self::calculate_item_ap_cost(item);

            // Generate explanation based on parsing result
            let calculation_explanation = match Self::parse_ap_value(&raw_ap_value) {
                ApValueParseResult::SingleValue(value) => {
                    if step == 1 {
                        value.to_string()
                    } else {
                        format!("{} × {}", value, step)
                    }
                }
                ApValueParseResult::MultipleValues(values) => {
                    match Self::calculate_multiple_values_cost(&values, step) {
                        Ok(_) => {
                            let step_values: Vec<String> = values
                                .iter()
                                .take(step as usize)
                                .map(|v| v.to_string())
                                .collect();

                            if step_values.len() > 1 {
                                format!("Sum: {}", step_values.join(" + "))
                            } else {
                                step_values.first().unwrap_or(&"0".to_string()).clone()
                            }
                        }
                        Err(error_msg) => format!("ERROR: {}", error_msg),
                    }
                }
                ApValueParseResult::ParseError => "Parse error".to_string(),
            };

            let ap_item = ApItem {
                name: item.name.clone(),
                item_type: item.item_type.clone(),
                raw_value: raw_ap_value.clone(),
                step: step_option,
                calculation: calculation_explanation.clone(),
                ap_cost: calculated_cost,
                was_excluded: false,
            };

            // Extract the base name (part before the first opening parenthesis)
            let base_name = Self::extract_base_name(&item.name);
            
            if highest_step_only_items.contains(&base_name.as_str()) {
                duplicate_groups
                    .entry(base_name)
                    .or_insert_with(Vec::new)
                    .push(ap_item);
            } else {
                // Regular item, no special rules
                result.push(ap_item);
            }
        }

        // Process duplicate groups - only keep the one with highest step
        for (_base_name, mut group) in duplicate_groups {
            if group.len() == 1 {
                // Only one instance, use it normally
                result.push(group.into_iter().next().unwrap());
            } else {
                // Multiple instances, find the one with highest step
                group.sort_by(|a, b| {
                    let step_a = a.step.unwrap_or(1);
                    let step_b = b.step.unwrap_or(1);
                    step_b.cmp(&step_a) // Sort by step descending
                });

                // Add the highest one with its full cost
                result.push(group[0].clone());

                // Add the others as excluded (so they show up in debug but don't count)
                for item in &group[1..] {
                    let mut excluded_item = item.clone();
                    excluded_item.was_excluded = true;
                    result.push(excluded_item);
                }
            }
        }

        result
    }

    /// Extract the base name from a full name (everything before the first opening parenthesis)
    /// Example: "Prinzipientreue (Hesindekirche)" -> "Prinzipientreue"
    fn extract_base_name(full_name: &str) -> String {
        if let Some(paren_pos) = full_name.find('(') {
            full_name[..paren_pos].trim().to_string()
        } else {
            full_name.to_string()
        }
    }

    /// Parse the AP value string into a structured result
    fn parse_ap_value(ap_value_str: &str) -> ApValueParseResult {
        if ap_value_str.contains(';') {
            // Handle semicolon-separated values
            let parsed_values: Result<Vec<i32>, _> = ap_value_str
                .split(';')
                .map(|s| s.trim().parse::<i32>())
                .collect();

            match parsed_values {
                Ok(values) if !values.is_empty() => ApValueParseResult::MultipleValues(values),
                _ => ApValueParseResult::ParseError,
            }
        } else {
            // Handle single value
            match ap_value_str.parse::<i32>() {
                Ok(value) => ApValueParseResult::SingleValue(value),
                Err(_) => ApValueParseResult::ParseError,
            }
        }
    }

    /// Calculate cost for a single value times its step
    fn calculate_single_value_cost(value: i32, step: i32) -> i32 {
        value * step
    }

    /// Calculate cumulative cost for multiple values
    fn calculate_multiple_values_cost(values: &[i32], step: i32) -> Result<i32, String> {
        if step > values.len() as i32 {
            return Err(format!("Step {} exceeds available values ({})", step, values.len()));
        }

        if step < 1 {
            return Err("Step must be at least 1".to_string());
        }

        // Sum up all costs from step 1 to the current step
        Ok(values.iter().take(step as usize).sum())
    }

    /// Calculate the effective AP cost for a single item, considering both APValue and step
    fn calculate_item_ap_cost(item: &crate::character::Item) -> i32 {
        // Get AP value string
        let ap_value_str = match item.system.get_ap_value() {
            Some(val) => val,
            None => return 0,
        };

        // Get step value (default to 1)
        let step = item.system.get_step_value()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(1);

        // Parse AP value and calculate cost based on type
        match Self::parse_ap_value(&ap_value_str) {
            ApValueParseResult::SingleValue(value) => {
                Self::calculate_single_value_cost(value, step)
            }
            ApValueParseResult::MultipleValues(values) => {
                Self::calculate_multiple_values_cost(&values, step).unwrap_or_else(|_| 0)
            }
            ApValueParseResult::ParseError => 0,
        }
    }

    /// Get a detailed breakdown that shows which duplicate items were excluded
    pub fn get_ap_items_breakdown(character: &Character) -> Vec<ApItem> {
        let mut breakdown = Self::apply_special_rules(character);
        
        // Sort the breakdown to ensure a consistent order and prevent flickering
        breakdown.sort_by(|a, b| {
            // First sort by name
            match a.name.cmp(&b.name) {
                std::cmp::Ordering::Equal => {
                    // If names are equal, sort by step (higher step first)
                    let step_a = a.step.unwrap_or(1);
                    let step_b = b.step.unwrap_or(1);
                    step_b.cmp(&step_a)
                },
                other => other,
            }
        });
        
        breakdown
    }

    /// Calculate AP cost for a single talent value using the progressive cost system
    fn talent_value_to_ap_cost(talent_value: i32, stf_multiplier: i32) -> i32 {
        if talent_value < 0 {
            return 0; // Handle invalid values gracefully
        }
        
        if talent_value <= 12 {
            talent_value * stf_multiplier
        } else {
            let cost_until_12 = 12 * stf_multiplier;
            let above_12 = talent_value - 12;
            let cost_above_12 = Self::cost_above_12(above_12, stf_multiplier);
            cost_until_12 + cost_above_12
        }
    }
    
    /// Calculate the progressive cost for talent values above 12
    fn cost_above_12(above_12: i32, stf_multiplier: i32) -> i32 {
        let mut total_cost = 0;
        let mut current_increment_cost = stf_multiplier;
        
        for _ in 0..above_12 {
            current_increment_cost += stf_multiplier;
            total_cost += current_increment_cost;
        }
        
        total_cost
    }

    /// Calculate AP spent on skills using the progressive cost system
    pub fn calculate_skills_ap(character: &Character) -> i32 {
        character.get_skills()
            .iter()
            .filter_map(|skill| {
                let talent_value = skill.system.get_talent_value()
                    .and_then(|v| v.parse::<i32>().ok())?;

                let stf_multiplier = skill.system.get_st_f_value()
                    .and_then(|stf| Self::stf_to_multiplier(&stf))?;

                Some(Self::talent_value_to_ap_cost(talent_value, stf_multiplier))
            })
            .sum()
    }

    /// Calculate AP cost for combat skills (starting from base value of 6)
    fn combat_skill_talent_value_to_ap_cost(talent_value: i32, stf_multiplier: i32) -> i32 {
        const COMBAT_SKILL_BASE: i32 = 6;

        if talent_value <= COMBAT_SKILL_BASE {
            return 0; // No AP cost for values at or below base
        }

        // Calculate the total AP cost as if this were a regular talent
        let total_ap_cost = Self::talent_value_to_ap_cost(talent_value, stf_multiplier);

        // Subtract the cost of the first 6 levels (which are free for combat skills)
        let free_levels_cost = Self::talent_value_to_ap_cost(COMBAT_SKILL_BASE, stf_multiplier);

        total_ap_cost - free_levels_cost
    }

    /// Calculate AP spent on combat skills using the progressive cost system
    pub fn calculate_combat_skills_ap(character: &Character) -> i32 {
        character.get_combat_skills()
            .iter()
            .filter_map(|combat_skill| {
                let talent_value = combat_skill.system.get_talent_value()
                    .and_then(|v| v.parse::<i32>().ok())?;

                let stf_multiplier = combat_skill.system.get_st_f_value()
                    .and_then(|stf| Self::stf_to_multiplier(&stf))?;

                Some(Self::combat_skill_talent_value_to_ap_cost(talent_value, stf_multiplier))
            })
            .sum()
    }

    /// Generic method to calculate AP cost for learned abilities (spells, rituals, liturgies, ceremonies)
    /// These all follow the same pattern: learning cost + progression cost
    fn learned_ability_talent_value_to_ap_cost(talent_value: i32, stf_multiplier: i32) -> i32 {
        let learning_ap_cost = stf_multiplier;
        let progression_ap_cost = Self::talent_value_to_ap_cost(talent_value, stf_multiplier);

        learning_ap_cost + progression_ap_cost
    }


    /// Generic method to calculate AP for a collection of learned abilities (spells, rituals, liturgies, ceremonies)
    fn calculate_learned_abilities_ap(items: &[&crate::character::Item]) -> i32 {
        items
            .iter()
            .filter_map(|item| {
                let talent_value = item.system.get_talent_value()
                    .and_then(|v| v.parse::<i32>().ok())?;

                let stf_multiplier = item.system.get_st_f_value()
                    .and_then(|stf| Self::stf_to_multiplier(&stf))?;

                Some(Self::learned_ability_talent_value_to_ap_cost(talent_value, stf_multiplier))
            })
            .sum()
    }

    /// Calculate AP spent on spells and rituals using the generic method
    pub fn calculate_spells_and_rituals_ap(character: &Character) -> i32 {
        let items = character.get_spells_and_rituals();
        Self::calculate_learned_abilities_ap(&items)
    }

    /// Calculate AP spent on liturgies and ceremonies using the generic method
    pub fn calculate_liturgies_and_ceremonies_ap(character: &Character) -> i32 {
        let items = character.get_liturgies_and_ceremonies();
        Self::calculate_learned_abilities_ap(&items)
    }

    /// Calculate APs spent on magic tricks
    pub fn calculate_magic_tricks_ap(character: &Character) -> i32 {
        // Learning a magic trick costs 1 AP
        character.get_magic_tricks().len() as i32
    }

    /// Calculate APs spent on blessings
    pub fn calculate_blessings_ap(character: &Character) -> i32 {
        // Learning a magic trick costs 1 AP
        character.get_blessings().len() as i32
    }

    /// Convert StF value to multiplier: A=1, B=2, C=3, D=4
    fn stf_to_multiplier(stf: &str) -> Option<i32> {
        match stf.to_uppercase().as_str() {
            "A" => Some(1),
            "B" => Some(2),
            "C" => Some(3),
            "D" => Some(4),
            _ => None,
        }
    }

    /// Calculate AP spent on Life Points (LeP) advances using D-level talent progression
    pub fn calculate_lep_ap(character: &Character) -> i32 {
        if let Some(system) = &character.system {
            if let Some(status) = &system.status {
                if let Some(wounds) = &status.wounds {
                    let advances = wounds.advances();
                    if advances > 0 {
                        // LeP advances use D-level talent cost (multiplier 4)
                        const LEP_MULTIPLIER: i32 = 4;
                        return Self::talent_value_to_ap_cost(advances, LEP_MULTIPLIER);
                    }
                }
            }
        }
        0
    }

    /// Calculate AP spent on Astral Energy (AsP) advances using D-level talent progression
    pub fn calculate_asp_ap(character: &Character) -> i32 {
        if let Some(system) = &character.system {
            if let Some(status) = &system.status {
                if let Some(astral_energy) = &status.astralenergy {
                    let advances = astral_energy.advances();
                    let rebuy_points = astral_energy.rebuy_points();

                    let mut total_cost = 0;

                    // Calculate cost for advances using D-level talent progression
                    if advances > 0 {
                        const ASP_MULTIPLIER: i32 = 4;
                        total_cost += Self::talent_value_to_ap_cost(advances, ASP_MULTIPLIER);
                    }

                    // Rebuy points cost 2 AP each (cost to recover permanently lost AsP)
                    if rebuy_points > 0 {
                        total_cost += rebuy_points * 2;
                    }

                    return total_cost;
                }
            }
        }
        0
    }

    /// Calculate AP spent on Karma Energy (KaP) advances using D-level talent progression
    pub fn calculate_kap_ap(character: &Character) -> i32 {
        if let Some(system) = &character.system {
            if let Some(status) = &system.status {
                if let Some(karma_energy) = &status.karmaenergy {
                    let advances = karma_energy.advances();
                    let rebuy_points = karma_energy.rebuy_points();

                    let mut total_cost = 0;

                    // Calculate cost for advances using D-level talent progression
                    if advances > 0 {
                        const KAP_MULTIPLIER: i32 = 4;
                        total_cost += Self::talent_value_to_ap_cost(advances, KAP_MULTIPLIER);
                    }

                    // Rebuy points cost 2 AP each (cost to recover permanently lost KaP)
                    if rebuy_points > 0 {
                        total_cost += rebuy_points * 2;
                    }

                    return total_cost;
                }
            }
        }
        0
    }

    // Calculate AP spent on energy values (LeP/AsP/KaP)
    pub fn calculate_energy_values_ap(character: &Character) -> i32 {
        Self::calculate_lep_ap(character) + Self::calculate_asp_ap(character) + Self::calculate_kap_ap(character)
    }

    /// Calculate AP cost for a characteristic value (possibly non-humans need more sophistication?)
    fn characteristic_to_ap_cost(value: i32) -> Result<i32, &'static str> {
        if value < 8 {
            return Err("Value must be greater or equal 8");
        } else if value <= 14 {
            return Ok((value - 8) * 15);
        } else {
            let cost_until_14 = 6 * 15; // Cost for values 8-14
            let above_14 = value - 14;
            let mut cost_above_14 = 0;

            for i in 1..=above_14 {
                cost_above_14 += 15 + (i * 15);
            }

            let cost = cost_until_14 + cost_above_14;
            Ok(cost)
        }
    }


    /// Calculate total AP spent on characteristics
    pub fn calculate_characteristics_ap(character: &Character) -> i32 {
        if let Some(system) = &character.system {
            if let Some(characteristics) = &system.characteristics {
                let mut total_cost = 0;

                // List of all characteristics to check
                let char_list = [
                    &characteristics.mu,
                    &characteristics.kl,
                    &characteristics.in_,
                    &characteristics.ch,
                    &characteristics.ff,
                    &characteristics.ge,
                    &characteristics.ko,
                    &characteristics.kk,
                ];

                for char_opt in char_list {
                    if let Some(char_value) = char_opt {
                        let nominal_value = char_value.nominal_value();
                        if let Ok(ap_cost) = Self::characteristic_to_ap_cost(nominal_value) {
                            total_cost += ap_cost;
                        }
                    }
                }

                return total_cost;
            }
        }
        0
    }

    /// Get breakdown of characteristics AP costs for debugging
    pub fn get_characteristics_ap_breakdown(character: &Character) -> Vec<(String, i32, i32)> {
        let mut breakdown = Vec::new();

        if let Some(system) = &character.system {
            if let Some(characteristics) = &system.characteristics {
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
                        let nominal_value = char_value.nominal_value();
                        if let Ok(ap_cost) = Self::characteristic_to_ap_cost(nominal_value) {
                            breakdown.push((name.to_string(), nominal_value, ap_cost));
                        }
                    }
                }
            }
        }

        breakdown
    }

    /// Get detailed breakdown of skills AP costs
    pub fn get_skills_ap_breakdown(character: &Character) -> Vec<(String, i32, String, i32)> {
        character.get_skills()
            .iter()
            .filter_map(|skill| {
                let talent_value = skill.system.get_talent_value()
                    .and_then(|v| v.parse::<i32>().ok())?;

                let stf_value = skill.system.get_st_f_value()?;
                let stf_multiplier = Self::stf_to_multiplier(&stf_value)?;
                let ap_cost = Self::talent_value_to_ap_cost(talent_value, stf_multiplier);

                // Only include skills that actually cost AP
                if ap_cost > 0 {
                    Some((skill.name.clone(), talent_value, stf_value, ap_cost))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get detailed breakdown of combat skills AP costs (for debugging)
    pub fn get_combat_skills_ap_breakdown(character: &Character) -> Vec<(String, i32, String, i32)> {
        character.get_combat_skills()
            .iter()
            .filter_map(|combat_skill| {
                let talent_value = combat_skill.system.get_talent_value()
                    .and_then(|v| v.parse::<i32>().ok())?;

                let stf = combat_skill.system.get_st_f_value()?;
                let stf_multiplier = Self::stf_to_multiplier(&stf)?;
                let ap_cost = Self::combat_skill_talent_value_to_ap_cost(talent_value, stf_multiplier);

                Some((combat_skill.name.clone(), talent_value, stf, ap_cost))
            })
            .collect()
    }

    /// Generic method to get breakdown for learned abilities (spells, rituals, liturgies, ceremonies)
    /// Returns: (name, talent_value, stf_multiplier, ap_cost)
    fn get_learned_abilities_ap_breakdown(items: &[&crate::character::Item]) -> Vec<(String, i32, String, i32)> {
        items
            .iter()
            .filter_map(|item| {
                let talent_value = item.system.get_talent_value()
                    .and_then(|v| v.parse::<i32>().ok())?;

                let stf = item.system.get_st_f_value()?;
                let stf_multiplier = Self::stf_to_multiplier(&stf)?;
                let ap_cost = Self::learned_ability_talent_value_to_ap_cost(talent_value, stf_multiplier);

                Some((item.name.clone(), talent_value, stf, ap_cost))
            })
            .collect()
    }

    /// Get detailed breakdown for spells and rituals AP calculation
    /// Returns: (name, talent_value, stf_multiplier, ap_cost)
    pub fn get_spells_and_rituals_ap_breakdown(character: &Character) -> Vec<(String, i32, String, i32)> {
        let items = character.get_spells_and_rituals();
        Self::get_learned_abilities_ap_breakdown(&items)
    }

    /// Get detailed breakdown for liturgies and ceremonies AP calculation
    /// Returns: (name, talent_value, stf_multiplier, ap_cost)
    pub fn get_liturgies_and_ceremonies_ap_breakdown(character: &Character) -> Vec<(String, i32, String, i32)> {
        let items = character.get_liturgies_and_ceremonies();
        Self::get_learned_abilities_ap_breakdown(&items)
    }

}