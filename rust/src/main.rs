use std::env;
use std::fs::File;
use std::io::Read;
use zip::read::ZipArchive;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filepath as an argument");
        return;
    }

    let filepath = &args[1];

    let file = File::open(filepath).expect("Failed to open file");

    let mut archive = ZipArchive::new(file).expect("Failed to open zip archive");

    // Read files from the zip archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to read file from zip");
        let file_name = file.name().to_owned();
        if file_name == "assets/myhealth.bundle" {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).expect("Failed to read file contents");
            let contents_str = String::from_utf8_lossy(&contents);

            let version_pattern = r"2\.3\.(\d+)";
            if let Some(captures) = regex::Regex::new(version_pattern)
                .unwrap()
                .captures(&contents_str)
            {
                let version_number = captures.get(1).unwrap().as_str();
                println!("Version number: {}", version_number);
            }
        }
    }
}
