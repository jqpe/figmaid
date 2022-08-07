pub mod config;

use clap::{crate_version, Command as Cmd};

pub fn cli() -> Cmd<'static> {
    Cmd::new("figmaid")
        .version(crate_version!())
        .about("Web server that allows you to use locally installed fonts in Figma")
        .subcommand(
            Cmd::new("config")
                .about(
                    "Create, open and validate configuration\
                \n\nRun without subcommands to print current directories and amount of loaded fonts",
                )
                .subcommand(Cmd::new("create").about("Create default configuration file"))
                .subcommand(Cmd::new("validate").about("Validate configuration"))
                .subcommand(Cmd::new("open").about("Open configuration file in text editor")),
        )
}
