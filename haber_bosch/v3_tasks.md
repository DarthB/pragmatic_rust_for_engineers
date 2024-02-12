# Tasks - Video 03 - Haber-Bosch Analysis Tool via Web Assembly WASM

First of all: You can find the online version of the full analysis tool [here](https://hb-analysis.janus.rs) - In the following tasks we end up with a smaller proof of concept.

Therefore, this time it's recommend to follow the video first and start at the `task:` commit to build up your solution. You can find the
the implementation of the tasks in the `hint:` commit and the full analysis tool in the `solu:` commit.

First, we will start by wireing WebAssembly into a HTML-based website and use the browser side developer console for outputs. After that we work on getting the Rendering of the diagrams working in the Browser. Then we react on different events that are sent by the brower like `onResize` and `onMouseMove`. As last implementation step, we use HTML Formulars to let the user change input variables of the plot. We conclude with a few words on
the implementation of the sophisticated analysis tool.

## Learn Goals

General:

- Use multiple software stacks (Rust + Vanilla JavaScript and HTML)
- Use the WEB-Browser as sandbox and user-interface
- Provide a analysis tool to investigate Haber-Bosch multi-bed reactor configurations

Rust:

- Use wasm-pack and serve via the Web
- HTML Formulars and CSS design
- Java-Script for Event-Handling
- Bind Web-Browser Developer Console to Rust panics
- Read HTML Formulars in Rust

## Implementation Plan and Tasks Description

We will divide our work in four parts. It is recommended to follow the video to get this initial steps right. This time we have
a slightly different meaning of the commits:

- `task: ` - contains the task description and code in `web_visualization.rs` and is the same starting point as in the video.
- `hint: ` - contains the code of the partly implemented analysis tool at the end of the video.
- `solu: ` - contains the code of the "full" analysis tool that was used during the first part of the video for the analysis with Jonathan.

1. Use the developer console of the Browser to output things from Rust
   1. Install `wasm-pack` and `basic-http-server`.
   2. Map the java-script console methods in the Rust scope
   3. Implement a macro for console output
   4. End panic-ing... ;-D
2. Render the Canvas in the HTML Browser window
   1. Add a `WebChart` and a `WebPoint` class to the Rust Code
   2. Add a `simulation` and a `drawing` function to `WebChart`
   3. Investigate new function signature for internal drawing functions
   4. Overwork `bootstrap.js` and implement the `setup` and `main` function in JavaScript
   5. Implement a `updatePlot` function in JavaScript 
3. Add Javascript Event functions and Interactivity
   1. Implement `onMouseMove` and `Resize`
   2. Add HTML Formulars for catalyst and pressure and re-trigger plotting when they change.
4. Look at the final solution

...

Feel free to extend the tool to your liking.

## References

This episode was all about exposing our Rust Implementation to users via a Browser. Therefore we used WebAssembly (WASM). For the WEB coupling the following tools were used:

1. https://github.com/rustwasm/wasm-bindgen - GitHub Repository of the Crate that allows us to use WebAssembly WASM as output target for our Rust Code.
2. https://github.com/rustwasm/wasm-pack - GitHub Repository of the command line tool we use to build WebAssembly packages.
3. https://github.com/brson/basic-http-server - A simple http server useful for development. Beware it should not be used in production.
4. https://www.nginx.com/ - A production ready http server. Used to serve the analysis tool online. [Analysis Tool](https://hb-analysis.janus.rs/)

As some of your are starting your journey with Rust and this course cannot give an in-depth introduction but focuses and solving problems with Rust and on this way just provides **enough** detail, we want to make you aware of those very good starter materials:

1. [The Rust Book](https://doc.rust-lang.org/book/) - Gives an introduction suitable for people who have experiences in other coding languages.
2. [Rust By Examlpe](https://doc.rust-lang.org/rust-by-example/) - A collection of code examples with short explanations similiar to the `hints.rs` files you find in this course.
3. [Rustlings](https://github.com/rust-lang/rustlings) - A project containing small Rust exercises and let you solve them in an interactive way. It follows the structure of [the rust book](https://doc.rust-lang.org/book/).
4. [The Cargo book](https://doc.rust-lang.org/cargo/index.html) - although we won't dig deep into it, we will play around with multiple binaries and we have a workspace structure that may become in handy in the future.
5. [Rust and WebAssembly](https://rustwasm.github.io/docs/book/) - A book with information about Rust and WebAssembly. It uses Conway's Game of Life as motivating example for a tutorial on the topic.