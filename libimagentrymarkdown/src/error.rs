generate_error_module!(
    generate_error_types!(MarkdownError, MarkdownErrorKind,
        MarkdownParsingError    => "Markdown parsing error"
    );
);

pub use self::error::MarkdownError;
pub use self::error::MarkdownErrorKind;


