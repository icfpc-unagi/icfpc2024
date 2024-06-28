# ICFP言語 (ICFP language)

ICFP（Interstellar Communication Functional Program）は、スペースで区切られたトークンのリストで構成されています。トークンは、ASCII文字コード33（!）から126（~）までの印字可能なASCII文字を1つ以上含みます。つまり、94種類の文字が使用可能で、トークンはそのような文字の空でない列です。

An *Interstellar Communication Functional Program* (ICFP) consists of a
list of space-separated *tokens*. A *token* consists of one or more
printable ASCII characters, from ASCII code 33 ('`!`') up to and
including code 126 ('`~`'). In other words, there are 94 possible
characters, and a *token* is a nonempty sequence of such characters.

トークンの最初の文字はインジケータと呼ばれ、トークンのタイプを決定します。トークンの残りの部分（空の場合もあります）はボディと呼ばれます。次のサブセクションでは、さまざまなトークンタイプについて説明します。

The first character of a *token* is called the *indicator*, and
determines the type of the *token*. The (possibly empty) remainder of
the *token* is called *body*. The different *token* types are explained
in the next subsections.

## ブール値 (Booleans)

インジケータが T でボディが空の場合は定数 true を、インジケータが F でボディが空の場合は定数 false を表します。

`indicator = T` and an empty *body* represents the constant `true`, and
`indicator = F` and an empty *body* represents the constant `false`.

## 整数 (Integers)

インジケータが I の場合は、ボディが必須です。

`indicator = I`, requires a non-empty *body*.

ボディは94進数として解釈されます。たとえば、数字は感嘆符が0、二重引用符が1などを表す94の印字可能なASCII文字です。例えば、I/6 は数値の 1337 を表します。

The *body* is interpreted as a base-94 number, e.g. the digits are the
94 printable ASCII characters with the exclamation mark representing
`0`, double quotes `1`, etc. For example, `I/6` represent the number
`1337`.

## 文字列 (Strings)

インジケータが S の場合です。

`indicator = S`

バウンド変数の教団は、ASCIIと同様の体系を使用して文字をエンコードしているようですが、順序が少し異なります。具体的には、ボディのASCIIコード33から126は、以下の順序で変換することで人間が読める形式のテキストになります。

The Cult of the Bound variable seems to use a system similar to ASCII to
encode characters, but ordered slightly differently. Specifically, ASCII
codes 33 to 126 from the *body* can be translated to human readable text
by converting them according to the following order:

    abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~<space><newline>

ここで、`<space>` は単一のスペース文字、`<newline>` は単一の改行文字を表します。例えば、`SB%,,/}Q/2,$_` は文字列 "Hello World!" を表します。

Here `<space>` denotes a single space character, and `<newline>` a
single newline character. For example, `SB%,,/}Q/2,$_` represents the
string "Hello World!".

## 単項演算子 (Unary operators)

インジケータが U の場合は、ボディは正確に1文字である必要があり、その後に続くトークンから解析可能なICFPが続く必要があります。

`indicator = U`, requires a *body* of exactly 1 character long, and
should be followed by an ICFP which can be parsed from the tokens
following it.

| 文字 | 意味 | 例 |
|----|----|----|
| `-` | 整数の符号反転 | `U- I$` -> `-3` |
| `!` | ブール値のNOT | `U! T` -> `false` |
| `#` | 文字列を整数に変換：文字列を94進数として解釈 | `U# S4%34` -> `15818151` |
| `$` | 整数を文字列に変換：上記の逆 | `U$ I4%34` -> `test` |

| Character | Meaning | Example |
|----|----|----|
| `-` | Integer negation | `U- I$` -\> `-3` |
| `!` | Boolean not | `U! T` -\> `false` |
| `#` | string-to-int: interpret a string as a base-94 number | `U# S4%34` -\> `15818151` |
| `$` | int-to-string: inverse of the above | `U$ I4%34` -\> `test` |

この表の `->` 記号は「評価される」と読むべきです。詳細は評価のセクションを参照してください。

