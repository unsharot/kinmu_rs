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
    pub ability: usize,
}

pub type ID = usize;

pub enum DayST {
    Weekday,
    Holiday,
    Furo,
    Furo2,
    Weight,
}

pub type Days = Vec<DayST>;

pub type NG = (usize, usize);

pub type NGList = Vec<NG>;

pub type KCounts = Vec<usize>;

pub type ICounts = Vec<usize>;

pub type OCounts = Vec<isize>;

pub type HCounts = Vec<isize>;

pub enum ScoreProp {
    IAKrenzoku(Score),
    KIArenzoku(Score),
    KNIArenzoku(Score),
    NNIArenzoku(Score),
    ONrenzoku(Score),
    NHrenzoku(Score),
    OHrenzoku(Score),
    Renkin4((Score, Score)),
    Renkin5((Score, Score)),
    Renkin6((Score, Score)),
    Renkyuu(Score),
    Renkyuu2(Score),
    OsoHayaBaransu(isize),
    YakinBaransu(usize),
    OsoBaransu(usize),
    HayaBaransu(usize),
    KokyuCount(Score),
    YakinCount(Score),
    OsoCount(Score),
    HayaCount(Score),
    Fukouhei(usize),
    YakinNinzuu(Score),
    NikkinNinzuu(((usize,usize),(usize,usize),(usize,usize),(usize,usize),(usize,usize))),
    NG(Score),
    OsoNinzuu((usize,usize)),
    HayaNinzuu((usize,usize)),
    Leader((Score,usize)),
    YakinWorker((Score,usize)),
    YakinAloneFuro(Score),
    HeyaMoti((Score, usize, usize)),
    NoUndef(usize),
    NoSamePair(Score),
}

// pub struct HyouProp {
//     pub workers: Vec<Worker>,
//     pub ng_list: NGList,
//     pub bounds: (usize, usize),
//     pub days: Days,
//     pub buffer: usize,
//     pub kibou: Hyou,
//     pub k_counts: KCounts,
//     pub i_counts: ICounts,
//     pub o_counts: OCounts,
//     pub h_counts: HCounts,
//     pub i_ninzuu: Vec<usize>,
//     pub step: usize,
//     pub seed: usize,
//     pub score_prop: Vec<ScoreProp>,
// }


///勤務表で使う値
pub struct HyouProp {
    pub workers: Vec<Worker>,
    pub ng_list: NGList,
    // pub bounds: (usize, usize),
    pub worker_count: usize,
    pub day_count: usize,
    pub days: Days,
    pub buffer: usize,
    pub kibou: Hyou,
    pub hyou_st: HyouST,
    pub k_counts: KCounts,
    pub i_counts: ICounts,
    pub o_counts: OCounts,
    pub h_counts: HCounts,
    pub i_ninzuu: Vec<usize>,
    pub seed: usize, //fill関数のシード値
    pub score_props: Vec<ScoreProp>, //結果表示のためのスコア
}
//下2つはここにあるべきじゃない気がする

///焼きなましの段階ごとの設定
pub struct AnnealingConfig {
    pub step: usize, //焼きなましのステップ数
    pub seed: usize, //焼きなましのupdate関数のシード
    pub score_props: Vec<ScoreProp>, //焼きなましのためのスコア
    pub update_func: String,
    pub max_temp: f32,
    pub min_temp: f32, 
}