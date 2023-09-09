# inflect-rs

![ci](https://github.com/Llywelwyn/inflect_rs/actions/workflows/cargo-build-test.yml/badge.svg)

inflect-rs is a Rust port of the Python inflect library, used to generate plurals, ordinals, indefinite articles, and to convert numbers to words.

```rust
fn test_si_pron() {
    assert_eq!("him", get_si_pron("acc", "them", Some("masculine")));
    assert_eq!("her", get_si_pron("acc", "them", Some("feminine")));
    assert_eq!("it", get_si_pron("acc", "them", Some("neuter")));
    assert_eq!("you", get_si_pron("acc", "you", None));
    assert_eq!("themselves", get_si_pron("acc", "itself", None));

    assert_ne!("him", get_si_pron("acc", "them", Some("feminine")));
    assert_ne!("her", get_si_pron("acc", "them", Some("masculine")));
}
```

inflect-rs is currently in-development, but is nearing completion, and so has been published for the sake of name-reservation. The first stable version should be here very soon.