
use crate::assembly::Inventory;
use crate::letter::Letter;
use crate::order::Order;

// Probably because I'm new to this but trying to get generics to work with
//   rand was a bit hard. There's probably a better type to use.
pub trait ValueGenerator<T> {
  fn value_in_range(&self, start: T, end: T, inclusive: bool) -> T;
}

pub struct RandomRange {
}

impl RandomRange {
  pub fn new() -> Self {
    RandomRange {}
  }
}

impl ValueGenerator<f32> for RandomRange {
  fn value_in_range(&self, start: f32, end: f32, inclusive: bool) -> f32 
  {
    if inclusive {
      rand::random_range(start..=end)
    } else {
      rand::random_range(start..end)
    }
  }
} 

pub struct Generator<T: ValueGenerator<f32>> {
  inv: Inventory,
  valuegen: T
}

impl<T: ValueGenerator<f32>> Generator<T> {
  pub fn new(_inv: Inventory, _valuegen: T) -> Self {
    Generator { 
      inv: _inv,
      valuegen: _valuegen
    }
  }

  pub fn inv(&self) -> &Inventory {
    &self.inv
  }

  pub fn valuegen(&self) -> &T {
    &self.valuegen
  }

  pub fn select_word_size(
    &self,
    from: u8, 
    to:u8
  ) -> u8 
  {
    let value = self.valuegen().value_in_range((from - 1) as f32, to as f32, false);
    value.ceil() as u8
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
    if letters.is_empty() {
      return "".to_string();
    }
  
    let total_chance: f32 = letters.iter().map(|l| l.weight(order)).sum();

    let mut random_value = self.valuegen().value_in_range(0.0, total_chance, false);

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
    let onset = self.select_random_letter(self.inv().onsets(), order);
    let vowels = self.inv.get_vowels(&onset);
    let vowel = self.select_random_letter(&vowels, order);
    let coda = self.select_random_letter(self.inv().codas(), order);

    format!("{}{}{}", onset, vowel, coda)
  } 

}

#[cfg(test)]
mod tests {
  use super::Generator;
  use super::ValueGenerator;
  use crate::assembly::Inventory;
use crate::letter::Letter;

  pub struct DeterministicRange {}

  impl ValueGenerator<f32> for DeterministicRange {
    fn value_in_range(
      &self, 
      from: f32, 
      _to: f32, 
      _inclusive: bool
    ) -> f32 
    {
      from
    }
  } 

  fn get_test_generator(inv: Inventory) -> Generator<DeterministicRange> {
    let valuegen = DeterministicRange {};
    Generator::new(inv, valuegen)
  }

  
  #[test]
  fn select_word() {
    let onset_1 = Letter::new("b".to_string(), 1.0, None, None);
    let onset_2 = Letter::new("c".to_string(), 1.0, None, None);
    let vowel_1 = Letter::new("a".to_string(), 1.0, None, None);
    let coda_1 = Letter::new("m".to_string(), 1.0, None, None);

    let onset_vowel_combos = std::collections::HashMap::from([
      ("b".to_string(), vec![vowel_1.clone()]),
      ("c".to_string(), vec![vowel_1.clone()]),
    ]);

    let inv = Inventory::new(
      vec![onset_1, onset_2],
      vec![coda_1], 
      onset_vowel_combos);

    let gennie = get_test_generator(inv);
    let word = gennie.gen_word(3);
    assert_eq!(word, "bambambam");
  }

  #[test]
  fn select_word_initial_weight() {
    let onset_1 = Letter::new("b".to_string(), 1.0, Some(0.0), None);
    let onset_2 = Letter::new("c".to_string(), 1.0, None, None);
    let vowel_1 = Letter::new("a".to_string(), 1.0, None, None);

    let onset_vowel_combos = std::collections::HashMap::from([
      ("b".to_string(), vec![vowel_1.clone()]),
      ("c".to_string(), vec![vowel_1.clone()]),
    ]);

    let inv = Inventory::new(
      vec![onset_1, onset_2],
      vec![], 
      onset_vowel_combos);

    let gennie = get_test_generator(inv);
    let word = gennie.gen_word(3);
    assert_eq!(word, "cababa");
  }

}
