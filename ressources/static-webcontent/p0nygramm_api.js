// The callback function get two parameters:
// Parameter 1: Response code (on timeout: 503)
// Parameter 2: Response JSON object (on timeout: null)

const httpTimeout = 10000 // 10 seconds
const httpTimeoutFileUpload = 120000 // 2 minutes

// POST data formats:
const post_urlencoded = 1;
const post_multipart_fd = 2; // content must be a FormData object (used for file upload)

// Global variables
var last_send_progress = -1;
var xhttp_ref = null;

// API functions
function login(username, password, keep_logged_in, callback) {
    var url_encoded_form_data = "";
    url_encoded_form_data.concat("username=", username, "&");
    url_encoded_form_data.concat("password=", password, "&");
    url_encoded_form_data.concat("keep_logged_in=", keep_logged_in);

    send_http_request("POST", post_urlencoded, "js-api/login", url_encoded_form_data, callback, null);
}

function logout(callback) {
    send_http_request("GET", null, "js-api/logout", null, callback, null);
}

// Helper functions
function send_http_request(method, post_data_format, path, content, callback, progress_callback) {
    var xhttp = new XMLHttpRequest();
    xhttp.timeout = httpTimeout;

    // Set options for post content type
    if (post_data_format == post_urlencoded) {
        xhttp.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
    }
    else if (post_data_format == post_multipart_fd) {
        xhttp_ref = xhttp;
    }

    // Request finished handler
    xhttp.onreadystatechange = function() {
        if (this.readyState == 4) {
            if (xhttp_ref != null) {
                xhttp_ref = null;
            }

            callback(this.status, this.response)
        }
    };

    // Timeout handler
    xhttp.ontimeout = function() {
        callback(503, null)
    };

    // Progress report
    if (progress_callback != null) {
        xhttp.upload.onprogress = function(event) {
            var progress = Math.round(100 / event.total * event.loaded);

            // Prevent multiple callbacks for the same progress number
            if (progress != last_send_progress) {
                last_send_progress = progress;
                progress_callback(progress);
            }
        };
    }

    xhttp.open(method, path, true);

    if (content === null) {
        xhttp.send();
    }
    else {
        xhttp.send(content);
    }
}