// the flow is like this
// build pareses the json dataset then produce the sqlite DB
// the compile time , uses that db and embed it on the binary itself
// the runtime , uses that embeddings and write down temp file on it proper directories on the target os
// now the operations of database start using that temp file
// the temp file is automatically wiped each system reboot

use crate::core::types::{DATA_BASE_PATH, ErrorType};
use chrono::{DateTime, Utc};
use std::error::Error;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{BufWriter, Write};

pub fn embed() -> Result<(), ErrorType> {
    let bytes = include_bytes!("../../data/data.db");
    let mut writer = write_temp()?;
    writer
        .write_all(bytes)
        .map_err(ErrorType::WriteTmpFileFailed)?;
    writer.flush().map_err(ErrorType::FlushFailed)?;

    Ok(())
}
#[cfg(target_os = "linux")]
fn write_temp() -> Result<BufWriter<File>, ErrorType> {
    // every linux distro have /tmp directory
    let _ = create_dir_all("/tmp/awqat").map_err(ErrorType::CreateTmpDirFailed)?;
    let timestamp = Utc::now().timestamp();
    let filename = format!("/tmp/awqat/awqat_{}.db", timestamp);
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filename)
        .map_err(ErrorType::CreateTmpFileFailed)?;
    let writer = BufWriter::new(file);
    Ok(writer)
}
#[cfg(target_os = "windows")]
fn write_temp() -> Result<BufWriter<File>, ErrorType> {
    Ok(todo!())
}
