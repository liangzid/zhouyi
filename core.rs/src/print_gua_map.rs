// Run with: cd core.rs && cargo run --bin print_gua_map

use std::collections::HashMap;

#[derive(Clone)]
struct SubGua {
    id: Vec<u8>,
    name: String,
}

impl SubGua {
    fn new(id: Vec<u8>, name: &str) -> Self {
        SubGua { id, name: name.to_string() }
    }
}

#[derive(Clone)]
struct Gua {
    id: Vec<u8>,
    gua_name: String,
    subgua_top: SubGua,
    subgua_bottom: SubGua,
    subxang_top: String,
    subxang_bottom: String,
}

impl Gua {
    fn init(id: Vec<u8>, name: String, subgua_top: &SubGua, subgua_bottom: &SubGua, subxang_top: &str, subxang_bottom: &str) -> Self {
        Gua {
            id,
            gua_name: name,
            subgua_top: subgua_top.clone(),
            subgua_bottom: subgua_bottom.clone(),
            subxang_top: subxang_top.to_string(),
            subxang_bottom: subxang_bottom.to_string(),
        }
    }
}

fn auto_set_attr_by_xang(gua: Gua, xang_map: &HashMap<String, SubGua>) -> Gua {
    let mut gua = gua;
    gua.subgua_top = xang_map.get(&gua.subxang_top).unwrap().clone();
    gua.subgua_bottom = xang_map.get(&gua.subxang_bottom).unwrap().clone();

    let mut vecs = gua.subgua_top.id.clone();
    let mut bottom_v = gua.subgua_bottom.id.clone();
    vecs.append(&mut bottom_v);
    gua.id = vecs;
    gua
}

