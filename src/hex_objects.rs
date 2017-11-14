use gtk::prelude::*;
use cairo::Context;
use gtk::DrawingArea;

pub trait Hex {
    fn draw_hex(p:Position, cr:&Context) {
        //Draw a hexagon centered at p.i, p.j, p.k
        let x=p.get_cartesian_x();
        let y=p.get_cartesian_y();
        cr.set_source_rgb(0.0,0.0,0.0);
        cr.set_line_width(0.1);
        cr.new_path();
        cr.move_to(1.0+x,0.0+y);
        cr.line_to(0.5+x,0.866+y);
        cr.line_to(-0.5+x,0.866+y);
        cr.line_to(-1.0+x,0.0+y);
        cr.line_to(-0.5+x,-0.866+y);
        cr.line_to(0.5+x,-0.866+y);
        cr.close_path();
        cr.stroke();
        
    }

    fn draw_fn(&self,cr:&Context);
}
pub struct Position{
    i:i32, //Number of hexes perpendicular to 'top' edge
    j:i32, //Number of hexes perpendicular to the top right edge
    k:i32, //Number of hexes perpendicular to the bottom right edge
    h:i32, //Number of pieces stacked under this one
} impl Position{
    fn new(i:f64, j:f64, k:f64, h:f64) -> Position{
        Position{i:i,j:j,k:k,h:h}
    }
    //Calculate equivalent cartesian coordinates. One unit corresponds to the max radius of each piece
    pub fn get_cartesian_x(&self) -> f64{
        0.7071*self.j+0.7071*self.k
    }
    pub fn get_cartesian_y(&self) -> f64{
        self.i+0.7071*self.i+0.7071*self.k
    }
}

pub struct EmptySpace{
pos:Position
}

impl Hex for EmptySpace{
    fn draw_fn(&self,cr:&Context){
            Self::draw_hex(self.pos,cr);

    }
}
