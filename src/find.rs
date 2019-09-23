use lazy_static::lazy_static;

const SHA: &str = r"[0-9a-f]{7, 40}";
const URL: &str = r"(?::https?://|git@|git://|ssh://|ftp://|file://)[^ ]+";

lazy_static! {
    static ref SHA_RE: regex::Regex = regex::Regex::new(SHA).unwrap();
    static ref URL_RE: regex::Regex = regex::Regex::new(URL).unwrap();
    static ref SET_RE: regex::RegexSet = regex::RegexSet::new(&[SHA, URL]).unwrap();
}

pub fn matches(hay: &str) -> Vec<regex::Match> {
    unimplemented!()
}
