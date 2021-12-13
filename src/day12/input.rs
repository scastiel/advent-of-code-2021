#[allow(dead_code)]
pub fn example1() -> Vec<(&'static str, &'static str)> {
    vec![
        ("start", "A"),
        ("start", "b"),
        ("A", "c"),
        ("A", "b"),
        ("b", "d"),
        ("A", "end"),
        ("b", "end"),
    ]
}

#[allow(dead_code)]
pub fn example2() -> Vec<(&'static str, &'static str)> {
    vec![
        ("dc", "end"),
        ("HN", "start"),
        ("start", "kj"),
        ("dc", "start"),
        ("dc", "HN"),
        ("LN", "dc"),
        ("HN", "end"),
        ("kj", "sa"),
        ("kj", "HN"),
        ("kj", "dc"),
    ]
}

#[allow(dead_code)]
pub fn example3() -> Vec<(&'static str, &'static str)> {
    vec![
        ("fs", "end"),
        ("he", "DX"),
        ("fs", "he"),
        ("start", "DX"),
        ("pj", "DX"),
        ("end", "zg"),
        ("zg", "sl"),
        ("zg", "pj"),
        ("pj", "he"),
        ("RW", "he"),
        ("fs", "DX"),
        ("pj", "RW"),
        ("zg", "RW"),
        ("start", "pj"),
        ("he", "WI"),
        ("zg", "he"),
        ("pj", "fs"),
        ("start", "RW"),
    ]
}

pub fn exercise() -> Vec<(&'static str, &'static str)> {
    vec![
        ("YW", "end"),
        ("DK", "la"),
        ("la", "XG"),
        ("end", "gy"),
        ("zq", "ci"),
        ("XG", "gz"),
        ("TF", "la"),
        ("xm", "la"),
        ("gy", "gz"),
        ("ci", "start"),
        ("YW", "ci"),
        ("TF", "zq"),
        ("ci", "DK"),
        ("la", "TS"),
        ("zq", "YW"),
        ("gz", "YW"),
        ("zq", "gz"),
        ("end", "gz"),
        ("ci", "TF"),
        ("DK", "zq"),
        ("gy", "YW"),
        ("start", "DK"),
        ("gz", "DK"),
        ("zq", "la"),
        ("start", "TF"),
    ]
}
