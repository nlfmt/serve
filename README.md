# serve

Quickly serve files to anyone on your local network, using a easy web ui

# Todo

- [ ] add download folder as zip or tar.gz
- [ ] folder uploads (locked behind flag)
  - [X] upload button in breadcrumbs, popup with drop zon
  - [X] drop zone anywhere in the window -> upload to current folder
  - [X] drop zone on folders => upload to that folder
  - [ ] if folder is uploaded, zip it and send it then unzip it
- [X] dont download symlinks, unless specified with flag
- [ ] allow setting a password & username
  - `--auth, -a`, or `--auth-file` (maybe hashed?)
- [X] show error message when path was not found with a button "back to root"
- [ ] add logging to file
- [X] add progress bar popup
- [ ] add support for multiple interfaces via `-i`, `--interface`
- [ ] support for custom headers
- [ ] tls support via `--tls-cert` and `--tls-key`
- [ ] context menu
  - [ ] allow creating directories
  - [ ] file/folder info popup
  - [ ] delete/rename ? (secured behind flag)
- [ ] add qr code button on mobile