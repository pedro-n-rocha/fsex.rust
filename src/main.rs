use std::fs::File;
use std::fs;
use flate2::read::GzDecoder;
use tar::Archive;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct ManifestEntry {
    Config: String,
    RepoTags: Vec<String>,
    Layers: Vec<String>,
}

#[derive(Parser)]
struct Cli {
    // input image tar file
    input:  String,
    // output directory
    #[arg(short, long)]
    output: String,
}

fn main() ->  Result<(), std::io::Error> {

    let args = Cli::parse();
    
    let input = args.input ; 
    let out   = args.output ; 
    let tmp   = ".tmp";
    let man   = format!("{}/manifest.json",tmp);
    let tar   = File::open(input)?;
    let mut archive = Archive::new(tar);
    archive.unpack(tmp)?;

    let data = fs::read_to_string(man)
        .expect("Unable to read file");

    let manifest: Vec<ManifestEntry> = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");
  
    for i in manifest.iter(){
      for i in i.Layers.iter() {
        let f = format!("{}/{}" , tmp , i) ;  
        let tgz = File::open(f)?;
        let tar = GzDecoder::new(tgz);
        let mut arch = Archive::new(tar);
        arch.unpack(out.clone()).unwrap();
      }
    }

    fs::remove_dir_all(tmp)?;
    Ok(())
}



fn original() ->  Result<(), std::io::Error> {

    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}
    
