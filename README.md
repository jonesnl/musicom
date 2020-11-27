# musicom
A terminal music player written in Rust.

![Screenshot](/screenshots/screenshot.png?raw=true)

# Features
Currently, musicom has a file browser to play music files from, and a rudimentary queue to play songs one after another.

Use `<?>` to get help text on available commands in musicom.

# Dependencies
musicom depends on:
* `gstreamer` to play music
* `ncurses` to draw to the terminal
* `sqlite` to store the library
* `taglib` to parse tags for the library

# Developing
`dev_env.bash` can be sourced to point `diesel_cli` at the musicom database without having to specify it manually.
