# simple_memo

CUIでメモを入力すると時刻・内容などを保存する

## 例
```
$ ./simple_memo <memo>
$ ./simple_memo <memo> <category>
```
メモを追加

```
$ ./simple_memo --list <num>
```
直近num件のメモを表示
  
```
$ ./simple_memo --search <word>
```
wordを含むメモ一覧を表示
  
```
$ ./simple_memo
```
毎回コマンドラインから入力するのではなく、終了するまで標準入力から繰り返しメモを入力する
