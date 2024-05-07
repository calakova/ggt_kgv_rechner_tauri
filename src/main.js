const { invoke } = window.__TAURI__.tauri;

let input1;
let input2;
let results;

async function calc() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  results.textContent = await invoke("calc", { num1: input1.value, num2: input2.value });
}

window.addEventListener("DOMContentLoaded", () => {
  input1 = document.querySelector("#input1");
  input2 = document.querySelector("#input2");
  results = document.querySelector("#results");
  document.querySelector("#calc-form").addEventListener("submit", (e) => {
    e.preventDefault();
    calc();
  });
});
