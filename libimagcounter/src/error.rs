generate_error_module!(
    generate_error_types!(CounterError, CounterErrorKind,
        StoreIdError            => "StoreId error",
        StoreReadError          => "Store read error",
        StoreWriteError         => "Store write error",
        HeaderTypeError         => "Header type error",
        HeaderFieldMissingError => "Header field missing error"
    );
);

pub use self::error::CounterError;
pub use self::error::CounterErrorKind;

