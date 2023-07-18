use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, Local};
use egui::{
    emath::align, util::History, Color32, FontData, FontDefinitions, FontFamily, TextFormat,
};
use env_logger::fmt::Color;
use rfd::FileDialog;
use serde_json;
use zhouyi::show_text_divinate;
use egui_extras::{Size,StripBuilder};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]// if we add new fields, give them default values when deserializing old state
pub struct ZhouyiUI {
    // settings, meta-information
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
    hm:HashMap<String,String>,

    // Example stuff:
    label: String,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for ZhouyiUI {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,

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
	    hm:HashMap::new(),
        }
    }
}

impl ZhouyiUI {
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

impl eframe::App for ZhouyiUI {
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
	    hm,
        } = self;

        let now = Local::now().format("%F-%T").to_string();
        // let mut place: String = "无关".to_owned();
        // let mut analyse=String::from("");
        // let mut comments:Vec<(String,String)>=vec![];
        if true {
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
                    ui.label("主题");
                    ui.radio_value(is_dark_theme, false, "☀️亮色").clicked();
                    ui.radio_value(is_dark_theme, true, "🌙暗色").clicked();
                });

                // if ui.button("Change Theme").clicked() {
                //     *is_dark_theme = !*is_dark_theme;
                // }
                ui.text_edit_multiline(inps);
                let mut track_divination = false;
                ui.horizontal(|ui| {
                    ui.label("卜法");
                    track_divination |= ui
                        .radio_value(divination_type, "dayanshi".to_owned(), "大衍筮法")
                        .clicked();
                    track_divination |= ui
                        .radio_value(divination_type, "coin".to_owned(), "铜钱爻")
                        .clicked();
                });

                let mut track_lang = true;
                let mut lang = "zh".to_owned();
                ui.horizontal(|ui| {
                    ui.label("语言");
                    track_lang |= ui.radio_value(&mut lang, "zh".to_owned(), "中文").clicked();
                    track_lang |= ui
                        .radio_value(&mut lang, "en".to_owned(), "English")
                        .clicked();
                });
                ui.horizontal(|ui| {
                    ui.label("占卜时刻");
                    let label = egui::widgets::Label::new(now.clone());
                    ui.add(label);
                    ui.ctx().request_repaint();
                });

                ui.horizontal(|ui| {
                    ui.label("占卜地点");
		    ui.horizontal(|ui|{
			ui.set_width(230.0);
			ui.add(egui::TextEdit::singleline(place));
		    });
                });

                let divinate_b = egui::Button::new("卜筮之");
                if ui.add(divinate_b).clicked() {
                    *is_visual = true;
		    // obtain the results of Gua
		    let res = show_text_divinate(divination_type, inps);

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
                ui.horizontal(|ui| {
                    ui.label("卜筮记录管理: ");
                    if ui.button("导出").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            let res = serde_json::to_string(historys).unwrap();
                            std::fs::write(path, res);
                        }
                    }
                    if ui.button("导入").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let content = std::fs::read_to_string(path).unwrap();
                            *historys = serde_json::from_str(&content).unwrap();
                        }
                    }
                    if ui.button("文本方式导入").clicked() {
			*is_open_import=true;
                    }
                    if ui.button("导出为可复制的文本").clicked() {
			*is_open_export=true;
                    }
                    if ui.button("清空").clicked() {
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
                ui.heading("往-卜");

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
                                        ui.label("求卜： ");
                                        ui.colored_label(color_blue.clone(), x.3.clone());
                                    });
                                    ui.separator();
                                    ui.horizontal(|ui| {
                                        ui.label("得卦");
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
                                        ui.label("分析: ");
                                        ui.colored_label(color_blue.clone(), x.6.clone());
                                    });
                                    ui.separator();
                                    // comments
                                    ui.collapsing("批注/应验", |ui| {
                                        egui::ScrollArea::vertical().max_height(100.)
					    .min_scrolled_width(200.0).show(
                                            ui,
                                            |ui| {
                                                if ui.button("记录之").clicked() {
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

		egui::Window::new("通过文本导入").default_width(300.0)
		    .open(is_open_import)
		    .show(ctx,|ui|{
			let mut read_text:String="".to_owned();
			ui.text_edit_multiline(&mut read_text);
			if ui.button("毕").clicked(){
			    *historys = serde_json::from_str(&read_text).unwrap();
			}
		    });
		egui::Window::new("导出为文本").default_width(300.0)
		    .open(is_open_export)
		    .show(ctx,|ui|{

			let scroll = egui::ScrollArea::vertical()
			    .max_height(400.0)
			    .auto_shrink([false;2])
			    .show(ui, |ui| {
			
			let res = serde_json::to_string(historys).unwrap();
			ui.vertical(|ui|{
			    let mut is_copyed=false;
			    if ui.button("复制之").clicked(){
				is_copyed=true;
				use clipboard::{ClipboardContext,ClipboardProvider};
				let mut ctx:ClipboardContext = ClipboardProvider::new().unwrap();
				let res = serde_json::to_string(historys).unwrap();
				ctx.set_contents(res).unwrap();
			    }
			ui.label(res);
			})
			    });
		    });
		
		// pop a new window to add the comments.
		egui::Window::new("记录之")
		    .default_width(320.0)
		    .open(pop_open)
		    .show(ctx, |ui| {
			ui.horizontal(|ui| {
			    ui.code("求卜：");
			    ui.label(inps.clone());
			});
			ui.separator();
			ui.heading(gua_name.clone());
			ui.label(gua.clone()).on_hover_text(
			    duan.clone() + &xang,
			);
			ui.separator();
			for i_yao in 0..yaos.len() {
			    ui.colored_label(
				Color32::from_rgb(3, 111, 4),
				yaos.get(i_yao).unwrap(),
			    )
			    .on_hover_text(
				yaos_xang.get(i_yao).unwrap(),
			    );
			    ui.set_min_height(200.0);
			}
			ui.separator();
			ui.vertical(|ui| {
                    ui.heading("解语");
                    ui.colored_label(color_blue.clone(),analyse.clone());
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
			if ui.button("添加").clicked() {
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
					    egui::Window::new("结果")
                                                        .default_width(340.0)
                                                        .open(is_visual)
                                                        .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.code("求卜：");
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
                    ui.heading("解易");
                    ui.label("  1. 以卦意察之\n  2. 以诸爻审之\n  3. 写下预言");
                    ui.label("回车确认");
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
