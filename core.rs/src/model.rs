use std::{collections::HashMap, io::Write};
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{Result,Value};
fn main() {
    println!("Test build 64 Gua.");

        let qian=SubGua {id: vec![1,1,1], name: String::from("乾")};
    let kun=SubGua {id: vec![0,0,0], name: String::from("坤")};
    let zhen=SubGua {id: vec![0,0,1], name: String::from("震")};
    let gen=SubGua {id: vec![1,0,0], name: String::from("艮")};
    let li=SubGua {id: vec![1,0,1], name: String::from("离")};
    let kan=SubGua {id: vec![0,1,0], name: String::from("坎")};
    let dui=SubGua {id: vec![0,1,1], name: String::from("兑")};
    let xun=SubGua {id: vec![1,1,0], name: String::from("巽")};

    let subguas=vec![qian,kun,zhen,gen,li,kan,dui,xun];
    
    let tian=String::from("天");
    let di=String::from("地");
    let lei=String::from("雷");
    let shan=String::from("山");
    let huo=String::from("火");
    let shui=String::from("水");
    let ze=String::from("泽");
    let feng=String::from("风");

    let xangs=vec![tian,di,lei,shan,huo,shui,ze,feng];

    let mut map_xang_gua:HashMap<& String,& SubGua>=HashMap::new();
    for i_sub in 0..subguas.len(){
        map_xang_gua.insert(xangs.get(i_sub).unwrap(),subguas.get(i_sub).unwrap(),);
    }
    let zhouyimodel=record_64_Gua(&subguas,&xangs,&map_xang_gua);

}
// the gua with three yaos, like 乾、坤、坎、离
#[derive(Default,Serialize,Deserialize,Clone)]
pub struct SubGua {
    pub id: Vec<u8>,
    pub name: String,
}

// the explaintation of a gua's text.
#[derive(Default,Clone,Deserialize,Serialize)]
struct ExplainText {
    gua: String,
    yaos: Vec<String>,
}

// the contents of a gua.
#[derive(Default,Clone,Deserialize,Serialize,Debug)]
struct GuaText {
    gua: String,
    duan: String,
    xang: String,
    yaos: StringVec, // with length 6
    yaos_xang: StringVec,
    // explains: Vec<ExplainText>, // explain texts
}

impl GuaText {
       fn from_json(path:&str)->Vec<GuaText> {
        let json_path=Path::new(path);
        let file=File::open(json_path).expect("Error while reading files.");
        let reader = BufReader::new(file);
        let gua_txt_ls: Vec<GuaText>=serde_json::from_reader(reader).expect("Error while parsing");
        return gua_txt_ls;
        // gua_txt_ls
       }
}

#[derive(Default,Clone,Debug,Deserialize,Serialize)]
struct StringVec(Vec<String>);

impl fmt::Display for StringVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings = self.0.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        write!(f, "[{}]", strings.join(", "))
    }
}

#[derive(Clone)]
pub struct Gua<'g> {
    id: Vec<u8>,
    gua_name: String,
    subgua_top: &'g SubGua,
    subgua_bottom:&'g SubGua,
    subxang_top: &'g String,
    subxang_bottom: &'g String,
    text:GuaText,
    hu_gua: Option<&'g Gua<'g>>,
    cuo_gua: Option<&'g Gua<'g>>,
    zong_gua: Option<&'g Gua<'g>>,
}

impl Gua<'_> {
    fn init<'g>(id:Vec<u8>,gua_name:String,subgua_top:&'g SubGua, subgua_bottom:&'g SubGua,
    subxang_top:&'g String,subxang_bottom:&'g String, text: GuaText,
    //  hu_gua:&'g Gua<'s,'g>,cuo_gua:&'g Gua<'s,'g>,zong_gua:&'g Gua<'s,'g>,
    )->Gua<'g>{
        let mut gua:Gua<'_>=Gua { id, gua_name, 
            subgua_top, subgua_bottom, subxang_top, subxang_bottom,
             text, hu_gua: None,cuo_gua:None,zong_gua:None,
        };
        return gua;
        // auto_set_attr_by_xang(& mut gua, xang_map);
        // return gua;
     }
     
}


