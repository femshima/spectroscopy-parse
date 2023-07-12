use std::{collections::HashMap, error::Error, fs::File};

use ole::{EntryType, Reader};

use crate::spectra::Spectra;

mod data_info;
mod spectra;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("test-data")?;
    let parser = Reader::new(file).unwrap();

    let mut spectra_name: HashMap<u32, &str> = HashMap::new();
    let mut spectras: HashMap<u32, Spectra> = HashMap::new();
    for entry in parser.iterate() {
        println!("{} {:?}", entry, entry.parent_node());

        match entry._type() {
            EntryType::RootStorage => {
                spectra_name.insert(entry.id(), "[Root]");
            }
            EntryType::UserStorage => {
                spectra_name.insert(entry.id(), entry.name());
            }
            EntryType::UserStream => {
                let parent_id = entry.parent_node().unwrap();
                let spectra = spectras.entry(parent_id).or_default();

                if let Ok(data) = parser.get_entry_slice(entry).as_mut() {
                    spectra.read_from(entry.name(), data)?;
                }
            }
            _ => (),
        }
    }

    for (id, spectra) in spectras {
        if spectra.y_data.len() > 0 {
            dbg!(
                spectra_name.get(&id).unwrap(),
                &spectra.data_info,
                &spectra.y_data.len()
            );
        }
    }
    Ok(())
}
