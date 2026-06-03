use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadType 
{
	
	PrecisionDetectorLamp, // PrecisionDetectorLamp - виртуальная лампа на роли прицизионного анализатора, чисто виртуальная, работает под текущее изделие, определяет корректность отработки стендовой платы и каждого компанента, харрактеристики будут меняться в процессе испытаний, по общей формуле - U_лампы = U_источника − U_падения_на_проверяемом_компоненте!
	
	
}
