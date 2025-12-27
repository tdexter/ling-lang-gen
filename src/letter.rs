
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Letter {
  romanization: String,
  weight: f64,
  final_weight: f64,
}

impl Letter {
  pub fn romanization(&self) -> &str {
    &self.romanization
  }
  
  pub fn weight(&self, is_final: bool) -> f64 {
    if is_final {
      self.final_weight
    } else {
      self.weight
    }
  }

  pub fn from_file(filename: &str) -> Result<Vec<Letter>, Box<dyn std::error::Error>> {
    let file_path = Path::new(filename);
    let file = File::open(file_path)?;

    let mut rdr = csv::Reader::from_reader(file);

    let mut results = vec![];
    // Iterate over the deserialized records
    for result in rdr.deserialize() {
        let record: Letter = result?;
        results.push(record);
    }

    Ok(results)
  }

}
