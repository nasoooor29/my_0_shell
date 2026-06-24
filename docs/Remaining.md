# Remaining Work

## Commands Not Implemented

- [ ] `echo`
- [ ] `cat`
- [ ] `cp`
- [ ] `rm`: needs `-r`
- [ ] `mv`
- [ ] `mkdir`
- [x] `ls`: supports `-F`

## Other Things

- [x] Prompt should be at least `$ `.
- [x] Unknown command message should exactly be: `Command '<name>' not found`.

- [x] Remove external binary usage, especially `stty` in terminal handling.
- [x] Implement terminal raw/non-canonical mode directly through OS APIs if arrow-key history is kept.

- [x] Optionally register `exit` as a command for consistency.
