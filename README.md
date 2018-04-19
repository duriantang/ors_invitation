# Origin Sport Invitation Service

![language](https://img.shields.io/badge/language-rust-blue.svg)
![license](https://img.shields.io/github/license/duriantang/ors_invitation.svg)
![Travis](https://img.shields.io/travis/duriantang/ors_invitation.svg)
![Libraries.io for GitHub](https://img.shields.io/librariesio/github/duriantang/ors_invitation.svg)

Invitation Service used by Origin Sport User Register.

Deploy via `Origin Sport Deploy System` .

## Build

Clone this repo and then cd to it's directory, run `make build` .

binary file will copy to `build` directory.

## Configuration

Edit `deploy/.env.j2` and rename to `build/.env` .

## Running

cd to `build` and then run `./invitation` .
