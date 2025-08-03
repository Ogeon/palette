# Generate Release Notes

This Github action updates the version number of a crate and its dependencies to specific version numbers. This is useful for incrementing the version when preparing a release. Version numbers that are updated:

* In Cargo.toml:
  * The crate version
  * Version numbers in the documentation link
  * Any dependency that is set through the `dependencies` input.
* In compilation targets (i.e. `lib.rs`, etc.):
  * Version numbers in the `#![doc(html_root_url = "...")]` attribute.
* In `README.md`:
  * Any mention of `my_crate = "1.2.3"`.
  * Any mention of `version = "1.2.3"`.

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
    name: Update crate versions
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2.3.4
      - name: Increment version for my_crate
        uses: ./.github/actions/increment_version_number
        with:
          version: ${{ github.event.inputs.version }}
          crate: my_crate # Optional, for when the root directory isn't the crate directory.
          dependencies: '{"companion_crate": "${{ github.event.inputs.version }}"}'
            # ^~~ Optional, for when dependencies are updated at the same time.

# ...
```

## Local Usage

The action can also run as a local version, mainly for inspection, from an alternate entry point. Make sure you have node.js and npm installed.

***Note!*** *The files will be changed. This is not a dry run!*

```shell
npm install
```

will install the dependencies. Then run the script by providing the repository owner and name:

```shell
node local/index.js 1.2.3 path/to/crate
```

Or add a version argument to set the upcoming version instead of "Unreleased":

```shell
node local/index.js 1.2.3 path/to/crate '{"companion_crate": "4.5.6"}'
```

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
