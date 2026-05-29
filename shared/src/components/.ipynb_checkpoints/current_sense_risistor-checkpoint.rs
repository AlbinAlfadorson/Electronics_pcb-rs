#[derive(Debug, Serialize, Desirealize)]
pub struct CurrentSenseResistor 
    {
        pub component_type: String,
        pub name: String,
        pub resistance_ohm: f64,
        pub tolerance_percen: f32,
        pub package: String,
        pub max_current_amp: f64,
        pub power_rating_watt: f64,
        pub temperature_coefficient_ppm: f64,
        pub operating_temperature_min_c: f64,
        pub operating_temperature_max_c: f64,
        
    };