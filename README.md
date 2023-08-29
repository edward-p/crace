# crace

基于 dioxus 构建的一个业余无线电考试刷题工具

### demo

[https://crace.edward-p.xyz](https://crace.edward-p.xyz)

### Build

克隆代码：

```bash
git clone https://github.com/edward-p/crace && cd crace
```

获取考试题库（根据需要修改脚本 `get_resources.sh` 中的内容）:

```bash
./get_resources.sh
```

安装 `dioxus-cli`:

```
cargo install dioxus-cli
```

编译:

```
dx build --release
```
