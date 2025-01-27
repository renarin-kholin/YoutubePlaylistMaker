use std::io::Write;

use std::path::Path;
use std::process::Command;
use std::time::Duration;
use std::{env, fs, path};
use std::{io, process::exit};

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let mut default_path = "input/input_terms.txt";
    if env::consts::OS.eq("winddows") {
        default_path = ".\\input\\input_terms.txt";
    }
    print!("Enter the filename to read the input terms from (relative to current working directory, press enter for default: {default_path}): ");
    let _ = io::stdout().flush();
    let mut input_buffer = String::new();
    let string_result = io::stdin().read_line(&mut input_buffer);
    if string_result.is_err() {
        println!("Error while reading the stdin....");
        exit(-1);
    }
    if input_buffer.eq("\n") {
        // let current_directory = String::from(
        //     std::env::current_dir()
        //         .expect("No current directory.")
        //         .to_str()
        //         .unwrap(),
        // );
        input_buffer.insert_str(0, &format!("./input/input_terms.txt"));
    }
    input_buffer.pop();
    println!("reading from: {input_buffer}");
    let file_path = fs::canonicalize(&input_buffer).expect("Error canonicalizing the path.");
    let file_contents = fs::read_to_string(file_path).expect("Could not read from the given file.");
    let input_terms: Vec<&str> = file_contents.split("\n").collect();
    let mut output_urls: Vec<String> = vec![];
    println!("Finding songs...");
    let bar = ProgressBar::new(
        input_terms
            .len()
            .try_into()
            .expect("Error converting usize to u64"),
    );
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap();
    bar.set_style(sty);
    bar.enable_steady_tick(Duration::from_secs(1));
    bar.println("Starting...");
    for term in input_terms {
        bar.inc(1);
        let output = Command::new("yt-dlp")
            .args(["-s", "--get-id", &format!("ytsearch:{term}")])
            .output()
            .unwrap();

        output_urls.push(String::from_utf8(output.stdout).unwrap());
    }
    bar.println("Done");
    bar.finish();
    println!("Generating playlist url...");

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
                if url.ends_with("\n") {
                    let mut url = url.clone();
                    let _ = url.split_off(url.len() - 1);
                    String::from(url)
                } else {
                    String::from(url.trim())
                }
            ));
        }
    }
    println!("{playlist_url}");
}
