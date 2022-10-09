use pest::error::LineColLocation;

#[derive(Clone, Debug)]
pub struct Span {
    line: String,
    location: LineColLocation,
}

impl Span {
    pub fn as_str(&self) -> &str {
        self.line.as_ref()
    }

    pub fn line_col(&self) -> &LineColLocation {
        &self.location
    }
}

impl From<pest::Span<'_>> for Span {
    fn from(span: pest::Span) -> Self {
        Self {
            line: span.as_str().to_string(),
            location: LineColLocation::Span(span.start_pos().line_col(), span.end_pos().line_col()),
        }
    }
}
