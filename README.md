# Learning on a foreign project
I was interested in Rust and cloned a Project, that was not maintained. There was no documentation of
what the program should do or if it ever did something. When I started, it was failing at Compilation.

For Compiling on linux, you might need to install additional packages. For me, the following helped:

``sudo apt install libalsaplayer-dev libudev-dev libasound2-dev``
___
# Compiling and running the project
Cargo will go a long way in helping you here. It should be a simple ```cargo run```. Tests (``cargo test``) wil also run, currently there are no tests implemented.
# Contributing
Feel free to open any Issues.