pub fn query<'g>(guals:&Vec<Gua>, yaos:Vec<u8>)->(HashMap<&'g str,String>,Vec<String>,Vec<String>){
        let mut x:HashMap<&str,String>=HashMap::new();
        let mut yaoss:Vec<String>=vec![];
        let mut yaos_xang:Vec<String>=vec![];
        // 1. first get related yao
        for gua in guals{
                if gua.id==yaos{
                        // 2. then got all related information!
                        x.insert("name", gua.gua_name.clone());
                        x.insert("xang_top",gua.subxang_top.clone());
                        x.insert("xang_bottom",gua.subxang_bottom.clone());
                        x.insert("gua_top",gua.subgua_top.name.clone());
                        x.insert("gua_bottom",gua.subgua_bottom.name.clone());
                        x.insert("gua",gua.text.gua.clone());
                        x.insert("duan", gua.text.duan.clone());
                        x.insert("xang",gua.text.xang.clone());
                        // for x in gua.text.yaos.0{yaoss.push(x.clone())};
                        // for x in gua.text.yaos_xang.0{yaos_xang.push(x.clone())};
                        yaoss=gua.text.yaos.0.clone();
                        yaos_xang=gua.text.yaos_xang.0.clone();
                break;
                }
        }
        (x,yaoss,yaos_xang)
}



impl fmt::Display for Gua<'_> {
        fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
                write!(f,"name: {}, 象: {}{}{}, 子卦: {}上{}下,{},{},{},{},{}",
                self.gua_name,self.subxang_top,
        self.subxang_bottom,self.gua_name,self.subgua_top.name,self.subgua_bottom.name,
self.text.gua,self.text.duan,self.text.xang,self.text.yaos,self.text.yaos_xang)
        }
}

fn auto_set_attr_by_xang<'g>(gua: Gua<'g>,
         xang_map:&'g HashMap<&'g String,&'g SubGua>)-> Gua<'g>{
        let mut gua=gua;
        gua.subgua_top=xang_map.get(gua.subxang_top).unwrap();
        gua.subgua_bottom=xang_map.get(gua.subxang_bottom).unwrap();
        
        let mut vecs:Vec<u8>=Vec::new();
        let mut bottom_v:Vec<u8>=gua.subgua_bottom.id.clone();
        vecs=gua.subgua_top.id.clone();
        vecs.append(&mut bottom_v);
        gua.id=vecs;
        // gua.clone()
        gua
    }

fn set_hucuozong<'s,'g>(gua:&'g mut Gua<'g>,
hu:&'g Gua<'g>,cuo:&'g Gua<'g>,zong:&'g Gua<'g>)->(){
        gua.hu_gua=Some(hu);
        gua.cuo_gua=Some(cuo);
        gua.zong_gua=Some(zong);
}

#[derive(Default,Clone)]
pub struct ZhouyiModel<'g> {
        subguas:Vec<SubGua>,
        xangs:Vec<String>,
        guas: Vec<Gua<'g>>,
        map_xang_gua:HashMap<&'g String,&'g SubGua>,
}





