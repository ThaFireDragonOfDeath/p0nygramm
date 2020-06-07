// Common functions called from HTML

// API functions
function js_login() {
    // Read input data
    var login_data = ui_get_login_data();

    // Disable login and register button
    ui_set_auth_state(ui_btn_state.deactivate);

    // Send login data
    api_login(login_data.username, login_data.password, login_data.keep_logged_in, js_login_callback);
}

function js_logout() {
    api_logout(js_logout_callback)
}

function js_register() {
    // Read input data
    var register_data = ui_get_register_data();

    // Check password
    if (register_data.password !== register_data.rpassword) {
        ui_report_msg("Die eingegebenen Passwörter stimmen nicht überein!", ui_message_output.register, ui_message_type.error);

        return;
    }

    // Disable login and register button
    ui_set_auth_state(ui_btn_state.deactivate);

    // Send register data
    api_register(register_data.username, register_data.password, register_data.invite_key, js_register_callback);
}

// Callback functions
function js_login_callback(response_code, response_content) {
    // Handle backend errors
    if (response_code != 200) {
        var error_msg = response_content.error_msg;

        ui_report_msg(error_msg, ui_message_output.login, ui_message_type.error);
        ui_set_auth_state(ui_btn_state.activate);

        return;
    }

    // Report login success
    ui_report_msg("Anmeldung erfolgreich. Die Seite wird in Kürze neu geladen.", ui_message_output.login, ui_message_type.success);

    // Reload page after two seconds
    window.setTimeout(ui_page_reload, 2000);
}

function js_logout_callback(response_code, response_content) {
    ui_page_reload();
}

function js_register_callback(response_code, response_content) {
    // Handle backend errors
    if (response_code != 200) {
        var error_msg = response_content.error_msg;
        js_register_error(message);

        document.getElementById("log_submit").disabled = false;
        document.getElementById("reg_submit").disabled = false;

        return;
    }

    // Report register success
    ui_report_msg("Registrierung erfolgreich abgeschlossen. Sie werden in Kürze angemeldet.", ui_message_output.register, ui_message_type.success);

    // Read input data
    var register_data = ui_get_register_data();

    // Login into the new registered account
    api_login(register_data.username, register_data.password, false, js_reglogin_callback);
}

function js_reglogin_callback(response_code, response_content) {
    // Handle backend errors
    if (response_code != 200) {
        var error_msg = response_content.error_msg;

        ui_report_msg(error_msg, ui_message_output.register, ui_message_type.error);
        ui_set_auth_state(ui_btn_state.activate);

        return;
    }

    ui_report_msg("Anmeldung erfolgreich. Die Seite wird in Kürze neu geladen.", ui_message_output.register, ui_message_type.success);

    // Reload page after two seconds
    window.setTimeout(ui_page_reload, 2000);
}
