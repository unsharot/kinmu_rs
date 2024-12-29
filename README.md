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

このソルバーはGUI入力を提供していません。
configファイルを用いた最低限のやりとりにとどめ、入力はすべてファイルで行えるようにしました。

## 導入方法

### 実行ファイルをダウンロードする場合

- kinmu.exeを任意のフォルダAに入れる A/kinmu.exeという状態
- ディレクトリA/configを作成
- A/config.tomlを作成
- A/config.tomlで指定したファイルに必要事項を記入
- ターミナルを開く
- Aへ移動する
- ./kinmu.exeをターミナルで実行

exeファイルを実行する際、デフォルトでメインconfigとしてconfig/config.tomlが読み込まれますが、
読み込むconfigを指定することもできます。
以下のように引数にファイル名を指定することで、読み込むファイルを指定することができます。
モードを切り替えて使いたい場合にご利用ください。

```sh
./kinmu.exe config/hoge/config.toml
```

### ソースファイルをダウンロードする場合

リポジトリのクローン
```sh
git clone https://github.com/unsharot/kinmu_rs
```

#### ビルドしない場合

リリースビルドでの実行

```sh
cargo run --release
```

#### ビルドする場合

リリースビルドでのビルド

```sh
cargo build --release
```

./target/release/kinmu (Linux)
または
./target/release/kinmu.exe (Windows)
が生成されるので、「実行ファイルをダウンロードする場合」同様に実行

## 使い方
ファイル構成は以下の通りですが、メインの設定ファイルであるconfig/config.toml以外は自由に配置することが可能です。
また、引数でメインの設定ファイルを指定する場合、このメインファイルも自由に配置可能です。
設定ファイルはtoml形式に従うため、`#`を用いたコメントアウトが使用可能です。
また、改行やスペースの数は問題になりません。
各ファイルの設定方法については、以下の説明や附属のconfig_templateフォルダを参照してください。

### main_config
呼び出すファイルを設定します。

#### schedule_config_paths
勤務表のconfigを列挙します。

例
```toml
schedule_config_paths = [
   "./config/configW.toml",
   "./config/configK.toml",
]
```

#### thread_count
焼きなましに用いるスレッド数を指定します。
結果もここで指定した数だけ表示されます。
CPUの論理プロセッサ数を指定すると高速に動作します。

例
```toml
thread_count = 8
```

### schedule_config
勤務表に使う基本的な値の設定です。

#### staff
スタッフに関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[staff]
```

##### attributes
職員ごとのパラメータ名を設定します。
日本語でも可です。

例
```toml
[staff]
attributes = [
   "Ability",
   "KDayCount",
   "IDayCount",
   "ODayCount",
   "HDayCount",
]
```
```toml
[staff]
attributes = [
   "能力",
   "公休数",
   "夜勤数",
   "遅番数",
   "早番数",
]
```

##### list
職員の能力、attributesで指定したパラメータ、名前を列挙します。
コメントで項目名を記しておくと便利です。

例
```toml
list = [
   {name = "職員A", attributes = [0, 8, 3, -1, -1]},
   {name = "職員B", attributes = [1, 9, 6,  0,  0]},
]
```

```toml
list = [
   #        名前                  能力 公休   夜勤   遅番  早番  番号
   {name = "職員A", attributes = [0,   8,    3,    -1,   -1]}, #0
   {name = "職員B", attributes = [1,   9,    6,     0,    0]}, #1
]
```

##### ng_list
特定の職員同士が夜勤で同じ日の勤務にならないようにするための設定です。
職員リストで上から0,1,2..と番号を振っていき、その番号で指定します。
行ごとに必ず改行を挟んで設定してください。

例
```toml
ng_list = [
   {from = 0, to = 1},
   {from = 5, to = 1},
]
```

##### count
職員の数を指定します。
職員リストの長さより小さい数が指定された場合、職員リストの上からその数だけカウントされ、余剰分は無視されます。

例
```toml
count = 12
```

#### day
日付に関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[day]
```

##### day_count
勤務表の日数を指定します。
30日の月で、バッファー日数を3日に指定している場合、30日+バッファー日数3日 = 33日として指定してください。

例
```toml
day_count = 33
```

##### states
日数で指定した日数分の日ごとの状態を設定します。状態は以下の通りです。

- W: WeekDay 平日
- H: Holiday 休日
- F: Bath フロ
- 2: Bath2 フロ2
- G: Weight 体重測定

例
```toml
states = "W2WHHWFW2GHHWFW2WHHWFW2WHHWFW2WHH"
```

