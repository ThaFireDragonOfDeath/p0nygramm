// Common functions called from HTML

// API functions
function js_register() {
    // Read input data
    var username = document.getElementById("reg_username").value;
    var password = document.getElementById("reg_password").value;
    var rpassword = document.getElementById("reg_rpassword").value;
    var invite_key = document.getElementById("reg_invite_key").value;

    // Check password
    if (password != rpassword) {
        js_register_error("Die eingegebenen Passwörter stimmen nicht überein!");
        return;
    }

    // Disable register button
    document.getElementById("reg_submit").disabled = true;

    // Send register data
    api_register(username, password, invite_key, js_register_callback);
}

// Helper functions
function js_register_callback(response_code, response_content) {
    // Handle backend errors
    if (response_code != 200) {
        var error_msg = response_content.error_msg;
        js_register_error(message);
        document.getElementById("reg_submit").disabled = false;
        return;
    }

    js_register_success("Registrierung erfolgreich abgeschlossen. Seite wird in Kürze neu geladen.");

    // Reload page after two seconds
    window.setTimeout(js_page_reload, 2000);
}

function js_register_error(message) {
    document.getElementById("div_register_error").innerHTML = message;
    document.getElementById("div_register_error").style.color = "red";
    document.getElementById("div_register_error").style.display = "block";
}

function js_register_success(message) {
    document.getElementById("div_register_error").innerHTML = message;
    document.getElementById("div_register_error").style.color = "green";
    document.getElementById("div_register_error").style.display = "block";
}

function js_page_reload() {
    location.reload();
}