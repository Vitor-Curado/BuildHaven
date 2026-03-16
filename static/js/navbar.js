document.addEventListener("click", function (event) {
    const dropdowns = document.querySelectorAll(".dropdown");

    dropdowns.forEach(dropdown => {
        const button = dropdown.querySelector(".dropdown-button");

        if (button.contains(event.target)) {
            dropdown.classList.toggle("open");
            return;
        }

        if (!dropdown.contains(event.target)) {
            dropdown.classList.remove("open");
        }
    });
});