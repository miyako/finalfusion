# finalfusion
Classic Natural Language Processing for 4D

## Preamble

Context aware transformer-based LLMs are the state of art tool for text vectorisation. But primitive, static, context-unaware tools can still be useful in some cases. For instance, computational resources are limited on consumer PCs. Or, for basic semantic search, a LLM might be an overkill.

## Solutions

|library|good at finding|
|:-:|-|
|[GloVe](https://nlp.stanford.edu/projects/glove/)|global co-occurrence patterns|
|[Word2Vec](code.google.com/p/word2vec)|semantic similarity|
|[FastText](https://fasttext.cc)|morphologically rich languages |

[finalfusion](https://docs.rs/finalfusion/latest/finalfusion/) can handle all 3 models.

 
