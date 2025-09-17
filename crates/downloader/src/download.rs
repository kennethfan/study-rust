use std::fs::File;
use std::io::copy;
use std::path::Path;

pub fn blocking_download(url: &str) -> Result<(), String> {
    let resp = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    let filename = Path::new(url)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let mut out = File::create(filename.clone()).map_err(|e| e.to_string())?;
    let mut content = resp.bytes().map_err(|e| e.to_string())?.as_ref();
    copy(&mut content, &mut out).map_err(|e| e.to_string())?;
    Ok(())
}

