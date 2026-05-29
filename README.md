# 🌿 fern**bedienung**

a cli to manage git remotes 

## 🛠️ install 

### cargo
```sh
$ cargo install --git https://github.com/drainpixie/fern.git
```

### nix
#### declarative
```nix
environment.systemPackages = [
    inputs.fern.packages.<arch>.fern;
];
```

#### imperative
```sh
$ nix profile install github:drainpixie/fern
```


## 🖥️ usage 

### basics
```sh
$ fern help
Usage: fern [COMMAND]

Commands:
  init     Initialise a repository with Git and Fern
  status   Show the current status of all tracked remotes
  remotes  Lists all tracked remotes
  add      Add a remote to be tracked
  remove   Remove a remote from being tracked
  rename   Rename a tracked remote
  push     Push to one or more remotes
  pull     Pull from one or more remotes
  fetch    Fetch from one or more remotes
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### new repository
```sh
# Initialise a repository
$ fern init

# Add your remotes
$ fern add github git@github.com:username/repo.git
$ fern add codeberg git@codeberg.org:username/repo.git

# Do some Git operations
$ echo "console.log('Hello, World');" > fern.mjs 
$ git add fern.mjs
$ git commit -m 'feat: we do some ferning'

# Push with Fern 
$ fern push
```


