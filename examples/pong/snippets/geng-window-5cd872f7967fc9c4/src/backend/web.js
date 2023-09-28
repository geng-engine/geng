export function initialize_window(canvas) {
    window.GENG_CANVAS_SCALE = 1.0;
    canvas.tabIndex = -1;
    function update() {
        canvas.width = Math.floor(canvas.clientWidth * window.GENG_CANVAS_SCALE);
        canvas.height = Math.floor(canvas.clientHeight * window.GENG_CANVAS_SCALE);

        var document = window.document;
        if (document.fullscreenElement ||
            document.mozFullScreenElement ||
            document.webkitFullscreenElement ||
            document.msFullscreenElement) {
            screen.lockOrientationUniversal = screen.lockOrientation || screen.mozLockOrientation || screen.msLockOrientation;
            if (screen.lockOrientationUniversal) {
                screen.lockOrientationUniversal("landscape");
            } else {
                try {
                    screen.orientation.lock("landscape").catch(function () {
                    });
                } catch (e) { }
            }
        } else {
            screen.unlockOrientationUniversal = screen.unlockOrientation || screen.mozUnlockOrientation || screen.msUnlockOrientation;
            if (screen.unlockOrientationUniversal) {
                screen.unlockOrientationUniversal();
            } else {
                try {
                    screen.orientation.unlock();
                } catch (e) { }
            }
        }
    };
    window.setInterval(update, 100);
    update();
}

export function is_fullscreen() {
    var document = window.document;
    if (document.fullscreenElement ||
        document.mozFullScreenElement ||
        document.webkitFullscreenElement ||
        document.msFullscreenElement) {
        return true;
    } else {
        return false;
    }
}

export function set_fullscreen(elem, fullscreen) {
    if (fullscreen) {
        if (elem.requestFullscreen) {
            elem.requestFullscreen();
        } else if (elem.msRequestFullscreen) {
            elem.msRequestFullscreen();
        } else if (elem.mozRequestFullScreen) {
            elem.mozRequestFullScreen();
        } else if (elem.webkitRequestFullscreen) {
            elem.webkitRequestFullscreen();
        }
    } else {
        var document = window.document;
        if (document.cancelFullScreen) {
            document.cancelFullScreen();
        } else if (document.msExitFullscreen) {
            document.msExitFullscreen();
        } else if (document.mozCancelFullScreen) {
            document.mozCancelFullScreen();
        } else if (document.webkitCancelFullScreen) {
            document.webkitCancelFullScreen();
        }
    }
}

export function show() {
    document.getElementById("geng-progress-screen").style.display = "none";
    document.getElementById("geng-canvas").style.display = "block";
}

export function request_animation_frame_loop(f) {
    function loop() {
        f();
        requestAnimationFrame(loop);
    }
    loop();
}