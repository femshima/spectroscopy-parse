use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, Default)]
pub struct DataInfo {
    _p1: [u32; 5],
    pub point_count: u32,
    pub start: f64,
    pub end: f64,
    pub step: f64,
    _p3: [u32; 4],
    pub xmin: f64,
    pub ymax: f64,
    pub xmax: f64,
    pub ymin: f64,
}

impl DataInfo {
    pub fn from_reader(mut rdr: impl Read) -> std::io::Result<Self> {
        Ok(DataInfo {
            _p1: [
                rdr.read_u32::<LittleEndian>()?,
                rdr.read_u32::<LittleEndian>()?,
                rdr.read_u32::<LittleEndian>()?,
                rdr.read_u32::<LittleEndian>()?,
                rdr.read_u32::<LittleEndian>()?,
            ],
            point_count: rdr.read_u32::<LittleEndian>()?,
            start: rdr.read_f64::<LittleEndian>()?,
            end: rdr.read_f64::<LittleEndian>()?,
            step: rdr.read_f64::<LittleEndian>()?,
            _p3: [
                rdr.read_u32::<LittleEndian>()?,
                rdr.read_u32::<LittleEndian>()?,
                rdr.read_u32::<LittleEndian>()?,
                rdr.read_u32::<LittleEndian>()?,
            ],
            xmin: rdr.read_f64::<LittleEndian>()?,
            ymax: rdr.read_f64::<LittleEndian>()?,
            xmax: rdr.read_f64::<LittleEndian>()?,
            ymin: rdr.read_f64::<LittleEndian>()?,
        })
    }
}
