# kinmu

## 概要
看護職員の勤務表を自動生成するツールです。
configに設定を入力し、プログラムを実行することで条件に合った勤務表を生成します。
アルゴリズムは焼きなまし法(Simulated Annealing法)を採用しています。

看護職員の勤務表を生成する問題はナーススケジューリング問題(Nurse Scheduling Problem, NSP)と呼ばれます。
NSPの難しさは、日ごとの勤務パターンが独立していない点にあります。
夜勤の入りが入った日の次は必ず夜勤の明けがあり、そのあとは必ず休みを与える必要がある点は、ほかのスケジューリング問題とは違います。
よって、通常のスケジューリング問題ソルバーはこれに対応せず、NSP専用のソルバーを作成する必要がありました。
このソルバーの最も特殊な部分は、この夜勤パターンを崩さないような更新関数を作成した点にあります。

NSP問題を解くアルゴリズムや製品は多く存在します。
しかし、その多くは有料で、
既存のフリーソフトウェアとしてはProNuSがありますが、これは非オープンソースで非常に古いソフトウェアです。
私の環境(Windows11)でも動くことは確認できました。
非常によくできたソフトウェアだと思います。
多くのスコアが一般化されており、計算速度も１０秒ほどと高速でした。
しかし、一般化されているため、実際の勤務表作成においてProNuSでは複雑な事情を表現しきれませんでした。
例えば、同じペアで夜勤を何度もすることは望ましくないですが、これを評価することができません（おそらく）。
こういった、病院レベル、あるいは病棟レベルで異なる事情を考慮するためには、一般化されたソルバーでは対応しきれない問題が生じてしまいます。
他にも、pythonの遺伝的アルゴリズムソルバーのdeapなど、様々ですが。
そこで、オープンソースでのソルバーの公開を考えました。
オープンソースであることにより、プログラミングができるユーザーはフォークを作成して自分の環境に最適なソルバーを作ることができます。
具体的には、スコアやシフトパターンを追加したり、表示方法を変更したりすることができるはずです。

このソルバーはGUIを提供していません。
これは、ProNuSを使ってみて、入力が冗長であったことによります。
GUIを提供するのは良いですが、半端なGUIではUXを低下させることにつながります。
よってconfigファイルを用いた最低限のやりとりにとどめ、入力はすべてファイルで行えるようにしました。

はじめこのソルバーはHaskellで書いていたのですが、GCによる計算速度の低下が深刻である可能性があると判断し、
GCがないためC++並みの速度を持ち、Haskellと同等の型の表現力を持つRustで書き換えることにしました。
実際、コード内でScorePropというenum型を使用しているのですが、
これをC++で表現することは複雑になり過ぎるため、避けました。

## 導入方法

### 実行ファイルをダウンロードする場合

- kinmu.exeを任意のフォルダAに入れる A/kinmu.exeという状態
- A/config.yamlを作成
- A/config.yamlで指定したファイルに必要事項を記入
- ターミナルを開く
- Aへ移動する
- ./kinmu.exeをターミナルで実行

### ソースファイルをダウンロードする場合

リポジトリのクローン
```
git clone https://github.com/unsharot/kinmu_rs
```

#### ビルドしない場合

リリースビルドでの実行

```
cargo run --release
```

#### ビルドする場合

リリースビルドでのビルド

```
cargo build --release
```

./target/release/kinmu (Linux)
または
./target/release/kinmu.exe (Windows)
が生成されるので、「実行ファイルをダウンロードする場合」同様に実行

## 使い方
ファイル構成は以下の通りですが、メインの設定ファイルであるconfig/config.yaml以外は自由に配置することが可能です。
設定ファイル内では#を用いたコメントアウトが使えます。
また、改行やスペースの数は問題になりませんが、一行に書くべき情報を改行する場合はエラーとなります。

### メインconfig
呼び出すファイルを設定します。

#### Files
勤務表のconfigを列挙します。

##### 例
```
Files:
./config/configW.yaml
./config/configK.yaml
```

### 勤務表のconfig
勤務表に使う基本的な値の設定です。

#### 職員リスト
職員の能力、公休数、夜勤数、遅番数、早番数、名前を列挙します。

##### 例
```
職員リスト:
0 8 3 -1 -1 職員A
1 9 6  0  0 職員B
```

#### NGリスト
特定の職員同士が夜勤で同じ日の勤務にならないようにするための設定です。
Workerのリストで上から1,2,3..と番号を振っていき、その番号で指定します。
行ごとに必ず改行を挟んで設定してください。

##### 例
```
NGリスト:
1 2
6 2
```

#### 職員数
職員の数を指定します。
職員リストの長さより小さい数が指定された場合、職員リストの上からその数だけカウントされ、余剰分は無視されます。

