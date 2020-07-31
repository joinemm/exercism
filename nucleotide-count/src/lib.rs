use std::collections::HashMap;

const NUCLEOTIDES: &[char; 4] = &['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    let mut count: usize = 0;
    for c in dna.chars() {
        if NUCLEOTIDES.contains(&c) {
            if c == nucleotide {
                count += 1;
            }
        } else {
            return Err(c);
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::<char, usize>::with_capacity(4);
    for nucleotide in NUCLEOTIDES {
        match count(*nucleotide, dna) {
            Ok(count) => {
                result.insert(*nucleotide, count);
            }
            Err(c) => {
                return Err(c);
            }
        }
    }
    Ok(result)
}
