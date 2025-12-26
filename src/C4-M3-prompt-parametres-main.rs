#![allow(unused)]
use std::env;
use std::process;
#[derive(Debug)]
struct Settings {
    video_file:String,
    subtitles:bool,
    high_def:bool,
}
fn collect_settings() -> Settings{
    // target/debug/iterators  rust.mp4 false nonsense
    //                      -> rust.mp4 true nonsense nonsense
    //                      -> rust.mp4 abc def
    let mut args=env::args().skip(1).take(3);
    let video_file=args.next().unwrap_or_else(|| 
        {
            eprintln!("No video file specified!"); 
            process::exit(1);
        });

    let mut settings = args.map(|s| s.parse::<bool>().unwrap_or(false));
    let subtitles:bool=settings.next().unwrap_or(false);
    let high_definition:bool= settings.next().unwrap_or(false);
    Settings {
            video_file,
            subtitles,
            high_def:high_definition,
    }
}
fn main(){

    let settings:Settings=collect_settings();
    println!("{settings:?}");
    
    
}
    



