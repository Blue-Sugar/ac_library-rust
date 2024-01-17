# ChangeMinMax
`chmin, chmax`を定義する。

### `self.chmin(x: Self)`
大小関係を確認し、もし$self > x$であれば、$self$に$x$を代入する。

### `self.chmax8x: Self`
大小関係を確認し、もし$self < x$であれば、$self$に$x$を代入する。



# run length encoding
ランレングス長に分解する。

### `fn run_length_encoding(a: Vec<T>) -> Vec<(T, usize)>`
`a`をランレングス分解する。$O(|a|)$。