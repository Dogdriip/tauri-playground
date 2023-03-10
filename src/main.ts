import { invoke } from '@tauri-apps/api'
import './style.css'

(async function() {
  document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
    <div>
      <h1>Welcome from Tauri!</h1>

      <h2>From JS:</h2>
      <p>${window.navigator.userAgent}</p>

      <h2>From Rust:</h2>
      <p>${await invoke('arch')}</p>
      <p>${await invoke('family')}</p>
      <p>${await invoke('os')}</p>
    </div>
  `
})()
