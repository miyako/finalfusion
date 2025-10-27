use std::fs::File;
use std::io::BufReader;
use finalfusion::prelude::*;
use finalfusion::io::WriteEmbeddings;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
        
    let cli = Cli::parse();
    let input_path: &str = &cli.input; 
    let output_path: &str = &cli.output; 
        
    let path = std::path::Path::new(&input_path);    
    let input_ext = path
    .extension()
    .and_then(|e| e.to_str())
    .unwrap_or(""); 
    
    let mut out_file = File::create(output_path)?;
    let mut reader = BufReader::new(File::open(input_path).unwrap());

    match input_ext {
        "bin" => {
        // (FastText)
        let embeddings = Embeddings::read_fasttext(&mut reader).unwrap();
        embeddings.write_embeddings(&mut out_file)?;
        }
        "vec" => {
        //  (Word2Vec) 
            let embeddings = Embeddings::read_word2vec_binary(&mut reader).unwrap();
            embeddings.write_embeddings(&mut out_file)?;
        }
        _ => {
        // (GloVe) 
            let embeddings = Embeddings::read_text(&mut reader).unwrap();
            embeddings.write_embeddings(&mut out_file)?;
        }
    }   
    
    Ok(())
}
