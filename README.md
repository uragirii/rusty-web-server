# Rusty Web Server

This is a very basic implementation of a `HTTP/1.1` web server in Rust. The initial idea came from the Udemy Course ["Learn Rust by Building Real Applications"](https://www.udemy.com/course/rust-fundamentals/) by _Lyubomir Gavadinov_. It's a great course where you learn Rust by implementing practical things.

## What can it do?

Welp being a very simple web server it can't do much. Currently, it can recieve and parse the `Request` and now it returns a very minimal `Response` also! I have some todos for it but I'm sure if they would be completed.

To run, make sure you have Rust installed and then simply do `cargo run` inside the project dir.

## What will it do?

It would be great if it can :

- [x] Parse `HTTP` Headers
  - [ ] Support getters for headers (case insensitive).
- [x] Return a simple `Response`.
- [ ] Follow `HTML` spec more tightly
  - [ ] Return appropriate `Response Codes`
  - [ ] Support for more `HTTP Methods`
- [ ] Serve files, CSS, HTML, JS!
- [ ] Render templates! (sounds like a big boi thing)
- [ ] Compete with Next.js
- [ ] Make me super rich
- [ ] Solve all world problems

tbh last 3 seems impossible but I have hopes.

## What does this project use?

This project uses [Rust](https://www.rust-lang.org/). Rust is a systems programming language and a lot of people love it. I'm trying it too!

You can find the dependencies in the `Cargo.toml` files, _if any_. I would try to link them here also.

## Looks great can I help?

I would love that! I'm still a beginner in Rust and this is a fun project so hop on! In general I would like to implement this on my own (& learn Rust). But there are many things were I would opt for a _programitically cheaper_ path and not give too much thoughts about it. If you can please, open a PR. (Bugs are also welcome!)

<details><summary>I would try to like here</summary>
I Forgor 💀
</details>
