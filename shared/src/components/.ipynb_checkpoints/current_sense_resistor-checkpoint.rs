#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentSenseResistor 
    {
        pub component_type: ComponentType,
        pub name: String,
        pub resistance_ohm: f64,
        pub tolerance_percen: f32,
        pub package: String,
        pub max_current_amp: f64,
        pub power_rating_watt: f64,
        pub temperature_coefficient_ppm: f64,
        pub operating_temperature_min_c: f64,
        pub operating_temperature_max_c: f64,
        
    }
#[derive(Debug, Serialize, Deserialize)]
pub struct FlightControllerBoard 
    {

        pub component_type: ComponentType,
        pub name: String,
        pub layers: u32,
        pub material: String,
        pub width_mm: f64,
        pub height_mm: f64,
        pub copper_thickness_um: u32,
        pub shape: String,
        pub corner_radius_mm: f64,
        pub mounting_holes: u16,
        pub mounting_hole_diametr_mm: f32,
        pub copper_pour_top: bool,
        pub copper_pour_bottom: bool,
        pub thermal_vias: bool,
        pub thermal_via_diametr_mm: f32,
        pub termal_via_pitch_mm: f32,
        pub max_current_amp: f32,
        pub max_tenp: f32,

        
    }