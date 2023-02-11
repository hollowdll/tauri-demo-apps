const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", () => {
  invoke("greet", { name: document.title })
  .then(response => {
    console.log(response);
    document.querySelector("#message").textContent = response;
  });

  invoke("say_hi")
  .then(response => {
    document.querySelector("#hi-message").textContent = response;
  })
});

