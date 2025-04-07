# serve

Quickly serve and receive files to/from anyone on your local network, using an easy web ui

!(serve ui example)[media/serve-ui-example.png]

## Features
- Easy and quick Uploads
- Zip Folder downloads
- Rename/Delete Files and Folders
- QR Code to connect
- Password Protection
- Sane Defaults
- Fast, thanks to rust backend
- Goodlooking UI

## Installation

Using cargo:
```sh
cargo install nlfmt-serve
serve --help
```

From source:
```sh
git clone https://github.com/nlfmt/serve
cd serve
# build frontend
cd app && pnpm i && pnpm build
cd .. && cargo install --path .
```

pre-built binaries coming soon...

## Todo

- [X] add download folder as zip or tar.gz
- [X] Uploads
  - [X] upload button in breadcrumbs, popup with drop zon
  - [X] drop zone anywhere in the window -> upload to current folder
  - [X] drop zone on folders => upload to that folder
- [ ] folder uploads (locked behind flag)
- [X] dont download symlinks, unless specified with flag
- [X] allow setting a password & username
  - `--auth, -a`, or `--auth-file` (maybe hashed?)
- [X] show error message when path was not found with a button "back to root"
- [ ] add logging to file
- [X] add progress bar popup
- [X] add support for multiple interfaces via `-i`, `--interface`
- [ ] support for custom headers
- [ ] tls support via `--tls-cert` and `--tls-key`
- [X] context menu
  - [ ] allow creating directories
  - [ ] file/folder info popup
  - [X] delete/rename ? (secured behind flag)
- [X] add right click action api & args
  - [X] rename
  - [X] delete
  - [ ] create folder
- [X] add qr code button on mobile
- [ ] lock zip generation behind flag, add tar/tar.gz