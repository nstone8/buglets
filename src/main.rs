extern crate gtk;
extern crate cairo;

mod hex_objects;
mod stacks;

use std::boxed::Box;

use gtk::prelude::*;
use gtk::DrawingArea;
use gtk::GestureDrag;

use cairo::Context;

use hex_objects::*;
use stacks::*;

fn main() {
    
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    
    let mut stack=Stack::new();
    stack.add_piece(Box::new(EmptySpace::new(Position::new(2,0,-2,0))));
    stack.add_piece(Box::new(EmptySpace::new(Position::new(2,1,-3,0))));
    stack.add_piece(Box::new(EmptySpace::new(Position::new(3,0,-3,0))));


    
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("buglets");
    let drawing_area = Box::new(DrawingArea::new)();
    drawing_area.connect_draw(move |_:&DrawingArea,cr:&Context|{
        let scale = stack.get_draw_scale();
        cr.scale(scale,scale);
        stack.draw_all_function(cr);
        Inhibit(false)
    });
    let drag=GestureDrag::new(&drawing_area);
    drag.connect_drag_update(|_:&GestureDrag,x:f64,y:f64|{
        println!("x={}, y={}",x,y);
        stack.set_temp_offset(x,y);
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
