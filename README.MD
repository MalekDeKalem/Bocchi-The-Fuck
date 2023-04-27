# How to run

- install rust if you do not have it
[follow the official manual](https://www.rust-lang.org/tools/install)
- install wasm-pack with the rust packagemanager (cargo)
```cargo install wasm-pack```
- also run ```npm install``` and ```cargo build``` to ensure that the javascript dependecies and the rust dependenciesare met
- run  ```wasm-pack build --target web``` to build our wasm project
- now you are ready to open the project in a browser using a localserver (I used a vscode extension called "Live Server for that")

# What is Bocchi the fuck?
It is an esoteric programming language similar to "Brainfuck" anoter esoteric programming language. Brainfuck consists of 8 symbols,
Bocchi the fuck on the other hand has 8 japanese phrases/words (written in hiragana) that are a reference from the popular Anime "Bocchi the Rock". 

| Bocchi-The-Fuck | Brainfuck | Function |
| --------------- | --------- | -------- |
| ぼち | + | Increments value |
| ごと | - | Decrements value |
| わかる | . | Outputs value |
| いくよ | , | Expect Input |
| きた | [ | is kind of like beginning of a while loop |
| ぎた | ] | is kind of like ending of while loop |
| ちんぽ | > | Increments Pointer |
| つんでれ | < | Decrements Pointer |