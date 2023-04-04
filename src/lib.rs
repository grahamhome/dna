use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .remove(&nucleotide)
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result: HashMap<char, usize> = "GATC".chars().map(|n| (n, 0)).collect();
    for n in dna.chars() {
        result.get_mut(&n).map(|count| *count += 1).ok_or(n)?;
    }
    Ok(result)
}
