use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use encoding_rs;
use chardetng::EncodingDetector;
use chrono::Local;

pub fn read_file_with_encoding(path: &Path) -> Result<(String, &'static encoding_rs::Encoding)> {
    let bytes = fs::read(path).context("读取文件失败")?;
    
    let mut detector = EncodingDetector::new();
    detector.feed(&bytes, true);
    let encoding = detector.guess(None, true);
    
    let (text, _, _) = encoding.decode(&bytes);
    Ok((text.into_owned(), encoding))
}

pub fn write_file_with_encoding(path: &Path, content: &str, encoding: &'static encoding_rs::Encoding) -> Result<()> {
    let (bytes, _, _) = encoding.encode(content);
    fs::write(path, bytes).context("写入文件失败")?;
    Ok(())
}

pub fn get_converted_file_path(original_path: &Path, converted_name: &str, is_new_mode: bool) -> PathBuf {
    let timestamp = if is_new_mode {
        format!("_{}", Local::now().format("%Y%m%d%H%M%S"))
    } else {
        String::new()
    };
    
    let extension = original_path.extension().unwrap_or_default().to_string_lossy();
    
    let new_file_name = if extension.is_empty() {
        format!("{}{}", converted_name, timestamp)
    } else {
        format!("{}{}.{}", converted_name, timestamp, extension)
    };
    
    original_path.with_file_name(new_file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converted_file_path() {
        let path = Path::new("繁体.txt");
        let converted_stem = "简体";
        
        // Test Replace mode (no timestamp)
        let replace_path = get_converted_file_path(path, converted_stem, false);
        assert_eq!(replace_path.file_name().unwrap().to_str().unwrap(), "简体.txt");
        
        // Test New mode (with timestamp)
        let new_path = get_converted_file_path(path, converted_stem, true);
        let new_name = new_path.file_name().unwrap().to_str().unwrap();
        assert!(new_name.starts_with("简体_"));
        assert!(new_name.ends_with(".txt"));
    }
}
