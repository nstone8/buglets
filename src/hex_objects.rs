use gtk::prelude::*;
use cairo::Context;
use gtk::DrawingArea;

pub trait Hex {
    fn draw_hex(p:Position, cr:&Context) {
        //Draw a hexagon centered at p.x,p.y with max radius of 'scale'
        cr.set_source_rgb(0.0,0.0,0.0);
        cr.set_line_width(0.1);
        cr.new_path();
        cr.move_to(1.0+p.x,0.0+p.y);
        cr.line_to(0.5+p.x,0.866+p.y);
        cr.line_to(-0.5+p.x,0.866+p.y);
        cr.line_to(-1.0+p.x,0.0+p.y);
        cr.line_to(-0.5+p.x,-0.866+p.y);
        cr.line_to(0.5+p.x,-0.866+p.y);
        cr.close_path();
        cr.stroke();
        
    }

    fn get_draw_fn(&self) -> fn(&DrawingArea,&Context) -> Inhibit {
        let draw_fn=|_:&DrawingArea,cr:&Context| -> Inhibit{
            cr.scale(100.0,100.0);
            Self::draw_hex(Position{x:2.0,y:2.0},cr);
            Inhibit(false)
        };
        return draw_fn;
    }
}
pub struct Position{
    x:f64,
    y:f64
}

pub struct EmptySpace{}

impl Hex for EmptySpace{}
