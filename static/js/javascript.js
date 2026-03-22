document.addEventListener("DOMContentLoaded", () => {
    /* THEME SWITCHING */
    const buttons = document.querySelectorAll("[data-theme]");

    buttons.forEach(button => {
        button.addEventListener("click", () => {

            const theme = button.dataset.theme;

            document.documentElement.setAttribute(
                "data-theme",
                theme
            );

            localStorage.setItem("theme", theme);

            buttons.forEach(b =>
                b.classList.remove("active-theme")
            );

            button.classList.add("active-theme");
        });
    });

    const savedTheme = localStorage.getItem("theme");
    if (savedTheme) {
        document.documentElement.setAttribute(
            "data-theme",
            savedTheme
        );
    }

    /* DROPDOWN TOGGLE */
    const dropdown = document.querySelector(".dropdown");
    const dropdownButton =
        document.querySelector(".dropdown-button");

    dropdownButton.addEventListener("click", () => {
        dropdown.classList.toggle("open");
    });

    /* CLOSE WHEN CLICKING OUTSIDE */
    document.addEventListener("click", (event) => {

        if (!dropdown.contains(event.target)) {
            dropdown.classList.remove("open");
        }
    });
});