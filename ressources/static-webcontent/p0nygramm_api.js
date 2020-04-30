// The callback function get two parameters:
// Parameter 1: Request completed (false if there was a timeout)
// Parameter 2: Result XMLHttpRequest object

const httpTimeout = 15000 // timeout in milliseconds

function send_http_request(method, path, content, callback) {
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

    xhttp.open(method, path, true);

    if (content === null) {
        xhttp.send();
    }
    else {
        xhttp.send(content);
    }
}

function login(username, password, keep_logged_in, callback) {
    var url_encoded_form_data = "";
    url_encoded_form_data.concat("username=", username, "&");
    url_encoded_form_data.concat("password=", password, "&");
    url_encoded_form_data.concat("keep_logged_in=", keep_logged_in);

    send_http_request("POST", "js-api/login", url_encoded_form_data, callback);
}

function logout(callback) {
    send_http_request("GET", "js-api/logout", null, callback);
}