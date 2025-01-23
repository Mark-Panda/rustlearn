# rustlearn

个人学习Rust,不定时删除重写

## vscode扩展包

- Rust下三件套
- Even Better TOML 没有这个Cargo.toml没法高亮

## 已经集成

> 通过动态多态分发实现依赖注入

- [x] axum集成
- [x] sqlx集成
- [x] redis集成
- [x] http请求集成
- [x] env配置
- [ ] kafka集成
- [x] restful与protoc集成
- [x] cron集成
- [ ] nacos集成

### 基本使用

```bash
./build.sh
```

### 指定镜像名称和标签

```bash
./build.sh -n myapp -t v1.0.0
```

### 使用自定义 Dockerfile 和构建参数

```bash
./build.sh -f Dockerfile.prod -b "--build-arg ENV=prod"
```
