# Start Leptos & Tauri template
The leptos part is based off this template: https://github.com/leptos-rs/start.
The Tauri part is based off this command: `cargo tauri init`.

## How to use:
I have not setup `cargo generate` yet, so you will have to clone this repo and
manually copy the files into your project.
Or, just poke around and copy some configs or code, as you wish.

## How to run:
To run tauri, run `cargo tauri dev` (or preferrably `RUST_LOG=info cargo tauri dev` for good logging).
To view static site, run `trunk serve --open --features ssg`.
To view serve site, run `cargo leptos serve` and open to `http://localhost:3000/`.
Currently, both `cargo leptos` and `trunk serve` use the same folder: `dist`, so
don't run them together (unless you change the folders they output to).

## WIP:
This template works for me on my machine, but there are still a few issues
- There is still an error about conflicting build artifacts


## Development:
I am developing a personal project, this is intended to give a foothold for others
who are trying to solve the same issues as me.

This article helped me: https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe

I am very happy to document this more, please, just add a github issue and I will!