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

### In Rust

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

#### In C/C++
We use [cbindgen](https://github.com/mozilla/cbindgen) to achieve it.
Check `my_head.h` in `core.rs/my_head.h`.

If you want to generate this binding by yourself, first execute:
```sh
cargo install --force cbindgen
```
to install `cbindgen`. Then generate the bindings by:

```sh
cbindgen --config cbindgen.toml --crate zhouyi --output headname_youset.h
```

#### In python

We use [pyo3](https://github.com/PyO3/pyo3) to build python3 bindings, by

```sh
# 1. build and activate a virtual environment.
python -m venv myenv
source myenv/bin/activate

# 2. install maturin
pip install maturin

# 3. develop this lib.
maturin develop
```
and you can find the package in both `src/target`, and your virtual environments.

To use this package in python, you can simple run:

```py
>>> import zhouyipy as z
>>> results=z.divinate_py("dayanshi","one or two?")
length of gua list: 64
>>> results
'({"duan": "《彖》曰：艮，止也。時止則止，時行則行，動靜不失其時，其道光明。艮其止，止其所也。上下敵應，不相與也。是以「不獲其身，行其庭不見其人，無咎」也。", "gua": "艮，艮其背，不獲其身，行其庭，不見其人，無咎。", "xang_top": "山", "xang": "《象》曰：兼山，艮。君子以思不出其位。", "gua_bottom": "艮", "gua_top": "艮", "xang_bottom": "山", "name": "艮"}, ["初六：艮其趾，無咎，利永貞。", "六二：艮其腓，不拯其隨，其心不快。", "九三：艮其限，列其夤，厲薰心。", "六四：艮其身，無咎。", "六五：艮其輔，言有序，悔亡。", "上九：敦艮，吉。"], ["《象》曰：艮其趾，未失正也。", "《象》曰：「不拯其隨」，未退聽也。", "《象》曰：「艮其限」，危薰心也。", "《象》曰：「艮其身」，止諸躬也。", "《象》曰：「艮其輔」，以中正也。", "《象》曰：敦艮之吉，以厚終也。"])'
```

Noted that this binding requires `python>=3.7`.

#### Note

when you compile python bindings, you should comment and uncomment
some of the configs in `Cargo.toml`. Specifically, there are two libs:

```toml

# version for general build
[lib]
name = "main"
path = "src/explain_gua.rs"

# version for python3 
[lib]
name = "zhouyipy"
path = "src/explain_gua.rs"
crate-type = ["cdylib"]
```

## zhouyi_ui

1. Set the compile environment.

```sh
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

2. Run natively

```sh
cargo run --release
```

3. Run WASM

```sh
1. install trunk
cargo install --locked trunk

2. locally debug
trunk serve # on http://127.0.0.1:8080/index.html#dev

3. release WSAM
trunk build --release
```


## Contact author

Feel free to send me emails or open issues for this repository.