generate_error_module!(
    generate_error_types!(NoteError, NoteErrorKind,
        StoreWriteError       => "Error writing store",
        StoreReadError        => "Error reading store",
        HeaderTypeError       => "Header type error",
        NoteToEntryConversion => "Error converting Note instance to Entry instance"
    );
);

pub use self::error::NoteError;
pub use self::error::NoteErrorKind;
pub use self::error::MapErrInto;

