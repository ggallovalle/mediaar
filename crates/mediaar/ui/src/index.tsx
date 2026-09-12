/* @refresh reload */
import { render } from "@solidjs/web";
import App from "./App";

const root = document.getElementById("root");

if (!root) {
  throw new Error("Root element #root not found");
}

document.documentElement.classList.add("mocha");
render(() => <App />, root);
