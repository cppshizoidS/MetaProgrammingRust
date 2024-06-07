#[macro_export]
macro_rules! exists {
    ($value:expr) => {
        $value.is_some()
    };
}

#[derive(Debug, Clone)]
pub struct RegexPart {
    source: String,
}

impl RegexPart {
    pub fn new(source: &str) -> Self {
        RegexPart {
            source: source.to_string(),
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

#[macro_export]
macro_rules! raw {
    ($str:expr) => {
        RegexPart::new($str)
    };
}

#[macro_export]
macro_rules! escape {
    ($str:expr) => {
        raw!($str.replace(&['-', '/', '\\', '^', '$', '*', '+', '?', '.', '(', ')', '|', '[', ']', '{', '}'][..], "\\$&"))
    };
}

#[macro_export]
macro_rules! non_capturing_group {
    ($part:expr) => {
        raw!(&format!("(?:{})", $part.source()))
    };
}

#[macro_export]
macro_rules! capturing_group {
    ($part:expr) => {
        raw!(&format!("({})", $part.source()))
    };
}

#[macro_export]
macro_rules! one_or_more {
    ($part:expr) => {
        raw!(&format!("{}+", non_capturing_group!($part).source()))
    };
}

#[macro_export]
macro_rules! zero_or_more {
    ($part:expr) => {
        raw!(&format!("{}*", non_capturing_group!($part).source()))
    };
}

#[macro_export]
macro_rules! any_of {
    ($($part:expr),+) => {
        non_capturing_group!(raw!(&[$($part.source()),+].join("|"))).as_str()
    };
}

#[macro_export]
macro_rules! sequence {
    ($($part:expr),+) => {
        raw!(&[$($part.source()),+].join(""))
    };
}

#[macro_export]
macro_rules! optional {
    ($part:expr) => {
        raw!(&format!("{}?", non_capturing_group!($part).source()))
    };
}

#[macro_export]
macro_rules! lazy {
    ($part:expr) => {
        raw!(&format!("{}*?", non_capturing_group!($part).source()))
    };
}

#[macro_export]
macro_rules! times {
    ($part:expr, $min:expr $(, $max:expr)?) => {
        raw!(&format!(
            "{}{{{},{}}}",
            non_capturing_group!($part).source(),
            $min,
            if exists!($max) {
                $max.unwrap_or("")
            } else {
                ""
            }
        ))
    };
}

#[macro_export]
macro_rules! not {
    ($part:expr) => {
        raw!(&format!("(?!{})", $part.source()))
    };
}

#[macro_export]
macro_rules! not_escaped {
    ($part:expr) => {
        sequence!(not!(escape!(r"\\")), $part)
    };
}

#[macro_export]
macro_rules! custom_word_boundary {
    () => {
        raw!(&format!(r"(?:[\s]+|{}|^|$)", one_or_more!(MARKS).source()))
    };
}
#[macro_export]
macro_rules! anything {
    () => {
        raw!(".")
    };
}

#[macro_export]
macro_rules! nothing {
    () => {
        raw!("")
    };
}

#[macro_export]
macro_rules! word_boundary {
    () => {
        raw!(r"\b")
    };
}

#[macro_export]
macro_rules! whitespace {
    () => {
        raw!(r"\s")
    };
}

#[macro_export]
macro_rules! alpha_numeric {
    () => {
        raw!(r"\w")
    };
}

#[macro_export]
macro_rules! non_alpha_numeric {
    () => {
        raw!(r"\W")
    };
}

#[macro_export]
macro_rules! digit {
    () => {
        raw!(r"\d")
    };
}

#[macro_export]
macro_rules! non_digit {
    () => {
        raw!(r"\D")
    };
}

#[macro_export]
macro_rules! begin {
    () => {
        raw!(r"^")
    };
}

#[macro_export]
macro_rules! end {
    () => {
        raw!(r"$")
    };
}

#[macro_export]
macro_rules! cyrillic_letter {
    () => {
        raw!(r"[а-яА-ЯёЁ]")
    };
}

#[macro_export]
macro_rules! punctuation {
    () => {
        raw!(r"[.,!?;:]")
    };
}
/*
#[macro_export]
macro_rules! marks {
    () => {
        raw!(r"[${escape('`~!@#$%^&*()_+-={}[]|\\:;"\'<>,.?/’‘')}]")
    };
}
*/

fn main() {
    let part = raw!("test");
    let regex = one_or_more!(part);
    println!("{}", regex.source());
}
