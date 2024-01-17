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