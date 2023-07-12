use std::{error::Error, fs::File};

use ole::{EntryType, Reader};

use crate::spectra::Spectra;

mod data_info;
mod spectra;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("test-data")?;
    let parser = Reader::new(file).unwrap();

    let mut spectra = Spectra::default();
    for entry in parser.iterate() {
        println!("{}", entry);
        if entry._type() != EntryType::UserStream {
            continue;
        }
        if let Ok(data) = parser.get_entry_slice(entry).as_mut() {
            spectra.read_from(entry.name(), data)?;
        }
    }

    dbg!(&spectra.data_info);
    dbg!(&spectra.y_data.len());
    // dbg!(spectra);
    Ok(())
}
