use std::error::Error;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // Home button callback
    ui.on_home_clicked({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_is_home_page(true);
                println!("Home page activated");
            }
        }
    });

    // Library button callback
    ui.on_library_clicked({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_is_home_page(false);
                println!("Library page activated");
            }
        }
    });

    ui.run()?;
    Ok(())
}
