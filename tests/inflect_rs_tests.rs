use inflect_rs::*;

#[test]
fn test_enclose() {
    assert_eq!(enclose("foo"), "(?:foo)");
}

#[test]
fn test_joinstem() {
    assert_eq!(
        joinstem(
            Some(-2),
            Some(vec![
                "ephemeris".to_string(),
                "iris".to_string(),
                ".*itis".to_string()
            ])
        ),
        "(?:ephemer|ir|.*it)"
    );
    assert_eq!(
        joinstem(None, Some(vec!["ephemeris".to_string()])),
        "(?:ephemeris)"
    );
    assert_eq!(joinstem(Some(5), None), "(?:)");
    assert_eq!(joinstem(None, None), "(?:)");
}

#[test]
fn test_bysize() {
    let words = vec![
        "ant", "cat", "dog", "pig", "frog", "goat", "horse", "elephant",
    ]
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
    assert_eq!("you", get_si_pron("acc", "you", None));
    assert_eq!("themselves", get_si_pron("acc", "itself", None));

    assert_ne!("him", get_si_pron("acc", "them", Some("feminine")));
    assert_ne!("her", get_si_pron("acc", "them", Some("masculine")));
}

#[test]
fn test_ordinal_suff() {
    let pattern = "ty|one|two|three|five|eight|nine|twelve";
    let re = ordinal_suff();
    assert!(re.is_match(pattern));
    assert!(re.find("one").is_some());
    assert!(re.find("thre|e").is_none());
    assert!(re.find("1").is_none());
}

#[test]
fn test_words() {
    let words = Words::new("A quick brown fox.");
    assert_eq!(words.lowered, "a quick brown fox.");
    assert_eq!(words.split_, vec!["A", "quick", "brown", "fox."]);
    assert_eq!(words.first, "A");
    assert_eq!(words.last, "fox.");
}

#[test]
fn test_word() {
    let word = Word::new(String::from("fox"));
    assert_eq!(word.expect("Failed to unwrap Word").get(), "fox");
}

#[test]
fn test_engine_gender() {
    let mut e = Engine::new();
    assert_eq!(e.check_gender(), "neuter");
    e.gender("masculine");
    assert_eq!(e.check_gender(), "masculine");
    e.gender("fff");
    assert_ne!(e.check_gender(), "fff");
    assert_eq!(e.check_gender(), "masculine");
}

#[test]
fn test_engine_get_count() {
    let mut e = Engine::new();

    // i32 Into<IntOrString>>
    for i in -1e2 as i32..1e2 as i32 {
        assert_eq!(e.get_count(Some(i)), i);
    }

    // None cases
    assert_eq!(e.get_count::<i32>(None), 0);
    assert_eq!(e.get_count::<String>(None), 0);
    assert_eq!(e.get_count::<&str>(None), 0);

    // Strings
    assert_eq!(e.get_count(Some("a".to_string())), 1);
    assert_eq!(e.get_count(Some("nil")), 2);
    assert_eq!(e.get_count(Some("some")), 2);

    e.classical_dict = all_classical();
    assert_eq!(e.get_count(Some("nil")), 1);

    e.persistent_count = Some(3);
    assert_eq!(e.get_count::<i32>(None), 3);
}
