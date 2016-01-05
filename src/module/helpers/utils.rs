/**
 * Utility helpers for CLI
 */
pub mod cli {
    use clap::ArgMatches;

    /**
     * Get a commandline option "tags" and split the argument by "," to be able to provide a
     * Vec<String> with the argument as array.
     */
    pub fn get_tags<'a>(sub: &ArgMatches<'a, 'a>) -> Vec<String> {

        fn reject_if_with_spaces(e: &String) -> bool {
            if e.contains(" ") {
                warn!("Tag contains spaces: '{}'", e);
                false
            } else {
                true
            }
        }

        debug!("Fetching tags from commandline");
        sub.value_of("tags").and_then(|tags| {
            Some(tags.split(",")
                     .into_iter()
                     .map(|s| s.to_string())
                     .filter(|e| reject_if_with_spaces(e))
                     .collect()
              )
        }).or(Some(vec![])).unwrap()
    }

}
