export function setupDropdown(element: HTMLElement) {
  const button = element.querySelector<HTMLButtonElement>(
    ".dropdown-button",
  );

  if (!button) {
    return;
  }

  const handleClick = () => {
    element.classList.toggle("open");
  };

  button.addEventListener("click", handleClick);

  return () => {
    button.removeEventListener("click", handleClick);
  };
}