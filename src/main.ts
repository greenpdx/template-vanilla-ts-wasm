import './style.css'
import typescriptLogo from './typescript.svg'
import viteLogo from '/vite.svg'
import wasmpackLogo from '/rustwasm.png'
import rustferris from '/rustacean-flat-happy.svg'
import { setupCounter } from './counter.ts'
import * as wasm from '../pkg'

//wasm.greet()

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
   <a href="https://rustwasm.github.io/wasm-pack/installer/" target="_blank">
      <img src="${wasmpackLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <a href="https://www.rust-lang.org/" target="_blank">
      <img src="${rustferris}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <a href="https://vite.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
     <h1>WASM + Rust + Vite + TypeScript</h1>
       <p class="read-the-docs">
      Click on any logo to learn more
    </p>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>

    <div id="wasmdom">
    </div>
  </div>
`

setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)
let err = wasm.add_dom()
console.log(err)
