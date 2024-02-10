use std::path::Path;

use indoc::formatdoc;

use crate::shell::Shell;

#[derive(Default)]
pub struct Elvish {}

impl Shell for Elvish {
    fn activate(&self, exe: &Path, flags: String) -> String {
        let exe = exe.to_string_lossy();
        let mut out = String::new();
        out.push_str(&formatdoc! {r#"
            set-env RTX_SHELL elvish

            edit:add-var rtx~ {{|@args|
              var command
              if (> (count $args) 0) {{
                set command = $args[0]
              }}
              if (or (eq $command "deactivate") (eq $command "shell")) {{
                eval ({exe} $@args | slurp)
              }} else {{
                {exe} $@args
              }}
            }}

            fn rtx_hook {{||
              eval ({exe} hook-env{flags} -s elvish | slurp)
            }}
            edit:add-var _rtx_hook $rtx_hook~

            set edit:before-readline = [ $@edit:before-readline $rtx_hook~ ]
            rtx_hook
            "#});

        out
    }

    fn deactivate(&self) -> String {
        formatdoc! {r#"
            unset-env MISE_SHELL
            set edit:before-readline = [(each {{|f|
              if (not-eq $f $_rtx_hook) {{
                put $f
              }}
            }} $edit:before-readline)]
            del rtx~
        "#}
    }

    fn set_env(&self, k: &str, v: &str) -> String {
        let k = shell_escape::unix::escape(k.into());
        let v = shell_escape::unix::escape(v.into());
        let v = v.replace("\\n", "\n");
        format!("set-env {k} {v}\n")
    }

    fn prepend_env(&self, k: &str, v: &str) -> String {
        format!("set-env {k} {v}\n")
    }

    fn unset_env(&self, k: &str) -> String {
        format!("unset-env {k}\n", k = shell_escape::unix::escape(k.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_init() {
        let elvish = Elvish::default();
        let exe = Path::new("/some/dir/rtx");
        assert_snapshot!(elvish.activate(exe, " --status".into()));
    }
}
