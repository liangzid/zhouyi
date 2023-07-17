use std::io::BufRead;

use rand::Rng;

type yao6 = Vec<u8>;

fn main() {
    // let res=coin_divinate();
    let res = dayanshi_divinate("").0;
    println!("{:?}", res)
}

pub fn coin_divinate(event:&str) -> yao6 {
    let mut rng = rand::thread_rng();
    let mut res_vec: Vec<u8> = Vec::new();

    for times in 0..6 {
        let x: u8 = rng.gen_range(0..2);
        res_vec.push(x);
    }
    // println!("{:?}",res_vec);
    res_vec.reverse();
    res_vec
}

// reference: https://zhuanlan.zhihu.com/p/409348265
pub fn dayanshi_divinate(event:&str) -> (yao6,Vec<Vec<(u8,u8)>>,Vec<String>) {
    let mut rng = rand::thread_rng();
    let mut res_vec: Vec<u8> = Vec::new();
    let mut str_ls:Vec<String>=Vec::new();
    let mut detailed_ls:Vec<Vec<(u8,u8)>>=Vec::new();

    for i_yao in 0..6 {
        // println!("----------");
        // 大衍 50
        let dayan: u8 = 50;
        // 奢草 49
        let mut shecao: u8 = 49;

        let mut this_detailed_ls:Vec<(u8,u8)>=Vec::new();

        // A: to obtain one yao.

        for repet_times in 0..3 {
            // A.1. 分而为二以象两

            let mut split1: u8 = rng.gen_range(0..shecao+1);
            let mut split2: u8 = shecao - split1;
            // println!("分二 {},{}",split1,split2);

            // A.2. 挂一以象三
            if repet_times==0 {
                let mut selected_index: u8 = rng.gen_range(0..2);
            if split1==1 || split1==0 {
                selected_index=1;
            }
            else if split2==1 || split2==0 {
                selected_index=0;
            }

            if selected_index == 0{
                split1 = split1 - 1; // 冲
            } else {
                split2 = split2 - 1;
            }
            }
            
            // println!("挂一之后：{},{}",split1,split2);

            // A.3. 数之以四以象四时 (揲四)
            
            // A.3.1 the phenomenon of 覍
            if split1==0{
                this_detailed_ls.push((0,4));
                split2-=4;
            }
            else if split2==0 {
                this_detailed_ls.push((4,0));
                split1-=4;
                
            }
            // A.3.2 the phenomenon of 4-4, the old yin
            else if split1%4==0{
                this_detailed_ls.push((4,4));
                // println!("-=4: {},{}",split1,split2);
            // A.4. 归奇于指以象闰
                split1-=4;
                split2-=4;
            }
            // A.3.3 others
            else {
                let yu1=split1%4;
                let yu2=split2%4;
                this_detailed_ls.push((yu1,yu2));

            // A.4. 归奇于指以象闰
                split1-=yu1;
                split2-=yu2;
            }
            // println!("归奇后: {} {}",split1,split2);
            shecao = split1 + split2;
            // println!("shecao remain: {}",shecao);
        }
        
        let mut xang_ls:Vec<u8>=vec![];
        for element in &this_detailed_ls{
            if element.0!=element.1{
                xang_ls.push(1); // yong
            }
            else {
                xang_ls.push(0); // yin
            }
        }
        this_detailed_ls.reverse();
        detailed_ls.push(this_detailed_ls);
        let res:u8=xang_ls.iter().sum();
        if res==0{
            str_ls.push(String::from("老阴"));
            res_vec.push(0);
        }
        else if res==1{
            str_ls.push(String::from("少阳"));
            res_vec.push(1);
        }
        else if res==2 {
            str_ls.push(String::from("少阴"));
            res_vec.push(0);            
        }
        else if res==3 {
            str_ls.push(String::from("老阳"));
            res_vec.push(1);
        }
        
    }
    res_vec.reverse();
    detailed_ls.reverse();
    str_ls.reverse();
    (res_vec,detailed_ls,str_ls)
}
