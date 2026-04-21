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
    const dropdowns =
        document.querySelectorAll(".dropdown");

    dropdowns.forEach(dropdown => {

        const button =
            dropdown.querySelector(".dropdown-button");

        button.addEventListener("click", event => {

            event.stopPropagation();
            dropdown.classList.toggle("open");
        });
    });

    /* CLOSE WHEN CLICKING OUTSIDE */
    document.addEventListener("click", () => {
        dropdowns.forEach(dropdown => {
            dropdown.classList.remove("open");
        });

    });

    document.addEventListener("keydown", event => {
        if (event.key === "Escape") {
            dropdowns.forEach(dropdown => {
                dropdown.classList.remove("open");
            });
        }
    });
});