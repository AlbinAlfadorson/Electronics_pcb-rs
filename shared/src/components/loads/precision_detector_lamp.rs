use::serde::{Serialize, Deserialize};
use crate::types::LoadType;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrecisionDetectorLamp 
{

    pub name: String,
    pub trigger_voltage_v: f64,
    pub tolerance_mv: f64,
    pub trigger_mode: String,
    pub current_draw_a: f64,
    pub load_type: LoadType,
    pub resistance_hot_ohm: f64,
    pub debounce_ms: f64,



}
