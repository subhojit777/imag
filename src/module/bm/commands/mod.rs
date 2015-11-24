mod add {
    use clap::ArgMatches;

    use module::command::{CommandError, CommandResult, CommandEnv, ExecutableCommand};
    use runtime::Runtime;
    use storage::backend::{StorageBackend, StorageBackendError};

    pub struct AddCommand;

    pub impl AddCommand {

        fn new() -> AddCommand {
            AddCommand
        }

    }

    pub impl ExecutableCommand for AddCommand {

        fn get_callname() -> &'static str {
            "add"
        }

        fn exec<'a>(&self, env: CommandEnv<'a>) -> CommandResult
        {
        }

    }

}

mod list {
    use clap::ArgMatches;

    use module::command::{CommandError, CommandResult, CommandEnv, ExecutableCommand};
    use runtime::Runtime;
    use storage::backend::{StorageBackend, StorageBackendError};

    pub struct ListCommand;

    pub impl ListCommand {

        fn new() -> ListCommand {
            ListCommand
        }

    }

    pub impl ExecutableCommand for ListCommand {

        fn get_callname() -> &'static str {
            "list"
        }

        fn exec<'a>(&self, env: CommandEnv<'a>) -> CommandResult
        {
        }

    }

}

mod remove {
    use clap::ArgMatches;

    use module::command::{CommandError, CommandResult, CommandEnv, ExecutableCommand};
    use runtime::Runtime;
    use storage::backend::{StorageBackend, StorageBackendError};

    pub struct RemoveCommand;

    pub impl RemoveCommand {

        fn new() -> RemoveCommand {
            RemoveCommand
        }

    }

    pub impl ExecutableCommand for RemoveCommand {

        fn get_callname() -> &'static str {
            "remove"
        }

        fn exec<'a>(&self, env: CommandEnv<'a>) -> CommandResult
        {
        }

    }

}