##### 例
```
職員数:
12
```

#### 日数
勤務表の日数を指定します。
30日の月で、バッファー日数を3日に指定している場合、30日+バッファー日数3日 = 33日として指定してください。

##### 例
```
日数:
33
```

#### DayState
日数で指定した日数分の日ごとのステータスを設定します。ステータスは以下の通りです。

- W: WeekDay 平日
- H: Holiday 休日
- F: Bath フロ
- 2: Bath2 フロ2
- G: Weight 体重測定

##### 例
```
DayState:
W2WHHWFW2GHHWFW2WHHWFW2WHHWFW2WHH
```

#### バッファー日数
バッファーの日数を指定します。
バッファーというのは、先月の終わり３日分など、考慮するべき日数です。
3日分を考慮する際は3を設定してください。

##### 例
```
バッファー日数:
3
```

#### 希望
希望として出された勤務表を指定します。
横軸が日、縦軸が職員です。
すなわち、行が職員ごとの希望、列が日ごとの希望となります。
ステータスは以下の通りです。

- N: 日勤
- K: 公休
- I: 夜勤入り
- A: 夜勤明け
- O: 遅番
- H: 早番
- Y: 有給
- U: 未定

希望を絶対として焼きなましを行います。
未定の場所以外は絶対条件としてカウントされ、出力で変化していることはありません。
また、未定(U)はスペース( )での入力も可能です。
バッファーが重要でない場合、Uとしても出力で変化することはありません。

##### 例
```
希望:
AKNUUUUUUUUUUIAKYUUUUUUUUIAKUUUUU
KHIAKUUUUUUUUUUUIAKYUUUUUUUUUUUUU
OIAKUUUUUUUUUUUUUUUUUUUIAKYUUUUUU
NOKKUUUUUUUUUUUUUUUUUUUUUUUUUUUUY
IAKUUUUUUUUUUUUUUYUUUUUUUKUUUUUUU
HYNUUUUUUUUUUUUUKUUUUYUUUUUUUUUUU
KNOUUUUUUUUUUUUUUUUUUUYUUUUUUUUUU
KNIAIAKKIAKUUUUUUUUUUUUUUUUUUUYUU
KNHUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU
IAKUUUUUUUUUUUIAKUUUUUUUUUUUUUUUU
UUUKKUUUUYKKUUUUUKKUUUUUKKUUUUUKK
UUUKKUUUUUKKUUUUUKKKUUUKKKUUUUUKK
```

```
希望:
   KKNNNNNKKNNNNNKKNNNNNKKNNNNNKK
NIAKKK    KKYK                   
AKN KK                           
NNIAK                        IAKY
KNN       KK                     
KNN                              
KNN                  YKK         
IAK               KK             
UUUKKYYYYYKYYYYYYKKYYYYYKYYYYYYKK
NKN      K                K      
NIAK                             
NNN             K   K     K      
NON    KK       KK               
ONK                              
AKO        K             K       
KNK                              
```

#### 夜勤の人数リスト
職員ごとの１カ月の夜勤(入り)の日数を指定します。
値はスペースで区切ります。

##### 例
```
夜勤の日数リスト:
3 6 6 6 6 6 6 6 3 6 0 0
```

#### fillの関数
焼きなましの前に未定(U)の場所をうめるための関数を指定します。
以下の関数があります。

- fill1: それぞれの枠をランダムな要素で埋めます。
- fill2: 夜勤の数と公休の数を守り、入りと明けの連続やその後の休みも加味して埋めます。

##### 例
```
fillの関数:
fill2
```

#### fillのシード値
fillの乱数に用いるシード値を指定します。
型はusizeで、最小値は0、最大値はusizeの上限で、64bitなら18446744073709551615、32bitなら4294967295です。
0の場合、乱数に用いるシード値はランダムで選ばれます。
それ以外の場合固定です。
再現性のあるテストがしたいときは固定し、実際に使う場合は0が良いでしょう。

##### 例
```
fillのシード値:
53
```

#### アニーリング
焼きなましの設定ファイルのパスを指定します。
焼きなましはここで列挙した順に行われます。

##### 例
```
アニーリング:
./config/anconfigW/randomWalk.yaml
./config/anconfigW/W1.yaml
./config/anconfigW/W2.yaml
```

#### 結果のスコア
焼きなまし終了後、結果を表示する際に用いるスコアを列挙します。
スコアの名前と、そのスコアに用いるパラメータを指定します。
順不同です。スコアは以下のとおりです。


