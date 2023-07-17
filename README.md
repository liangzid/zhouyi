# zhouyi

A classical, effective, flexible, and open-sourced divination application
for traditional-chinese *zhouyi* theory.

## core.rs

### introduction

This directory first models the structure of zhouyi model, and then reproduces several types of most popular divination method, to export a `Gua` and related `Yao`s for a given `event`, including:
- [x] the coin divination (铜钱卦)
- [x] Da-yanshi divination (大衍筮法)
- [ ] the plum-yi divination (梅花易数)
- [ ] ...

### Use this module

You can compile it as the execute application, or build it as the library.

In the first situation, check "[[bin]]" in [core.rs/Cargo.toml](https://github.com/liangzid/zhouyi/blob/master/core.rs/Cargo.toml) and run `cargo run --bin XXX`, where `XXX` is the bin name.
As for the library, just run:
build project
```sh
cargo build
```

This module only provides one function `show_text_divinate`, which takes a `&str` named `divinate_type` and a `&str` named `event` as the input, and return all important information for this it.

Here is an example: suppose you want to predict whether it is rain of a sunday tomorrow with the *Da-yanshi divination* method, you can write:

```rust
fn main(){
    let x=show_text_divinate("dayanshi",
    "help me predict if it is rain tomorrow.");
    println!("{:?}",x);
}
```
, and the result might be

```rust
({"gua_top": "兑", "xang_top": "泽", "gua_bottom": "离",
"name": "革", "duan": "《彖》曰：革，水火相息，二女同居，其志不相得，曰革。
「己日乃孚」，革而信也。文明以說，大亨以正，革而當，其悔乃亡。
天地革而四時成，湯武革命，順乎天而應乎人，革之時義大矣哉！",
"gua": "革，己日乃孚，元亨利貞，悔亡。",
"xang_bottom": "火",
"xang": "《象》曰：澤中有火，革。君子以治歷明時。"},
["初九：鞏用黃牛之革。",
"六二：己日乃革之，征吉，無咎。",
"九三：征凶，貞厲，革言三就，有孚。",
"九四：悔亡，有孚改命，吉。",
"九五：大人虎變，未占有孚。",
"上六：君子豹變，小人革面，征凶，居貞吉。"],
["《象》曰：「鞏用黃牛」，不可以有為也。",
"《象》曰：「己日革之」，行有嘉也。",
"《象》曰：「革言三就」，又何之矣。",
"《象》曰：改命之吉，信志也。",
"《象》曰：「大人虎變」，其文炳也。",
"《象》曰：「君子豹變」，其文蔚也。
「小人革面」，順以從君也。"])
```

Just further warp this structure of type `(HashMap<&'g str,String>,Vec<String>,Vec<String>)` and select the information you need for this.

Based on the `Gua` of `革`, we might take a conclusion that the weather tomorrow will changed from today's, so there might be rainy in my place.

## Contact author

Feel free to send me emails or open issues for this repository.