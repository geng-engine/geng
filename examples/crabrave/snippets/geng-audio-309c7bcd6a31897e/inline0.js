
            export function setup(request, handler) {
                request.onreadystatechange = function () {
                    if (request.readyState == 4) {
                        handler(request.status == 200 || request.status == 206); // TODO why is there 206?
                    }
                };
            }
            