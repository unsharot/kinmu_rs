use crate::kata::{
    Hyou,
    Waku,
    Waku::*,
    HyouProp,
    DayST,
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

        println!();
    }

    println!();

    //曜日を表示
    print_days(&hp.days, hp.buffer);

    //日ごとの統計を表示
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


///曜日を表示
fn print_days(days: &Vec<DayST>, buffer: isize) {
    let mut i = 0;
    for d in days {
        print!("{}", match d {
            DayST::Weekday => "W",
            DayST::Holiday => "H",
            DayST::Furo => "F",
            DayST::Furo2 => "2",
            DayST::Weight => "G",
            // _ => "UNDEFINED!!!!",
        });
        i += 1;
        if i == buffer {
            print!("|");
        }
    }
    println!();
}