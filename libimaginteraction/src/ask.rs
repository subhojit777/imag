// functions to ask the user for data, with crate:spinner

use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;
use ansi_term::Colour::*;

/// Ask the user for a Yes/No answer. Optionally provide a default value. If none is provided, this
/// keeps loop{}ing
pub fn ask_bool(s: &str, default: Option<bool>) -> bool {
    ask_bool_(s, default, &mut BufReader::new(stdin()))
}

fn ask_bool_<R: BufRead>(s: &str, default: Option<bool>, input: &mut R) -> bool {
    lazy_static! {
        static ref R_YES: Regex = Regex::new(r"^[Yy]$").unwrap();
        static ref R_NO: Regex  = Regex::new(r"^[Nn]$").unwrap();
    }

    loop {
        ask_question(s, false);
        if match default { Some(s) => s, _ => true } {
            println!(" [Yn]: ");
        } else {
            println!(" [yN]: ");
        }

        let mut s = String::new();
        let _     = input.read_line(&mut s);

        if R_YES.is_match(&s[..]) {
            return true
        } else if R_NO.is_match(&s[..]) {
            return false
        } else {
            if default.is_some() {
                return default.unwrap();
            }

            // else again...
        }
    }
}

/// Ask the user for an unsigned number. Optionally provide a default value. If none is provided,
/// this keeps loop{}ing
pub fn ask_uint(s: &str, default: Option<u64>) -> u64 {
    ask_uint_(s, default, &mut BufReader::new(stdin()))
}

fn ask_uint_<R: BufRead>(s: &str, default: Option<u64>, input: &mut R) -> u64 {
    use std::str::FromStr;

    loop {
        ask_question(s, false);

        let mut s = String::new();
        let _     = input.read_line(&mut s);

        let u : Result<u64, _> = FromStr::from_str(&s[..]);
        match u {
            Ok(u)  => { return u; },
            Err(_) => {
                if default.is_some() {
                    return default.unwrap();
                } // else keep looping
            }
        }
    }
}

pub fn ask_string(s: &str) -> String {
    unimplemented!()
}

pub fn ask_enum<E: From<String>>(s: &str) -> E {
    unimplemented!()
}

/// Helper function to print a imag question string. The `question` argument may not contain a
/// trailing questionmark.
///
/// The `nl` parameter can be used to configure whether a newline character should be printed
pub fn ask_question(question: &str, nl: bool) {
    if nl {
        println!("[imag]: {}?", Yellow.paint(question));
    } else {
        print!("[imag]: {}?", Yellow.paint(question));
    }
}

#[cfg(test)]
mod test {
    use std::io::BufReader;

    use super::ask_bool_;
    use super::ask_uint_;

    #[test]
    fn test_ask_bool_nodefault_yes() {
        let question = "Is this true";
        let default  = None;
        let answers  = "\n\n\n\n\ny";

        assert!(ask_bool_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_bool_nodefault_no() {
        let question = "Is this true";
        let default  = None;
        let answers  = "n";

        assert!(false == ask_bool_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_bool_default_no() {
        let question = "Is this true";
        let default  = Some(false);
        let answers  = "n";

        assert!(false == ask_bool_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_bool_default_yes() {
        let question = "Is this true";
        let default  = Some(true);
        let answers  = "y";

        assert!(true == ask_bool_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_bool_default_yes_answer_no() {
        let question = "Is this true";
        let default  = Some(true);
        let answers  = "n";

        assert!(false == ask_bool_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_bool_default_no_answer_yes() {
        let question = "Is this true";
        let default  = Some(false);
        let answers  = "y";

        assert!(true == ask_bool_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_bool_default_no_without_answer() {
        let question = "Is this true";
        let default  = Some(false);
        let answers  = "\n";

        assert!(false == ask_bool_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_bool_default_yes_without_answer() {
        let question = "Is this true";
        let default  = Some(true);
        let answers  = "\n";

        assert!(true == ask_bool_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_uint_nodefault() {
        let question = "Is this 1";
        let default  = None;
        let answers  = "1";

        assert!(1 == ask_uint_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_uint_default() {
        let question = "Is this 1";
        let default  = Some(1);
        let answers  = "1";

        assert!(1 == ask_uint_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_uint_default_2_input_1() {
        let question = "Is this 1";
        let default  = Some(2);
        let answers  = "1";

        assert!(1 == ask_uint_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_uint_default_2_noinput() {
        let question = "Is this 1";
        let default  = Some(2);
        let answers  = "\n";

        assert!(2 == ask_uint_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_uint_default_2_several_noinput() {
        let question = "Is this 1";
        let default  = Some(2);
        let answers  = "\n\n\n\n";

        assert!(2 == ask_uint_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

    #[test]
    fn test_ask_uint_default_2_wrong_input() {
        let question = "Is this 1";
        let default  = Some(2);
        let answers  = "\n\n\nasfb\nsakjf\naskjf\n-2";

        assert!(2 == ask_uint_(question, default, &mut BufReader::new(answers.as_bytes())));
    }

}
