/// Folds its contents to a result.
pub trait FoldResult: Sized {
    type Item;

    /// Processes all contained items returning the last successful result or the first error.
    /// If there are no items, returns `Ok(R::default())`.
    fn fold_defresult<R, E, F>(self, func: F) -> Result<R, E>
        where R: Default,
              F: FnMut(Self::Item)
        -> Result<R, E>
    {
        self.fold_result(R::default(), func)
    }

    /// Processes all contained items returning the last successful result or the first error.
    /// If there are no items, returns `Ok(default)`.
    fn fold_result<R, E, F>(self, default: R, mut func: F) -> Result<R, E>
        where F: FnMut(Self::Item) -> Result<R, E>;
}

impl<X, I: Iterator<Item = X>> FoldResult for I {
    type Item = X;

    fn fold_result<R, E, F>(self, default: R, mut func: F) -> Result<R, E>
        where F: FnMut(Self::Item) -> Result<R, E>
    {
        self.fold(Ok(default), |acc, item| acc.and_then(|_| func(item)))
    }
}

