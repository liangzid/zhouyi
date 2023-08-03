mod model;
mod divination;
use model::{record_64_Gua,Gua,SubGua,query};
use divination::{coin_divinate,dayanshi_divinate};
use std::{collections::HashMap};
use std::ffi::{CString,CStr,c_char};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[no_mangle]
pub extern "C" fn divinate_c(typ: *const c_char,
			     event: *const c_char)-> *const c_char{
    let dt=unsafe{CStr::from_ptr(typ)}.to_str().unwrap();
    let ev=unsafe{CStr::from_ptr(event)}.to_str().unwrap();
    let res=show_text_divinate(dt,ev);
    let s_res:String=String::from(format!("{:?}",res));
    let return_s=CString::new(s_res).unwrap().as_ptr();
    return_s    
}

#[cfg(feature = "python")]
#[pyfunction]
fn divinate_py(typ:String,event:String) ->String{
    let res=show_text_divinate(&typ,&event);
    let res=String::from(format!("{:?}",res));
    res
}

#[cfg(feature = "python")]
#[pymodule]
fn zhouyipy(_py: Python<'_>, m: &PyModule) -> PyResult<()>{
    m.add_function(wrap_pyfunction!(divinate_py,m)?)?;
    Ok(())
}


pub fn show_text_divinate<'g>(divinate_type:&str,event:&str)->(HashMap<&'g str,String>,Vec<String>,Vec<String>){
    // 1. first init the zhouyi model
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
    

    // 2. then do divination based on the divination type
    let yaos=dayanshi_divinate(event).0;
    if divinate_type=="coin"{
        let yaos=coin_divinate(event);
    }
    else if divinate_type=="dayanshi"{
        let yaos=dayanshi_divinate(event).0;
    }

    let res=query(&zhouyimodel,yaos);
    res
}


pub fn inter_dashiyan_divinate<'g>(event:&str)->(HashMap<&'g str,String>,Vec<String>,Vec<String>){
    // 1. first init the zhouyi model
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
    

    // 2. then do divination based on the divination type
    let yaos=dayanshi_divinate(event).0;
    let res=query(&zhouyimodel,yaos);
    res
}







fn main(){
    let x=show_text_divinate("dayanshi","test");
    println!("{:?}",x);
}
