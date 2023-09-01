use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, Local};
use egui::{
    emath::align, util::History, Color32, FontData, FontDefinitions, FontFamily, TextFormat,
};
use env_logger::fmt::Color;

use rfd;
use serde_json;
use zhouyi::show_text_divinate;
use egui_extras::{Size,StripBuilder};

use crate::communicate::{query_login};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]// if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // settings, meta-information
    lang:String,
    divination_type: String,
    is_dark_theme: bool,

    // contents of the zhouyi
    gua_name: String,
    gua: String,
    duan: String,
    xang: String,
    xang_up: String,
    xang_bottom: String,
    subgua_up: String,
    subgua_bottom: String,
    yaos: Vec<String>,
    yaos_xang: Vec<String>,

    // contents of user inputs.
    inps: String,
    is_visual: bool,
    historys: Vec<(
        HashMap<String, String>,
        Vec<String>,
        Vec<String>,
        String,                // inps
        String,                // time
        String,                //place
        String,                // analysis
        Vec<(String, String)>, // comments with (text, time)
    )>,

    place: String,
    analyse: String,
    comments: Vec<(String, String)>,
    temp_comment: String,
    pop_open:bool,
    current_point:usize,
    is_open_import:bool,
    is_open_export:bool,
    is_open_login:bool,
    is_open_signup:bool,

     //account related
     email:String,
     pwd:String,
     pwd2:String,

    login_state:i8,
    user_type:String,
    activation_state:String,
    utype_ls:Vec<String>,
    activation_ls:Vec<String>,
    is_open_activate_help:bool,
     
    hm:HashMap<String,String>,

    // Example stuff:
    label: String,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,

        lang:"en".to_owned(),
	    divination_type:"dayanshi".to_owned(),
	    is_dark_theme:false,
	    gua_name: "乾".to_owned(),
	    gua: "乾，元亨，利貞。".to_owned(),
	    duan:"《彖》曰：大哉乾元，萬物資始，乃統天。雲行雨施，品物流形，大明終始，六位時成，時乘六龍以御天。乾道變化，各正性命，保合大和，乃利貞。".to_owned(),
	    xang:"《象》曰：天行健，君子以自強不息。".to_owned(),
	    xang_up:"天".to_owned(),
	    xang_bottom:"天".to_owned(),
	    subgua_up:"乾".to_owned(),
	    subgua_bottom:"乾".to_owned(),
	    yaos:vec![
            "初九：潛龍勿用。".to_owned(),
            "九二：見龍再田，利見大人。".to_owned(),
            "九三：君子終日乾乾，夕惕若，厲，無咎。".to_owned(),
            "九四：或躍在淵，無咎。".to_owned(),
            "九五：飛龍在天，利見大人。".to_owned(),
            "上九：亢龍有悔。".to_owned(),
            "用九：見群龍無首，吉。".to_owned()
            ],
	    yaos_xang:vec!["《象》曰：潛龍勿用，陽在下也。".to_owned(),
            "《象》曰：見龍在田，德施普也。".to_owned(),
            "《象》曰：終日乾乾，反復道也。".to_owned(),
            "《象》曰：或躍在淵，進無咎也。".to_owned(),
            "《象》曰：飛龍在天，大人造也。".to_owned(),
            "《象》曰：亢龍有悔，盈不可久也。".to_owned(),
            "《象》曰：用九，天德不可為首也。".to_owned()],
	    inps:"明天的我会快乐么".to_owned(),
        is_visual:false,
        historys:vec![],
	    place:"无关".to_owned(),
	    analyse:"".to_owned(),
	    comments:vec![],
	    temp_comment:"".to_owned(),
	    pop_open:false,
	    current_point:0,
	    is_open_import:false,
	    is_open_export:false,
        is_open_login:true,
        is_open_signup:false,

        //account related
        email:"".to_owned(),
        pwd:"".to_owned(),
        pwd2:"".to_owned(),
	    login_state:0,
	    user_type:"nothing".to_owned(),
	    activation_state:"not_activate".to_owned(),
	    utype_ls:vec!["nothing".to_owned(),"regular".to_owned(),
			  "VIP1".to_owned()],
	    activation_ls:vec!["not_activate".to_owned(),
			       "activate".to_owned(),
	    ],
        is_open_activate_help:false,
	    hm:HashMap::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        // load CJK fonts.
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "wenquan".to_owned(),
            FontData::from_static(include_bytes!("../data/wenquan.ttf")),
        );

        // set priority
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "wenquan".to_owned());

        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("wenquan".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,

            lang,
            divination_type,
            is_dark_theme,
            gua_name,
            gua,
            duan,
            xang,
            xang_up,
            xang_bottom,
            subgua_up,
            subgua_bottom,
            yaos,
            yaos_xang,
            inps,
            is_visual,
            historys,
            place,
            analyse,
            comments,
            temp_comment,
	    pop_open,
	    current_point,
	    is_open_import,
	    is_open_export,
        is_open_login,
        is_open_signup,

        //account related
        email,
        pwd,
        pwd2,
	    login_state,
	    user_type,
	    activation_state,

        utype_ls,
        activation_ls,
        is_open_activate_help,

	    hm,
        } = self;
        let now = Local::now().format("%F-%T").to_string();
        // let mut place: String = "无关".to_owned();
        // let mut analyse=String::from("");
        // let mut comments:Vec<(String,String)>=vec![];
        if true {
	    let tt_login= match lang.as_str(){
		"zh"=>"登录，以同步您的私有信息",
		_=>"Login to async your information!",
	    };
            egui::Window::new(tt_login).default_width(300.0)
		    .open(is_open_login)
		    .show(ctx,|ui|{
                
                // ui.heading("Login to your account!");
                ui.horizontal(|ui|{
                    ui.label("Email:");
                    ui.text_edit_singleline(email);
                });
                ui.horizontal(|ui|{
                    ui.label("Password:");
                    password_ui(ui,pwd)
                });
			match lang.as_str(){
			    "zh"=>ui.small("不少于8个字符，仅数字、字母与特殊符号。"),
			    _=>ui.small("no less than 8 characters."),
			};
            ui.horizontal(|ui|{
		let tt_fgt=match lang.as_str(){
		    "zh"=>"忘记密码？",
			_=>"I forget the password",
		};
                if ui.button(tt_fgt).clicked(){
                    let _=1;
                }
		let tt_lgi=match lang.as_str(){"zh"=>"登录",_=>"Login."};
                if ui.button(tt_lgi).clicked(){
                    let _x=1;
		    let res=query_login(email,pwd);
		    if res.0=="Ok"{
			*login_state=res.1.parse().unwrap();
			*user_type=res.2;
			*activation_state=res.3;
		    }
		    else if res.0=="pwd_error"{
    			ui.label("Incorrect emails or passwords.");
		    }
		    else{
	    		ui.label("Incorrect emails or passwords.");
		    }
                }
		let tt_sgu=match lang.as_str(){"zh"=>"注册账号",_=>"No account? Sign Up"};
                if ui.button(tt_sgu).clicked(){
                    // *is_open_login=false;
                    *is_open_signup=true;
                }
            });
		    });

	    let tt_sguu=match lang.as_str(){"zh"=>"注册，以同步您的私有信息",
				   _=>"Sign up, to sync your information"};
            egui::Window::new(tt_sguu).default_width(300.0)
		    .open(is_open_signup)
		    .show(ctx,|ui|{
                
                // ui.heading("Sign Up Now!");
                ui.horizontal(|ui|{
		        match lang.as_str(){
			    "zh"=>ui.label("邮箱："),
			_   =>ui.label("Email:"),
		        };
                    ui.text_edit_singleline(email);
                });
                ui.horizontal(|ui|{
		    match lang.as_str(){
			"zh"=>ui.label("密码："),
			_=>ui.label("Password:"),
		    };
                    
                    password_ui(ui,pwd)
                });
		match lang.as_str(){
		    "zh"=>ui.small("不少于8个字符，仅数字、字母与特殊符号。"),
		    _=>ui.small("no less than 8 characters."),
		};
            
                ui.horizontal(|ui|{
		    match lang.as_str(){
"zh"=>ui.label("再次输入:"),
_=>ui.label("Password Again:"),
		    };
                    
                    password_ui(ui,pwd2)
                });

                if pwd!=pwd2{
		    let tt_pic=match lang.as_str(){
			"zh"=>"密码不一致",
			_=>"Password inconsistant"
		    };
                    ui.colored_label(egui::Color32::RED,
                         tt_pic);
                }

            ui.horizontal(|ui|{
		let tt_sgu_b=match lang.as_str(){"zh"=>"注册",_=>"Now Sign Up!"};
		let tt_sgu_b_ah=match lang.as_str(){"zh"=>"转至登录页面",_=>"Already have a account? Login."};
                if ui.button(tt_sgu_b).clicked(){
                    let _x=1;
                }
                if ui.button(tt_sgu_b_ah).clicked(){
                    *is_open_login=true;
                    // *is_open_signup=false;
                }
            });
		    });

            egui::CentralPanel::default().show(ctx, |ui| {

                let mut color_blue: Color32;
                if *is_dark_theme {
                    ctx.set_visuals(egui::Visuals::dark());
                    color_blue = Color32::from_rgb(255, 255, 1);
                } else {
                    ctx.set_visuals(egui::Visuals::light());
                    color_blue = Color32::from_rgb(33, 24, 68);
                }
                let (default_color, strong_color) = if ui.visuals().dark_mode {
                    (Color32::LIGHT_GRAY, Color32::WHITE)
                } else {
                    (Color32::DARK_GRAY, Color32::BLACK)
                };
                ui.horizontal(|ui| {
		    match lang.as_str(){"zh"=>{
			ui.label("主题");
		    }
			       _=>{ui.label("Theme:");}
		    }
		    ui.radio_value(is_dark_theme, false, "☀️").clicked();
		    ui.radio_value(is_dark_theme, true, "🌙").clicked();
                });

                // if ui.button("Change Theme").clicked() {
                //     *is_dark_theme = !*is_dark_theme;
                // }
                ui.text_edit_multiline(inps);
                let mut track_divination = false;
                ui.horizontal(|ui| {
		    match lang.as_str(){"zh"=>ui.label("卜法"),_=>ui.label("Divination method")};
		    match lang.as_str(){
			"zh"=>{
                    track_divination |= ui
                        .radio_value(divination_type, "dayanshi".to_owned(), "大衍筮法")
                        .clicked();
                    track_divination |= ui
                        .radio_value(divination_type, "coin".to_owned(), "铜钱爻")
                        .clicked();
			}
			_=>{
                    track_divination |= ui
                        .radio_value(divination_type, "dayanshi".to_owned(), "Dayanshi-method")
                        .clicked();
                    track_divination |= ui
                        .radio_value(divination_type, "coin".to_owned(), "Coin-method")
                        .clicked();

			}}
                });

                let mut track_lang = true;
                // let mut lang.as_str() = "zh".to_owned();
                ui.horizontal(|ui| {
		    match lang.as_str() {"zh"=>ui.label("语言"),_=>ui.label("Language"),};
                    track_lang |= ui.radio_value(lang, "zh".to_owned(), "中文").clicked();
                    track_lang |= ui
                        .radio_value(lang, "en".to_owned(), "English")
                        .clicked();
                });
                ui.horizontal(|ui| {
		    match lang.as_str() {"zh"=>ui.label("占卜时刻"),_=>ui.label("Divination Time"),};
                    let label = egui::widgets::Label::new(now.clone());
                    ui.add(label);
                    ui.ctx().request_repaint();
                });

                ui.horizontal(|ui| {
		    match lang.as_str() {"zh"=>ui.label("占卜地点"),_=>ui.label("Divination Position"),};
		    ui.horizontal(|ui|{
			ui.set_width(230.0);
			ui.add(egui::TextEdit::singleline(place));
		    });
                });

                let divinate_b =match lang.as_str(){
		    "zh"=>egui::Button::new("卜筮之"),
		    _=>egui::Button::new("Divinate it!"),
		} ;
                if ui.add(divinate_b).clicked() {
                    *is_visual = true;
		    // obtain the results of Gua
		    let res = show_text_divinate(divination_type,
                 inps);

		    // at current results to history
		    *hm = res
			.0
			.iter()
			.map(|(k, v)| (String::from(*k), v.clone()))
			.collect();
                    // update the divination results
                    *gua_name = res.0.get("name").unwrap().to_string();
                    *gua = res.0.get("gua").unwrap().to_string();
                    *duan = res.0.get("duan").unwrap().to_string();
                    *xang = res.0.get("xang").unwrap().to_string();
                    *xang_up = res.0.get("xang_top").unwrap().to_string();
                    *xang_bottom = res.0.get("xang_bottom").unwrap().to_string();
                    *subgua_up = res.0.get("gua_top").unwrap().to_string();
                    *subgua_bottom = res.0.get("gua_bottom").unwrap().to_string();
                    *yaos = res.1;
                    *yaos_xang = res.2;

                }

                ui.separator();
                // add the export and import button.
		match lang.as_str() {"zh"=>ui.heading("卜筮记录管理"),_=>ui.heading("Records Management"),};
                ui.horizontal(|ui|{
		    match lang.as_str() {"zh"=>ui.label("当前状态："),_=>ui.label("Current State"),};
		    if (*login_state).eq(&0){
			match lang.as_str() {"zh"=>ui.label("未登录"),
				    _=>ui.label("Visitor, not logged in"),}
		    }
		    else{
			match lang.as_str() {"zh"=>ui.label(format!("账户 {} 登录.",email)),
				    _=>ui.label(format!("User {} logged in.",email)),}
		    }
                });

		match lang.as_str(){
		    "zh" =>{

                ui.label("卜筮：✔");
                ui.label("数据于当前设备缓存：✔");
                if activation_state=="not_activate"{
                    ui.label("数据导出/导入：✖");
                    ui.label("跨设备云端存储：✖");
                    ui.label("AI检索：✖");
                }
                else{
                    ui.label("数据导出/导入：✔");
                    ui.label("跨设备云端存储：✔");
                    ui.label("AI检索：马上推出");
                }
		    }

		    _=>{

                ui.label("Divination：✔");
                ui.label("Store history in local deivce：✔");
                if activation_state=="not_activate"{
                    ui.label("Records Import/Export：✖");
                    ui.label("Cloud Storage and sync：✖");
                    ui.label("AI-based Retrieval：✖");
                }
                else{
                    ui.label("Records Import/Export：✔");
                    ui.label("Cloud Storage and sync：✔");
                    ui.label("AI-based Retrieval：comming soon");
                }
		    }

		}


                ui.horizontal(|ui|{
		    match lang.as_str(){
			"zh"=>{

                    if ui.button("登录").clicked(){
                        *is_open_login=true;
                    }
                    if ui.button("注册").clicked(){
                        *is_open_signup=true;
                    }
		    if ui.button("注销").clicked(){
			*login_state=0;
			*user_type="nothing".to_owned();
			*activation_state="not_activate".to_owned();
		    }

			}
			_=>{
                    if ui.button("Log in").clicked(){
                        *is_open_login=true;
                    }
                    if ui.button("Sign up").clicked(){
                        *is_open_signup=true;
                    }
		    if ui.button("Quit account").clicked(){
			*login_state=0;
			*user_type="nothing".to_owned();
			*activation_state="not_activate".to_owned();
		    }

			}
		    }
                });
                ui.horizontal(|ui| {
                    
		    #[cfg(not(target_arch = "wasm32"))]
		    let tt_export=match lang.as_str(){
			"zh"=>"导出",
			_=>"export"
		    };
                    if ui.button(tt_export).clicked() {
                        if activation_state=="not_activate"{
                            *is_open_activate_help=true;
                        }
                        else{
                            if let Some(path) = rfd::FileDialog::new().save_file() {
                                let res = serde_json::to_string(historys).unwrap();
                                std::fs::write(path, res);
                            }
                        }
                    }
                    egui::Window::new("Notion").open(is_open_activate_help)
                    .show(ctx, |ui| {
                        ui.label("Sorry, you have no permission to do this operation!");
                        ui.hyperlink_to("Update your account now!", "https://liangzid.github.io/");
                    });

		    #[cfg(not(target_arch = "wasm32"))]
		    let tt_import=match lang.as_str(){
			"zh"=>"导入",
			_=>"import"
		    };
                    if ui.button(tt_import).clicked() {
                        if activation_state=="not_activate"{
                            *is_open_activate_help=true;    
                        }
                        else{
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                let content = std::fs::read_to_string(path).unwrap();
                                *historys = serde_json::from_str(&content).unwrap();
                            }
                        }
                    }

		    let tt_imports=match lang.as_str(){
			"zh"=>"文本方式导入",
			_=>"import from string"
		    };
                    if ui.button(tt_imports).clicked() {
                        if activation_state=="not_activate"{
                            *is_open_activate_help=true;    
                        }
                        else{
                            *is_open_import=true;
                        }
                    }
		    let tt_exports=match lang.as_str(){
			"zh"=>"导出为可复制的文本",
			_=>"export as string"
		    };
                    if ui.button(tt_exports).clicked() {
                        if activation_state=="not_activate"{
                            *is_open_activate_help=true;    
                        }
                        else{
			                *is_open_export=true;
                        }
                    }
		    let tt_clear=match lang.as_str(){
			"zh"=>"清空",
			_=>"clear"
		    };
                    if ui.button(tt_clear).clicked() {
                        *historys = vec![];
                        *comments = vec![];
			*place = "".to_owned();
			*analyse = "".to_owned();
			*temp_comment = "".to_owned();
			*pop_open = false;
			*current_point = 0;
			*is_open_import = false;
			*is_open_export = false;
			*is_visual=false;
			
                    }
                });

                ui.separator();

		let tt_h=match lang.as_str(){
		    "zh"=>"往-卜",
		    _=>"History"
		};
                ui.heading(tt_h);

                let scroll = egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let mut newhis = historys.clone();
                            newhis.reverse();
			    let mut i_x:u8=0;
                            for x in newhis {
                                let mut t_job = egui::text::LayoutJob::default();
                                t_job.append(
                                    &(x.0.get("name").unwrap().clone() + "   "),
                                    0.0,
                                    TextFormat {
                                        color: strong_color,
                                        ..Default::default()
                                    },
                                );

                                t_job.append(
                                    &x.4.clone(),
                                    0.0,
                                    TextFormat {
                                        color: default_color,
                                        background: Color32::from_rgb(239, 83, 80),
                                        ..Default::default()
                                    },
                                );

                                t_job.append(
                                    "   ",
                                    0.0,
                                    TextFormat {
                                        color: strong_color,
                                        ..Default::default()
                                    },
                                );

                                t_job.append(
                                    &x.5.clone(),
                                    0.0,
                                    TextFormat {
                                        color: default_color,
                                        background: Color32::from_rgb(124, 179, 66),
                                        ..Default::default()
                                    },
                                );

                                ui.collapsing(t_job, |ui| {
                                    // question
                                    ui.horizontal(|ui| {
					match lang.as_str(){"zh"=>ui.label("求卜： "),
					_=>ui.label("Event divanated:"),};
                                        
                                        ui.colored_label(color_blue.clone(), x.3.clone());
                                    });
                                    ui.separator();
                                    ui.horizontal(|ui| {
					match lang.as_str(){"zh"=>ui.label("得卦"),
					_=>ui.label("Results of GUA:"),};
                                        ui.label(x.0.get("name").unwrap().clone())
                                            .on_hover_cursor(egui::CursorIcon::Help)
                                            .on_hover_ui(|ui| {
                                                ui.heading(x.0.get("name").unwrap().clone());
                                                ui.label(x.0.get("gua").unwrap().clone());
                                                ui.colored_label(
                                                    Color32::from_rgb(128, 140, 255),
                                                    x.0.get("duan").unwrap().clone(),
                                                );
                                                ui.colored_label(
                                                    Color32::from_rgb(128, 128, 12),
                                                    x.0.get("xang").unwrap().clone(),
                                                );

                                                ui.separator();

                                                for i_yao in 0..yaos.len() {
                                                    ui.colored_label(
                                                        Color32::from_rgb(3, 111, 4),
                                                        x.1.clone().get(i_yao).unwrap(),
                                                    );
                                                    ui.colored_label(
                                                        Color32::from_rgb(111, 12, 4),
                                                        x.2.clone().get(i_yao).unwrap(),
                                                    );
                                                    ui.set_min_height(300.0);
                                                }
                                            });
                                    });
                                    ui.separator();
                                    // analysis
                                    ui.horizontal(|ui| {
					match lang.as_str(){"zh"=>ui.label("分析："),
					_=>ui.label("Analysis:"),};
                                        ui.colored_label(color_blue.clone(), x.6.clone());
                                    });
                                    ui.separator();
                                    // comments
				    let tt_com=match lang.as_str(){
					"zh"=>"批注/应验",
					_=>"Comments/Whether comes true"
				    };
                                    ui.collapsing(tt_com, |ui| {
                                        egui::ScrollArea::vertical().max_height(100.)
					    .min_scrolled_width(200.0).show(
                                            ui,
                                            |ui| {

						// comments
						let tt_com=match lang.as_str(){
						    "zh"=>"记录之",
						    _=>"Record it now."
						};
                                                if ui.button(tt_com).clicked() {
						    *pop_open=true;
						    *current_point=(*historys).len()-1-(i_x as usize);
                                                }
                                                ui.vertical(|ui| {
                                                    let mut xx = x.7.clone();
                                                    xx.reverse();
                                                    for com in xx {
                                                        ui.collapsing(com.1, |ui| {
                                                            ui.label(com.0);
                                                        });
                                                    }
                                                });
                                            },
                                        );
                                    })
                                });
                                // // if gua_head.clicked(){
                                // //     *is_visual=true;
                                // // }
                                // ui.label((x.3).clone());
                                // ui.label(x.4.clone());
                                // ui.label(x.5.clone());
				i_x+=1
                            }
                        });
                    });

		let tt_done=match lang.as_str(){"zh"=>"毕",_=>"Done."};
		let tt_cp=match lang.as_str(){"zh"=>"复制之",_=>"Copy it."};
        let tt_imports=match lang.as_str(){
			"zh"=>"文本方式导入",
			_=>"import from string"
		    };
		egui::Window::new(tt_imports).default_width(300.0)
		    .open(is_open_import)
		    .show(ctx,|ui|{
			let mut read_text:String="".to_owned();
			ui.text_edit_multiline(&mut read_text);
			if ui.button(tt_done).clicked(){
			    *historys = serde_json::from_str(&read_text).unwrap();
			}
		    });
            let tt_exports=match lang.as_str(){
                "zh"=>"导出为可复制的文本",
                _=>"export as string"
                };
		egui::Window::new(tt_exports).default_width(300.0)
		    .open(is_open_export)
		    .show(ctx,|ui|{

			let scroll = egui::ScrollArea::vertical()
			    .max_height(400.0)
			    .auto_shrink([false;2])
			    .show(ui, |ui| {
			
			let res = serde_json::to_string(historys).unwrap();
			ui.vertical(|ui|{
			    let mut is_copyed=false;
			    if ui.button(tt_cp).clicked(){
				is_copyed=true;
				use clipboard::{ClipboardContext,ClipboardProvider};
				let mut ctx:ClipboardContext = ClipboardProvider::new().unwrap();
				let res = serde_json::to_string(historys).unwrap();
				// ctx.set_contents(res).unwrap();
				ui.output_mut(|o| o.copied_text = res.to_string());
			    }
			// ui.label(res);
			    code_view_ui(ui,&res);
			})
			    });
		    });
		
						let tt_com=match lang.as_str(){
						    "zh"=>"记录之",
						    _=>"Record it now."
						};
						let tt_qiubu=match lang.as_str(){
						    "zh"=>"求卜：",
						    _=>"Things divinated:"
						};
		// pop a new window to add the comments.
		egui::Window::new(tt_com)
		    .default_width(320.0)
		    .open(pop_open)
		    .show(ctx, |ui| {
			ui.horizontal(|ui| {
			    ui.code(tt_qiubu);
			    ui.label((historys.get(*current_point as usize).unwrap()).3.clone());
			});
			ui.separator();
            let temp_map=historys.get(*current_point as usize)
            .unwrap().0.clone();
			ui.heading(temp_map.get("name").unwrap().clone());
			ui.label(temp_map.get("gua").unwrap().clone()).on_hover_text(
			    temp_map.get("duan").unwrap().clone() +
                 &temp_map.get("xang").unwrap().clone(),
			);
			ui.separator();
            let temp_yaos=historys.get(*current_point).unwrap().1.clone();
            let temp_yxs=historys.get(*current_point).unwrap().2.clone();
			for i_yao in 0..temp_yaos.len() {
			    ui.colored_label(
				Color32::from_rgb(3, 111, 4),
				temp_yaos.get(i_yao).unwrap(),
			    )
			    .on_hover_text(
				temp_yxs.get(i_yao).unwrap(),
			    );
			    ui.set_min_height(200.0);
			}
			ui.separator();
			ui.vertical(|ui| {
		match lang.as_str(){"zh"=>ui.heading("解语"), _=>ui.heading("Your analysis:"),};
                    
                    ui.colored_label(color_blue.clone(),historys.get(*current_point).unwrap()
                    .6.clone());
                });
		    ui.separator();
		    ui.vertical(|ui| {
			let mut xx = (*(historys.get(*current_point as usize).unwrap())).7.clone();
			xx.reverse();
			for com in xx {
			    ui.collapsing(com.1, |ui| {
				ui.label(com.0);
			    });
			}
		    });
		    ui.separator();
		    ui.horizontal(|ui| {
			ui.text_edit_singleline(
			    temp_comment,
			);
			let tt_add=match lang.as_str(){"zh"=>"添加",_=>"Add"};
			if ui.button(tt_add).clicked() {
			    (*historys)[*current_point as usize].7.push((
				temp_comment.clone(),
				now.clone(),
			    ));
			}
		    });
		});

                // ui.heading("eframe template");
                // ui.hyperlink("https://github.com/emilk/eframe_template");
                // ui.add(egui::github_link_file!(
                //     "https://github.com/emilk/eframe_template/blob/master/",
                //     "Source code."
                // ));
                // egui::warn_if_debug_build(ui);
            });
        }

			let tt_res=match lang.as_str(){"zh"=>"结果",_=>"Results"};
					    egui::Window::new(tt_res)
                                                        .default_width(340.0)
                                                        .open(is_visual)
                                                        .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    
                    let tt_qiubu=match lang.as_str(){
                        "zh"=>"求卜：",
                        _=>"Things divinated:"
                    };
                    ui.code(tt_qiubu);
                    ui.label(inps.clone());
                });
                ui.separator();
                ui.heading(gua_name);
                ui.label(gua.clone());
                ui.colored_label(Color32::from_rgb(128, 140, 255), duan);
                ui.colored_label(Color32::from_rgb(128, 128, 12), xang);

                ui.separator();

                for i_yao in 0..yaos.len() {
                    ui.colored_label(Color32::from_rgb(3, 111, 4), yaos.get(i_yao).unwrap());
                    ui.colored_label(Color32::from_rgb(111, 12, 4), yaos_xang.get(i_yao).unwrap());
                    ui.set_min_height(300.0);
                }
                ui.separator();
                ui.vertical(|ui| {
		    match lang.as_str(){
			"zh"=>{
                    ui.heading("解易");
                    ui.label("  1. 以卦意察之\n  2. 以诸爻审之\n  3. 写下预言");
			}
			_=>{
                    ui.heading("Analyse it:");
                    ui.label("  1. Observe it in the context of the oracle's meaning.\n  2. Examine it by the hexagrams.\n  3. Write down your prophecy.");

			}

		    }
                    // ui.label("例：\n  1. ")
                    let response=ui.add(egui::TextEdit::multiline(analyse));
		    if response.lost_focus(){
                    (*historys).push((
                        hm.clone(),
                        yaos.clone(),
                        yaos_xang.clone(),
                        inps.clone(),
                        String::from(now.clone()),
                        place.clone(),
                        analyse.clone(),
                        comments.clone(),
                    ));

		    }

                });
                // if ui.button("回返之").clicked() {
                //     *is_visual = false;
                // }
            });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
