use crate::scan::context::ScanPosition;

///
/// The start and end position of an expression in the original source.
/// This will include all sub-expressions.
///
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParsePosition {
    pub start: ScanPosition,      // offset of start of expression in source
    pub end: ScanPosition,        // offset of end of expression in source
}
impl ParsePosition {
    pub fn new(start: &ScanPosition, end: &ScanPosition) -> ParsePosition{
        ParsePosition{
            start: *start,
            end: *end
        }
    }
}
