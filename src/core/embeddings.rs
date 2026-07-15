// the flow is like this
// build pareses the json dataset then produce the sqlite DB
// the compile time , uses that db and embed it on the binary itself
// the runtime , uses that embeddings and write down temp file on it proper directories on the target os
// now the operations of database start using that temp file
// the temp file is automatically wiped each system reboot

use crate::core::types::{ ErrorType, APP_VERSION};
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{BufWriter, Write};
use std::path::{Path};
// WE COULD USE THE TEMPFILE CRATE AS OUR GO TO TEMP FILE MANAGEMENT TOOL, BUT OUT OF EXPERIENCE , WE TRIED TO "RE-INVENT THE WHEEL"
// the flow is like this
// if tmp file exist and app version is same with the one in the file name
// we use that file directly no io operations
// if file doesnt exist or version is different , we write new file
pub fn embed() -> Result<(), ErrorType> {
    let bytes = include_bytes!("../../data/data.db");

    let mut writer = if let Some(writer) = write_temp()? {
        writer
    } else {
        return Ok(());
    };
    writer
        .write_all(bytes)
        .map_err(ErrorType::WriteTmpFileFailed)?;
    writer.flush().map_err(ErrorType::FlushFailed)?;
    Ok(())
}
#[cfg(target_os = "linux")]
fn write_temp() -> Result<Option<BufWriter<File>>, ErrorType> {
    let base_path = Path::new("/tmp/awqat");
    let mut file_path = base_path.to_path_buf();
    file_path.push(format!("awqat_{}.db", APP_VERSION));
    if file_path.exists() {
        return Ok(None);
    }
    // every Linux distro have /tmp directory
    let _ = create_dir_all(base_path).map_err(ErrorType::CreateTmpDirFailed)?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file_path)
        .map_err(ErrorType::CreateTmpFileFailed)?;
    let writer = BufWriter::new(file);
    Ok(Some(writer))
}
#[cfg(target_os = "windows")]
fn write_temp() -> Result<BufWriter<File>, ErrorType> {
    Ok(todo!())
}
