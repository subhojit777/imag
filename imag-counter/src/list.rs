use libimagrt::runtime::Runtime;

pub fn list(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("list")
        .map(|scmd| {
            debug!("Found 'list' subcommand...");

        });
}
