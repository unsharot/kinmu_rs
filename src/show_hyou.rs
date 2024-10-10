use crate::kata::{
    Hyou,
    Waku,
    Waku::*,
    HyouProp,
};

///表を出力
pub fn show(h: &Hyou, hp: &HyouProp) {
    for (row_index, row) in h.iter().enumerate() { 
        //Wakuの行を出力
        print_waku_row(&row, hp.buffer);

        //統計情報
        print_waku_count(&row, H);
        print_waku_count(&row, O);
        print_waku_count(&row, I);
        print_waku_count(&row, N);
        print_waku_count(&row, K);
        print_waku_count(&row, Y);

        //名前
        print!(" {}", hp.workers[row_index].name);

        print!("\n");
    }
}

///Wakuの行を出力
fn print_waku_row(row: &Vec<Waku>, buffer: isize) {
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
}

///指定した枠の数を出力
fn print_waku_count(row: &Vec<Waku>, target_w: Waku) {
    let mut sum = 0;
    for w in row {
        if *w == target_w {
            sum += 1;
        }
    }
    print!(" {:>2}", sum);
}