| Prop名                 | 引数の型               | 説明                                                                                                                                                                                                                                   |
| :--------------------- | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAKpattern             | f32                    | 入り、明け、休みの順番が取れていない場合のペナルティを指定します。                                                                                                                                                                     |
| KIApattern             | f32                    | 公休、入り、明けの順番になっている場合のペナルティを指定します。                                                                                                                                                                       |
| KNIApattern            | f32                    | 公休、日勤、入り、明けの順番になっている場合のペナルティを指定します。                                                                                                                                                                 |
| NNIApattern            | f32                    | 日勤、日勤、入り、明けの順番になっている場合の報酬を指定します。                                                                                                                                                                       |
| ONpattern              | f32                    | 遅番、日勤の順番になっている場合のペナルティを指定します。                                                                                                                                                                             |
| NHpattern              | f32                    | 日勤、早番の順番になっている場合のペナルティを指定します。                                                                                                                                                                             |
| OHpattern              | f32                    | 遅番、早番の順番になっている場合のペナルティを指定します。                                                                                                                                                                             |
| WorkingDayStreak4      | (f32, f32)             | 4日連続勤務の場合のペナルティを指定します。                                                                                                                                                                                            |
| WorkingDayStreak5      | (f32, f32)             | WorkingDayStreak4同様、5日連続勤務の場合のペナルティを指定します。                                                                                                                                                                     |
| WorkingDayStreak6      | (f32, f32)             | WorkingDayStreak4同様、6日連続勤務の場合のペナルティを指定します。                                                                                                                                                                     |
| HolidayReward          | f32                    | 連休の場合の報酬を指定します。2日の連休ごとに発生します。                                                                                                                                                                              |
| Need2Holidays          | f32                    | 職員ごとに、一カ月の間に2連休がない場合のペナルティを指定します。                                                                                                                                                                      |
| Need2HolidaysNoBf      | f32                    | Need2Holidays同様のスコアですが、バッファーを含みません。                                                                                                                                                                              |
| OHBalance              | f32                    | 職員ごとの、1か月の遅番と早番のバランスによるペナルティの倍率を指定します。                                                                                                                                                            |
| ShiftHalfBalance       | (Shift, f32)           | 指定したシフトが夜勤の月の前半と後半でバランスが取れていない場合のペナルティの倍率を指定します。                                                                                                                                       |
| ShiftDirPriority       | (Shift, f32)           | 指定したシフトが月の前後どちらにあるほうが良いか設定します。実数フィールドが正なら前を優先、負なら後ろを優先します。                                                                                                                   |
| KDayCount              | f32                    | 職員ごとの公休の数がconfigファイルで設定した通りになっていない場合のペナルティの倍率を指定します。                                                                                                                                     |
| IDayCount              | f32                    | 職員ごとの夜勤の数がconfigファイルで設定した通りになっていない場合のペナルティの倍率を指定します。                                                                                                                                     |
| ODayCount              | f32                    | 職員ごとの遅番の数がconfigファイルで設定した通りになっていない場合のペナルティの倍率を指定します。                                                                                                                                     |
| HDayCount              | f32                    | 職員ごとの早番の数がconfigファイルで設定した通りになっていない場合のペナルティの倍率を指定します。                                                                                                                                     |
| IStaffCount            | f32                    | 夜勤の人数がconfigファイルで設定した通りになっていない場合のペナルティを指定します。                                                                                                                                                   |
| NStaffCount            | (DayState, isize, f32) | 曜日ごとの日勤の人数と、その通りになっていない場合のペナルティの倍率を指定します。引数は3つで、1つ目が曜日、2つ目が人数、3つ目が倍率です。曜日は平日(W)、休日(H)、フロ(F)、フロ2(2)、Weight(G)に対応します。                           |
| OStaffCount            | (isize, f32)           | 遅番の人数と、その通りになっていない場合のペナルティを指定します。                                                                                                                                                                     |
| HStaffCount            | (isize, f32)           | 早番の人数と、その通りになっていない場合のペナルティを指定します。                                                                                                                                                                     |
| NGPair                 | f32                    | NGに指定されたペアが夜勤で同じ日になる場合のペナルティを指定します。                                                                                                                                                                   |
| LeaderAbility          | (isize, f32)           | リーダーとしての能力の番号と、祝日の日勤にリーダーがいない場合のペナルティを指定します。職員の能力を参照し、職員の能力がここで指定する番号で割り切れないならリーダーとします。                                                         |
| IAloneAbility          | (isize, f32)           | 一人で夜勤ができるワーカーの能力の番号と、ワーカーの夜勤が一人で、それが看護にワーカーの仕事を教えられる人でない場合のペナルティを指定します。職員の能力を参照し、職員の能力がここで指定する番号で割り切れないなら能力があるとします。 |
| IAloneBeforeBath       | f32                    | フロ日の前にワーカーの一人夜勤がある場合のペナルティを指定します。                                                                                                                                                                     |
| NStaffCountWithAbility | (isize, isize, f32)    | 対象の能力の番号と、一日に必要な能力持ちの人数と、能力持ちの人数が十分でない場合のペナルティを指定します。                                                                                                                             |
| NoSamePair3            | f32                    | 夜勤で同じペアが3回以上ある場合のペナルティを指定します。                                                                                                                                                                              |
| NoSamePair2            | f32                    | 夜勤で同じペアが2回以上ある場合のペナルティを指定します。                                                                                                                                                                              |
| NoUndef                | f32                    | 未定(U)の枠があった場合のペナルティの倍率を指定します。                                                                                                                                                                                |



