extern crate gtk;
extern crate cairo;

mod hex_objects;

use gtk::prelude::*;
use gtk::DrawingArea;
use hex_objects::*;
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let e = EmptySpace{};
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("buglets");
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(e.get_draw_fn());
    window.set_default_size(300, 300);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.add(&drawing_area);
    window.show_all();

    gtk::main();
}
