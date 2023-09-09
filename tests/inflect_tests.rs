use inflect::*;

#[test]
fn test_enclose() {
    assert_eq!(enclose("foo"), "(?:foo)");
}

#[test]
fn test_joinstem() {
    assert_eq!(
        joinstem(
            Some(-2),
            Some(vec!["ephemeris".to_string(), "iris".to_string(), ".*itis".to_string()])
        ),
        "(?:ephemer|ir|.*it)"
    );
    assert_eq!(joinstem(None, Some(vec!["ephemeris".to_string()])), "(?:ephemeris)");
    assert_eq!(joinstem(Some(5), None), "(?:)");
    assert_eq!(joinstem(None, None), "(?:)");
}

#[test]
fn test_bysize() {
    let words = vec!["ant", "cat", "dog", "pig", "frog", "goat", "horse", "elephant"]
        .iter()
        .map(|s| s.to_string())
        .collect();
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

#[test]
fn test_si_pron() {
    assert_eq!("him", get_si_pron("acc", "them", Some("masculine")));
    assert_eq!("her", get_si_pron("acc", "them", Some("feminine")));
    assert_eq!("it", get_si_pron("acc", "them", Some("neuter")));
    assert_eq!("themselves", get_si_pron("acc", "itself", None));
    assert_ne!("him", get_si_pron("acc", "them", Some("feminine")));
    assert_ne!("her", get_si_pron("acc", "them", Some("masculine")));
}
