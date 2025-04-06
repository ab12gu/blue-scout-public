use std::fmt::Display;

use blue_scout_macros::{define_reduced_columns, define_struct, define_team_data};
#[cfg(feature = "ssr")]
use duckdb::ToSql;

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    U16(u16),
    U32(u32),
    U64(u64),
    I16(i16),
    I32(i32),
    I64(i64),
    String(String),
    Bool(bool),
    Float(f32),
}

impl DataType {
    pub fn name(&self) -> DataTypeName {
        match self {
            DataType::U16(_) => DataTypeName::U16,
            DataType::U32(_) => DataTypeName::U32,
            DataType::U64(_) => DataTypeName::U64,
            DataType::I16(_) => DataTypeName::I16,
            DataType::I32(_) => DataTypeName::I32,
            DataType::I64(_) => DataTypeName::I64,
            DataType::String(_) => DataTypeName::String,
            DataType::Bool(_) => DataTypeName::Bool,
            DataType::Float(_) => DataTypeName::Float,
        }
    }
}

impl From<u16> for DataType {
    fn from(value: u16) -> Self {
        DataType::U16(value)
    }
}

impl From<u32> for DataType {
    fn from(value: u32) -> Self {
        DataType::U32(value)
    }
}

impl From<u64> for DataType {
    fn from(value: u64) -> Self {
        DataType::U64(value)
    }
}

impl From<String> for DataType {
    fn from(value: String) -> Self {
        DataType::String(value)
    }
}

impl From<bool> for DataType {
    fn from(value: bool) -> Self {
        DataType::Bool(value)
    }
}

impl From<f32> for DataType {
    fn from(value: f32) -> Self {
        DataType::Float(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataTypeName {
    U16,
    U32,
    U64,
    I16,
    I32,
    I64,
    String,
    Bool,
    Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    Normal,
    Checklist,
    Select,
    None,
}

impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterType::Normal => write!(f, "normal"),
            FilterType::Checklist => write!(f, "checklist"),
            FilterType::Select => write!(f, "select"),
            FilterType::None => write!(f, "none"),
        }
    }
}

// PLEASE NOTE:
// Changing the display name will not affect the column name in the database
// but if you change the field name, it will affect the column name in
// the database. This means migrating the column name is not supported yet.
define_struct!(
    DataPoint,
    name: String => "Name" @ Normal,
    match_number: u16 => "Match" @ Normal,
    team_number: u32 => "Team" @ Normal,
    auto_coral: u16 => "Auto Coral" @ Normal,
    auto_algae: u16 => "Auto Algae" @ Normal,
    auto_leave: bool => "Auto Leave" @ Select,
    algae_clear: bool => "Algae Clear" @ Select,
    l1_coral: u16 => "L1" @ Normal,
    l2_coral: u16 => "L2" @ Normal,
    l3_coral: u16 => "L3" @ Normal,
    l4_coral: u16 => "L4" @ Normal,
    dropped_coral: u16 => "Dropped" @ Normal,
    algae_barge: u16 => "Algae Barge" @ Normal,
    algae_floor_hole: u16 => "Algae Floor Hole" @ Normal,
    climb: String => "Climb" @ Checklist,
    defense_bot: bool => "Defense" @ Select,
    notes: String => "Notes" @ None,
);

define_reduced_columns!(
    DataPoint,
    "Match" @ Normal => s.match_number.to_string(),
    "Team" @ Normal => s.team_number.to_string(),
    "Auto Coral" @ Normal => s.auto_coral.to_string(),
    "Auto Leave" @ Select => if s.auto_leave { "Yes".to_string() } else { "No".to_string() },
    "Algae Clear" @ Select => if s.algae_clear { "Yes".to_string() } else { "No".to_string() },
    "Teleop Coral" @ Normal => (s.l1_coral + s.l2_coral + s.l3_coral + s.l4_coral).to_string(),
    "Teleop Algae" @ Normal => (s.algae_barge + s.algae_floor_hole).to_string(),
    "Climb" @ Checklist => s.climb.clone(),
    "Defense" @ Select => if s.defense_bot { "Yes".to_string() } else { "No".to_string() },
);

define_team_data!(
    DataPoint,
    "Avg Coral" => {
        format!("{:.1}", v
            .iter()
            .map(|x| (x.l4_coral + x.l3_coral + x.l2_coral + x.l1_coral) as u32)
            .sum::<u32>() as f64
        / v.len() as f64)
    },
    "Avg Auto Coral" => {
        format!("{:.1}",
        v
            .iter()
            .map(|x| x.auto_coral as u32)
            .sum::<u32>() as f64
        / v.len() as f64)
    },
    "Avg Barge Algae" => {
        format!("{:.1}", v
            .iter()
            .map(|x| x.algae_barge as u32)
            .sum::<u32>() as f64
            / v.len() as f64)
    },
    "Scoring Locations" => {
        let (score_l1, score_l2, score_l3, score_l4) = (
            v.iter().filter(|x| x.l1_coral > 0).count() as u32,
            v.iter().filter(|x| x.l2_coral > 0).count() as u32,
            v.iter().filter(|x| x.l3_coral > 0).count() as u32,
            v.iter().filter(|x| x.l4_coral > 0).count() as u32,
        );
        let locations = [
            ("L1", score_l1),
            ("L2", score_l2),
            ("L3", score_l3),
            ("L4", score_l4),
        ]
            .iter()
            .filter_map(|x| if x.1 > 0 { Some(x.0) } else { None })
            .collect::<Vec<&str>>()
            .join(", ");
        if locations.is_empty() { "None".to_string() } else { locations }
    },
    "Sum of Deep Climbs" => {
        v.iter().filter(|x| x.climb == "Deep").count() as u32
    },
    "Sum of Not Attempted" => {
        v.iter().filter(|x| x.climb == "Not Attempted").count() as u32
    }
);
