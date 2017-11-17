use cairo::Context;

pub trait Hex {
    fn draw_hex(&self,p:&Position, cr:&Context) {
        //Draw a hexagon centered at p.i, p.j, p.k
        //let one unit be the distance from the center of the hexagon to an edge along a line which bisects that edge
        let x=p.get_cartesian_x();
        let y=p.get_cartesian_y();
        cr.set_source_rgb(0.0,0.0,0.0);
        cr.set_line_width(0.1);
        cr.new_path();
        cr.move_to(1.1547+x,0.0+y);
        cr.line_to(0.5774+x,1.0+y);
        cr.line_to(-0.5774+x,1.0+y);
        cr.line_to(-1.1547+x,0.0+y);
        cr.line_to(-0.5774+x,-1.0+y);
        cr.line_to(0.5774+x,-1.0+y);
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
    pub fn new(i:i32, j:i32, k:i32, h:i32) -> Position{
        Position{i:i,j:j,k:k,h:h}
    }
    //Calculate equivalent cartesian coordinates. One unit corresponds to 1/2 the distance between 2 parallel sides
    pub fn get_cartesian_x(&self) -> f64{
        let x=1.1547*(self.i as f64)-0.5774*(self.j as f64)-0.5774*(self.k as f64);
        println!("x={}",x);
        return x;
    }
    pub fn get_cartesian_y(&self) -> f64{
        let y=self.j-self.k;
        println!("y={}",y);
        return y as f64;
            
    }
}

pub struct EmptySpace{
pos:Position
} impl EmptySpace {
    pub fn new(pos:Position) -> EmptySpace{
        EmptySpace{pos:pos}
    }
} impl Hex for EmptySpace{
    fn draw_fn(&self,cr:&Context){
            self.draw_hex(&self.pos,cr);

    }
}
