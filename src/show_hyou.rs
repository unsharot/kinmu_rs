use crate::kata::{
    Hyou,
    Waku::*,
    HyouProp
};

pub fn show(h: &Hyou, hp: &HyouProp) {
    for row in h { 
        let mut i = 0;
        for w in row {
            print!("{}",match w {
                N => "N",
                K => "K",
                I => "I",
                A => "A",
                O => "O",
                H => "H",
                Y => "Y",
                D => "D",
                U => "U",
            });
            i += 1;
            if i == hp.buffer {
                print!("|");
            }
        }
        print!("\n");
    }
}