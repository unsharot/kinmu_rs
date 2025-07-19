# main_config
呼び出すファイルを設定します。

## schedule_config_paths
勤務表のconfigのパスを列挙します。
メインconfigが存在するフォルダからの相対パスを書いてください。

```toml
schedule_config_paths = [
   "configW.toml",
   "configK.toml",
]
```

## thread_count
焼きなましに用いるスレッド数を指定します。
結果もここで指定した数だけ表示されます。
CPUの論理プロセッサ数を指定すると高速に動作します。
未記入の場合、値は1と認識され実行されます。

```toml
thread_count = 8
```
