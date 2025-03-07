use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::time::Duration;

use indexmap::IndexMap;
use log::LevelFilter;

use crate::config::AliasMap;
use crate::env;
use crate::env::{
    RTX_ASDF_COMPAT, RTX_DISABLE_DEFAULT_SHORTHANDS, RTX_JOBS, RTX_LOG_LEVEL, RTX_SHIMS_DIR,
    RTX_SHORTHANDS_FILE, RTX_VERBOSE,
};
use crate::plugins::PluginName;

#[derive(Debug, Clone)]
pub struct Settings {
    pub experimental: bool,
    pub missing_runtime_behavior: MissingRuntimeBehavior,
    pub always_keep_download: bool,
    pub legacy_version_file: bool,
    pub plugin_autoupdate_last_check_duration: Duration,
    pub aliases: IndexMap<PluginName, IndexMap<String, String>>,
    pub verbose: bool,
    pub asdf_compat: bool,
    pub jobs: usize,
    pub shorthands_file: Option<PathBuf>,
    pub disable_default_shorthands: bool,
    pub log_level: LevelFilter,
    pub shims_dir: Option<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            experimental: false,
            missing_runtime_behavior: MissingRuntimeBehavior::Prompt,
            always_keep_download: false,
            legacy_version_file: true,
            plugin_autoupdate_last_check_duration: Duration::from_secs(60 * 60 * 24 * 7),
            aliases: IndexMap::new(),
            verbose: *RTX_VERBOSE || !console::user_attended_stderr(),
            asdf_compat: *RTX_ASDF_COMPAT,
            jobs: *RTX_JOBS,
            shorthands_file: RTX_SHORTHANDS_FILE.clone(),
            disable_default_shorthands: *RTX_DISABLE_DEFAULT_SHORTHANDS,
            log_level: *RTX_LOG_LEVEL,
            shims_dir: RTX_SHIMS_DIR.clone(),
        }
    }
}

impl Settings {
    pub fn to_index_map(&self) -> IndexMap<String, String> {
        let mut map = IndexMap::new();
        map.insert("experimental".to_string(), self.experimental.to_string());
        map.insert(
            "missing_runtime_behavior".to_string(),
            self.missing_runtime_behavior.to_string(),
        );
        map.insert(
            "always_keep_download".to_string(),
            self.always_keep_download.to_string(),
        );
        map.insert(
            "legacy_version_file".to_string(),
            self.legacy_version_file.to_string(),
        );
        map.insert(
            "plugin_autoupdate_last_check_duration".to_string(),
            (self.plugin_autoupdate_last_check_duration.as_secs() / 60).to_string(),
        );
        map.insert("verbose".into(), self.verbose.to_string());
        map.insert("asdf_compat".into(), self.asdf_compat.to_string());
        map.insert("jobs".into(), self.jobs.to_string());
        if let Some(shorthands_file) = &self.shorthands_file {
            map.insert(
                "shorthands_file".into(),
                shorthands_file.to_string_lossy().to_string(),
            );
        }
        map.insert(
            "disable_default_shorthands".into(),
            self.disable_default_shorthands.to_string(),
        );
        map.insert("log_level".into(), self.log_level.to_string());
        if let Some(shims) = &self.shims_dir {
            map.insert("shims_dir".into(), shims.to_string_lossy().to_string());
        }
        map
    }
}

#[derive(Debug, Default, Clone)]
pub struct SettingsBuilder {
    pub experimental: Option<bool>,
    pub missing_runtime_behavior: Option<MissingRuntimeBehavior>,
    pub always_keep_download: Option<bool>,
    pub legacy_version_file: Option<bool>,
    pub plugin_autoupdate_last_check_duration: Option<Duration>,
    pub aliases: Option<AliasMap>,
    pub verbose: Option<bool>,
    pub asdf_compat: Option<bool>,
    pub jobs: Option<usize>,
    pub shorthands_file: Option<PathBuf>,
    pub disable_default_shorthands: Option<bool>,
    pub log_level: Option<LevelFilter>,
    pub shims_dir: Option<PathBuf>,
}

impl SettingsBuilder {
    // pub fn new(missing_runtime_behavior: Option<MissingRuntimeBehavior>) -> Self {
    //     Self {
    //         missing_runtime_behavior: missing_runtime_behavior,
    //     }
    // }

