fn dna_to_rna(dna: &str) -> String {
    // dna.chars().into_iter().map(|c| if c == 'T' { 'U' } else { c }).collect()
    dna.replace("T", "U")
}

#[cfg(test)]
mod tests {
    use super::dna_to_rna;

    #[test]
    fn returns_expected() {
        assert_eq!(dna_to_rna("TTTT"), "UUUU");
        assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }
}