The `->` symbol in this table should be read as "will evaluate to", see
[Evaluation](#evaluation).

## 二項演算子 (Binary operators)

インジケータが B の場合は、ボディは正確に1文字である必要があり、その後に2つのICFP（それらを x と y と呼ぶことにします）が続く必要があります。

`indicator = B`, requires a *body* of exactly 1 character long, and
should be followed by two ICFPs (let's call them `x` and `y`).

| 文字 | 意味 | 例 |
|----|----|----|
| `+` | 整数の加算 | `B+ I# I$` -> `5` |
| `-` | 整数の減算 | `B- I$ I#` -> `1` |
| `*` | 整数の乗算 | `B* I$ I#` -> `6` |
| `/` | 整数の除算（ゼロ方向に切り捨て） | `B/ U- I( I#` -> `-3` |
| `%` | 整数の剰余 | `B% U- I( I#` -> `-1` |
| `<` | 整数の比較 | `B< I$ I#` -> `false` |
| `>` | 整数の比較 | `B> I$ I#` -> `true` |
| `=` | 等価性の比較（整数、ブール値、文字列で動作） | `B= I$ I#` -> `false` |
| `\|` | ブール値のOR | `B\| T F` -> `true` |
| `&` | ブール値のAND | `B& T F` -> `false` |
| `.` | 文字列の連結 | `B. S4% S34` -> `"test"` |
| `T` | 文字列 y の最初の x 文字を取得 | `BT I$ S4%34` -> `"tes"` |
| `D` | 文字列 y の最初の x 文字を削除 | `BD I$ S4%34` -> `"t"` |
| `$` | 項 x を y に適用（ラムダ抽象を参照） |  |

| Character | Meaning | Example |
|----|----|----|
| `+` | Integer addition | `B+ I# I$` -\> `5` |
| `-` | Integer subtraction | `B- I$ I#` -\> `1` |
| `*` | Integer multiplication | `B* I$ I#` -\> `6` |
| `/` | Integer division (truncated towards zero) | `B/ U- I( I#` -\> `-3` |
| `%` | Integer modulo | `B% U- I( I#` -\> `-1` |
| `<` | Integer comparison | `B< I$ I#` -\> `false` |
| `>` | Integer comparison | `B> I$ I#` -\> `true` |
| `=` | Equality comparison, works for int, bool and string | `B= I$ I#` -\> `false` |
| `\|` | Boolean or | `B\| T F` -\> `true` |
| `&` | Boolean and | `B& T F` -\> `false` |
| `.` | String concatenation | `B. S4% S34` -\> `"test"` |
| `T` | Take first `x` chars of string `y` | `BT I$ S4%34` -\> `"tes"` |
| `D` | Drop first `x` chars of string `y` | `BD I$ S4%34` -\> `"t"` |
| `$` | Apply term `x` to `y` (see [Lambda abstractions](#lambda-abstractions)) |  |

## If

インジケータが ? でボディが空の場合は、その後に3つのICFPが続きます。1つ目はブール値に評価され、それがtrueの場合は2つ目が結果として評価され、そうでない場合は3つ目が評価されます。例えば、以下のようになります。

`indicator = ?` with an empty *body*, followed by three ICFPs: the first
should evaluate to a boolean, if it's true then the second is evaluated
for the result, else the third. For example:

    ? B> I# I$ S9%3 S./

これは no に評価されます。

evaluates to `no`.

## ラムダ抽象 (Lambda abstractions)

インジケータが L の場合はラムダ抽象で、ボディは整数の場合と同じように94進数として解釈され、これが変数番号になります。インジケータが v の場合は変数で、ここでもボディは94進数の変数番号です。

`indicator = L` is a lambda abstraction, where the *body* should be
interpreted as a base-94 number in the same way as
[integers](#integers), which is the variable number. `indicator = v` is
a variable, with again a *body* being the base-94 variable number.

ラムダ抽象が二項適用演算子 `$` の最初の引数として現れる場合、適用の2番目の引数がその変数に割り当てられます。例えば、以下のICFP

When a lambda abstraction appears as the first argument of the binary
application operator `$`, the second argument of the application is
assigned to that variable. For example, the ICFP

    B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK

は、（例えばHaskellスタイルで）以下のようなプログラムを表します。

represents the program (e.g. in Haskell-style)

    ((\v2 -> \v3 -> v2) ("Hello" . " World!")) 42

これは文字列 "Hello World!" に評価されます。

which would evaluate to the string `"Hello World!"`.

## 評価 (Evaluation)

最も一般的なICFPメッセージングソフトウェアであるMacroware Insightは、ICFPメッセージを名前呼び出し戦略を使用して評価します。つまり、二項適用演算子は非正格です。2番目の引数は、（キャプチャを回避する置換を使用して）束縛変数の場所に置換されます。上記の例で v3 のように引数がラムダ抽象の本体で使用されない場合、その引数は評価されません。変数が複数回使用される場合、式は複数回評価されます。

The most prevalent ICFP messaging software, Macroware Insight, evaluates
ICFP messages using a call-by-name strategy. This means that the binary
application operator is non-strict; the second argument is substituted
in the place of the binding variable (using capture-avoiding
substitution). If an argument is not used in the body of the lambda
abstraction, such as `v3` in the above example, it is never evaluated.
When a variable is used several times, the expression is evaluated
multiple times.

例えば、評価は以下のようなステップを踏みます。

For example, evaluation would take the following steps:

    B$ L# B$ L" B+ v" v" B* I$ I# v8
    B$ L" B+ v" v" B* I$ I#
    B+ B* I$ I# B* I$ I#
    B+ I' B* I$ I#
    B+ I' I'
    I-

## 制限 (Limits)

地球との通信は複雑なため、教団はMacroware Insightソフトウェアにいくつかの制限を設けているようです。具体的には、1000万回のβ簡約を超えるとメッセージ処理が中止されます。組み込み演算子は（もちろん B$ を除いて）正格で、β簡約の制限にはカウントされません。したがって、参加者のメッセージはこれらの制限内に収める必要があります。

As communication with Earth is complicated, the Cult seems to have put
some restrictions on their Macroware Insight software. Specifically,
message processing is aborted when exceeding `10_000_000` beta
reductions. Built-in operators are strict (except for `B$`, of course)
and do not count towards the limit of beta reductions. Contestants'
messages therefore must stay within these limits.

例えば、以下の項は16に評価されますが、評価中に109回のβ簡約を使用します。

For example, the following term, which evaluates to `16`, uses `109`
beta reductions during evaluation:

    B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%

研究者は、β簡約の量の制限が参加者が直面する可能性のある唯一の制限であると予想していますが、メモリ使用量と合計実行時間にも（不明な）制限があるようです。

Researchers expect that the limit on the amount beta reductions is the
only limit that contestants may run into, but there seem to also be some
(unknown) limits on memory usage and total runtime.

## 未知の演算子 (Unknown operators)

上記の言語構成要素は、研究者が発見したすべてであり、教団が地球に向けたコミュニケーションで他のものを使用することは決してないと推測されています。しかし、他の言語構成要素が存在するかどうかは不明です。

The above set of language constructs are all that researchers have
discovered, and it is conjectured that the Cult will never use anything
else in their communication towards Earth. However, it is unknown
whether more language constructs exist.
