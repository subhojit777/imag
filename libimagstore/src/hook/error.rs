generate_error_imports!();
use std::convert::Into;

generate_error_types!(HookError, HookErrorKind,
    HookExecutionError  => "Hook exec error",
    AccessTypeViolation => "Hook access type violation"
);

