# string
文字列に関する便利な関数を集めました。

### `fn is_palindrome(s: Vec<T>) -> bool`
`s`がpalindromeになっているかどうかを返す。
$O(|s|)$

### `fn next(s: &Vec<u8>) -> Vec<Vec<usize>>`
`res[char][i]`は文字種`char`のインデックス`i`以降で出てくるインデックスを表す。ここで、`i`以降は`i`を含む。
`i = s.len()`ととれ、このときは確定で`INF`を返す。
$O(|s| \times |set(char)|)$