use eframe::egui;
use std::sync::Arc;
use std::path::PathBuf;
use crate::queue::{TaskQueue, TaskStatus};
use crate::converter::{self, ConversionMode};
use crate::file_ops;
use rfd::FileDialog;
use rayon::prelude::*;

use serde::{Serialize, Deserialize};

const APP_KEY: &str = "tc_sc_converter_config";

struct UIStrings {
    file_menu: &'static str,
    add_files: &'static str,
    clear_queue: &'static str,
    about_menu: &'static str,
    app_version: &'static str,
    app_feature: &'static str,
    app_developer: &'static str,
    input_label: &'static str,
    input_hint: &'static str,
    mode_t2s: &'static str,
    mode_s2t: &'static str,
    replace_mode: &'static str,
    new_mode: &'static str,
    auto_convert: &'static str,
    auto_process: &'static str,
    btn_convert_text: &'static str,
    btn_start_batch: &'static str,
    queue_label: &'static str,
    col_name: &'static str,
    col_size: &'static str,
    col_path: &'static str,
    col_status: &'static str,
    col_time: &'static str,
    status_pending: &'static str,
    status_processing: &'static str,
    status_completed: &'static str,
    drag_feedback: &'static str,
    tip_t2s: &'static str,
    tip_s2t: &'static str,
    tip_replace: &'static str,
    tip_new: &'static str,
    tip_auto_convert: &'static str,
    tip_auto_process: &'static str,
    tip_convert_text: &'static str,
    tip_start_batch: &'static str,
    tip_clear_queue: &'static str,
    btn_copy: &'static str,
    btn_save_as: &'static str,
    tip_copy: &'static str,
    tip_save_as: &'static str,
    toast_copy_success: &'static str,
    toast_save_success: &'static str,
}

const ZH_HANS: UIStrings = UIStrings {
    file_menu: "文件",
    add_files: "添加文件",
    clear_queue: "清空队列",
    about_menu: "关于",
    app_version: "简繁转换工具 v1.0.0",
    app_feature: "功能：支持批量简繁中文转换",
    app_developer: "开发者：Yssssssss",
    input_label: "文本输入与实时预览：",
    input_hint: "在此输入文本或拖入文件...",
    mode_t2s: "繁体 -> 简体",
    mode_s2t: "简体 -> 繁体",
    replace_mode: "替换模式",
    new_mode: "新建模式",
    auto_convert: "自动转换预览",
    auto_process: "拖入即转换",
    btn_convert_text: "立即转换文本",
    btn_start_batch: "开始批量转换",
    queue_label: "处理队列：",
    col_name: "文件名",
    col_size: "大小",
    col_path: "路径",
    col_status: "状态",
    col_time: "耗时",
    status_pending: "等待中",
    status_processing: "转换中...",
    status_completed: "完成",
    drag_feedback: "释放文件以导入",
    tip_t2s: "将输入的文字或队列中的文件从繁体中文转换为简体中文",
    tip_s2t: "将输入的文字或队列中的文件从简体中文转换为繁体中文",
    tip_replace: "直接修改原始文件，操作不可逆，请谨慎使用",
    tip_new: "保留原始文件，在同目录下创建带时间戳的新文件",
    tip_auto_convert: "在文本框失去焦点时，自动将内容按当前模式转换",
    tip_auto_process: "文件拖入程序后立即开始批量转换任务",
    tip_convert_text: "立即对上方输入框内的文字执行简繁转换",
    tip_start_batch: "依次处理下方队列中的所有待处理文件",
    tip_clear_queue: "清空当前所有的任务列表",
    btn_copy: "复制全部",
    btn_save_as: "保存为文件",
    tip_copy: "将上方文本框内的所有内容复制到系统剪贴板",
    tip_save_as: "将上方文本框内的内容导出为本地文件",
    toast_copy_success: "已复制到剪贴板",
    toast_save_success: "文件保存成功",
};