##### buffer_count
バッファーの日数を指定します。
バッファーというのは、先月の終わり３日分など、考慮するべき日数です。
3日分を考慮する際は3を設定してください。

例
```toml
buffer_count = 3
```

##### requested_schedule
希望として出された勤務表を指定します。
横軸が日、縦軸が職員です。
すなわち、行が職員ごとの希望、列が日ごとの希望となります。
シフトは以下の通りです。

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
コメントで職員名と日付を記しておくと便利です。

例
```toml
requested_schedule = [
   "AKNUUUUUUUUUUIAKYUUUUUUUUIAKUUUUU",
   "KHIAKUUUUUUUUUUUIAKYUUUUUUUUUUUUU",
   "OIAKUUUUUUUUUUUUUUUUUUUIAKYUUUUUU",
   "NOKKUUUUUUUUUUUUUUUUUUUUUUUUUUUUY",
   "IAKUUUUUUUUUUUUUUYUUUUUUUKUUUUUUU",
   "HYNUUUUUUUUUUUUUKUUUUYUUUUUUUUUUU",
   "KNOUUUUUUUUUUUUUUUUUUUYUUUUUUUUUU",
   "KNIAIAKKIAKUUUUUUUUUUUUUUUUUUUYUU",
   "KNHUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU",
   "IAKUUUUUUUUUUUIAKUUUUUUUUUUUUUUUU",
   "UUUKKUUUUYKKUUUUUKKUUUUUKKUUUUUKK",
   "UUUKKUUUUUKKUUUUUKKKUUUKKKUUUUUKK",
]
```

```toml
requested_schedule = [
   #W2WHHWFW2GHHWFW2WHHWFW2WHHWFW2WHH
   #000123456789012345678901234567890
   "   KKNNNNNKKNNNNNKKNNNNNKKNNNNNKK", #職員A
   "NIAKKK    KKYK                   ", #職員B
   "AKN KK                           ", #職員C
   "NNIAK                        IAKY", #職員D
   "KNN       KK                     ", #職員E
   "KNN                              ", #職員F
   "KNN                  YKK         ", #職員G
   "IAK               KK             ", #職員H
   "UUUKKYYYYYKYYYYYYKKYYYYYKYYYYYYKK", #職員I
   "NKN      K                K      ", #職員J
   "NIAK                             ", #職員K
   "NNN             K   K     K      ", #職員L
   "NON    KK       KK               ", #職員M
   "ONK                              ", #職員N
   "AKO        K             K       ", #職員O
   "KNK                              ", #職員P
]
```

##### attributes
日付ごとのパラメータ名と値を設定します。
パラメータの数は任意です。
値はスペースで区切ります。

```toml
attributes = [
   {name = "IStaffCount", values = [0, 0, 0, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2]},
   {name = "OStaffCount", values = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]},
]
```

#### fill
fillに関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[fill]
```

##### function
焼きなましの前に未定(U)の場所をうめるための関数を指定します。
以下の関数があります。

- fill1: それぞれの枠をランダムな要素で埋めます。
- fill2: 夜勤の数と公休の数を守り、入りと明けの連続やその後の休みも加味して埋めます。

例
```toml
function = "fill2"
```

##### seed
fillの乱数に用いるシード値を指定します。
型はusizeで、最小値は0、最大値はusizeの上限で、64bitなら18446744073709551615、32bitなら4294967295です。
0の場合、乱数に用いるシード値はランダムで選ばれます。
それ以外の場合固定です。
再現性のあるテストがしたいときは固定し、実際に使う場合は0が良いでしょう。

例
```toml
seed = 53
```

#### annealing
焼きなましに関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[annealing]
```

##### config_paths
焼きなましの設定ファイルのパスを指定します。
焼きなましはここで列挙した順に行われます。

例
```toml
config_paths = [
   "./config/anconfigW/randomWalk.toml",
   "./config/anconfigW/W1.toml",
   "./config/anconfigW/W2.toml",
]
```

