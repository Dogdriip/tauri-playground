import './style.css'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>Welcome from Tauri!</h1>
    <p>${window.navigator.userAgent}</p>
  </div>
`
