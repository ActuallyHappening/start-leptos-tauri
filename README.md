# Start Leptos & Tauri template
The leptos part is based off this template: https://github.com/leptos-rs/start.
The Tauri part is based off this command: `cargo tauri init`.

## How to use:
To run tauri, run `cargo tauri dev`.
To view static site, run `trunk serve --open --features ssg`.
To view serve site, run `cargo leptos serve` and open to `http://localhost:3000/`.
Currently, both `cargo leptos` and `trunk serve` use the same folder: `dist`, so
don't run them together (unless you change the folders they output to).

## WIP:
This template works for me on my machine, but there are still a few issues
- There is still an error about conflicting build artifacts
- I haven't tested all of leptos' features using client side rendering (i.e. using trunk)

## Development:
I am developing a personal project, this is intended to give a foothold for others
who are trying to solve the same issues as me.

I am very happy to document this more, please, just add a github issue and I will!