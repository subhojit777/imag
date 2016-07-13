/// Processes `iter` returning the last successful result or the first error.
pub fn fold_ok<X, I, R, E, F>(iter: I, mut func: F) -> Result<R, E>
    where I: Iterator<Item = X>,
          R: Default,
          F: FnMut(X) -> Result<R, E>
{
    iter.fold(Ok(R::default()), |acc, item| acc.and_then(|_| func(item)))
}