const ZH_HANT: UIStrings = UIStrings {
    file_menu: "文件",
    add_files: "添加文件",
    clear_queue: "清空隊列",
    about_menu: "關於",
    app_version: "簡繁轉換工具 v1.0.0",
    app_feature: "功能：支持批量簡繁中文轉換",
    app_developer: "開發者：Yssssssss",
    input_label: "文本輸入與實時預覽：",
    input_hint: "在此輸入文本或拖入文件...",
    mode_t2s: "繁體 -> 簡體",
    mode_s2t: "簡體 -> 繁體",
    replace_mode: "替換模式",
    new_mode: "新建模式",
    auto_convert: "自動轉換預覽",
    auto_process: "拖入即轉換",
    btn_convert_text: "立即轉換文本",
    btn_start_batch: "開始批量轉換",
    queue_label: "處理隊列：",
    col_name: "文件名",
    col_size: "大小",
    col_path: "路徑",
    col_status: "狀態",
    col_time: "耗時",
    status_pending: "等待中",
    status_processing: "轉換中...",
    status_completed: "完成",
    drag_feedback: "釋放文件以導入",
    tip_t2s: "將輸入的文字或隊列中的文件從繁體中文轉換为簡體中文",
    tip_s2t: "將輸入的文字或隊列中的文件從簡體中文轉換为繁體中文",
    tip_replace: "直接修改原始文件，操作不可逆，請謹慎使用",
    tip_new: "保留原始文件，在同目錄下創建帶時間戳的新文件",
    tip_auto_convert: "在文本框失去焦點時，自動將內容按當前模式轉換",
    tip_auto_process: "文件拖入程序后立即開始批量轉換任務",
    tip_convert_text: "立即對上方輸入框内的文字執行簡繁轉換",
    tip_start_batch: "依次處理下方隊列中的所有待處理文件",
    tip_clear_queue: "清空當前所有的任務列表",
    btn_copy: "複製全部",
    btn_save_as: "保存為文件",
    tip_copy: "將上方文本框内的所有内容複製到系統剪貼板",
    tip_save_as: "將上方文本框内的内容導出為本地文件",
    toast_copy_success: "已複製到剪貼板",
    toast_save_success: "文件保存成功",
};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum FileMode {
    Replace,
    New,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub auto_convert: bool,
    pub auto_process_on_drop: bool,
    pub file_mode: FileMode,
    pub conversion_mode: ConversionMode,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auto_convert: true,
            auto_process_on_drop: true,
            file_mode: FileMode::New,
            conversion_mode: ConversionMode::T2S,
        }
    }
}

use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize)]
struct Toast {
    message: String,
    #[serde(skip)]
    expiry: Option<Instant>,
}

pub struct ConverterApp {
    input_text: String,
    last_input_text: String,
    config: AppConfig,
    queue: TaskQueue,
    is_processing: bool,
    toasts: Vec<Toast>,
}

