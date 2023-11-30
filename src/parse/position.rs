use crate::scan::context::ScanPosition;



///
/// start and end of expression in source
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
