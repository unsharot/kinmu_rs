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
    pub ability: isize,
}

pub type ID = isize;

pub enum DayST {
    Weekday,
    Holiday,
    Furo,
    Furo2,
    Weight,
}

pub type Days = Vec<DayST>;

pub type NG = (isize, isize);

pub type NGList = Vec<NG>;

pub type KCounts = Vec<isize>;

pub type ICounts = Vec<isize>;

pub type OCounts = Vec<isize>;

pub type HCounts = Vec<isize>;

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
    OsoHaya(isize),
    YakinBaransu(isize),
    OsoBaransu(isize),
    HayaBaransu(isize),
    KokyuCount(isize),
    YakinCount(isize),
    OsoCount(isize),
    HayaCount(isize),
    Fukouhei(isize),
    YakinNinzuu(Score),
    NikkinNinzuu(((isize,isize),(isize,isize),(isize,isize),(isize,isize),(isize,isize))),
    NG(Score),
    OsoNinzuu((isize,isize)),
    HayaNinzuu((isize,isize)),
    Leader((Score,isize)),
    YakinWorker((Score,isize)),
    YakinAloneFuro(Score),
    HeyaMoti((Score, isize, isize)),
    NoUndef(isize),
    NoSamePair(Score),
}

// pub struct HyouProp {
//     pub workers: Vec<Worker>,
//     pub ng_list: NGList,
//     pub bounds: (isize, isize),
//     pub days: Days,
//     pub buffer: isize,
//     pub kibou: Hyou,
//     pub k_counts: KCounts,
//     pub i_counts: ICounts,
//     pub o_counts: OCounts,
//     pub h_counts: HCounts,
//     pub i_ninzuu: Vec<isize>,
//     pub step: isize,
//     pub seed: isize,
//     pub score_prop: Vec<ScoreProp>,
// }


///勤務表で使う値
pub struct HyouProp {
    pub workers: Vec<Worker>,
    pub ng_list: NGList,
    pub bounds: (isize, isize),
    pub days: Days,
    pub buffer: isize,
    pub kibou: Hyou,
    pub hyou_st: HyouST,
    pub k_counts: KCounts,
    pub i_counts: ICounts,
    pub o_counts: OCounts,
    pub h_counts: HCounts,
    pub i_ninzuu: Vec<isize>,
    pub seed: isize, //fill関数のシード値
    pub score_prop: Vec<ScoreProp>, //結果表示のためのスコア
}
//下2つはここにあるべきじゃない気がする

///焼きなましの段階ごとの設定
pub struct AnnealingConfig {
    pub step: isize, //焼きなましのステップ数
    pub seed: isize, //焼きなましのupdate関数のシード
    pub score_prop: Vec<ScoreProp>, //焼きなましのためのスコア
    pub update_func: String,
    pub max_temp: f32,
    pub min_temp: f32, 
}