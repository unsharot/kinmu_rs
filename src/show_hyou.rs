use crate::kata::{
    Hyou,
    Waku::*,
};

pub fn show(h: &Hyou, buffer: isize) {
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
            if i == buffer {
                print!("|");
            }
        }
        print!("\n");
    }
}