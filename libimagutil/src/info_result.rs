pub trait InfoResult<T, E> : Sized {

    fn map_info<F: FnOnce(&T) -> String>(self, f: F) -> Self;

    fn map_info_str(self, s: &str) -> Self {
        self.map_info(|_| format!("{}", s))
    }

    fn map_info_err<F: FnOnce(&E) -> String>(self, f: F) -> Self;

    fn map_info_err_str(self, s: &str) -> Self {
        self.map_info_err(|_| format!("{}", s))
    }

}

impl<T, E> InfoResult<T, E> for Result<T, E> {

    fn map_info<F: FnOnce(&T) -> String>(self, f: F) -> Self {
        self.map(|t| { info!("{}", f(&t)); t })
    }

    fn map_info_err<F: FnOnce(&E) -> String>(self, f: F) -> Self {
        self.map_err(|e| { info!("{}", f(&e)); e })
    }

}

