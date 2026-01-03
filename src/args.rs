use clap::Parser;

const MIN_WORD_SIZE: u8= 1;
const MAX_WORD_SIZE: u8 = 3;
const GEN_SIZE: u16 = 20;

/// Generate new words based on phoneme inventories
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Number of words to generate
    #[arg(short, long, default_value_t = GEN_SIZE)]
    count: u16,

    /// minimum syllable count per word
    #[arg(long, default_value_t = MIN_WORD_SIZE)]
    min: u8,
    
    /// maximum syllable count per word
    #[arg(long, default_value_t = MAX_WORD_SIZE)]
    max: u8,
}

impl Args {
    pub fn create() -> Self {
        Args::parse()
    }

    pub fn count(&self) -> u16 {
        self.count
    }

    pub fn min(&self) -> u8 {
        self.min
    }

    pub fn max(&self) -> u8 {
        self.max
    }
}

// no logic to test
