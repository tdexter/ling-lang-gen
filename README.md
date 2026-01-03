
# Linguistic Language Generator
A basic word generator for a set of language restrictions for a constructed language.


## Features

### Arguments
* count: Number of words to generate
* min: minimum syllable count per word
* max: maximum syllable count per word

### Configuration
Separate configuration files for onsets.csv, vowels.csv and codas.csv
Each contains the following header line:

"romanization","weight","initial_weight","final_weight"

initial_weight and final_weight are optional and are used for the first and last syllables respectively. If not given, weight is used.

Onset to vowel mapping is contained in onset-vowels.csv with the following header line:
"onset","vowel"

Onsets and vowels must match the romanization of an entry in onsets.csv and vowels.csv respectively.


## License
The MIT License

