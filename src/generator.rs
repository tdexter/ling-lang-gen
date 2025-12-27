
use crate::assembly::Inventory;
use crate::letter::Letter;


pub struct Generator<'g> {
  inv: &'g Inventory,
}

impl<'g> Generator<'g> {
  pub fn new(_inv: &'g Inventory) -> Self {
    Generator { 
      inv: _inv
    }
  }
  
  pub fn select_word_size(
    &self,
    from: u8, 
    to:u8
  ) -> u8 
  {
    let size = to - from + 1;
    let random_value = rand::random::<f64>() * (size as f64);
    random_value.floor() as u8 + from
  }

  pub fn gen_word(
    &self,
    size: usize
  ) -> String 
  {
    let mut word = String::new();
    for index in 1..=size {
      let is_final = index == size;
      let is_initial = index == 1;
      let syllable = self.gen_syllable(is_initial,is_final);
      word.push_str(&syllable);
    }

    word
  }

  fn select_random_letter(
    &self,
    letters: &Vec<Letter>, 
    is_final: bool
  ) -> String 
  {
    let total_chance: f64 = letters.iter().map(|l| l.weight(is_final)).sum();
    let mut random_value = rand::random::<f64>() * total_chance;

    for letter in letters {
      if random_value < letter.weight(is_final) {
        return letter.romanization().to_string();
      }
      random_value -= letter.weight(is_final);
    }

    letters[letters.len() - 1].romanization().to_string()
  }

  fn gen_syllable(
    &self,
    is_initial: bool,
    is_final: bool
  ) -> String 
  {
    let onset = self.select_random_letter(&self.inv.onsets(), is_final);
    let mut vowels = self.inv.get_vowels(&onset).clone();
    if is_initial {
      if let Some(pos) = vowels.iter().position(|v| v.romanization() == "") {
        vowels.remove(pos);
      }
    }
  
    let vowel = self.select_random_letter(&vowels, is_final);
    let coda = self.select_random_letter(&self.inv.codas(), is_final);

    format!("{}{}{}", onset, vowel, coda)
  } 

}

