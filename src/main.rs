use std::{process::{Command, Stdio}, path::Path};

#[derive(Debug)]
pub struct Account {
    pub password: String,

    pub username: Option<String>,

    pub email: Option<String>,

    pub service: Option<String>,
}

pub fn decr(filepath: String) -> Result<String, String> {
    match Command::new("gpg")
        // decrypt file quietly 
        .args(["-dq", &format!("{}", filepath)])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("There was an error while calling gpg")
        .wait_with_output()
    {
        Ok(command) => return Ok(String::from_utf8(command.stdout).unwrap()),
        Err(err) => return Err(err.to_string()),
    }
}

pub fn parse_pass_file(file_content: String, filename: String) -> Account {
    let mut acc = Account{
        password: String::from(""),
        username: None,
        email: None,
        service: None,
    };
    let lines: Vec<&str> = file_content.split("\n").collect();
    acc.password = lines[0].to_owned();
    acc.service = Some(get_service_from_filename(filename));
    for line in &lines[1..] {
        if line.starts_with("username: ") {
            acc.username = Some(line.strip_prefix("username: ").unwrap().to_string());
        }
        if line.starts_with("email: ") {
            acc.email = Some(line.strip_prefix("email: ").unwrap().to_string());
        }
    }

    return acc
}

pub fn get_service_from_filename(filepath: String) -> String {
    let filename = Path::new(&filepath).file_name().unwrap();
    if filename.to_string_lossy().into_owned().strip_suffix(".asc").unwrap().contains(".") {
        return filename.to_string_lossy().into_owned().strip_suffix(".asc").unwrap().to_string();
    }
    return Path::new(&filepath).parent().unwrap().to_string_lossy().into_owned();
}

fn main() -> Result<(), String> {
    let output = decr("test.asc".to_string())?;
    let acc = parse_pass_file(output, "service.com.asc".to_string());
    println!("{:?}", acc);
    Ok(())
}
