use std::{fs, io};

use serde::Deserialize;

#[derive(Deserialize)]
struct Structure {
    nickname: String,
    cells: Vec<Vec<u8>>
}

#[derive(Deserialize)]
pub struct Structures {
    name: String,
    structures: Vec<Structure>,
}

pub fn load_from_json(path: String) -> io::Result<Structures> {
    let contents = fs::read_to_string(path).unwrap_or("".to_string());
    let structures: Structures = serde_json::from_str(&contents)?;
    println!("# structures on bank \"{}\": {}", &structures.name, &structures.structures.len());
    for s in &structures.structures {
        println!("{}", s.nickname);
    }

    Ok(structures)
}

impl Structures {
    pub fn get_cells(&self, idx: usize) -> Vec<Vec<u8>> {
        self.structures[idx].cells.clone()
    }

    pub fn len(&self) -> usize {
        self.structures.len()
    }
}

