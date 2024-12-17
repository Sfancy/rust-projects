use clap::Parser;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Parser)]
#[command(name = "cutv")]
struct Cli {
    /// Path to the video file
    #[arg(short, long, value_name = "FILE")]
    video: String,

    /// Path to the timestamps file or inline timestamps
    #[arg(short, long, default_value = "timestamps.txt")]
    timestamps: String,

    /// Optional duration
    #[arg(short, long, default_value_t = 3)]
    duration: u8,
}

fn create_folder(video: &str) -> String {
    let video_name = Path::new(video)
        .file_stem()
        .expect("Invalid video file name")
        .to_string_lossy();
    let folder_name = format!("{}", video_name);
    fs::create_dir_all(&folder_name).expect("Failed to create folder");
    folder_name.to_string()
}

fn parse_timestamps(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = if input.ends_with(".txt") {
        fs::read_to_string(input)?
    } else {
        input.to_string()
    };

    let lines: Vec<String> = contents
        .lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if lines.is_empty() {
        return Err("Empty timestamps".into());
    }
    Ok(lines)
}

fn to_h264_clip(video: &str, timestamp: &str, duration: u8, output: &str) {
    // Execute the ffmpeg command
    let ffmpeg_command = format!(
        "ffmpeg -ss {} -t {} -i {} -c:v libx264 -c:a aac -strict experimental {} -y ",
        timestamp, duration, video, output
    );
    println!("Executing: {}", ffmpeg_command); // Debug print
                                               // Uncomment this to actually execute the command
    std::process::Command::new("sh")
        .arg("-c")
        .arg(ffmpeg_command)
        .spawn()
        .expect("Failed to execute ffmpeg command");
}

fn execute_all_timestamps(
    video: &str,
    timestamps: Vec<String>,
    duration: u8,
) -> Result<(), Box<dyn Error>> {
    let folder_name = create_folder(video);
    for (index, timestamp) in timestamps.iter().enumerate() {
        let output_name = format!(
            "{}/{}-{}.mp4",
            folder_name,
            Path::new(video).file_stem().unwrap().to_string_lossy(),
            index + 1
        );

        to_h264_clip(video, timestamp, duration, &output_name);
    }
    Ok(())
}

fn run(config: &Cli) -> Result<(), Box<dyn Error>> {
    if config.video.is_empty() {
        return Err("No video file specified".into());
    }
    if !Path::new(&config.video).exists() {
        return Err(format!("Video {} not exist", config.video).into());
    }

    let timestamps = match parse_timestamps(&config.timestamps) {
        Ok(d) => d,
        Err(error) => {
            if let Some(io_error) = error.downcast_ref::<io::Error>() {
                // Handle the I/O error here
                match io_error.kind() {
                    io::ErrorKind::NotFound => {
                        return Err(format!("File {} not found", config.timestamps).into());
                    }
                    _ => {
                        return Err(format!("File {} is empty", config.timestamps).into());
                    }
                }
            } else {
                return Err(error);
            }
        }
    };

    let _ = execute_all_timestamps(&config.video, timestamps, config.duration);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(&cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
