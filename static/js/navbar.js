document.addEventListener("click", function (event) {
    const themeButton = event.target.closest("[data-theme]");

    if (themeButton) {
        const theme = themeButton.dataset.theme;
        setTheme(theme);
        return;
    }
});