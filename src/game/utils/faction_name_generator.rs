use rand::{seq::SliceRandom, thread_rng, Rng};

const FIRST_PART: &[&str] = &[
    "Sfe", "Grh", "Abs", "Inf", "Hs", "Bt", "Nswe", "Newo", "Abnoe", "Bq", "Zo", "Ri", "Vub",
    "Oie", "Pao", "Kba", "Ube", "Xr", "Cuo", "Ey",
];

const SECOND_PART: &[&str] = &[
    "ehob", "few", "gij", "his", "ian", "jo", "keq", "leh", "mal", "nve", "ooi", "pui", "qaah",
    "ra", "seeii", "tete", "uq", "vaaa", "wbai", "xoga", "y", "z",
];

const THIRD_PART: &[&str] = &["abab", "ba", "cewew", "oabao", "bir", "zwow"];

pub fn generate_faction_name() -> String {
    let mut rng = &mut thread_rng();
    let apos = if rng.gen_bool(0.3) { "'" } else { "" };

    let has_2_parts = rng.gen_bool(0.5);

    let a = FIRST_PART.choose(&mut rng).unwrap();
    let b = SECOND_PART.choose(&mut rng).unwrap();
    let c = THIRD_PART.choose(&mut rng).unwrap();

    if has_2_parts {
        let is_use_second = rng.gen_bool(0.5);

        if is_use_second {
            format!("{a}{apos}{b}")
        } else {
            format!("{a}{apos}{c}")
        }
    } else {
        let is_apos_front = rng.gen_bool(0.5);

        if is_apos_front {
            format!("{a}{apos}{b}{c}")
        } else {
            format!("{a}{b}{apos}{c}")
        }
    }
}
