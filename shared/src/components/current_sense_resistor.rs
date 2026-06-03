use serde::{Serialize, Deserialize}; 
use crate::enumes_for_components;
use enumes_for_components::types::ComponentType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentSenseResistor 
    {
        pub component_type: ComponentType,
        pub name: String,
        pub resistance_ohm: f64,
        pub tolerance_percen: f64,
        pub package: String,
        pub max_current_amp: f64,
        pub power_rating_watt: f64,
        pub temperature_coefficient_ppm: f64,
        pub operating_temperature_min_c: f64,
        pub operating_temperature_max_c: f64,
        
    }
