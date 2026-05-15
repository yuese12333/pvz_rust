# 🌻 pvz-rust

Plants vs. Zombies 完整复刻版，使用 Rust + Bevy 构建，全部美术资源由 AI 生成。

---

## 截图

> *(开发中，截图持续更新)*

---

## 特性

- 完整核心玩法：种植、阳光、僵尸寻路、子弹碰撞
- 多种植物：豌豆射手、向日葵、坚果墙、雪豌豆、樱桃炸弹……
- 多种僵尸：普通、路障头、铁桶头……
- 波次关卡系统，数据全部由 RON 配置文件驱动
- 选卡商店，含冷却 CD 与阳光校验
- 完整 UI：主菜单、关卡选择、HUD、胜负界面、暂停
- 音效与 BGM
- **全部美术资源由 AI 生成**（Midjourney + DALL-E）

---

## 技术栈

| 层级 | 技术 |
|------|------|
| 语言 | Rust 2021 |
| 引擎 | [Bevy 0.14](https://bevyengine.org/)（ECS） |
| UI | [bevy_egui 0.28](https://github.com/mvlabat/bevy_egui) |
| 数据 | [RON](https://github.com/ron-rs/ron) + serde |
| 美术 | Midjourney v6 / DALL-E 3 |

---

## 运行

```bash
git clone https://github.com/yuese12333/rust_pvz.git
cd rust_pvz
cargo run
```

发布构建：

```bash
cargo build --release
```

**依赖**：Rust 1.79+，无需额外安装游戏引擎或运行时。

---

## 项目结构

```
src/
├── main.rs          # 入口，注册所有 Plugin
├── states.rs        # GameState 枚举
├── grid/            # 网格坐标系
├── plants/          # 植物逻辑
├── zombies/         # 僵尸逻辑
├── projectiles/     # 子弹与碰撞
├── sun/             # 阳光系统
├── shop/            # 选卡商店
├── animations/      # 帧动画系统
├── levels/          # 关卡与波次
├── ui/              # 界面
└── audio/           # 音效

assets/
├── data/            # RON 配置（植物/僵尸属性、关卡波次）
├── textures/        # AI 生成图片
├── audio/           # 音效与 BGM
└── fonts/
```

---

## 开发进度

- [ ] 网格渲染
- [ ] 点击种植
- [ ] 僵尸移动
- [ ] 射击与碰撞
- [ ] 阳光系统
- [ ] 死亡判定
- [ ] 帧动画
- [ ] 选卡商店
- [ ] 关卡系统
- [ ] 完整植物/僵尸种类
- [ ] 主菜单与胜负界面
- [ ] 音效与 BGM

---

## 关于 AI 美术

本项目是一次完整的 **vibecoding** 实验——代码由 AI 辅助编写（Cursor + Claude），美术资源全部由 AI 生成。

生图工作流：

1. 用锁定风格词在 Midjourney 生成每种角色的主帧
2. 以主帧为 reference image 生成动画序列，保证帧间一致性
3. idle 动画优先用 Bevy Transform 程序动画实现，减少对帧数量的依赖
4. 必要时用 Aseprite 做像素化后处理，统一视觉风格

---

## License

MIT
