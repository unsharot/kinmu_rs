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
        print_waku_count_row(&row, H);
        print_waku_count_row(&row, O);
        print_waku_count_row(&row, I);
        print_waku_count_row(&row, N);
        print_waku_count_row(&row, K);
        print_waku_count_row(&row, Y);

        //名前
        print!(" {}", hp.workers[row_index].name);

        println!();
    }

    println!();

    //曜日を表示
    print_days(&hp.days, hp.buffer);

    //日ごとの統計を表示
    print_waku_count_column(&h, hp, N);
    print_waku_count_column(&h, hp, I);
    print_waku_count_column(&h, hp, A);
    print_waku_count_column(&h, hp, K);
    print_waku_count_column(&h, hp, O);
    print_waku_count_column(&h, hp, H);

    //スコア表示
}

///Wakuの行を出力
fn print_waku_row(row: &Vec<Waku>, buffer: usize) {
    for (i, w) in row.iter().enumerate() {
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
        if i + 1 == buffer {
            print!("|");
        }
    }
}

///指定した枠の数を出力
fn print_waku_count_row(row: &Vec<Waku>, target_w: Waku) {
    let mut sum = 0;
    for w in row {
        if *w == target_w {
            sum += 1;
        }
    }
    print!(" {:>2}", sum);
}


///曜日を表示
fn print_days(days: &Vec<DayST>, buffer: usize) {
    for (i, d) in days.iter().enumerate() {
        print!("{}", match d {
            DayST::Weekday => "W",
            DayST::Holiday => "H",
            DayST::Furo => "F",
            DayST::Furo2 => "2",
            DayST::Weight => "G",
            // _ => "UNDEFINED!!!!",
        });
        if i + 1 == buffer {
            print!("|");
        }
    }
    println!();
}

/// 指定した枠の列の和を表示
fn print_waku_count_column(h: &Hyou, hp: &HyouProp, target_w: Waku) {
    let mut v: Vec<String> = Vec::new();
    let mut max_length = 0;
    for c in 0..hp.day_count {
        let mut sum = 0;
        for r in 0..hp.worker_count {
            if h[r][c] == target_w {
                sum += 1;
            }
        }
        let s = sum.to_string();
        v.push(s.clone());
        if max_length < s.len() {
            max_length = s.len();
        }
    }
    
    for l in 0..max_length {
        for c in 0..hp.day_count {
            if l < v[c].len() {
                print!("{}", &v[c][l..l+1]);
            } else {
                print!(" ");
            }
            if c + 1 == hp.buffer {
                print!("|");
            }
        }
        if l == 0 {
            print!(" {}", match target_w {
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
        println!();
    }
}