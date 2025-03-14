use regex::Regex;
use std::collections::{HashMap, HashSet};

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
                if c < 0 {
                    &w[..w.len() - (-c as usize)]
                } else {
                    &w[..c as usize]
                }
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
    do_joinstem: bool,
) -> (
    Vec<String>,
    HashMap<usize, HashSet<String>>,
    HashMap<usize, HashSet<String>>,
    String,
) {
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

fn pl_sb_irregular_s() -> HashMap<String, String> {
    return vec![
        ("corpus", "corpuses|corpora"),
        ("opus", "opuses|opera"),
        ("genus", "genera"),
        ("mythos", "mythoi"),
        ("penis", "penises|penes"),
        ("testis", "testes"),
        ("atlas", "atlases|atlantes"),
        ("yes", "yeses"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();
}

fn pl_sb_irregular() -> HashMap<String, String> {
    let mut pl_sb_irregular: HashMap<String, String> = vec![
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
        ("carmen", "carmina"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();
    pl_sb_irregular.extend(pl_sb_irregular_s());
    pl_sb_irregular
}

fn pl_sb_irregular_caps() -> HashMap<&'static str, &'static str> {
    return vec![
        ("Romany", "Romanies"),
        ("Jerry", "Jerrys"),
        ("Mary", "Marys"),
        ("Rom", "Roma"),
    ]
    .into_iter()
    .collect();
}

fn pl_sb_irregular_compound() -> HashMap<&'static str, &'static str> {
    return vec![("prima donna", "prima donnas|prime donne")]
        .into_iter()
        .collect();
}

fn si_sb_irregular() -> HashMap<String, String> {
    let mut si_sb_irregular: HashMap<String, String> =
        pl_sb_irregular().into_iter().map(|(k, v)| (v, k)).collect();
    let mut keys_to_remove = Vec::new();
    let keys: Vec<String> = si_sb_irregular.keys().cloned().collect();
    for k in keys.iter() {
        if k.contains('|') {
            keys_to_remove.push(k);
        }
    }
    for k in keys_to_remove {
        si_sb_irregular.remove(k);
        let (k1, k2) = k.split_once('|').unwrap();
        si_sb_irregular.insert(k1.to_string(), k.clone());
        si_sb_irregular.insert(k2.to_string(), k.clone());
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
    return vec!["snooze"].iter().map(|s| s.to_string()).collect();
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
    return vec!["itis"].iter().map(|s| s.to_string()).collect();
}

fn pl_sb_c_is_ides() -> String {
    let pl_sb_c_is_ides: Vec<String> = pl_sb_c_is_ides_complete()
        .iter()
        .map(|s| s.to_string())
        .chain(
            pl_sb_c_is_ides_endings()
                .into_iter()
                .map(|w| format!(".*{}", w)),
        )
        .collect();
    return joinstem(Some(-2), Some(pl_sb_c_is_ides));
}

fn pl_sb_c_is_ides_list() -> Vec<String> {
    let mut pl_sb_c_is_ides_complete = pl_sb_c_is_ides_complete();
    pl_sb_c_is_ides_complete.append(&mut pl_sb_c_is_ides_endings());
    pl_sb_c_is_ides_complete
}

fn si_sb_c_is_ides_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_is_ides_list(), "ides", Some(2), false).0;
}

fn si_sb_c_is_ides_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_is_ides_list(), "ides", Some(2), false).1;
}

fn pl_sb_c_is_ides_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_is_ides_list(), "ides", Some(2), false).2;
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
        "pragma",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_c_a_ata_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_a_ata_list(), "ata", Some(1), true).0;
}

fn si_sb_c_a_ata_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_a_ata_list(), "ata", Some(1), true).1;
}

fn pl_sb_c_a_ata_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_a_ata_list(), "ata", Some(1), true).2;
}

fn pl_sb_c_a_ata() -> String {
    return make_pl_si_lists(pl_sb_c_a_ata_list(), "ata", Some(1), true).3;
}

fn pl_sb_u_a_ae_list() -> Vec<String> {
    return vec!["alumna", "alga", "vertebra", "persona", "vita"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_u_a_ae_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_a_ae_list(), "e", None, true).0;
}

fn si_sb_u_a_ae_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_a_ae_list(), "e", None, true).1;
}

fn pl_sb_u_a_ae_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_a_ae_list(), "e", None, true).2;
}

fn pl_sb_u_a_ae() -> String {
    return make_pl_si_lists(pl_sb_u_a_ae_list(), "e", None, true).3;
}

fn pl_sb_c_a_ae_list() -> Vec<String> {
    return vec![
        "amoeba",
        "antenna",
        "formula",
        "hyperbola",
        "medusa",
        "nebula",
        "parabola",
        "abscissa",
        "hydra",
        "nova",
        "lacuna",
        "aurora",
        "umbra",
        "flora",
        "fauna",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_c_a_ae_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_a_ae_list(), "e", None, true).0;
}

fn si_sb_c_a_ae_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_a_ae_list(), "e", None, true).1;
}

fn pl_sb_c_a_ae_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_a_ae_list(), "e", None, true).2;
}

fn pl_sb_c_a_ae() -> String {
    return make_pl_si_lists(pl_sb_c_a_ae_list(), "e", None, true).3;
}

fn pl_sb_c_en_ina_list() -> Vec<String> {
    return vec!["stamen", "foramen", "lumen"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_c_en_ina_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_en_ina_list(), "ina", Some(2), true).0;
}

fn si_sb_c_en_ina_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_en_ina_list(), "ina", Some(2), true).1;
}

fn pl_sb_c_en_ina_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_en_ina_list(), "ina", Some(2), true).2;
}

fn pl_sb_c_en_ina() -> String {
    return make_pl_si_lists(pl_sb_c_en_ina_list(), "ina", Some(2), true).3;
}

fn pl_sb_u_um_a_list() -> Vec<String> {
    return vec![
        "bacterium",
        "agendum",
        "desideratum",
        "erratum",
        "stratum",
        "datum",
        "ovum",
        "extremum",
        "candelabrum",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_u_um_a_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_um_a_list(), "a", Some(2), true).0;
}

fn si_sb_u_um_a_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_um_a_list(), "a", Some(2), true).1;
}

fn pl_sb_u_um_a_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_um_a_list(), "a", Some(2), true).2;
}

fn pl_sb_u_um_a() -> String {
    return make_pl_si_lists(pl_sb_u_um_a_list(), "a", Some(2), true).3;
}

fn pl_sb_c_um_a_list() -> Vec<String> {
    return vec![
        "maximum",
        "minimum",
        "momentum",
        "optimum",
        "quantum",
        "cranium",
        "curriculum",
        "dictum",
        "phylum",
        "aquarium",
        "compendium",
        "emporium",
        "encomium",
        "gymnasium",
        "honorarium",
        "interregnum",
        "lustrum",
        "memorandum",
        "millennium",
        "rostrum",
        "spectrum",
        "speculum",
        "stadium",
        "trapezium",
        "ultimatum",
        "medium",
        "vacuum",
        "velum",
        "consortium",
        "arboretum",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_c_um_a_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_um_a_list(), "a", Some(2), true).0;
}

