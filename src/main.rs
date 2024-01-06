slint::include_modules!();
use std::fs;
use rfd::FileDialog;
use pdf_extract::extract_text;
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });
    // let ui_handle = ui.as_weak();
    // ui.on_submit_url(move |url| {
    //    let ui: AppWindow = ui_handle.unwrap();
    //     ui.set_url(url);
    // });

    let ui_handle = ui.as_weak();
    ui.on_open_file(move || {
    let ui: AppWindow = ui_handle.unwrap();
    
        let files = FileDialog::new()
        .add_filter("pdf", &["pdf"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file();
    //    println!("{:?}",files);
        ui.set_status("Loading...".into());

        ui.set_url(files.clone().unwrap().display().to_string().into());
        let mut extracted_txt_vec = Vec::new();
        if let Some(ref file)= files{
        let extracted_txt = extract_text(file.clone()).expect("Error");
         extracted_txt_vec.push(extracted_txt.clone());
        //  println!("{:?}",extracted_txt_vec);
        // println!("************** {} ***********",&file.display());
       }
    //    println!("{:?}",extracted_txt_vec);
    let mut out_file_path = files.clone().unwrap().display().to_string();
    // add .txt extension to file path
    out_file_path.push_str(".txt");
    
    let out_file = FileDialog::new()
    .set_directory(out_file_path.clone())
    .set_file_name("out.txt")
    .save_file();
    // println!("{:?}",out_file);

for line in extracted_txt_vec{
    fs::write(out_file.clone().unwrap(), line).expect("Unable to write file");
    
};

ui.set_status("Done".into());
    });
    ui.run()
}
