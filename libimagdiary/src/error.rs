generate_error_module!(
    generate_error_types!(DiaryError, DiaryErrorKind,
        StoreWriteError     => "Error writing store",
        StoreReadError      => "Error reading store",
        CannotFindDiary     => "Cannot find diary",
        CannotCreateNote    => "Cannot create Note object for diary entry",
        DiaryEditError      => "Cannot edit diary entry",
        PathConversionError => "Error while converting paths internally",
        EntryNotInDiary     => "Entry not in Diary",
        IOError             => "IO Error"
    );
);

pub use self::error::DiaryError;
pub use self::error::DiaryErrorKind;