fn si_sb_c_um_a_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_um_a_list(), "a", Some(2), true).1;
}

fn pl_sb_c_um_a_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_um_a_list(), "a", Some(2), true).2;
}

fn pl_sb_c_um_a() -> String {
    return make_pl_si_lists(pl_sb_c_um_a_list(), "a", Some(2), true).3;
}

fn pl_sb_u_us_i_list() -> Vec<String> {
    return vec![
        "alumnus",
        "alveolus",
        "bacillus",
        "bronchus",
        "locus",
        "nucleus",
        "stimulus",
        "meniscus",
        "sarcophagus",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_u_us_i_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_us_i_list(), "i", Some(2), true).0;
}

fn si_sb_u_us_i_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_us_i_list(), "i", Some(2), true).1;
}

fn pl_sb_u_us_i_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_us_i_list(), "i", Some(2), true).2;
}

fn pl_sb_u_us_i() -> String {
    return make_pl_si_lists(pl_sb_u_us_i_list(), "i", Some(2), true).3;
}

fn pl_sb_c_us_i_list() -> Vec<String> {
    return vec![
        "focus",
        "radius",
        "genius",
        "incubus",
        "succubus",
        "nimbus",
        "fungus",
        "nucleolus",
        "stylus",
        "torus",
        "umbilicus",
        "uterus",
        "hippopotamus",
        "cactus",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_c_us_i_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_us_i_list(), "i", Some(2), true).0;
}

fn si_sb_c_us_i_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_us_i_list(), "i", Some(2), true).1;
}

fn pl_sb_c_us_i_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_us_i_list(), "i", Some(2), true).2;
}

fn pl_sb_c_us_i() -> String {
    return make_pl_si_lists(pl_sb_c_us_i_list(), "i", Some(2), true).3;
}

fn pl_sb_c_us_us() -> Vec<String> {
    return vec![
        "status",
        "apparatus",
        "prospectus",
        "sinus",
        "hiatus",
        "impetus",
        "plexus",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn pl_sb_c_us_us_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(pl_sb_c_us_us());
}

fn pl_sb_u_on_a_list() -> Vec<String> {
    return vec![
        "criterion",
        "perihelion",
        "aphelion",
        "phenomenon",
        "prolegomenon",
        "noumenon",
        "organon",
        "asyndeton",
        "hyperbaton",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_u_on_a_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_on_a_list(), "a", Some(2), true).0;
}

fn si_sb_u_on_a_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_on_a_list(), "a", Some(2), true).1;
}

fn pl_sb_u_on_a_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_on_a_list(), "a", Some(2), true).2;
}

fn pl_sb_u_on_a() -> String {
    return make_pl_si_lists(pl_sb_u_on_a_list(), "a", Some(2), true).3;
}

fn pl_sb_c_on_a_list() -> Vec<String> {
    return vec!["oxymoron"].iter().map(|s| s.to_string()).collect();
}

fn si_sb_c_on_a_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_on_a_list(), "a", Some(2), true).0;
}

fn si_sb_c_on_a_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_on_a_list(), "a", Some(2), true).1;
}

fn pl_sb_c_on_a_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_on_a_list(), "a", Some(2), true).2;
}

fn pl_sb_c_on_a() -> String {
    return make_pl_si_lists(pl_sb_c_on_a_list(), "a", Some(2), true).3;
}

fn pl_sb_c_o_i() -> Vec<String> {
    return vec![
        "solo",
        "soprano",
        "basso",
        "alto",
        "contralto",
        "tempo",
        "piano",
        "virtuoso",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn pl_sb_c_o_i_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(pl_sb_c_o_i());
}

fn si_sb_c_o_i_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(
        pl_sb_c_o_i()
            .iter()
            .map(|w| format!("{}i", &w[..w.len() - 1]))
            .collect(),
    );
}

fn pl_sb_c_o_i_stems() -> String {
    return joinstem(Some(-1), Some(pl_sb_c_o_i()));
}

fn pl_sb_u_o_os_complete() -> Vec<String> {
    return vec!["ado", "ISO", "NATO", "NCO", "NGO", "oto"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_u_o_os_complete() -> Vec<String> {
    return pl_sb_u_o_os_complete()
        .iter()
        .map(|w| format!("{}s", w))
        .collect();
}

fn pl_sb_u_o_os_endings() -> Vec<String> {
    let mut pl_sb_u_o_os_endings: Vec<String> = vec![
        "aficionado",
        "aggro",
        "albino",
        "allegro",
        "ammo",
        "Antananarivo",
        "archipelago",
        "armadillo",
        "auto",
        "avocado",
        "Bamako",
        "Barquisimeto",
        "bimbo",
        "bingo",
        "Biro",
        "bolero",
        "Bolzano",
        "bongo",
        "Boto",
        "burro",
        "Cairo",
        "canto",
        "cappuccino",
        "casino",
        "cello",
        "Chicago",
        "Chimango",
        "cilantro",
        "cochito",
        "coco",
        "Colombo",
        "Colorado",
        "commando",
        "concertino",
        "contango",
        "credo",
        "crescendo",
        "cyano",
        "demo",
        "ditto",
        "Draco",
        "dynamo",
        "embryo",
        "Esperanto",
        "espresso",
        "euro",
        "falsetto",
        "Faro",
        "fiasco",
        "Filipino",
        "flamenco",
        "furioso",
        "generalissimo",
        "Gestapo",
        "ghetto",
        "gigolo",
        "gizmo",
        "Greensboro",
        "gringo",
        "Guaiabero",
        "guano",
        "gumbo",
        "gyro",
        "hairdo",
        "hippo",
        "Idaho",
        "impetigo",
        "inferno",
        "info",
        "intermezzo",
        "intertrigo",
        "Iquico",
        "jumbo",
        "junto",
        "Kakapo",
        "kilo",
        "Kinkimavo",
        "Kokako",
        "Kosovo",
        "Lesotho",
        "libero",
        "libido",
        "libretto",
        "lido",
        "Lilo",
        "limbo",
        "limo",
        "lineno",
        "lingo",
        "lino",
        "livedo",
        "loco",
        "logo",
        "lumbago",
        "macho",
        "macro",
        "mafioso",
        "magneto",
        "magnifico",
        "Majuro",
        "Malabo",
        "manifesto",
        "Maputo",
        "Maracaibo",
        "medico",
        "memo",
        "metro",
        "Mexico",
        "micro",
        "Milano",
        "Monaco",
        "mono",
        "Montenegro",
        "Morocco",
        "Muqdisho",
        "myo",
        "neutrino",
        "Ningbo",
        "octavo",
        "oregano",
        "Orinoco",
        "Orlando",
        "Oslo",
        "panto",
        "Paramaribo",
        "Pardusco",
        "pedalo",
        "photo",
        "pimento",
        "pinto",
        "pleco",
        "Pluto",
        "pogo",
        "polo",
        "poncho",
        "Porto-Novo",
        "Porto",
        "pro",
        "psycho",
        "pueblo",
        "quarto",
        "Quito",
        "repo",
        "rhino",
        "risotto",
        "rococo",
        "rondo",
        "Sacramento",
        "saddo",
        "sago",
        "salvo",
        "Santiago",
        "Sapporo",
        "Sarajevo",
        "scherzando",
        "scherzo",
        "silo",
        "sirocco",
        "sombrero",
        "staccato",
        "sterno",
        "stucco",
        "stylo",
        "sumo",
        "Taiko",
        "techno",
        "terrazzo",
        "testudo",
        "timpano",
        "tiro",
        "tobacco",
        "Togo",
        "Tokyo",
        "torero",
        "Torino",
        "Toronto",
        "torso",
        "tremolo",
        "typo",
        "tyro",
        "ufo",
        "UNESCO",
        "vaquero",
        "vermicello",
        "verso",
        "vibrato",
        "violoncello",
        "Virgo",
        "weirdo",
        "WHO",
        "WTO",
        "Yamoussoukro",
        "yo-yo",
        "zero",
        "Zibo",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    pl_sb_u_o_os_endings.extend(pl_sb_c_o_i());
    pl_sb_u_o_os_endings
}

fn pl_sb_u_o_os_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(pl_sb_u_o_os_endings());
}

fn si_sb_u_o_os_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(
        pl_sb_u_o_os_endings()
            .iter()
            .map(|w| format!("{}s", w))
            .collect(),
    );
}

fn pl_sb_u_ch_chs_list() -> Vec<String> {
    return vec!["czech", "eunuch", "stomach"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_u_ch_chs_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_ch_chs_list(), "s", None, true).0;
}

fn si_sb_u_ch_chs_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_ch_chs_list(), "s", None, true).1;
}

