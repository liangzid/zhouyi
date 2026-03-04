// 直接复制 model.rs 的逻辑生成完整64卦 JSON
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone)]
struct SubGua {
    id: Vec<u8>,
    name: String,
}

#[derive(Debug, Clone)]
struct Gua {
    id: Vec<u8>,
    gua_name: String,
    subgua_top: SubGua,
    subgua_bottom: SubGua,
    subxang_top: String,
    subxang_bottom: String,
}

// 读取文本数据
fn load_text_data() -> Vec<(String, String, String, String, Vec<String>, Vec<String>)> {
    let path = Path::new("src/Structured_Zhouyi.json");
    let content = std::fs::read_to_string(path).expect("Failed to read Structured_Zhouyi.json");
    let data: Vec<serde_json::Value> = serde_json::from_str(&content).expect("Failed to parse JSON");

    let mut result = Vec::new();
    for item in data {
        let gua_name = item[0].as_str().unwrap().to_string();
        let gua_ci = item[1].as_str().unwrap().to_string();
        let duan = item[2].as_str().unwrap().to_string();
        let xiang = item[3].as_str().unwrap().to_string();
        let yao_ci: Vec<String> = item[4].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
        let yao_xiang: Vec<String> = item[5].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
        result.push((gua_name, gua_ci, duan, xiang, yao_ci, yao_xiang));
    }
    result
}

// 完全按照 model.rs 的 record_64_Gua 函数定义64卦
fn create_64_gua() -> Vec<Gua> {
    // 八卦（与 model.rs 完全一致）
    let qian = SubGua { id: vec![1, 1, 1], name: String::from("乾") };
    let kun = SubGua { id: vec![0, 0, 0], name: String::from("坤") };
    let zhen = SubGua { id: vec![0, 0, 1], name: String::from("震") };
    let gen = SubGua { id: vec![1, 0, 0], name: String::from("艮") };
    let li = SubGua { id: vec![1, 0, 1], name: String::from("离") };
    let kan = SubGua { id: vec![0, 1, 0], name: String::from("坎") };
    let dui = SubGua { id: vec![0, 1, 1], name: String::from("兑") };
    let xun = SubGua { id: vec![1, 1, 0], name: String::from("巽") };

    // 八卦对应的象（与 model.rs 完全一致）
    let tian = String::from("天");
    let di = String::from("地");
    let lei = String::from("雷");
    let shan = String::from("山");
    let huo = String::from("火");
    let shui = String::from("水");
    let ze = String::from("泽");
    let feng = String::from("风");

    // 按照 model.rs 的 Gua::init 调用顺序创建64卦
    // 参数顺序: id, name, subgua_top, subgua_bottom, subxang_top, subxang_bottom
    let guas: Vec<Gua> = vec![
        // 乾 坤 屯 蒙
        create_gua(&qian, &qian, &tian, &tian, "乾"),
        create_gua(&kun, &kun, &di, &di, "坤"),
        create_gua(&kan, &zhen, &shui, &lei, "屯"),
        create_gua(&gen, &kan, &shan, &shui, "蒙"),
        // 需 讼 师 比
        create_gua(&kan, &qian, &shui, &tian, "需"),
        create_gua(&qian, &kan, &tian, &shui, "讼"),
        create_gua(&kun, &kan, &di, &shui, "師"),
        create_gua(&kan, &kun, &shui, &di, "比"),
        // 小畜 履 秦 否
        create_gua(&xun, &qian, &feng, &tian, "小畜"),
        create_gua(&dui, &qian, &ze, &tian, "履"),
        create_gua(&kun, &qian, &di, &tian, "泰"),
        create_gua(&qian, &kun, &tian, &di, "否"),
        // 同人 大有 谦 豫
        create_gua(&li, &qian, &huo, &tian, "同人"),
        create_gua(&qian, &li, &tian, &huo, "大有"),
        create_gua(&gen, &kun, &shan, &di, "謙"),
        create_gua(&zhen, &kun, &lei, &di, "豫"),
        // 随 蛊 临 观
        create_gua(&zhen, &dui, &lei, &ze, "隨"),
        create_gua(&gen, &li, &shan, &huo, "蠱"),
        create_gua(&zhen, &kun, &lei, &di, "臨"),
        create_gua(&xun, &li, &feng, &huo, "觀"),
        // 噬嗑 賁 剝 復
        create_gua(&li, &zhen, &huo, &lei, "噬嗑"),
        create_gua(&gen, &li, &shan, &huo, "賁"),
        create_gua(&kun, &gen, &di, &shan, "剝"),
        create_gua(&zhen, &kun, &lei, &di, "復"),
        // 无妄 大畜 颐 大过
        create_gua(&gen, &qian, &shan, &tian, "無妄"),
        create_gua(&qian, &gen, &tian, &shan, "大畜"),
        create_gua(&gen, &zhen, &shan, &lei, "頤"),
        create_gua(&xun, &dui, &feng, &ze, "大過"),
        // 坎 离 咸 恒
        create_gua(&kan, &kan, &shui, &shui, "坎"),
        create_gua(&li, &li, &huo, &huo, "離"),
        create_gua(&dui, &gen, &ze, &shan, "咸"),
        create_gua(&xun, &zhen, &feng, &lei, "恆"),
        // 遯 大壮 晋 明夷
        create_gua(&qian, &gen, &tian, &shan, "遯"),
        create_gua(&zhen, &qian, &lei, &tian, "大壯"),
        create_gua(&li, &kun, &huo, &di, "晉"),
        create_gua(&li, &gen, &huo, &shan, "明夷"),
        // 家人 睽 蹇 解
        create_gua(&li, &li, &huo, &huo, "家人"),
        create_gua(&dui, &li, &ze, &huo, "睽"),
        create_gua(&gen, &kan, &shan, &shui, "蹇"),
        create_gua(&zhen, &kan, &lei, &shui, "解"),
        // 损 益 夬 姤
        create_gua(&dui, &gen, &ze, &shan, "損"),
        create_gua(&zhen, &xun, &lei, &feng, "益"),
        create_gua(&dui, &qian, &ze, &tian, "夬"),
        create_gua(&qian, &xun, &tian, &feng, "姤"),
        // 萃 升 困 井
        create_gua(&dui, &kun, &ze, &di, "萃"),
        create_gua(&kun, &xun, &di, &feng, "升"),
        create_gua(&dui, &kan, &ze, &shui, "困"),
        create_gua(&kan, &xun, &shui, &feng, "井"),
        // 革 鼎 震 艮
        create_gua(&li, &dui, &huo, &ze, "革"),
        create_gua(&li, &dui, &huo, &ze, "鼎"),
        create_gua(&zhen, &zhen, &lei, &lei, "震"),
        create_gua(&gen, &gen, &shan, &shan, "艮"),
        // 渐 归妹 丰 旅
        create_gua(&xun, &li, &feng, &huo, "漸"),
        create_gua(&dui, &kan, &ze, &shui, "歸妹"),
        create_gua(&li, &zhen, &huo, &lei, "丰"),
        create_gua(&li, &xun, &huo, &feng, "旅"),
        // 巽 兑 涣 节
        create_gua(&xun, &xun, &feng, &feng, "巽"),
        create_gua(&dui, &dui, &ze, &ze, "兌"),
        create_gua(&zhen, &xun, &lei, &feng, "渙"),
        create_gua(&xun, &kan, &feng, &shui, "節"),
        // 中孚 小过 既济 未济
        create_gua(&li, &xun, &huo, &feng, "中孚"),
        create_gua(&xun, &kan, &feng, &shui, "小過"),
        create_gua(&kan, &li, &shui, &huo, "既濟"),
        create_gua(&li, &kan, &huo, &shui, "未濟"),
    ];

    guas
}

