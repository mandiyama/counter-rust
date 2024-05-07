use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = AppWindow::new()?;
    app.set_window_title(SharedString::from("Счетик"));
    app.run()?;

    Ok(())
}