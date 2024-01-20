# Weighted Directed Graph
重み付き有向グラフを扱う。

### `self.n: usize`
有向グラフの頂点数。

### `self.ew: Vec<(usize, usize, isize)>`
有向グラフの辺。
$(u, v, w) \in ew$
は、頂点
$u$
から頂点
$v$
の辺でその重みが
$w$
であることを表す。

### `WeightedDirectedGraph::build(n: usize, ew: Vec<(usize, usize, isize)>) -> WeightedDirectedGraph`
頂点数
$n$
で、辺が
$ew$
であるような有向グラフを作る。

### `self.adjoint_list() -> Vec<Vec<usize, isize>>`
隣接グラフを作る。
$(v, w) \in adjoint_list[u] \equiv (u, v, w) \in ew$
となる。
$O(|e|)$

### `self.bellman_ford_min(from: usize) -> Vec<isize>`
頂点
$from$
からの各頂点への最短距離を計算する。
辺の重みが負でも正しく動く。
$O(n|e|)$

### `self.bellman_ford_max(from: usize) -> Vec<isize>`
頂点
$from$
からの各頂点への最大距離を計算する。
辺の重みが負でも正しく動く。
$O(n|e|)$

### `self.floyd_warshall() -> Vec<Vec<isize>>`
各頂点から各頂点への最短距離を表す。
$res[u][v] = k \equiv dist(u, v) = k$
という意味になる。
$O(n^3)$