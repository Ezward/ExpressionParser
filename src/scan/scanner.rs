//!
//! Higher order scanners using the [Scanner] trait.
//!
use super::context::ScanContext;

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
/// Implement [Scanner] trait for all [ScannerFn]
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
    let (matched, position) = context;
    if (!matched) || position.byte_index > s.len(){
        return (false, position)
    }

    // scanner_right.scan(s, scanner_left.scan(s, context))
    let mut result = scanner_left.scan(s, context);
    if result.0 {
        result = scanner_right.scan(s, result);
    }
    result
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
fn scan_all<T>(
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
    let (matched, position) = context;
    if (!matched) || position.byte_index > s.len(){
        return (false, position)
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

///
/// Scan for match by applying a sequence of scanners in order.
/// Scanning proceeds in the order the iterator provides that scanners
/// and stops after a scanner is matched OR all scanners do not match.
///
/// - **s**: the string to scan
/// - **context**: the current scanning state
/// - **scanners**: a iterable collection of [Scanner] to apply in order
/// - **returns**:
///   - The scan result as a [ScanContext]
///     - matched is true if a scanner matched (or there we no scanners)
///     - matched is false all scanners did not match or if context's byte offset is out of range
///     - byte offset is offset after last byte in last matching char (aka total number of bytes matched)
///     - char offset is offset after last matching char (aka total number of utf-8 chars matched)
///
#[allow(unused)]
fn scan_any<T>(
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
    let (matched, position) = context;
    if (!matched) || position.byte_index > s.len(){
        return (false, position)
    }

    let mut scanned = context;
    for scanner in scanners {
        scanned = scanner.scan(s, scanned);
        if scanned.0 {
            break;
        }
    }
    scanned
}


#[cfg(test)]
mod tests {
    use crate::scan::context::{scan_one_or_more_chars, scan_zero_or_more_chars, scan_n_chars, ScanPosition};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_scan_pair_ok() {
        let s = "foo123bar_doo_2";
        let context = (true, ScanPosition::default());

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
        assert_eq!((true, ScanPosition::new("foo123bar".len(), 9, 0, 0, 0)), result);

        //
        // scan with closure's coerced to function
        //
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric);
        assert_eq!((true, ScanPosition::new("foo123bar".len(), 9, 0, 0, 0)), result);

    }

    #[test]
    fn test_scan_pair_ok_out_of_range() {
        let s = "foo123bar_doo_2";

        //
        // offset beyond end of string will not match
        // and will return the byte and char indices unchanged.
        //
        let context = (true, ScanPosition::new(s.len() + 69, s.chars().count() + 69, 0, 0, 0));
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric);
        assert_eq!((false, context.1), result);

        //
        // scanning zero out of range will not match
        //
        let scan_0_chars: ScannerFn = |s, c| scan_n_chars(s, c, 0, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_0_chars, scan_0_chars);
        assert_eq!((false, context.1), result);
    }

    #[test]
    fn test_scan_pair_ok_end_of_input() {
        let s = "foo bar";

        //
        // offset at end of input is no match,
        // returning byte offset and char offset unchanged.
        //
        let context = (true, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0));
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric);
        assert_eq!((false, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0)), result);

        //
        // scanning zero at end of input will still match
        //
        let scan_0_chars: ScannerFn = |s, c| scan_n_chars(s, c, 0, |ch| ch.is_alphanumeric());
        let result = scan_pair(s, context, scan_0_chars, scan_0_chars);
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_pair_lines_ok() {
        let s = "foo\nbar\r\nbaz\r\n";
        let context = (true, ScanPosition::default());

        //
        // count number of alphabetic runs followed by new line
        //
        let scan_1_or_more_alphabetic: ScannerFn = |st, ctx| scan_one_or_more_chars(st, ctx, |ch| ch.is_alphabetic());

        //
        // We cannot use a capturing closure; here the scan_pair closure is using to captured local functions:
        //
        // ```
        // let scan_carriage_returns: ScannerFn = |st, ctx| scan_zero_or_more_chars(st, ctx, |ch| ch == '\r');
        // let scan_new_line: ScannerFn = |st, ctx| scan_n_chars(st, ctx, 1, |ch| ch == '\n');
        // let scan_line_ending: ScannerFn = |st, ctx| scan_pair(st, ctx, scan_carriage_returns, scan_new_line);
        // ```
        //
        // but it works if we use inline noncapturing closures
        //
        let scan_line_ending: ScannerFn = |st, ctx| scan_pair(st, ctx,
            (|st, ctx| scan_zero_or_more_chars(st, ctx, |ch| ch == '\r')) as ScannerFn,
            (|st, ctx| scan_n_chars(st, ctx, 1, |ch| ch == '\n')) as ScannerFn
        );

        // scan first line
        let result = scan_pair(s, context, scan_1_or_more_alphabetic, scan_line_ending);
        assert_eq!((true, ScanPosition::new("foo\n".len(), "foo\n".chars().count(), 1, "foo\n".len(), "foo\n".chars().count())), result);

        // scan second line
        let result = scan_pair(s, result, scan_1_or_more_alphabetic, scan_line_ending);
        assert_eq!((true, ScanPosition::new("foo\nbar\r\n".len(), "foo\nbar\r\n".chars().count(), 2, "foo\nbar\r\n".len(), "foo\nbar\r\n".chars().count())), result);

        // scan last line
        let result = scan_pair(s, result, scan_1_or_more_alphabetic, scan_line_ending);
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 3, s.len(), s.chars().count())), result);
    }

    #[test]
    fn test_scan_sequence_ok() {
        let s = "foo123bar_doo_2";
        let context = (true, ScanPosition::default());

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
        let result = scan_all(s, context, [scan_one_or_more_alphabetic, scan_zero_or_more_alphanumeric]);
        assert_eq!((true, ScanPosition::new("foo123bar".len(), 9, 0, 0, 0)), result);

        //
        // with non-capturing closures coerced to function
        //
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_all(s, context, [scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric]);
        assert_eq!((true, ScanPosition::new("foo123bar".len(), 9, 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_sequence_ok_out_of_range() {
        let s = "foo123bar_doo_2";

        //
        // offset beyond end of string will not match
        // and will return the byte and char indices unchanged.
        //
        let context = (true, ScanPosition::new(s.len() + 69, s.chars().count() + 69, 0, 0, 0));
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_all(s, context, [scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric]);
        assert_eq!((false, context.1), result)
    }

    #[test]
    fn test_scan_sequence_ok_end_of_input() {
        let s = "foo bar";

        //
        // offset at end of input is no match,
        // returning byte offset and char offset unchanged.
        //
        let context = (true, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0));
        let scan_1_or_more_alphabetic: ScannerFn = |s, c| scan_one_or_more_chars(s, c, |ch| ch.is_alphabetic());
        let scan_0_or_more_alphanumeric: ScannerFn = |s, c| scan_zero_or_more_chars(s, c, |ch| ch.is_alphanumeric());
        let result = scan_all(s, context, [scan_1_or_more_alphabetic, scan_0_or_more_alphanumeric]);
        assert_eq!((false, context.1), result);

        //
        // scanning zero at end of input will still match
        //
        let scan_0_chars: ScannerFn = |s, c| scan_n_chars(s, c, 0, |ch| ch.is_alphanumeric());
        let result = scan_all(s, context, [scan_0_chars, scan_0_chars]);
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0)), result);
    }

    #[test]
    fn test_scan_sequence_lines_ok() {
        let s = "foo\nbar\r\nbaz\r\n";
        let context = (true, ScanPosition::default());

        //
        // scanner for a line
        //
        let scan_line: ScannerFn = |st, ctx| scan_all(st, ctx, [
            (|st, ctx| scan_zero_or_more_chars(st, ctx, |ch| ch.is_alphabetic())) as ScannerFn,
            (|st, ctx| scan_zero_or_more_chars(st, ctx, |ch| ch == '\r')) as ScannerFn,
            (|st, ctx| scan_n_chars(st, ctx, 1, |ch| ch == '\n')) as ScannerFn
        ]);

        //
        // scan 3 lines
        //
        let result = scan_all(s, context, [scan_line, scan_line, scan_line]);
        assert_eq!((true, ScanPosition::new(s.len(), s.chars().count(), 3, s.len(), s.chars().count())), result);
    }
}