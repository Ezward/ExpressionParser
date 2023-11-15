//!
//! Higher order scanners using the [Scanner] trait.
//!
use super::scan::ScanContext;

///
/// A scanner function pointer that takes a string slice to
/// scan and the current [ScanContext] and returns
/// the result of the scan as a [ScanContext].
///
#[allow(unused)]
pub type ScannerFn = fn(&str, ScanContext) -> ScanContext;

///
/// Scanner's implement that scan() method, which
/// takes a string slice to scan and the current [ScanContext]
/// and returns the result of the scan as a [ScanContext].
///
pub trait Scanner {
    fn scan(self, s: &str, context: ScanContext) -> ScanContext;
}

///
/// All [ScannerFn] function pointers implement the [Scanner] trait.
///
impl Scanner for fn(&str, ScanContext) -> ScanContext {
    fn scan(self, s: &str, context: ScanContext) -> ScanContext {
        self(s, context)
    }
}


///
/// Scan for match by applying two scanners in order.
///
/// - **s**: the string to scan
/// - **context**: the current scanning state
/// - **scanner_left**: the first [Scanner] to apply
/// - **scanner_right**: the second [Scanner] to apply
/// - **returns**:
///   - The scan result as a [ScanContext]
///     - matched is true if both scanners matched
///     - matched is false either scanner did not match or if context's byte offset is out of range
///     - byte offset is offset after last byte in last matching char (aka total number of bytes matched)
///     - char offset is offset after last matching char (aka total number of utf-8 chars matched)
///
#[allow(unused)]
pub fn scan_pair(
    s: &str,                        // IN : the string to scan
    context: ScanContext,           // IN : the string and offset to scan
    scanner_left: impl Scanner,     // scanner left side of pair
    scanner_right: impl Scanner)    // scanner for right side of pair
    -> ScanContext                  // RET: scan result as an ScanContext
                                    //      matched is true if both scanners matched
                                    //      matched if false if either scanner did not match
                                    //      byte offset after last byte in last matching char (aka number of bytes matched)
                                    //      char offset after last matching char (aka number of utf-8 chars matched)
{
    let (matched, bytes, chars) = context;
    if (!matched) || bytes > s.len() {
        return (false, bytes, chars)
    }

    scanner_right.scan(s, scanner_left.scan(s, context))
}


///
/// Scan for match by applying a sequence of scanners in order.
/// Scanning proceeds in the order the iterator provides that scanners
/// and stops after all scanners are matched OR any scanner does not match.
///
/// - **s**: the string to scan
/// - **context**: the current scanning state
/// - **scanners**: a iterable collection of [Scanner] to apply in order
/// - **returns**:
///   - The scan result as a [ScanContext]
///     - matched is true if all scanners matched (or there we no scanners)
///     - matched is false any scanner did not match or if context's byte offset is out of range
///     - byte offset is offset after last byte in last matching char (aka total number of bytes matched)
///     - char offset is offset after last matching char (aka total number of utf-8 chars matched)
///
#[allow(unused)]
fn scan_sequence<T>(
    s: &str,                // IN : the string to scan
    context: ScanContext,   // IN : scanning state
    scanners: T)            // IN : iterable collection of scanners to apply in order
    -> ScanContext          // RET: scan result as an ScanContext
                            //      matched is true if all scanners matched
                            //      matched if false if any scanner did not match
                            //      byte offset after last byte in last matching char (aka number of bytes matched)
                            //      char offset after last matching char (aka number of utf-8 chars matched)
    where
        T: IntoIterator,
        T::Item: Scanner
{
    let (matched, bytes, chars) = context;
    if (!matched) || bytes > s.len() {
        return (false, bytes, chars)
    }

    let mut scanned = context;
    for scanner in scanners {
        scanned = scanner.scan(s, scanned);
        if ! scanned.0 {
            break;
        }
    }
    scanned
}

