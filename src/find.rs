macro_rules! regex_set {
    ($($name:ident: $regex:expr);* $(;)?) => {
        paste::item! {
            $(const $name: &str = $regex;)*
            lazy_static::lazy_static! {
                $(static ref [<$name _RE>]: regex::Regex = regex::Regex::new($name).unwrap();)*
                static ref ALL_RE: [&'static regex::Regex; count!($($name),*)] = [$(&*[<$name _RE>]),*];
                static ref SET_RE: regex::RegexSet = regex::RegexSet::new(&[$($name),*]).unwrap();
            }
        }
    }
}

regex_set! {
    IPV4: r"[[:digit:]]{1,3}\.[[:digit:]]{1,3}\.[[:digit:]]{1,3}\.[[:digit:]]{1,3}";
    IPV6: r"[[:xdigit:]]+:+[[:xdigit:]]+[^[[:space:]]]+";
    KUBE: r"[a-z0-9]+(?:-[a-z0-9]+)+";
    PATH: r"/?([[[:word:]]-~\.]+/)+[[[:word:]]-\.]*";
    SHA:  r"[[:xdigit:]]{7, 40}";
    UID:  r"[[:xdigit:]]{8}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{12}";
    URL:  r#"(?:https?://|git@|git://|ssh://|ftp://|file://)[^'"\]\)[[:space:]]]+"#;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Match<'s> {
    pub row: u16,
    pub col: u16,
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
                        row: row as u16,
                        col: r#match.start() as u16,
                        txt: r#match.as_str(),
                    }
                }))
        );
    }
    matches.sort();
    matches
}
