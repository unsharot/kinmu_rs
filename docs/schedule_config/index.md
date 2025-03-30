# schedule_config
勤務表に使う基本的な値の設定です。

## staff
スタッフに関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[staff]
```

### attributes
職員ごとのパラメータ名を設定します。
日本語でも可です。

```toml
[staff]
attributes = [
   "Leader",
   "KDayCount",
   "IDayCount",
   "ODayCount",
   "HDayCount",
]
```
```toml
[staff]
attributes = [
   "リーダー",
   "公休数",
   "夜勤数",
   "遅番数",
   "早番数",
]
```

### list
職員の能力、attributesで指定したパラメータ、名前を列挙します。
コメントで項目名を記しておくと便利です。

```toml
list = [
   {name = "職員A", attributes = [0, 8, 3, -1, -1]},
   {name = "職員B", attributes = [1, 9, 6,  0,  0]},
]
```

```toml
list = [
   #        名前                  リーダー 公休   夜勤   遅番  早番  番号
   {name = "職員A", attributes = [0,      8,    3,    -1,   -1]}, #0
   {name = "職員B", attributes = [1,      9,    6,     0,    0]}, #1
]
```

### ng_list
特定の職員同士が夜勤で同じ日の勤務にならないようにするための設定です。
職員リストで上から0,1,2..と番号を振っていき、その番号で指定します。
使用しない場合、空のリストを宣言してください。

```toml
ng_list = [
   {from = 0, to = 1},
   {from = 5, to = 1},
]
```

```toml
# 空のリストを宣言
ng_list = []
```

### count
職員の数を指定します。
職員リストの長さより小さい数が指定された場合、職員リストの上からその数だけカウントされ、余剰分は無視されます。

```toml
count = 12
```

## day
日付に関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[day]
```

### day_count
勤務表の日数を指定します。
30日の月で、バッファー日数を3日に指定している場合、30日+バッファー日数3日 = 33日として指定してください。

```toml
day_count = 33
```

### states
日数で指定した日数分の日ごとの状態を設定します。状態は以下の通りです。

- W: WeekDay 平日
- H: Holiday 休日
- F: Bath フロ
- 2: Bath2 フロ2
- G: Weight 体重測定

```toml
states = "W2WHHWFW2GHHWFW2WHHWFW2WHHWFW2WHH"
```

### buffer_count
バッファーの日数を指定します。
バッファーというのは、先月の終わり３日分など、考慮するべき日数です。
3日分を考慮する際は3を設定してください。

```toml
buffer_count = 3
```

### requested_schedule
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

```toml
requested_schedule = [
"IAKUUUUUUUUUUUUUUUUUIAKKUUUUUUUUUU",
"NIAKUUUUUUUUUUUUUUUUUUUUUUUUYUUUUU",
"IAKUUUUUUUUUUUUUUUUYYUUUUUUUUUUUUU",
"NIAKUUUUUUUUUIAKYUUUUUUUUUKUUUUUUU",
"KNIAKUUUUUUUUUUUUUUUIAKKUUUUUUUUUU",
"KNIAKUUUUUUUUUUUUUUUUUUUUUIAKUUUUU",
"AKKUUUUUUUUUUUUUUKKUUUUUUUUUUUUUUU",
"AKKUUUUUUUUUUIAKUUUUUUUUUUUUUUUUUU",
"KNNUUUUUIAKUUUUUUUUUUUUUUUUUUUUUUU",
"NKKUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU",
"KKNUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU",
"KKNUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU",
"KKNUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU",
]
```

```toml
requested_schedule = [
#HHWBW2MHHWBW2WHHWBW2WHHWBW2WHHWBW2
#8901234567890123456789012345678901
"IAK                 IAKK          ", # 職員A
"NIAK                        Y     ", # 職員B
"IAK                YY             ", # 職員C
"NIAK         IAKY         K       ", # 職員D
"KNIAK               IAKK          ", # 職員E
"KNIAK                     IAK     ", # 職員F
"AKK              KK               ", # 職員G
"AKK          IAK                  ", # 職員H
"KNN     IAK                       ", # 職員I
"NKK                               ", # 職員J
"KKN                               ", # 職員K 土日祝休み
"KKN                               ", # 職員L 土日祝休み 4連勤以上不可
"KKN                               ", # 職員M 土日祝休み
#HHWBW2MHHWBW2WHHWBW2WHHWBW2WHHWBW2
#8901234567890123456789012345678901
]
```

