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

#[cfg(test)]
mod test {

    use super::generate_variants;

    #[test]
    fn test_variants_simple() {
        let base = 1;
        let vars = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let res = generate_variants(base, vars, &|base, var| base + var);

        assert!(res.len() == 11, format!("Length is {} instead of 11", res.len()));
        assert!(res.iter().all(|i| *i > 0));
        assert!(res == vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
    }

    #[test]
    fn test_variants_pathes() {
        use std::path::PathBuf;

        let base = PathBuf::from("/");
        let vars = vec!["foo", "bar", "baz"];
        let res = generate_variants(base, vars, &|base, var| {
            let mut base = base.clone();
            base.push(var);
            base
        });

        assert!(res.len() == 3, format!("Length is {} instead of 3", res.len()));
        let eq_vec = vec!["/foo", "/bar", "/baz"];
        let eq     = eq_vec.iter().map(PathBuf::from);
        assert!(res.into_iter().zip(eq).all(|(orig, equi)| orig == equi));
    }

}

