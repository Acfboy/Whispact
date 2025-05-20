## 检查

我部署了 Github Action，交 pr 后会自动对代码风格和潜在问题进行检查。

### 前端
提交前先跑 ESlint 和 prettier 检查，用于检查潜在问题和代码风格。

终端到项目根目录运行：
- `pnpm prettier --check "src/**/*.{js,ts,jsx,tsx,css,scss,html}"`
- `pnpm eslint src/ --ext .js,.ts,.jsx,.tsx`

可能后面会引入 `jest` 做前端测试框架。

### 后端

- `cargo fmt` 格式化代码。`cargo fmt --all --check --manifest-path ./src-tauri/Cargo.toml`
- `cargo clippy` 潜在问题检查。`cargo clippy  --target aarch64-linux-android --manifest-path ./src-tauri/Cargo.toml`
- `cargo test` 运行测试。

## `TODO.md` / `ROADMAP.md`

`TODO.md` 一级一级写要做的事。交代码记得更新。

`ROADMAP.md` 写总体要实现的功能。

## 文档

进入 `src-tauri` 目录后运行 `cargo doc` 可以看到文档。

接口应该会放在一个单独的 interface 模块里。