#### result
結果に関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[result]
```

##### score_functions
焼きなまし終了後、結果を表示する際に用いるスコアを列挙します。
スコアの名前と、そのスコアに用いるパラメータを指定します。
順不同です。スコアは以下のとおりです。

| Prop名                       | 引数の型                                        | 説明                                                                                                    |
| :--------------------------- | :---------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
| PatternGeneral               | (Cond, [[Shift]], Score)                        | 指定したシフトパターンが出現した場合のペナルティを指定                                                  |
| PatternFixed                 | (Cond, [Shift], Score)                          | 指定したシフトパターンが出現した場合のペナルティを指定                                                  |
| PatternGeneralAny            | (Cond, [[Shift]], Score)                        | 指定したシフトパターンが出現する職員ごとにペナルティを指定                                              |
| PatternFixedAny              | (Cond, [Shift], Score)                          | 指定したシフトパターンが出現する職員ごとにペナルティを指定                                              |
| Streak                       | (Cond, [Shift], isize, Score)                   | 指定したシフトが指定した回数連続した場合のペナルティを指定                                              |
| ShiftsBalance                | (Cond, Shift, Shift, Score)                     | 指定した2つのシフトのバランスが悪い場合のペナルティを指定                                               |
| ShiftHalfBalance             | (Cond, Shift, Score)                            | 指定したシフトが指定範囲の前半と後半でバランスが取れていない場合のペナルティを指定                      |
| ShiftDirPriority             | (Cond, Shift, Score)                            | 指定したシフトが指定範囲の前後どちらにあるほうが良いか指定 指定スコアが正なら前を優先、負なら後ろを優先 |
| DayCountRegardStaffAttribute | (Cond, Shift, StaffAttributeName, Score)        | 職員ごとの指定したパラメータと指定したシフトの数の差によるペナルティを指定                              |
| StaffCountRegardDayAttribute | (Cond, Shift, DayAttributeName, Score)          | 日付ごとの指定したパラメータと指定したシフトの数の差によるペナルティを指定                              |
| StaffCount                   | (Cond, Shift, isize, Score)                     | 指定した値と指定したシフトの人数の差によるペナルティを指定                                              |
| StaffCountWithPremise        | (Cond, Shift, isize, Cond, Shift, isize, Score) | 指定したシフトの人数を満たした日付に対して、指定した値と指定したシフトの人数の差によるペナルティを指定  |
| NGPair                       | (Cond, Shift, Score)                            | NGに指定されたペアが指定したシフトで同じ日になる場合のペナルティを指定                                  |
| NoSamePair                   | (Cond, isize, Shift, Score)                     | 指定したシフトで同じペアが指定回数以上ある場合のペナルティを指定                                        |

型の詳細は以下の通り

| 型名      | 説明                             | 例                                             |
| :-------- | :------------------------------- | :--------------------------------------------- |
| Cond      | スコアを適用する勤務表の枠の条件 | And (DayExceptBuffer (), ParticularDayState B) |
| Shift     | シフト N,K,I,A,O,H,Y,D,U         | N                                              |
| [Shift]   | シフトのリスト                   | [N, O, H]                                      |
| [[Shift]] | シフトのリストのリスト           | [[N], [K, Y]]                                  |
| Score     | スコア 実数                      | -100.3                                         |
| isize     | 整数                             | -3                                             |
| usize     | 非負整数                         | 4                                              |
| DayState  | 曜日 W,H,B,2,M                   | B                                              |

Condの詳細は以下の通り

| 種類               | 引数の型       | 説明                                                               |
| :----------------- | :------------- | :----------------------------------------------------------------- |
| Every              | ()             | すべての枠を有効とする                                             |
| Or                 | (Cond, Cond)   | 指定した2つのCondのどちらかを満たしていれば有効とする              |
| And                | (Cond, Cond)   | 指定した2つのCondの両方を満たしていれば有効とする                  |
| Not                | Cond           | 指定した条件を満たしていなければ有効とする                         |
| DayExceptBuffer    | ()             | バッファーでないなら有効                                           |
| DayInRange         | (usize, usize) | 指定した範囲の日付でないなら有効 日数はバッファーから0,1,2..と続く |
| ParticularDayState | DayState       | 指定の曜日なら有効                                                 |
| BeforeDayState     | DayState       | 指定の曜日の前日なら有効                                           |
| ParticularDay      | usize          | 指定の日付のみ有効                                                 |
| StaffInRange       | (usize, usize) | 指定した範囲のスタッフなら有効                                     |
| StaffWithAbility   | isize          | 指定した番号の能力を持つスタッフなら有効                           |
| ParticularStaff    | usize          | 指定した番号のスタッフなら有効                                     |

例
```toml
score_functions = [
   "PatternGeneral (Every (), [[I], [N,O,H,I,K,Y]], 1000)",
   "PatternGeneral (Every (), [[A], [N,O,H,I,A]], 1000)",
   "PatternFixed (Every (), [K,I], 100)",
   "PatternFixed (Every (), [Y,I], 100)",
   "PatternGeneral (Every (), [[K,Y],[N,O,H],[I]], 10)",
   "PatternGeneral (Every (), [[N,O,H],[N,O,H],[I]], -300)",
   "PatternFixed (Every (), [O,N], 100)",
   "PatternFixed (Every (), [N,H], 1000)",
   "PatternFixed (Every (), [O,H], 2000)",
   "Streak (Every (), [N,O,H,I,A], 4, 200)",
   "Streak (Every (), [N,O,H,I,A], 5, 1000)",
   "Streak (Every (), [N,O,H,I,A], 6, 4000)",
   "Streak (Every (), [N,O,H,I,A], 7, 10000)",
   "NGPair (DayExceptBuffer (), I, 1000)",
   "Streak (Every (), [K,Y], 2, -100)",
   "Need2Holidays (Every (), [K,Y], 1000)",
   "Need2Holidays (DayExceptBuffer (), [K,Y], 1000)",
   "ShiftsBalance (DayExceptBuffer (), O, H, 3)",
   "ShiftHalfBalance (DayExceptBuffer (), I, 10)",
   "ShiftHalfBalance (DayExceptBuffer (), O, 3)",
   "ShiftHalfBalance (DayExceptBuffer (), H, 3)",
   "DayCountRegardStaffAttribute (DayExceptBuffer (), K, KDayCount, 10)",
   "DayCountRegardStaffAttribute (DayExceptBuffer (), I, IDayCount, 10)",
   "DayCountRegardStaffAttribute (DayExceptBuffer (), O, ODayCount, 100)",
   "DayCountRegardStaffAttribute (DayExceptBuffer (), H, HDayCount, 100)",
   "StaffCountRegardDayAttribute (DayExceptBuffer (), I, IStaffCount, 10)",
   "StaffCount (And (DayExceptBuffer (), ParticularDayState B), N, 4, 5)",
   "StaffCount (And (DayExceptBuffer (), ParticularDayState 2), N, 2, 5)",
   "StaffCount (And (DayExceptBuffer (), ParticularDayState W), N, 2, 5)",
   "StaffCount (And (DayExceptBuffer (), ParticularDayState H), N, 2, 5)",
   "StaffCount (And (DayExceptBuffer (), ParticularDayState M), N, 2, 5)",
   "StaffCount (DayExceptBuffer (), O, 1, 100)",
   "StaffCount (DayExceptBuffer (), H, 1, 100)",
   "StaffCountWithPremise (DayExceptBuffer (), I, 1, And (DayExceptBuffer (), StaffWithAbility 2), I, 1, 70)",
   "StaffCount (And (BeforeDayState B, DayExceptBuffer ()), I, 1, 30)",
   "StaffCount (DayExceptBuffer (), U, 0, 100000)",
   "NoSamePair (DayExceptBuffer (), 3, I, 1000)",
   "NoSamePair (DayExceptBuffer (), 2, I, 500)",
]
```

### annealing_config
焼きなまし法で用いるパラメータを設定します。

#### step_count
焼きなましのステップ数を指定します。

例
```toml
step_count = 20000
```

#### seed
焼きなまし法の更新関数に用いる乱数のシード値を指定します。
型はusizeで、最小値は0、最大値はusizeの上限で、64bitなら18446744073709551615、32bitなら4294967295です。
0の場合、乱数に用いるシード値はランダムで選ばれます。
それ以外の場合固定です。
再現性のあるテストがしたいときは固定し、実際に使う場合は0が良いでしょう。

例
```toml
seed = 6554
```

#### score_functions
焼きなましに用いるスコアとパラメータを指定します。
記述方法は勤務表のconfigのスコアと同じです

#### update_function
焼きなましの更新に用いる更新関数を指定します。
更新関数は以下の通りです。

- update4: N,O,HをN,O,Hのうちのランダムな要素に入れ替えます。
- update5: 夜勤と公休をランダムに移動します。夜勤の数や公休の数は維持されます。

例
```toml
update_function = "update5"
```

#### temp
焼きなましの最高温度と最高温度を実数で指定します。
序盤に許容するスコアの悪化幅を指定するとよいでしょう。

例
```toml
temp = {max = 25, min = 0}
```

## 改造するには

CLIの出力形式を変更したかったり、GUIを実装したかったりする場合はsrc/io内のコードを

職場に特有の考慮事項があり、既存のスコアで評価不可能な場合、kinmu_libを

アルゴリズムを焼きなまし法から山登り法などに変更したい場合、annealingを

それぞれ編集してください。
