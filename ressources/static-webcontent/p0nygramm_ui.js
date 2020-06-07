// Functions to interact with the gui

// Const variables
// Submit buttons
const ui_login_btn = "log_submit";
const ui_register_btn = "reg_submit";

// Message outputs
const ui_login_output_element = "div_login_error";
const ui_register_output_element = "div_register_error";

// Register input elements
const ui_reg_username = "reg_username";
const ui_reg_password = "reg_password";
const ui_reg_rpassword = "reg_rpassword";
const ui_reg_invite_key = "reg_invite_key";

// Login input elements
const ui_log_username = "log_username";
const ui_log_password = "log_password";
const ui_keep_logged_in = "log_keep_logged_in";

// Enums
const ui_btn_state = {
    activate: 0,
    deactivate: 1
};

const ui_message_output = {
    login: 0,
    register: 1,
    alert: 2
};

const ui_message_type = {
    normal: 0,
    error: 1,
    success: 2
};

// API Functions
function ui_get_login_data() {
    var login_data = {
        username: document.getElementById(ui_log_username).value,
        password: document.getElementById(ui_log_password).value,
        keep_logged_in: document.getElementById(ui_keep_logged_in).checked
    };

    return login_data;
}

function ui_get_register_data() {
    var register_data = {
        username: document.getElementById(ui_reg_username).value,
        password: document.getElementById(ui_reg_password).value,
        rpassword: document.getElementById(ui_reg_rpassword).value,
        invite_key: document.getElementById(ui_reg_invite_key).value
    };

    return register_data;
}

// Hide message
function ui_hide_msg(output_channel) {
    var output_element = null;

    // Get output element
    if (output_channel === ui_message_output.login) {
        output_element = document.getElementById(ui_login_output_element);
    }
    else if (output_channel === ui_message_output.register) {
        output_element = document.getElementById(ui_register_output_element);
    }

    output_element.style.display = "none";
}

// Enable or disable the login and register button
function ui_set_auth_state(btn_state) {
    if (btn_state === ui_btn_state.activate) {
        document.getElementById(ui_login_btn).disabled = false;
        document.getElementById(ui_register_btn).disabled = false;
    }
    else if (btn_state === ui_btn_state.deactivate) {
        document.getElementById(ui_login_btn).disabled = true;
        document.getElementById(ui_register_btn).disabled = true;
    }
}

// Reload page
function ui_page_reload() {
    location.reload();
}

function ui_report_msg(message, output_channel, message_type) {
    var output_element = null;
    var msg_color = "white";

    // Get output element
    if (output_channel === ui_message_output.login) {
        output_element = document.getElementById(ui_login_output_element);
    }
    else if (output_channel === ui_message_output.register) {
        output_element = document.getElementById(ui_register_output_element);
    }

    // Get font color
    if (output_channel !== ui_message_output.alert) {
        if (message_type === ui_message_type.error) {
            msg_color = "red";
        }
        else if(message_type === ui_message_type.success) {
            msg_color = "green";
        }
    }

    // Print message
    if (output_element !== null) {
        output_element.innerHTML = message;
        output_element.style.color = msg_color;
        output_element.style.display = "block";
    }
}