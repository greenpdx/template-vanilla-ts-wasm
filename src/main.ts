import './style.css'
import viteLogo from '/vite.svg'
import wasmpackLogo from '/wasm-ferris.png' 
import bindgen from '/rustwasm.png'
import rustferris from '/rustacean-flat-happy.svg'
import { setupCounter } from './counter.ts'
import * as wasm from '../pkg'
console.log(wasm)
//wasm.greet()

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
   <a href="https://rustwasm.github.io//" target="_blank">
      <img src="${wasmpackLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <a href="https://www.rust-lang.org/" target="_blank">
      <img src="${rustferris}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <a href="https://github.com/rustwasm/wasm-bindgen" target="_blank">
      <img src="${bindgen}" class="logo vanilla" alt="TypeScript logo" />
    </a>
   <a href="https://vite.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
     <h1>WASM + Rust + bindgen + Vite</h1>
       <p class="read-the-docs">
      Click on any logo to learn more
    </p>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <div id="wasmdom"></div>
    <canvas id="canvas"></canvas>
  </div>
`

setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)
let err = wasm.add_dom()
console.log(err)
let rslt =  await wasm.run("https://jsonplaceholder.typicode.com/users")
console.log(rslt)
console.log(wasm.rtn_js(rslt))
console.log(rslt)
const map: Map<string, wasm.User> = wasm.vec2map(rslt)
//let ans = wasm.webgl()
console.log(map)
wasm.threed()