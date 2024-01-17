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