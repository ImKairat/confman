# confman

**confman** is a lightweight, command-line configuration manager for Linux systems. It helps automate the setup of your environment by managing dotfiles, installing packages, and applying system preferences — all from a single, portable Git repository.

confman uses a simple, declarative configuration file called `configuration.cm`, written in a Lua-like syntax, making it easy to customize and extend.

## Features

- Manage and apply dotfiles (e.g. `.bashrc`, `.vimrc`, `.zshrc`)
- Install packages automatically (e.g. with `apt`, `pacman`, etc.)
- Run custom post-install scripts
- Easily portable — just clone and run on any machine
- Configuration via `configuration.cm` using familiar Lua-like syntax
