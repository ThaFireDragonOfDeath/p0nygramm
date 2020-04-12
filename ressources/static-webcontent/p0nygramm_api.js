// The callback function get two parameters:
// Parameter 1: Request completed (false if there was a timeout)
// Parameter 2: Result XMLHttpRequest object

const httpTimeout = 15000 // time in milliseconds

function logout(callback) {
    var xhttp = new XMLHttpRequest();
    xhttp.timeout = httpTimeout;

    xhttp.onreadystatechange = function() {
        if (this.readyState == 4) {
            callback(true, this)
        }
    };
    xhttp.ontimeout = function() {
        callback(false, this)
    };

    xhttp.open("GET", "js-api/logout", true);
    xhttp.send();
}