fn pl_sb_u_ch_chs_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_ch_chs_list(), "s", None, true).2;
}

fn pl_sb_u_ch_chs() -> String {
    return make_pl_si_lists(pl_sb_u_ch_chs_list(), "s", None, true).3;
}

fn pl_sb_u_ex_ices_list() -> Vec<String> {
    return vec!["codex", "murex", "silex"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_u_ex_ices_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_ex_ices_list(), "ices", Some(2), true).0;
}

fn si_sb_u_ex_ices_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_ex_ices_list(), "ices", Some(2), true).1;
}

fn pl_sb_u_ex_ices_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_ex_ices_list(), "ices", Some(2), true).2;
}

fn pl_sb_u_ex_ices() -> String {
    return make_pl_si_lists(pl_sb_u_ex_ices_list(), "ices", Some(2), true).3;
}

fn pl_sb_u_ix_ices_list() -> Vec<String> {
    return vec!["radix", "helix"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_u_ix_ices_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_ix_ices_list(), "ices", Some(2), true).0;
}

fn si_sb_u_ix_ices_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_ix_ices_list(), "ices", Some(2), true).1;
}

fn pl_sb_u_ix_ices_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_ix_ices_list(), "ices", Some(2), true).2;
}

fn pl_sb_u_ix_ices() -> String {
    return make_pl_si_lists(pl_sb_u_ix_ices_list(), "ices", Some(2), true).3;
}

fn pl_sb_c_ex_ices_list() -> Vec<String> {
    return vec![
        "vortex", "vertex", "cortex", "latex", "pontifex", "apex", "index", "simplex",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_c_ex_ices_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_ex_ices_list(), "ices", Some(2), true).0;
}

fn si_sb_c_ex_ices_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_ex_ices_list(), "ices", Some(2), true).1;
}

fn pl_sb_c_ex_ices_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_ex_ices_list(), "ices", Some(2), true).2;
}

fn pl_sb_c_ex_ices() -> String {
    return make_pl_si_lists(pl_sb_c_ex_ices_list(), "ices", Some(2), true).3;
}

fn pl_sb_c_ix_ices_list() -> Vec<String> {
    return vec!["appendix"].iter().map(|s| s.to_string()).collect();
}

fn si_sb_c_ix_ices_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_ix_ices_list(), "ices", Some(2), true).0;
}

fn si_sb_c_ix_ices_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_ix_ices_list(), "ices", Some(2), true).1;
}

fn pl_sb_c_ix_ices_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_ix_ices_list(), "ices", Some(2), true).2;
}

fn pl_sb_c_ix_ices() -> String {
    return make_pl_si_lists(pl_sb_c_ix_ices_list(), "ices", Some(2), true).3;
}

fn pl_sb_c_i_list() -> Vec<String> {
    return vec!["afreet", "afrit", "efreet"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_c_i_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_i_list(), "i", None, true).0;
}

fn si_sb_c_i_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_i_list(), "i", None, true).1;
}

fn pl_sb_c_i_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_i_list(), "i", None, true).2;
}

fn pl_sb_c_i() -> String {
    return make_pl_si_lists(pl_sb_c_i_list(), "i", None, true).3;
}

fn pl_sb_c_im_list() -> Vec<String> {
    return vec!["goy", "seraph", "cherub"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_c_im_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_c_im_list(), "im", None, true).0;
}

fn si_sb_c_im_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_im_list(), "im", None, true).1;
}

fn pl_sb_c_im_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_c_im_list(), "im", None, true).2;
}

fn pl_sb_c_im() -> String {
    return make_pl_si_lists(pl_sb_c_im_list(), "im", None, true).3;
}

fn pl_sb_u_man_mans_list() -> Vec<String> {
    return vec![
        "ataman", "caiman", "cayman", "ceriman", "desman", "dolman", "farman", "harman", "hetman",
        "human", "leman", "ottoman", "shaman", "talisman",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn pl_sb_u_man_mans_caps_list() -> Vec<String> {
    return vec![
        "Alabaman",
        "Bahaman",
        "Burman",
        "|German",
        "Hiroshiman",
        "Liman",
        "Nakayaman",
        "Norman",
        "Oklahoman",
        "Panaman",
        "Roman",
        "Selman",
        "Sonaman",
        "Tacoman",
        "Yakiman",
        "Yokohaman",
        "Yuman",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_u_man_mans_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_man_mans_list(), "s", None, false).0;
}

fn si_sb_u_man_mans_caps_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_man_mans_caps_list(), "s", None, false).0;
}

fn si_sb_u_man_mans_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_man_mans_list(), "s", None, false).1;
}

fn si_sb_u_man_mans_caps_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_man_mans_caps_list(), "s", None, false).1;
}

fn pl_sb_u_man_mans_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_man_mans_list(), "s", None, false).2;
}

fn pl_sb_u_man_mans_caps_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_man_mans_caps_list(), "s", None, false).2;
}

fn pl_sb_u_louse_lice_list() -> Vec<String> {
    return vec!["booklouse", "grapelouse", "louse", "woodlouse"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_u_louse_lice_list() -> Vec<String> {
    return make_pl_si_lists(pl_sb_u_louse_lice_list(), "lice", Some(5), false).0;
}

fn si_sb_u_louse_lice_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_louse_lice_list(), "lice", Some(5), false).1;
}

fn pl_sb_u_louse_lice_bysize() -> HashMap<usize, HashSet<String>> {
    return make_pl_si_lists(pl_sb_u_louse_lice_list(), "lice", Some(5), false).2;
}

