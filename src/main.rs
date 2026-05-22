use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let state = Rc::new(RefCell::new(AppState::new()));

    let ui_handle = ui.as_weak();
    let state_clone = state.clone();
    ui.on_increment(move || {
        let mut state = state_clone.borrow_mut();
        state.increment();
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_counter(state.counter);
        }
    });
//idk
    app_handler.initialize_ui();

    let res = app_handler.run();
    app_handler.save();
    res
}