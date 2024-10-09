use crate::kata::{
    Hyou,
    Waku::*,
};

pub fn show(h: &Hyou) {
    for row in h { 
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
        }
        print!("\n");
    }
}