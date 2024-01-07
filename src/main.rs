slint::include_modules!();
use std::fs;
use rfd::FileDialog;
use pdf_extract::extract_text;
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_open_file(move || {
        let ui: AppWindow = ui_handle.unwrap();
        let files = FileDialog::new()
        .add_filter("pdf", &["pdf"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file();
        ui.set_status("Loading...".into());
        ui.set_url(files.as_ref().unwrap().display().to_string().into());
        let mut extracted_txt_string = String::new();
        if let Some(ref file)= files{
        let extracted_txt = extract_text(file).expect("Error");
        extracted_txt_string.push_str(&extracted_txt.as_str());
    }
    let mut out_file_path = files.as_ref().unwrap().display().to_string();
    // add .txt extension to file path
    out_file_path.push_str(".txt");
    
    let out_file = FileDialog::new()
    .set_directory(out_file_path)
    .set_file_name("out.txt")
    .save_file();

let _ = fs::write(out_file.as_ref().unwrap(), extracted_txt_string);
ui.set_status("Done".into());
    });
    ui.run()
}
