# finalfusion
Classic Natural Language Processing for 4D

## Preamble

Before transformer-based LLMs, there were static linear embeddings; `GloVe` `Word2Vec` `FastText` being the most popular.

This repository is primarily a research project to learn the evolution of natural language processing technology and the path to modern AI. Compared to LLMs, early models have notable limitations:

* Language specific
* Large in size
* Contaxt agnostic

On the other hand they might still be useful for basic semantic searches where an LLM would be an overkill.

## Solutions

|library|good at finding|repository|original model|
|:-:|-|:-:|:-:|
|[GloVe](https://nlp.stanford.edu/projects/glove/)|global co-occurrence patterns|active|[available](https://nlp.stanford.edu/projects/glove/)|
|[Word2Vec](https://code.google.com/archive/p/word2vec/)|semantic similarity|n/a|n/a|
|[FastText](https://fasttext.cc)|morphologically rich languages |n/a|[available](https://fasttext.cc/docs/en/crawl-vectors.html)|

[finalfusion](https://docs.rs/finalfusion/latest/finalfusion/) can handle all 3 model formats. But the `.fifu` pretrained models are [unavailable](https://finalfusion.github.io/pretrained). Models can be [converted](https://docs.rs/finalfusion/latest/finalfusion/compat/index.html) to `.fifu`.

## Models

|library|description|dimensions|size|download|language|
|:-:|-|:-:|:-:|:-:|:-:|
|FastText|Common Crawl and Wikipedia|300|4.83 GB|[Google Drive](https://drive.google.com/file/d/1z0sNhxmTlsMLi091aQDKCRIz4BGB12CY/view?usp=sharing)|English|
|FastText|Common Crawl and Wikipedia|300|||French|
|FastText|Common Crawl and Wikipedia|300|||German|
|~~FastText~~|~~Common Crawl and Wikipedia~~|~~300~~|||~~Japanese~~|
|~~Word2Vec~~|~~Google News dataset~~|~~300~~|||~~English~~|
|GloVe|2024 Wikipedia + Gigaword 5|300|1.57 GB|[releases](https://github.com/miyako/finalfusion/releases/tag/glove.300d.fifu)|English|
|GloVe|2024 Wikipedia + Gigaword 5|200|1.06 GB|[releases](https://github.com/miyako/finalfusion/releases/tag/glove.200d.fifu)|English|
|GloVe|2024 Wikipedia + Gigaword 5|100|537.5 MB|[releases](https://github.com/miyako/finalfusion/releases/tag/glove.100d.fifu)|English|
|GloVe|2024 Wikipedia + Gigaword 5|50|285.7 MB|[releases](https://github.com/miyako/finalfusion/releases/tag/glove.50d.fifu)|English|

> [!WARNING]
> `Embeddings::read_fasttext` fails on Japanese model.  
> ```
> called `Result::unwrap()` on an `Err` value:`
> Format("Token contains invalid UTF-8: incomplete utf-8 byte sequence from index 0")
> ```

> [!WARNING]
> `Embeddings::read_word2vec_binary` fails on Google Word2Vec model.  
> ```
> assertion `left == right` failed: words contained duplicate entries.
>  left: 3000000
> right: 2999997
> ```
 
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

## Server

A simple HTTP server that converts text to vector. Words are separated by word boundaries. Every word is represented by a vector each. The mean pooling vector is returned in `.aggregate`.

### Usage

```
finalfusion-server --model <MODEL>
```

Pass a `.fifu` (see above) model file path. Optionally pass `--port n (default=8080)`.

### Example

```
finalfusion-server --model glove.50d.fifu
```

```4d
var $response : Text
var $data : Object
var $embedding1; $embedding2 : 4D.Vector
```

```4d
$request:=Lowercase("The quick brown fox jumps over the lazy dog")
$status:=HTTP Request(HTTP POST method; "http://127.0.0.1:8080/embeddings"; $request; $response)
If ($status=200)
	$data:=JSON Parse($response; Is object)
	$embedding1:=4D.Vector.new($data.aggregate)
End if
```

```json
{"aggregate":[-0.067359634,-0.06297489,0.0064072222,-0.022331553,-0.036555834,0.0070329043,-0.0077260607,0.0337032,-0.023793846,-0.07052101,-0.06342882,0.0104390085,0.074622944,-0.0029501277,-0.01684898,0.07893474,-0.02623674,-0.1157724,0.018254098,0.08287585,-0.05488345,0.01548778,0.038043603,-0.019966552,-0.036003098,-0.005867314,0.008692934,0.0039872355,0.015800161,0.033145465,-0.106028035,-0.00080079836,-0.06922984,-0.020627433,-0.0031583777,0.0020935552,-0.03582899,0.6958763,0.0032898108,-0.039791472,0.0061395625,0.06452158,0.02552054,0.034229327,-0.0023772568,0.04238738,0.006902876,0.016086826,-0.018333975,0.047030102],"words":[{"word":"the","vector":[-0.060212653,-0.075490244,-0.04305407,0.020840172,0.010190242,-0.014525472,-0.000469739,0.026528709,0.06716906,0.0050353636,0.09433288,0.00071536645,0.034797437,-0.016775697,0.05339021,-0.038974516,-0.010292169,0.025918253,-0.015534682,-0.041941993,-0.05522095,-0.09357653,-0.11752552,-0.04117731,-0.06284907,0.048132464,0.02080452,0.023484124,-0.0727117,0.06853791,-0.05157643,-0.017636335,-0.049966972,-0.092059575,-0.037966404,-0.060128316,-0.06812645,0.9352698,0.021553496,-0.01013229,0.07169715,-0.0030215005,0.04093121,-0.046106506,0.031249186,0.043605633,0.025843026,0.063839585,-0.03579298,-0.06286995]},{"word":"quick","vector":[-0.047973502,-0.10644228,0.0023346478,0.03907835,-0.08562657,-0.1516965,-0.10135504,-0.05884156,-0.066502735,-0.10302321,-0.14363846,-0.051273324,0.1609785,-0.03666742,0.058390692,0.09040046,0.0034080236,0.053134017,-0.02488036,0.14013909,-0.11778249,-0.029735345,-0.0045701284,0.08042009,0.08364523,-0.10470003,-0.071404696,0.08101851,0.3046559,-0.078451596,-0.0959468,0.095956236,-0.017807964,-0.0034405445,0.04275477,-0.057890035,-0.06613719,0.72103584,0.017114393,0.0071205758,-0.14974207,0.2767108,0.07755929,0.09659541,-0.043527834,0.12676275,-0.026017573,-0.054092765,-0.03435847,0.07123005]},{"word":"brown","vector":[-0.0026324035,-0.008062429,0.035941947,0.07839313,0.13969862,0.16026166,-0.054713476,0.096537605,-0.02869945,-0.028882459,-0.14248462,0.11539755,0.17899919,0.07381368,-0.12999547,0.04067922,-0.01573418,-0.22855267,0.045076188,0.08945434,-0.016677056,0.18322654,0.15761817,0.06646814,0.08309429,0.03628102,0.21422145,0.00932623,-0.10369133,-0.040480707,-0.09641959,0.00092666934,0.031689417,0.06895892,-0.049987834,-0.017344447,-0.001689353,0.7273962,0.015199597,0.034234982,0.19765289,-0.0064637875,-0.05632128,-0.010137527,0.012478068,0.17355683,-0.14371215,-0.050805997,-0.07681404,-0.035730936]},{"word":"fox","vector":[0.016874805,0.03315313,0.24078056,-0.119947754,-0.017207716,0.0000043337427,-0.04672602,0.13503213,-0.07947296,0.08469473,-0.07560017,0.045468643,-0.013466909,0.05226927,0.12655808,0.13329035,0.010556406,-0.39058033,0.10400332,0.081023455,0.023276728,0.07598745,0.08456629,0.04864311,-0.114845365,0.030300938,0.13616599,-0.021873975,-0.055401385,-0.009411313,0.019002477,0.11900575,-0.14413555,0.110182844,-0.09120853,0.013922148,0.015269351,0.6824815,0.10246405,0.039241843,0.050233986,0.036278546,0.036932748,-0.18537997,0.051892035,0.07490637,0.08858564,-0.07324222,0.13133565,0.114317626]},{"word":"jumps","vector":[-0.12520967,-0.08384987,-0.107486255,-0.12476778,-0.22705416,0.012821685,-0.029784096,-0.1207304,-0.21392994,-0.13180526,-0.06304376,-0.0033516716,0.18552409,-0.11775946,-0.18837357,0.16709752,0.026947863,-0.09774439,0.114111416,0.26865977,-0.08797602,-0.18113425,0.107569516,0.05113414,-0.13927756,0.036385857,-0.022637088,-0.03926537,0.10349415,0.1948384,-0.14728673,-0.03890452,-0.20038168,-0.105417,0.07839802,0.022196073,-0.028983558,0.39137596,-0.08947199,-0.19740236,-0.24033453,0.08397796,0.04226459,0.19277179,0.16798684,-0.047850706,0.10390756,0.14736666,-0.18107684,0.02893983]},{"word":"over","vector":[0.049677342,-0.087517075,0.034427773,-0.047040466,-0.04989129,-0.015196866,-0.031585842,-0.017505655,-0.006512307,-0.103238516,0.02894485,0.08342671,0.062158935,-0.0056255525,0.022454284,-0.0831202,0.06205682,0.024084007,0.0013054261,0.050387867,0.07173354,-0.060930427,0.06727441,-0.14875138,0.051532872,0.013731185,0.050891355,0.0024708556,-0.089088336,0.0038334331,-0.030816356,-0.051424827,-0.08281072,-0.027091132,0.020925686,-0.11053398,0.02667839,0.9066342,-0.051760983,-0.10766323,-0.03509794,0.047200065,0.03842739,0.07637912,-0.059282254,-0.0075495983,-0.107052356,-0.03332345,0.019561462,-0.093086794]},{"word":"the","vector":[-0.060212653,-0.075490244,-0.04305407,0.020840172,0.010190242,-0.014525472,-0.000469739,0.026528709,0.06716906,0.0050353636,0.09433288,0.00071536645,0.034797437,-0.016775697,0.05339021,-0.038974516,-0.010292169,0.025918253,-0.015534682,-0.041941993,-0.05522095,-0.09357653,-0.11752552,-0.04117731,-0.06284907,0.048132464,0.02080452,0.023484124,-0.0727117,0.06853791,-0.05157643,-0.017636335,-0.049966972,-0.092059575,-0.037966404,-0.060128316,-0.06812645,0.9352698,0.021553496,-0.01013229,0.07169715,-0.0030215005,0.04093121,-0.046106506,0.031249186,0.043605633,0.025843026,0.063839585,-0.03579298,-0.06286995]},{"word":"lazy","vector":[-0.20961645,-0.07964188,-0.07082166,0.07189842,-0.21677548,0.018433526,0.09210693,0.039160226,-0.03015281,-0.16264607,-0.19979657,-0.0853516,-0.053012952,-0.061521567,0.015939778,0.32556704,-0.21985066,-0.3163214,-0.06597588,0.17840767,-0.054358386,0.23239535,0.19472598,-0.10000065,0.011864468,-0.08913147,-0.14902993,-0.015465911,0.041526563,0.062503,-0.23698908,0.078567885,-0.06581246,-0.021312973,0.073399425,0.09823305,-0.06367555,0.35607624,-0.015179004,0.02355213,-0.008109533,0.067178436,0.043747254,0.23260263,-0.14689764,-0.05168783,0.07570552,-0.017663058,0.013990064,0.25352988]},{"word":"dog","vector":[-0.16693148,-0.08343312,0.008596134,-0.1402782,0.10747362,0.06771924,0.10346245,0.17661905,0.076787435,-0.19985901,-0.16390641,-0.01179595,0.08083075,0.1024913,-0.16339503,0.11444725,-0.08293061,-0.13780728,0.021716151,0.021694403,-0.20172548,0.10673376,-0.029740749,-0.09525781,-0.17434369,-0.071938254,-0.121579744,-0.027293466,0.08612929,0.02840212,-0.2626434,-0.1760617,-0.043875616,-0.02340788,-0.026774127,0.19051583,-0.06767007,0.60734683,0.008135236,-0.1369426,0.09725895,0.081855245,-0.034787565,-0.0025545189,-0.06654288,0.02613734,0.019023193,0.09886311,0.033942364,0.20981117]}]}
```

<img width="1277" height="574" alt="" src="https://github.com/user-attachments/assets/f5a14f34-f7d7-4202-a698-978a4d38de64" />
