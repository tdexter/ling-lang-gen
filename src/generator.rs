
use crate::assembly::Inventory;
use crate::letter::Letter;
use crate::order::Order;

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
    rand::random_range(from..=to)
  }

  pub fn gen_word(
    &self,
    size: usize
  ) -> String 
  {
    let mut word = String::new();
    for index in 1..=size {
      let order = Order::from_position(index, size);
      let syllable = self.gen_syllable(&order);
      word.push_str(&syllable);
    }

    word
  }

  fn select_random_letter(
    &self,
    letters: &Vec<Letter>,
    order: &Order,
  ) -> String 
  {
    let total_chance: f64 = letters.iter().map(|l| l.weight(order)).sum();

    let mut random_value = rand::random_range(0.0..total_chance);

    for letter in letters {
      if random_value < letter.weight(order) {
        return letter.romanization().to_string();
      }
      random_value -= letter.weight(order);
    }

    letters[letters.len() - 1].romanization().to_string()
  }

  fn gen_syllable(
    &self,
    order: &Order,
  ) -> String 
  {
    let onset = self.select_random_letter(&self.inv.onsets(), order);
    let vowels = self.inv.get_vowels(&onset);
    let vowel = self.select_random_letter(&vowels, order);
    let coda = self.select_random_letter(&self.inv.codas(), order);

    format!("{}{}{}", onset, vowel, coda)
  } 

}

