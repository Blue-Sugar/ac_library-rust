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