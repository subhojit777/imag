generate_error_module!(
    generate_error_types!(TagError, TagErrorKind,
        TagTypeError     => "Entry Header Tag Type wrong",
        HeaderReadError  => "Error while reading entry header",
        HeaderWriteError => "Error while writing entry header",
        NotATag          => "String is not a tag"
    );
);

pub use self::error::TagError;
pub use self::error::TagErrorKind;

