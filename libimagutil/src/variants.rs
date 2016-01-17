/**
 * Generate variants of a base value by applying parts
 *
 * Example:
 *
 * ```ignore
 * generate_variants(path, vec!["foo", "bar", "baz"], |b, v| {
 *    let b = b.clone();
 *    b.push(v);
 *    b
 * })
 *
 * ```
 *
 */
pub fn generate_variants<A, B, C, F>(base: A, modders: Vec<B>, f: &F)
    -> Vec<C>
    where
        F: Fn(&A, B) -> C
{
    modders.into_iter().map(|m| f(&base, m)).collect()
}

