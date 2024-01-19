use std::collections::HashMap;

pub fn duplicate_encode(word:&str) -> String {
    let counts = word
        .chars()
        .into_iter()
        .fold(HashMap::new(), |mut acc, current| {
            *acc.entry(current.to_string().to_ascii_uppercase()).or_insert(0) += 1;
            acc
        });

    word.chars()
        .into_iter()
        .map(|c| {
            match counts.get(&c.to_string().to_ascii_uppercase()) {
                Some(x) => if *x > 1 { ")" } else { "(" }
                None => "("
            }
        })
        .collect()
}

#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"),"(((");
    assert_eq!(duplicate_encode("recede"),"()()()");
    assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
    assert_eq!(duplicate_encode("(( @"),"))((");
}