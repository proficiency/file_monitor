use std::path::*;
use tokio::io::AsyncWriteExt;

pub(crate) async fn run_tests() {
    let dir = PathBuf::from("inbox");
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir)
            .await
            .expect("failed to create inbox/test directory");
    }

    let mut to_remove = Vec::new();
    for n in 0..10 {
        let path = dir.join(format!("file{n}.txt"));

        async_create_file(&path).await;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        async_modify_file(&path).await;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        to_remove.push(path);
    }

    for (index, file) in to_remove.iter().enumerate() {
        // leave a few files on disk to test the dir cache
        if index >= 7 {
            break;
        }

        async_remove_file(file).await;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

async fn async_create_file(file_path: &Path) {
    tokio::fs::File::create(file_path)
        .await
        .expect("failed to create file");
}

async fn async_modify_file(file_path: &Path) {
    let mut file = tokio::fs::File::create(file_path)
        .await
        .expect("failed to modify file");

    file.write_all(b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")
        .await
        .expect("failed to write to file");
}

async fn async_remove_file(file_path: &Path) {
    if file_path.exists() {
        tokio::fs::remove_file(file_path)
            .await
            .expect("failed to remove file");
    }
}
