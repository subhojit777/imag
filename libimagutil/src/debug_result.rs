pub trait DebugResult<T, E> : Sized {

    fn map_dbg<F: FnOnce(&T) -> String>(self, f: F) -> Self;

    fn map_dbg_str(self, s: &str) -> Self {
        self.map_dbg(|_| format!("{}", s))
    }

    fn map_dbg_err<F: FnOnce(&E) -> String>(self, f: F) -> Self;

    fn map_dbg_err_str(self, s: &str) -> Self {
        self.map_dbg_err(|_| format!("{}", s))
    }

}

impl<T, E> DebugResult<T, E> for Result<T, E> {

    fn map_dbg<F: FnOnce(&T) -> String>(self, f: F) -> Self {
        self.map(|t| { debug!("{}", f(&t)); t })
    }

    fn map_dbg_err<F: FnOnce(&E) -> String>(self, f: F) -> Self {
        self.map_err(|e| { debug!("{}", f(&e)); e })
    }

}