### attributes
日付ごとのパラメータ名と値を設定します。
パラメータの数は任意です。
値はスペースで区切ります。

```toml
attributes = [
   {name = "IStaffCount", values = [0, 0, 0, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2]},
   {name = "OStaffCount", values = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]},
]
```

## fill
fillに関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[fill]
```

### function
焼きなましの前に未定(U)の場所をうめるための関数を指定します。
以下の関数があります。

- no_fill: 何もしません。Uはそのまま焼きなましの段階に渡されます。
- fill1: それぞれの枠をランダムな要素で埋めます。
- fill2: 夜勤の数と公休の数を守り、入りと明けの連続やその後の休みも加味して埋めます。

```toml
function = "fill2"
```

### seed
fillの乱数に用いるシード値を指定します。
型はu64で、最小値は0、最大値はu64の上限で、18446744073709551615です。
未記入の場合、シード値は実行のたびにランダムで選ばれ実行されます。
再現性のあるテストがしたいときは固定し、実際に使う場合は未記入で実行するのが良いでしょう。

```toml
# 多くの場合、シード値の指定は不要です。
# コメントアウトしておくといいでしょう。
# seed = 0
```

```toml
seed = 53
```

## annealing
焼きなましに関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[annealing]
```

### config_paths
焼きなましの設定ファイルのパスを指定します。
焼きなましはここで列挙した順に行われます。
絶対パスと現在のディレクトリからの相対パスの両方に対応しています。

```toml
config_paths = [
   "./config/anconfigW/randomWalk.toml",
   "./config/anconfigW/W1.toml",
   "./config/anconfigW/W2.toml",
]
```

## result
結果に関する設定のグループです。
以下のタグのもと設定してください。
順不同です。

```toml
[result]
```

### score_functions
焼きなまし終了後、結果を表示する際に用いるスコアを列挙します。
いくつかのスコアをまとめて、最終的に一つの値で表示します。
display_nameには表示名、scoresにはスコアの名前と、そのスコアに用いるパラメータを任意の数指定します。
filterは任意で、任意の有理数パラメータlow_passとhigh_passを持ちます。low_passが設定されている場合、まとめたスコアの計算結果が設定したスコアを越えるなら結果表示の際に横に`[warning]`と表示されます。
スコアは以下のとおりです。

| Prop名                       | 引数の型                                    | 説明                                                                                                    |
| :--------------------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------ |
| PatternGeneral               | (Cond, [[Shift]], Score)                    | 指定したシフトパターンが出現した場合のペナルティを指定                                                  |
| PatternFixed                 | (Cond, [Shift], Score)                      | 指定したシフトパターンが出現した場合のペナルティを指定                                                  |
| PatternGeneralAny            | (Cond, [[Shift]], Score)                    | 指定したシフトパターンが出現する職員ごとにペナルティを指定                                              |
| PatternFixedAny              | (Cond, [Shift], Score)                      | 指定したシフトパターンが出現する職員ごとにペナルティを指定                                              |
| Streak                       | (Cond, [Shift], i32, Score)                 | 指定したシフトが指定した回数連続した場合のペナルティを指定                                              |
| ShiftsBalance                | (Cond, Shift, Shift, Score)                 | 指定した2つのシフトのバランスが悪い場合のペナルティを指定                                               |
| ShiftHalfBalance             | (Cond, Shift, Score)                        | 指定したシフトが指定範囲の前半と後半でバランスが取れていない場合のペナルティを指定                      |
| ShiftDirPriority             | (Cond, Shift, Score)                        | 指定したシフトが指定範囲の前後どちらにあるほうが良いか指定 指定スコアが正なら前を優先、負なら後ろを優先 |
| ShiftDistance                | (Cond, Shift, Score)                        | 指定したシフトどうしの距離が離れていることによるペナルティを指定                                        |
| DayCountRegardStaffAttribute | (Cond, Shift, StaffAttributeName, Score)    | 職員ごとの指定したパラメータと指定したシフトの数の差によるペナルティを指定                              |
| StaffCountRegardDayAttribute | (Cond, Shift, DayAttributeName, Score)      | 日付ごとの指定したパラメータと指定したシフトの数の差によるペナルティを指定                              |
| StaffCount                   | (Cond, Shift, i32, Score)                   | 指定した値と指定したシフトの人数の差によるペナルティを指定                                              |
| StaffCountAtLeast            | (Cond, Shift, i32, Score)                   | 指定した値に指定したシフトの人数が達していない場合のペナルティを指定                                    |
| StaffCountWithPremise        | (Cond, Shift, i32, Cond, Shift, i32, Score) | 指定したシフトの人数を満たした日付に対して、指定した値と指定したシフトの人数の差によるペナルティを指定  |
| NGPair                       | (Cond, Shift, Score)                        | NGに指定されたペアが指定したシフトで同じ日になる場合のペナルティを指定                                  |
| NoSamePair                   | (Cond, i32, Shift, Score)                   | 指定したシフトで同じペアが指定回数以上ある場合のペナルティを指定                                        |

