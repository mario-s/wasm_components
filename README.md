# WebAssembly Components

This repository is a sample to demonstrate how to use [WebAssembly components](https://github.com/WebAssembly/component-model) to build a softwaresystem based on independent, interchangeable components.

It is based on Rust and uses a [cargo subcommand](https://github.com/bytecodealliance/cargo-component?tab=readme-ov-file) to create the components.

## Quick walkthrough
### Greeter component

To create a new component with the name 'greeter' from scratch, you use the following command:

`cargo component new greeter --lib`

In directory of the component you can find a folder with the name wit and in the file word.wit is a definition of the interface in the WIT-language.

To build the component use the command `cargo component build`.

### Greeter Client

Another component 'greeter-client' can use the previous created component. So we create another one, this time without the `--lib` switch:

`cargo component new greeter-client`

In order to use the first component, the second needs to import it in thw world.wit:

```
world app {
    import component:greeter/greet@0.1.0;
}
```
Ensure that the 'greeter-client' component can find the first wit-file by adding the location needs to the Cargo.toml:
```
[package.metadata.component.target.dependencies]
"component:greeter" = { path = "../greeter/wit" }
```

To compose the system you need the tool [WAC](https://github.com/bytecodealliance/wac).

From within the greeter-client directory you execute this command:

```
wac plug target/wasm32-wasip1/debug/greeter-client.wasm --plug ../greeter/target/wasm32-wasip1/debug/greeter.wasm -o ./composed-greeter.wasm
```
The system can be started with [Wasmtime](https://wasmtime.dev).
`wasmtime ./composed-greeter.wasm`

### More components

For composing the whole provided sample you can use this command:
```
wac plug target/wasm32-wasip1/debug/greeter-client.wasm --plug ../greeter/target/wasm32-wasip1/debug/greeter.wasm  --plug ../even-odd/target/wasm32-wasip1/debug/evenodd.wasm -o ./composed-greeter.wasm
```