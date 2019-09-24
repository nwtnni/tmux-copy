use lazy_static::lazy_static;

const SHA: &str = r"[0-9a-f]{7, 40}";
const URL: &str = r"(?::https?://|git@|git://|ssh://|ftp://|file://)[^ ]+";

lazy_static! {
    static ref SHA_RE: regex::Regex = regex::Regex::new(SHA).unwrap();
    static ref URL_RE: regex::Regex = regex::Regex::new(URL).unwrap();
    static ref ALL_RE: [&'static regex::Regex; 2] = [&*SHA_RE, &*URL_RE];
    static ref SET_RE: regex::RegexSet = regex::RegexSet::new(&[SHA, URL]).unwrap();
}

pub fn matches(text: &str) -> Vec<regex::Match> {
    let mut matches = Vec::new();
    for line in text.split('\n') {
        matches.extend(
            SET_RE.matches(text)
                .iter()
                .map(|index| ALL_RE[index])
                .flat_map(|re| re.find_iter(line))
        );
    }
    matches.sort_by_key(|r#match| r#match.start());
    matches
}
