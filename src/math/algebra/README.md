# Monoid
次の性質を満たす集合$S$とその上の二項演算$op$をMonoidという。
$$
\forall a, b, c \in S, (a*b)*c = a*(b*c) \\
\exist e \in S, \forall a \in S, e * a = a * e = a
$$

`trait Monoid`は単位元を`e()`、二項演算を`op(*, *)`として、集合がモノイドであることを表す。