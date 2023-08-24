use crate::context::PyFormatContext;
use crate::expression::parentheses::{NeedsParentheses, OptionalParentheses};
use crate::prelude::{dynamic_text, format_with, space, text};
use crate::{AsFormat, FormatNodeRule, PyFormatter};
use ruff_formatter::{write, Buffer, FormatResult};
use ruff_python_ast::node::AnyNodeRef;
use ruff_python_ast::{ConversionFlag, Expr, FormattedValue};

#[derive(Default)]
pub struct FormatFormattedValue;

impl FormatNodeRule<FormattedValue> for FormatFormattedValue {
    fn fmt_fields(&self, item: &FormattedValue, f: &mut PyFormatter) -> FormatResult<()> {
        let FormattedValue {
            range: _,
            expression,
            debug_text,
            conversion,
            format_spec,
        } = item;

        let conversion_text = match conversion {
            ConversionFlag::Repr => Some("!r"),
            ConversionFlag::Str => Some("!s"),
            ConversionFlag::Ascii => Some("!a"),
            ConversionFlag::None => None,
        };

        // if expression starts with a `{`, we need a space to avoid turning into a literal curly
        // with `{{`
        let extra_space = match **expression {
            Expr::Dict(_) | Expr::DictComp(_) | Expr::Set(_) | Expr::SetComp(_) => Some(space()),
            _ => None,
        };

        // TODO: do we want to subtract this space from the debug_text (can both happen at once?)

        let formatted_format_spec = format_with(|f| f.join().entries(format_spec.iter()).finish());

        write!(
            f,
            [
                text("{"),
                debug_text.as_ref().map(|d| dynamic_text(&d.leading, None)),
                extra_space,
                expression.format(),
                // not strictly needed here but makes for nicer symmetry
                extra_space,
                debug_text.as_ref().map(|d| dynamic_text(&d.trailing, None)),
                conversion_text.map(text),
                (!format_spec.is_empty()).then_some(text(":")),
                formatted_format_spec,
                text("}")
            ]
        )
    }
}

impl NeedsParentheses for FormattedValue {
    fn needs_parentheses(
        &self,
        _parent: AnyNodeRef,
        _context: &PyFormatContext,
    ) -> OptionalParentheses {
        OptionalParentheses::Multiline
    }
}
