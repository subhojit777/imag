generate_error_imports!();

generate_error_types!(CounterError, CounterErrorKind,
    StoreReadError          => "Store read error",
    StoreWriteError         => "Store write error",
    HeaderTypeError         => "Header type error",
    HeaderFieldMissingError => "Header field missing error"
);

