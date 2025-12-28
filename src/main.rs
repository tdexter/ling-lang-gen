
mod args;
mod assembly; 
mod combinations;
mod generator;
mod letter;
mod order;

pub use crate::args::Args;
pub use crate::assembly::Inventory;
pub use crate::generator::Generator;


fn main() {

    let args = Args::create();

    let inventory = Inventory::load_from_files();

    // read in each of the phoneme files
    let gennie = Generator::new(&inventory);
    for _ in 1..=args.count() {
      let word_size = gennie.select_word_size(args.min(), args.max());
      let word = gennie.gen_word(word_size as usize);
      println!("{}", word);
    }

}
