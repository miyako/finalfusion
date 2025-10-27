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

## Model

2024 Wikipedia + Gigaword 5 (11.9B tokens, 1.2M vocab, uncased, 300d vectors, 1.6 GB download) from GloVe converted to `.fifu` format is available in [releases](https://github.com/miyako/finalfusion/releases/tag/glove.300d.fifu).
 
