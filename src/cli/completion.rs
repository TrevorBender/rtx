use clap::builder::PossibleValue;
use clap::ValueEnum;
use eyre::Result;

use crate::cli::Cli;
use crate::shell::completions;

/// Generate shell completions
#[derive(Debug, clap::Args)]
#[clap(aliases = ["complete", "completions"], verbatim_doc_comment, after_long_help = AFTER_LONG_HELP)]
pub struct Completion {
    /// Shell type to generate completions for
    #[clap(required_unless_present = "shell_type")]
    shell: Option<Shell>,

    /// Shell type to generate completions for
    #[clap(long = "shell", short = 's', hide = true)]
    shell_type: Option<Shell>,

    /// Always use usage for completions.
    /// Currently, usage is the default for fish and bash but not zsh since it has a few quirks
    /// to work out first.
    ///
    /// This requires the `usage` CLI to be installed.
    /// https://usage.jdx.dev
    #[clap(long, verbatim_doc_comment)]
    usage: bool,
}

impl Completion {
    pub fn run(mut self) -> Result<()> {
        let shell = self.shell.or(self.shell_type).unwrap();
        if let Shell::Bash | Shell::Fish = shell {
            self.usage = true;
        }

        let script = if self.usage {
            match self.call_usage(shell) {
                Ok(script) => script,
                Err(e) => {
                    debug!("usage command failed, falling back to prerendered completions");
                    debug!("error: {e:?}");
                    self.prerendered(shell)
                }
            }
        } else {
            completions::zsh_complete(&Cli::command())?
        };
        miseprintln!("{}", script.trim());

        Ok(())
    }

    fn call_usage(&self, shell: Shell) -> std::io::Result<String> {
        cmd!(
            "usage",
            "generate",
            "completion",
            shell.to_string(),
            "mise",
            "--usage-cmd",
            "mise usage"
        )
        .read()
    }

    fn prerendered(&self, shell: Shell) -> String {
        match shell {
            Shell::Bash => include_str!("../../completions/mise.bash"),
            Shell::Elvish => include_str!("../../completions/mise.elv"),
            Shell::Fish => include_str!("../../completions/mise.fish"),
            Shell::Zsh => include_str!("../../completions/_mise"),
        }
        .to_string()
    }
}

static AFTER_LONG_HELP: &str = color_print::cstr!(
    r#"<bold><underline>Examples:</underline></bold>

    $ <bold>mise completion bash > /etc/bash_completion.d/mise</bold>
    $ <bold>mise completion zsh  > /usr/local/share/zsh/site-functions/_mise</bold>
    $ <bold>mise completion fish > ~/.config/fish/completions/mise.fish</bold>
    $ <bold>eval (mise completion elvish | slurp)</bold>
"#
);

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
enum Shell {
    Bash,
    Elvish,
    Fish,
    Zsh,
}

impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Bash, Self::Elvish, Self::Fish, Self::Zsh]
    }
    //fn from_str(input: &str, _ignore_case: bool) -> std::result::Result<Self, String> {
        //match input {
            //"bash" => Ok(Self::Bash),
            //"elvish" => Ok(Self::Elvish),
            //"fish" => Ok(Self::Fish),
            //"zsh" => Ok(Self::Zsh),
            //_ => Err(format!("unknown shell type: {}", input)),
        //}
    //}

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.to_string()))
    }
}

//impl Display for Shell {
    //fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //match self {
            //Self::Bash => write!(f, "bash"),
            //Self::Elvish => write!(f, "elvish"),
            //Self::Fish => write!(f, "fish"),
            //Self::Zsh => write!(f, "zsh"),
        //}
    //}
//}
#[cfg(test)]
mod tests {
    #[test]
    fn test_completion() {
        assert_cli!("completion", "zsh");
        assert_cli!("completion", "bash");
        assert_cli!("completion", "elvish");
        assert_cli!("completion", "fish");
    }
}