型の詳細は以下の通り

| 型名      | 説明                             | 例                                             |
| :-------- | :------------------------------- | :--------------------------------------------- |
| Cond      | スコアを適用する勤務表の枠の条件 | And (DayExceptBuffer (), ParticularDayState B) |
| Shift     | シフト N,K,I,A,O,H,Y,D,U         | N                                              |
| [Shift]   | シフトのリスト                   | [N, O, H]                                      |
| [[Shift]] | シフトのリストのリスト           | [[N], [K, Y]]                                  |
| Score     | スコア 実数                      | -100.3                                         |
| i32       | 整数                             | -3                                             |
| usize     | 非負整数                         | 4                                              |
| DayState  | 曜日 W,H,B,2,M                   | B                                              |

Condの詳細は以下の通り

| 種類               | 引数の型                  | 説明                                                               |
| :----------------- | :------------------------ | :----------------------------------------------------------------- |
| Every              | ()                        | すべての枠を有効とする                                             |
| Or                 | (Cond, Cond)              | 指定した2つのCondのどちらかを満たしていれば有効とする              |
| And                | (Cond, Cond)              | 指定した2つのCondの両方を満たしていれば有効とする                  |
| Not                | Cond                      | 指定した条件を満たしていなければ有効とする                         |
| DayExceptBuffer    | ()                        | バッファーでないなら有効                                           |
| DayInRange         | (usize, usize)            | 指定した範囲の日付でないなら有効 日数はバッファーから0,1,2..と続く |
| ParticularDayState | DayState                  | 指定の曜日なら有効                                                 |
| BeforeDayState     | DayState                  | 指定の曜日の前日なら有効                                           |
| ParticularDay      | usize                     | 指定の日付のみ有効                                                 |
| StaffInRange       | (usize, usize)            | 指定した範囲のスタッフなら有効                                     |
| StaffWithAttribute | (StaffAttributeName, i32) | 指定した職員ごとのパラメータが指定した整数であるスタッフなら有効   |
| ParticularStaff    | usize                     | 指定した番号のスタッフなら有効                                     |

```toml
score_functions = [
   {display_name = "夜勤の禁止パターン", scores = [
      "PatternGeneral (Every (), [[I], [N,O,H,I,K,Y]], 1000)",
      "PatternGeneral (Every (), [[A], [N,O,H,I,A]], 1000)"
   ], filter = {low_pass = 1000}},
   {display_name = "望ましいパターン", scores = [
      "PatternFixed (Every (), [K,I], 100)",
      "PatternFixed (Every (), [Y,I], 100)",
      "PatternGeneral (Every (), [[K,Y],[N,O,H],[I]], 10)",
      "PatternGeneral (Every (), [[N,O,H],[N,O,H],[I]], -300)"
   ]},
]
```
