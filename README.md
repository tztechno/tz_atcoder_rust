# tz_atcoder_rust

### latest

---
```

```
---
```

```
---
```

```
---
```

```
---
```

```
---
```

```
---
```
N を i32 から usize に変更しました。
A の宣言で mut を使ってミュータブルにし、B を Vec<i32> として宣言し、A.clone() を使って元の配列をコピーしました。
print(i); を println!("{}", i); に修正して、改行を追加しました。
exit() を return; に変更しました。
```
---
```
`let mut input = String::new();`:
   - 新しい空の文字列`input`を作成します。この文字列はユーザーからの入力を格納するために使われます。

`io::stdin().read_line(&mut input).expect("Failed to read input");`:
   - 標準入力から1行を読み込み、その内容を`input`に格納します。入力の読み込みに失敗した場合はエラーメッセージを表示します。

```
---
