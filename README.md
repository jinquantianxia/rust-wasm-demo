# 使用 Rust 生成 wasm 模块 demo
## 目录结构说明
```
|-src  # rust代码
|-www  # vite + vue 演示demo
|-www2 # 原生 javascript 演示demo
|-pkg  # 生成的wasm目标文件
```

##  开始
### rust生成wasm

1. www项目中使用，请在项目根目录下执行以下命令：
```
wasm-pack build --release
```

2. www2项目中使用，请在项目根目录下执行以下命令：
```
wasm-pack build --release --target web
```

### 运行www
进入www目录后，执行以下命令：
```
npm run dev
```

### 运行www2
进入www2目录后，执行以下命令：
```
npm i -g http-server
http-server
```
