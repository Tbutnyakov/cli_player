

use std::{error::Error, fs::{self, File}, io::{self, BufReader, Write}, path::{Path, PathBuf}};
use rodio::{Decoder, Player};
use clap::{Parser};
use colored::Colorize;

const AUDIO_EXTENSIONS: [&str; 8] = [
    "mp3", "wav", "flac", "m4a", 
    "aac", "ogg", "wma", "aiff"
];

struct CliPlayer {
   sink: rodio::MixerDeviceSink,
   player: Player,
   is_playing: bool,
   current_dir: String,
   volume: f32,
   files: Vec<PathBuf>,
   current_track_i: usize,
}


#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgramParams {
    #[arg(short, long, default_value = ".")]
    dir: String,
}



fn is_audio_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        AUDIO_EXTENSIONS.contains(
            &ext.to_string_lossy().to_lowercase().as_str()
        )
    } else {
        false
    }
}

fn pretty_line() {
    println!("==============================");
}



fn scan_directory(dir: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut res: Vec<PathBuf> = vec![];
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
    
        if is_audio_file(&entry.path())  {
           res.push(path);
        }
    }

    return  Ok(res);
}


fn read_user_input() -> String {
    let mut input_string = String::new();

    println!("Type your command:");
    
    io::stdout().flush().unwrap_or_default();
    io::stdin().read_line(&mut input_string).unwrap_or_default();
    
    input_string.trim().to_string()
}

fn beautify_track_string(path: &PathBuf) -> String {
    path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

impl  CliPlayer {
    pub fn new() -> Self {
        let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = rodio::Player::connect_new(&sink.mixer());

        Self { 
             sink,
            player,
            is_playing: false,
            current_dir: String::from("."),
            current_track_i: 0,
            files: vec![],
            volume: 1.0,
        }
    }

    pub fn run(&mut self, params: &ProgramParams) -> Result<(), Box<dyn Error>> {
        pretty_line();
        println!("program params {:#?}", params);
        pretty_line();

        self.set_directory(&params.dir);

        loop {
            self.print_status();
            self.listen_commands();
        }
    }

    fn print_help(&mut self) {
        println!("\n🎼 CLI Audio Player — Commands");
        println!("-------------------------");
        println!("▸ play/stop/pause/unpause — control playback");
        println!("▸ next/prev — next/previous track");
        println!("▸ list — show all tracks");
        println!("▸ track <N> — play track #N (0,1,2...)");
        println!("▸ volume <V> — set volume (0.0–1.0)");
        println!("▸ help — this message");
        println!("▸ quit — exit");
        println!("-------------------------");
        println!("Dir: {}, Tracks: {}", self.current_dir, self.files.len());
    }
  

    fn print_status(&mut self) {
        let current_track = match self.is_playing {
            true => beautify_track_string(&self.get_current_track_path()),
            _ => String::from("-"),
        };

        let status_text = if self.is_playing {
            "Now playing".green().bold()
        } else {
            "Nothing playing".red().bold()
        };
        
        pretty_line();
        println!("{} {:?}.", status_text, current_track);
        println!("Volume: {} %", self.volume * 100.0);
        pretty_line();
    }
    


    fn listen_commands(&mut self) {
        
        let input_line = read_user_input();
        let tokens: Vec<&str> = input_line.split_whitespace().collect();

        if tokens.is_empty() {
            return;
        }

        let first_argument = tokens[0].to_lowercase();

        match first_argument.as_str() {
            "play" => {
                let _ = self.play();
            },
            "stop" => self.stop(),
            "pause" => self.pause(),
            "unpause" => self.unpause(),
            "next" => self.next_track(),
            "prev" => self.prev_track(),
            "list" => self.list_directory(),
            "track"=> {
                if tokens.len() < 2 {
                    return;
                }

                let track_i = tokens[1].parse::<usize>().unwrap_or(self.current_track_i);

                self.play_track_by_index(track_i);
            },
            "volume" => {
                if tokens.len() < 2 {
                    return;
                }

                let volume = tokens[1].parse::<f32>().unwrap_or(self.volume);
                self.set_volume(volume);
            }
            "help" => self.print_help(),
            _ => println!("Unknown command. Type \"help\" for help")
        }
    }

    fn set_directory(&mut self, directory: &str) {
        self.current_dir = directory.to_string();

        match scan_directory(& self.current_dir)  {
            Ok(files) => {
                self.files.clear();
                self.files = files;
            },
            Err(_) => self.files.clear()
        };

        self.current_track_i = 0;
    }


    fn get_current_track_path(&mut self) -> PathBuf {
        self.files[self.current_track_i].clone()
    }

    
    fn play(&mut self) -> Result<(), Box<dyn Error>> {
        self.stop();

        let file = File::open(self.get_current_track_path())?;
        let reader = BufReader::new(file);
        let source = Decoder::try_from(reader)?;

        self.player.set_volume(self.volume);
        self.player.append(source);
        self.is_playing = true;

        Ok(())
    }


    fn list_directory(&mut self) {
       for (index, file) in self.files.iter().enumerate()  {
            let text: String = beautify_track_string(file);
            
            if self.is_playing && self.current_track_i == index {
                println!("{}", text.green().bold())
            } else {
                println!("{}", text);
            }
            
        }
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        self.player.set_volume(volume);
    }

    fn pause(&mut self) {
        self.is_playing = false;
        self.player.pause();
    }

    fn unpause(&mut self) {
        self.is_playing = true;
        self.player.play();
    }

    fn stop(&mut self) {
        self.is_playing = false;
        self.player.stop();
        
    }

    fn play_track_by_index(&mut self, track_i: usize) {
       
        self.set_track_index(track_i); 
        let _ = self.play();
    }

    fn next_track(&mut self) {
        self.play_track_by_index(self.current_track_i + 1); 
    }

    fn prev_track(&mut self) {
        self.play_track_by_index(self.current_track_i - 1); 
    }

    fn set_track_index(&mut self, track_index: usize) {
        let track_count = self.files.len();
        self.current_track_i = track_index.clamp(0, track_count - 1);
    }


}




pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let program_params = ProgramParams::parse();
    let mut app = CliPlayer::new();

    let _ = app.run(&program_params);

    Ok(())
}