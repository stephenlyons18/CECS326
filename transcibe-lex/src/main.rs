let id = Id::from_raw("https://www.youtube.com/playlist?list=PLrAXtmErZgOdP_8GztsuKi9nrraNbKKp4")?;

// get all the videos in the pllaylist and save their titles and llinks in a file
let mut file = File::create("episodes.txt")?;

// save all the flies into a tex file in the current directory 

for video in playlist.videos( ) ? { 
    file.write_all(format! )
}
fn main() {
    println!("Hello, world!");
}
