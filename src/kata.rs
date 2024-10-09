
pub type Int = isize;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Copy)]
pub enum Waku {
    N,
    K,
    I,
    A,
    O,
    H,
    Y,
    D,
    U,
}

pub type Hyou = Vec<Vec<Waku>>;

pub type HyouRow = Vec<Waku>;

pub type HyouColumn = Vec<Waku>;

pub type Score = f32;

#[derive(PartialEq)]
pub enum WakuST {
    Absolute,
    Kibo,
    Random,
}

pub type HyouST = Vec<Vec<WakuST>>;

pub struct Worker {
    pub name: String,
    pub ability: Int,
}

pub type ID = Int;

pub enum DayST {
    Weekday,
    Holiday,
    Furo,
    Furo2,
    Weight,
}

pub type Days = Vec<DayST>;

pub type NG = (Int, Int);

pub type NGList = Vec<NG>;

pub type KCounts = Vec<Int>;

pub type ICounts = Vec<Int>;

pub type OCounts = Vec<Int>;

pub type HCounts = Vec<Int>;

pub enum ScoreProp {
    IAKrenzoku(Score),
    KIArenzoku(Score),
    KNIArenzokuP(Score),
    NNIArenzokuP(Score),
    ONrenzoku(Score),
    NHrenzoku(Score),
    OHrenzoku(Score),
    Renkin4((Score, Score)),
    Renkin5((Score, Score)),
    Renkin6((Score, Score)),
    Renkyuu(Score),
    Renkyuu2(Score),
    OsoHaya(Int),
    YakinBaransu(Int),
    OsoBaransu(Int),
    HayaBaransu(Int),
    KokyuCount(Int),
    YakinCount(Int),
    OsoCount(Int),
    HayaCount(Int),
    Fukouhei(Int),
    YakinNinzuu(Score),
    NikkinNinzuu(((Int,Int),(Int,Int),(Int,Int),(Int,Int),(Int,Int))),
    NG(Score),
    OsoNinzuu((Int,Int)),
    HayaNinzuu((Int,Int)),
    Leader((Score,Int)),
    YakinWorker((Score,Int)),
    YakinAloneFuro(Score),
    HeyaMoti((Score, Int, Int)),
    NoUndef(Int),
    NoSamePair(Score),
}

// pub struct HyouProp {
//     pub workers: Vec<Worker>,
//     pub ng_list: NGList,
//     pub bounds: (Int, Int),
//     pub days: Days,
//     pub buffer: Int,
//     pub kibou: Hyou,
//     pub k_counts: KCounts,
//     pub i_counts: ICounts,
//     pub o_counts: OCounts,
//     pub h_counts: HCounts,
//     pub i_ninzuu: Vec<Int>,
//     pub step: Int,
//     pub seed: Int,
//     pub score_prop: Vec<ScoreProp>,
// }


///勤務表で使う値
pub struct HyouProp {
    pub workers: Vec<Worker>,
    pub ng_list: NGList,
    pub bounds: (Int, Int),
    pub days: Days,
    pub buffer: Int,
    pub kibou: Hyou,
    pub hyou_st: HyouST,
    pub k_counts: KCounts,
    pub i_counts: ICounts,
    pub o_counts: OCounts,
    pub h_counts: HCounts,
    pub i_ninzuu: Vec<Int>,
    pub seed: Int, //fill関数のシード値
    pub score_prop: Vec<ScoreProp>, //結果表示のためのスコア
}
//下2つはここにあるべきじゃない気がする

///焼きなましの段階ごとの設定
pub struct AnnealingConfig {
    pub step: Int, //焼きなましのステップ数
    pub seed: Int, //焼きなましのupdate関数のシード
    pub score_prop: Vec<ScoreProp>, //焼きなましのためのスコア
    pub update_func: String,
    pub max_temp: f32,
    pub min_temp: f32, 
}