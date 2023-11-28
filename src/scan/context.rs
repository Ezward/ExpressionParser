//!
//! Scanning primitives using [ScanContext] to maintain
//! the scanning state across the application of multiple scanners,
//! so that a string can be scanned from beginning to end.
//!
//! ```rust
//! #[test]
//! fn test_scan_chars_ok_sequentially() {
//!     let s = "foo\nbar";
//!     let context = (true, ScanPosition::default());
//!
//!     //
//!     // scan the first 'f' character using a lambda
//!     //
//!     let result = scan_one_or_more_chars(s, context, |ch| ch == 'f');
//!     assert_eq!((true, ScanPosition::new('f'.len_utf8(), 1, 0, 0, 0)), result);
//!
//!     //
//!     // scan the 'o' characters starting from last scan result
//!     //
//!     let result = scan_one_or_more_chars(s, result, |ch| ch == 'o');
//!     assert_eq!((true, ScanPosition::new("foo".len(), 3, 0, 0, 0)), result);
//!
//!     //
//!     // scan the remaining underscore and alphabetic characters
//!     // starting from the last can result.
//!     //
//!     let result = scan_zero_or_more_chars(s, result, |ch| ch == '\n' || ch.is_alphabetic());
//!     assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 1, "foo\n".len(), "foo\n".chars().count())), result);
//!
//!     //
//!     // do the same thing in one function call
//!     //
//!     let result = scan_zero_or_more_chars(s,
//!                     scan_one_or_more_chars(s,
//!                         scan_one_or_more_chars(s,
//!                             context,
//!                             |ch| ch == 'f'),
//!                         |ch| ch == 'o'),
//!                     |ch| ch == '\n' || ch.is_alphabetic());
//!     assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 1, "foo\n".len(), "foo\n".chars().count())), result);
//! }
//! ```
//!
use std::usize;

const NEWLINE: char = '\n';
const NEWLINE_LEN: usize = NEWLINE.len_utf8();

///
/// scan position at byte index, char index and line index.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScanPosition {
    pub byte_index: usize,      // index in bytes
    pub char_index: usize,      // index in utf-8 characters
    pub line_index: usize,      // index of line
    pub line_byte_index: usize, // byte index of beginning of line
    pub line_char_index: usize, // char index of beginning of line
}
impl Default for ScanPosition {
    fn default() -> Self {
        Self { byte_index: 0, char_index: 0, line_index: 0, line_byte_index: 0, line_char_index: 0 }
    }
}
impl ScanPosition {
    pub fn new(byte_index: usize, char_index: usize, line_index: usize, line_byte_index: usize, line_char_index: usize) -> Self {
        Self {
            byte_index: byte_index,
            char_index: char_index,
            line_index: line_index,
            line_byte_index: line_byte_index,
            line_char_index: line_char_index
        }
    }
}


///
/// ScanContext maintains the scanning state:
/// - **matched: bool** is whether scan has matched so far
/// - **byte offset: usize** is the number of bytes matched so far
/// - **char offset: usize** is the number of chars matched
/// - **line offset: usize** is the number of line endings scanned
/// - **line byte offset: usize** start of line in bytes
/// - **line char offset: usize** start of line in chars
///
pub type ScanContext = (
    bool,           // matched: true if scanning has matched so far
    ScanPosition,   // byte offset after last byte in last matching char (aka number of bytes matched)
                    // char offset after last matching char (aka number of utf-8 chars matched)
                    // line offset after last matching char (aka number of line endings scanned)
                    // line byte offset of start of line including char after last matching char (aka start of current line)
                    // line char offset of start of line including char after last matching char (aka start of current line)
);