fn create_gua(subgua_top: &SubGua, subgua_bottom: &SubGua, subxang_top: &str, subxang_bottom: &str, name: &str) -> Gua {
    let mut id = subgua_top.id.clone();
    id.extend(&subgua_bottom.id);

    Gua {
        id,
        gua_name: String::from(name),
        subgua_top: subgua_top.clone(),
        subgua_bottom: subgua_bottom.clone(),
        subxang_top: String::from(subxang_top),
        subxang_bottom: String::from(subxang_bottom),
    }
}

#[derive(Debug, Serialize)]
struct OutputGua {
    gua_index: usize,
    gua_name: String,
    gua_ci: String,
    duan: String,
    xiang: String,
    yao_ci: Vec<String>,
    yao_xiang: Vec<String>,
    binary_vec: Vec<u8>,
    upper_binary: Vec<u8>,
    lower_binary: Vec<u8>,
    upper_gua: String,
    lower_gua: String,
    upper_xiang: String,
    lower_xiang: String,
    gua_xiang: String,
}

fn main() {
    // 创建64卦
    let guas = create_64_gua();

    // 加载文本数据
    let text_data = load_text_data();

    // 组合输出
    let mut output: Vec<OutputGua> = Vec::new();

    for (i, gua) in guas.iter().enumerate() {
        let (_, gua_ci, duan, xiang, yao_ci, yao_xiang) = &text_data[i];

        let upper_binary: Vec<u8> = gua.id[0..3].to_vec();
        let lower_binary: Vec<u8> = gua.id[3..6].to_vec();

        output.push(OutputGua {
            gua_index: i,
            gua_name: gua.gua_name.clone(),
            gua_ci: gua_ci.clone(),
            duan: duan.clone(),
            xiang: xiang.clone(),
            yao_ci: yao_ci.clone(),
            yao_xiang: yao_xiang.clone(),
            binary_vec: gua.id.clone(),
            upper_binary,
            lower_binary,
            upper_gua: gua.subgua_top.name.clone(),
            lower_gua: gua.subgua_bottom.name.clone(),
            upper_xiang: gua.subxang_top.clone(),
            lower_xiang: gua.subxang_bottom.clone(),
            gua_xiang: format!("{}{}", gua.subxang_top, gua.subxang_bottom),
        });
    }

    // 写入JSON
    let json_content = serde_json::to_string_pretty(&output).expect("Failed to serialize JSON");
    let mut file = File::create("src/ZhouyiFull.json").expect("Failed to create file");
    file.write_all(json_content.as_bytes()).expect("Failed to write file");

    println!("Generated ZhouyiFull.json with {} gua", output.len());

    // 验证
    println!("\n=== 验证示例 ===");
    for i in [0, 1, 2, 8, 10, 42, 62, 63] {
        let g = &output[i];
        println!("Index {:2}: {:4} | 上卦:{:2}({:}) 下卦:{:2}({:}) | 卦象:{} | binary: {:?}",
            i, g.gua_name, g.upper_gua, g.upper_xiang, g.lower_gua, g.lower_xiang, g.gua_xiang, g.binary_vec);
    }
}
