# musicom
A terminal music player written in Rust.

![Screenshot](/screenshots/screenshot2.png?raw=true)

# Features
Currently, musicom has a file browser to play music files from, a basic music library, and a rudimentary queue to play songs one after another.

There is still a lot of undocumented behavior as this is a work in progress project, but as things solidify I will do my best to update the `<?>` documentation.

Use `<?>` to get help text on available commands in musicom.

# Dependencies
musicom depends on:
* `gstreamer` to play music
* `ncurses` to draw to the terminal
* `sqlite` to store the library
* `taglib` to parse tags for the library