use egui::text::LayoutJob;

/// View some code with syntax highlighting and selection.
pub fn code_view_ui(ui: &mut egui::Ui, mut code: &str) {
    ui.add(
        egui::TextEdit::multiline(&mut code)
            .font(egui::TextStyle::Monospace) // for cursor height
            .code_editor()
            .desired_rows(1)
            .lock_focus(true),
    );
}



#[allow(clippy::ptr_arg)] // false positive
pub fn password_ui(ui: &mut egui::Ui, password: &mut String) -> egui::Response {
    // This widget has its own state — show or hide password characters (`show_plaintext`).
    // In this case we use a simple `bool`, but you can also declare your own type.
    // It must implement at least `Clone` and be `'static`.
    // If you use the `persistence` feature, it also must implement `serde::{Deserialize, Serialize}`.

    // Generate an id for the state
    let state_id = ui.id().with("show_plaintext");

    // Get state for this widget.
    // You should get state by value, not by reference to avoid borrowing of [`Memory`].
    let mut show_plaintext = ui.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

    // Process ui, change a local copy of the state
    // We want TextEdit to fill entire space, and have button after that, so in that case we can
    // change direction to right_to_left.
    let result = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        // Toggle the `show_plaintext` bool with a button:
        let response = ui
            .add(egui::SelectableLabel::new(show_plaintext, "👁"))
            .on_hover_text("Show/hide password");

        if response.clicked() {
            show_plaintext = !show_plaintext;
        }

        // Show the password field:
        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::singleline(password).password(!show_plaintext),
        );
    });

    // Store the (possibly changed) state:
    ui.data_mut(|d| d.insert_temp(state_id, show_plaintext));

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, …) and maybe show a tooltip:
    result.response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(…)`
/// Password entry field with ability to toggle character hiding.
///
/// ## Example:
/// ``` ignore
/// ui.add(password(&mut my_password));
/// ```
pub fn password(password: &mut String) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| password_ui(ui, password)
}

// pub fn url_to_file_source_code() -> String {
//     format!("https://github.com/emilk/egui/blob/master/{}", file!())
// }
