## 実行方法

### 実行ファイルをダウンロードする場合

ビルドせずに実行ファイル(Windowsの場合exeファイル)をダウンロードする場合、以下の手順に従ってください。

1. 実行ファイル`kinmu.exe`を任意のフォルダ`A`に入れる (`A/kinmu.exe`という状態)
2. ディレクトリ`A/config`を作成
3. メインの設定ファイル`A/config/config.toml`を作成
4. `A/config/config.toml`で指定した次の設定ファイル(schedule_config)に必要事項を記入
5. schedule_configで指定したファイルで指定したannealing_configに必要事項を記入
6. ターミナルを開き、`A`へ移動する
7. `./kinmu.exe`をターミナルで実行

exeファイルを実行する際、デフォルトでメインconfigとして`example/main_config.toml`が読み込まれますが、読み込むconfigを指定することもできます。
以下のように引数に`-l`あるいは`--load`オプションからファイルのパスを指定することで、読み込むファイルを指定できます。
絶対パスと現在のディレクトリからの相対パスの両方に対応しています。
モードを切り替えて使いたい場合にご利用ください。

```sh
# 指定する場合、-lオプションからメインconfigのパスを指定
# config/hoge/config.tomlをメインとする場合
./kinmu.exe -l config/hoge/config.toml
```

```sh
# 指定しない場合、付属のexample/main_config.tomlが実行される
./kinmu.exe
```

そのまま実行すると、ターミナルに下の画像のように出力されます。

![](../../example/simple_case/output_stdout.png)

また、`-o`あるいは`--output`オプションから出力先パスを指定できます。
指定した場合、標準出力の代わりに指定ファイルに結果がテキスト出力されます。
Windowsの場合、出力先の指定にパイプラインを用いると文字化けやカラーコードが残る場合があるため、こちらの機能をご利用ください。

```sh
# 指定する場合、-oオプションからテキストファイルのパスを指定
# output.txtを出力先とする場合
./kinmu.exe -o output.txt
```

`--html`オプションで実行すると、HTMLとして出力できます。`-o`オプションと併用できます。
HTMLの出力では、tableを用いて出力されます。結果を印刷したい場合に活用してください。

```sh
# output.htmlにhtmlを出力
# 出力されたファイルはブラウザで開く
./kinmu.exe --html -o output.html
```

`./kinmu.exe -l .\example\real_case\main_config.toml --html -o .\example\real_case\result.html`
を実行し、生成されたファイルを開くと下の画像のように表示されます。

![](../../example/real_case/output_html.png)

### ソースコードをダウンロードする場合

ソースコードをビルドして実行します。
RustのビルドシステムCargoが必要です。
お使いのOS向けの実行ファイルが配布されていない場合や、ソースコードを改造したい場合、この方法を使ってください。

まず、以下のコマンドでリポジトリをクローンしてください。

```sh
git clone https://github.com/unsharot/kinmu_rs
```

バージョンを指定したい場合、続けて下のようにコマンドを実行して、バージョンを変更してください。
(v2.1.0の場合)

```sh
git checkout v2.1.0
```

#### ビルドしない場合

リリースビルドでの実行の場合、ターミナルで以下のコマンドを実行してください。
`-r`は`--release`のエイリアスで、なしだと実行速度が遅くなります。

```sh
cargo run -r
```

```sh
# 読み込むファイルを指定する場合
cargo run -r -- -l config/hoge/config.toml
```

#### ビルドする場合

リリースビルドでのビルドを以下のコマンドで行ってください。

```sh
cargo build -r
```

`./target/release/kinmu` (Linux)
または
`./target/release/kinmu.exe` (Windows)
が生成されるので、「実行ファイルをダウンロードする場合」同様に実行してください。
