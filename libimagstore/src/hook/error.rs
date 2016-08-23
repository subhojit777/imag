use std::default::Default;

generate_error_imports!();

generate_custom_error_types!(HookError, HookErrorKind, CustomData,
    HookExecutionError  => "Hook exec error",
    AccessTypeViolation => "Hook access type violation",
    MutableHooksNotAllowed => "Mutable Hooks are denied"
);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
pub struct CustomData {
    aborting: bool,
}

impl CustomData {

    pub fn aborting(mut self, b: bool) -> CustomData {
        self.aborting = b;
        self
    }

}

impl Default for CustomData {

    fn default() -> CustomData {
        CustomData {
            aborting: true
        }
    }

}

impl HookError {

    pub fn is_aborting(&self) -> bool {
        match self.custom_data {
            Some(b) => b.aborting,
            None => true
        }
    }

}
