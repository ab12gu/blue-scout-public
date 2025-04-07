use core::fmt::Display;

use blue_scout_macros::{define_reduced_columns, define_struct, define_team_data};
#[cfg(feature = "ssr")]
use duckdb::ToSql;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
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
    #[must_use]
    pub const fn name(&self) -> DataTypeName {
        match *self {
            Self::U16(_) => DataTypeName::U16,
            Self::U32(_) => DataTypeName::U32,
            Self::U64(_) => DataTypeName::U64,
            Self::I16(_) => DataTypeName::I16,
            Self::I32(_) => DataTypeName::I32,
            Self::I64(_) => DataTypeName::I64,
            Self::String(_) => DataTypeName::String,
            Self::Bool(_) => DataTypeName::Bool,
            Self::Float(_) => DataTypeName::Float,
        }
    }
}

impl From<u16> for DataType {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for DataType {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for DataType {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<String> for DataType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for DataType {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f32> for DataType {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
#[non_exhaustive]
pub enum FilterType {
    Normal,
    Checklist,
    Select,
    None,
}

impl Display for FilterType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Normal => write!(f, "normal"),
            Self::Checklist => write!(f, "checklist"),
            Self::Select => write!(f, "select"),
            Self::None => write!(f, "none"),
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
    "Match" @ Normal => |s: &Self| s.match_number.to_string(),
    "Team" @ Normal => |s: &Self| s.team_number.to_string(),
    "Auto Coral" @ Normal => |s: &Self| s.auto_coral.to_string(),
    "Auto Leave" @ Select => |s: &Self| if s.auto_leave { "Yes".to_owned() } else { "No".to_owned() },
    "Algae Clear" @ Select => |s: &Self| if s.algae_clear { "Yes".to_owned() } else { "No".to_owned() },
    "Teleop Coral" @ Normal => |s: &Self| (s.l1_coral + s.l2_coral + s.l3_coral + s.l4_coral).to_string(),
    "Teleop Algae" @ Normal => |s: &Self| (s.algae_barge + s.algae_floor_hole).to_string(),
    "Climb" @ Checklist => |s: &Self| s.climb.clone(),
    "Defense" @ Select => |s: &Self| if s.defense_bot { "Yes".to_owned() } else { "No".to_owned() },
);

define_team_data!(
    DataPoint,
    "Avg Coral" => |v: &[DataPoint]| {
        format!("{:.1}", f64::from(v
            .iter()
            .map(|x| u32::from(x.l4_coral + x.l3_coral + x.l2_coral + x.l1_coral))
            .sum::<u32>())
        / v.len() as f64)
    },
    "Avg Auto Coral" => |v: &[DataPoint]|{
        format!("{:.1}",
        f64::from(v
            .iter()
            .map(|x| u32::from(x.auto_coral))
            .sum::<u32>())
        / v.len() as f64)
    },
    "Avg Barge Algae" => |v: &[DataPoint]|{
        format!("{:.1}", f64::from(v
            .iter()
            .map(|x| u32::from(x.algae_barge))
            .sum::<u32>())
            / v.len() as f64)
    },
    "Scoring Locations" => |v: &[DataPoint]|{
        let (score_l1, score_l2, score_l3, score_l4) = (
            u32::try_from(v.iter().filter(|x| x.l1_coral > 0).count()).expect("This should not be bigger than u32::MAX"),
            u32::try_from(v.iter().filter(|x| x.l2_coral > 0).count()).expect("This should not be bigger than u32::MAX"),
            u32::try_from(v.iter().filter(|x| x.l3_coral > 0).count()).expect("This should not be bigger than u32::MAX"),
            u32::try_from(v.iter().filter(|x| x.l4_coral > 0).count()).expect("This should not be bigger than u32::MAX"),
        );
        let locations = [
            ("L1", score_l1),
            ("L2", score_l2),
            ("L3", score_l3),
            ("L4", score_l4),
        ]
            .iter()
            .filter_map(|x| (x.1 > 0).then_some(x.0))
            .collect::<Vec<&str>>()
            .join(", ");
        if locations.is_empty() { "None".to_owned() } else { locations }
    },
    "Sum of Deep Climbs" => |v: &[DataPoint]|{
        u32::try_from(v.iter().filter(|x| x.climb == "Deep").count()).expect("This should not be bigger than u32::MAX")
    },
    "Sum of Not Attempted" => |v: &[DataPoint]|{
        u32::try_from(v.iter().filter(|x| x.climb == "Not Attempted").count()).expect("This should not be bigger than u32::MAX")
    }
);
