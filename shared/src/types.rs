use serde::{Serialize, Deserialize}; 

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentType
{
    CurrentSenseResistor, // Shunt - Токоизмерительный резистор на четырёх ножках - Vishay Dale WSLP5931L2000FEA!
    FlightControllerBoard, // ControllerBoard - Тяжолая плата: 0.7 мм ( 2 унции меди на квадратный фут ), 4 слоя: F.Cu - сигналы, In1.Cu - GND, In2.Cu - PWR, B.Cu - нижний слой.. Первые два слоя залиты медью (в процентном соотношении из теплового расчёта операясь на показатель >= 200А)!
    

    
}