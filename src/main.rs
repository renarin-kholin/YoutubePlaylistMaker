use std::io::Write;

use std::process::Command;
use std::{fs, path};
use std::{io, process::exit};

#[tokio::main]
async fn main() {
    print!("Enter the filename to read the input terms from (relative to current working directory, press enter for default: input/input_terms.txt): ");
    let _ = io::stdout().flush();
    let mut input_buffer = String::new();
    let string_result = io::stdin().read_line(&mut input_buffer);
    if string_result.is_err() {
        println!("Error while reading the stdin....");
        exit(-1);
    }
    if input_buffer.eq("\n") {
        let current_directory = String::from(
            std::env::current_dir()
                .expect("No current directory.")
                .to_str()
                .unwrap(),
        );
        input_buffer.insert_str(0, &format!("{current_directory}/input/input_terms.txt"));
    }
    println!("reading from: {input_buffer}");
    let file_path =
        path::absolute("/home/renarin/Code/Throwaway/ytplaylistmaker/input/input_terms.txt")
            .unwrap();
    let file_contents = fs::read_to_string(file_path).expect("Could not read from the given file.");
    let input_terms: Vec<&str> = file_contents.split("\n").collect();
    let mut output_urls: Vec<String> = vec![];
    for term in input_terms {
        let output = Command::new("yt-dlp")
            .args(["-s", "--get-id", &format!("ytsearch:{term}")])
            .output()
            .unwrap();

        output_urls.push(String::from_utf8(output.stdout).unwrap());
    }

    let mut playlist_url = String::from("https://www.youtube.com/watch_videos?video_ids=");
    for (index, url) in output_urls.iter().enumerate() {
        if url.len() > 0 {
            playlist_url.push_str(&format!(
                "{}{}",
                if index > 0 {
                    String::from(",")
                } else {
                    String::new()
                },
                url.trim().split_once("\n").unwrap().0
            ));
        }
    }
    println!("{playlist_url}");
}
