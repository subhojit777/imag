#[macro_export]
macro_rules! generate_result_logging_extension {
    {
        $name: ident,
        $map_name: ident,
        $map_str_name: ident,
        $map_err_name: ident,
        $map_err_str_name: ident,
        $closure: expr
    } => {
        pub trait InfoResult<T, E> : Sized {

            fn $map_name<F: FnOnce(&T) -> String>(self, f: F) -> Self;

            fn $map_str_name(self, s: &str) -> Self {
                self.$map_name(|_| format!("{}", s))
            }

            fn $map_err_name<F: FnOnce(&E) -> String>(self, f: F) -> Self;

            fn $map_err_str_name(self, s: &str) -> Self {
                self.$map_err_name(|_| format!("{}", s))
            }

        }

        impl<T, E> InfoResult<T, E> for Result<T, E> {

            fn $map_name<F: FnOnce(&T) -> String>(self, f: F) -> Self {
                self.map(|x| { $closure(f(&x)); x })
            }

            fn $map_err_name<F: FnOnce(&E) -> String>(self, f: F) -> Self {
                self.map_err(|e| { $closure(f(&e)); e })
            }

        }

    }
}
