use std::path::Path;

use indoc::formatdoc;

use crate::shell::bash::Bash;
use crate::shell::{is_dir_in_path, Shell};

#[derive(Default)]
pub struct Zsh {}

impl Shell for Zsh {
    fn activate(&self, exe: &Path, status: bool) -> String {
        let dir = exe.parent().unwrap();
        let exe = exe.display();
        let status = if status { " --status" } else { "" };
        let mut out = String::new();

        // much of this is from direnv
        // https://github.com/direnv/direnv/blob/cb5222442cb9804b1574954999f6073cc636eff0/internal/cmd/shell_zsh.go#L10-L22
        if !is_dir_in_path(dir) {
            out.push_str(&format!("export PATH=\"{}:$PATH\"\n", dir.display()));
        }
        out.push_str(&formatdoc! {r#"
            export RTX_SHELL=zsh

            rtx() {{
              local command
              command="${{1:-}}"
              if [ "$#" = 0 ]; then
                command {exe}
                return
              fi
              shift

              case "$command" in
              deactivate|shell)
                eval "$({exe} "$command" "$@")"
                ;;
              *)
                command {exe} "$command" "$@"
                ;;
              esac
            }}

            _rtx_hook() {{
              trap -- '' SIGINT;
              eval "$("{exe}" hook-env{status} -s zsh)";
              trap - SIGINT;
            }}
            typeset -ag precmd_functions;
            if [[ -z "${{precmd_functions[(r)_rtx_hook]+1}}" ]]; then
              precmd_functions=( _rtx_hook ${{precmd_functions[@]}} )
            fi
            typeset -ag chpwd_functions;
            if [[ -z "${{chpwd_functions[(r)_rtx_hook]+1}}" ]]; then
              chpwd_functions=( _rtx_hook ${{chpwd_functions[@]}} )
            fi

            _rtx_hook
            "#});

        out
    }

    fn deactivate(&self) -> String {
        formatdoc! {r#"
        precmd_functions=( ${{precmd_functions[(r)_rtx_hook]}} )
        chpwd_functions=( ${{chpwd_functions[(r)_rtx_hook]}} )
        unset -f _rtx_hook
        unset -f rtx
        unset RTX_SHELL
        "#}
    }

    fn set_env(&self, k: &str, v: &str) -> String {
        Bash::default().set_env(k, v)
    }

    fn unset_env(&self, k: &str) -> String {
        Bash::default().unset_env(k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::replace_path;
    use insta::assert_snapshot;

    #[test]
    fn test_hook_init() {
        let zsh = Zsh::default();
        let exe = Path::new("/some/dir/rtx");
        assert_snapshot!(zsh.activate(exe, true));
    }

    #[test]
    fn test_set_env() {
        assert_snapshot!(Zsh::default().set_env("FOO", "1"));
    }

    #[test]
    fn test_unset_env() {
        assert_snapshot!(Zsh::default().unset_env("FOO"));
    }

    #[test]
    fn test_deactivate() {
        let deactivate = Zsh::default().deactivate();
        assert_snapshot!(replace_path(&deactivate));
    }
}
