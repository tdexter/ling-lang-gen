
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

use crate::order::Order;

#[derive(Debug, Deserialize, Clone)]
pub struct Letter {
  romanization: String,
  weight: f64,
  #[serde(default)]
  initial_weight:Option<f64>,
  #[serde(default)]
  final_weight: Option<f64>,
}

impl Letter {
  pub fn romanization(&self) -> &str {
    &self.romanization
  }
  
  pub fn weight(&self, order: &Order) -> f64 {
    if order.is_initial() && self.initial_weight.is_some() {
      self.initial_weight.unwrap()
    } else if order.is_final() && self.final_weight.is_some() {
      self.final_weight.unwrap()
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
