generate_error_imports!();

generate_error_types!(NoteError, NoteErrorKind,
    StoreWriteError       => "Error writing store",
    StoreReadError        => "Error reading store",
    HeaderTypeError       => "Header type error",
    NoteToEntryConversion => "Error converting Note instance to Entry instance"
);

