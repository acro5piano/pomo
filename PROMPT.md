Create simple Pomodoro Timer in Rust.

# Stack

- Rust
- Clap

# How it works

When user inputs,

```
$ pomo
```

It will display current time like this:

```
25:00 🍅
```

The value will changes as time passes like this:

```
24:59 🍅
```

When reaching 0,

```
00:00 🍅
```

Rest phase will start:

```
05:00 🌴
```

# Other TODO

- User can pause timer by just clicking Ctrl+C:
- User can restart just writing `pomo`
- All data will be saved in `~/.pomo.json`
- Linux notification should fire when time reached to 0.
