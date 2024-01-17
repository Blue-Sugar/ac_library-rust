# Eratosthenesの篩
整数上の区間$[0, M)$の元に対して、Eratosthenesの篩を行う。これによって、空間計算量を少々用いることで、高速に素因数分解と約数の列挙が可能となる。

### `self.max_value: usize`
整数上の区間の最大値。

### `self.is_prime: Vec<bool>`
整数が素数であるかどうかを管理する配列。

### `self.min_factor: Vec<isize>`
整数の約数で1より大きいものの最小値。整数が素数の場合は`-1`とする。

### `Eratosthenes::build(max_value: usize) -> Eratosthenes`
Eratosthenesの篩を整数上の区間$[0, maxValue)$で行う。計算量$O(N \log \log N)$。

### `self.prime_factorization(m: usize) -> Vec<(usize, usize)>`
以下を満たす配列$(p_i, e_i)_i$を返す。
$$
m = \prod_i {p_i}^{e_i} \\
\forall i, j , i < j \Rightarrow p_i < p_j \\ 
$$
これはつまり、$m$の素因数分解である。
$O(\log m)$

### `self.dividors(m: usize) -> Vec<usize>`
整数$m$の約数全体を昇順に返す。$O(??) < O(m)$。



# finite field
有限体$\Z/p\Z$となる。ここで、$p$は`MOD`として定義される。

### `FiniteField::new(x: usize) -> FiniteField`
$x \in \Z$に対して、$\dot{x} \in \Z/p\Z$を返す。$O(1)$。

### `x.inv() -> FiniteField`
$x \in \Z/p\Z$に対して、$x^{-1} \in \Z/p\Z$を返す。$O(\log(p + x))$。

### `x.pow(n: usize) -> FiniteField`
$x \in \Z/p\Z$に対して、$x^n \in \Z/p\Z$を返す。$O(\log n)$。

### `+, -, *, /, +=, -=, *=, /=`
$\Z/p\Z$上の演算を定義する。
`+, -, *, +=, -=, *=`は$O(1)$。`/, /=`は$O(\log (p + x))$。



# Pascalの三角形
Pascalの三角形を用いて空間計算量を払うことで、高速にcombinationの値を計算する。集合$S$としては、$\N$や`FiniteField`を想定する。このとき、$S$に対して`trait Number`を定義しなくてはならない。

### `self.size: usize`
計算できるcombinationの値の最大値。

### `self.v: Vec<Vec<S>>`
集合$S$上のcombinationの値を管理する配列。

### `PascalTriangle::build(size: usize) -> PascalTriangle`
大きさが`size`のPascalの三角形を構築する。計算量$O(size^2)$。

### `self.combination(n: usize, r: usize) -> S`
$_nC_r$を計算する。$O(1)$。

### `self.duplicated_combination(n: usize, r: usize) -> S`
$_nH_r = _{n + r - 1}C_r$を計算する。$O(1)$。



# slope trick
slope trickとは以下の性質を満たす関数$f: \Z \to \Z$にある操作をするためのデータ構造である。

1. 連続である。
2. 区分線形である。
3. 傾きは整数値である。
4. 傾きは単調増加する。

ここで関数$f: \Z \to \Z$から作られる関数を定義する。
$$
f_+(x) = \max\{ f(x), 0 \} \\
f_-(x) = \max\{ -f(x), 0 \}
$$

このとき以下の等式が成り立つ。
$$
|f| = f_+ + f_-
$$

また、先の性質を満たす任意の関数は次の形に一意に分解できる。
$$
f(x) = \sum_i (x - a_i)_+ + \sum_j (x - b_j)_- + const
$$

可能な操作は

1. 定数関数$a$を加える。
2. $(x-a)_+$を加える。
3. $(x-a)_-$を加える。
4. $|x - a|$を加える。
5. 最小値を計算する。

である。

### `self.min: isize`
最小値を管理する定数。

### `self.l: BinaryHeap<isize>`
関数$f$の最小をとる値よりも小さな区間で、傾きが変化する点。2以上変化するときは複数個含まれているものとする。

### `self.r: BinaryHeap<Reverse<isize>>`
関数$f$の最小をとる値よりも大きな区間で、傾きが変化する点。2以上変化するときは複数個含まれているものとする。

### `SlopeTrick::new() -> SlopeTrick`
関数$f = 0$としてslope trickを初期化する。$O(1)$。

### `self.min() -> isize`
関数$f$の最小値を返す。$O(1)$。

### `self.add_const(a: isize)`
定数関数$a$を加える。$O(1)$。

### `self.add_plus(a: isize)`
関数$(x - a)_+$を加える。$O(|l|)$。

### `self.add_minus(a: isize)`
関数$(x - a)_-$を加える。$O(|r|)$。

### `self.add_absolute(a: isize)`
関数$|x - a|$を加える。$O(|l| + |r|)$。