// new_gua_ls,Vec<Gua<'static>>,
pub fn record_64_Gua<'g> (subgua_ls:&'g Vec<SubGua>,xang_ls:&'g Vec<String>,map_xang_gua:
    &'g HashMap<&'g String,&'g SubGua>) -> Vec<Gua<'g>>{
        // record 8 gua


        let qian=subgua_ls.get(0).expect("1");
        let kun=subgua_ls.get(1).expect("1");
        let zhen=subgua_ls.get(2).expect("1");
        let gen=subgua_ls.get(3).expect("1");
        let li=subgua_ls.get(4).expect("1");
        let kan=subgua_ls.get(5).expect("1");
        let dui=subgua_ls.get(6).expect("1");
        let xun=subgua_ls.get(7).expect("1");


        let tian=xang_ls.get(0).expect("msg");
        let di=xang_ls.get(1).expect("msg");
        let lei=xang_ls.get(2).expect("msg");
        let shan=xang_ls.get(3).expect("msg");
        let huo=xang_ls.get(4).expect("msg");
        let shui=xang_ls.get(5).expect("msg");
        let ze=xang_ls.get(6).expect("msg");
        let feng=xang_ls.get(7).expect("msg");



    // record 64 gua
    let mut qian6=Gua::init(vec![1,1,1, 1,1,1],
        String::from("乾"),
        &qian,&qian,
&tian, &tian,
      GuaText::default());
    let mut kun6=Gua::init(vec![0,0,0,0,0,0],
        String::from("坤"),
        &kun,&kun,
&di, &di,
      GuaText::default());
      let mut zhun=Gua::init(vec![1,1,1, 1,1,1],
        String::from("屯"),
        &kan,&zhen,
&shui, &lei,
      GuaText::default());
      let mut meng=Gua::init(vec![1,1,1, 1,1,1],
        String::from("蒙"),
        &gen,&kan,
&shan, &shui,
      GuaText::default());
      let mut xu=Gua::init(vec![1,1,1, 1,1,1],
        String::from("需"),
        &kan,&qian,
&shui, &tian,
      GuaText::default());
      let mut song=Gua::init(vec![1,1,1, 1,1,1],
        String::from("讼"),
        &qian,&kan,
&tian, &shui,
      GuaText::default());
      let mut shi=Gua::init(vec![1,1,1, 1,1,1],
        String::from("师"),
        &kun,&kan,
&di, &shui,
      GuaText::default());
      let mut bi3=Gua::init(vec![1,1,1, 1,1,1],
        String::from("比"),
        &kan,&kun,
&shui, &di,
      GuaText::default());
      let mut xiaoxu=Gua::init(vec![1,1,1, 1,1,1],
        String::from("小畜"),
        &kan,&qian,
&feng, &tian,
      GuaText::default());
      let mut lv=Gua::init(vec![1,1,1, 1,1,1],
        String::from("履"),
        &kan,&qian,
&tian, &ze,
      GuaText::default());
      let mut tai=Gua::init(vec![1,1,1, 1,1,1],
        String::from("泰"),
        &kan,&qian,
&di, &tian,
      GuaText::default());
      let mut pi=Gua::init(vec![1,1,1, 1,1,1],
        String::from("否"),
        &kan,&qian,
&tian, &di,
      GuaText::default());
      let mut tongren=Gua::init(vec![1,1,1, 1,1,1],
        String::from("同人"),
        &kan,&qian,
&tian, &huo,
      GuaText::default());
      let mut dayou=Gua::init(vec![1,1,1, 1,1,1],
        String::from("大有"),
        &kan,&qian,
&huo, &tian,
      GuaText::default());
      let mut qian1=Gua::init(vec![1,1,1, 1,1,1],
        String::from("谦"),
        &kan,&qian,
&di, &shan,
      GuaText::default());
      let mut yu=Gua::init(vec![1,1,1, 1,1,1],
        String::from("豫"),
        &kan,&qian,
&lei, &di,
      GuaText::default());
      let mut sui=Gua::init(vec![1,1,1, 1,1,1],
        String::from("随"),
        &kan,&qian,
&ze, &lei,
      GuaText::default());
      let mut gu=Gua::init(vec![1,1,1, 1,1,1],
        String::from("蛊"),
        &kan,&qian,
&shan, &feng,
      GuaText::default());
      let mut lin=Gua::init(vec![1,1,1, 1,1,1],
        String::from("临"),
        &kan,&qian,
&di, &ze,
      GuaText::default());
      let mut guan=Gua::init(vec![1,1,1, 1,1,1],
        String::from("观"),
        &kan,&qian,
&feng, &di,
      GuaText::default());
      let mut shihe=Gua::init(vec![1,1,1, 1,1,1],
        String::from("噬嗑"),
        &kan,&qian,
&huo, &lei,
      GuaText::default());
      let mut bi=Gua::init(vec![1,1,1, 1,1,1],
        String::from("贲"),
        &kan,&qian,
&shan, &huo,
      GuaText::default());
      let mut bo=Gua::init(vec![1,1,1, 1,1,1],
        String::from("剥"),
        &kan,&qian,
&shan, &di,
      GuaText::default());
      let mut fu=Gua::init(vec![1,1,1, 1,1,1],
        String::from("复"),
        &kan,&qian,
&di, &lei,
      GuaText::default());
      let mut wuwang=Gua::init(vec![1,1,1, 1,1,1],
        String::from("无妄"),
        &kan,&qian,
&tian, &lei,
      GuaText::default());
      let mut daxu=Gua::init(vec![1,1,1, 1,1,1],
        String::from("大畜"),
        &kan,&qian,
&shan, &tian,
      GuaText::default());
      let mut yi2=Gua::init(vec![1,1,1, 1,1,1],
        String::from("颐"),
        &kan,&qian,
&shan, &lei,
      GuaText::default());
      let mut daguo=Gua::init(vec![1,1,1, 1,1,1],
        String::from("大过"),
        &kan,&qian,
&ze, &feng,
      GuaText::default());
      let mut kan6=Gua::init(vec![1,1,1, 1,1,1],
        String::from("坎"),
        &kan,&qian,
&shui, &shui,
      GuaText::default());
      let mut li6=Gua::init(vec![1,1,1, 1,1,1],
        String::from("离"),
        &kan,&qian,
&huo, &huo,
      GuaText::default());
      let mut xian=Gua::init(vec![1,1,1, 1,1,1],
        String::from("咸"),
        &kan,&qian,
&ze, &shan,
      GuaText::default());
      let mut heng=Gua::init(vec![1,1,1, 1,1,1],
        String::from("恒"),
        &kan,&qian,
&lei, &feng,
      GuaText::default());
      let mut dun=Gua::init(vec![1,1,1, 1,1,1],
        String::from("遁"),
        &kan,&qian,
&tian, &shan,
      GuaText::default());
      let mut dazhuang=Gua::init(vec![1,1,1, 1,1,1],
        String::from("大壮"),
        &kan,&qian,
&lei, &tian,
      GuaText::default());
      let mut jin=Gua::init(vec![1,1,1, 1,1,1],
        String::from("晋"),
        &kan,&qian,
&huo, &di,
      GuaText::default());
      let mut mingyi=Gua::init(vec![1,1,1, 1,1,1],
        String::from("明夷"),
        &kan,&qian,
&di, &huo,
      GuaText::default());
      let mut jiaren=Gua::init(vec![1,1,1, 1,1,1],
        String::from("家人"),
        &kan,&qian,
&feng, &huo,
      GuaText::default());
      let mut kui=Gua::init(vec![1,1,1, 1,1,1],
        String::from("睽"),
        &kan,&qian,
&huo, &ze,
      GuaText::default());
      let mut jian=Gua::init(vec![1,1,1, 1,1,1],
        String::from("蹇"),
        &kan,&qian,
&shui, &shan,
      GuaText::default());
      let mut xie=Gua::init(vec![1,1,1, 1,1,1],
        String::from("解"),
        &kan,&qian,
&lei, &shui,
      GuaText::default());
      let mut sun=Gua::init(vec![1,1,1, 1,1,1],
        String::from("损"),
        &kan,&qian,
&shan, &ze,
      GuaText::default());
      let mut yi=Gua::init(vec![1,1,1, 1,1,1],
        String::from("益"),
        &kan,&qian,
&feng, &lei,
      GuaText::default());
      let mut guai=Gua::init(vec![1,1,1, 1,1,1],
        String::from("夬"),
        &kan,&qian,
&ze, &tian,
      GuaText::default());
      let mut gou=Gua::init(vec![1,1,1, 1,1,1],
        String::from("姤"),
        &kan,&qian,
&tian, &feng,
      GuaText::default());
      let mut cui=Gua::init(vec![1,1,1, 1,1,1],
        String::from("萃"),
        &kan,&qian,
&ze, &di,
      GuaText::default());
      let mut sheng=Gua::init(vec![1,1,1, 1,1,1],
        String::from("升"),
        &kan,&qian,
&di, &feng,
      GuaText::default());
      let mut kun4=Gua::init(vec![1,1,1, 1,1,1],
        String::from("困"),
        &kan,&qian,
&ze, &shui,
      GuaText::default());
      let mut jing=Gua::init(vec![1,1,1, 1,1,1],
        String::from("井"),
        &kan,&qian,
&shui, &feng,
      GuaText::default());
      let mut ge=Gua::init(vec![1,1,1, 1,1,1],
        String::from("革"),
        &kan,&qian,
&ze, &huo,
      GuaText::default());
      let mut ding=Gua::init(vec![1,1,1, 1,1,1],
        String::from("鼎"),
        &kan,&qian,
&huo, &feng,
      GuaText::default());
      let mut zhen6=Gua::init(vec![1,1,1, 1,1,1],
        String::from("震"),
        &kan,&qian,
&lei, &lei,
      GuaText::default());
      let mut gen6=Gua::init(vec![1,1,1, 1,1,1],
        String::from("艮"),
        &kan,&qian,
&shan, &shan,
      GuaText::default());
      let mut jian4=Gua::init(vec![1,1,1, 1,1,1],
        String::from("渐"),
        &kan,&qian,
&feng, &shan,
      GuaText::default());      
      let mut guimei=Gua::init(vec![1,1,1, 1,1,1],
        String::from("归妹"),
        &kan,&qian,
&lei, &ze,
      GuaText::default());
      let mut feng1=Gua::init(vec![1,1,1, 1,1,1],
        String::from("丰"),
        &kan,&qian,
&lei, &huo,
      GuaText::default());
      let mut lvxing=Gua::init(vec![1,1,1, 1,1,1],
        String::from("旅"),
        &kan,&qian,
&huo, &shan,
      GuaText::default());
      let mut xun6=Gua::init(vec![1,1,1, 1,1,1],
        String::from("巽"),
        &kan,&qian,
&feng, &feng,
      GuaText::default());
      let mut dui6=Gua::init(vec![1,1,1, 1,1,1],
        String::from("兑"),
        &kan,&qian,
&ze, &ze,
      GuaText::default());
      let mut huan=Gua::init(vec![1,1,1, 1,1,1],
        String::from("涣"),
        &kan,&qian,
&feng, &shui,
      GuaText::default());
      let mut jie=Gua::init(vec![1,1,1, 1,1,1],
        String::from("节"),
        &kan,&qian,
&shui, &ze,
      GuaText::default());
      let mut zhongfu=Gua::init(vec![1,1,1, 1,1,1],
        String::from("中孚"),
        &kan,&qian,
&feng, &ze,
      GuaText::default());
      let mut xiaoguo=Gua::init(vec![1,1,1, 1,1,1],
        String::from("小过"),
        &kan,&qian,
&lei, &shan,
      GuaText::default());
      let mut jiji=Gua::init(vec![1,1,1, 1,1,1],
        String::from("既济"),
        &kan,&qian,
&shui, &huo,
      GuaText::default());
      let mut weiji=Gua::init(vec![1,1,1, 1,1,1],
        String::from("未济"),
        &kan,&qian,
&huo, &shui,
      GuaText::default());
          
        // let mut gua_ls:Vec<&mut Gua>=vec![&mut qian6,&mut kun6,&mut zhun,
        // &mut meng,&mut xu,&mut song,&mut shi,&mut bi3,&mut xiaoxu,&mut lv,&mut tai,
        // &mut pi,&mut tongren,&mut dayou,&mut qian1,&mut yu,&mut sui,&mut gu,&mut lin,
        // &mut guan,&mut shihe,&mut bi,&mut bo,&mut fu,&mut wuwang,&mut daxu,&mut yi2,
        // &mut daguo,&mut kan6,&mut li6,&mut xian,&mut heng,&mut dun,&mut dazhuang,
        // &mut jin,&mut mingyi,&mut jiaren,&mut kui,&mut jian,&mut xie,&mut sun,&mut yi,
        // &mut guai,&mut gou,&mut cui,&mut sheng,&mut kun,&mut jing,&mut ge,&mut ding,&mut zhen6,
        // &mut gen6,&mut jian4,&mut guimei,&mut feng1,&mut lvxing,&mut xun6,&mut dui6,&mut huan,
        // &mut jie,&mut zhongfu,&mut xiaoguo,&mut jiji,&mut weiji];

        let mut gua_ls:Vec<Gua>=vec![qian6, kun6, zhun,
        meng,xu,song,shi,bi3,xiaoxu,lv,tai,
        pi,tongren, dayou,qian1, yu, sui, gu, lin,
         guan, shihe, bi, bo, fu, wuwang, daxu, yi2,
         daguo, kan6, li6, xian, heng, dun, dazhuang,
         jin, mingyi, jiaren, kui, jian, xie, sun, yi,
         guai, gou, cui, sheng, kun4, jing, ge, ding, zhen6,
         gen6, jian4, guimei, feng1, lvxing, xun6, dui6, huan,
         jie, zhongfu, xiaoguo, jiji, weiji];
      
        println!("length of gua list: {}",gua_ls.len());

    let mut new_gua_ls:Vec<Gua>=Vec::new();
    let rawtexts=GuaText::from_json("./NewStructuredRustJson_RawText.json");
//     println!("{}" ,rawtexts.len());
        let mut i=0;
        for mut gua in gua_ls{
                let gua: Gua<'_>=auto_set_attr_by_xang(gua, &map_xang_gua);
                // gua.text=*rawtexts.get(i).unwrap();
                let mut gua=gua;
                gua.text=rawtexts[i].clone();
                // println!("GUA: {}",& mut gua);
                let gua=gua;
                new_gua_ls.push(gua);
                i=i+1;
        }

        let new_gua_ls=new_gua_ls;
        new_gua_ls
        // ZhouyiModel { subguas: vec![qian,kun,zhen,gen,li,kan,dui,xun], 
        //         xangs: vec![tian,di,lei,shan,huo,shui,ze,feng],
        //          guas: new_gua_ls, map_xang_gua:map_xang_gua }

        // finally save the structed data of zhouyi.
        // === here we cannot save because we use the reference of other struct.
        // let json_str=serde_json::to_string(&new_gua_ls).expect("cannot transfer to json str.");
        // let mut file=File::create("gua_backup.json").expect("creat storage file error");
        // file.write_all(json_str.as_bytes()).expect("write json str to created file error");
        // // Ok(());

}










