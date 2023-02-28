use std::path::Path;

use indoc::formatdoc;
use itertools::Itertools;

use crate::shell::{is_dir_in_path, Shell};

#[derive(Default)]
pub struct Elvish {}

impl Shell for Elvish {
    fn activate(&self, exe: &Path, status: bool) -> String {
        let dir = exe.parent().unwrap();
        let exe = exe.display();
        let status = if status { " --status" } else { "" };
        let mut out = String::new();
        if !is_dir_in_path(dir) {
            out.push_str(&format!(
                "set paths = [ {dir} $@paths ]\n",
                dir = dir.display()
            ));
        }
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

            fn rtx_hook {{
              eval ({exe} hook-env{status} -s elvish | slurp)
            }}

            set edit:before-readline = [ $@edit:before-readline $rtx_hook~ ]
            rtx_hook
            "#});

        out
    }

    fn deactivate(&self, path: String) -> String {
        let path = path.split(':').join(" ");
        format!("set paths = [ {path} ]\n")
    }

    fn set_env(&self, k: &str, v: &str) -> String {
        let k = shell_escape::unix::escape(k.into());
        let v = shell_escape::unix::escape(v.into());
        let v = v.replace("\\n", "\n");
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
        insta::assert_snapshot!(elvish.activate(exe, true))
    }
}
