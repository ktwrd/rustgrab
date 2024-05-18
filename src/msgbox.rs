use native_dialog::{MessageDialog, MessageType};
pub fn error(code: usize) {
    message_dialog(
        crate::locale::error(code),
        "Error".to_string(),
        MessageType::Error);
}
pub fn error_msg(code: usize, msg: String) {
    let content = crate::locale::error(code)
        .replace("%s", msg.as_str());
    message_dialog(
        content,
        "Error".to_string(),
        MessageType::Error);
}

pub fn error_custom(text: String, title: String) {
    message_dialog(text, title, MessageType::Error);
}
pub fn message_dialog(text: String, title: String, msg_type: MessageType) {
    let t = text.replace("\n", "\\n");
    if let Err(e) = MessageDialog::new()
        .set_type(msg_type)
        .set_title(title.as_str())
        .set_text(t.as_str())
        .show_alert() {
        eprintln!("[msgbox.message_dialog] failed to show dialog: {:#?}", e);
        eprintln!("[msgbox.message_dialog] failed to show dialog\ntext: {}\ntitle: {}\ntype: {}",
            text,
            title,
            messagetype_as_str(msg_type));
    }
}
// why the FUCK is Debug not implemented on MessageType????
fn messagetype_as_str(v: MessageType) -> String {
    match v {
        MessageType::Info => "Info",
        MessageType::Warning => "Warning",
        MessageType::Error => "Error"
    }.to_string()
}