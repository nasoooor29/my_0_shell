# Remaining Work

## Commands Not Implemented

- `echo`
- `cat`
- `cp`
- `rm`: needs `-r`
- `mv`
- `mkdir`
- `ls`: needs `-F`

## Other Things

- Prompt should be at least `$ `.
- Unknown command message should exactly be: `Command '<name>' not found`.
- Remove external binary usage, especially `stty` in terminal handling.
- Implement terminal raw/non-canonical mode directly through OS APIs if arrow-key history is kept.
- Ensure all commands are implemented from scratch without spawning external binaries.
- Optionally register `exit` as a command for consistency.
- Remaining bonus features: auto-completion, command chaining with `;`, pipes with `|`, redirection with `>` and `<`, general environment variable expansion, and current-directory prompt.
