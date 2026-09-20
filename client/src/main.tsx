import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { setupDropdown } from "./components/Dropdown";
import { setupThemeSwitcher } from "./components/ThemeSwitcher";
import "./index.css";

const root = document.getElementById("root");

if (root) {
  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
}

const dropdowns = document.querySelectorAll<HTMLElement>("[data-dropdown]");

dropdowns.forEach(setupDropdown);

setupThemeSwitcher();