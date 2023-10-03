use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::units::Unit;

#[derive(Debug, Clone, Default)]
pub struct DataInfo {
    _p1: [u32; 3],
    pub channels: u32,
    _p2: u32,
    pub point_count: u32,
    pub start: f64,
    pub end: f64,
    pub step: f64,
    pub unit1: (Unit, Unit),
    pub unit2: (Unit, Unit),
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
            ],
            channels: rdr.read_u32::<LittleEndian>()?,
            _p2: rdr.read_u32::<LittleEndian>()?,
            point_count: rdr.read_u32::<LittleEndian>()?,
            start: rdr.read_f64::<LittleEndian>()?,
            end: rdr.read_f64::<LittleEndian>()?,
            step: rdr.read_f64::<LittleEndian>()?,
            unit1: (
                Unit::from(rdr.read_u32::<LittleEndian>()?),
                Unit::from(rdr.read_u32::<LittleEndian>()?),
            ),
            unit2: (
                Unit::from(rdr.read_u32::<LittleEndian>()?),
                Unit::from(rdr.read_u32::<LittleEndian>()?),
            ),
            xmin: rdr.read_f64::<LittleEndian>()?,
            ymax: rdr.read_f64::<LittleEndian>()?,
            xmax: rdr.read_f64::<LittleEndian>()?,
            ymin: rdr.read_f64::<LittleEndian>()?,
        })
    }
}
