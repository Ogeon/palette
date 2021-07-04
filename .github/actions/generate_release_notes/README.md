# Generate Release Notes

This Github action generates release notes from the commit history. It uses git tags to know where each release point is and requests commits between those versions. Each merge commit with a `#123` pull request number is collected as a log entry if the linked pull request isn't labelled with a label named "internal" (case insensitive).

## Github Action Usage

Example usage:

```yml
name: Prepare release
on:
  workflow_dispatch:
    inputs:
      version:
        description: The new version number
        required: true

jobs:
  compile_and_test:
    name: Update release notes
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2.3.4
        with:
          fetch-depth: 0 # We want the full history and all tags
      - name: Generate release notes
        uses: .github/actions/generate_release_notes
        with:
          version: ${{ github.event.inputs.version }}
          token: ${{ secrets.GITHUB_TOKEN }} # Optional, defaults to ${{ github.token }}
          file: CHANGELOG.md # Optional, this is the default value

# ...
```

## Local Usage

The action can also run as a local version, mainly for inspection, from an alternate entry point. Make sure you have node.js and npm installed. Then run

```shell
npm install
```

to install the dependencies. Then run the script by providing the repository owner and name:

```shell
node local/index.js MyUserName MyRepoName
```

Or add a version argument to set the upcoming version instead of "Unreleased":

```shell
node local/index.js MyUserName MyRepoName 1.2.3
```

The resulting output will be printed in markdown format to `stdout`.

## Developing

The action is implemented in Typescript. To rebuild the files in `local`, run

```shell
npm run build
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