fn main() {
    // Setup subguas (Rust order: 0=乾, 1=坤, 2=震, 3=艮, 4=离, 5=坎, 6=兑, 7=巽)
    let subguas = vec![
        SubGua::new(vec![1,1,1], "乾"),  // 0
        SubGua::new(vec![0,0,0], "坤"),  // 1
        SubGua::new(vec![0,0,1], "震"),  // 2
        SubGua::new(vec![1,0,0], "艮"),  // 3
        SubGua::new(vec![1,0,1], "离"),  // 4
        SubGua::new(vec![0,1,0], "坎"),  // 5
        SubGua::new(vec![0,1,1], "兑"),  // 6
        SubGua::new(vec![1,1,0], "巽"),  // 7
    ];

    let xangs = vec!["天", "地", "雷", "山", "火", "水", "泽", "风"];

    let mut xang_to_subgua: HashMap<String, SubGua> = HashMap::new();
    for (i, xang) in xangs.iter().enumerate() {
        xang_to_subgua.insert(xang.to_string(), subguas[i].clone());
    }

    // 64 gua as defined in Rust code
    let gua_defs = vec![
        ("乾",  &subguas[0], &subguas[0], "天", "天"),
        ("坤",  &subguas[1], &subguas[1], "地", "地"),
        ("屯",  &subguas[5], &subguas[2], "水", "雷"),
        ("蒙",  &subguas[3], &subguas[5], "山", "水"),
        ("需",  &subguas[5], &subguas[0], "水", "天"),
        ("讼",  &subguas[0], &subguas[5], "天", "水"),
        ("师",  &subguas[1], &subguas[5], "地", "水"),
        ("比",  &subguas[5], &subguas[1], "水", "地"),
        ("小畜", &subguas[7], &subguas[0], "风", "天"),
        ("履",  &subguas[6], &subguas[0], "泽", "天"),
        ("泰",  &subguas[1], &subguas[0], "地", "天"),
        ("否",  &subguas[0], &subguas[1], "天", "地"),
        ("同人", &subguas[4], &subguas[0], "火", "天"),
        ("大有", &subguas[0], &subguas[4], "天", "火"),
        ("谦",  &subguas[1], &subguas[3], "地", "山"),
        ("豫",  &subguas[2], &subguas[1], "雷", "地"),
        ("随",  &subguas[6], &subguas[2], "泽", "雷"),
        ("蛊",  &subguas[7], &subguas[3], "风", "山"),
        ("临",  &subguas[1], &subguas[6], "地", "泽"),
        ("观",  &subguas[7], &subguas[1], "风", "地"),
        ("噬嗑", &subguas[4], &subguas[2], "火", "雷"),
        ("贲",  &subguas[3], &subguas[4], "山", "火"),
        ("剥",  &subguas[3], &subguas[1], "山", "地"),
        ("复",  &subguas[1], &subguas[2], "地", "雷"),
        ("无妄", &subguas[2], &subguas[0], "雷", "天"),
        ("大畜", &subguas[3], &subguas[0], "山", "天"),
        ("颐",  &subguas[3], &subguas[2], "山", "雷"),
        ("大过", &subguas[6], &subguas[7], "泽", "风"),
        ("坎",  &subguas[5], &subguas[5], "水", "水"),
        ("离",  &subguas[4], &subguas[4], "火", "火"),
        ("咸",  &subguas[6], &subguas[3], "泽", "山"),
        ("恒",  &subguas[2], &subguas[7], "雷", "风"),
        ("遁",  &subguas[0], &subguas[3], "天", "山"),
        ("大壮", &subguas[2], &subguas[0], "雷", "天"),
        ("晋",  &subguas[4], &subguas[1], "火", "地"),
        ("明夷", &subguas[1], &subguas[4], "地", "火"),
        ("家人", &subguas[7], &subguas[4], "风", "火"),
        ("睽",  &subguas[4], &subguas[6], "火", "泽"),
        ("蹇",  &subguas[5], &subguas[3], "水", "山"),
        ("解",  &subguas[2], &subguas[5], "雷", "水"),
        ("损",  &subguas[3], &subguas[6], "山", "泽"),
        ("益",  &subguas[7], &subguas[2], "风", "雷"),
        ("夬",  &subguas[6], &subguas[0], "泽", "天"),
        ("姤",  &subguas[0], &subguas[7], "天", "风"),
        ("萃",  &subguas[6], &subguas[1], "泽", "地"),
        ("升",  &subguas[1], &subguas[7], "地", "风"),
        ("困",  &subguas[6], &subguas[5], "泽", "水"),
        ("井",  &subguas[5], &subguas[7], "水", "风"),
        ("革",  &subguas[6], &subguas[4], "泽", "火"),
        ("鼎",  &subguas[4], &subguas[7], "火", "风"),
        ("震",  &subguas[2], &subguas[2], "雷", "雷"),
        ("艮",  &subguas[3], &subguas[3], "山", "山"),
        ("渐",  &subguas[7], &subguas[3], "风", "山"),
        ("归妹", &subguas[2], &subguas[6], "雷", "泽"),
        ("丰",  &subguas[2], &subguas[4], "雷", "火"),
        ("旅",  &subguas[4], &subguas[3], "火", "山"),
        ("巽",  &subguas[7], &subguas[7], "风", "风"),
        ("兑",  &subguas[6], &subguas[6], "泽", "泽"),
        ("涣",  &subguas[7], &subguas[5], "风", "水"),
        ("节",  &subguas[5], &subguas[6], "水", "泽"),
        ("中孚", &subguas[7], &subguas[6], "风", "泽"),
        ("小过", &subguas[2], &subguas[3], "雷", "山"),
        ("既济", &subguas[5], &subguas[4], "水", "火"),
        ("未济", &subguas[4], &subguas[5], "火", "水"),
    ];

    println!("// Rust 64 gua mapping (from Rust code)");
    println!("// Rust SubGua indices: 0=乾, 1=坤, 2=震, 3=艮, 4=离, 5=坎, 6=兑, 7=巽");
    println!("// Binary: 000=坤, 001=震, 010=坎, 011=兑, 100=艮, 101=离, 110=巽, 111=乾");
    println!();
    println!("// index | name | upperSubguaIdx | lowerSubguaIdx | upper_binary | lower_binary | upperXiang | lowerXiang | hexagram_name");
    println!("// -------------------------------------------------------------------------------------------------------------------");

    for (i, (name, upper_sg, lower_sg, xiang_top, xiang_bottom)) in gua_defs.iter().enumerate() {
        let upper_binary = &upper_sg.id;
        let lower_binary = &lower_sg.id;

        // Find Rust indices
        let upper_idx = subguas.iter().position(|s| s.name == upper_sg.name).unwrap();
        let lower_idx = subguas.iter().position(|s| s.name == lower_sg.name).unwrap();

        // Hexagram name in format "上卦下卦"
        let hex_name = format!("{}{}", xiang_bottom, xiang_top);

        println!("  {:2} | {} | {:2} | {:2} | {:?} | {:?} | {} | {} | {}",
            i, name, upper_idx, lower_idx, upper_binary, lower_binary, xiang_bottom, xiang_top, hex_name);
    }
}