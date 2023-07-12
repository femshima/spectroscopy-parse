use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

use crate::data_info::DataInfo;

#[derive(Debug, Clone, Default)]
pub struct Spectra {
    pub data_info: DataInfo,
    pub y_data: Vec<f32>,
}

impl Spectra {
    pub fn read_from(&mut self, name: &str, mut reader: impl Read) -> std::io::Result<()> {
        match name {
            "DataInfo" => {
                self.data_info = DataInfo::from_reader(reader)?;
            }
            "Y-Data" => {
                self.y_data = Vec::new();
                while let Ok(chunk) = reader.read_f32::<LittleEndian>() {
                    self.y_data.push(chunk);
                }
            }
            _ => (),
        }
        Ok(())
    }
}
