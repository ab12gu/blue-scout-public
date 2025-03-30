use blue_scout_macros::{define_reduced_columns, define_struct};
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

define_struct!(
    DataPoint,
    name: String => "Name",
    match_number: u16 => "Match",
    team_number: u32 => "Team",
    auto_coral: u16 => "Auto Coral",
    auto_algae: u16 => "Auto Algae",
    auto_leave: bool => "Auto Leave",
    algae_clear: bool => "Algae Clear",
    l1_coral: u16 => "L1",
    l2_coral: u16 => "L2",
    l3_coral: u16 => "L3",
    l4_coral: u16 => "L4",
    dropped_coral: u16 => "Dropped",
    algae_barge: u16 => "Algae Barge",
    algae_floor_hole: u16 => "Algae Floor Hole",
    climb: String => "Climb",
    defense_bot: bool => "Defense",
    notes: String => "Notes",
);

define_reduced_columns!(DataPoint,
    "Match" => s.match_number.to_string(),
    "Team" => s.team_number.to_string(),
    "Auto Coral" => s.auto_coral.to_string(),
    "Auto Leave" => if s.auto_leave { "Yes".to_string() } else { "No".to_string() },
    "Algae Clear" => if s.algae_clear { "Yes".to_string() } else { "No".to_string() },
    "Teleop Coral" => (s.l1_coral + s.l2_coral + s.l3_coral + s.l4_coral).to_string(),
    "Teleop Algae" => (s.algae_barge + s.algae_floor_hole).to_string(),
    "Climb" => s.climb.clone(),
    "Defense" => if s.defense_bot { "Y".to_string() } else { "N".to_string() },
);
