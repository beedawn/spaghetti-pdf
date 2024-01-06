slint::include_modules!();
use rfd::FileDialog;

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
        // ui.set_url(url);
        let files = FileDialog::new()
    .add_filter("pdf", &["pdf"])
    .add_filter("rust", &["rs", "toml"])
    .set_directory("/")
    .pick_file();
       println!("{:?}",files);
    //    ui.set_url(files.unwrap());
    });


    ui.run()
}