##### 例
```
結果のスコア:
IAKpattern 1000
KIApattern 100
KNIApattern 10
NNIApattern 300
ONpattern 100
NHpattern 1000
OHpattern 2000
WorkingDayStreak4 (1000,200)
WorkingDayStreak5 (4000,1000)
WorkingDayStreak6 (10000,4000)
NGPair 1000
HolidayReward 2
Need2Holidays 1000
Need2HolidaysNoBf 1000
OHBalance 3
ShiftHalfBalance (I,10)
ShiftHalfBalance (O,3)
ShiftHalfBalance (H,3)
KDayCount 10
IDayCount 10
ODayCount 100
HDayCount 100
IStaffCount 10
NStaffCount (F,4,5)
NStaffCount (2,2,5)
NStaffCount (W,2,5)
NStaffCount (H,2,5)
NStaffCount (G,2,5)
OStaffCount (1,100)
HStaffCount (1,100)
IAloneAbility (2,5000)
IAloneBeforeBath 1000
NoUndef 10
NoSamePair3 1000
NoSamePair2 500
```

### 焼きなましのconfig
焼きなまし法で用いるパラメータを設定します。

#### ステップ数
焼きなましのステップ数を指定します。

##### 例
```
ステップ数:
20000
```

#### 乱数のシード
焼きなまし法の更新関数に用いる乱数のシード値を指定します。
型はusizeで、最小値は0、最大値はusizeの上限で、64bitなら18446744073709551615、32bitなら4294967295です。
0の場合、乱数に用いるシード値はランダムで選ばれます。
それ以外の場合固定です。
再現性のあるテストがしたいときは固定し、実際に使う場合は0が良いでしょう。

```
乱数のシード:
6554
```

#### 各スコアのパラメータ
焼きなましに用いるスコアのパラメータを指定します。
順不同です。
パラメータは結果のスコアと同じです。


##### 例
```
各スコアのパラメータ:
KIApattern 100
KNIApattern 10
NNIApattern 300
WorkingDayStreak4 (1000,200)
WorkingDayStreak5 (4000,1000)
WorkingDayStreak6 (10000,4000)
Need2Holidays 1000
Need2HolidaysNoBf 500
NGPair 1000
ShiftHalfBalance (I,10)
IStaffCount 100
NStaffCount (F,6,4)
NStaffCount (2,4,4)
NStaffCount (W,4,4)
NStaffCount (H,4,4)
NStaffCount (G,4,4)
IAloneAbility (2,5000)
IAloneBeforeBath 100
NoSamePair3 1000
NoSamePair2 500
```

#### 更新関数
焼きなましの更新に用いる更新関数を指定します。
更新関数は以下の通りです。

- update4: N,O,HをN,O,Hのうちのランダムな要素に入れ替えます。
- update5: 夜勤と公休をランダムに移動します。夜勤の数や公休の数は維持されます。

##### 例
```
更例関数:
update5
```

#### max_and_min_temp
焼きなましの最高温度と最高温度を実数で指定します。
序盤に許容するスコアの悪化幅を指定するとよいでしょう。

##### 例
```
max_and_min_temp:
25 0
```

## ファイル構成

```
.
│  .gitignore
│  Cargo.lock
│  Cargo.toml
│  README.md
│
├─annealing
│  │  .gitignore
│  │  Cargo.lock
│  │  Cargo.toml
│  │
│  └─src
│          annealing.rs
│          lib.rs
│
└──src
   │  lib.rs
   │  main.rs
   │
   ├─io
   │  │  display.rs
   │  │  mod.rs
   │  │
   │  └─reader
   │          common.rs
   │          mod.rs
   │          seed.rs
   │          type_reader.rs
   │
   └─kinmu_lib
           check.rs
           fill.rs
           mod.rs
           score.rs
           types.rs
           update.rs
```

## 改造するには

CLIの出力形式を変更したかったり、GUIを実装したかったりする場合はsrc/io内のコードを

職場に特有の考慮事項がある場合、kinmu_libを

アルゴリズムを焼きなまし法から山登り法などに変更したい場合、annealingを

それぞれ編集してください。