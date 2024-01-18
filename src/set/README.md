# Disjoint Set Union
直和分解可能な集合を定義する。つまり、全体を$S$としたときに$S$の部分集合族$(S_i)_i$で次の性質を満たすようなものを管理する。

$$
\bigsqcup_i S_i = S
$$

### `self.p: Vec<isize>`
`p[i] >= 0`のときは、元`i`が元`p[i]`と同じ部分集合族に属していることを表す。このとき、`i`の親は`p[i]`であるという。
`p[i] < 0`のときは、元`i`の含まれる部分集合族の大きさが`-p[i]`であることを表す。このとき、`i`は大きさ`p[i]`の根であるという。

### `DSU::new(n: usize) -> DSU`
`DSU`を初期化する。つまり、$S = \{ 0, 1, \dots, n - 1 \}$かつ$S_i = \{ i \}, (\forall i = 0, \dots, n - 1)$となるようなDSUを返す。

### `self.root(v: usize) -> usize`
$v$と同じ集合に属する元の中で、根であるものを返す。$O(\log n)$。

### `self.is_root(v: usize) -> bool`
$v$が根であるかどうかを返す。$O(1)$。

### `self.is_same(u: usize, v: usize) -> bool`
$u, v$が同じ集合に属しているかどうかを返す。$O(\log n)$。

### `self.unite(u: usize, v: usize)`
$u, v$に対して、それらが含まれる部分集合の合併を新しい部分集合族の元とする。$O(\log n)$。

### `self.size(v: usize) -> usize`
$v$の属する集合の大きさを返す。$O(\log n)$。



# Potential Disjoint Set Union
DSUの中でも同じ集合のなかにある任意の2つの元の間にpotentialが入ったデータ構造。

### `self.n: usize`
全体の集合$Sの大きさ。

### `self.p: Vec<isize>`
`p[i] >= 0`のときは、元`i`が元`p[i]`と同じ部分集合族に属していることを表す。このとき、`i`の親は`p[i]`であるという。
`p[i] < 0`のときは、元`i`の含まれる部分集合族の大きさが`-p[i]`であることを表す。このとき、`i`は大きさ`p[i]`の根であるという。

### `self.w: Vec<isize>`
`w[i] = d`であるとき、これは`p[i]`を基準とした`i`のポテンシャルが`d`であることを表す。

### `PDSU::new(n: usize) -> PDSU`
`PDSU`を初期化する。つまり、$S = \{ 0, 1, \dots, n - 1 \}$かつ$S_i = \{ i \}, (\forall i = 0, \dots, n - 1)$となるようなPDSUを返す。$O(1)$。

### `self.root(v: usize) -> (usize, isize)`
$v$と同じ集合に属するもので、根とその元を基準とした$v$のポテンシャルを返す。$O(\log n)$。

### `self.is_same(u: usize, v: usize) -> bool`
$u, v$が同じ集合に属するかどうかを返す。$O(\log n)$。

### `self.unite(u: usize, v: usize, d: isize) -> bool`
$u, v$を$v$を基準とした$u$のpotenialが$d$となるように結合する。このとき、ポテンシャルに矛盾が生じるときは結合はせずに`false`を返す。$O(\log n)$。

### `self.size(v: usize) -> usize`
&v&が含まれる集合の大きさを返す。$O(\log n)$。