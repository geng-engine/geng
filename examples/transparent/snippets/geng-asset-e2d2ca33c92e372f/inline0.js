
        export function setup_image(image, handler) {
            image.onload = function() { handler(true); };
            image.onerror = function() { handler(false); };
        }
        