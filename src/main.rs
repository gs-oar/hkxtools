use eframe::{egui, Frame};
use egui::{Context as EguiContext, Color32, RichText, Ui};
use rfd::FileDialog;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Result, Context as AnyhowContext};
use std::fs;

const HKXC_EXE: &[u8] = include_bytes!("hkxc.exe");

struct HkxToolsApp {
    input_paths: Vec<PathBuf>,
    output_folder: Option<PathBuf>,
    output_suffix: String,
    output_format: OutputFormat,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum OutputFormat {
    Xml,
    SkyrimLE,
    SkyrimSE,
}

impl OutputFormat {
    fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Xml => "xml",
            OutputFormat::SkyrimLE | OutputFormat::SkyrimSE => "hkx",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            OutputFormat::Xml => "XML",
            OutputFormat::SkyrimLE => "Skyrim LE",
            OutputFormat::SkyrimSE => "Skyrim SE",
        }
    }
}

impl Default for HkxToolsApp {
    fn default() -> Self {
        Self {
            input_paths: Vec::new(),
            output_folder: None,
            output_suffix: String::new(),
            output_format: OutputFormat::Xml,
        }
    }
}

impl HkxToolsApp {
    fn update_output_folder(&mut self) {
        if let Some(input_path) = self.input_paths.first() {
            self.output_folder = Some(input_path.parent().unwrap_or(Path::new("")).to_path_buf());
        }
    }

    fn get_output_path(&self, input_path: &Path) -> Option<PathBuf> {
        let output_folder = self.output_folder.as_ref()?;
        let file_name = input_path.file_stem()?.to_str()?;
        let extension = self.output_format.extension();
        let output_name = if self.output_suffix.is_empty() {
            format!("{}.{}", file_name, extension)
        } else {
            format!("{}_{}.{}", file_name, self.output_suffix, extension)
        };
        Some(output_folder.join(output_name))
    }

    fn run_conversion(&mut self) -> Result<()> {
        if self.input_paths.is_empty() {
            return Err(anyhow::anyhow!("No input files selected"));
        }
        if self.output_folder.is_none() {
            return Err(anyhow::anyhow!("No output folder selected"));
        }

        for input_path in &self.input_paths {
            let output_path = self.get_output_path(input_path)
                .context("Failed to determine output path")?;
            
            println!("Converting {:?} to {:?}", input_path, output_path);

            self.run_hkxc(input_path, &output_path)?;

            // Verify output file
            if !output_path.exists() {
                return Err(anyhow::anyhow!("Output file was not created: {:?}", output_path));
            }
            println!("Output file created successfully: {:?}", output_path);
            let metadata = fs::metadata(&output_path)?;
            println!("Output file size: {} bytes", metadata.len());
        }

        Ok(())
    }

    fn run_hkxc(&self, input: &Path, output: &Path) -> Result<()> {
        let mut command = Command::new("hkxc");
        command.arg("convert");
        command.arg("--input").arg(input);
        command.arg("--output").arg(output);
        
        command.arg("--format").arg(match self.output_format {
            OutputFormat::Xml => "xml",
            OutputFormat::SkyrimLE => "win32",
            OutputFormat::SkyrimSE => "amd64",
        });

        let output = command
            .output()
            .context("Failed to execute hkxc")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Print to console
        eprintln!("hkxc stdout:\n{}", stdout);
        eprintln!("hkxc stderr:\n{}", stderr);

        // Also print to application log
        println!("hkxc stdout:\n{}", stdout);
        println!("hkxc stderr:\n{}", stderr);

        if !output.status.success() {
            return Err(anyhow::anyhow!("hkxc failed: {}", stderr));
        }

        Ok(())
    }

    fn render_main_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading(RichText::new("HKX Conversion Tool").size(24.0).color(Color32::LIGHT_BLUE));
            ui.add_space(10.0);
        });

        ui.separator();

        egui::Grid::new("main_grid").num_columns(2).spacing([10.0, 10.0]).show(ui, |ui| {
            ui.label("Input Files:");
            if ui.button("Browse Input Files").clicked() {
                if let Some(paths) = FileDialog::new().pick_files() {
                    self.input_paths = paths;
                    self.update_output_folder();
                }
            }
            ui.end_row();

            ui.label("Selected Files:");
            ui.vertical(|ui| {
                for path in &self.input_paths {
                    ui.label(path.file_name().unwrap_or_default().to_string_lossy());
                }
            });
            ui.end_row();

            ui.label("Output Folder:");
            self.render_output_folder(ui);
            ui.end_row();

            ui.label("Output Suffix:");
            ui.text_edit_singleline(&mut self.output_suffix);
            ui.end_row();

            ui.label("Output Format:");
            self.render_output_format(ui);
            ui.end_row();
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Run Conversion").clicked() {
                self.handle_conversion(ui);
            }
        });
    }

    fn render_output_folder(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if let Some(ref output_folder) = self.output_folder {
                ui.label(output_folder.to_string_lossy());
            }
            if ui.button("Browse").clicked() {
                if let Some(folder) = FileDialog::new().pick_folder() {
                    self.output_folder = Some(folder);
                }
            }
        });
    }

    fn render_output_format(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for format in [OutputFormat::Xml, OutputFormat::SkyrimLE, OutputFormat::SkyrimSE] {
                if ui.selectable_label(self.output_format == format, format.label()).clicked() {
                    self.output_format = format;
                }
            }
        });
    }

    fn handle_conversion(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        match self.run_conversion() {
            Ok(_) => {
                ui.colored_label(Color32::GREEN, "✓ Conversion completed successfully");
            }
            Err(e) => {
                ui.colored_label(Color32::RED, format!("❌ Error during conversion: {}", e));
            }
        }
    }
}

impl eframe::App for HkxToolsApp {
    fn update(&mut self, ctx: &EguiContext, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_ui(ui);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // Write hkxc.exe to a temporary location
    let temp_dir = tempfile::Builder::new().prefix("hkxtools_").tempdir().unwrap();
    let hkxc_path = temp_dir.path().join("hkxc.exe");
    fs::write(&hkxc_path, HKXC_EXE).unwrap();

    // Add hkxc.exe to the PATH
    let mut path = std::env::var("PATH").unwrap_or_default();
    path.push_str(&format!(";{}", temp_dir.path().to_str().unwrap()));
    std::env::set_var("PATH", path);

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "HKX Tools GUI",
        options,
        Box::new(|_cc| Ok(Box::new(HkxToolsApp::default()))),
    )
}