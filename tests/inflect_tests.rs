use inflect::*;

#[test]
fn test_enclose() {
    assert_eq!(enclose("foo"), "(?:foo)");
}

#[test]
fn test_joinstem() {
    assert_eq!(
        joinstem(Some(-2), Some(vec!["ephemeris", "iris", ".*itis"])),
        "(?:ephemer|ir|.*it)"
    );
    assert_eq!(joinstem(None, Some(vec!["ephemeris"])), "(?:ephemeris)");
    assert_eq!(joinstem(Some(5), None), "(?:)");
    assert_eq!(joinstem(None, None), "(?:)");
}

#[test]
fn test_bysize() {
    let words = vec!["ant", "cat", "dog", "pig", "frog", "goat", "horse", "elephant"];
    let result = bysize(words);
    if let Some(set) = result.get(&3) {
        let mut sorted_words: Vec<&String> = set.iter().collect();
        sorted_words.sort();
        assert_eq!(sorted_words, vec!["ant", "cat", "dog", "pig"]);
    }
    if let Some(set) = result.get(&4) {
        let mut sorted_words: Vec<&String> = set.iter().collect();
        sorted_words.sort();
        assert_eq!(sorted_words, vec!["frog", "goat"]);
    }
    if let Some(set) = result.get(&5) {
        let sorted_words: Vec<&String> = set.iter().collect();
        assert_eq!(sorted_words, vec!["horse"]);
    }
}
