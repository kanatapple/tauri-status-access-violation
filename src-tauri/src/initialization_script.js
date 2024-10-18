document.addEventListener('DOMContentLoaded', () => {
  window.__TAURI__.event.listen('test', (event) => {
    console.log(event);
  });
});
