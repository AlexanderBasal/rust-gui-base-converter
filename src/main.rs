use eframe::egui::{CentralPanel, ComboBox, Context, Align2, pos2};

#[derive(Default)]
struct ConverterApp {
    input_type: String,
    input_value: String,
    output_binary: String,
    output_decimal: String,
    output_hexadecimal: String,
    error_message: String,
}

impl eframe::App for ConverterApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Binary, Decimal, Hexadecimal Converter");

            ui.horizontal(|ui| {
                ui.label("Input Type:");
                ComboBox::from_id_source("input_type")
                    .selected_text(&self.input_type)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.input_type, "Binary".to_string(), "Binary");
                        ui.selectable_value(&mut self.input_type, "Decimal".to_string(), "Decimal");
                        ui.selectable_value(&mut self.input_type, "Hexadecimal".to_string(), "Hexadecimal");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Input Value:");
                ui.text_edit_singleline(&mut self.input_value);
            });

            if ui.button("Convert").clicked() {
                self.convert();
            }

            ui.separator();

            if !self.error_message.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.error_message);
            }

            ui.label(format!("Binary: {}", self.output_binary));
            ui.label(format!("Decimal: {}", self.output_decimal));
            ui.label(format!("Hexadecimal: {}", self.output_hexadecimal));
        });

        
        let rect = ctx.available_rect();
        let painter = ctx.layer_painter(egui::LayerId::background());
        let sig = "Made in Rust with egui By Alexander Basal";

        painter.text(
            pos2(rect.max.x - 10.0, rect.max.y - 20.0),
            Align2::RIGHT_BOTTOM,
            sig,
            egui::TextStyle::Body.resolve(&ctx.style()),
            egui::Color32::WHITE,
        );
    }
}

impl ConverterApp {
    fn convert(&mut self) {
        self.error_message.clear();
        match self.input_type.as_str() {
            "Binary" => {
                match u32::from_str_radix(self.input_value.trim(), 2) {
                    Ok(num) => {
                        self.output_binary = format!("{:b}", num);
                        self.output_decimal = format!("{}", num);
                        self.output_hexadecimal = format!("{:X}", num);
                    }
                    Err(_) => {
                        self.error_message = "Invalid binary number.".to_string();
                        self.output_binary.clear();
                        self.output_decimal.clear();
                        self.output_hexadecimal.clear();
                    }
                }
            }
            "Decimal" => {
                match self.input_value.trim().parse::<u32>() {
                    Ok(num) => {
                        self.output_binary = format!("{:b}", num);
                        self.output_decimal = format!("{}", num);
                        self.output_hexadecimal = format!("{:X}", num);
                    }
                    Err(_) => {
                        self.error_message = "Invalid decimal number.".to_string();
                        self.output_binary.clear();
                        self.output_decimal.clear();
                        self.output_hexadecimal.clear();
                    }
                }
            }
            "Hexadecimal" => {
                match u32::from_str_radix(self.input_value.trim(), 16) {
                    Ok(num) => {
                        self.output_binary = format!("{:b}", num);
                        self.output_decimal = format!("{}", num);
                        self.output_hexadecimal = format!("{:X}", num);
                    }
                    Err(_) => {
                        self.error_message = "Invalid hexadecimal number.".to_string();
                        self.output_binary.clear();
                        self.output_decimal.clear();
                        self.output_hexadecimal.clear();
                    }
                }
            }
            _ => {
                self.error_message = "Please select a valid input type.".to_string();
            }
        }
    }
}

fn main() {
    let app = ConverterApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Base Converter",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}