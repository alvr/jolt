use anyhow::Result;
use clap::{
    Parser, Subcommand,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
    crate_description, crate_version,
};

const STYLES: Styles = Styles::styled()
    .header(
        AnsiColor::BrightYellow
            .on_default()
            .effects(Effects::BOLD.insert(Effects::UNDERLINE)),
    )
    .usage(
        AnsiColor::BrightYellow
            .on_default()
            .effects(Effects::BOLD.insert(Effects::UNDERLINE)),
    )
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::Green.on_default())
    .invalid(AnsiColor::BrightMagenta.on_default().effects(Effects::CURLY_UNDERLINE))
    .error(AnsiColor::BrightRed.on_default().effects(Effects::INVERT))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(author)]
#[command(
    name = "jolt",
    about = crate_description!(),
    version = crate_version!(),
    styles = STYLES,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Installs a specific version of a JDK from a specified distribution")]
    Install {
        #[arg(help = "(required): The distribution of the JDK to install (e.g., openjdk, oracle)")]
        distribution: String,

        #[arg(help = "(required): The specific version of the JDK to install (e.g., 11, 17)")]
        version: String,
    },

    #[command(about = "Lists the JDKs installed on the system")]
    List {
        #[arg(
            short,
            long = "dist",
            help = "(optional): Filters the installed JDKs by distribution (e.g., openjdk)"
        )]
        distribution: Option<String>,

        #[arg(short, long, help = "(optional): Filters the installed JDKs by version (e.g., 11)")]
        version: Option<String>,
    },

    #[command(about = "Searches for available JDKs to download")]
    Search {
        #[arg(
            short,
            long = "dist",
            help = "(optional, required unless version is provided): Filters the available JDKs by distribution (e.g., openjdk)",
            required_unless_present = "version"
        )]
        distribution: Option<String>,

        #[arg(
            short,
            long,
            help = "(optional, required unless distribution is provided): Filters the available JDKs by version (e.g., 11)",
            required_unless_present = "distribution"
        )]
        version: Option<String>,
    },

    #[command(about = "Updates the list of available JDKs")]
    Update,
}

pub fn cli() -> Result<()> {
    match Cli::parse().command {
        Commands::Install {
            distribution: _,
            version: _,
        } => todo!(),
        Commands::List {
            distribution: _,
            version: _,
        } => todo!(),
        Commands::Search {
            distribution: _,
            version: _,
        } => todo!(),
        Commands::Update => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_no_command() {
        let args = vec!["jolt"];
        let cli = Cli::try_parse_from(args);

        match cli {
            Err(e) => assert_eq!(e.exit_code(), 2),
            _ => panic!("Expected error for missing command"),
        }
    }

    #[test]
    fn test_install_command() {
        let args = vec!["jolt", "install", "openjdk", "11"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Install { distribution, version } => {
                assert_eq!(distribution, "openjdk");
                assert_eq!(version, "11");
            }
            _ => panic!("Expected Install command"),
        }
    }

    #[test]
    fn test_install_command_missing_distribution_arg() {
        let args = vec!["jolt", "install", "11"];
        let cli = Cli::try_parse_from(args);

        match cli {
            Err(e) => assert_eq!(e.exit_code(), 2),
            _ => panic!("Expected error for missing version"),
        }
    }

    #[test]
    fn test_install_command_missing_version_arg() {
        let args = vec!["jolt", "install", "openjdk"];
        let cli = Cli::try_parse_from(args);

        match cli {
            Err(e) => assert_eq!(e.exit_code(), 2),
            _ => panic!("Expected error for missing version"),
        }
    }

    #[test]
    fn test_list_command_without_args() {
        let args = vec!["jolt", "list"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::List { distribution, version } => {
                assert_eq!(distribution, None);
                assert_eq!(version, None);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_list_command_with_both_args_long() {
        let args = vec!["jolt", "list", "--dist", "openjdk", "--version", "11"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::List { distribution, version } => {
                assert_eq!(distribution, Some("openjdk".to_string()));
                assert_eq!(version, Some("11".to_string()));
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_list_command_with_both_args_short() {
        let args = vec!["jolt", "list", "-d", "openjdk", "-v", "11"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::List { distribution, version } => {
                assert_eq!(distribution, Some("openjdk".to_string()));
                assert_eq!(version, Some("11".to_string()));
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_list_command_with_distribution_arg_long() {
        let args = vec!["jolt", "list", "--dist", "openjdk"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::List { distribution, version } => {
                assert_eq!(distribution, Some("openjdk".to_string()));
                assert_eq!(version, None);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_list_command_with_distribution_arg_short() {
        let args = vec!["jolt", "list", "-d", "openjdk"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::List { distribution, version } => {
                assert_eq!(distribution, Some("openjdk".to_string()));
                assert_eq!(version, None);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_list_command_with_version_arg_long() {
        let args = vec!["jolt", "list", "--version", "11"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::List { distribution, version } => {
                assert_eq!(distribution, None);
                assert_eq!(version, Some("11".to_string()));
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_list_command_with_version_arg_short() {
        let args = vec!["jolt", "list", "-v", "11"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::List { distribution, version } => {
                assert_eq!(distribution, None);
                assert_eq!(version, Some("11".to_string()));
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_search_command_without_args() {
        let args = vec!["jolt", "search"];
        let cli = Cli::try_parse_from(args);

        match cli {
            Err(e) => assert_eq!(e.exit_code(), 2),
            _ => panic!("Expected error for missing version"),
        }
    }

    #[test]
    fn test_search_command_with_both_args_long() {
        let args = vec!["jolt", "search", "--dist", "openjdk", "--version", "11"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Search { distribution, version } => {
                assert_eq!(distribution, Some("openjdk".to_string()));
                assert_eq!(version, Some("11".to_string()));
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_search_command_with_both_args_short() {
        let args = vec!["jolt", "search", "-d", "openjdk", "-v", "11"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Search { distribution, version } => {
                assert_eq!(distribution, Some("openjdk".to_string()));
                assert_eq!(version, Some("11".to_string()));
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_search_command_with_distribution_arg_long() {
        let args = vec!["jolt", "search", "--dist", "openjdk"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Search { distribution, version } => {
                assert_eq!(distribution, Some("openjdk".to_string()));
                assert_eq!(version, None);
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_search_command_with_distribution_arg_short() {
        let args = vec!["jolt", "search", "-d", "openjdk"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Search { distribution, version } => {
                assert_eq!(distribution, Some("openjdk".to_string()));
                assert_eq!(version, None);
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_search_command_with_version_arg_long() {
        let args = vec!["jolt", "search", "--version", "11"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Search { distribution, version } => {
                assert_eq!(distribution, None);
                assert_eq!(version, Some("11".to_string()));
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_search_command_with_version_arg_short() {
        let args = vec!["jolt", "search", "-v", "11"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Search { distribution, version } => {
                assert_eq!(distribution, None);
                assert_eq!(version, Some("11".to_string()));
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_update_command() {
        let args = vec!["jolt", "update"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Update => {
                // Success if this branch is reached
            }
            _ => panic!("Expected Update command"),
        }
    }
}
