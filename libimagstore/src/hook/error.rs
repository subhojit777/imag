use std::convert::Into;
generate_error_imports!();

generate_custom_error_types!(HookError, HookErrorKind, CustomData,
    HookExecutionError  => "Hook exec error",
    AccessTypeViolation => "Hook access type violation"
);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
pub struct CustomData {
    aborting: bool,
}

impl HookError {

    pub fn is_aborting(&self) -> bool {
        match self.custom_data {
            Some(b) => b.aborting,
            None => true
        }
    }

}
