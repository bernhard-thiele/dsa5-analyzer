use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    #[serde(rename = "type")]
    pub character_type: String,
    pub img: Option<String>,
    pub items: Vec<Item>,
    pub system: Option<CharacterSystem>,
    // Add other top-level fields as needed
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSystem {
    pub characteristics: Option<Characteristics>,
    pub status: Option<StatusValues>,
    pub details: Option<Details>,

    // Catch remaining fields that we don't specifically handle
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Characteristics {
    pub mu: Option<CharacteristicValue>,      // Mut
    pub kl: Option<CharacteristicValue>,      // Klugheit
    #[serde(rename = "in")]
    pub in_: Option<CharacteristicValue>,     // Intuition (renamed to avoid keyword)
    pub ch: Option<CharacteristicValue>,      // Charisma
    pub ff: Option<CharacteristicValue>,      // Fingerfertigkeit
    pub ge: Option<CharacteristicValue>,      // Gewandtheit
    pub ko: Option<CharacteristicValue>,      // Konstitution
    pub kk: Option<CharacteristicValue>,      // Körperkraft
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacteristicValue {
    pub initial: i32,
    #[serde(default)]
    pub species: i32,          // Default to 0 if missing
    #[serde(default)]
    pub modifier: i32,         // Default to 0 if missing
    pub advances: i32,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl CharacteristicValue {
    /// Calculate the nominal value of the characteristic (initial + advances)
    /// This is the stable characteristic rating used for skill checks and game mechanics
    pub fn nominal_value(&self) -> i32 {
        self.initial + self.advances
    }

    /// Get the base initial value
    pub fn initial_value(&self) -> i32 {
        self.initial
    }

    /// Get the advances spent on this characteristic
    pub fn advances(&self) -> i32 {
        self.advances
    }

    /// Get species bonus (if any)
    pub fn species_bonus(&self) -> i32 {
        self.species
    }

    pub fn modifier(&self) -> i32 {
        self.modifier
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusValues {
    pub wounds: Option<WoundValue>,
    pub astralenergy: Option<AstralEnergyValue>,
    pub karmaenergy: Option<KarmaEnergyValue>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WoundValue {
    pub initial: i32,
    pub value: i32,
    pub advances: i32,
    pub modifier: i32,
    pub current: i32,
    pub max: i32,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

// TODO self.value seem to be the current value! Rethink dealing with it. Maybe "Derived Values" is an okayish group?
impl WoundValue {
    /// Get the base value (value field from JSON)
    pub fn base_value(&self) -> i32 {
        self.value
    }

    /// Get advances spent on wounds/health
    pub fn advances(&self) -> i32 {
        self.advances
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstralEnergyValue {
    pub initial: i32,
    pub value: i32,
    pub advances: i32,
    pub modifier: i32,
    pub current: i32,
    pub max: i32,
    #[serde(rename = "permanentLoss", default)]
    pub permanent_loss: i32,
    #[serde(default)]
    pub rebuy: i32,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl AstralEnergyValue {
    /// Get the base value (value field from JSON)
    pub fn base_value(&self) -> i32 {
        self.value
    }

    /// Get advances spent on astral energy
    pub fn advances(&self) -> i32 {
        self.advances
    }

    /// Get permanent loss (negative modifier to max ASP)
    pub fn permanent_loss(&self) -> i32 {
        self.permanent_loss
    }

    /// Get rebuy points spent (AP spent to regain lost ASP)
    pub fn rebuy_points(&self) -> i32 {
        self.rebuy
    }
}

// Add this after AstralEnergyValue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarmaEnergyValue {
    pub initial: i32,
    pub value: i32,
    pub advances: i32,
    pub modifier: i32,
    pub current: i32,
    pub max: i32,
    #[serde(rename = "permanentLoss", default)]
    pub permanent_loss: i32,
    #[serde(default)]
    pub rebuy: i32,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl KarmaEnergyValue {
    /// Get the base value (value field from JSON)
    pub fn base_value(&self) -> i32 {
        self.value
    }

    /// Get advances spent on karma energy
    pub fn advances(&self) -> i32 {
        self.advances
    }

    /// Get permanent loss (negative modifier to max KaP)
    pub fn permanent_loss(&self) -> i32 {
        self.permanent_loss
    }

    /// Get rebuy points spent (AP spent to regain lost KaP)
    pub fn rebuy_points(&self) -> i32 {
        self.rebuy
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Details {
    pub experience: Option<Experience>,

    // Catch remaining fields that we don't specifically handle
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub total: i32,
    pub spent: i32,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl Experience {
    /// Get total experience points received
    pub fn total(&self) -> i32 {
        self.total
    }

    /// Get experience points spent
    pub fn spent(&self) -> i32 {
        self.spent
    }
}




/**************************************************
* Item system
**************************************************/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub _id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub img: Option<String>,
    pub system: ItemSystem,
    // Store everything else as raw JSON to avoid parsing issues
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSystem {
    // Store everything as raw JSON values to avoid parsing issues
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl ItemSystem {
    pub fn get_ap_value(&self) -> Option<String> {
        self.data.get("APValue")
            .and_then(|v| v.get("value"))
            .and_then(|v| match v {
                serde_json::Value::String(s) => Some(s.clone()),
                serde_json::Value::Number(n) => Some(n.to_string()),
                _ => None,
            })
    }

    pub fn get_description(&self) -> Option<String> {
        self.data.get("description")
            .and_then(|v| v.get("value"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    pub fn get_gm_description(&self) -> Option<String> {
        self.data.get("gmdescription")
            .and_then(|v| v.get("value"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    pub fn get_value_as_string(&self, key: &str) -> Option<String> {
        self.data.get(key)
            .and_then(|v| v.get("value"))
            .and_then(|v| match v {
                serde_json::Value::String(s) => Some(s.clone()),
                serde_json::Value::Number(n) => Some(n.to_string()),
                serde_json::Value::Bool(b) => Some(b.to_string()),
                _ => None,
            })
    }

    pub fn get_price(&self) -> Option<String> {
        self.get_value_as_string("price")
    }

    pub fn get_quantity(&self) -> Option<String> {
        self.get_value_as_string("quantity")
    }

    pub fn get_weight(&self) -> Option<String> {
        self.get_value_as_string("weight")
    }

    pub fn get_group(&self) -> Option<String> {
        self.get_value_as_string("group")
    }

    pub fn get_talent_value(&self) -> Option<String> {
        self.get_value_as_string("talentValue")
    }

    pub fn get_characteristic_values(&self) -> Option<(String, String, String)> {
        // Use the ? operator for clean early returns
        let c1 = self.get_value_as_string("characteristic1")?;
        let c2 = self.get_value_as_string("characteristic2")?;
        let c3 = self.get_value_as_string("characteristic3")?;

        Some((c1, c2, c3))
    }

    pub fn get_guidevalue_value(&self) -> Option<String> {
        self.get_value_as_string("guidevalue")
    }

    pub fn get_st_f_value(&self) -> Option<String> {
        self.get_value_as_string("StF")
    }

    pub fn get_step_value(&self) -> Option<String> {
        self.get_value_as_string("step")
    }
}

impl Character {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let character: Character = serde_json::from_str(&content)?;
        Ok(character)
    }

    pub fn get_ap_items(&self) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| item.system.get_ap_value().is_some())
            .collect()
    }

    // Private helper method
    fn get_items_by_types(&self, types: &[&str]) -> Vec<&Item> {
        let mut items: Vec<&Item> = self.items
            .iter()
            .filter(|item| types.contains(&item.item_type.as_str()))
            .collect();

        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn get_skills(&self) -> Vec<&Item> {
        self.get_items_by_types(&["skill"])
    }

    pub fn get_combat_skills(&self) -> Vec<&Item> {
        self.get_items_by_types(&["combatskill"])
    }

    pub fn get_spells(&self) -> Vec<&Item> {
        self.get_items_by_types(&["spell"])
    }

    pub fn get_rituals(&self) -> Vec<&Item> {
        self.get_items_by_types(&["ritual"])
    }

    pub fn get_spells_and_rituals(&self) -> Vec<&Item> {
        self.get_items_by_types(&["spell", "ritual"])
    }

    pub fn get_magic_tricks(&self) -> Vec<&Item> {
        self.get_items_by_types(&["magictrick"])
    }

    pub fn get_liturgies(&self) -> Vec<&Item> {
        self.get_items_by_types(&["liturgy"])
    }

    pub fn get_ceremonies(&self) -> Vec<&Item> {
        self.get_items_by_types(&["ceremony"])
    }

    pub fn get_liturgies_and_ceremonies(&self) -> Vec<&Item> {
        self.get_items_by_types(&["liturgy", "ceremony"])
    }

    pub fn get_blessings(&self) -> Vec<&Item> {
        self.get_items_by_types(&["blessing"])
    }

    pub fn get_advantages(&self) -> Vec<&Item> {
        self.get_items_by_types(&["advantage"])
    }

    pub fn get_disadvantages(&self) -> Vec<&Item> {
        self.get_items_by_types(&["disadvantage"])
    }

    pub fn has_image(&self) -> bool {
        self.img.is_some() && !self.img.as_ref().unwrap().is_empty()
    }

    pub fn get_image_url(&self) -> Option<&str> {
        self.img.as_deref()
    }
}