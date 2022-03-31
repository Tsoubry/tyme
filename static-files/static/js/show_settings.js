export function popSettings() {
    let settings = document.getElementById("settings");
    if (settings.style.display === "none") {
        settings.style.display = "block";
    } else {
        settings.style.display = "none";
    }
}