# git-find-uncomited
Find git repositories with uncommited changes

git-find-uncommited recurses the file tree at the current working directory
and searches for directories named '.git'. It then runs `git-status` on the
directory. If changes to the working directory are found, they are displayed.

## Usage
```
$ cd /usr/local/src && git-find-uncommited
```
## Dependencies
- git
