use lazy_static::lazy_static;

const IPV4: &str = r"[[:digit:]]{1,3}\.[[:digit:]]{1,3}\.[[:digit:]]{1,3}\.[[:digit:]]{1,3}";
const IPV6: &str = r"[[:xdigit:]]+:+[[:xdigit:]]+[^[[:space:]]]+";
const PATH: &str = r"/?(?:(?:[[:word:]]|-|~|\.)+/)+(?:[[:word:]]|-|~|\.)*";
const SHA: &str = r"[[:xdigit:]]{7, 40}";
const URL: &str = r"(?:https?://|git@|git://|ssh://|ftp://|file://)[^[[:space:]]]+";
const UID: &str = r"[[:xdigit:]]{8}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{12}";

lazy_static! {
    static ref IPV4_RE: regex::Regex = regex::Regex::new(IPV4).unwrap();
    static ref IPV6_RE: regex::Regex = regex::Regex::new(IPV6).unwrap();
    static ref PATH_RE: regex::Regex = regex::Regex::new(PATH).unwrap();
    static ref SHA_RE: regex::Regex = regex::Regex::new(SHA).unwrap();
    static ref URL_RE: regex::Regex = regex::Regex::new(URL).unwrap();
    static ref UID_RE: regex::Regex = regex::Regex::new(UID).unwrap();
    static ref ALL_RE: [&'static regex::Regex; 6] = [&*IPV4_RE, &*IPV6_RE, &*PATH_RE, &*SHA_RE, &*URL_RE, &*UID_RE];
    static ref SET_RE: regex::RegexSet = regex::RegexSet::new(&[IPV4, IPV6, PATH, SHA, URL]).unwrap();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Match<'s> {
    pub row: usize,
    pub col: usize,
    pub txt: &'s str,
}

pub fn matches(text: &str) -> Vec<Match> {
    let mut matches = Vec::new();
    for (row, line) in text.split('\n').enumerate() {
        matches.extend(
            SET_RE.matches(text)
                .iter()
                .map(|index| ALL_RE[index])
                .flat_map(move |re| re.find_iter(line).map(move |r#match| {
                    Match {
                        row,
                        col: r#match.start(),
                        txt: r#match.as_str(),
                    }
                }))
        );
    }
    matches.sort();
    matches
}