impl ConverterApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load config from persistent storage
        let config: AppConfig = cc.storage
            .and_then(|s| eframe::get_value(s, APP_KEY))
            .unwrap_or_default();

        // Setup custom font
        let mut fonts = egui::FontDefinitions::default();
        let font_data = include_bytes!("../fonts/zh.ttf");
        fonts.font_data.insert(
            "zh_font".to_owned(),
            egui::FontData::from_static(font_data),
        );
        
        // Use 14pt as default size
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "zh_font".to_owned());
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
            .push("zh_font".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        // Visual style optimization
        let mut style = (*cc.egui_ctx.style()).clone();
        
        // Material Design-ish Colors & Theme
        style.visuals.window_rounding = 12.0.into();
        style.visuals.widgets.noninteractive.rounding = 8.0.into();
        style.visuals.widgets.inactive.rounding = 8.0.into();
        style.visuals.widgets.hovered.rounding = 8.0.into();
        style.visuals.widgets.active.rounding = 8.0.into();
        style.visuals.widgets.open.rounding = 8.0.into();
        
        // Soft shadows and better contrast
        style.visuals.window_shadow = egui::Shadow {
            offset: egui::vec2(0.0, 10.0),
            blur: 20.0,
            spread: 0.0,
            color: egui::Color32::from_black_alpha(40),
        };
        
        style.spacing.item_spacing = egui::vec2(12.0, 12.0);
        style.spacing.window_margin = egui::Margin::same(20.0);
        style.spacing.button_padding = egui::vec2(16.0, 8.0);
        
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(18.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(18.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::new(16.0, egui::FontFamily::Monospace),
        );
        style.visuals.override_text_color = Some(egui::Color32::from_gray(240));
        
        // Use a more vibrant accent color
        style.visuals.selection.bg_fill = egui::Color32::from_rgb(0, 120, 215);
        style.visuals.hyperlink_color = egui::Color32::from_rgb(100, 180, 255);
        
        cc.egui_ctx.set_style(style);

        Self {
            input_text: String::new(),
            last_input_text: String::new(),
            config,
            queue: TaskQueue::new(),
            is_processing: false,
            toasts: Vec::new(),
        }
    }

    fn show_toast(&mut self, message: impl Into<String>) {
        self.toasts.push(Toast {
            message: message.into(),
            expiry: Some(Instant::now() + Duration::from_secs(2)),
        });
    }

    fn render_toasts(&mut self, ctx: &egui::Context) {
        self.toasts.retain(|t| t.expiry.map_or(false, |e| Instant::now() < e));
        
        if !self.toasts.is_empty() {
            let message = self.toasts.last().unwrap().message.clone();
            egui::Window::new("")
                .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -60.0))
                .frame(egui::Frame::window(&ctx.style())
                    .fill(egui::Color32::from_rgba_premultiplied(40, 40, 40, 230))
                    .rounding(8.0)
                    .inner_margin(egui::Margin::symmetric(20.0, 10.0))
                    .stroke(egui::Stroke::NONE))
                .title_bar(false)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new(message).color(egui::Color32::WHITE).size(18.0));
                });
            ctx.request_repaint_after(Duration::from_millis(100));
        }
    }

    fn process_files(&mut self) {
        let tasks_arc = Arc::clone(&self.queue.tasks);
        let conversion_mode = self.config.conversion_mode;
        let file_mode = self.config.file_mode;

        self.is_processing = true;

        std::thread::spawn(move || {
            let pending_tasks: Vec<(usize, PathBuf)> = {
                let tasks = tasks_arc.lock().unwrap();
                tasks.iter().enumerate()
                    .filter(|(_, t)| t.status == TaskStatus::Pending)
                    .map(|(i, t)| (i, t.path.clone()))
                    .collect()
            };

            pending_tasks.par_iter().for_each(|(idx, path)| {
                {
                    let mut tasks = tasks_arc.lock().unwrap();
                    tasks[*idx].status = TaskStatus::Processing;
                    tasks[*idx].start_time = Some(std::time::Instant::now());
                }

                let result = (|| -> anyhow::Result<String> {
                    let (content, encoding) = file_ops::read_file_with_encoding(path)?;
                    let converted = converter::convert(&content, conversion_mode);
                    
                    // Convert filename as well
                    let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
                    let converted_stem = converter::convert(&file_stem, conversion_mode);
                    
                    let target_path = file_ops::get_converted_file_path(
                        path, 
                        &converted_stem, 
                        file_mode == FileMode::New
                    );
                    
                    file_ops::write_file_with_encoding(&target_path, &converted, encoding)?;
                    
                    // If in replace mode and the filename actually changed, remove the old file
                    if file_mode == FileMode::Replace && target_path != *path {
                        let _ = std::fs::remove_file(path);
                    }
                    
                    Ok(format!("{:?}", std::time::Instant::now()))
                })();

                let mut tasks = tasks_arc.lock().unwrap();
                let duration = tasks[*idx].start_time.map(|s| s.elapsed()).unwrap_or_default();
                let duration_str = format!("{:.2?}", duration);
                
                match result {
                    Ok(_) => tasks[*idx].status = TaskStatus::Completed(duration_str),
                    Err(e) => tasks[*idx].status = TaskStatus::Error(e.to_string()),
                }
            });
        });
    }

    fn render_main_content(&mut self, ui: &mut egui::Ui, ui_strings: &UIStrings) {
        ui.vertical(|ui| {
            // Text Area Card
            egui::Frame::group(ui.style())
                .fill(ui.visuals().widgets.noninteractive.bg_fill)
                .rounding(12.0)
                .inner_margin(15.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(egui::RichText::new(ui_strings.input_label).strong().size(20.0));
                    ui.add_space(8.0);
                    
                    let edit = egui::TextEdit::multiline(&mut self.input_text)
                        .hint_text(ui_strings.input_hint)
                        .desired_width(f32::INFINITY)
                        .desired_rows(10)
                        .font(egui::TextStyle::Monospace)
                        .lock_focus(true);
                    
                    let response = ui.add(edit);
                    
                    if response.changed() && self.config.auto_convert {
                        // Debounce or just convert on lost focus/enter? 
                        // For now, let's stick to the button or lost focus logic
                    }

                    if response.lost_focus() && self.config.auto_convert && self.input_text != self.last_input_text {
                        self.input_text = converter::convert(&self.input_text, self.config.conversion_mode);
                        self.last_input_text = self.input_text.clone();
                    }

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button(ui_strings.btn_copy).on_hover_text(ui_strings.tip_copy).clicked() {
                            ui.output_mut(|o| o.copied_text = self.input_text.clone());
                            self.show_toast(ui_strings.toast_copy_success);
                        }
                        if ui.button(ui_strings.btn_save_as).on_hover_text(ui_strings.tip_save_as).clicked() {
                            if let Some(path) = FileDialog::new()
                                .add_filter("Text", &["txt", "md"])
                                .save_file() {
                                if file_ops::write_file_with_encoding(&path, &self.input_text, encoding_rs::UTF_8).is_ok() {
                                    self.show_toast(ui_strings.toast_save_success);
                                }
                            }
                        }
                    });
                });

            ui.add_space(16.0);
            self.render_action_area(ui, ui_strings);
            ui.add_space(16.0);
            self.render_queue_area(ui, ui_strings);
        });
    }

    fn render_action_area(&mut self, ui: &mut egui::Ui, ui_strings: &UIStrings) {
        egui::Frame::group(ui.style())
            .fill(ui.visuals().widgets.noninteractive.bg_fill)
            .rounding(12.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("转换模式").strong());
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.config.conversion_mode, ConversionMode::T2S, ui_strings.mode_t2s)
                                .on_hover_text(ui_strings.tip_t2s);
                            ui.selectable_value(&mut self.config.conversion_mode, ConversionMode::S2T, ui_strings.mode_s2t)
                                .on_hover_text(ui_strings.tip_s2t);
                        });
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("输出选项").strong());
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.config.file_mode, FileMode::Replace, ui_strings.replace_mode)
                                .on_hover_text(ui_strings.tip_replace);
                            ui.selectable_value(&mut self.config.file_mode, FileMode::New, ui_strings.new_mode)
                                .on_hover_text(ui_strings.tip_new);
                        });
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("便捷设置").strong());
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.config.auto_convert, ui_strings.auto_convert)
                                .on_hover_text(ui_strings.tip_auto_convert);
                            ui.checkbox(&mut self.config.auto_process_on_drop, ui_strings.auto_process)
                                .on_hover_text(ui_strings.tip_auto_process);
                        });
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(egui::Button::new(egui::RichText::new(ui_strings.btn_start_batch).strong()))
                            .on_hover_text(ui_strings.tip_start_batch)
                            .clicked() {
                            self.process_files();
                        }
                        
                        if ui.button(ui_strings.btn_convert_text)
                            .on_hover_text(ui_strings.tip_convert_text)
                            .clicked() {
                            self.input_text = converter::convert(&self.input_text, self.config.conversion_mode);
                            self.last_input_text = self.input_text.clone();
                        }
                    });
                });
            });
    }

    fn render_queue_area(&mut self, ui: &mut egui::Ui, ui_strings: &UIStrings) {
        egui::Frame::group(ui.style())
            .fill(ui.visuals().widgets.noninteractive.bg_fill)
            .rounding(12.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(ui_strings.queue_label).strong().size(20.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(ui_strings.clear_queue).on_hover_text(ui_strings.tip_clear_queue).clicked() {
                            self.queue.tasks.lock().unwrap().clear();
                        }
                    });
                });
                ui.add_space(8.0);

                let tasks = self.queue.tasks.lock().unwrap();
                if tasks.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(egui::RichText::new("队列为空，请拖入文件或从菜单添加").weak());
                        ui.add_space(20.0);
                    });
                } else {
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        egui::Grid::new("task_grid")
                            .num_columns(5)
                            .spacing([20.0, 12.0])
                            .striped(true)
                            .show(ui, |ui| {
                                // Header
                                ui.label(egui::RichText::new(ui_strings.col_name).strong());
                                ui.label(egui::RichText::new(ui_strings.col_size).strong());
                                ui.label(egui::RichText::new(ui_strings.col_path).strong());
                                ui.label(egui::RichText::new(ui_strings.col_status).strong());
                                ui.label(egui::RichText::new(ui_strings.col_time).strong());
                                ui.end_row();

                                for task in tasks.iter() {
                                    ui.label(&task.name);
                                    ui.label(format_size(task.size));
                                    
                                    let full_path = task.path.to_string_lossy();
                                    let path_label = shorten_path(&full_path);
                                    
                                    let response = ui.add(egui::Label::new(
                                        egui::RichText::new(path_label)
                                            .color(ui.visuals().hyperlink_color)
                                    ).sense(egui::Sense::click()));
                                    
                                    if response.on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text(full_path.as_ref())
                                        .clicked() {
                                        let _ = std::process::Command::new("explorer")
                                            .arg("/select,")
                                            .arg(task.path.as_os_str())
                                            .spawn();
                                    }
                                    
                                    match &task.status {
                                        TaskStatus::Pending => { ui.label(ui_strings.status_pending); }
                                        TaskStatus::Processing => { 
                                            ui.horizontal(|ui| {
                                                ui.spinner();
                                                ui.label(ui_strings.status_processing);
                                            });
                                        }
                                        TaskStatus::Completed(_) => { 
                                            ui.label(egui::RichText::new(ui_strings.status_completed).color(egui::Color32::GREEN));
                                        }
                                        TaskStatus::Error(e) => {
                                            ui.label(egui::RichText::new("错误").color(egui::Color32::RED))
                                                .on_hover_text(e);
                                        }
                                    }

                                    if let TaskStatus::Completed(time) = &task.status {
                                        ui.label(time);
                                    } else {
                                        ui.label("-");
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                }
            });
    }
}

