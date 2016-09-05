generate_error_module!(
    generate_error_types!(LinkError, LinkErrorKind,
        EntryHeaderReadError    => "Error while reading an entry header",
        EntryHeaderWriteError   => "Error while writing an entry header",
        ExistingLinkTypeWrong   => "Existing link entry has wrong type",
        LinkTargetDoesNotExist  => "Link target does not exist in the store",
        InternalConversionError => "Error while converting values internally",
        InvalidUri              => "URI is not valid",
        StoreReadError          => "Store read error",
        StoreWriteError         => "Store write error",
        StoreIdError            => "StoreId handling error"
    );
);

pub use self::error::LinkError;
pub use self::error::LinkErrorKind;
pub use self::error::MapErrInto;

