import "./style.css";
import typescriptLogo from "./typescript.svg";
import viteLogo from "/vite.svg";
import { setupCounter } from "./counter.ts";
import { Empire } from "../../pkg/empire";

const db = new Empire();

db.set("key", "value");
db.set("1", "One");
db.set("2", "two");
db.set("3", "three");
db.set("4", "four");
db.set("5", "five");
db.set("6", "six");

// Get a value from the database
const result = db.get("key");
console.log(result); // Output: value

// Remove a value from the database
const removed = db.remove("key");
console.log(removed); // Output: true

console.log(db.getAll());

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <h1>Vite + TypeScript</h1>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <p class="read-the-docs">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`;

setupCounter(document.querySelector<HTMLButtonElement>("#counter")!);
