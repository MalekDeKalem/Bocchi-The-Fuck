# How to run

- install rust if you do not have it
[follow the official manual](https://www.rust-lang.org/tools/install)
- install wasm-pack with the rust packagemanager (cargo)
```cargo install wasm-pack```
- also run ```npm install``` and ```cargo build``` to ensure that the javascript dependecies and the rust dependenciesare met
- run  ```wasm-pack build --target web``` to build our wasm project
- now you are ready to open the project in a browser using a localserver (I used a vscode extension called "Live Server for that")