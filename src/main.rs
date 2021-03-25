extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

fn build_app(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Funicular");
    window.set_default_size(350, 70);

    let button = Button::with_label("Click me!");
    button.connect_clicked(|_| {
        println!("Boo !!");
    });
    window.add(&button);

    window.show_all();
}

fn main() {
    let application = Application::new(Some("org.mountains.funicular"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| build_app(&app));

    application.run(&[]);
}
