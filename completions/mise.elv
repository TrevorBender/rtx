use flag
use path
use str

fn spaces {|n|
  repeat $n ' ' | str:join ''
}

fn cand {|text desc|
  edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
}

var subcmds~ = (constantly (
  cand activate 'Initializes mise in the current shell session'
  cand alias 'Manage aliases [aliases: a]'
  cand bin-paths 'List all the active runtime bin paths'
  cand cache  'Manage the mise cache'
  cand completion  'Generate shell completions'
  cand cfg  '[experimental] Manage config files'
  cand config  '[experimental] Manage config files'
  cand current  'Shows current active and installed runtime versions'
  cand deactivate  'Disable mise for current shell session'
  cand direnv  'Output direnv function to use mise inside direnv'
  cand doctor  'Check mise installation for possible problems'
  cand env  'Exports env vars to activate mise a single time'
  cand exec  'Execute a command with tool(s) set'
  cand implode  'Removes mise CLI and all related data'
  cand install  'Install a tool version'
  cand latest  'Gets the latest available version for a plugin'
  cand link  'Symlinks a tool version into mise'
  cand local 'Show local config'
  cand ls  'List installed and active tool versions'
  cand ls-remote  'List runtime versions available for install'
  cand outdated  'Shows outdated tool versions'
  cand plugins  'Manage plugins'
  cand prune  'Delete unused versions of tools'
  cand reshim  'rebuilds the shim farm'
  cand run  '[experimental] Run a tasks'
  cand self-update  'Updates mise itself'
  cand set  'Manage environment variables'
  cand settings  'Manage settings'
  cand shell  'Sets a tool version for the current session'
  cand sync  'Add tool versions from external tools to mise'
  cand tasks  '[experimental] Manage tasks'
  cand trust  'Marks a config file as trusted'
  cand uninstall  'Removes runtime versions'
  cand unset  'Remove environment variable(s) from the config file'
  cand upgrade  'Upgrades outdated tool versions'
  cand use  'Install tool version and add it to config'
  cand version  'Show mise version'
  cand watch  '[experimental] Run a tasks watching for changes'
  cand where  'Display the installation path for a runtime'
  cand which  'Shows the path that a bin name points to'
))

# TODO: this can be removed from final version
fn -debug {|@args &color=blue|
  echo (styled (echo ">>> " $@args) $color) >/dev/tty
}

fn dirs {|arg|
  edit:complete-filename $arg | each {|c|
    if (path:is-dir $c[stem]) {
      put $c
    }
  }
}

fn -files {|@args|
  var cur = $@args[-1]
  edit:complete-filename $cur
}

fn shells {|_|
  all [bash elvish fish nu xonsh zsh]
}

fn -plugins {|@_|
  mise plugins --core --user | from-lines
}

fn -tool-versions-arg {|cur|
  if (str:contains $cur '@') {
    var plug = $cur[..(str:last-index $cur '@')]
    mise ls-remote $plug | from-lines | each {|v| put $plug@$v}
  } else {
# otherwise complete plugins
    -plugins
  }
}

fn -tool-versions {|@args|
  var cur = $@args[-1]
  -tool-versions-arg $cur
}

var subs = [
  &activate={|@_|
    all [bash elvish fish nu xonsh zsh]
  }
  &current=$-plugins~
  &direnv={|@_|
    cand activate 'Output direnv function to use mise inside direnv'
  }
  &env=$-tool-versions~
  &install=$-tool-versions~
  &latest=&-tool-versions~
  &link=&-tool-versions~ # TODO: link <tool@version> <path>
  &local=$-tool-versions~
  &ls=$-plugins~
  &ls-remote=$-tool-versions~
  &outdated=$-tool-versions~
  &plugins={|@_|
    all [install link ls ls-remote uninstall update help] # TODO: completion for sub-commands
  }
  &prune=$-plugins~
  &settings={|@_|
    all [get ls set unset help] # TODO: completion for sub-commands
  }
  &shell=$-tool-versions~
  &sync={|@_|
    all [node python help]
  }
  &tasks={|@_|
    all [deps edit ls run help]
  }
  &trust=$-files~
  &uninstall=$-tool-versions~
  &upgrade=$-tool-versions~
  &use=$-tool-versions~
  &where=$-tool-versions~
]

