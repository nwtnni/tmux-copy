use crate::util;

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
    NAME: r"[a-zA-Z0-9]+(?:[-_.:][a-zA-Z0-9]+)+";
    PATH: r"/?(?:[[[:word:]]-~\.]+/)+[[[:word:]]-\.]*";
    SHA:  r"[[:xdigit:]]{7, 40}";
    UID:  r"[[:xdigit:]]{8}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{12}";
    URL:  r#"(?:https?://|git@|git://|ssh://|ftp://|file://)[^'"\]\)[[:space:]]]+"#;
    MD:   r"`([^`]+)`";
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
        let ascii = &util::Lazy::new(|| line.is_ascii());

        /// Constructs a `Match` from a `regex::Match`.
        /// We use a macro here to (a) avoid the overhead of
        /// constructing a possibly unnecessary closure every
        /// iteration, and (b) avoid having to name the
        /// `util::Lazy<F>` type in a function parameter.
        macro_rules! found {
          ($match:expr) => {
            {
              let start = $match.start();
              Match {
                row: row as u16,
                col: if ascii.force() { start } else { column(line, start) } as u16,
                txt: $match.as_str(),
              }
            }
          }
        }

        matches.extend(
            SET_RE.matches(text)
                .iter()
                .map(|index| ALL_RE[index])
                .flat_map(move |re| {
                  if re.captures_len() == 1 {
                    util::Or::L(
                      re.find_iter(line)
                        .map(move |r#match| found!(r#match))
                    )
                  } else {
                    util::Or::R(
                      re.captures_iter(line)
                        .filter_map(|capture| capture.get(1))
                        .map(move |r#match| found!(r#match))
                    )
                  }
                })
        );
    }

    matches.sort();
    matches
}

fn column(line: &str, index: usize) -> usize {
    line.char_indices()
        .enumerate()
        .find(|(_, (idx, _))| {
            *idx == index
        })
        .map(|(col, _)| col)
        .unwrap()
}
