---
title: A Local-First Bash Toolkit for History, Prompts, and Directory Jumps
date: 2026-09-07
status: published
---

When a developer sees a small annoyance, there is one obvious response: rewrite it in Rust! I went the other way: I took the main features I use from [fzf](https://github.com/junegunn/fzf), [atuin](https://atuin.sh/), [oh-my-posh](https://ohmyposh.dev/), and [zoxide](https://github.com/ajeetdsouza/zoxide) and rewrote them in Bash, with SQLite doing the remembering. The result is a native prompt, searchable <kbd>Ctrl-R</kbd> history, smart directory jumps, and a time tracker &mdash; all local, with no account or daemon, though you can sync the history over "ssh" to a remote computer.

The "tt" script and "battape.sh" hooks live in my [dotfiles repository](https://github.com/jhthorsen/dotfiles). The goal is a capable, standalone alternative for people who prefer local data, a small dependency set, and scripts they can inspect and adapt.

Want to try it first? Skip to [installation](#what-you-need) or [configuration](#configuration).

## Prompt: Replacing Oh My Posh

The prompt shows a shortened working directory, Git branch, dirty repository state, unpushed or unpulled work, how long the previous command took, and whether it succeeded. Paths longer than three components are shortened from the left.

For example, after a successful command in a clean checkout, the prompt might look like this:

```text
superwoman ~/…/git/dotfiles (main) ✓
```

A dirty checkout is red, a branch that is ahead of or behind its upstream is magenta, and a failed command gets a red cross. Colour is omitted from these examples:

```text
superwoman ~/src/customer-api (fix-login) 12s ✗
```

The host name is included when working remotely, making it harder to mistake a production shell for a local one.

## Command history: Replacing fzf and Atuin

Press `Ctrl-R` to open battape's history picker. Start typing to filter commands; the search is case-insensitive and keeps words in order. Use Up/Down, `Ctrl-P`/`Ctrl-N`, or `Ctrl-K`/`Ctrl-J` to choose an entry, then press Enter to put it on the command line. Nothing runs until you press Enter a second time at the normal prompt.

Here is a representative search for `git log`:

```text
? git log
> git log --oneline -5
  git status && git log --stat
  git log -- templates/blog/rewrite-it-in-bash.md
```

The selected command is the newest matching command from the current terminal when possible. Duplicate commands are hidden, failed commands are visually marked, and older entries are dimmed. The picker also accepts UTF-8 input and pasted text.

Unlike a conventional shell history file, battape records the command, its start and end time, working directory, hostname, terminal, and exit status. The data stays local in a SQLite file. There is no account, daemon, or network service.

## Smart directory changes: Replacing zoxide

Every recorded command makes its working directory a candidate for `cd`. Regular Bash path handling still wins, so `cd ..`, `cd /tmp`, `CDPATH`, and ordinary relative paths work just as before. If Bash cannot find a one-word destination, battape searches directories from command history and picks the most frequently and recently used match.

```text
$ cd dotfiles
$ pwd
/home/superwoman/git/dotfiles
```

Multi-word terms match path components in order, so `cd customer api` can find a directory such as `~/src/customer-api`. Directories that no longer exist are skipped.

## Time tracking with `tt`

`tt` records time entries in the same database as command history. An entry has an ID, times, account, description, tags, host, and user. Start one with an account, description, and optional tags:

```text
$ tt start --account Acme --description 'Fix login bug' --tags 'web urgent'

  ID          Date        Account  Start  Stop  Duration  Tags
  ——————————————————————————————————————————————————————————————————
  QkYy8uYJyG  2026-09-06  Acme     09:15  -     0h 0m     urgent,web

```

Stop the active entry when you are done:

```text
$ tt stop

  ID          Date        Account  Start  Stop   Duration  Tags
  ——————————————————————————————————————————————————————————————————
  QkYy8uYJyG  2026-09-06  Acme     09:15  10:05  0h 50m   urgent,web

```

IDs are random, so yours will differ. `tt status` shows entries from today, while `tt report` can select a date range, account, tag, or entry ID. It can also group the result by day or month:

```text
$ tt report --start -1m --group day

  Period      Accounts  Duration  Tags
  —————————————————————————————————————————————————
  2026-09-06  Acme      1h 30m    urgent,web,review

  Period:   2026-08-07 -> 2026-09-06
  Account:  any
  Tag:      any
  Events:   2
  Total:    1h 30m

```

Run `tt commands` to see a summary of commands executed during matching entries, or add `--full` for every invocation. This is handy when filling in a timesheet or reconstructing what happened during an incident.

```text
$ tt commands --full

  Date        Start  Command                 Duration  Exit
  —————————————————————————————————————————————————————————
  2026-09-06  09:19  git status -sb          1s        0
  2026-09-06  09:26  nvim app/login.js       18m 42s   0
  2026-09-06  09:47  npm test -- login       2m 11s    0

```

`tt` understands epoch seconds, `HH:MM`, ISO timestamps, and offsets such as `-1d` or `-1m`. Use `tt --help` for the full command reference.

## Syncing another machine

If your machines can reach each other over SSH, you can merge command history and time entries without a third-party service:

```text
$ tt sync ssh.example.com
$ tt sync --pull ssh.example.com
```

The first command pushes this machine's database to the remote host; the second pulls from it. Sync uses a SQLite backup and row IDs. If the same ID has conflicting data from another host, it reports the conflict instead of overwriting it silently.

## What you need

The tools require Bash 4.4 or newer and `sqlite3`. Linux distributions generally package both. On macOS, the system Bash is too old, so install a current Bash and SQLite with Homebrew:

```bash
brew install bash sqlite;
```

Homebrew does not replace macOS's `/bin/bash`; start the Homebrew Bash (or make it your preferred shell) before using the setup.

Download `tt` and `battape.sh` directly. This puts the executable in the usual user-local bin directory and the Bash module under `~/.config`:

```bash
mkdir -p "$HOME/.local/bin" "$HOME/.config/battape";
curl -fsSLo "$HOME/.local/bin/tt" https://raw.githubusercontent.com/jhthorsen/dotfiles/main/bin/tt;
curl -fsSLo "$HOME/.config/battape/battape.sh" https://raw.githubusercontent.com/jhthorsen/dotfiles/main/config/bash/battape.sh;
chmod +x "$HOME/.local/bin/tt";
```

Add the following to `~/.bashrc`, then open a new Bash session. The tools create their shared SQLite database automatically at `${XDG_DATA_HOME:-$HOME/.local/share}/battape/battape.sqlite`.

```bash
export PATH="$HOME/.local/bin:$PATH";

source "$HOME/.config/battape/battape.sh";
battape_prompt_enable;

bind -m emacs -x '"\C-r":battape_render_history_ui';
bind -m vi-insert -x '"\C-r":battape_render_history_ui';
bind -m vi-command -x '"\C-r":battape_render_history_ui';

cd() { battape_cd "$@"; }
```

`battape_prompt_enable` creates the database, enables command recording, and installs the prompt. If you only want persistent command history, use `battape_recorder_enable` instead. Remove the bindings, the `cd` function, or the prompt-enable line for integrations you do not want.

Battape uses Bash's DEBUG trap to measure commands. It intentionally refuses to enable recording when another DEBUG trap is already installed, rather than overwriting it.

## Import Existing History Entries

After installing and sourcing `battape.sh`, import commands from an existing Bash history file with this script. It skips Bash's optional timestamp lines and escapes single quotes before inserting commands into SQLite:

```bash
#!/usr/bin/env bash

history_file=${HISTFILE:-"$HOME/.bash_history"}

while IFS= read -r cmd || [[ -n $cmd ]]; do
  [[ -z $cmd || $cmd =~ ^#[0-9]+$ ]] && continue

  cmd=${cmd//\'/\'\'}
  sqlite3 "$BATTAPE_DB" "
    insert into history (id, start, end, hostname, tty, pwd, command, exit_status)
    values (
      substr(hex(randomblob(10)), 1, 10),
      0, 0,
      '$(hostname)',
      '/dev/tty',
      '$HOME',
      '$cmd',
      0
    );
  "
done < "$history_file"
```

Imported entries do not have their original working directory, terminal, duration, or exit status, so the script uses your home directory and zero values for those fields.

## Import Directories for Quick Jump

Battape learns directories from recorded commands. To seed its directory index, run the following from a directory tree you want to include. This records directories up to two levels below the current directory:

```bash
#!/usr/bin/env bash

find . -type d -maxdepth 2 -mindepth 1 -print0 |
  while IFS= read -r -d '' dir; do
    (
      cd -- "$dir" || exit
      eval "$PROMPT_COMMAND"
    )
  done
```

The subshell keeps the script in its original directory, and `-print0` handles directory names containing spaces or other special characters.

## Configuration

You can override many options before sourcing `battape.sh`. The defaults work without any configuration; this is one possible adjustment for an ASCII-only prompt and a smaller history picker:

```bash
export BATTAPE_DATA_HOME="$HOME/.local/share/battape"
export BATTAPE_HISTORY_MAX_ROWS=8
export BATTAPE_PROMPT_HOST=never
export BATTAPE_PROMPT_PATH_DEPTH=2
export BATTAPE_PROMPT_SUCCESS='ok'
export BATTAPE_PROMPT_FAILURE='!'
export BATTAPE_PROMPT_SEPARATOR='>'
```

The following is the complete set of optional environment variables. Set them before sourcing `battape.sh`; `tt` reads the database and report settings when it starts.

| Variable | Default | Effect |
| --- | --- | --- |
| `XDG_DATA_HOME` | `$HOME/.local/share` | Standard XDG base directory used to form the default data path. |
| `BATTAPE_DATA_HOME` | `${XDG_DATA_HOME:-$HOME/.local/share}/battape` | Directory that contains the database. |
| `BATTAPE_DB` | `$BATTAPE_DATA_HOME/battape.sqlite` | Path to the shared SQLite database. |
| `BATTAPE_HISTORY_MAX_ROWS` | `12` | Maximum number of matches in the history picker. |
| `BATTAPE_HISTORY_COLOR_FAIL` | red | Colour for failed commands in the picker. |
| `BATTAPE_HISTORY_COLOR_SUCCESS` | green | Colour for successful commands in the picker. |
| `BATTAPE_HISTORY_COLOR_RECENT` | bright white | Colour for recent entries. |
| `BATTAPE_HISTORY_COLOR_OLD` | white | Colour for older entries. |
| `BATTAPE_HISTORY_COLOR_OLDEST` | bright black | Colour for the oldest entries. |
| `BATTAPE_HISTORY_COLOR_SELECTED` | bold bright white | Colour for the selected entry. |
| `BATTAPE_HISTORY_COLOR_RESET` | terminal reset | Escape sequence that restores the picker colour. |
| `BATTAPE_PROMPT_COLOR_BG` | dark grey | Prompt background colour. |
| `BATTAPE_PROMPT_COLOR_BG_RESET` | default background | Escape sequence that restores the background. |
| `BATTAPE_PROMPT_COLOR_FG` | cyan | Normal prompt foreground colour. |
| `BATTAPE_PROMPT_COLOR_RED` | red | Failure and dirty-Git colour. |
| `BATTAPE_PROMPT_COLOR_MAGENTA` | magenta | Ahead/behind Git colour. |
| `BATTAPE_PROMPT_COLOR_SEPARATOR` | dark grey | Foreground colour for the prompt separator. |
| `BATTAPE_PROMPT_COLOR_RESET` | terminal reset | Escape sequence that restores prompt colours. |
| `BATTAPE_PROMPT_HOST` | `auto` | Show the short hostname: `auto` only over SSH, `always`, or `never`. |
| `BATTAPE_PROMPT_PATH_DEPTH` | `3` | Number of trailing path components to display; invalid values fall back to `3`. |
| `BATTAPE_PROMPT_GIT` | `1` | Set to `0` to skip Git status checks. |
| `BATTAPE_PROMPT_SUCCESS` | `✓` | Marker after a successful command. |
| `BATTAPE_PROMPT_FAILURE` | `✗` | Marker after a failed command. |
| `BATTAPE_PROMPT_SEPARATOR` | `` | Character at the right edge of the prompt. |
| `BATTAPE_CD_OSC7` | `auto` | Emit the OSC 7 working-directory escape sequence: `auto`, `always`, `never`, `0`, or `no`. In `auto`, it is emitted only to a non-dumb terminal. |
| `BATTAPE_CD_RECENCY_DAYS` | `14` | Positive number of days used for the directory-ranking recency boost; invalid values fall back to `14`. |
| `BATTAPE_DEFAULT_ACCOUNT` | unset | Account to use for `tt start` when `--account` is omitted. |
| `BATTAPE_REPORT_LIMIT` | `1000` | Default maximum rows for `tt report` and `tt commands`. |

The colour variables accept terminal escape sequences. Prompt colour values must include Bash's non-printing markers, for example `export BATTAPE_PROMPT_COLOR_FG='\[\e[36m\]'`; history-picker values do not use those markers, for example `export BATTAPE_HISTORY_COLOR_SUCCESS=$'\e[36m'`.

## A small trade-off

This setup still uses standard Unix tools—especially SQLite, `awk`, `sed`, and `tput`—so it is not “pure Bash” in a strict sense. It targets SQLite 3.46.x, which is available in Debian. In return for fewer dependencies and a single local database, you take responsibility for adapting the scripts to your own preferences.

### Prompt render time

It is possible to measure the cost of drawing the prompt, and the result is a useful check on the joke: Bash is not automatically faster. On this machine, I timed 100 warm renders in this checkout with Bash 5.3. Battape averaged 41 ms per prompt, including its normal Git checks and SQLite history write. My existing Oh My Posh 29.33.0 configuration averaged 18 ms per prompt.

Battape does more than render its prompt at that point. It stores the previous command's start and end time, directory, hostname, terminal, and exit status in SQLite; calculates its duration; and runs the Git checks. Those stored records power the <kbd>Ctrl-R</kbd> history picker and smart `cd`, and are available to `tt` for time tracking. That makes the extra work part of the toolkit, rather than prompt decoration alone.

That is not a universal comparison: prompts, repositories, disks, and caches differ, and the two configurations do not have identical features. It does mean I would not present battape as a performance replacement for Oh My Posh. Its appeal is that the implementation and data are local, small, and easy to adapt; prompt latency is something to measure with your own configuration.

## Conclusion

Battape is not a universal replacement for the tools it borrows ideas from, nor is it the fastest prompt on my machine. It is a small, local toolkit whose behaviour and data are easy to inspect, install, and change. If that sounds useful, [install it](#what-you-need), try the parts you want, and make it your own.