impl eframe::App for ConverterApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_KEY, &self.config);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let ui_strings = match self.config.conversion_mode {
            ConversionMode::T2S => &ZH_HANS,
            ConversionMode::S2T => &ZH_HANT,
        };

        // Handle drag and drop
        let mut dropped_any = false;
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    if let Some(path) = &file.path {
                        self.queue.add_task(path.clone());
                        dropped_any = true;
                    }
                }
            }
        });

        if dropped_any && self.config.auto_process_on_drop {
            self.process_files();
        }

        // Drag feedback overlay
        if ctx.input(|i| !i.raw.hovered_files.is_empty()) {
            egui::Area::new(egui::Id::new("drag_feedback"))
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .order(egui::Order::Foreground)
                .interactable(false)
                .show(ctx, |ui| {
                    let screen_rect = ui.ctx().screen_rect();
                    
                    // Dark semi-transparent overlay
                    ui.painter().rect_filled(
                        screen_rect,
                        0.0,
                        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 160),
                    );

                    // Blue border
                    ui.painter().rect_stroke(
                        screen_rect.shrink(10.0),
                        8.0,
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 120, 215)),
                    );

                    ui.vertical_centered(|ui| {
                        ui.add_space(screen_rect.height() / 2.0 - 40.0);
                        ui.label(
                            egui::RichText::new(ui_strings.drag_feedback)
                                .font(egui::FontId::proportional(40.0))
                                .strong()
                                .color(egui::Color32::WHITE)
                        );
                    });
                });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(ui_strings.file_menu, |ui| {
                    if ui.button(ui_strings.add_files).clicked() {
                        if let Some(files) = FileDialog::new()
                            .add_filter(ui_strings.add_files, &[
                                "txt", "md", "html", "htm", 
                                "js", "ts", "py", "sh", "bat", "ps1",
                                "json", "xml", "yaml", "yml", "ini", "toml",
                                "c", "cpp", "h", "hpp", "java", "cs", "rs", "go"
                            ])
                            .pick_files() {
                            for path in files {
                                self.queue.add_task(path);
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button(ui_strings.clear_queue).clicked() {
                        self.queue.tasks.lock().unwrap().clear();
                        ui.close_menu();
                    }
                });
                ui.menu_button(ui_strings.about_menu, |ui| {
                    ui.label(ui_strings.app_version);
                    ui.label(ui_strings.app_feature);
                    ui.label(ui_strings.app_developer);
                    ui.separator();
                    ui.label("基于 Rust 编写");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_content(ui, ui_strings);
        });

        // Repaint periodically if processing
        if self.is_processing {
            ctx.request_repaint();
            let tasks = self.queue.tasks.lock().unwrap();
            if tasks.iter().all(|t| !matches!(t.status, TaskStatus::Pending | TaskStatus::Processing)) {
                self.is_processing = false;
            }
        }

        self.render_toasts(ctx);
    }
}

fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else {
        format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
    }
}

fn shorten_path(path: &str) -> String {
    if path.len() <= 50 {
        return path.to_string();
    }
    let parts: Vec<&str> = path.split('\\').collect();
    if parts.len() <= 3 {
        return path.to_string();
    }
    format!("{}\\{}\\....\\{}", parts[0], parts[1], parts.last().unwrap())
}
