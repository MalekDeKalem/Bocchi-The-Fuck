# How to run

- install rust if you do not have it
[follow the official manual](https://www.rust-lang.org/tools/install)
- install wasm-pack with the rust packagemanager (cargo)
```cargo install wasm-pack```
- also run ```npm install``` and ```cargo build``` to ensure that the javascript dependecies and the rust dependenciesare met
- run  ```wasm-pack build --target web``` to build our wasm project
- now you are ready to open the project in a browser using a localserver (I used a vscode extension called "Live Server for that")

# What is Bocchi-The-Fuck?
It is an esoteric programming language similar to "Brainfuck" anoter esoteric programming language. Brainfuck consists of 8 symbols,
Bocchi the fuck on the other hand has 8 japanese phrases/words (written in hiragana) that are a reference from the popular Anime "Bocchi the Rock". 

| Bocchi-The-Fuck | Romaji | Brainfuck | Function |
| --------------- | ------- | --------- | -------- |
| ぼち | Bocchi | + | Increments value |
| ごと | Gotoh | - | Decrements value |
| わかる | Wakaru | . | Outputs value |
| いくよ | Ikuyo | , | Expect Input |
| きた | Kita | [ | is kind of like beginning of a while loop |
| ぎた | Gita | ] | is kind of like ending of while loop |
| ちんぽ | Chinpo | > | Increments Pointer |
| つんでれ | Tsundere | < | Decrements Pointer |

Hello World in BocchiTheFuck

<!-- ```
ぼちきたごとごとごとごとごとちんぽぼちぼちぼちつんでれぎたちんぽぼちわかるぼちわかる
きたごとごとごとちんぽぼちつんでれぎたちんぽごとごとごとわかる
ぼちきたごとごとごとごとごとちんぽぼちつんでれぎたちんぽわかるぼちぼちわかるごとごとわかる
``` -->

```
ぼちぼちぼちぼちぼちぼちぼちぼちきたちんぽぼちぼちぼちぼちきたちんぽぼちぼちちんぽぼちぼちぼちちんぽぼちぼちぼちちんぽぼち
つんでれつんでれつんでれつんでれごとぎたちんぽぼちちんぽぼちちんぽごとちんぽちんぽぼちきたつんでれぎたつんでれごとぎた
ちんぽちんぽわかる
ちんぽごとごとごとわかる
ぼちぼちぼちぼちぼちぼちぼちわかるわかるぼちぼちぼちわかる
ちんぽちんぽわかる
つんでれごとわかる
つんでれわかる
ぼちぼちぼちわかるごとごとごとごとごとごとわかるごとごとごとごとごとごとごとごとわかる
ちんぽちんぽぼちわかる
ちんぽぼちぼちわかる
```

This should give you this output:
```
Hello World!
```
