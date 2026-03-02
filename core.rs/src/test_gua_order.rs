use zhouyi::{record_64_Gua, SubGua};
use std::collections::HashMap;

fn main() {
    let qian = SubGua {id: vec![1,1,1], name: String::from("乾")};
    let kun = SubGua {id: vec![0,0,0], name: String::from("坤")};
    let zhen = SubGua {id: vec![0,0,1], name: String::from("震")};
    let gen = SubGua {id: vec![1,0,0], name: String::from("艮")};
    let li = SubGua {id: vec![1,0,1], name: String::from("离")};
    let kan = SubGua {id: vec![0,1,0], name: String::from("坎")};
    let dui = SubGua {id: vec![0,1,1], name: String::from("兑")};
    let xun = SubGua {id: vec![1,1,0], name: String::from("巽")};

    let subguas = vec![qian,kun,zhen,gen,li,kan,dui,xun];
    
    let tian = String::from("天");
    let di = String::from("地");
    let lei = String::from("雷");
    let shan = String::from("山");
    let huo = String::from("火");
    let shui = String::from("水");
    let ze = String::from("泽");
    let feng = String::from("风");

    let xangs = vec![tian,di,lei,shan,huo,shui,ze,feng];

    let mut map_xang_gua:HashMap<& String,& SubGua> = HashMap::new();
    for i_sub in 0..subguas.len(){
        map_xang_gua.insert(xangs.get(i_sub).unwrap(),subguas.get(i_sub).unwrap());
    }
    
    let zhouyi_model = record_64_Gua(&subguas, &xangs, &map_xang_gua);
    
    // 打印八卦编号
    println!("=== 八卦编号 ===");
    for (i, sg) in subguas.iter().enumerate() {
        println!("{}: {} -> id={:?}", i, sg.name, sg.id);
    }
    
    // 打印所有64卦
    println!("\n=== 64卦顺序 ===");
    for (i, gua) in zhouyi_model.iter().enumerate() {
        println!("{}: {} (上={}, 下={})", i, gua.gua_name, gua.subxang_top, gua.subxang_bottom);
    }
}
