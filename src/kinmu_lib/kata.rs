use std::str::FromStr;
use std::fmt;

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

impl fmt::Display for Waku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Waku::N => "N",
            Waku::K => "K",
            Waku::I => "I",
            Waku::A => "A",
            Waku::O => "O",
            Waku::H => "H",
            Waku::Y => "Y",
            Waku::D => "D",
            Waku::U => "U",
        };
        write!(f, "{}", s)
    }
}

pub type Hyou = Vec<Vec<Waku>>;

pub type HyouRow = Vec<Waku>;

pub type HyouColumn = Vec<Waku>;

pub type Score = f32;

#[derive(PartialEq)]
pub enum WakuST {
    Absolute,
    Random,
}

pub type HyouST = Vec<Vec<WakuST>>;

pub struct Worker {
    pub name: String,
    pub ability: isize,
    pub k_count: isize,
    pub i_count: isize,
    pub o_count: isize,
    pub h_count: isize,
}

pub type ID = usize;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DayST {
    Weekday,
    Holiday,
    Furo,
    Furo2,
    Weight,
}

impl fmt::Display for DayST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            DayST::Weekday => "W",
            DayST::Holiday => "H",
            DayST::Furo => "F",
            DayST::Furo2 => "2",
            DayST::Weight => "G",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for DayST {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "W" => Ok(DayST::Weekday),
            "H" => Ok(DayST::Holiday),
            "F" => Ok(DayST::Furo),
            "2" => Ok(DayST::Furo2),
            "G" => Ok(DayST::Weight),
            _ => Err(format!("Failed to parse DayST: {}", s))
        }
    }
}

pub type Days = Vec<DayST>;

pub type NG = (usize, usize);

pub type NGList = Vec<NG>;

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
    YakinBaransu(Score),
    OsoBaransu(Score),
    HayaBaransu(Score),
    KokyuCount(Score),
    YakinCount(Score),
    OsoCount(Score),
    HayaCount(Score),
    Fukouhei(usize),
    YakinNinzuu(Score),
    NikkinNinzuu((DayST,isize,Score)),
    OsoNinzuu((isize,Score)),
    HayaNinzuu((isize,Score)),
    NGPair(Score),
    Leader((isize,Score)),
    YakinAloneWorker((isize,Score)),
    YakinAloneBeforeFuro(Score),
    HeyaMoti((isize,isize,Score)),
    NoSamePair3(Score),
    NoSamePair2(Score),
    NoUndef(Score),
}

impl fmt::Display for ScoreProp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
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
            ScoreProp::OsoNinzuu(p) => format!("OsoNinzuu({:?})", p),
            ScoreProp::HayaNinzuu(p) => format!("HayaNinzuu({:?})", p),
            ScoreProp::NGPair(p) => format!("NGPair({:?})", p),
            ScoreProp::Leader(p) => format!("Leader({:?})", p),
            ScoreProp::YakinAloneWorker(p) => format!("YakinAloneWorker({:?})", p),
            ScoreProp::YakinAloneBeforeFuro(p) => format!("YakinAloneBeforeFuro({:?})", p),
            ScoreProp::HeyaMoti(p) => format!("HeyaMoti({:?})", p),
            ScoreProp::NoSamePair3(p) => format!("NoSamePair3({:?})", p),
            ScoreProp::NoSamePair2(p) => format!("NoSamePair2({:?})", p),
            ScoreProp::NoUndef(p) => format!("NoUndef({:?})", p),
        };
        write!(f, "{}", s)
    }
}


/// 勤務表ごとの設定
pub struct HyouProp {
    pub workers: Vec<Worker>,
    pub ng_list: NGList,
    pub worker_count: usize,
    pub day_count: usize,
    pub days: Days,
    pub buffer: usize,
    pub kibou: Hyou,
    pub hyou_st: HyouST,
    pub i_ninzuu: Vec<isize>,
    pub score_props: Vec<ScoreProp>, // 結果表示のためのスコア
}

pub struct FillConfig {
    pub name: String,
    pub seed: usize,
}

/// 焼きなましの段階ごとの設定
pub struct AnnealingConfig {
    pub step: usize, // 焼きなましのステップ数
    pub seed: usize, // 焼きなましのupdate関数のシード
    pub score_props: Vec<ScoreProp>, // 焼きなましのためのスコア
    pub update_func: String,
    pub max_temp: f32,
    pub min_temp: f32,
}