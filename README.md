# todo

A minimal command-line todo list manager written in Rust. Design stolen from <https://github.com/sjl/t>
with a few extra features (undo, and timestamps).

## Usage

Run with no arguments to list pending todos:

```
$ todo
   1 ☐ Buy groceries (2h ago)
   2 ☐ Write tests (30m ago)
   3 ☐ Review PR (5m ago)
```

Pass a description to add a new todo:

```
$ todo Fix the login bug
$ todo Deploy staging server
```

Mark a todo as finished by its number:

```
$ todo -f 2
```

Edit a todo's description:

```
$ todo -e 1 Buy groceries and cook dinner
```

Undo the last operation:

```
$ todo -u
```

By default, todos are stored in `.todos` in the current directory.
Use `--list` to specify a different file. Use aliases to manage multiple todo lists:

```shell
alias t='todo --list ~/.todos'
alias gt='todo --list .git/.todos'
```