var opts = [
# opts that apply to every command
  &-global=[
    [&short=C &long=cd &desc='Change directory before running command' &arg-required=$true &completer=$dirs~]
    [&short=q &long=quiet &desc='Supress non-error messages']
    [&short=v &long=verbose &desc='Show extra output (use -vv for even more)']
    [&short=y &long=yes &desc='Answer yes to all confirmation prompts']
  ]
  &activate=[
    [&long=shims &desc='Use shims instead of modifying PATH']
  ]
  &env=[
    [&short=J &long=json &desc='Output in JSON format']
    [&short=s &long=shell &desc='Shell type to generate environment variables for' &arg-required=$true &completer=$shells~]
  ]
  &local=[
    [&short=p &long=parent &desc='Recurse up to find a .tool-versions file']
    [&long=pin &desc='Save exact version']
    [&long=path &desc='Get the path of the config file']
  ]
  &ls=[
    [&short=c &long=current &desc='Only show tool versions currently specified in a .tool-versions/.mise.toml']
    [&short=g &long=global &desc='Only show tool versions currently specified in the global .tool-versions/.mise.toml']
    [&short=i &long=installed &desc='Only show tool versions that are installed']
    [&short=m &long=missing &desc='Display missing tool versions']
    [&long=no-header &desc="Don't display headers"]
  ]
  &plugins=[
    [&short=c &long=core &desc='The built-in plugins only']
    [&long=user &desc='List installed plugins - use with --core']
    [&short=u &long=urls &desc='Show the git url plugin']
  ]
  &prune=[
    [&short=n &long=dry-run &desc='Do not actually delete anything']
  ]
  &run=[
    [&short=n &long=dry-run &desc="Don't actually run the task(s), just print them in order of execution"]
    [&short=f &long=force &desc='Force the tasks to run even if outputs are up to date']
    [&short=p &long=prefix &desc='Print stdout/stderr by line, with prefix by task']
    [&short=j &long=jobs &desc='Number of tasks to run in parallel' &arg-required=$true]
    [&long=timings &desc='Show elapsed time after each task']
  ]
  &self-update=[
    [&long=no-plugins &desc='Disable auto-updating plugins']
  ]
  &set=[
    [&long=file &desc='The TOML file to update' &arg-required=$true]
    [&short=g &long=global &desc='Set the environment variable in the global config file']
  ]
  &shell=[
    [&short=j &long=jobs &desc='Number of jobs to run in parallel' &arg-required=$true]
    [&long=raw &desc='Directly pipe stdin/stdout/stderr from plugin']
    [&short=u &long=unset &desc='Removes a previously set version']
  ]
  &tasks=[
    [&long=no-header &desc='Do not print table header']
    [&long=hidden &desc='Show hidden tasks']
  ]
  &trust=[
    [&long=untrust &desc='No longer trust this config']
  ]
  &uninstall=[
    [&short=n &long=dry-run &desc='Do not actually delete anything']
  ]
  &unset=[
    [&short=f &long=file &desc='Specify a file to use instead' &arg-required=$true &completer=$edit:complete-filename~]
  ]
  &upgrade=[
    [&short=n &long=dry-run &desc='Just print what would be done']
    [&short=j &long=jobs &desc='Number of jobs to run in parallel' &arg-required=$true]
    [&short=i &long=interactive &desc='Display multiselect menu to choose which tools to upgrade']
    [&long=raw &desc='Directly pipe stdin/stdout/stderr from plugin']
  ]
  &use=[
    [&short=f &long=force &desc='Force reinstall if already installed']
    [&long=fuzzy &desc='Save fuzzy version to config file']
    [&short=g &long=global &desc='Use the global config file']
    [&short=j &long=jobs &desc='Number of jobs to run in parallel' &arg-required=$true]
    [&long=raw &desc='Directly pipe stdin/stdout/stderr from plugin']
    [&long=remove &desc='Remove the plugin from the config file' &arg-required=$true &completer=$-plugins~]
    [&long=pin &desc='Save exact version']
    [&short=p &long=path &desc='Path of the config file' &arg-required=$true &completer=$edit:complete-filename~]
  ]
  &watch=[
    [&short=t &long=task &desc='Tasks to run']
    [&short=g &long=glob &desc='Files to watch' &arg-required=$true &completer=$edit:complete-filename~]
  ]
  &which=[
    [&long=plugin &desc='Show the plugin name instead of the path']
    [&long=version &desc='Show the version instead of the path']
    [&short=t &long=tool &desc='Use a specific tool@version' &arg-required=$true &completer=$-tool-versions-arg~]
  ]
]

fn complete {|@args|
  var all-opts = [(keys $opts | each {|k| all $opts[$k]})]
  var vals = (flag:parse-getopt $args[1..] $all-opts | {
    var rs = [(all)]
    if (> (count $rs) 0) {
      put $rs[1]
    }
  })
  var opt-specs = $opts[-global]
  if (> (count $vals) 0) {
    var sub = $vals[0]
    if (has-key $opts $sub) {
      var sub-opts = $opts[$sub]
      set opt-specs = [$@opt-specs $@sub-opts]
    }
  }

  var arg-handlers = [
    {|_| nop }
    {|_| subcmds }
    {|arg|
      var sub = $vals[0]
      if (has-key $subs $sub) {
        $subs[$sub] $args
      }
    } ...
  ]
  edit:complete-getopt $args $opt-specs $arg-handlers
}

set edit:completion:arg-completer[mise] = $complete~
