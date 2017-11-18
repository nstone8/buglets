use std::vec::Vec;
use cairo::Context;
use hex_objects::*;

pub struct Stack{
    all_pieces:Vec<Box<Hex>>,
    global_offset:Offset
}impl Stack{
    pub fn new() -> Stack{
        Stack{all_pieces:Vec::new(),
              global_offset:Offset{x:0,y:0}}
    }
    pub fn get_draw_scale(&self) -> f64{
        return 30.0;
    }
    pub fn add_piece(&mut self,h: Box<Hex>){
        self.all_pieces.push(h);
    }
    pub fn get_global_offset(&self) -> f64{
        return self.global_offset;
    }
    pub fn draw_all_function(&self,cr:&Context){
        for piece in &self.all_pieces{
            piece.draw_fn(cr);
        }
    }
}
