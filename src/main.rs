extern crate gtk;
extern crate cairo;

mod hex_objects;
mod stacks;

use std::vec::Vec;
use std::boxed::Box;
use gtk::prelude::*;
use gtk::DrawingArea;
use cairo::Context;

use hex_objects::*;
use stacks::*;

fn main() {
    
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    
    let spaceStack=Stack::new::<EmptySpace>();
    let e = EmptySpace::new(Position::new(2,0,-2,0));
    let et= EmptySpace::new(Position::new(2,1,-3,0));
    let etc= EmptySpace::new(Position::new(3,0,-3,0));
    spaceStack.add_piece(e);
    spaceStack.add_piece(et);
    spaceStack.add_piece(etc);

    
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("buglets");
    let drawing_area = Box::new(DrawingArea::new)();
    drawing_area.connect_draw(move |_:&DrawingArea,cr:&Context|{
        let scale = spaceStack.get_draw_scale();
        cr.scale(scale,scale);
        spaceStack.draw_draw_all_function(cr);
        Inhibit(false)
    });
    window.set_default_size(300, 300);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.add(&drawing_area);
    window.show_all();

    gtk::main();
}
