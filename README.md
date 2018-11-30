# [Origin Sport] Invitation Service

[Origin Sport]: https://originsport.io/ "Origin Sport"

[language]: https://img.shields.io/badge/language-rust%201.30+-blue.svg
[language website]: https://www.rust-lang.org/ "The Rust Programming Language"

[license]: https://img.shields.io/github/license/duriantang/ors_invitation.svg
[MIT]: https://raw.githubusercontent.com/duriantang/ors_invitation/master/LICENSE "MIT"

[ci]: https://img.shields.io/travis/duriantang/ors_invitation.svg
[travis-ci]: https://travis-ci.org/duriantang/ors_invitation "Travis CI"

[cov]: https://img.shields.io/codecov/c/github/duriantang/ors_invitation.svg
[codecov]: https://codecov.io/gh/duriantang/ors_invitation "Codecov.io"

[libs]: https://img.shields.io/librariesio/github/duriantang/ors_invitation.svg
[libraries.io]: https://libraries.io/github/duriantang/ors_invitation "Libraries.io for GitHub"

[![language]][language website]
[![license]][MIT]
[![ci]][travis-ci]
[![cov]][codecov]
[![libs]][libraries.io]

Invitation Service used by [Origin Sport] User Register.

Deploy via `Origin Sport Deploy System` .

## Build

Clone this repo and then cd to it's directory, run `make build` .

binary file will copy to `build` directory.

## Configuration

Edit `deploy/.env.j2` and rename to `build/.env` .

## Running

cd to `build` and then run `./invitation` .
