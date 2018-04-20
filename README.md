# [Origin Sport](https://originsport.io/) Invitation Service

[![language](https://img.shields.io/badge/language-rust-blue.svg)](https://www.rust-lang.org)
[![license](https://img.shields.io/github/license/duriantang/ors_invitation.svg)](https://raw.githubusercontent.com/duriantang/ors_invitation/master/LICENSE)
[![Travis](https://img.shields.io/travis/duriantang/ors_invitation.svg)](https://travis-ci.org/duriantang/ors_invitation)
[![Codecov](https://img.shields.io/codecov/c/github/duriantang/ors_invitation.svg)](https://codecov.io/gh/duriantang/ors_invitation)
[![Libraries.io for GitHub](https://img.shields.io/librariesio/github/duriantang/ors_invitation.svg)](https://libraries.io/github/duriantang/ors_invitation)

Invitation Service used by [Origin Sport](https://originsport.io/) User Register.

Deploy via `Origin Sport Deploy System` .

## Build

Clone this repo and then cd to it's directory, run `make build` .

binary file will copy to `build` directory.

## Configuration

Edit `deploy/.env.j2` and rename to `build/.env` .

## Running

cd to `build` and then run `./invitation` .
