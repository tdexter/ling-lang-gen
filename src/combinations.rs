use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn empty_string() -> String {
    String::new()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Combo {
    #[serde(default = "empty_string")]
    onset: String,
    vowel: String,
}

impl Combo {
    pub fn from_file(filename: &str) -> 
      Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let file_path = Path::new(filename);
        let file = File::open(file_path)?;

        let mut rdr = csv::Reader::from_reader(file);

        let mut map = HashMap::new();

        // Iterate over the deserialized records
        for result in rdr.deserialize() {
            let record: Combo = result?;

            map.entry(record.onset.clone())
                .or_insert_with(Vec::new)
                .push(record.vowel);
        }

        Ok(map)
    }

}
