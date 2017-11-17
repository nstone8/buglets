use std::vec::Vec;
use cairo::Context;
use hex_objects::*;

pub struct Stack{
    all_pieces:Vec<Box<Hex>>,
    global_offset:f64
}impl Stack{
    pub fn new() -> Stack{
        Stack{all_pieces:Vec::new(),global_offset:30.0}
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

// pub struct MetaStack<'a>{
//     //This could be done with pre-allocated arrays, but I'm going to be lazy for now
//     all_stacks: Vec<Box<Hex + 'a>>,
//     global_offset:f64
// }impl<'a> MetaStack<'a>{
//     pub fn new(s: Stack<'a Hex+'a>) -> MetaStack{
//         let me=MetaStack{all_stacks:Vec::new::<Box<Stack<>>>(),global_offset:0.0};
//         me.add_stack(s);
//         return me
//     }
//     pub fn get_draw_scale() -> f64{
//         return 30.0;
//     }
//     pub fn get_global_offset(&self) -> f64{
//         return self.global_offset;
//     }
//     pub fn add_stack<H:Hex>(&mut self,s:Stack<H>){
//         self.all_stacks.push(Box::new(s));
//     }
//     pub fn draw_universe(&self,cr:Context){
//         for s in self.all_stacks{
//             s.draw_all_function(cr);
//         }
//     }
// }
