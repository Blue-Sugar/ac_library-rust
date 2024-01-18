# ChangeMinMax
`chmin, chmax`を定義する。

### `self.chmin(x: Self)`
大小関係を確認し、もし
$self > x$
であれば、
$self$
に
$x$
を代入する。

### `self.chmax8x: Self`
大小関係を確認し、もし
$self < x$
であれば、
$self$
に
$x$
を代入する。



# lower_bound
単調性を持つ関数
$f$
に対して、lower_boundを求める。ここで関数が単調性を持つとは、以下の性質が成り立つことである。

$$
x < y, f(y) = true \Rightarrow f(x) = true
$$

### `lower_bound(l: i128, ng: i128, f: Fn(&i128) -> bool) -> i128`
単調性を持つ関数
$f$
に対して、以下の性質を満たす
$r$
を求める。

$$
f(r) = false \\
f(i) = true,(\forall i \in \{ l, \cdots, r - 1 \})
$$

$O(l + ng)$

### `fn lower_bound_in_vec(a: &Vec<S>, l: usize, f: Fn(&S) -> bool) -> usize`
ここでの大小関係はindexによるとする。このとき、以下の性質を満たす
$r$
を求める。

$$
f(a[r]) = false \\
f(a[i]) = true, (\forall i \in \{ l, \cdots, r - 1 \})
$$

$O(l + |a|)$


# Vector Functions
ベクトルに関する便利な関数たち。

### `fn run_length_encoding(a: Vec<T>) -> Vec<(T, usize)>`
`a`をランレングス分解する。
$O(|a|)$

### `fn same_pair(a: Vec<T>) -> usize`
`a`の中で同じ要素からなるペアがいくつあるかを返す。
$O(|a|)$