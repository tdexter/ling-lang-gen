use std::collections::HashMap;

pub use crate::letter::Letter;
pub use crate::combinations::Combo;

const VOWEL_FILE: &str = "vowels.csv";
const ONSET_FILE: &str = "onsets.csv";
const CODA_FILE: &str = "codas.csv";
const ONSET_VOWELS_FILE: &str = "onset-vowels.csv";

pub struct Inventory {
  onsets: Vec<Letter>,
  codas: Vec<Letter>,
  onset_vowels: HashMap<String, Vec<Letter>>,
}

impl Inventory {
  pub fn new(
    onsets: Vec<Letter>,
    codas: Vec<Letter>,
    onset_vowels: HashMap<String, Vec<Letter>>,
  ) -> Self {
    Self {
      onsets,
      codas,
      onset_vowels,
    }
  }

  pub fn onsets(&self) -> &Vec<Letter> {
    &self.onsets
  } 

  pub fn codas(&self) -> &Vec<Letter> {
    &self.codas
  }

  pub fn onset_vowels(&self) -> &HashMap<String, Vec<Letter>> {
    &self.onset_vowels
  }

  pub fn load_from_files() -> Self {
    let vowels = Letter::from_file(VOWEL_FILE)
        .expect(format!("Failed to read {}", VOWEL_FILE).as_str());
    let onsets = Letter::from_file(ONSET_FILE)
        .expect(format!("Failed to read {}", ONSET_FILE).as_str());
    let codas = Letter::from_file(CODA_FILE)
        .expect(format!("Failed to read {}", CODA_FILE).as_str());

    // read and process the onset-vowel combinations
    let onset_vowel_combos = Combo::from_file(ONSET_VOWELS_FILE)
        .expect(format!("Failed to read {}", ONSET_VOWELS_FILE).as_str());
    let onset_vowels = Inventory::map_onset_vowels(&onset_vowel_combos, &onsets, &vowels);

    Inventory {
        onsets,
        codas,
        onset_vowels,
    }
  }

  fn map_onset_vowels(
    combos: &HashMap<String, Vec<String>>, 
    onsets: &Vec<Letter>, 
    vowels: &Vec<Letter>
  ) -> HashMap<String, Vec<Letter>> 
  {
    let mut map = HashMap::new();

    for (onset_str, combo_list) in combos {
      let onset_letter = onsets.iter()
        .find(|l| l.romanization() == onset_str)
        .expect("Onset letter not found in onset-vowels");

      let mut vowel_letters = vec![];
      for vowel in combo_list {
        let vowel_letter = vowels.iter()
          .find(|l| l.romanization() == vowel)
          .expect("Vowel letter not found");
        vowel_letters.push(vowel_letter.clone());
      }

      map.insert(onset_letter.romanization().to_string(), vowel_letters);
    }

    map
  }

  pub fn get_vowels(
    &self,
    onset: &str, 
  ) -> Vec<Letter> 
  {
    match self.onset_vowels.get(onset) {
      Some(vowels) => vowels.clone(),
      None => vec![],
    }
  }

}
