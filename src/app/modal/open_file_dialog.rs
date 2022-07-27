use crate::app;
use rfd::FileHandle;
use tokio::sync::oneshot;

pub fn open_file_dialog() -> oneshot::Receiver<Option<FileHandle>> {
    let task = rfd::AsyncFileDialog::new()
        .add_filter("Image files", &["png", "jpg", "jpeg"])
        .set_directory("/")
        .pick_file();

    let (sender, receiver) = oneshot::channel();

    app::execute(async move {
        let file = task.await;
        sender.send(file).ok();
    });

    receiver
}
