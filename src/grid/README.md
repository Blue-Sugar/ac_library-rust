# Grid
$h \times w$の大きさのグリッドを定義する。

### `Grid::build(h: usize, w: usize) -> Grid`
$h \times w$のグリッドを作成する。$O(1)$。

### `self.neighbor4(x: usize, y: usize) -> Vec<(usize, usize)>`
$(x, y)$の上下左右のマスを返す。$O(1)$。

### `self.neighbor8(x: usize, y: usize) -> Vec<(usize, usize)>`
$(x, y)$の上下左右斜めのマスを返す。$O(1)$。