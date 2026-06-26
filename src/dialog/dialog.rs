use rfd::{AsyncFileDialog, AsyncMessageDialog, MessageButtons, MessageDialogResult, MessageLevel};

pub async fn open_dialog_choose_folder() -> String {
    let file_dialog = AsyncFileDialog::new();

    let file_handle = file_dialog.pick_folder().await;

    if let Some(path) = file_handle {
        path.path().display().to_string()
    } else {
        String::new()
    }
}

fn create_basic_dialog(description: &str) -> AsyncMessageDialog {
    let mut dialog = AsyncMessageDialog::new();
    dialog = dialog.set_title("Lib Builder");
    dialog = dialog.set_description(description);

    dialog
}

pub async fn show_info_message(description: &str) {
    let mut dialog = create_basic_dialog(description);
    dialog = dialog.set_level(MessageLevel::Info);

    dialog.show().await;
}

pub async fn show_warning_message(description: &str) {
    let mut dialog = create_basic_dialog(description);
    dialog = dialog.set_level(MessageLevel::Warning);

    dialog.show().await;
}

pub async fn show_error_message(description: &str) {
    let mut dialog = create_basic_dialog(description);
    dialog = dialog.set_level(MessageLevel::Error);

    dialog.show().await;
}

pub async fn show_yes_no_dialog(description: &str) -> bool {
    let mut dialog = create_basic_dialog(description);
    dialog = dialog.set_level(MessageLevel::Warning);
    dialog = dialog.set_buttons(MessageButtons::YesNo);

    let result = dialog.show().await;

    match result {
        MessageDialogResult::Yes => true,
        MessageDialogResult::No => false,
        _ => false,
    }
}
