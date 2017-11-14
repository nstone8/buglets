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
    let scale=30;
    
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let mut painter=Monet::new(scale);
    let e = EmptySpace{pos:Position::new(0,0,0,0)};
    painter.add_piece(e);
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("buglets");
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(|d:&DrawingArea,cr:&Context|{
        cr.scale(painter.scale,painter.scale);
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
struct Monet{
    scale: f64,
    allPieces:Vec<Box<EmptySpace>>
}impl Monet{
    fn new(scale: f64) -> Monet{
        Monet{scale:scale,allPieces:Vec::new()}
    }
    fn add_piece(&mut self,h: EmptySpace){
        self.allPieces.push(Box::new(h));
    }
    fn draw_all_function(&self,cr:&Context){
        for piece in &self.allPieces{
            let draw_fn=piece.get_draw_fn();
            draw_fn(cr);
        }
    }
}
