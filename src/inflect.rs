use std::collections::{ HashMap, HashSet };

/// Encloses a string 's' in a non-capturing group.
pub fn enclose(s: &str) -> String {
    format!("(?:{})", s)
}

/// Joins the stem of each word in 'words' into a string for Regex.
pub fn joinstem(cutpoint: Option<i32>, words: Option<Vec<&str>>) -> String {
    let words = words.unwrap_or_else(|| Vec::new());
    let stem = words
        .iter()
        .map(|w| {
            if let Some(c) = cutpoint {
                if c < 0 { &w[..w.len() - (-c as usize)] } else { &w[..c as usize] }
            } else {
                w
            }
        })
        .collect::<Vec<&str>>()
        .join("|");
    enclose(&stem)
}

/// From a list of words, returns a HashMap of HashSets of words, keyed by word length.
pub fn bysize(words: Vec<&str>) -> HashMap<usize, HashSet<String>> {
    let mut res: HashMap<usize, HashSet<String>> = HashMap::new();
    for word in words {
        let len = word.len();
        let entry = res.entry(len).or_insert_with(HashSet::new);
        entry.insert(word.to_string());
    }
    res
}
