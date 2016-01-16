pub trait SingleUseLock<T> {
    fn access(&self) -> &T;
    fn unlock(self) -> T;
}

