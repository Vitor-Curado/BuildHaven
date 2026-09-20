export function setupThemeSwitcher() {
  const buttons = document.querySelectorAll<HTMLButtonElement>("[data-theme]");
  const savedTheme = localStorage.getItem("theme");

  if (savedTheme) {
    document.documentElement.setAttribute("data-theme", savedTheme);
  }

  buttons.forEach((button) => {
    button.addEventListener("click", () => {
      const theme = button.dataset.theme;

      if (!theme) {
        return;
      }

      document.documentElement.setAttribute("data-theme", theme);
    });
  });
}