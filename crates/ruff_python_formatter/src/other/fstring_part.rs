use crate::context::{InsideFormattedValue, SurroundingFStringQuotes};
use crate::expression::string::normalize_string;
use crate::prelude::*;
use crate::AsFormat;
use ruff_formatter::{write, Buffer, FormatResult};
use ruff_python_ast::{FStringPart, PartialString};

impl Format<PyFormatContext<'_>> for FStringPart {
    fn fmt(&self, f: &mut PyFormatter) -> FormatResult<()> {
        match self {
            FStringPart::Literal(PartialString { value: _, range }) => {
                let preferred_quotes = match f.context().inside_formatted_value() {
                    InsideFormattedValue::Inside(SurroundingFStringQuotes { closest, .. }) => {
                        closest
                    }
                    InsideFormattedValue::Outside => unreachable!(),
                };

                let string_content = f.context().locator().slice(*range);
                let (normalized, _contains_newlines) =
                    normalize_string(string_content, preferred_quotes, false);
                write!(f, [dynamic_text(&normalized, None)])
            }
            FStringPart::FormattedValue(formatted_value) => {
                write!(f, [formatted_value.format()])
            }
        }
    }
}
