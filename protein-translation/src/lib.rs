// Needs a lot of memory, but should be fast after initialization due to simple HashMap lookup.

use std::collections::HashMap;

type Codon = (char, char, char);
type CodonName<'a> = &'a str;

pub struct CodonDB<'a> {
    name_by_codon: HashMap<Codon, CodonName<'a>>
}

impl<'a> CodonDB<'a> {
    pub fn name_for(&self, codon_str: &str) -> Result<CodonName, ()> {
        to_codon(codon_str)
            .and_then(|codon|
                self.name_by_codon.get(&codon)
                    .map(|&str| str)
                    .ok_or(())
            )
    }
}

static VALID_INPUT_CHARS: &'static str = "ACGTUWSMKRYBDHVN";

fn to_codon(codon: &str) -> Result<Codon, ()> {
    if codon.len() != 3 {
        return Err(());
    }

    let valid_chars: Vec<char> = codon.chars()
        .filter(|&c| VALID_INPUT_CHARS.chars().any(|vc| vc == c)).collect();

    if valid_chars.len() != 3 {
        return Err(());
    }

    Ok((valid_chars[0], valid_chars[1], valid_chars[2]))
}

fn expand<'a>(nucleotide: char) -> &'a str {
    // https://en.wikipedia.org/wiki/Nucleic_acid_notation
    match nucleotide {
        'A' => "AWMRDHVN",
        'C' => "CSMYBHVN",
        'G' => "GSKRBDVN",
        'T' => "TWKYBDHN",
        _ => panic!("invalid nucleotide")
    }
}

pub fn parse<'a>(pairs: Vec<(&'static str, CodonName<'a>)>) -> CodonDB<'a> {
    let mut name_by_codon: HashMap<Codon, CodonName> = HashMap::new();
    for (codon_str, name) in pairs {
        let codon = to_codon(codon_str).unwrap();
        for c0 in expand(codon.0).chars() {
            for c1 in expand(codon.1).chars() {
                for c2 in expand(codon.2).chars() {
                    name_by_codon.insert((c0, c1, c2), name);
                }
            }
        }
    }
    CodonDB { name_by_codon: name_by_codon }
}
