# Tree
根を指定したtreeを構築する。

### `self.root: usize`
木の根。

### `self.: usize`
木の頂点の数。

### `self.e: Vec<(usize, usize)>`
木の辺の集合。

### `self.adjoint_list: Vec<Vec<usize>>`
木の隣接リスト。

### `self.parent: Vec<isize>`
木のそれぞれの頂点の親を表す。もし、値が`-1`であればその頂点が根であることを表す。

### `self.children: Vec<Vec<usize>>`
木のそれぞれの頂点に直接つながる子の集合。

### `Tree::build(root: usize, n: usize, e: Vec<(usize, usize)>) -> Tree`
木を構築する。$O(n + |e|)$。

### `self.parent(u: usize) -> Option<usize>`
$u$の親を返す。もし、$u$が根であれば`None`を返す。$O(1)$。

### `self.distance_from_root() -> Vec<usize>`
それぞれの頂点の根からの距離を返す。$O(n + |e|)$。

### `self.diameter() -> usize`
木の直径を求める。ここで直径とは、$\max dist(u, v)$のことである。



# Binary Index Tree
自然数の列$A$に対して、以下の操作を簡単にこなすデータ構造。

1. ある元に自然数$x$を加算する。
2. $A$の区間に対して総和を計算する。

### `self.n: usize`
列$A$の元の個数。

### `self.data: Vec<usize>`
列$A$から作られるBinary Index Treeのデータ。

### `BinaryIndexTree::new(n: usize)`
列$A = (0)_{i = 0, \cdots, n - 1}$として`Binary Index Tree`を構築する。$O(1)$。

### `self.add(i: usize, x: usize)`
列$A$に対して、$A_i$に$x$を加える。$O(\log n)$。

### `BinaryIndexTree::build(a: &Vec<usize>) -> BinaryIndexTree`
列$A$に対して、Binary Index Treeを構築する。$O(n \log n)$。

### `self.sum(l: usize, r: usize) -> usize`
$\sum_{i = l}^{r - 1} A_i$を計算する。$O(\log n)$。



# Lowest Common Ancestor
木の共通祖先を高速に求めれるようになるデータ構造である。

### `self.tree: Tree`
対象とする木。

### `self.parent: Vec<Vec<isize>>`
`parent[k][i]`で頂点$i$から$2^k$の距離だけ根に戻ったときの頂点を表す。もし、そのような頂点が存在しなければ`-1`を返す。

### `self.dist: Vec<usize>`
各頂点の根までの距離を表す。

### `LCA::build(tree: Tree) -> Self`
LCAを構築する。$O(n \log n)$。

### `self.lowest_common_ancestor(u: usize, v: usize) -> usize`
$u, v$の最短共通祖先を返す。$O(\log n)$。

### `self.get_dist(u: usize, v: usize) -> usize`
$u, v$の間の距離を返す。$O(\log n)$。

### `self.is_on_path(a: usize, u: usize, v: usize) -> bool`
$u, v$のパス上に$a$が存在するかどうかを返す。$O(\log n)$。




# Segment Tree
segment treeは`Monoid`$S$上の列$A$で、次の操作を高速にするためのtreeである。

1. 1点代入(`a[i]`に`x`を代入)
2. 区間積(`[l, r)`の総積を計算)

### `self.n: usize`
列$A$の元の個数。

### `self.data: Vec<S>`
セグメントツリーの内容。平衡二分木で葉が`n.next_power_of_two()`個。

### `self.size: usize`
`n.next_power_of_two()`の値のメモ。

### `SegmentTree::new(n: usize) -> SegmentTree`
$S$の単位元$e$を$n$個からなる列に対して、`SegmentTree`を構築する。$O(n)$。

### `SegmentTree::build(a: &Vec<S>) -> SegmentTree`
$S$上の列$a$に対して、`SegmentTree`を構築する。$O(|a|)$。

### `self.set_at(i: usize, x: S)`
$a$の$i$番目の元$a_i$の値を$x$に変える。$O(\log n)$。

### `self.prod(l: usize, r: usize) -> S`
$\prod_{i=l}^{r-1} a_i$を計算する。$O(\log n)$。

### `self.all_prod() -> S`
$\prod_{i=0}^{n - 1} a_i$を計算する。$O(1)$。

### `self.get(i: usize) -> S`
$a_i$を返す。$O(1)$。

### `self.max_right(l: usize, f: Fn(&S) -> bool) -> usize`
単調性を持つような関数$f$に対して、次の条件を満たすような$r$を返す。
$$
f\Big(\prod_{i = l}^{r - 1} a_i\Big) = true \\
f\Big( \prod_{i = 1}^{r} a_i \Big) = false
$$
ここで関数$f$が単調性を持つとは、
$$
f(a * b) = true \Rightarrow f(a) = f(b) = true \\
$$
を満たすことである。$O(\log n)$。