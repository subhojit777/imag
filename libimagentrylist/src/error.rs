generate_error_imports!();

generate_error_types!(ListError, ListErrorKind,
    FormatError    => "FormatError",
    EntryError     => "EntryError",
    IterationError => "IterationError",
    CLIError       => "No CLI subcommand for listing entries"
);

