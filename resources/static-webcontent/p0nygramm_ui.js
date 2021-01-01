// Functions to interact with the gui

// Const variables
// Submit buttons
const ui_login_btn = "log_submit";
const ui_register_btn = "reg_submit";

// Register elements
const ui_reg_username = "reg_username";
const ui_reg_password = "reg_password";
const ui_reg_rpassword = "reg_rpassword";
const ui_reg_invite_key = "reg_invite_key";
const ui_register_output_element = "div_register_error";

// Login elements
const ui_log_username = "log_username";
const ui_log_password = "log_password";
const ui_keep_logged_in = "log_keep_logged_in";
const ui_login_output_element = "div_login_error";

// Overlay elements
const ui_overlay_container_id = "overlay_container";
const ui_overlay_msg_id = "overlay_msg_txt";

// Upload elements
const ui_main_content_section_id = "main_content";
const ui_prv_id_prefix = "prvh_";
const ui_upl_id_prefix = "uplh_";

// Size constants
const ui_prv_size = 100;
const ui_prv_padding = 2;

// Enums
const ui_btn_state = {
    activate: 0,
    deactivate: 1
};

const ui_message_output_channel = {
    login: 0,
    register: 1,
    overlay: 2,
    content_section: 3
};

const ui_message_type = {
    normal: 0,
    error: 1,
    success: 2
};

// Global variables
var active_upload_prv_objects = null;
var active_upload_object = null;
var current_upload_rows = 0;

// API Functions
function ui_calc_row_len() {
    var display_width = document.getElementById(ui_main_content_section_id).offsetWidth;
    var req_len = display_width / (ui_prv_size + ui_prv_padding)

    return Math.ceil(req_len);
}

function ui_clear_content_section() {
    active_upload_prv_objects = null;
    active_upload_object = null;

    var content_element = document.getElementById(ui_main_content_section_id);

    content_element.innerHTML = "";
}

function ui_close_overlay() {
    var overlay_container = document.getElementById(ui_overlay_container_id);

    output_element.style.display = "none";
}

function ui_display_upload(upload_id, display_after_row) {
    var target_row_is_last_row = false;

    if (display_after_row >= current_upload_rows - 1) {
        target_row_is_last_row = true;
    }

    var content_section_element = document.getElementById(ui_main_content_section_id);

    // Display upload
    var upload_prv_data = ui_get_upload_prv_data_by_id(upload_id);
    var upload_url = upload_prv_data.upload_url;
    var upload_type = upload_prv_data.upload_type;

    // TODO: Implement

    // Display upvotes, tags, comments, etc
    //var get_upload_data_callback = function(response_code, response_content, callback_param) {
    //    // Handle backend errors
    //    if (response_code != 200) {
    //        var error_msg = response_content.error_msg;
    //        ui_report_msg(error_msg, ui_message_output_channel.overlay, ui_message_type.error);
    //        return;
    //    }
    //}
}

function ui_display_upload_prv() {
    var uploads_count = active_upload_prv_objects.length;
    var uploads_per_row = ui_calc_row_len();
    var current_upload_column = 0;
    var current_upload_row = 0;
    var current_content_section_html = "";

    for (var i = 0; i < uploads_count; i++) {
        var current_upload_prv = active_upload_prv_objects[i];

        var current_upload_id = current_upload_prv.upload_id;
        var current_upload_prv_url = current_upload_prv.upload_prv_url;

        if (typeof current_upload_prv_url == "string") {
            if (current_upload_column === 0) {
                current_content_section_html += "<div id=\"row_" + current_upload_row + "\">";
            }
            else if (current_upload_column >= uploads_per_row) {
                current_content_section_html += "</div>";
                current_upload_column = 0;
                current_upload_row += 1;
            }

            current_content_section_html += "<a class=\"upload_prv\" onclick=\"ui_display_upload(";
            current_content_section_html += current_upload_id + ", " + current_upload_row + ")\">";
            current_content_section_html += "<img src=\"";
            current_content_section_html += current_upload_prv_url;
            current_content_section_html += "\"></a>";

            current_upload_column += 1;
        }
        else {
            console.error("Failed to get prv url for the current upload");
        }

        var content_element = document.getElementById(ui_main_content_section_id);
        content_element.innerHTML = current_content_section_html;
        current_upload_rows = current_upload_row;
    }
}

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
    if (output_channel === ui_message_output_channel.login) {
        output_element = document.getElementById(ui_login_output_element);
    }
    else if (output_channel === ui_message_output_channel.register) {
        output_element = document.getElementById(ui_register_output_element);
    }
    else if (output_channel === ui_message_output_channel.overlay) {
        ui_close_overlay();

        return;
    }

    if (output_element != null) {
        output_element.style.display = "none";
    }
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

function ui_set_uploads_prv(uploads) {
    active_upload_prv_objects = uploads;
}

// Reload page
function ui_page_reload() {
    location.reload();
}

function ui_report_msg(message, output_channel, message_type) {
    var output_element = null;
    var msg_color = "white";

    // Get output element
    if (output_channel === ui_message_output_channel.login) {
        output_element = document.getElementById(ui_login_output_element);
    }
    else if (output_channel === ui_message_output_channel.register) {
        output_element = document.getElementById(ui_register_output_element);
    }
    else if (output_channel === ui_message_output_channel.overlay) {
        output_element = document.getElementById(ui_overlay_msg_id);
    }
    else if (output_channel === ui_message_output_channel.content_element) {
        output_element = document.getElementById(ui_main_content_section_id);
    }

    // Put message in paragraph for content section messages
    if (output_channel === ui_message_output_channel.content_element) {
        output_element = document.getElementById(ui_main_content_section_id);
        message = "<p id=\"cnt_msg\">" + message + "</p>";
    }

    // Get font color
    if (output_channel !== ui_message_output_channel.overlay ||
        output_channel !== ui_message_output_channel.content_element) {
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

        if (output_channel === ui_message_output_channel.overlay) {
            var overlay_element = document.getElementById(ui_overlay_container_id);
            overlay_element.style.display = "block";
        }
    }
}

// Helper functions
function ui_get_upload_prv_data_by_id(upload_id) {
    var uploads_count = active_upload_prv_objects.length;

    for (var i = 0; i < uploads_count; i++) {
        var current_upload_prv = active_upload_prv_objects[i];
        var current_upload_id = current_upload_prv.upload_id;

        if (current_upload_id === upload_id) {
            return current_upload_prv;
        }
    }

    console.error("Couldn't find wanted upload prv object in cache");

    return null;
}