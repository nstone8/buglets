use cairo::Context;
use hex_objects::*;

pub struct Stack{
    all_pieces:Vec<Box<Hex>>,
    global_offset:Offset,
    temp_offset:Offset        
}impl Stack{
    pub fn new() -> Stack{
        Stack{all_pieces:Vec::new(),
              global_offset:Offset::new(0.0,0.0),
              temp_offset:Offset::new(0.0,0.0)}
    }
    pub fn set_temp_offset(&self,x:f64,y:f64){
        self.temp_offset.x=x;
        self.temp_offset.y=y;
    }
    pub fn finalize_drag(&self){
        self.global_offset=self.global_offset+self.temp_offset;
        self.temp_offset=Offset{x:0,y:0};
    }
    pub fn get_draw_scale(&self) -> f64{
        return 30.0;
    }
    pub fn add_piece(&mut self,h: Box<Hex>){
        self.all_pieces.push(h);
    }
    // pub fn get_global_offset(&self) -> &Offset{
    //     return &self.global_offset;
    // }
    pub fn draw_all_function(&self,cr:&Context){
        for piece in &self.all_pieces{
            piece.draw_fn(&(self.global_offset+self.global_offset),cr);
        }
    }
}