    pub fn _merge(&mut self, other: Self) -> &mut Self {
        if other.experimental.is_some() {
            self.experimental = other.experimental;
        }
        if other.missing_runtime_behavior.is_some() {
            self.missing_runtime_behavior = other.missing_runtime_behavior;
        }
        if other.always_keep_download.is_some() {
            self.always_keep_download = other.always_keep_download;
        }
        if other.legacy_version_file.is_some() {
            self.legacy_version_file = other.legacy_version_file;
        }
        if other.plugin_autoupdate_last_check_duration.is_some() {
            self.plugin_autoupdate_last_check_duration =
                other.plugin_autoupdate_last_check_duration;
        }
        if other.verbose.is_some() {
            self.verbose = other.verbose;
        }
        if other.asdf_compat.is_some() {
            self.asdf_compat = other.asdf_compat;
        }
        if other.jobs.is_some() {
            self.jobs = other.jobs;
        }
        if other.shorthands_file.is_some() {
            self.shorthands_file = other.shorthands_file;
        }
        if other.disable_default_shorthands.is_some() {
            self.disable_default_shorthands = other.disable_default_shorthands;
        }
        if other.log_level.is_some() {
            self.log_level = other.log_level;
        }
        if other.shims_dir.is_some() {
            self.shims_dir = other.shims_dir;
        }
        if other.aliases.is_some() {
            self.aliases = other.aliases;
        }
        self
    }

    pub fn build(&self) -> Settings {
        let mut settings = Settings::default();
        settings.experimental = self.experimental.unwrap_or(settings.experimental);
        settings.missing_runtime_behavior = match env::RTX_MISSING_RUNTIME_BEHAVIOR
            .to_owned()
            .unwrap_or_default()
            .as_ref()
        {
            "autoinstall" => MissingRuntimeBehavior::AutoInstall,
            "warn" => MissingRuntimeBehavior::Warn,
            "ignore" => MissingRuntimeBehavior::Ignore,
            "prompt" => MissingRuntimeBehavior::Prompt,
            _ => self
                .missing_runtime_behavior
                .clone()
                .unwrap_or(settings.missing_runtime_behavior),
        };
        settings.always_keep_download = self
            .always_keep_download
            .unwrap_or(settings.always_keep_download);
        settings.legacy_version_file = self
            .legacy_version_file
            .unwrap_or(settings.legacy_version_file);
        settings.plugin_autoupdate_last_check_duration = self
            .plugin_autoupdate_last_check_duration
            .unwrap_or(settings.plugin_autoupdate_last_check_duration);
        settings.verbose = self.verbose.unwrap_or(settings.verbose);
        settings.asdf_compat = self.asdf_compat.unwrap_or(settings.asdf_compat);
        settings.jobs = self.jobs.unwrap_or(settings.jobs);
        settings.shorthands_file = self.shorthands_file.clone().or(settings.shorthands_file);
        settings.disable_default_shorthands = self
            .disable_default_shorthands
            .unwrap_or(settings.disable_default_shorthands);
        settings.log_level = self.log_level.unwrap_or(settings.log_level);
        settings.shims_dir = self.shims_dir.clone().or(settings.shims_dir);
        settings.aliases = self.aliases.clone().unwrap_or(settings.aliases);

        settings
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MissingRuntimeBehavior {
    AutoInstall,
    Prompt,
    Warn,
    Ignore,
}

impl Display for MissingRuntimeBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MissingRuntimeBehavior::AutoInstall => write!(f, "autoinstall"),
            MissingRuntimeBehavior::Prompt => write!(f, "prompt"),
            MissingRuntimeBehavior::Warn => write!(f, "warn"),
            MissingRuntimeBehavior::Ignore => write!(f, "ignore"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::settings::MissingRuntimeBehavior::{AutoInstall, Ignore, Prompt, Warn};

    #[test]
    fn test_settings_merge() {
        let mut s1 = SettingsBuilder::default();
        let s2 = SettingsBuilder {
            missing_runtime_behavior: Some(AutoInstall),
            ..SettingsBuilder::default()
        };
        s1._merge(s2);

        assert_eq!(s1.missing_runtime_behavior, Some(AutoInstall));
    }

    #[test]
    fn test_missing_runtime_behavior_display() {
        assert_eq!(AutoInstall.to_string(), "autoinstall");
        assert_eq!(Prompt.to_string(), "prompt");
        assert_eq!(Warn.to_string(), "warn");
        assert_eq!(Ignore.to_string(), "ignore");
    }
}
