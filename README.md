# 🌻 pvz-rust

Plants vs. Zombies 完整复刻版，使用 Rust + Bevy 构建，全部美术资源由 AI 生成。

---

## 截图

> *(开发中，截图持续更新)*

---

## 特性

- 完整核心玩法：种植、阳光、僵尸寻路、子弹碰撞（开发中）
- 多种植物：豌豆射手、向日葵、坚果、寒冰射手、樱桃炸弹等（数据驱动）
- 多种僵尸：普通、旗帜、路障、撑杆跳、铁桶、读报、铁门、橄榄球、舞王、伴舞等
- 防具体系：一类/二类防具独立配置（`armor.ron`）
- 波次关卡系统，数值全部由 RON 配置文件驱动
- 选卡商店，含冷却 CD 与阳光校验（开发中）
- 主菜单与冒险进度存档
- 音效与 BGM（开发中）
- **全部美术资源由 AI 生成**（Midjourney + DALL-E）

---

## 技术栈

| 层级 | 技术 |
|------|------|
| 语言 | Rust 2024 edition |
| 引擎 | [Bevy 0.18](https://bevyengine.org/)（ECS） |
| UI | [bevy_egui 0.39](https://github.com/mvlabat/bevy_egui) |
| 数据 | [RON](https://github.com/ron-rs/ron) 0.12 + serde |
| 随机 | rand 0.10 |
| 美术 | Midjourney v6 / DALL-E 3 |

---

## 运行

**依赖**：Rust **1.85+**（2024 edition 最低要求），无需额外安装游戏引擎或运行时。

```bash
git clone https://github.com/yuese12333/pvz_rust.git
cd pvz_rust
cargo run
```

发布构建：

```bash
cargo build --release
```

Lint 检查：

```bash
cargo clippy -- -D warnings
```

---

## 项目结构

```
src/
├── main.rs          # 入口，注册所有 Plugin
├── states.rs        # GameState 枚举
├── armors/          # 防具数据（armor.ron）
├── grid/            # 网格坐标系
├── plants/          # 植物逻辑
├── zombies/         # 僵尸逻辑（含舞王状态机组件）
├── projectiles/     # 子弹与碰撞
├── sun/             # 阳光系统
├── shop/            # 选卡商店
├── animations/      # 帧动画系统
├── levels/          # 关卡加载、波次与点数出怪
├── ui/              # 主菜单等界面
└── audio/           # 音效

assets/
├── data/
│   ├── plants.ron
│   ├── zombies.ron
│   ├── armor.ron
│   └── levels/      # 关卡波次与覆盖
├── textures/
├── audio/
└── fonts/

mechanics_and_values.md   # 数值与机制文档（与 RON 对齐）
```

---

## 开发进度

- [ ] 网格渲染
- [ ] 点击种植
- [ ] 僵尸移动与舞王状态机
- [ ] 射击与碰撞
- [ ] 阳光系统
- [ ] 死亡判定
- [ ] 帧动画
- [ ] 选卡商店
- [x] 关卡 RON 加载与点数出怪逻辑
- [x] 僵尸/防具数据目录（10 种僵尸 + 5 种防具）
- [x] 主菜单与冒险进关
- [ ] 完整植物种类
- [ ] 胜负界面
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
