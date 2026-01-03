
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

use crate::order::Order;

#[derive(Debug, Deserialize, Clone)]
pub struct Letter {
  romanization: String,
  weight: f32,
  #[serde(default)]
  initial_weight:Option<f32>,
  #[serde(default)]
  final_weight: Option<f32>,
}

impl Letter {
  pub fn new(
    romanization: String,
    weight: f32,
    initial_weight: Option<f32>,
    final_weight: Option<f32>,
  ) -> Self {
    Self {
      romanization,
      weight,
      initial_weight,
      final_weight,
    }
  }

  pub fn romanization(&self) -> &str {
    &self.romanization
  }

  pub fn weight(&self, order: &Order) -> f32 {
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

#[cfg(test)]
mod tests {
    use super::Letter;
    use crate::order::Order;

    fn get_test_letter() -> Letter {
        Letter {
            romanization: "a".to_string(),
            weight: 1.0,
            initial_weight: Some(2.0),
            final_weight: Some(3.0),
        }
    }

    #[test]
    fn normal_weight() {
        let letter = get_test_letter();
        let order = Order::Medial;
        assert_eq!(letter.weight(&order), 1.0);
    }

    #[test]
    fn initial_weight() {
        let letter = get_test_letter();
        let order = Order::Initial;
        assert_eq!(letter.weight(&order), 2.0);
    }

    #[test]
    fn final_weight() {
        let letter = get_test_letter();
        let order = Order::Final;
        assert_eq!(letter.weight(&order), 3.0);
    }

    // from_file is not tested because it is mostly integration. 
}
