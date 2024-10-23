use crate::kinmu_lib::kata::{
    Hyou,
    Waku,
    Waku::*,
    HyouProp,
    DayST,
};

/// 表を出力
pub fn print_hyou(hp: &HyouProp, h: &Hyou) {
    for r in 0..hp.worker_count {
        // Wakuの行を出力
        print_waku_row(hp, h, r);

        // 統計情報
        print_waku_count_row(H, hp, h, r);
        print_waku_count_row(O, hp, h, r);
        print_waku_count_row(I, hp, h, r);
        print_waku_count_row(N, hp, h, r);
        print_waku_count_row(K, hp, h, r);
        print_waku_count_row(Y, hp, h, r);

        // 名前
        print!(" {}", hp.workers[r].name);

        println!();
    }

    println!();

    // 曜日を表示
    print_days(hp);

    // 日ごとの統計を表示
    print_waku_count_columns(N, hp, h);
    print_waku_count_columns(I, hp, h);
    print_waku_count_columns(A, hp, h);
    print_waku_count_columns(K, hp, h);
    print_waku_count_columns(O, hp, h);
    print_waku_count_columns(H, hp, h);

    // スコア表示
}

/// Wakuの行を出力
fn print_waku_row(hp: &HyouProp, h: &Hyou, r: usize) {
    for c in 0..hp.day_count{
        print!("{}",match h[r][c] {
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
        if c + 1 == hp.buffer {
            print!("|");
        }
    }
}

/// 指定した枠の数を出力
fn print_waku_count_row(target_w: Waku, hp: &HyouProp, h: &Hyou, r: usize) {
    let mut sum = 0;
    for c in hp.buffer..hp.day_count {
        if h[r][c] == target_w {
            sum += 1;
        }
    }
    print!(" {:>2}", sum);
}


/// 曜日を表示
fn print_days(hp: &HyouProp) {
    for c in 0..hp.day_count {
        print!("{}", match hp.days[c] {
            DayST::Weekday => "W",
            DayST::Holiday => "H",
            DayST::Furo => "F",
            DayST::Furo2 => "2",
            DayST::Weight => "G",
        });
        if c + 1 == hp.buffer {
            print!("|");
        }
    }
    println!();
}

/// 指定した枠の列の和を表示
fn print_waku_count_columns(target_w: Waku, hp: &HyouProp, h: &Hyou) {
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