///
/// Scan for a literal string.
/// - **s**: the string to scan
/// - **context**: the current scanning state
/// - **literal**: the literal string to match
/// - **returns**:
///   - The scan result as a [ScanContext]
///     - matched is true if entire literal matched
///     - matched is false if any of literal did not match
///     - byte offset is offset after last byte in last matching char (aka total number of bytes matched)
///     - char offset is offset after last matching char (aka total number of utf-8 chars matched)
///     - line offset is number of line endings scanned up to and including the last matched character.
///
pub fn scan_literal(
    s: &str,                // IN : the string to scan
    context: ScanContext,   // IN : scanning state
    literal: &'static str)  // IN : the literal string to match
    -> ScanContext          // RET: scan result as an ScanContext
                            //      matched is false if not all chars in literal matched
                            //      matched is true all chars in literal matched
                            //      byte offset after last byte in last matching char (aka number of bytes matched)
                            //      char offset after last matching char (aka number of utf-8 chars matched)
                            //      line offset after last matching char (aka number of line-endings scanned)
{
    let (matched, mut position) = context;
    if (!matched) || position.byte_index > s.len(){
        return (false, position)
    }

    let mut _matches = 0;
    let mut s_chars = s[position.byte_index..].chars();
    for ch in literal.chars() {
        if let Some(sch) = s_chars.next() {
            if ch == NEWLINE {
                position.line_index += 1;
                position.line_byte_index = position.byte_index + NEWLINE_LEN;
                position.line_char_index = position.char_index + 1;
            }
            if ch == sch {
                _matches += 1;
                position.byte_index += ch.len_utf8();
                position.char_index += 1;
                continue;
            }
        }

        // return context where match failed
        return (false, position)
    }

    // entire literal matched
    (true, position)
}

///
/// Greedy scan for any chars that pass test.
/// - **s**: the string to scan
/// - **context**: the current scanning state
/// - **test**: a function that tests a character for a match
/// - **returns**:
///   - The scan result as a [ScanContext]
///     - matched is true if zero or more chars matched
///     - matched is false if context's byte offset is out of range
///     - byte offset is offset after last byte in last matching char (aka total number of bytes matched)
///     - char offset is offset after last matching char (aka total number of utf-8 chars matched)
///     - line offset is number of line endings scanned up to and including the last matched character.
///
pub fn scan_zero_or_more_chars(
    s: &str,                // IN : the string to scan
    context: ScanContext,   // IN : the string and offset to scan
    test: fn(char) -> bool) // IN : the function that applies the test to the characters
    -> ScanContext          // RET: scan result as an ScanContext
                            //      matched is false out of range
                            //      matched is true if zero or more chars matched
                            //      byte offset is offset after last byte in last matching char (aka total number of bytes matched)
                            //      char offset after last matching char (aka total number of utf-8 chars matched)
                            //      line offset after last matching char (aka number of line-endings scanned)
{
    let (matched, mut position) = context;
    if (!matched) || position.byte_index > s.len(){
        return (false, position)
    }

    let mut _matches: usize = 0;
    for ch in s[position.byte_index..].chars() {
        if ! test(ch) {
            return (true, position)
        }
        if ch == NEWLINE {
            position.line_index += 1;
            position.line_byte_index = position.byte_index + NEWLINE_LEN;
            position.line_char_index = position.char_index + 1;
        }
        _matches += 1;
        position.byte_index += ch.len_utf8();
        position.char_index += 1;
    }

    // entire string matches
    (true, position)
}

///
/// Greedy scan for one or more characters matching the test.
/// - **s**: the string to scan
/// - **context**: the current scanning state
/// - **test**: a function that tests a character for a match
/// - **returns**:
///   - The scan result as a [ScanContext]
///     - matched is true if one or more chars matched
///     - matched is false not matched or if context's byte offset is out of range
///     - byte offset is offset after last byte in last matching char (aka total number of bytes matched)
///     - char offset is offset after last matching char (aka total number of utf-8 chars matched)
///     - line offset is number of line endings scanned up to and including the last matched character.
///
pub fn scan_one_or_more_chars(
    s: &str,                // IN : the string to scan
    context: ScanContext,   // IN : the current scan state
    test: fn(char) -> bool) // IN : the function that applies the test to the characters
    -> ScanContext          // RET: scan result as an ScanContext
                            //      matched is false if zero chars matched
                            //      matched is true if one or more chars matched
                            //      byte offset after last byte in last matching char (aka number of bytes matched)
                            //      char offset after last matching char (aka number of utf-8 chars matched)
                            //      line offset after last matching char (aka number of line-endings scanned)
{
    let (matched, mut position) = context;
    if (!matched) || position.byte_index > s.len(){
        return (false, position)
    }

    let mut matches: usize = 0;
    for ch in s[position.byte_index..].chars() {
        if ! test(ch) {
            return (matches > 0, position)
        }
        if ch == NEWLINE {
            position.line_index += 1;
            position.line_byte_index = position.byte_index + NEWLINE_LEN;
            position.line_char_index = position.char_index + 1;
        }
        matches += 1;
        position.byte_index += ch.len_utf8();
        position.char_index += 1;
    }

    // entire string matches
    (matches > 0, position)
}

///
/// Scan for exactly n characters that match the test.
/// - **s**: the string to scan
/// - **context**: the current scanning state
/// - **n**: the number of characters that must match
/// - **test**: a function that tests a character for a match
/// - **returns**:
///   - The scan result as a [ScanContext]
///     - matched is true if n characters matched
///     - matched is false not matched or if context's byte offset is out of range
///     - byte offset is offset after last byte in last matching char (aka total number of bytes matched)
///     - char offset is offset after last matching char (aka total number of utf-8 chars matched)
///     - line offset is number of line endings scanned up to and including the last matched character.
///
pub fn scan_n_chars(
    s: &str,                // IN : the string to scan
    context: ScanContext,   // IN : the string and offset to scan
    n: usize,               // IN : required number of character matches
    test: fn(char) -> bool) // IN : the function that applies the test to the characters
    -> ScanContext          // RET: scan result as an ScanContext
                            //      matched is false if less than n chars matched
                            //      matched is true if n chars matched
                            //      byte offset after last byte in last matching char (aka number of bytes matched)
                            //      char offset after last matching char (aka number of utf-8 chars matched)
                            //      line offset after last matching char (aka number of line-endings scanned)
{
    let (matched, mut position) = context;
    if (!matched) || position.byte_index > s.len(){
        return (false, position)
    }

    let mut matches: usize = 0;
    for ch in s[position.byte_index..].chars() {
        if matches == n {
            return (true, position) // return offset after last match
        }

        if ch == NEWLINE {
            position.line_index += 1;
            position.line_byte_index = position.byte_index + NEWLINE_LEN;
            position.line_char_index = position.char_index + 1;
        }

        if test(ch) {
            matches += 1;
            position.byte_index += ch.len_utf8();
            position.char_index += 1;
            continue;
        };

        // we found a mismatch, so we are done
        return (false, position)
    }

    // we hit end of input
    (n == matches, position)
}

#[cfg(test)]
mod tests {
    use std::char;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_scan_literal_ok_match() {
        let s = "foo βαρ";
        let context = (true, ScanPosition::default());

        // scan identity
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0)), scan_literal(s, context, s));

        // scan "foo"
        let foo_context = scan_literal(s, context, "foo");
        assert_eq!((true, ScanPosition::new("foo".len(), 3, 0, 0, 0)), foo_context);

        // scan space
        let space_context = scan_literal(s, foo_context, " ");
        assert_eq!((true, ScanPosition::new("foo ".len(), 4, 0, 0, 0)), space_context);

        // scan "bar"
        let result = scan_literal(s, space_context, "βαρ");
        assert_eq!((true, ScanPosition::new("foo_βαρ".len(), 7, 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_literal_ok_no_match() {
        let s = "foo bar";
        let context = (true, ScanPosition::default());

        //
        // not matching returns false
        // along with index of char after last match
        // and number of matched chars.
        //
        let result = scan_literal(s, context, "xxx");
        assert_eq!((false, ScanPosition::default()), result);

        //
        // this will stop matching after first char
        // it returns false because it was not a complete match
        // and return index after 'b' match
        // and the count of 1 matched chars
        //
        let result = scan_literal(s, context, "fxx");
        assert_eq!((false, ScanPosition::new("f".len(), 1, 0, 0, 0)), result);

    }

    #[test]
    fn test_scan_literal_ok_out_of_range() {
        let s = "foo bar";

        //
        // offset beyond end of string will not match
        // and will return the byte and char indices unchanged.
        //
        let context = (true, ScanPosition::new(s.len() + 69, s.chars().count() + 69, 0, 0, 0));
        let result = scan_literal(s, context, "bar");
        assert_eq!((false, context.1), result)
    }

    #[test]
    fn test_scan_literal_ok_end_of_input() {
        let s = "foo bar";

        //
        // offset at end of input is no match,
        // returning byte offset and char offset unchanged.
        //
        let context = (true, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0));
        let result = scan_literal(s, context, "foo");
        assert_eq!((false, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_literal_ok_not_enough_input() {
        let s = "foo bar";
        let context = (true, ScanPosition::default());

        //
        // there are not enough chars in the buffer.
        // this will return false because literal is not completely matched.
        // it will return index at end of input (since it stopped there)
        // and number of matched chars.
        //
        let result = scan_literal(s, context, "foo bar baz");
        assert_eq!((false, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_chars_ok_lambda() {
        let s = "foo_βαρ";
        let context = (true, ScanPosition::default());

        //
        // scan the first 'f' character using a lambda
        //
        let result = scan_one_or_more_chars(s, context, |c| c == 'f');
        assert_eq!((true, ScanPosition::new('f'.len_utf8(), 1, 0, 0, 0)), result);

        //
        // scan the characters in "foo"
        //
        fn is_alphabetic(ch: char) -> bool {
            ch.is_alphabetic()
        }
        let result = scan_one_or_more_chars(s, context, is_alphabetic);
        assert_eq!((true, ScanPosition::new("foo".len(), 3, 0, 0, 0)), result);

        let result = scan_one_or_more_chars(s, context, |c: char| c.is_alphabetic());
        assert_eq!((true, ScanPosition::new("foo".len(), 3, 0, 0, 0)), result);

        //
        // no matches returns false
        //
        let result = scan_one_or_more_chars(s, context, |c| c == 'x');
        assert_eq!((false, ScanPosition::default()), result);

        //
        // scan wide character
        //
        let context = (true, ScanPosition::new("foo_β".len(), "foo_β".chars().count(), 0, 0, 0));
        let result = scan_one_or_more_chars(s, context, |c| c == 'α');
        assert_eq!((true, ScanPosition::new("foo_βα".len(), "foo_βα".chars().count(), 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_chars_ok_out_of_range() {
        let s = "foo bar";

        //
        // offset beyond end of string will not match
        // and will return the byte and char indices unchanged.
        //
        let context = (true, ScanPosition::new(s.len() + 69, s.chars().count() + 69, 0, 0, 0));
        let result = scan_one_or_more_chars(s, context, |c| c.is_alphabetic());
        assert_eq!((false, context.1), result)
    }

    #[test]
    fn test_scan_chars_ok_end_of_input() {
        let s = "foo bar";

        //
        // offset at end of input is no match,
        // returning byte offset and char offset unchanged.
        //
        let context = (true, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0));
        let result = scan_one_or_more_chars(s, context, |c| c.is_alphabetic());
        assert_eq!((false, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_n_chars_ok() {
        let s = "foo_βαρ";
        let context = (true, ScanPosition::default());

        //
        // scan the first 'f' character using a lambda
        //
        let result = scan_n_chars(s, context, 1, |c| c == 'f');
        assert_eq!((true, ScanPosition::new('f'.len_utf8(), 1, 0, 0, 0)), result);

        //
        // scan the "foo" using a lambda
        //
        let result = scan_n_chars(s, context, 3, |c| c.is_alphabetic());
        assert_eq!((true, ScanPosition::new("foo".len(), 3, 0, 0, 0)), result);

        //
        // scan wide character
        //
        let context = (true, ScanPosition::new("foo_β".len(), "foo_β".chars().count(), 0, 0, 0));
        let result = scan_n_chars(s, context, 1, |c| c == 'α');
        assert_eq!((true, ScanPosition::new("foo_βα".len(), "foo_βα".chars().count(), 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_n_chars_zero_ok() {
        let s = "foo_bar";
        let context = (true, ScanPosition::default());

        //
        // n of zero always matches
        //
        let result = scan_n_chars(s, context, 0, |c| c == 'x');
        assert_eq!((true, ScanPosition::default()), result);

        //
        // n of zero always matches
        //
        let result = scan_n_chars(s, context, 0, |c| c.is_alphabetic());
        assert_eq!((true, ScanPosition::default()), result);
    }

    #[test]
    fn test_scan_n_chars_ok_out_of_range() {
        let s = "foo bar";

        //
        // offset beyond end of string will not match
        // even for n of zero
        // and will return the byte and char indices unchanged.
        //
        let context = (true, ScanPosition::new(s.len() + 69, s.chars().count() + 69, 0, 0, 0));
        let result = scan_n_chars(s, context, 0, |c| c.is_alphabetic());
        assert_eq!((false, context.1), result)
    }

    #[test]
    fn test_scan_n_chars_ok_end_of_input() {
        let s = "foo bar";

        //
        // offset at end of input is no match,
        // returning byte offset and char offset unchanged.
        //
        let context = (true, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0));
        let result = scan_n_chars(s, context, 1, |c| c.is_alphabetic());
        assert_eq!((false, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0)), result);

        //
        // scanning zero at end of input will still match
        //
        let result = scan_n_chars(s, context, 0, |c| c.is_alphabetic());
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0)), result);
    }


    #[test]
    fn test_scan_chars_ok_sequentially() {
        let s = "foo\nbar";
        let context = (true, ScanPosition::default());

        //
        // scan the first 'f' character using a lambda
        //
        let result = scan_one_or_more_chars(s, context, |ch| ch == 'f');
        assert_eq!((true, ScanPosition::new('f'.len_utf8(), 1, 0, 0, 0)), result);

        //
        // scan the 'o' characters starting from last scan result
        //
        let result = scan_one_or_more_chars(s, result, |ch| ch == 'o');
        assert_eq!((true, ScanPosition::new("foo".len(), 3, 0, 0, 0)), result);

        //
        // scan the remaining underscore and alphabetic characters
        // starting from the last can result.
        //
        let result = scan_zero_or_more_chars(s, result, |ch| ch == '\n' || ch.is_alphabetic());
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 1, "foo\n".len(), "foo\n".chars().count())), result);

        //
        // do the same thing in one function call
        //
        let result = scan_zero_or_more_chars(s,
                        scan_one_or_more_chars(s,
                            scan_one_or_more_chars(s,
                                context,
                                |ch| ch == 'f'),
                            |ch| ch == 'o'),
                        |ch| ch == '\n' || ch.is_alphabetic());
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 1, "foo\n".len(), "foo\n".chars().count())), result);
    }

    #[test]
    fn test_scan_chars_ok_sequence_stops_after_no_match() {
        let s = "foo\nbar";
        let context = (true, ScanPosition::default());

        //
        // scanners should not continue scanning
        // after a mismatch is detected.
        //

        //
        // scan the first 'f' character using a lambda
        //
        let result = scan_one_or_more_chars(s, context, |ch| ch == 'f');
        assert_eq!((true, ScanPosition::new('f'.len_utf8(), 1, 0, 0, 0)), result);

        //
        // Attempt to scan 'x' characters starting from last scan result.
        // This will not match, so subsequent scanners should not match.
        //
        let result = scan_one_or_more_chars(s, result, |ch| ch == 'x');
        assert_eq!((false, ScanPosition::new("f".len(), 1, 0, 0, 0)), result);

        //
        // scan the remaining underscore and alphabetic characters
        // starting from the last can result.
        //
        let result = scan_zero_or_more_chars(s, result, |ch| ch == '\n' || ch.is_alphabetic());
        assert_eq!((false, ScanPosition::new("f".len(), 1, 0, 0, 0)), result);

        //
        // do the same thing in one function call
        //
        let result = scan_zero_or_more_chars(s,
                        scan_one_or_more_chars(s,
                            scan_one_or_more_chars(s,
                                context,
                                |ch| ch == 'f'),
                            |ch| ch == 'x'),
                        |ch| ch == '_' || ch.is_alphabetic());
        assert_eq!((false, ScanPosition::new("f".len(), 1, 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_lines_ok() {
        let s = "foo\nbar\r\nbaz";
        let context = (true, ScanPosition::default());

        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 2, "foo\nbar\r\n".len(), "foo\nbar\r\n".chars().count())), scan_literal(s, context, s));
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 2, "foo\nbar\r\n".len(), "foo\nbar\r\n".chars().count())), scan_zero_or_more_chars(s, context, |_ch| true));
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 2, "foo\nbar\r\n".len(), "foo\nbar\r\n".chars().count())), scan_one_or_more_chars(s, context, |_ch| true));
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 2, "foo\nbar\r\n".len(), "foo\nbar\r\n".chars().count())), scan_n_chars(s, context, s.len(), |_ch| true));
    }

    #[test]
    fn test_scan_lines_last_line_ending_ok() {
        let s = "foo\nbar\r\nβαρ\r\n";
        let context = (true, ScanPosition::default());

        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 3, s.len(), s.chars().count())), scan_literal(s, context, s));
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 3, s.len(), s.chars().count())), scan_zero_or_more_chars(s, context, |_ch| true));
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 3, s.len(), s.chars().count())), scan_one_or_more_chars(s, context, |_ch| true));
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 3, s.len(), s.chars().count())), scan_n_chars(s, context, s.chars().count(), |_ch| true));
    }

}
