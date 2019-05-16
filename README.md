# npm_pub
Rust script to handle npm publish

This CLI Tool runs:

1. `npm run build` - runs your project's build script for production
    * this will need to be in your package.json "scripts"
    * read more here: [npm docs](https://docs.npmjs.com/misc/scripts)
2. `npm version $ver` - based on argument passed 
    * acceptable versions `major`, `minor`, `patch`
    * read more here [Semantic Versioning](https://semver.org/)
3. `git push` - pushes to git repo set in `remote`
    * `git rev-parse --abbrev-ref HEAD` - to get current branch
    * `git push origin $branch`
4. `npm publish` - publishes to [npmjs.com](https://npmjs.com) via details in `package.json`

## Installing

* Download [binary executable](https://github.com/Sparkmasterflex/npm_pub/releases/tag/0.1.0) 
* move it to `/usr/local/bin/`

or

* [Install rustlang](https://www.rust-lang.org/tools/install)
    - or with homebrew: `brew install rustup`
* clone repo
* `cd` to local directory and run `cargo build --release`
* `cp target/release/npm_pub /usr/local/bin/`

_This is all assuming that `/usr/local/bin/` is in your `$PATH` variable_

## Running Tool

### Prerequisites

* Project must
   *  be a valid npm project with
      * package.json
      * build script (webpack, etc)
   * be an initiated Git repo
      * with a remote repo (Github, etc)
* User must be logged in to npm via terminal
  * `npm login`

In your command line (terminal, iterm, etc) run:
`$ npm_pub [major, minor, patch]`