fn pl_sb_uninflected_s_complete() -> Vec<String> {
    return vec![
        "breeches",
        "britches",
        "pajamas",
        "pyjamas",
        "clippers",
        "gallows",
        "hijinks",
        "headquarters",
        "pliers",
        "scissors",
        "testes",
        "herpes",
        "pincers",
        "shears",
        "proceedings",
        "trousers",
        "cantus",
        "coitus",
        "nexus",
        "contretemps",
        "corps",
        "debris",
        "siemens",
        "mumps",
        "diabetes",
        "jackanapes",
        "series",
        "species",
        "subspecies",
        "rabies",
        "chassis",
        "innings",
        "news",
        "mews",
        "haggis",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn pl_sb_uninflected_s_endings() -> Vec<String> {
    return vec!["ois", "measles"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn pl_sb_uninflected_s() -> Vec<String> {
    let mut pl_sb_uninflected_s = pl_sb_uninflected_s_complete();
    pl_sb_uninflected_s.extend(
        pl_sb_uninflected_s_endings()
            .iter()
            .map(|w| format!(".*{}", w)),
    );
    pl_sb_uninflected_s
}

fn pl_sb_uninflected_herd() -> Vec<String> {
    return vec![
        "wildebeest",
        "swine",
        "eland",
        "bison",
        "buffalo",
        "cattle",
        "elk",
        "rhinoceros",
        "zucchini",
        "caribou",
        "dace",
        "grouse",
        "guinea fowl",
        "guinea-fowl",
        "haddock",
        "hake",
        "halibut",
        "herring",
        "mackerel",
        "pickerel",
        "pike",
        "roe",
        "seed",
        "shad",
        "snipe",
        "teal",
        "turbot",
        "water fowl",
        "water-fowl",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn pl_sb_uninflected_complete() -> Vec<String> {
    return vec![
        "tuna",
        "salmon",
        "mackerel",
        "trout",
        "bream",
        "sea-bass",
        "sea bass",
        "carp",
        "cod",
        "flounder",
        "whiting",
        "moose",
        "graffiti",
        "djinn",
        "samuri",
        "offspring",
        "pence",
        "quid",
        "hertz",
    ]
    .iter()
    .map(|s| s.to_string())
    .chain(pl_sb_uninflected_complete())
    .collect();
}

fn pl_sb_uninflected_caps() -> Vec<String> {
    return vec![
        "Portuguese",
        "Amoyese",
        "Borghese",
        "Congoese",
        "Faroese",
        "Foochowese",
        "Genevese",
        "Genoese",
        "Gilbertese",
        "Hottentotese",
        "Kiplingese",
        "Kongoese",
        "Lucchese",
        "Maltese",
        "Nankingese",
        "Niasese",
        "Pekingese",
        "Piedmontese",
        "Pistoiese",
        "Sarawakese",
        "Shavese",
        "Vermontese",
        "Wenchowese",
        "Yengeese",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn pl_sb_uninflected_endings() -> Vec<String> {
    return vec![
        "butter",
        "cash",
        "furniture",
        "information",
        "fish",
        "deer",
        "sheep",
        "nese",
        "rese",
        "lese",
        "mese",
        "pox",
        "craft",
    ]
    .iter()
    .map(|s| s.to_string())
    .chain(pl_sb_uninflected_s_endings())
    .collect();
}

fn pl_sb_uninflected_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(pl_sb_uninflected_endings());
}

fn pl_sb_singular_s_complete() -> Vec<String> {
    return vec![
        "acropolis",
        "aegis",
        "alias",
        "asbestos",
        "bathos",
        "bias",
        "bronchitis",
        "bursitis",
        "caddis",
        "cannabis",
        "canvas",
        "chaos",
        "cosmos",
        "dais",
        "digitalis",
        "epidermis",
        "ethos",
        "eyas",
        "gas",
        "glottis",
        "hubris",
        "ibis",
        "lens",
        "mantis",
        "marquis",
        "metropolis",
        "pathos",
        "pelvis",
        "polis",
        "rhinoceros",
        "sassafras",
        "trellis",
    ]
    .iter()
    .map(|s| s.to_string())
    .chain(pl_sb_c_is_ides_complete())
    .collect();
}

fn pl_sb_singular_s_endings() -> Vec<String> {
    return vec!["ss", "us"]
        .iter()
        .map(|s| s.to_string())
        .chain(pl_sb_c_is_ides_endings())
        .collect();
}

fn pl_sb_singular_s_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(pl_sb_singular_s_endings());
}

fn si_sb_singular_s_complete() -> Vec<String> {
    return pl_sb_singular_s_complete()
        .iter()
        .map(|w| format!("{}es", w))
        .collect();
}

fn si_sb_singular_s_endings() -> Vec<String> {
    return pl_sb_singular_s_endings()
        .iter()
        .map(|w| format!("{}es", w))
        .collect();
}

fn si_sb_singular_s_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(si_sb_singular_s_endings());
}

fn pl_sb_singular_s_es() -> Vec<String> {
    return vec!["[A-Z].*es"].iter().map(|s| s.to_string()).collect();
}

fn pl_sb_singular_s() -> String {
    let mut concat: Vec<String> = Vec::new();
    concat.extend(pl_sb_singular_s_complete().iter().map(|w| w.to_string()));
    concat.extend(
        pl_sb_singular_s_endings()
            .iter()
            .map(|w| format!(".*{}", w)),
    );
    concat.extend(pl_sb_singular_s_es().iter().map(|w| w.to_string()));
    return enclose(&concat.join("|"));
}

fn si_sb_ois_oi_case() -> Vec<String> {
    return vec!["Bolshois", "Hanois"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_uses_use_case() -> Vec<String> {
    return vec!["Betelgeuses", "Duses", "Meuses", "Syracuses", "Toulouses"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_use_uses() -> Vec<String> {
    return vec![
        "abuses",
        "applauses",
        "blouses",
        "carouses",
        "causes",
        "chartreuses",
        "clauses",
        "contuses",
        "douses",
        "excuses",
        "fuses",
        "grouses",
        "hypotenuses",
        "masseuses",
        "menopauses",
        "misuses",
        "muses",
        "overuses",
        "pauses",
        "peruses",
        "profuses",
        "recluses",
        "reuses",
        "ruses",
        "souses",
        "spouses",
        "suffuses",
        "transfuses",
        "uses",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_ies_ie_case() -> Vec<String> {
    return vec![
        "Addies",
        "Aggies",
        "Allies",
        "Amies",
        "Angies",
        "Annies",
        "Annmaries",
        "Archies",
        "Arties",
        "Aussies",
        "Barbies",
        "Barries",
        "Basies",
        "Bennies",
        "Bernies",
        "Berties",
        "Bessies",
        "Betties",
        "Billies",
        "Blondies",
        "Bobbies",
        "Bonnies",
        "Bowies",
        "Brandies",
        "Bries",
        "Brownies",
        "Callies",
        "Carnegies",
        "Carries",
        "Cassies",
        "Charlies",
        "Cheries",
        "Christies",
        "Connies",
        "Curies",
        "Dannies",
        "Debbies",
        "Dixies",
        "Dollies",
        "Donnies",
        "Drambuies",
        "Eddies",
        "Effies",
        "Ellies",
        "Elsies",
        "Eries",
        "Ernies",
        "Essies",
        "Eugenies",
        "Fannies",
        "Flossies",
        "Frankies",
        "Freddies",
        "Gillespies",
        "Goldies",
        "Gracies",
        "Guthries",
        "Hallies",
        "Hatties",
        "Hetties",
        "Hollies",
        "Jackies",
        "Jamies",
        "Janies",
        "Jannies",
        "Jeanies",
        "Jeannies",
        "Jennies",
        "Jessies",
        "Jimmies",
        "Jodies",
        "Johnies",
        "Johnnies",
        "Josies",
        "Julies",
        "Kalgoorlies",
        "Kathies",
        "Katies",
        "Kellies",
        "Kewpies",
        "Kristies",
        "Laramies",
        "Lassies",
        "Lauries",
        "Leslies",
        "Lessies",
        "Lillies",
        "Lizzies",
        "Lonnies",
        "Lories",
        "Lorries",
        "Lotties",
        "Louies",
        "Mackenzies",
        "Maggies",
        "Maisies",
        "Mamies",
        "Marcies",
        "Margies",
        "Maries",
        "Marjories",
        "Matties",
        "McKenzies",
        "Melanies",
        "Mickies",
        "Millies",
        "Minnies",
        "Mollies",
        "Mounties",
        "Nannies",
        "Natalies",
        "Nellies",
        "Netties",
        "Ollies",
        "Ozzies",
        "Pearlies",
        "Pottawatomies",
        "Reggies",
        "Richies",
        "Rickies",
        "Robbies",
        "Ronnies",
        "Rosalies",
        "Rosemaries",
        "Rosies",
        "Roxies",
        "Rushdies",
        "Ruthies",
        "Sadies",
        "Sallies",
        "Sammies",
        "Scotties",
        "Selassies",
        "Sherries",
        "Sophies",
        "Stacies",
        "Stefanies",
        "Stephanies",
        "Stevies",
        "Susies",
        "Sylvies",
        "Tammies",
        "Terries",
        "Tessies",
        "Tommies",
        "Tracies",
        "Trekkies",
        "Valaries",
        "Valeries",
        "Valkyries",
        "Vickies",
        "Virgies",
        "Willies",
        "Winnies",
        "Wylies",
        "Yorkies",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_ies_ie() -> Vec<String> {
    return vec![
        "aeries",
        "baggies",
        "belies",
        "biggies",
        "birdies",
        "bogies",
        "bonnies",
        "boogies",
        "bookies",
        "bourgeoisies",
        "brownies",
        "budgies",
        "caddies",
        "calories",
        "camaraderies",
        "cockamamies",
        "collies",
        "cookies",
        "coolies",
        "cooties",
        "coteries",
        "crappies",
        "curies",
        "cutesies",
        "dogies",
        "eyries",
        "floozies",
        "footsies",
        "freebies",
        "genies",
        "goalies",
        "groupies",
        "hies",
        "jalousies",
        "junkies",
        "kiddies",
        "laddies",
        "lassies",
        "lies",
        "lingeries",
        "magpies",
        "menageries",
        "mommies",
        "movies",
        "neckties",
        "newbies",
        "nighties",
        "oldies",
        "organdies",
        "overlies",
        "pies",
        "pinkies",
        "pixies",
        "potpies",
        "prairies",
        "quickies",
        "reveries",
        "rookies",
        "rotisseries",
        "softies",
        "sorties",
        "species",
        "stymies",
        "sweeties",
        "ties",
        "underlies",
        "unties",
        "veggies",
        "vies",
        "yuppies",
        "zombies",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_oes_oe_case() -> Vec<String> {
    return vec![
        "Chloes",
        "Crusoes",
        "Defoes",
        "Faeroes",
        "Ivanhoes",
        "Joes",
        "McEnroes",
        "Moes",
        "Monroes",
        "Noes",
        "Poes",
        "Roscoes",
        "Tahoes",
        "Tippecanoes",
        "Zoes",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_oes_oe() -> Vec<String> {
    return vec![
        "aloes",
        "backhoes",
        "canoes",
        "does",
        "floes",
        "foes",
        "hoes",
        "mistletoes",
        "oboes",
        "pekoes",
        "roes",
        "sloes",
        "throes",
        "tiptoes",
        "toes",
        "woes",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_z_zes() -> Vec<String> {
    return vec!["quartzes", "topazes"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_zzes_zz() -> Vec<String> {
    return vec!["buzzes", "fizzes", "frizzes", "razzes"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_ches_che_case() -> Vec<String> {
    return vec![
        "Andromaches",
        "Apaches",
        "Blanches",
        "Comanches",
        "Nietzsches",
        "Porsches",
        "Roches",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_ches_che() -> Vec<String> {
    return vec![
        "aches",
        "avalanches",
        "backaches",
        "bellyaches",
        "caches",
        "cloches",
        "creches",
        "douches",
        "earaches",
        "fiches",
        "headaches",
        "heartaches",
        "microfiches",
        "niches",
        "pastiches",
        "psyches",
        "quiches",
        "stomachaches",
        "toothaches",
        "tranches",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_xes_xe() -> Vec<String> {
    return vec!["annexes", "axes", "deluxes", "pickaxes"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_sses_sse_case() -> Vec<String> {
    return vec!["Hesses", "Jesses", "Larousses", "Matisses"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_sses_sse() -> Vec<String> {
    return vec![
        "bouillabaisses",
        "crevasses",
        "demitasses",
        "impasses",
        "mousses",
        "posses",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn si_sb_ves_ve_case() -> Vec<String> {
    return vec!["Clives", "Palmolives"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn si_sb_ves_ve() -> Vec<String> {
    return vec![
        "interweaves",
        "weaves",
        "olives",
        "bivalves",
        "dissolves",
        "resolves",
        "salves",
        "twelves",
        "valves",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn plverb_special_s() -> String {
    let mut concat: Vec<String> = Vec::new();
    concat.push(pl_sb_singular_s());
    concat.extend(pl_sb_uninflected_s());
    let pl_sb_irregular_s_keys: Vec<String> = pl_sb_irregular_s().keys().cloned().collect();
    concat.extend(pl_sb_irregular_s_keys);
    concat.extend(
        vec!["(.*[csx])is", "(.*)ceps", "[A-Z].*s"]
            .iter()
            .map(|s| s.to_string()),
    );
    return enclose(&concat.join("|"));
}

fn _pl_sb_postfix_adj_defn() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert(
        "general".to_string(),
        enclose(r"(?!major|lieutenant|brigadier|adjutant|.*star)\S+"),
    );
    m.insert("martial".to_string(), enclose("court"));
    m.insert("force".to_string(), enclose("pound"));
    m
}

fn pl_sb_postfix_adj() -> Vec<String> {
    return _pl_sb_postfix_adj_defn()
        .iter()
        .map(|(k, v)| format!("{}(?=(?:-|\\s+){})", v, k))
        .collect();
}

fn pl_sb_postfix_adj_stems() -> String {
    return format!("({})(.*)", pl_sb_postfix_adj().join("|"));
}

fn si_sb_es_is() -> Vec<String> {
    return vec![
        "amanuenses",
        "amniocenteses",
        "analyses",
        "antitheses",
        "apotheoses",
        "arterioscleroses",
        "atheroscleroses",
        "axes",
        "catalyses",
        "catharses",
        "chasses",
        "cirrhoses",
        "cocces",
        "crises",
        "diagnoses",
        "dialyses",
        "diereses",
        "electrolyses",
        "emphases",
        "exegeses",
        "geneses",
        "halitoses",
        "hydrolyses",
        "hypnoses",
        "hypotheses",
        "hystereses",
        "metamorphoses",
        "metastases",
        "misdiagnoses",
        "mitoses",
        "mononucleoses",
        "narcoses",
        "necroses",
        "nemeses",
        "neuroses",
        "oases",
        "osmoses",
        "osteoporoses",
        "paralyses",
        "parentheses",
        "parthenogeneses",
        "periphrases",
        "photosyntheses",
        "probosces",
        "prognoses",
        "prophylaxes",
        "prostheses",
        "preces",
        "psoriases",
        "psychoanalyses",
        "psychokineses",
        "psychoses",
        "scleroses",
        "scolioses",
        "sepses",
        "silicoses",
        "symbioses",
        "synopses",
        "syntheses",
        "taxes",
        "telekineses",
        "theses",
        "thromboses",
        "tuberculoses",
        "urinalyses",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn pl_prep_list() -> Vec<String> {
    return vec![
        "about", "above", "across", "after", "among", "around", "at", "athwart", "before",
        "behind", "below", "beneath", "beside", "besides", "between", "betwixt", "beyond", "but",
        "by", "during", "except", "for", "from", "in", "into", "near", "of", "off", "on", "onto",
        "out", "over", "since", "till", "to", "under", "until", "unto", "upon", "with",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn pl_prep_list_da() -> Vec<String> {
    let mut concat = pl_prep_list();
    concat.push("de".to_string());
    concat.push("du".to_string());
    concat.push("da".to_string());
    return concat;
}

fn pl_prep_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(pl_prep_list_da());
}

fn pl_prep() -> String {
    return enclose(&pl_prep_list_da().join("|"));
}

fn pl_sb_prep_dual_compound() -> String {
    return format!(r"(.*?)((?:-|\s+)(?:{})(?:-|\s+))a(?:-|\s+)(.*)", pl_prep());
}

fn singular_pronoun_genders() -> Vec<String> {
    return vec![
        "neuter",
        "feminine",
        "masculine",
        "gender-neutral",
        "feminine or masculine",
        "masculine or feminine",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn pl_pron_nom() -> HashMap<String, String> {
    return vec![
        // Nominative: Reflexive
        ("i", "we"),
        ("myself", "ourselves"),
        ("you", "you"),
        ("yourself", "yourselves"),
        ("she", "they"),
        ("herself", "themselves"),
        ("he", "they"),
        ("himself", "themselves"),
        ("it", "they"),
        ("itself", "themselves"),
        ("they", "they"),
        ("themself", "themselves"),
        // Possessive
        ("mine", "ours"),
        ("yours", "yours"),
        ("hers", "theirs"),
        ("his", "theirs"),
        ("its", "theirs"),
        ("theirs", "theirs"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect();
}

fn pl_pron_acc() -> HashMap<String, String> {
    return vec![
        ("me", "us"),
        ("myself", "ourselves"),
        ("you", "you"),
        ("yourself", "yourselves"),
        ("her", "them"),
        ("herself", "themselves"),
        ("it", "them"),
        ("itself", "themselves"),
        ("them", "them"),
        ("themself", "themselves"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect();
}

fn pl_pron_acc_keys() -> String {
    return enclose(
        &pl_pron_acc()
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .join("|"),
    );
}

fn pl_pron_acc_keys_bysize() -> HashMap<usize, HashSet<String>> {
    return bysize(pl_pron_acc().keys().cloned().collect::<Vec<String>>());
}

fn pron_tuples() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    return vec![
        ("nom", "they", "neuter", "it"),
        ("nom", "they", "feminine", "she"),
        ("nom", "they", "masculine", "he"),
        ("nom", "they", "gender-neutral", "they"),
        ("nom", "they", "feminine or masculine", "she or he"),
        ("nom", "they", "masculine or feminine", "he or she"),
        ("nom", "themselves", "neuter", "itself"),
        ("nom", "themselves", "feminine", "herself"),
        ("nom", "themselves", "masculine", "himself"),
        ("nom", "themselves", "gender-neutral", "themself"),
        (
            "nom",
            "themselves",
            "feminine or masculine",
            "herself or himself",
        ),
        (
            "nom",
            "themselves",
            "masculine or feminine",
            "himself or herself",
        ),
        ("nom", "theirs", "neuter", "its"),
        ("nom", "theirs", "feminine", "hers"),
        ("nom", "theirs", "masculine", "his"),
        ("nom", "theirs", "gender-neutral", "theirs"),
        ("nom", "theirs", "feminine or masculine", "hers or his"),
        ("nom", "theirs", "masculine or feminine", "his or hers"),
        ("acc", "them", "neuter", "it"),
        ("acc", "them", "feminine", "her"),
        ("acc", "them", "masculine", "him"),
        ("acc", "them", "gender-neutral", "them"),
        ("acc", "them", "feminine or masculine", "her or him"),
        ("acc", "them", "masculine or feminine", "him or her"),
        ("acc", "themselves", "neuter", "itself"),
        ("acc", "themselves", "feminine", "herself"),
        ("acc", "themselves", "masculine", "himself"),
        ("acc", "themselves", "gender-neutral", "themself"),
        (
            "acc",
            "themselves",
            "feminine or masculine",
            "herself or himself",
        ),
        (
            "acc",
            "themselves",
            "masculine or feminine",
            "himself or herself",
        ),
    ];
}

fn si_pron() -> HashMap<String, HashMap<String, HashMap<String, String>>> {
    let mut si_pron: HashMap<String, HashMap<String, HashMap<String, String>>> = HashMap::new();
    let mut nom: HashMap<String, HashMap<String, String>> = HashMap::new();
    for (k, v) in pl_pron_nom() {
        let mut entry = HashMap::new();
        entry.insert(k.to_string(), v.to_string());
        nom.insert(k.to_string(), entry);
    }
    //nom.insert("we".to_string(), "I".to_string());
    let mut acc: HashMap<String, HashMap<String, String>> = HashMap::new();
    for (k, v) in pl_pron_acc() {
        let mut entry = HashMap::new();
        entry.insert(k.to_string(), v.to_string());
        acc.insert(k.to_string(), entry);
    }
    si_pron.insert("nom".to_string(), nom);
    si_pron.insert("acc".to_string(), acc);

    for data in pron_tuples() {
        let (this_case, this_plur, this_gend, this_sing) = data;
        let case = si_pron
            .entry(this_case.to_string())
            .or_insert_with(HashMap::new);
        let plur = case
            .entry(this_plur.to_string())
            .or_insert_with(HashMap::new);
        plur.insert(this_gend.to_string(), this_sing.to_string());
    }

    si_pron
}

pub fn get_si_pron(thecase: &str, word: &str, gender: Option<&str>) -> String {
    match si_pron().get(thecase) {
        Some(case) => match case.get(word) {
            Some(sing) => match sing.get(gender.unwrap_or("N/A")) {
                Some(specific) => specific.clone(),
                None => sing.clone().values().next().unwrap().clone(),
            },
            None => panic!("No such case for word: {}", word),
        },
        None => panic!("No such case: {}", thecase),
    }
}

fn plverb_irregular_pres() -> HashMap<String, String> {
    return vec![
        ("am", "are"),
        ("are", "are"),
        ("is", "are"),
        ("was", "were"),
        ("were", "were"),
        ("have", "have"),
        ("has", "have"),
        ("do", "do"),
        ("does", "do"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect();
}

fn plverb_ambiguous_pres() -> HashMap<String, String> {
    return vec![
        ("act", "act"),
        ("acts", "act"),
        ("blame", "blame"),
        ("blames", "blame"),
        ("can", "can"),
        ("must", "must"),
        ("fly", "fly"),
        ("flies", "fly"),
        ("copy", "copy"),
        ("copies", "copy"),
        ("drink", "drink"),
        ("drinks", "drink"),
        ("fight", "fight"),
        ("fights", "fight"),
        ("fire", "fire"),
        ("fires", "fire"),
        ("like", "like"),
        ("likes", "like"),
        ("look", "look"),
        ("looks", "look"),
        ("make", "make"),
        ("makes", "make"),
        ("reach", "reach"),
        ("reaches", "reach"),
        ("run", "run"),
        ("runs", "run"),
        ("sink", "sink"),
        ("sinks", "sink"),
        ("sleep", "sleep"),
        ("sleeps", "sleep"),
        ("view", "view"),
        ("views", "view"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect();
}

fn plverb_ambiguous_pres_keys() -> Regex {
    let pattern = format!(
        r"^({})((\s.*)?)$",
        enclose(
            &plverb_ambiguous_pres()
                .keys()
                .cloned()
                .collect::<Vec<String>>()
                .join("|")
        )
    );
    return Regex::new(&pattern).expect("Failed to compile regex");
}

fn plverb_irregular_non_pres() -> Vec<String> {
    return vec![
        "did", "had", "ate", "made", "put", "spent", "fought", "sank", "gave", "sought", "shall",
        "could", "ought", "should",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn plverb_ambiguous_non_pres() -> Regex {
    let pattern = format!(r"^((?:thought|saw|bent|will|might|cut))((\s.*)?)$");
    return Regex::new(&pattern).expect("Failed to compile regex");
}

fn pl_v_oes_oe() -> Vec<String> {
    return vec!["canoes", "floes", "oboes", "roes", "throes", "woes"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn pl_v_oes_oe_endings_size4() -> Vec<String> {
    return vec!["hoes", "toes"].iter().map(|s| s.to_string()).collect();
}

fn pl_v_oes_oe_endings_size5() -> Vec<String> {
    return vec!["shoes"].iter().map(|s| s.to_string()).collect();
}

fn pl_count_zero() -> Vec<String> {
    return vec!["0", "no", "zero", "nil"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn pl_count_one() -> Vec<String> {
    return vec!["1", "a", "an", "one", "each", "every", "this", "that"]
        .iter()
        .map(|s| s.to_string())
        .collect();
}

fn pl_adj_special() -> HashMap<String, String> {
    return vec![
        ("a", "some"),
        ("an", "some"),
        ("this", "these"),
        ("that", "those"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect();
}

fn pl_adj_special_keys() -> Regex {
    let pattern = format!(
        r"^({})$",
        enclose(
            &pl_adj_special()
                .keys()
                .cloned()
                .collect::<Vec<String>>()
                .join("|")
        )
    );
    return Regex::new(&pattern).expect("Failed to compile regex");
}

fn pl_adj_poss() -> HashMap<String, String> {
    return vec![
        ("my", "our"),
        ("your", "your"),
        ("its", "their"),
        ("her", "their"),
        ("his", "their"),
        ("their", "their"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect();
}

fn pl_adj_poss_keys() -> Regex {
    let pattern = format!(
        r"^({})$",
        enclose(
            &pl_adj_poss()
                .keys()
                .cloned()
                .collect::<Vec<String>>()
                .join("|")
        )
    );
    return Regex::new(&pattern).expect("Failed to compile regex");
}

fn a_abbrev() -> Regex {
    return Regex::new(
        r"^(?:FJO|[HLMNS]Y\.|RY[EO]|SQU|(?:F[LR]?|[HL]|MN?|N|RH?|S[CHKLMNPTVW]?|X(?:YL)?) [AEIOU])[FHLMNRSX][A-Z]"
    ).expect("Failed to compile regex");
}

fn a_y_cons() -> Regex {
    return Regex::new(r"^(y(b[lor]|cl[ea]|fere|gg|p[ios]|rou|tt))")
        .expect("Failed to compile regex");
}

fn a_explicit_a() -> Regex {
    return Regex::new(r"^((?:unabomber|unanimous|US))").expect("Failed to compile regex");
}

fn a_explicit_an() -> Regex {
    return Regex::new(r"^((?:euler|hour(?!i)|heir|honest|hono[ur]|mpeg))")
        .expect("Failed to compile regex");
}

fn a_ordinal_a() -> Regex {
    return Regex::new(r"^([aefhilmnorsx]-?th)").expect("Failed to compile regex");
}

fn a_ordinal_an() -> Regex {
    return Regex::new(r"^([bcdgjkpqtuvwyz]-?th)").expect("Failed to compile regex");
}

fn nth() -> HashMap<u32, String> {
    return vec![
        (0, "th"),
        (1, "st"),
        (2, "nd"),
        (3, "rd"),
        (4, "th"),
        (5, "th"),
        (6, "th"),
        (7, "th"),
        (8, "th"),
        (9, "th"),
        (11, "th"),
        (12, "th"),
        (13, "th"),
    ]
    .iter()
    .map(|&(k, v)| (k, v.to_string()))
    .collect();
}

fn nth_suff() -> HashSet<String> {
    return nth().values().cloned().collect();
}

fn ordinal() -> HashMap<String, String> {
    return vec![
        ("ty", "tieth"),
        ("one", "first"),
        ("two", "second"),
        ("three", "third"),
        ("five", "fifth"),
        ("eight", "eighth"),
        ("nine", "ninth"),
        ("twelve", "twelfth"),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v.to_string()))
    .collect();
}

pub fn ordinal_suff() -> Regex {
    let pattern = format!(
        "({})",
        ordinal().keys().cloned().collect::<Vec<String>>().join("|")
    );
    return Regex::new(&format!("{}\\z", pattern)).expect("Failed to compile regex");
}

fn unit() -> Vec<String> {
    return vec![
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn teen() -> Vec<String> {
    return vec![
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn ten() -> Vec<String> {
    return vec![
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn mill() -> Vec<String> {
    return vec![
        " ",
        " thousand",
        " million",
        " billion",
        " trillion",
        " quadrillion",
        " quintillion",
        " sextillion",
        " septillion",
        " octillion",
        " nonillion",
        " decillion",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn def_classical() -> HashMap<String, bool> {
    return vec![
        ("all", false),
        ("zero", false),
        ("herd", false),
        ("names", true),
        ("persons", false),
        ("ancient", false),
    ]
    .iter()
    .map(|&(k, v)| (k.to_string(), v))
    .collect();
}

pub fn all_classical() -> HashMap<String, bool> {
    return def_classical()
        .iter()
        .map(|(k, _)| (k.to_string(), true))
        .collect();
}

pub fn no_classical() -> HashMap<String, bool> {
    return def_classical()
        .iter()
        .map(|(k, _)| (k.to_string(), false))
        .collect();
}

// TODO: May not need to be Option<bool>, or have the None case.
fn string_to_constant() -> HashMap<String, Option<bool>> {
    return vec![("True", Some(true)), ("False", Some(false)), ("None", None)]
        .iter()
        .map(|&(k, v)| (k.to_string(), v))
        .collect();
}

fn dollar_digits() -> Regex {
    Regex::new("\\$(\\d+)").expect("Failed to compile Regex")
}

// TODO: Pre-compiled REGEX objects, ln1950 @ og inflect

pub struct Words {
    pub lowered: String,
    pub split_: Vec<String>,
    pub first: String,
    pub last: String,
}

impl Words {
    pub fn new(s: &str) -> Words {
        let split: Vec<String> = s.split_whitespace().map(String::from).collect();
        Words {
            lowered: s.to_lowercase(),
            split_: split.clone(),
            first: split.get(0).cloned().unwrap_or_else(String::new),
            last: split.last().cloned().unwrap_or_else(String::new),
        }
    }
}

// FIXME: This is terrible. Placeholder.
#[derive(Debug, Clone)]
pub struct Word(String);

impl Word {
    pub fn new(word: String) -> Result<Self, &'static str> {
        if word.len() >= 1 {
            Ok(Word(word))
        } else {
            Err("Word must be >= 1 chars long.")
        }
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

pub struct Engine {
    pub classical_dict: HashMap<String, bool>,
    pub persistent_count: Option<i32>,
    mill_count: i32,
    pl_sb_user_defined: Vec<Option<Word>>,
    pl_v_user_defined: Vec<Option<Word>>,
    pl_adj_user_defined: Vec<Option<Word>>,
    si_sb_user_defined: Vec<Option<Word>>,
    a_a_user_defined: Vec<Option<Word>>,
    the_gender: String,
    _number_args: Option<HashMap<String, String>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            classical_dict: def_classical(),
            persistent_count: None,
            mill_count: 0,
            pl_sb_user_defined: Vec::new(),
            pl_v_user_defined: Vec::new(),
            pl_adj_user_defined: Vec::new(),
            si_sb_user_defined: Vec::new(),
            a_a_user_defined: Vec::new(),
            the_gender: "neuter".to_string(),
            _number_args: None,
        }
    }

    fn _number_args(&self) -> Option<&HashMap<String, String>> {
        self._number_args.as_ref()
    }

    fn set_number_args(&mut self, args: Option<HashMap<String, String>>) {
        self._number_args = args;
    }

    fn defnoun(&mut self, singular: &Option<Word>, plural: &Option<Word>) {
        self.checkpat(singular);
        self.checkpatplural(plural);
        self.pl_sb_user_defined
            .extend(vec![singular.clone(), plural.clone()]);
        self.si_sb_user_defined
            .extend(vec![plural.clone(), singular.clone()]);
    }

    fn defverb(
        &mut self,
        singular_1st: &Option<Word>,
        singular_2nd: &Option<Word>,
        singular_3rd: &Option<Word>,
        plural_1st: &Option<Word>,
        plural_2nd: &Option<Word>,
        plural_3rd: &Option<Word>,
    ) {
        self.checkpat(singular_1st);
        self.checkpat(singular_2nd);
        self.checkpat(singular_3rd);
        self.checkpat(plural_1st);
        self.checkpat(plural_2nd);
        self.checkpat(plural_3rd);
        self.pl_v_user_defined.extend(vec![
            singular_1st.clone(),
            singular_2nd.clone(),
            singular_3rd.clone(),
            plural_1st.clone(),
            plural_2nd.clone(),
            plural_3rd.clone(),
        ]);
    }

    fn defadj(&mut self, singular: &Option<Word>, plural: &Option<Word>) {
        self.checkpat(singular);
        self.checkpatplural(plural);
        self.pl_adj_user_defined
            .extend(vec![singular.clone(), plural.clone()]);
    }

    fn defa(&mut self, pattern: &Option<Word>) {
        self.checkpat(pattern);
        self.a_a_user_defined.extend(vec![
            pattern.clone(),
            Some(Word::new(String::from("a")).expect("Failed to make Word")),
        ]);
    }

    fn defan(&mut self, pattern: &Option<Word>) {
        self.checkpat(pattern);
        self.a_a_user_defined.extend(vec![
            pattern.clone(),
            Some(Word::new(String::from("an")).expect("Failed to make Word")),
        ]);
    }

    fn checkpat(&self, pattern: &Option<Word>) {
        if pattern.is_none() {
            return;
        }

        if pattern.is_some() {
            let word = pattern.clone().expect("Failed to unwrap Word");
            let _re = Regex::new(word.get()).expect("Failed to compile regex");
        }
    }

    // TODO: Check for replace pattern.
    fn checkpatplural(&self, pattern: &Option<Word>) {
        self.checkpat(pattern);
    }

    pub fn gender(&mut self, gender: &str) {
        if singular_pronoun_genders().contains(&String::from(gender)) {
            self.the_gender = gender.to_string();
        }
    }

    pub fn check_gender(&self) -> &String {
        &self.the_gender
    }

    pub fn get_count<T: Into<IntOrString>>(&self, count: Option<T>) -> i32 {
        if count.is_none() {
            if self.persistent_count.is_some() {
                return self.persistent_count.unwrap();
            } else {
                return 0;
            }
        }

        let c = count.unwrap().into();
        match c {
            IntOrString::Int(n) => return n,
            IntOrString::Str(s) => {
                if pl_count_one().contains(&s)
                    || (*self.classical_dict.get("zero").unwrap_or(&false)
                        && pl_count_zero().contains(&s.to_lowercase()))
                {
                    return 1;
                } else {
                    return 2;
                }
            }
        }
    }
}

enum IntOrString {
    Int(i32),
    Str(String),
}

impl From<i32> for IntOrString {
    fn from(n: i32) -> Self {
        IntOrString::Int(n)
    }
}

impl From<String> for IntOrString {
    fn from(s: String) -> Self {
        IntOrString::Str(s)
    }
}

impl From<&str> for IntOrString {
    fn from(s: &str) -> Self {
        IntOrString::Str(s.to_string())
    }
}
