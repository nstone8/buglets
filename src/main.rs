extern crate gtk;
extern crate cairo;

mod hex_objects;

use std::vec::Vec;
use std::boxed::Box;
use gtk::prelude::*;
use gtk::DrawingArea;
use cairo::Context;
use hex_objects::*;
fn main() {
    
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let mut painter=Monet::new();
    let e = EmptySpace::new(Position::new(-1,4,-3,0));
    let et= EmptySpace::new(Position::new(-2,4,-2,0));
    painter.add_piece(e);
    painter.add_piece(et);
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("buglets");
    let drawing_area = Box::new(DrawingArea::new)();
    drawing_area.connect_draw(move |_:&DrawingArea,cr:&Context|{
        let scale = Monet::get_draw_scale();
        cr.scale(scale,scale);
        painter.draw_all_function(cr);
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
pub struct Monet{
    all_pieces:Vec<EmptySpace>
}impl Monet{
    pub fn new() -> Monet{
        Monet{all_pieces:Vec::new()}
    }
    pub fn get_draw_scale() -> f64{
        return 30.0;
    }
    fn add_piece(&mut self,h: EmptySpace){
        self.all_pieces.push(h);
    }
    fn draw_all_function(&self,cr:&Context){
        for piece in &self.all_pieces{
            piece.draw_fn(cr);
        }
    }
}
