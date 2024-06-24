
    export function show_error(text) {
        document.getElementById("geng-progress-screen").style.display = "none";
        document.getElementById("geng-canvas").style.display = "none";
        document.getElementById("error-message").textContent = text;
        document.getElementById("geng-error-screen").style.display = "block";
    }
    