#[cfg(test)]
mod tests {
    use crate::scan::scan::{scan_one_or_more_chars, scan_zero_or_more_chars, scan_n_chars};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_scan_pair_ok() {
        let s = "foo123bar_doo_2";
        let context = (true, 0, 0);

        //
        // scan one or more alphabetic characters
        //
        fn scan_one_or_more_alphabetic(s: &str, context: ScanContext) -> ScanContext {
            scan_one_or_more_chars(s, context, |c| c.is_alphabetic())
        }

        //
        // scan zero or more alphanumeric characters
        //
        fn scan_zero_or_more_alphanumeric(s: &str, context: ScanContext) -> ScanContext {
            scan_zero_or_more_chars(s, context, |c| c.is_alphanumeric())
        }

        //
        // scan variable name that starts with alphabetic the zero or more alphanumerics
        //
        let result = scan_pair(s, context, scan_one_or_more_alphabetic as ScannerFn, scan_zero_or_more_alphanumeric as ScannerFn);
        assert_eq!((true, "foo123bar".len(), 9), result);

        //
        // scan with closure's coerced to function
        //
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric);
        assert_eq!((true, "foo123bar".len(), 9), result);

    }

    #[test]
    fn test_scan_pair_ok_out_of_range() {
        let s = "foo123bar_doo_2";

        //
        // offset beyond end of string will not match
        // and will return the byte and char indices unchanged.
        //
        let context = (true, s.len() + 69, s.chars().count() + 69);
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric);
        assert_eq!((false, context.1, context.2), result);

        //
        // scanning zero out of range will not match
        //
        let scan_0_chars: ScannerFn = |s, c| scan_n_chars(s, c, 0, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_0_chars, scan_0_chars);
        assert_eq!((false, context.1, context.2), result);
    }

    #[test]
    fn test_scan_pair_ok_end_of_input() {
        let s = "foo bar";

        //
        // offset at end of input is no match,
        // returning byte offset and char offset unchanged.
        //
        let context = (true, s.len(), s.chars().count());
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric);
        assert_eq!((false, s.len(), s.chars().count()), result);

        //
        // scanning zero at end of input will still match
        //
        let scan_0_chars: ScannerFn = |s, c| scan_n_chars(s, c, 0, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_0_chars, scan_0_chars);
        assert_eq!((true, s.len(), s.chars().count()), result);
    }


    #[test]
    fn test_scan_sequence_ok() {
        let s = "foo123bar_doo_2";
        let context = (true, 0, 0);

        //
        // scan one or more alphabetic characters
        //
        fn scan_one_or_more_alphabetic(s: &str, context: ScanContext) -> ScanContext {
            scan_one_or_more_chars(s, context, |c| c.is_alphabetic())
        }

        //
        // scan zero or more alphanumeric characters
        //
        fn scan_zero_or_more_alphanumeric(s: &str, context: ScanContext) -> ScanContext {
            scan_zero_or_more_chars(s, context, |c| c.is_alphanumeric())
        }

        //
        // scan variable name that starts with alphabetic the zero or more alphanumerics
        //
        let result = scan_sequence(s, context, [scan_one_or_more_alphabetic, scan_zero_or_more_alphanumeric]);
        assert_eq!((true, "foo123bar".len(), 9), result);

        //
        // with non-capturing closures coerced to function
        //
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_sequence(s, context, [scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric]);
        assert_eq!((true, "foo123bar".len(), 9), result);
    }

    #[test]
    fn test_scan_sequence_ok_out_of_range() {
        let s = "foo123bar_doo_2";

        //
        // offset beyond end of string will not match
        // and will return the byte and char indices unchanged.
        //
        let context = (true, s.len() + 69, s.chars().count() + 69);
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_sequence(s, context, [scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric]);
        assert_eq!((false, context.1, context.2), result)
    }

    #[test]
    fn test_scan_sequence_ok_end_of_input() {
        let s = "foo bar";

        //
        // offset at end of input is no match,
        // returning byte offset and char offset unchanged.
        //
        let context = (true, s.len(), s.chars().count());
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_sequence(s, context, [scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric]);
        assert_eq!((false, context.1, context.2), result);

        //
        // scanning zero at end of input will still match
        //
        let scan_0_chars: ScannerFn = |s, c| scan_n_chars(s, c, 0, |ch| ch.is_alphanumeric());
        let result = scan_sequence(s, context, [scan_0_chars, scan_0_chars]);
        assert_eq!((true, s.len(), s.chars().count()), result);
    }
}