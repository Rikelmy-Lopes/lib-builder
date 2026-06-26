use rfd::AsyncFileDialog;

pub async fn open_dialog_choose_folder() -> String {
    let file_dialog = AsyncFileDialog::new();

    let file_handle = file_dialog.pick_folder().await;

    if let Some(path) = file_handle {
        path.path().display().to_string()
    } else {
        String::new()
    }
}
