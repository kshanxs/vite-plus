//! Options written before the command they belong to.
//!
//! pnpm accepts `pnpm --filter app run dev` as well as
//! `pnpm run --filter app dev`. It finds the command first, then parses every
//! option against that command's option table. [`hoist_leading_options`] does
//! the same with the clap definitions: it finds the command, checks the tokens
//! in front of it against the options that command declares, and moves them
//! behind it so the regular parser sees the usual order.

use clap::{Arg, Command};

/// Move options written before a command to just after it.
///
/// `args` is the user's argument list without the program name. It is returned
/// unchanged unless every token before the command is an option that command
/// declares (or the value of one). Commands that forward their arguments to a
/// tool declare no options, so `vp -w build` is never rewritten.
#[must_use]
pub fn hoist_leading_options(root: &Command, mut args: Vec<String>) -> Vec<String> {
    let starts_with_command_option = args
        .first()
        .is_some_and(|arg| arg.starts_with('-') && !is_declared_option(root, &args[..1]));
    if !starts_with_command_option {
        return args;
    }
    let command_index = (1..args.len()).find(|&index| {
        root.find_subcommand(&args[index])
            .is_some_and(|command| is_declared_option(command, &args[..index]))
    });
    if let Some(command_index) = command_index {
        args[..=command_index].rotate_right(1);
    }
    args
}

/// Whether `tokens` consists only of options `command` declares and their values.
fn is_declared_option(command: &Command, tokens: &[String]) -> bool {
    let mut tokens = tokens.iter();
    while let Some(token) = tokens.next() {
        let needs_value = if let Some(long) = token.strip_prefix("--") {
            let (name, inline_value) = match long.split_once('=') {
                Some((name, _)) => (name, true),
                None => (long, false),
            };
            let Some(takes_value) = find_long(command, name).map(takes_value) else {
                return false;
            };
            if inline_value && !takes_value {
                return false;
            }
            takes_value && !inline_value
        } else if let Some(shorts) = token.strip_prefix('-') {
            let mut needs_value = false;
            let mut shorts = shorts.chars();
            while let Some(short) = shorts.next() {
                let Some(takes_value) = find_short(command, short).map(takes_value) else {
                    return false;
                };
                if takes_value {
                    // `-Fapp` and `-F=app` carry the value; `-F` takes the next token.
                    needs_value = shorts.as_str().is_empty();
                    break;
                }
            }
            if token.len() == 1 {
                return false;
            }
            needs_value
        } else {
            return false;
        };
        if needs_value && tokens.next().is_none() {
            return false;
        }
    }
    true
}

fn find_long<'a>(command: &'a Command, name: &str) -> Option<&'a Arg> {
    command.get_arguments().find(|arg| {
        arg.get_long() == Some(name)
            || arg.get_all_aliases().is_some_and(|aliases| aliases.contains(&name))
    })
}

fn find_short(command: &Command, short: char) -> Option<&Arg> {
    command.get_arguments().find(|arg| {
        arg.get_short() == Some(short)
            || arg.get_all_short_aliases().is_some_and(|aliases| aliases.contains(&short))
    })
}

fn takes_value(arg: &Arg) -> bool {
    arg.get_action().takes_values()
}

#[cfg(test)]
mod tests {
    use clap::{Args, Parser, Subcommand};

    use super::*;

    #[derive(Parser)]
    struct Cli {
        #[arg(short = 'V', long)]
        version: bool,
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        #[command(visible_alias = "r")]
        Run(RunArgs),
        /// Forwards everything to a tool.
        Build {
            #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
            args: Vec<String>,
        },
    }

    #[derive(Args)]
    struct RunArgs {
        #[arg(short = 'r', long)]
        recursive: bool,
        #[arg(short = 'w', long)]
        workspace_root: bool,
        #[arg(short = 'F', long, visible_alias = "scope")]
        filter: Vec<String>,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        task: Vec<String>,
    }

    fn hoist(args: &[&str]) -> Vec<String> {
        hoist_leading_options(
            &<Cli as clap::CommandFactory>::command(),
            args.iter().map(ToString::to_string).collect(),
        )
    }

    #[test]
    fn moves_leading_options_after_the_command() {
        assert_eq!(hoist(&["--filter", "app", "run", "dev"]), ["run", "--filter", "app", "dev"]);
        assert_eq!(hoist(&["--filter=app", "run", "dev"]), ["run", "--filter=app", "dev"]);
        assert_eq!(hoist(&["-F", "app", "-r", "run", "dev"]), ["run", "-F", "app", "-r", "dev"]);
        assert_eq!(hoist(&["-Fapp", "run", "dev"]), ["run", "-Fapp", "dev"]);
        assert_eq!(hoist(&["--recursive", "run", "dev"]), ["run", "--recursive", "dev"]);
        // Combined short flags, with and without a trailing value.
        assert_eq!(hoist(&["-rw", "run", "dev"]), ["run", "-rw", "dev"]);
        assert_eq!(hoist(&["-rF", "app", "run", "dev"]), ["run", "-rF", "app", "dev"]);
        // Aliases of both the option and the command.
        assert_eq!(hoist(&["--scope", "app", "r", "dev"]), ["r", "--scope", "app", "dev"]);
        // Options after the command stay where they are.
        assert_eq!(hoist(&["-r", "run", "dev", "-w"]), ["run", "-r", "dev", "-w"]);
    }

    #[test]
    fn an_option_value_is_not_mistaken_for_the_command() {
        assert_eq!(hoist(&["--filter", "run", "run", "dev"]), ["run", "--filter", "run", "dev"]);
        assert_eq!(hoist(&["--filter", "run"]), ["--filter", "run"]);
    }

    #[test]
    fn leaves_everything_else_alone() {
        // Not an option of the command.
        assert_eq!(hoist(&["--parallel", "run", "dev"]), ["--parallel", "run", "dev"]);
        assert_eq!(hoist(&["-rx", "run", "dev"]), ["-rx", "run", "dev"]);
        assert_eq!(hoist(&["--recursive=yes", "run"]), ["--recursive=yes", "run"]);
        // A command that forwards its arguments declares no options.
        assert_eq!(hoist(&["-w", "build"]), ["-w", "build"]);
        // Options of the program itself keep their meaning.
        assert_eq!(hoist(&["-V", "run"]), ["-V", "run"]);
        assert_eq!(hoist(&["--version", "run"]), ["--version", "run"]);
        // No command, no leading option, or not a command at all.
        assert_eq!(hoist(&["-r"]), ["-r"]);
        assert_eq!(hoist(&["run", "-r", "dev"]), ["run", "-r", "dev"]);
        assert_eq!(hoist(&["-r", "unknown"]), ["-r", "unknown"]);
        assert_eq!(hoist(&["-", "run"]), ["-", "run"]);
        assert!(hoist(&[]).is_empty());
    }
}
