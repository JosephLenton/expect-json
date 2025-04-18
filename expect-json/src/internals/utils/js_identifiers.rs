pub fn is_unquotable_js_identifier<S>(js_identifier: S) -> bool
where
    S: AsRef<str>,
{
    let mut js_chars = js_identifier.as_ref().chars();
    let maybe_first = js_chars.next();
    match maybe_first {
        None => return false,
        Some(c) => {
            if !is_unquotable_first_js_char(c) {
                return false;
            }
        }
    }

    js_chars.all(is_unquotable_js_char)
}

fn is_unquotable_first_js_char(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

fn is_unquotable_js_char(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}

#[cfg(test)]
mod test_is_unquotable_js_identifier {
    use super::*;

    #[test]
    fn it_should_be_false_for_empty_string() {
        let output = is_unquotable_js_identifier("");
        assert_eq!(output, false);
    }

    #[test]
    fn it_should_be_false_for_string_that_starts_with_numbers() {
        let output = is_unquotable_js_identifier("0abc");
        assert_eq!(output, false);
    }

    #[test]
    fn it_should_be_true_for_single_letter_characters() {
        let output = is_unquotable_js_identifier("_");
        assert_eq!(output, true);

        let output = is_unquotable_js_identifier("a");
        assert_eq!(output, true);

        let output = is_unquotable_js_identifier("A");
        assert_eq!(output, true);
    }

    #[test]
    fn it_should_be_true_for_identifiers_with_underscores_in_name() {
        let output = is_unquotable_js_identifier("abc_xyz");
        assert_eq!(output, true);
    }

    #[test]
    fn it_should_be_true_for_identifiers_with_numbers_in_name() {
        let output = is_unquotable_js_identifier("abc123");
        assert_eq!(output, true);
    }
}
