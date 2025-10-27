# finalfusion
Classic Natural Language Processing for 4D

## Preamble

Context aware transformer-based LLMs are the state of art tool for text vectorisation. But primitive, static, context-unaware tools can still be useful in some cases. For instance, computational resources are limited on consumer PCs. 

This repository is primarily a research project to understand the evolution of natural language processing and AI. That said, it might be useful for basic semantic search where a LLM might be an overkill.

## Solutions

|library|good at finding|repository|model|
|:-:|-|:-:|:-:|
|[GloVe](https://nlp.stanford.edu/projects/glove/)|global co-occurrence patterns|active|[available](https://nlp.stanford.edu/projects/glove/)|
|[Word2Vec](https://code.google.com/archive/p/word2vec/)|semantic similarity|n/a|n/a|
|[FastText](https://fasttext.cc)|morphologically rich languages |n/a|[available](https://fasttext.cc/docs/en/crawl-vectors.html)|

[finalfusion](https://docs.rs/finalfusion/latest/finalfusion/) can handle all 3 models. But the `.fifu` pretrained models are [unavailable](https://finalfusion.github.io/pretrained).

## Models

* **Glove**: 2024 Wikipedia + Gigaword 5 (11.9B tokens, 1.2M vocab, uncased, 300d vectors, 1.6 GB download). `.fifu` format is available in [releases](https://github.com/miyako/finalfusion/releases/tag/glove.300d.fifu).

* **FastText**: Word vectors for English, trained on Common Crawl and Wikipedia using CBOW with position-weights, in dimension 300, with character n-grams of length 5, a window of size 5 and 10 negatives. `.fifu` format is available in [releases]

## Converter 

Rust code to convert GloVe model to finalfusion

```
cargo new finalfusion-conveter --bin
cargo build --release --target aarch64-apple-darwin
```

```toml
[package]
name = "finalfusion-conveter"
version = "0.1.0"
edition = "2024"

[dependencies]
finalfusion = "0.18"
anyhow = "1.0"
```

```go
use std::fs::File;
use std::io::BufReader;
use finalfusion::prelude::*;
use finalfusion::io::WriteEmbeddings;
use anyhow::Result;

fn main() -> Result<()> {
        
    let mut reader = BufReader::new(File::open("wiki_giga_2024_300_MFT20_vectors_seed_2024_alpha_0.75_eta_0.05_combined.txt").unwrap());

    /*
        .txt: word embeddings in text format.
        In this format, each line contains a word followed by its embedding.
        The word and the embedding vector components are separated by a space.
        This format is used by GloVe.
    */

    let embeddings = Embeddings::read_text(&mut reader).unwrap();

    /*
        .bin: word embeddings in fasttext format.
        This format is used by FastText.

        let embeddings = Embeddings::read_fasttext(&mut reader).unwrap();
    */

    let mut out_file = File::create("glove.300d.fifu")?;
    
    embeddings.write_embeddings(&mut out_file)?;

    Ok(())
}
```

