# project-init
A project initialization tool


```
$ project-init -h
project-init 0.1.0
nokaa <nokaa@cock.li>
A project initializer

USAGE:
    project-init [FLAGS] <PROJECT NAME> <LICENSE>

FLAGS:
    -c, --cargo        Create new project with cargo
    -d, --directory    Initialize project in the current directory
    -h, --help         Prints help information
    -V, --version      Prints version information

ARGS:
    <PROJECT NAME>    Sets the name of the new project
    <LICENSE>         Sets the license of the new project
```

LICENSE is searched for in `$XDG_CONFIG_HOME/license-add` or `~/.config/license-add`.

For a standard set of licenses, you can run `git clone https://github.com/nokaa/Licenses ~/.config/license-add`.
