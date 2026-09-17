import { useEffect, useState } from "react";

const themes = ["forest", "night", "ocean", "sunset"] as const;

type Theme = (typeof themes)[number];

const ThemeSwitcher = () => {
  const [theme, setTheme] = useState<Theme | null>(null);

  useEffect(() => {
    const savedTheme = localStorage.getItem("theme") as Theme | null;

    if (savedTheme && themes.includes(savedTheme as Theme)) {
      const saved = savedTheme as Theme;

      setTheme(saved);
      document.documentElement.setAttribute("data-theme", saved);
    }
  }, []);

  const selectTheme = (newTheme: Theme) => {
    setTheme(newTheme);
    document.documentElement.setAttribute("data-theme", newTheme);
    localStorage.setItem("theme", newTheme);
  };

  return (
    <ul className="dropdown-menu">
      {themes.map((themeName) => (
        <li key={themeName}>
          <button
            type="button"
            className={`dropdown-link${theme === themeName ? " active-theme" : ""}`}
            data-theme={themeName}
            onClick={() => selectTheme(themeName)}
          >
            {themeName}
          </button>
        </li>
      ))}
    </ul>
  );
};

export default ThemeSwitcher;
