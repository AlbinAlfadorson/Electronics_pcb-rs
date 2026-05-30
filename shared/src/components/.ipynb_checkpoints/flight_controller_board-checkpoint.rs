#[derive(Debug, Serialize, Deserialize)]
pub struct FlightControllerBoard 
    {

        pub component_type: ComponentType,
        pub name: String,
        pub layers: u8,
        pub material: String,
        pub width_mm: f64,
        pub height_mm: f64,
        pub copper_thickness_um: u16,
        pub shape: String,
        pub corner_radius_mm: f64,
        pub mounting_holes: u8,
        pub mounting_hole_diameter_mm: f32,
        pub copper_pour_top: bool,
        pub copper_pour_bottom: bool,
        pub thermal_vias: bool,
        pub thermal_via_diameter_mm: f32,
        pub thermal_via_pitch_mm: f32,
        pub max_current_amp: f64,
        pub max_temp_c: f64,

        
    }