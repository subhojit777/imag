generate_error_module!(
    generate_error_types!(TodoError, TodoErrorKind,
        ConversionError     => "Conversion Error",
        StoreError          => "Store Error",
        StoreIdError        => "Store Id handling error",
        ImportError         => "Error importing"
    );
);

pub use self::error::TodoError;
pub use self::error::TodoErrorKind;
pub use self::error::MapErrInto;

