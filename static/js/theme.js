window.setTheme = function (theme) {
    document.documentElement.setAttribute("data-theme", theme)
    localStorage.setItem("theme", theme)
}

function loadTheme() {
    const savedTheme = localStorage.getItem("theme")

    if (savedTheme) {
        document.documentElement.dataset.theme = savedTheme
    } else {
        document.documentElement.dataset.theme = "night"
    }
}

loadTheme();