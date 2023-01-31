# git-find-uncommited
Find git repositories with uncommited changes

git-find-uncommited recurses the file tree at the path given as argument
or the current working directory if invoked without a path.
It executes `git-status` on directories containing '.git'.
If changes to the working directory are found, they are displayed.

## Usage
```
$ cd /usr/local/src && git-find-uncommited
```
# or
```
$ git-find-uncommited /usr/local/src
```

## Build
```
$ git clone https://github.com/bbusse/git-find-uncommited
$ cd git-find-uncommited
$ cargo build
```
## Install
```
$ cargo install --path .
```
## Dependencies
- git
