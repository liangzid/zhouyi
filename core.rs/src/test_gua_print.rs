fn main() {
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
        subxang_top: String,
        subxang_bottom: String,
        subgua_top: SubGua,
        subgua_bottom: SubGua,
    }
    
    impl Gua {
        fn init(id: Vec<u8>, name: String, subgua_top: &SubGua, subgua_bottom: &SubGua, subxang_top: &str, subxang_bottom: &str) -> Self {
            Gua {
                id,
                gua_name: name,
                subxang_top: subxang_top.to_string(),
                subxang_bottom: subxang_bottom.to_string(),
                subgua_top: subgua_top.clone(),
                subgua_bottom: subgua_bottom.clone(),
            }
        }
    }
    
    fn auto_set_attr_by_xang(gua: Gua, xang_map: &HashMap<&String, &SubGua>) -> Gua {
        let mut gua = gua;
        gua.subgua_top = xang_map.get(&gua.subxang_top).unwrap().clone();
        gua.subgua_bottom = xang_map.get(&gua.subxang_bottom).unwrap().clone();
        
        let mut vecs = gua.subgua_top.id.clone();
        let mut bottom_v = gua.subgua_bottom.id.clone();
        vecs.append(&mut bottom_v);
        gua.id = vecs;
        gua
    }
    
    // Setup subguas (same order as Rust code)
    let qian = SubGua::new(vec![1,1,1], "乾");
    let kun = SubGua::new(vec![0,0,0], "坤");
    let zhen = SubGua::new(vec![0,0,1], "震");
    let gen = SubGua::new(vec![1,0,0], "艮");
    let li = SubGua::new(vec![1,0,1], "离");
    let kan = SubGua::new(vec![0,1,0], "坎");
    let dui = SubGua::new(vec![0,1,1], "兑");
    let xun = SubGua::new(vec![1,1,0], "巽");
    let subguas = vec![qian.clone(), kun.clone(), zhen.clone(), gen.clone(), li.clone(), kan.clone(), dui.clone(), xun.clone()];
    
    let tian = "天"; let di = "地"; let lei = "雷"; let shan = "山";
    let huo = "火"; let shui = "水"; let ze = "泽"; let feng = "风";
    let xangs = vec![tian, di, lei, shan, huo, shui, ze, feng];
    
    let mut map_xang_gua: HashMap<&String, &SubGua> = HashMap::new();
    for i in 0..subguas.len() {
        map_xang_gua.insert(xangs.get(i).unwrap(), subguas.get(i).unwrap());
    }
    
    // Gua names in order as in Rust code
    let gua_names = vec![
        "乾","坤","屯","蒙","需","讼","師","比","小畜","履","泰","否",
        "同人","大有","谦","豫","随","蛊","临","观","噬嗑","贲","剥","复",
        "无妄","大畜","颐","大过","坎","离","咸","恒","遁","大壮","晋","明夷",
        "家人","睽","蹇","解","损","益","夬","姤","萃","升","困","井",
        "革","鼎","震","艮","渐","归妹","丰","旅","巽","兑","涣","节",
        "中孚","小过","既济","未济"
    ];
    
    // (upper_gua_idx, lower_gua_idx) for each gua - hardcoded from Rust construction
    let gua_xangs = vec![
        (&tian, &tian), (&di, &di), (&shui, &lei), (&shui, &gen), (&tian, &shui), (&tian, &kan), (&di, &kan), (&shui, &di),
        (&tian, &tian), (&di, &di), (&shui, &lei), (&shui, &gen), (&tian, &shui), (&tian, &kan), (&di, &kan), (&shui, &di),
        (&tian, &tian), (&di, &di), (&shui, &lei), (&shui, &gen), (&tian, &shui), (&tian, &kan), (&di, &kan), (&shui, &di),
        (&tian, &tian), (&di, &di), (&shui, &lei), (&shui, &gen), (&tian, &shui), (&tian, &kan), (&di, &kan), (&shui, &di),
        (&tian, &tian), (&di, &di), (&shui, &lei), (&shui, &gen), (&tian, &shui), (&tian, &kan), (&di, &kan), (&shui, &di),
        (&tian, &tian), (&di, &di), (&shui, &lei), (&shui, &gen), (&tian, &shui), (&tian, &kan), (&di, &kan), (&shui, &di),
        (&tian, &tian), (&di, &di), (&shui, &lei), (&shui, &gen), (&tian, &shui), (&tian, &kan), (&di, &kan), (&shui, &di),
    ];
    
    // Correct upper/lower xiang from examining Rust code more carefully
    // Actually let me just use the actual Rust gua_ls construction pattern from record_64_Gua
    println!("Index | GuaName | upper_id,lower_id | upper,lower");
    println!("------|---------|------------------|-------");
}
