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
    Renkyuu2NoBf(Score),
    OsoHayaBaransu(Score),
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

impl ScoreProp {
    pub fn show(&self) -> String {
        match self {
            ScoreProp::IAKrenzoku(p) => format!("IAKrenzoku({:?})", p),
            ScoreProp::KIArenzoku(p) => format!("KIArenzoku({:?})", p),
            ScoreProp::KNIArenzoku(p) => format!("KNIArenzoku({:?})", p),
            ScoreProp::NNIArenzoku(p) => format!("NNIArenzoku({:?})", p),
            ScoreProp::ONrenzoku(p) => format!("ONrenzoku({:?})", p),
            ScoreProp::NHrenzoku(p) => format!("NHrenzoku({:?})", p),
            ScoreProp::OHrenzoku(p) => format!("OHrenzoku({:?})", p),
            ScoreProp::Renkin4(p) => format!("Renkin4({:?})", p),
            ScoreProp::Renkin5(p) => format!("Renkin5({:?})", p),
            ScoreProp::Renkin6(p) => format!("Renkin6({:?})", p),
            ScoreProp::Renkyuu(p) => format!("Renkyuu({:?})", p),
            ScoreProp::Renkyuu2(p) => format!("Renkyuu2({:?})", p),
            ScoreProp::Renkyuu2NoBf(p) => format!("Renkyuu2NoBf({:?})", p),
            ScoreProp::OsoHayaBaransu(p) => format!("OsoHayaBaransu({:?})", p),
            ScoreProp::YakinBaransu(p) => format!("YakinBaransu({:?})", p),
            ScoreProp::OsoBaransu(p) => format!("OsoBaransu({:?})", p),
            ScoreProp::HayaBaransu(p) => format!("HayaBaransu({:?})", p),
            ScoreProp::KokyuCount(p) => format!("KokyuCount({:?})", p),
            ScoreProp::YakinCount(p) => format!("YakinCount({:?})", p),
            ScoreProp::OsoCount(p) => format!("OsoCount({:?})", p),
            ScoreProp::HayaCount(p) => format!("HayaCount({:?})", p),
            ScoreProp::Fukouhei(p) => format!("Fukouhei({:?})", p),
            ScoreProp::YakinNinzuu(p) => format!("YakinNinzuu({:?})", p),
            ScoreProp::NikkinNinzuu(p) => format!("NikkinNinzuu({:?})", p),
            ScoreProp::NG(p) => format!("NG({:?})", p),
            ScoreProp::OsoNinzuu(p) => format!("OsoNinzuu({:?})", p),
            ScoreProp::HayaNinzuu(p) => format!("HayaNinzuu({:?})", p),
            ScoreProp::Leader(p) => format!("Leader({:?})", p),
            ScoreProp::YakinWorker(p) => format!("YakinWorker({:?})", p),
            ScoreProp::YakinAloneFuro(p) => format!("YakinAloneFuro({:?})", p),
            ScoreProp::HeyaMoti(p) => format!("HeyaMoti({:?})", p),
            ScoreProp::NoUndef(p) => format!("NoUndef({:?})", p),
            ScoreProp::NoSamePair(p) => format!("NoSamePair({:?})", p),
            // _ => "NO WAY!!!".to_string(),
        }
    }
}


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