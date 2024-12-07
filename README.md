# Teach Rust in browser template<br>
<br>

### Scaffolding Your First WASM in browser Project<br>
***Install Rust***<br>
Go to [Rustup](https://rustup.rs/) and install Rust

**Install WASM Support**<br>
go to [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and install wasm-pack

> **Compatibility Note:**
> Vite requires [Node.js](https://nodejs.org/en/) version 18+, 20+. However, some templates require a higher Node.js version to work, please upgrade if your package manager warns about it.
<br>
There are two ways to install this template<br>
<br>
$ npx degit greenpdx/template-vanilla-ts-wasm APPNAME
<br>
or 
<br>
$ git clone https://github.com/greenpdx/template-vanilla-ts-wasm.git APPNAME
<br>
<br>
$ cd APPNAME<br>
$ npm install<br>
$ npm run wasm  # This needs to be done after every rust change<br>
$ npm run dev<br>
<br>
> working on watching rust file changes


