mod letter;

pub use crate::letter::Letter;

struct Inventory {
  vowels: Vec<Letter>,
  onsets: Vec<Letter>,
  codas: Vec<Letter>,
}

fn select_random_letter(letters: &Vec<Letter>) -> &Letter {
  let total_chance: f64 = letters.iter().map(|l| l.weight()).sum();
  let mut random_value = rand::random::<f64>() * total_chance;

  for letter in letters {
    if random_value < letter.weight() {
      return letter;
    }
    random_value -= letter.weight();
  }

  &letters[letters.len() - 1]
}

fn select_word_size(from: u8, to:u8) -> u8 {
  let size = to - from + 1;
  let random_value = rand::random::<f64>() * (size as f64);
  random_value.floor() as u8 + from
}

fn gen_syllable(inv: &Inventory) -> String {
  let onset = select_random_letter(&inv.onsets);
  let vowel = select_random_letter(&inv.vowels);
  let coda = select_random_letter(&inv.codas);

  format!("{}{}{}", onset.romanization(), vowel.romanization(), coda.romanization())
} 

fn gen_word(inv: &Inventory, size: usize) -> String {
  let mut word = String::new();
  for _ in 0..size {
    let syllable = gen_syllable(inv);
    word.push_str(&syllable);
  }

  word
}

fn main() {
  const VOWEL_FILE: &str = "vowels.csv";
  const ONSET_FILE: &str = "onsets.csv";
  const CODA_FILE: &str = "coda.csv";
  const GEN_SIZE: usize = 100;
  const MIN_WORD_SIZE: u8 = 1;
  const MAX_WORD_SIZE: u8 = 3;

  // read in each of the phoneme files
  let inv = Inventory {
    vowels: Letter::from_file(VOWEL_FILE)
      .expect("Failed to read vowels.csv"),
    onsets: Letter::from_file(ONSET_FILE)
      .expect("Failed to read onsets.csv"),
    codas: Letter::from_file(CODA_FILE)
      .expect("Failed to read codas.csv"),
  };

  for _ in 1..=GEN_SIZE {
    let word_size = select_word_size(MIN_WORD_SIZE, MAX_WORD_SIZE);
    let word = gen_word(&inv, word_size as usize);
    println!("{}", word);
  }

}
