use clap::Parser;
use std::io::ErrorKind;
use std::io::Result;
use std::io::Error;

use super::input;

use super::model::Args;
use super::model::Cli;
use super::model::Commands;

pub fn get_args() -> Result<Args> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Args(arg_data)) => {
            return Ok(arg_data);
        }
        Some(Commands::Interactive) => {
            return Ok(get_interactive_args());
        }
        None => {
            println!("No command given. Use --help for more information.");
            return Err(Error::new(ErrorKind::Other, "No command given."));
        }
    }
}

fn get_interactive_args() -> Args {
    Args {
        name: input::string_must("Name"),
        url: input::string_must("URL"),
        description: input::string("Description", "An example application."),
        version: input::string("Version", "0.1.0"),
        author: input::string("Author", "hamza72x"),
        identifier: input::string("Identifier", "com.example.testapp"),
        icon: input::optional_string("Icon", "icon_path.png"),
        is_release_build: input::bool("Release build", true),
        user_agent: input::optional_string("User agent", "Mozilla/5.0"),
    }
}
