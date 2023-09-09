use std::collections::{ HashMap, HashSet };

/// Encloses a string 's' in a non-capturing group.
pub fn enclose(s: &str) -> String {
    format!("(?:{})", s)
}

/// Joins the stem of each word in 'words' into a string for Regex.
pub fn joinstem(cutpoint: Option<i32>, words: Option<Vec<String>>) -> String {
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
pub fn bysize(words: Vec<String>) -> HashMap<usize, HashSet<String>> {
    let mut res: HashMap<usize, HashSet<String>> = HashMap::new();
    for word in words {
        let len = word.len();
        let entry = res.entry(len).or_insert_with(HashSet::new);
        entry.insert(word.to_string());
    }
    res
}

pub fn make_pl_si_lists(
    list: Vec<String>,
    pl_ending: &str,
    si_ending_size: Option<i32>,
    do_joinstem: bool
) -> (Vec<String>, HashMap<usize, HashSet<String>>, HashMap<usize, HashSet<String>>, String) {
    let si_ending_size = si_ending_size.map(|size| -size);
    let si_list: Vec<String> = list
        .iter()
        .map(|w| {
            if let Some(size) = si_ending_size {
                format!("{}{}", &w[..w.len() - (size as usize)], pl_ending)
            } else {
                format!("{}{}", w, pl_ending)
            }
        })
        .collect();
    let pl_bysize = bysize(list.clone());
    let si_bysize = bysize(si_list.clone());
    if do_joinstem {
        let stem = joinstem(si_ending_size, Some(list));
        (si_list, si_bysize, pl_bysize, stem)
    } else {
        (si_list, si_bysize, pl_bysize, String::new())
    }
}

fn pl_sb_irregular_s() -> HashMap<&'static str, &'static str> {
    return vec![
        ("corpus", "corpuses|corpora"),
        ("opus", "opuses|opera"),
        ("genus", "genera"),
        ("mythos", "mythoi"),
        ("penis", "penises|penes"),
        ("testis", "testes"),
        ("atlas", "atlases|atlantes"),
        ("yes", "yeses")
    ]
        .into_iter()
        .collect();
}

fn pl_sb_irregular() -> HashMap<&'static str, &'static str> {
    let mut pl_sb_irregular: HashMap<&str, &str> = vec![
        ("child", "children"),
        ("chili", "chilis|chilies"),
        ("brother", "brothers|brethren"),
        ("infinity", "infinities|infinity"),
        ("loaf", "loaves"),
        ("lore", "lores|lore"),
        ("hoof", "hoofs|hooves"),
        ("beef", "beefs|beeves"),
        ("thief", "thiefs|thieves"),
        ("money", "monies"),
        ("mongoose", "mongooses"),
        ("ox", "oxen"),
        ("cow", "cows|kine"),
        ("graffito", "graffiti"),
        ("octopus", "octopuses|octopodes"),
        ("genie", "genies|genii"),
        ("ganglion", "ganglions|ganglia"),
        ("trilby", "trilbys"),
        ("turf", "turfs|turves"),
        ("numen", "numina"),
        ("atman", "atmas"),
        ("occiput", "occiputs|occipita"),
        ("sabretooth", "sabretooths"),
        ("sabertooth", "sabertooths"),
        ("lowlife", "lowlifes"),
        ("flatfoot", "flatfoots"),
        ("tenderfoot", "tenderfoots"),
        ("romany", "romanies"),
        ("jerry", "jerries"),
        ("mary", "maries"),
        ("talouse", "talouses"),
        ("rom", "roma"),
        ("carmen", "carmina")
    ]
        .into_iter()
        .collect();
    pl_sb_irregular.extend(pl_sb_irregular_s());
    pl_sb_irregular
}

fn pl_sb_irregular_caps() -> HashMap<&'static str, &'static str> {
    return vec![("Romany", "Romanies"), ("Jerry", "Jerrys"), ("Mary", "Marys"), ("Rom", "Roma")]
        .into_iter()
        .collect();
}

fn pl_sb_irregular_compound() -> HashMap<&'static str, &'static str> {
    return vec![("prima donna", "prima donnas|prime donne")].into_iter().collect();
}

fn si_sb_irregular() -> HashMap<&'static str, &'static str> {
    let mut si_sb_irregular: HashMap<&str, &str> = pl_sb_irregular()
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect();
    let mut keys_to_remove = Vec::new();
    for &k in si_sb_irregular.keys() {
        if k.contains('|') {
            keys_to_remove.push(k);
        }
    }
    for k in keys_to_remove {
        si_sb_irregular.remove(&k);
        let (k1, k2) = k.split_once('|').unwrap();
        si_sb_irregular.insert(k1, k);
        si_sb_irregular.insert(k2, k);
    }
    si_sb_irregular
}

fn si_sb_irregular_caps() -> HashMap<&'static str, &'static str> {
    return pl_sb_irregular_caps()
        .iter()
        .map(|(&k, &v)| (v, k))
        .collect();
}

fn si_sb_irregular_compound() -> HashMap<&'static str, &'static str> {
    let mut si_sb_irregular_compound: HashMap<&str, &str> = pl_sb_irregular_compound()
        .iter()
        .map(|(&k, &v)| (v, k))
        .collect();
    let mut keys_to_remove = Vec::new();
    for &k in si_sb_irregular_compound.keys() {
        if k.contains('|') {
            keys_to_remove.push(k);
        }
    }
    for k in keys_to_remove {
        si_sb_irregular_compound.remove(&k);
        let (k1, k2) = k.split_once('|').unwrap();
        si_sb_irregular_compound.insert(k1, k);
        si_sb_irregular_compound.insert(k2, k);
    }
    si_sb_irregular_compound
}

fn pl_sb_z_zes_list() -> Vec<String> {
    return vec!["quartz", "topaz"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn pl_sb_z_zes_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(pl_sb_z_zes_list());
}

fn sb_ze_zes_list() -> Vec<String> {
    return vec!["snooze"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn sb_ze_zes_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(sb_ze_zes_list());
}

fn pl_sb_c_is_ides_complete() -> Vec<String> {
    return vec!["ephemeris", "iris", "clitoris", "chrysalis", "epididymis"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn pl_sb_c_is_ides_endings() -> Vec<String> {
    return vec!["itis"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn pl_sb_c_is_ides() -> String {
    let endings = pl_sb_c_is_ides_endings()
        .into_iter()
        .map(|w| format!(".*{}", w));
    let pl_sb_c_is_ides: Vec<String> = pl_sb_c_is_ides_complete()
        .iter()
        .map(|s| s.to_string())
        .chain(endings)
        .collect();
    return joinstem(Some(-2), Some(pl_sb_c_is_ides));
}

fn pl_sb_c_is_ides_list() -> Vec<String> {
    let mut pl_sb_c_is_ides_complete = pl_sb_c_is_ides_complete();
    pl_sb_c_is_ides_complete.append(&mut pl_sb_c_is_ides_endings());
    pl_sb_c_is_ides_complete
}

fn si_sb_c_is_ides_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_is_ides_list(), "ides", Some(2), true).0;
}

fn si_sb_c_is_ides_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_is_ides_list(), "ides", Some(2), true).1;
}

fn pl_sb_c_is_ides_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_is_ides_list(), "ides", Some(2), true).2;
}

fn pl_sb_c_a_ata_list() -> Vec<String> {
    return vec![
        "anathema",
        "bema",
        "carcinoma",
        "charisma",
        "diploma",
        "dogma",
        "drama",
        "edema",
        "enema",
        "enigma",
        "lemma",
        "lymphoma",
        "magma",
        "melisma",
        "miasma",
        "oedema",
        "sarcoma",
        "schema",
        "soma",
        "stigma",
        "stoma",
        "trauma",
        "gumma",
        "pragma"
    ]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_c_a_ata_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_a_ata_list(), "ata", Some(1), false).0;
}

fn si_sb_c_a_ata_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_a_ata_list(), "ata", Some(1), false).1;
}

fn pl_sb_c_a_ata_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_a_ata_list(), "ata", Some(1), false).2;
}

fn pl_sb_c_a_ata() -> String {
    return make_pl_si_lists(pl_sb_c_a_ata_list(), "ata", Some(1), false).3;
}
