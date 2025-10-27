# finalfusion
Classic Natural Language Processing for 4D

## Preamble

Context aware transformer-based LLMs are the state of art tool for text vectorisation. But primitive, static, context-unaware tools can still be useful in some cases. For instance, computational resources are limited on consumer PCs. Or, for basic semantic search, a LLM might be an overkill.

## Solutions

* Word2Vec

Source code repository is no longer maintained. Model download links are inactive.

* GloVe (Global Vectors for Word Representation)

Source code repository is well maintained. Model download links are active.


|library|good at finding|
|:-:|-|
|[GloVe](https://nlp.stanford.edu/projects/glove/)|global co-occurrence patterns|
|[Word2Vec](https://code.google.com/archive/p/word2vec/)|semantic similarity|
|[FastText](https://fasttext.cc)|morphologically rich languages |

[finalfusion](https://docs.rs/finalfusion/latest/finalfusion/) can handle all 3 models.

But the `.fifu` pretrained models are [unavailable](https://finalfusion.github.io/pretrained).

## Model

2024 Wikipedia + Gigaword 5 (11.9B tokens, 1.2M vocab, uncased, 300d vectors, 1.6 GB download) from GloVe converted to `.fifu` format is available in releases.
 
