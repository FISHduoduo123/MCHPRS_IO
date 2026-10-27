# MCHPRS_IO 项目台账

> **用途**：记录需求、缺陷、决策与版本，是项目状态的唯一真相源。
> **用法**：平时只记录、不动代码；用户说"写代码"才改代码，说"编译"才编译。
> 协作铁律见 §0，当前状态速查见 §2，每个条目的完整上下文见 §3。

---

## 0. 协作约定

### 0.1 委任与铁律（2026-10-25 起，最高优先级）

用户于 2026-10-25 正式委任本会话（DSH）接任项目维护。以下五条**优先级高于本文档其余条款**。

1. **目标先行（最重要）**：动手前必须完全明确用户目标；有不明确之处**立即反问**，禁止猜测或脑补需求。
2. **只记录、不擅自改代码**：功能、改进方案、bug 一律先登记（统一 `BUG-nnn`）；**只有用户明确说"写代码"才进入版本迭代**，其余时间只允许阅读代码。
3. **写代码与编译分离**：说"写代码"才写，说"编译"才编译；两步独立，不得自动衔接。**严禁自行执行任何 cargo 命令**（`build` / `run` / `test` / `check`）或运行编译产物。
4. **全程中文汇报**：正在做什么、做完什么、怎么做的，及时用中文汇报。
5. 第 1 条为最高优先级，确认语义优先于推进速度。

### 0.2 台账规则

1. 每条待办一个编号：**统一使用 `BUG-nnn`**，涵盖功能需求、改进方案与缺陷（用户 2026-10-25 决定）。编号只增，不复用、不重排。
   - **例外（编号回收）**：用户明确要求并入他条目的编号可以**撤销并回收**。`BUG-009` 已因并入 `BUG-001` 被撤销，**下一个新条目继续用 009**（用户 2026-10-25 明确："编号直接删除，以后有 bug 记录到 009"）。
2. 条目统一模板：**状态 / 登记日期 / 影响文件 / 现象 / 根因 / 方案（或实际改动）/ 验证方式 / 备注**。
3. 批量实施后：在 §7 登记版本，并提升对应版本号。
   - 内核版本：`Cargo.toml` 的 `[workspace.package] version`
   - 协议版本：`crates/core/src/automation.rs` 中 HELLO 的硬编码字面量
   - 两者**独立编号**。当前：内核 `0.2.0-beta`、协议 `0.2.2-beta`（详见 §7）
4. 结论必须写清"怎么验证的"，不能只写"应该没问题"。

### 0.3 文件放置规则（2026-10-27 起，长期生效）

**我在 Termux 侧新建的任何文件一律放进 `~/dsh/` 下，不得散落在 `~` 根目录。**

- 临时脚本、补丁、草稿、中间产物 → `~/dsh/scratch/`
- `~` 下不得留下 `.py` / `.rs` / `.md` / `.json` / `.txt` 等散落文件
- 项目自身的产物按项目约定放（源码在仓库内、二进制 `~/mchprs/mchprs`、备份 `~/.ssset/backup/`），本条只约束临时文件
- 同一规则已写入容器侧 `~/.dsh/AGENTS.md`

**附带教训**：写含单引号的源码必须走 Python 脚本（`chr(39)` 构造引号）。直接用 `tx '...'` heredoc 写文件时，外层单引号会吃掉内容里的 `'='`、`':'` 等字符字面量，0.4.2-beta 首次编译失败即由此引起。

---

## 1. 环境

### 1.1 代码位置（两处并存，均由用户管理）

| 环境 | 路径 | 说明 |
| --- | --- | --- |
| Windows | `D:\APP\MC\mchprs\MCHPRS_IO-master` | 原环境，有 git（基线 `main` @ `11ff766`） |
| Android | `~/mchprs/source/MCHPRS_IO`（Termux） | 2026-10-25 起使用；**不是 git 仓库**（无 `.git`）；同级 `~/mchprs/source/MCHPRS` 为上游对照副本，只读参考 |

- Windows 侧调用 git 需全路径：`& 'C:\Program Files\Git\cmd\git.exe' ...`（DSH 进程沿用安装前的环境变量）
- Windows 侧 `core.autocrlf=true` 且 `.gitattributes` 未固定行尾：`checkout` / `stash` / 重置可能把工作区改写成 CRLF，产生巨量假 diff
- Android 侧访问方式：DSH 容器通过 `tx '<命令>'` 进入（SSH 隧道 `127.0.0.1:8022`，用户 `u0_a351`，普通应用权限、无 root）

### 1.2 工具链与测试

Android 侧：cargo / rustc `1.99.0`，依赖缓存在 `~/.cargo/registry`。

冷启动耗时：`cargo check -p mchprs_redpiler` 约 20 秒；`cargo check --workspace --all-targets` 约 1 分 40 秒。

测试入口：

| 命令 | 覆盖 | 用例数 |
| --- | --- | --- |
| `cargo run -p rilc test crates/redpiler/ril_tests` | RIL 图变换 | 9 |
| `cargo test -p mchprs --test components` | 元件级行为 | 18 |
| `cargo test -p mchprs --test timings` | 时序 | 6 |

### 1.3 常见坑

- Termux 的 `/tmp` **不可写**，临时文件放 `$HOME`
- 运行目录是 `MCHPRS_IO/MCHPRS_IO/`（含 `Config.toml`、`test.py`、`world/`、`schems/`），**不是仓库根**
- redpiler 把比较器 override 编成**编译期常量**：必须先装水 / 装满物品，再 `/redpiler compile`；编译后改动不会被重新采样

---

## 2. 状态一览

| 编号 | 标题 | 类型 | 状态 | 版本 |
| --- | --- | --- | --- | --- |
| BUG-001 | 水锅读不到信号；空锅 / 空容器 / 空堆肥桶致崩溃 | 缺陷 | 已实施，待运行验证；**2026-10-25 后续修订（空容器降级为警告）已实施** | 0.3.0-beta |
| BUG-002 | `/gamemode` 等命令无法 Tab 补全 | 缺陷 | 已实施，待运行验证 | 0.2.0-beta |
| BUG-003 | PIN/POUT 位序改为 YZX（破坏性协议变更） | 协议变更 | 已实施，待运行验证 | 0.2.0-beta |
| BUG-004 | 比较器输入未处理时编译崩溃 → 优雅中止 | 缺陷 | 已实施，待运行验证 | 0.2.0-beta |
| BUG-005 | `/whitelist` 失败无反馈 → 删除整个白名单 | 缺陷 | 已实施，待运行验证 | 0.2.1-beta |
| BUG-006 | 编译失败会丢失地皮已排队的 tick | 缺陷 | ~~待实施~~ **已明确不做**（见 §6） | — |
| BUG-007 | `coalesce` 合并条件未对齐上游 | 缺陷 | ✅ **已实施**，✅ release 编译通过，待运行验证 | 0.3.0-beta |
| BUG-008 | 信号强度分析对齐上游（SSSet + pending tick） | 缺陷 | 已实施，单测通过，端到端未验证 | 0.2.2-beta（拟定，待落地） |
| BUG-009 | 服务器没有玩家时电路不运行（上游同源） | 缺陷 | ✅ **已实施**，✅ release 编译通过，待运行验证 | 0.3.0-beta |
| BUG-010 | 只保留 p0,0，其余地皮一律不加载（空岛模型） | 缺陷 | ✅ **已实施**，✅ release 编译通过，待运行验证 | 0.3.0-beta |
| BUG-011 | 编译崩溃时保存存档 → 终端打印 → 进程退出 | 缺陷 | ✅ **已实施**，✅ release 编译通过，待运行验证 | 0.3.0-beta |
| BUG-012 | 无玩家时无法触发编译 → 自动化脚本直接发命令 | 缺陷 | ✅ **已实施**，✅ release 编译通过，待运行验证 | 0.3.0-beta |
| BUG-013 | 自动化端口绑定失败被静默吞掉 | 缺陷 | ✅ **已实施**，✅ release 编译通过，待运行验证 | 0.3.0-beta |
| BUG-014 | 空容器警告改红色，且一次列出全部坐标 | 改进 | ✅ **已实施**，✅ release 编译通过，待运行验证 | 0.4.0-beta |
| BUG-015 | 从文件读取 ROM（Logisim v3.0 hex）驱动红石 IO | 功能 | ✅ **已实施**，✅ release 编译通过，待运行验证（`*.ram.txt` 已拆为 BUG-021） | 0.4.0-beta / 0.4.1-beta（§37） |
| BUG-016 | `PIN`/`POUT` 的 hex 改为整条数据前端补零（整数编码） | 协议变更 | ✅ **已实施**，✅ release 编译通过，待运行验证 | 0.4.0-beta |
| BUG-017 | 存储器的 data 拉杆与 address 元件相邻时禁用该 ROM | 缺陷 | ✅ **已实施**，✅ release 编译通过，待运行验证 | 0.4.1-beta |
| BUG-018 | 暴露点状红石粉的 0–15 信号强度（可读可写，仅限无入边的孤点） | 功能 | 📝 **已登记，待澄清 4 点** | 拟定 0.4.x |
| BUG-019 | 红石粉相关的兼容性问题（具体情况待补充） | 缺陷 | 📝 **已登记，待补充细节** | 待定 |
| BUG-020 | 自动化协议**整体重写**（含全部关键字与 `OK`/`ERR` 格式；端口定义改由 `io.txt` 声明） | 协议变更 | 📝 **仅 hex 序列规则定稿，其余待重定** | 待定 |
| BUG-021 | 可写存储器 `.ram.txt`（含回写格式按 8 字块对齐，原 BUG-028 已并入） | 功能 | ✅ 已实施（含规则 7b 与 0.4.3 回写格式），✅ release 编译通过；⏳ 待运行验证 | 0.4.1-beta / 0.4.2-beta / 0.4.3-beta |
| BUG-022 | 端口坐标支持“步进展开”与 `matrix` 矩阵展开（按 hex 逐行补零对齐） | 功能 | ✅ **已实施**（0.4.2-beta），✅ release 编译通过，待运行验证 | 0.4.2-beta |
| BUG-023 | 端口列表支持 `width` 声明（必填），校验展开后的坐标个数 | 功能 | ✅ **已实施**（0.4.2-beta），✅ release 编译通过，待运行验证 | 0.4.2-beta |
| BUG-024 | 告示牌朝向固定（根因：`as u8` 饱和把 rotation 固定为 15；**上游同源**） | 缺陷 | ✅ **已修复**（0.4.3-beta，收包处统一归一化），✅ release 编译通过，⏳ 待运行验证 | 0.4.3-beta |
| BUG-025 | 报错出口统一：脚本只收 `OK`/`ERR`，详情写终端与玩家聊天框 | 改进 | 📝 **规则已定（4 条），未写代码** | 待定 |
| BUG-026 | 存储器性能与容量风险（落盘阻塞 tick、按位写节点、总线 clone、无容量上限） | 缺陷/性能 | ✅ **0.4.2 已实施其中两项**（只写变化的位 / 去掉总线 clone），✅ 编译通过；余四项不做 | 0.4.2-beta |
| BUG-027 | `/radvance` 与批量 tick 跳过存储器 / IO 采样（RAM 边沿丢失、OUTPUT 中间态丢失） | 缺陷 | ✅ **已实施**（0.4.2-beta），✅ release 编译通过，待运行验证 | 0.4.2-beta |

> "待运行验证" = 已编入代码、通过编译与自动化测试，但**尚未在真实服务器上跑过端到端场景**。
>
> **待实施的 4 条互相耦合、不能拆成四轮做**——BUG-001 后续修订 / BUG-009 / BUG-010 / BUG-011 的依赖关系、建议顺序与分组，见 **§2.1**。

---

### 2.1 待实施条目的耦合关系与实施顺序

当前有 **4 条待实施**的内容，它们**不是四个独立任务**：互相依赖，而且改同一批代码。分四轮各自做会互相覆盖，顺序颠倒还会产生比现在更难用的中间状态。

| 条目 | 内容 |
| --- | --- |
| **BUG-001 后续修订** | 空容器：编译中止 → 黄色警告 + 按 0 继续编译 |
| **BUG-009** | 解除 `update()` 的「无人不 tick」门禁（方向 A） |
| **BUG-010** | 只保留 p0,0，其余地皮一律不加载、永不卸载（空岛模型） |
| **BUG-011** | 编译崩溃 → 保存存档 → 终端打印 → 进程 `exit(1)` |

**耦合之一：改同一段代码**

`crates/core/src/plot/mod.rs` 的 `update()` 一个函数里挤着两个条目的改动点：

```rust
if !self.players.is_empty() {      // ← BUG-009 要拆掉的
    self.last_player_time = now;
    ...全部 tick...
} else {
    if self.last_player_time.elapsed().as_secs() > 600 && !self.always_running {
        self.running = false;      // ← BUG-010 要管的
    }
}
```

- BUG-009 拆 `if`，BUG-010 改 `else` → 同一个函数，分两轮改必冲突
- `remove_oob_players`（:1349）被 BUG-010 改，也在同一文件
- `server.rs` 的按需加载路径（:275）被 BUG-010 删，而 BUG-010 的登录传送又与 BUG-009 的 tick 门禁在同一批文件里

**耦合之二：语义依赖（一个的答案就是另一个的待定项）**

- BUG-009 一直卡着的待定项是「**卸载策略 A1 / A2 / A3**」；BUG-010 的「只有 p0,0 且永不卸载」**正好就是 A1** → 010 不定，009 写不下去
- BUG-011 说「保存**当前**地块存档」，这个单数说法成立，**正是因为 010 之后只有一块地皮**；若仍可有多块地皮，得先定义"当前"是哪块

**耦合之三：状态机依赖（顺序反了会出事）**

BUG-011 的口径是「**只有真 panic 才退服，优雅中止不退服**」。这个口径能立住，前提是 **BUG-001 修订已经做完**——空容器不再算编译失败。

> **反例**：先做 011 再做 001 修订 → 中间这段状态里，一个空木桶 = 编译失败 → 服务器直接 `exit(1)`，比现在难用得多。

**建议的实施顺序**

| 步骤 | 条目 | 理由 |
| --- | --- | --- |
| 1 | **BUG-010** | 先把世界收窄到"只有一块地皮"，后面所有事都变简单（卸载策略、保存范围、"当前地块"的定义全部塌缩） |
| 2 | **BUG-009** | 卸载策略已被 010 钉死，只剩解除门禁 |
| 3 | **BUG-011** | 单地皮 + 编译失败面已缩小，崩溃路径才干净 |
| 4 | **BUG-001 修订** | 越早越好：它是 011 口径成立的前提；文件（`comparator.rs` / `identify_nodes.rs` / `task_monitor.rs`）与前三项基本不重叠 |

**分组建议**

- 第 1–3 步改的是同一批文件（`plot/mod.rs`、`server.rs`）→ **建议一轮写完、一次编译**
- 第 4 步文件基本不重叠，但**语义上与 BUG-011 绑定**，不宜晚于第 3 步

**与已实施条目的关系**

| 条目 | 关系 |
| --- | --- |
| BUG-004（优雅中止） | BUG-001 修订会把"空容器中止"这一支改掉；`Unsupported` 仍走 BUG-004 的中止路径，所以 BUG-004 不失效 |
| BUG-006（tick 丢失） | 已明确不做；BUG-001 修订后最常见的失败源消失，影响面进一步变小 |
| BUG-008（SSSet + pending tick） | 无直接耦合；但 `pending_tick` 是 `NodeState` 的字段，BUG-007 的 `state` 比较会受它影响 |
| BUG-007（`coalesce`） | 与上述四条**无耦合**，可独立做（改 `coalesce.rs` 一个文件） |
| §6「不做全局 panic 兜底」 | 已被 BUG-011 收窄：编译路径的 panic 改为保存 + 打印 + `exit(1)`，其余 panic 点维持原样 |
| BUG-012（无玩家自动编译） | 与 BUG-009 改**同一个函数** `update()`（判定块就在 tick 区域内），建议排在 BUG-009 之后 |
| BUG-013（端口绑定静默失败） | 改 `server.rs` 启动处，**与上述各条无耦合**，可随时独立做 |

---

## 3. 条目详情

### BUG-001　水锅读不到信号 & 空锅 / 空容器 / 空堆肥桶导致崩溃

**状态**：✅ 已实施（0.2.0-beta，待运行验证）——与 BUG-004 一起完成
**登记日期**：2026-09-30
**影响文件**：`crates/redstone/src/comparator.rs`、`crates/redpiler/src/passes/frontend/identify_nodes.rs`

**现象**

- **水锅**（`minecraft:water_cauldron`，1/2/3 格水）接比较器：读出恒为 **0**，应为 1/2/3
- **空锅**（`minecraft:cauldron`）接比较器：`/redpiler compile` 时服务器 **panic**

原版行为（Minecraft Wiki · Cauldron）：空锅 → 0；1/3 满 → 1；2/3 满 → 2；满 → 3；岩浆锅 → 3；细雪锅 → 按层数 1~3。

**根因**（已核实）

`comparator.rs` 里 `has_override`（闸门，决定是否走 override 查询）与 `get_override`（实际取值）两份名单**双向不一致**：

| 方块 | `has_override` | `get_override` | 后果 |
| --- | --- | --- | --- |
| `Block::Cauldron`（空锅） | ✅ 在（:34） | ❌ 不在 | 过闸门却无分支 → `unreachable!` **panic**（:61-71） |
| `Block::WaterCauldron { level }`（水锅） | ❌ **不在** | ✅ 在（:57） | 闸门永不放行 → :57 是**死代码** → 恒 **0** |

关键事实：

1. `has_override` 是**唯一闸门**，5 个调用点都要先过它：`comparator.rs` :79/:96/:101、`identify_nodes.rs` :177、`input_search.rs` :314
2. 全仓搜 `WaterCauldron`，除 `generated.rs` 的定义外只出现在 `comparator.rs:57`，**无其它补救逻辑**
3. 水锅 `is_solid()` = false（`generated.rs:742` 是显式白名单，不含炼药锅），红石模拟路径也拿不到信号
4. **编译器同样中招**：redpiler 对 `has_override` 为真的方块生成 `NodeType::Constant`；水锅不过闸门 → 不生成节点 → 编译出的比较器输入也是 0

**已确认的期望行为**

| 比较器读到的方块 | 行为 |
| --- | --- |
| 水锅（1/2/3） | 正常取值 → `level` |
| 空锅 | ~~交给 BUG-004 报错路径：编译中止 + 聊天窗报坐标~~ → **已修订为：警告 + 按 0 继续编译**（见下方「后续修订」） |
| 空木桶 / 空熔炉 / 空漏斗 | 同上（已修订） |
| 空堆肥桶（`level: 0`） | 同上（已修订） |
| 非空木桶 / 熔炉 / 漏斗 / 堆肥桶（level 1–8） | 正常取值 |
| `Cake`、`EndPortalFrame` | 正常取值（照旧） |

**"空"的判定**

- 空锅：`Block::Cauldron`（单一状态；`WaterCauldron` 的 level 是 1–3，无 level 0）
- 空容器（`Barrel` / `Furnace` / `Hopper`）：满足**任一**即为空
  - `world.get_block_entity(pos)` 返回 `None`（原理图未带 BE）
  - `Some(BlockEntity::Container { comparator_override: 0, .. })`（箱子存在但空）
  - `Some(other)`（BE 类型不对）
- 空堆肥桶：`Block::Composter { level: 0 }`（level 范围 0–8）

**实际改动**

- `has_override` 补入 `WaterCauldron`
- 新增 `OverrideError`；`get_override` 拆为 `try_get_override`（返回 `Result`，供编译）+ `get_override`（`unwrap_or(0)`，供红石模拟，保证正常 tick 不 panic）
- 空锅 / 空容器 / 空堆肥桶 → `Error::EmptyContainer`；未覆盖方块 → `Error::Unsupported`
- 删除前代留下的临时 `eprintln!`

**范围界定**：岩浆锅 / 细雪锅**不处理**。备查：这两个方块在 `mc_data/blocks.json` 有定义，但未登记进 `mc_data/gen_info.yaml` 白名单，生成器以白名单为迭代依据（`block_data_gen/src/main.rs:909-913`）→ 不在 `Block` 枚举中（`generated.rs:206-209`）→ 原理图载入时 `Block::from_name(...).unwrap_or(Block::Air)`（`schematic/src/lib.rs:138`）**静默变空气**，进不到这条路径。

**验证方式（用户执行）**

- 装水 1/2/3 格后 `/redpiler compile`，比较器输出 **1/2/3**
- 堆肥桶 level 1–8、木桶/熔炉/漏斗有物品时按数量取值
- ~~比较器对着空锅 / 空木桶 / 空熔炉 / 空漏斗 / 空堆肥桶 → 编译中止、状态 **Stopped**、**终端无输出**、聊天窗出现英文报错 + **绝对坐标**~~——**已被下方「后续修订」取代**（改为编译通过 + 黄色警告 + 按 0 处理）
- 非 redpiler 的普通红石模拟下，上述"空"情形读到 **0** 且**不崩溃**

**后续修订（2026-10-25 登记，待实施）**：空容器不再中止编译 → 改为警告 + 按 0 继续编译

> 本小节记录了用户 2026-10-25 提出的新要求。它**推翻了**上方期望行为表中「交给 BUG-004 报错路径」的三行，但**不改**"空"的判定规则，也**不改**红石模拟路径。原拟独立编号 `BUG-009`，经用户决定**并入本条**，该编号**作废并回收**。

**用户原话**

> 当前编译时检测到空锅、空桶等直接编译失败，我要改成编译通过，但是在聊天框里警告，内容和当前类似。

**已确认的四个决策**（用户 2026-10-25 逐条选择）

| 议题 | 决定 | 备注 |
| --- | --- | --- |
| 降级范围 | **只降级"空"类**（空锅 / 空木桶 / 空熔炉 / 空漏斗 / 空堆肥桶）→ 警告 + 按 **0** 参与编译 | `OverrideError::Unsupported`（无比较器规则的方块）**仍然中止编译**，BUG-004 的行为不变 |
| 投递对象 | **只发给执行 `/redpiler compile` 的玩家**（`source: Option<usize>`）；自动编译（`source == None`）**静默** | 与现有失败提示的投递对象一致 |
| 条数 | **只报第一处**（沿用 `TaskMonitor` 只记首个错误的策略） | 玩家修好第一处后再编译会看到下一处 |
| 文案与颜色 | **黄色系统消息**，句式**完全沿用现有**，只把 `aborted` 换成 `warning` | 形如 `Redpiler compile warning: comparator at (x, y, z) reads an empty barrel` |

**为什么现在会中止**（完整链路）

1. `comparator.rs`：`try_get_override` 对空容器返回 `Err(OverrideError::EmptyContainer { pos, kind })`
2. `identify_nodes.rs:201`：`NodeState::ss(comparator::try_get_override(block, world, pos)?)` —— `?` 把错误抛给 `identify_block`
3. `identify_nodes` 把错误写进 `TaskMonitor::set_error`
4. `passes/mod.rs` 检测到 `monitor.error().is_some()` 即提前返回，后续 pass 全部跳过
5. `plot/mod.rs` 的 `start_redpiler` 匹配 `Err(CompileError::Override(..))` → 置 `Stopped` + 红色 `send_error_message`

**拟定方案（未实施，仅备查）**

| 文件 | 改动要点 |
| --- | --- |
| `crates/redstone/src/comparator.rs` | 新增返回 `(u8, Option<OverrideError>)` 的取值入口（或让 `EmptyContainer` 走"值 0 + 诊断"双返回）；`Unsupported` 仍走 `Result` |
| `crates/redpiler/src/passes/frontend/identify_nodes.rs` | 空容器不再 `?` 上抛，改为取 0，同时 `monitor.set_warning(..)` |
| `crates/redpiler/src/task_monitor.rs` | 错误槽旁新增**警告槽**（同样"只记首个"）；`error()` 只被 `Unsupported` 使用 |
| `crates/redpiler/src/passes/mod.rs` | 提前返回条件保持只看 `monitor.error()`，警告不影响流程 |
| `crates/core/src/plot/mod.rs` | 成功分支（`Ok(Ok(()))`）在置 `Running` 之后，若 `source` 为 `Some(p)` 且存在警告，调 `send_system_message`（黄色）发出；`Err(CompileError::Override(..))` 分支保留给 `Unsupported` |

**现成接口**：`crates/core/src/player.rs` 已有 `send_error_message`（红）与 `send_system_message`（黄），无需新增消息通道。

**范围边界**

- **不改**红石模拟路径：`get_override`（`unwrap_or(0)`）行为不变，普通 tick 依旧读到 0 且不崩溃
- **不改** `Unsupported` 的中止行为（用户明确只降级"空"类）
- `CompileError::Override` 变体**保留**（仍被 `Unsupported` 使用），不删除
- **不做** BUG-006 的 tick 回填（用户明确不管，见 §6）

**验证方式（待实施后由用户执行）**

1. 比较器对着空木桶 → `/redpiler compile`：**编译成功**、状态 **Running**、聊天窗一条**黄色** `Redpiler compile warning: comparator at (x, y, z) reads an empty barrel`、终端无输出
2. 该比较器在编译结果中**按 0 处理**（下游电路按"无信号"响应）
3. 比较器对着"无比较器规则的方块" → 仍**中止**，仍是红色 `aborted` 文案
4. 空锅 / 空熔炉 / 空漏斗 / 空堆肥桶（level 0）四条路径同第 1 条
5. 自动编译（无触发玩家）遇到空容器 → 编译通过、**无任何聊天消息**
6. 回归：`ril_tests`（9 例）+ `components`（18 例）+ `timings`（6 例）全绿

**版本**：实施后建议内核升到 `0.2.3-beta`（本轮不改协议，协议版本维持 `0.2.2-beta`）。

---

### BUG-002　`/gamemode` 等命令无法用 Tab 键补全

**状态**：✅ 已实施（0.2.0-beta，待运行验证）——采用"甲"方案
**登记日期**：2026-09-30
**影响文件**：`crates/core/src/plot/commands.rs`（命令树）、`crates/network/src/packets/clientbound.rs`（参数类型）

**现象**

`/gamemode`、`/gmc`、`/gmsp` 能正确执行，但 **Tab 补不出来**。

**根因**（已核实）

客户端补全完全依赖服务器下发的 `DeclareCommands` 包（`commands.rs:561` 的 `DECLARE_COMMANDS`），而该命令树里**根本没有 gamemode 节点**：根节点 children（:565-574）为 `[1,4,5,6,8,10,11,13,18,30,34,41,43,44,45,49,51,52]`，全树 53 个节点（0–52）也没有相关名字。

执行是另一条独立链路：`commands.rs:445` `"gmsp"`、:446 `"gmc"`、:447 `"gamemode"`。两条链路无关联，所以"能执行、补不出"。

**顺带发现的同类问题**：`/plot lock`（节点 39）与 `/plot unlock`（节点 40）被声明了却没有父节点引用——根 children 无 39/40，`/plot`（节点 13，:686）的 children 也没有。它们成了**游离节点**，同样补不出来（`commands.rs:148`/`:158` 能执行）。

**方案（甲）：只补补全，不新增模式支持**

- 命令树新增 `/gamemode`（LITERAL）+ 参数节点（`ARGUMENT | EXECUTABLE`，parser = `GameMode`）+ `/gmc`、`/gmsp`（`LITERAL | EXECUTABLE`），并挂进根 children
- `CDeclareCommandsNodeParser`（`clientbound.rs:397`）新增变体，写出 `minecraft:gamemode`
  - 该类型在 1.20.4 存在：`mc_data/registries.json:1296`，protocol_id **40**；客户端自己就能补出 4 种模式，**无需**服务器实现补全请求
- 39 / 40 补进节点 13 的 children

**服务端解析改动量为 0**：`commands.rs:453-461` 现有 `match` 里 `survival` / `adventure` 本就落 `_` 分支 → 红色 `Unknown gamemode`；`Gamemode` 枚举不变（仍只有 `Creative`、`Spectator`）。

**其它细节**

- 数字别名 `1` / `3`：改用 `minecraft:gamemode` 后客户端会**本地拒绝**，`/gamemode 1` 发不到服务端；服务端保留这两个别名（无害）
- 日志：**保留**现有的全局 `info!("… issued command: …")`，不为本情况新增日志

**验证方式（用户执行）**

- `/game` + Tab → 补出 `gamemode`；`/gamemode ` + Tab → **4 种**模式
- 选 survival / adventure → 聊天窗红色 `Unknown gamemode`，无新增日志
- `/gm` + Tab → `gmc`、`gmsp`；`/plot lo` → `lock`；`/plot un` → `unlock`

---

### BUG-003　PIN/POUT 位序改为 YZX（破坏性协议变更）

**状态**：✅ 已实施（0.2.0-beta，待运行验证）
**登记日期**：2026-09-30
**类型**：协议行为变更（**破坏性**：依赖旧位序的外部脚本 / 点阵会错位）
**影响文件**：`crates/core/src/automation.rs`、`src/protocol.txt`、`src/protocol_zh.txt`、`README.md`、`README_zh.md`、`MCHPRS_IO/test.py`

**旧行为（已核实）**

点 `(i, j, k)` 位于 `(x0 + i*dx, y0 + j*dy, z0 + k*dz)`，位序号 = `i*(ny*nz) + j*nz + k`，位 0 为最高有效位；生成顺序 `for i { for j { for k } }` —— **Z 变化最快**，即 ZYX。

**新行为（已与用户确认）**

变化快慢改为 **Y → Z → X**：

- 位序号 = `j + k*ny + i*ny*nz`
- 生成循环 `for i { for k { for j } }`
- `dy` 规则：`ny == 1 → dy == 0`；`ny > 1 → dy < 0`（严格为负，避免多位落在同一坐标）
- `dx`、`dz` 维持增大方向

**等价性（重要）**：只有 `ny > 1` **且** `nz > 1` 时新旧才不同；任一轴为 1 时等价。所以实际影响是"3D 点阵的位序 + Y 方向符号约定"。

**实际改动**

- `automation.rs`：位序号公式、循环次序、`dy` 校验、:48-50 注释
- `src/protocol.txt` / `protocol_zh.txt`：公式、位推进顺序、偏移正负规则、示例会话（`PIN` 示例 `dy=1` → 原点 y=67 / `dy=-1`）
- `MCHPRS_IO/test.py`：`ADD_IN` 原点 y 11→25、`dy` 2→‑2；`ADD_OUT` 原点 y 11→27、`dy` 2→‑2；`pack_add_in` / `parse_add_out` 按新位序重算（**保持与旧版相同的物理↔逻辑对应**，故原原理图无需重建），移除失效的 `_REV8` / `_REV12` 查表

**验证方式（用户执行）**

- 更新后的 `test.py` 能注册成功（`OK PIN` / `OK POUT`），500 组随机向量全过
- 另造 `ny > 1 且 nz > 1` 的点阵，核对位序号符合 `j + k*ny + i*ny*nz`
- 确认 `dy > 0` 被拒绝并返回 `ERR RANGE`

---

### BUG-004　编译遇到未处理的比较器输入时崩溃 → 优雅中止

**状态**：✅ 已实施（0.2.0-beta，待运行验证）——接收 BUG-001 投递的错误
**登记日期**：2026-09-30
**影响文件**：`crates/redstone/src/comparator.rs`、`crates/redpiler/src/passes/frontend/identify_nodes.rs`、`passes/mod.rs`、`crates/redpiler/src/lib.rs`、`task_monitor.rs`、`crates/core/src/plot/mod.rs`

**现象**

`/redpiler compile` 时若比较器读到取不到合法值的方块，整个 plot **直接崩溃**，终端还打印坐标。

**根因**（已核实）

1. `get_override` 兜底分支是 `unreachable!("Block does not override comparators")`（`comparator.rs:61-71`）
2. 编译时由 `identify_block` 调用它（`identify_nodes.rs:177-180`）
3. 编译跑在 `thread::scope` 起的**子线程**里（`plot/mod.rs:940-944`），子线程 panic 会在作用域结束时**重抛给父线程** → 整个 plot 线程崩溃
4. 因此走不到 `set_redpiler_state(Running)`

**期望行为（用户要求）**

1. 不崩溃
2. 停止编译，状态回到 `RedpilerState::Stopped`
3. 终端**不打印任何错误**
4. 在该玩家聊天窗报错，附出错方块的**绝对坐标**

**实际改动**

- `comparator.rs`：`unreachable!` 改为可失败返回（见 BUG-001）
- `task_monitor.rs`：新增错误槽 `set_error` / `error`（只记首个错误）
- `lib.rs`：新增 `CompileError { Override, Cancelled }`；`Compiler::compile` 返回 `Result`；`CompilerInput` 增加 `monitor` 字段
- `identify_nodes.rs`：`identify_block` 返回 `Result`，错误写入 monitor，随后跳过注释遍
- `passes/mod.rs`：`run_passes` 检测到错误即提前返回
- `plot/mod.rs`：`start_redpiler(options, source)` 按结果显示 Running / **Stopped**，失败时向触发者发红色英文消息（含绝对坐标）；**顺带修掉"无条件设 Running"**（连取消也会被设成 Running 的既有问题）
- 错误类型携带种类，支持两种文案：
  - `Redpiler compile aborted: comparator at (x, y, z) reads an empty barrel`
  - `Redpiler compile aborted: comparator at (x, y, z) reads a block with no comparator override`
- 自动编译失败（无触发玩家，`mod.rs:1457`）：**仅停止**，不报错、不打印

**范围边界（重要）**：本条是**点状修复**，只处理"比较器取不到合法值"这一条路径；**不做**全局 panic 兜底（见 §6）。

**验证方式（用户执行）**

- 构造比较器读到未覆盖方块的场景 → `/redpiler compile`：不崩溃、状态 **Stopped**、终端**无输出**、聊天窗出现含绝对坐标的红色报错
- 编译成功时状态仍为 Running

---

### BUG-005　`/whitelist` 失败时玩家毫无反馈 → 删除整个白名单

**状态**：✅ 已实施（0.2.1-beta，待运行验证）——按"删除整个白名单功能"落地
**登记日期**：2026-09-30
**影响文件**：见下方牵连清单

**现象**

`/whitelist add|remove <用户名>` 在两种情况下都"什么都没发生"：

1. 用户名解析失败（不存在 / Mojang 非 200 / 网络不通）
2. 任务内 panic

**根因**

- 错误分支只写 debug 日志、不回消息：`commands.rs:252-254` 与 `:267-269`
- `src/main.rs:25-32` 的 `EnvFilter` 默认级别是 **INFO**，`debug!` **根本不输出**
- 于是"崩溃"与"查不到名字"从玩家侧、默认日志侧看完全一样

**决定**：彻底删除白名单功能（而不是补提示）——从根上消除本条，并顺带消掉整条 HTTPS / Mojang 依赖（`reqwest` 全仓仅此一处使用）。

**牵连清单（已核实）**

| 位置 | 内容 |
| --- | --- |
| `crates/core/src/profile.rs` | 整个文件（唯一 reqwest 使用者） |
| `commands.rs:238-278` | `/whitelist` 命令分支 |
| `commands.rs:836-871` | 命令树节点 30–33 及根 children 中的 `30` |
| `server.rs:59-62,143-144,156,190-197,212,275-276,521-531,642-677` | 消息枚举、`WhitelistEntry`、`whitelist` 字段、`whitelist.json` 读写、登录校验、两个处理分支 |
| `config.rs:72` | `whitelist: bool` 字段 |
| `Cargo.toml` / `crates/core/Cargo.toml` | `reqwest` + `rustls` |
| `MCHPRS_IO/Config.toml`、两份 README | `whitelist` 行 |

**实施记录**

- 按**甲方案**落地（零重编号）：**只从根节点 children 删掉 `30`**，节点 30–33 保留为不可达游离节点
- ⚠️ 遗留技术债：与 BUG-002 修的"游离节点"同类；将来可做乙方案（真删 + 整体重编号）清掉。本轮为规避重编号风险而保留（写错不会报编译错，只会让客户端拿到错误 / 越界命令树）
- 随删除清理的 import：`server.rs` 的 `HyphenatedUUID`、`serde::{Deserialize, Serialize}`、`std::path::Path`，`std::fs::{self, File}` → `std::fs`；`commands.rs` 的 `PlayerPacketSender`。保留 `serde_json::json`、`CDisconnectLogin`、`fs`
- 协议版本 → `0.2.1-beta`（HELLO 串 + 两份协议文档 + 两个 README）；内核保持 `0.2.0-beta`

**配置兼容性**：`config.rs` 的 `gen_config!` 只**补**缺失键（`or_insert_with`），从不删键；serde 反序列化默认忽略未知字段。所以残留的 `whitelist = false` 无害。

**⚠️ 行为后果**：服务器**不再有任何访问控制**，任何人都能连进来。

**验证方式（用户执行）**

- Tab 补不出 `/whitelist`，敲 `/whitelist` 回 "Command not found!"
- 依赖里不再有 `reqwest` / `rustls`（`Cargo.lock` 相应瘦身）
- `Config.toml` 里残留 `whitelist = false` 不影响启动；任意玩家能直接进服

---

### BUG-006　编译失败会丢失地皮已排队的 tick

**状态**：❌ **已明确不做**（用户 2026-10-25 决定："如果编译崩溃，tick 丢失不用管"）。以下内容仅作历史记录保留，不再排期。
**登记日期**：2026-10-25
**影响文件**：`crates/core/src/plot/mod.rs`（`start_redpiler`，drain 处约 :938；`Err` 分支约 :965-995）
**性质**：BUG-004 引入"优雅中止"之后才变得可达；上游无此问题（失败即 panic）

**现象**

编译失败（比较器读到空容器 / 无 override 方块，或未来 monitor 取消）后：

- 该地皮 `to_be_ticked` 中被 drain 走的**所有排队红石事件永久消失**
- redpiler 状态置 `Stopped`，电路停在半拍
- 玩家只看到"编译失败"的聊天提示，不知道排队事件已经丢了

**根因**

```rust
let ticks = self.world.to_be_ticked.drain(..).collect();  // 清空
...
self.redpiler.compile(&self.world, bounds, options, ticks, monitor)
```

`ticks` 的所有权随后交给 `compile()`；失败分支只置状态、发消息，**没有回填**。

**方案**

| 方案 | 做法 | 代价 | 评价 |
| --- | --- | --- | --- |
| A1 | 编译前 `ticks.clone()` 存一份，`Err` 时 `self.world.to_be_ticked = backup` | 约 10 行 | **推荐** |
| A2 | 只把 clone 交给后端，成功后才 drain | 需改 `compile()` 入参语义 | 牵连面大 |
| A3 | 不改 | 0 | 保留该坑 |

A1 的安全性评估：编译期间 plot 线程被 `thread::scope` 占住，世界不会产生新 tick；`drain` 保持队列原有顺序，直接赋回不会破坏排序。

**验证方式**

- 需先有能覆盖 plot 层的测试手段（当前无）
- 手工：让比较器读空容器触发编译失败，且当时某中继器处于排队状态 → 失败后再次触发红石，确认该中继器仍按原延迟响应、未被吞掉
- 自动化：`MCHPRS_IO/test.py` 反复编译时观察是否出现"编译失败后电路不响应"

**备注**：`TickEntry` 的 clone 成本可忽略（队列通常几十条）。

---

### BUG-007　`coalesce` 合并条件未对齐上游

**状态**：✅ **已实施**（0.3.0-beta，✅ release 编译通过，待运行验证）——2026-10-25 随大更新一并落地
**登记日期**：2026-10-25
**影响文件**：`crates/redpiler/src/passes/opt/coalesce.rs`（约 :79-84 的合并判断）
**性质**：既有缺陷（非 BUG-008 引入），但 BUG-008 让 `NodeState` 多了 `pending_tick` 维度，风险放大

**现象**

本分支的合并判断只比较 `node.ty` 与"入边只有 1 条"；上游还要求：

```rust
&& dest.state == into.state
&& graph[edge_idx].ty == LinkType::Default
```

**根因与后果**

1. **缺 `state` 比较**：类型相同、输入相同但**当前状态不同**的节点（通电与否、锁定与否、是否排队）会被合并，被合并方的状态丢失，之后按共享节点更新 → 下游可能看到错误电平。表现为"电路偶尔卡一下 / 首个 tick 电平不对"这类难查的问题。
2. **缺 `LinkType::Default` 检查**：侧输入可能被当作默认输入参与合并判断，锁存器结构（中继器的侧输入来自另一个中继器）可能被错误合并。

这类状态不一致通常只存在一个 tick 内、很快被修正，所以此前不易暴露；`pending_tick` 让窗口变长。

**方案**（已确认：采用"对齐上游"，不做自定义改法）

已核实：本分支与上游 `coalesce.rs` 逐字相同，**只差这两行**（另有两行本分支独有的注释，不影响行为）；`NodeState` 已 `derive(Debug, Clone, Default, PartialEq, Eq)`（`compile_graph/mod.rs:53`），`LinkType` 为 `{Default, Side}` 且 `Copy + PartialEq`，前提条件全部满足。

补齐两个条件即可，约 2 行：

```rust
if dest.ty == into.ty
    && dest.state == into.state
    && dest.is_removable()
    && graph[edge_idx].ty == LinkType::Default
    && graph.neighbors(dest_idx, Direction::Incoming).count() == 1
```

**代价与风险**

- 少合并 → 编译产物略大、节点更多，可能轻微影响 timings
- `state` 比较里包含 `pending_tick`（BUG-008 新增字段）：排队中的中继器不会再被合并进普通中继器，与上游语义一致
- 影响**所有**电路，必须跑全量回归：`components`（18 例）+ `timings`（6 例）+ `ril_tests`（9 例）

**验证方式**（用户 2026-10-25 决定：**不新增回归测试**，仅靠现有测试兜底）

- 现有三套测试全绿：`ril_tests`（9 例）+ `components`（18 例）+ `timings`（6 例）
- 新增测试**不做**（用户明确"不用"）
- 备查：`.ril` 语法其实**可以**表达 repeater 的 `powered` / `locked`（`ril.rs:859` → `NodeState::repeater(powered, locked)`），将来若想补 `state` 条件的用例是可行的；`LinkType::Side` 与 `pending_tick` 则无法用 `.ril` 表达，需 Rust 单测构造图

---

### BUG-008　信号强度分析对齐上游（SSRangeAnalysis → SSSetAnalysis）

**状态**：✅ 已实施（2026-10-25）：编译通过 + 单测全绿；**端到端未验证**；版本 `0.2.2-beta` 尚未落地
**登记日期**：2026-10-25
**影响文件**：`passes/analysis/{ss_set_analysis.rs(新), ss_range_analysis.rs(删)}`、`passes/analysis/mod.rs`、`passes/mod.rs`、`passes/opt/{unreachable_output,constant_fold,prune_orphans}.rs`、`passes/frontend/identify_nodes.rs`、`compile_graph/mod.rs`、`lib.rs`、`crates/rilc/src/{compile,test}.rs`、`ril_tests/opt/unreachable_output.ril(新)`

**背景**

本分支基点是较早的上游提交，信号强度分析仍是旧的 `SSRangeAnalysis`（`{low, high}` 区间近似）；上游已重写为 `SSSetAnalysis`（`u16` 位集 + 单调不动点，能精确表达 `{0,15}` 这类非连续集合），并新增 **pending tick** 建模。

**两处差异**

1. **精度**：区间比位集粗，`unreachable_output` 少剪边；旧实现还留着一个 `unreachable!("evaluate node ty")` panic 点（原 `ss_range_analysis.rs:313`）
2. **待决 tick**：旧的编译图**完全不建模排队中的红石事件**（`NodeState` 无 `pending_tick`，`CompilerInput` 的 `ticks` 被摘除）。后果是"当前输出 0、下一 tick 会打脉冲"的中继器被判 `max = 0`，而 `unreachable_output` 的规则是 `edge.ss >= max` 即删边 → 它的**出边会被全部剪掉**。上游已修，本分支此前同病

**实际改动（用户选定"完全对齐上游"）**

| 方面 | 改动 |
| --- | --- |
| 分析模型 | 换成上游 `SSSetAnalysis`（221 行） |
| 消费者 | `unreachable_output` 改取 `get_set(idx).max()` |
| pending tick | `NodeState` 加 `pending_tick`；`CompilerInput` 加回 `ticks: &[TickEntry]`；`identify_nodes` 消费 `input.ticks` 打标；分析对排队中继器按 `{0,15}` 起手；`constant_fold` 不折叠、`prune_orphans` 保根、`is_removable()` 不删 |
| 调用点 | `rilc` 两处 `CompilerInput` 构造补 `ticks: &[]` |
| 测试 | 补上原本缺失的 `unreachable_output.ril`（上游 3 例） |

**验证结果（2026-10-25，Android）**

| 检查 | 结果 |
| --- | --- |
| `cargo check --workspace --all-targets` | ✅（仅 2 个原有 dead_code 警告） |
| `cargo run -p rilc test crates/redpiler/ril_tests` | ✅ 9/9 |
| `cargo test -p mchprs --test components` | ✅ 18/18 |
| `cargo test -p mchprs --test timings` | ✅ 6/6 |
| 端到端 demo（release + `test.py`） | ❌ 未做（用户指示暂不验证） |

**回滚**：改动前原文件在 `~/.ssset/backup/`（按仓库内相对路径存放）

**未完成项**

1. `constant_fold` 的 pending tick 守卫**无测试覆盖**（`.ril` 语法无法表达 pending tick；上游同样没有测试）。补测需扩语法或写 Rust 单测，属"写代码"范畴
2. `coalesce` 未对齐上游（已立项为 **BUG-007**）

---

### BUG-009　服务器没有玩家时电路完全不运行（上游同源）

**状态**：✅ **已实施**（0.3.0-beta，✅ release 编译通过，待运行验证）——方向 A + A1，见「实际改动」
**登记日期**：2026-10-25
**来源**：用户观察——"我发现上游也有同样的问题，服务器没有玩家时，电路不运行"
**性质**：上游原生行为，**不是本 fork 引入**；但对 MCHPRS_IO 的自动化定位是致命缺陷

**现象**

没有任何 Minecraft 玩家站在某地皮上时，该地皮：

- 红石完全不 tick（电路静止）
- 自动化协议的 `PIN` / `INPUT` 送进去没有任何效果
- `POUT` / `SUB` 不再采样，`EVENT` 不推送
- `auto_redpiler` 不会触发自动编译
- 只有等一个真实玩家客户端进入该地皮，才恢复运行

**根因（三层，全部已核实）**

**第一层：tick 门禁（主因）** —— `crates/core/src/plot/mod.rs:1404-1406`

```rust
fn update(&mut self) {
    self.handle_messages();

    // Only tick if there are players in the plot
    if !self.players.is_empty() {
        self.timings.set_ticking(true);
        ...
```

`self.players` 是**真实 MC 玩家**的 `Vec`；自动化 TCP 连接不属于玩家（只是 `AutomationServer` 里的一条 socket）。整个 tick 主体都在这道门禁内，因此无人时以下全部跳过：

| 被跳过的内容 | 位置 |
| --- | --- |
| `tickn(batch_size)` / `tick()` —— 红石模拟 | :1447 / :1471 |
| `redpiler.flush(&mut self.world)` —— 编译器后端 | :1448 |
| `sample_sub_watches()` / `sample_pout_ports()` —— `SUB` / `POUT` 采样 | :1434-1435 / :1454 |
| `take_io_events()` + 推送 —— `EVENT` 推送 | :1450-1453 |
| `auto_redpiler` 自动编译判定 | :1475 |
| `world.flush_block_changes()` | :1487 |
| `timings.set_ticking(true)` / `last_player_time` 刷新 | :1407 / :1412 |

**第二层：地皮生命周期**

- `server.rs:194`：出生点 `(0,0)` 启动时硬编码加载，`always_running = true`
- `server.rs:275`：**其它地皮只在玩家进入时才加载**，`always_running = false`
- `plot/mod.rs:1506`：`if self.last_player_time.elapsed().as_secs() > 600 && !self.always_running { self.running = false }` —— 无人 600 秒后**整个地皮卸载**
- 关键：`always_running` **只防卸载，不解除 tick 门禁**。即使是出生点，没人时也照样静止

**第三层：自动化请求被拒绝** —— `server.rs:323-325`

```rust
} else if let Some(automation) = &mut self.automation {
    // The plot is not loaded, so no circuit can be running there.
    automation.handle_reply(conn_id, AutomationResponse::Inactive(pos));
}
```

地皮未加载时，自动化请求直接返回 `Inactive(pos)`。

**为什么是上游原生行为（证据）**

- `diff` 本 fork 与上游 `plot/mod.rs` 的 `update()`：门禁 `if !self.players.is_empty()` 与 `// Only tick if there are players in the plot` 注释**逐字相同**
- 本 fork 新增的自动化采样逻辑（`automation_per_tick_push`、`sample_pout_ports`、`take_io_events`）**全部写在门禁内部** → 继承了同一限制
- 上游 README 中**没有任何**关于"无人不 tick"的说明
- `Config.toml` 的全部 13 个配置项（`config.rs:71-90`）**没有任何**能强制持续 tick 的开关

**已确认的决策**

| 议题 | 决定 | 时间 |
| --- | --- | --- |
| 修复方向 | **A：直接解除 tick 门禁，无条件 tick** | 2026-10-25 |
| 是否可直接改上游代码 | **可以**，直接改 `update()` 的门禁，不做旁路/配置绕过 | 2026-10-25 |
| 卸载策略 | **待定** —— 用户 2026-10-25 表示"我有另一个需求能完美跳过这个问题"，本条暂停等新需求 | 2026-10-25 |

**待定项说明（为什么卸载策略必须一起定）**（**已闭合**：用户 2026-10-25 提出 BUG-010「只保留 p0,0 且永远加载」，等价于选定 **A1**）

现状是**同一个 `if` 同时管 tick 与卸载**：

```rust
if !self.players.is_empty() {
    self.last_player_time = now;      // :1412 只在有玩家时刷新
    ... 全部 tick ...
} else {
    if self.last_player_time.elapsed().as_secs() > 600 && !self.always_running {
        self.running = false;         // :1506 卸载地皮
    }
}
```

因此只把 tick 挪出 `if` 会踩三个坑：

| 坑 | 后果 |
| --- | --- |
| `last_player_time` 跟着挪出去 | 永远等于"刚才" → 卸载逻辑永远不触发，等于偷偷关掉卸载 |
| 卸载判定留在 `else` | 无人地皮走不到 `else`（tick 已无条件执行）→ 同样等于关掉卸载 |
| 两者都照旧 | 地皮 10 分钟后照样卸载 → 自动化脚本跑 10 分钟后拿到 `Inactive`，问题只是推迟 |

候选的子方案（**尚未选定**）：

| 编号 | 做法 | 代价 |
| --- | --- | --- |
| A1 | 彻底取消无人卸载，加载过就常驻 tick | 看过的地皮全常驻，内存/CPU 随时间累积，重启才清 |
| A2 | 保留卸载，但"活动"= 有玩家 ∥ 有自动化订阅/端口 ∥ 红石图已编译激活 | 规则更复杂，但兼顾不浪费与不断服 |
| A3 | 卸载照旧只按玩家计时 | 与"无人跑电路"目标直接矛盾，不可取 |

**验证方式（待实施后由用户执行）**

1. 不启动任何 MC 客户端，直接用自动化脚本 `PIN` 一个输入 → `POUT` 能读到对应输出变化（证明无人也在 tick）
2. 同一场景持续运行 **超过 10 分钟** 后再 `PIN`，仍能正常响应（证明地皮没有在 600 秒时被卸载，具体取决于最终选定的卸载子方案）
3. 有玩家时行为与现在一致，`components`（18 例）+ `timings`（6 例）+ `ril_tests`（9 例）全绿
4. CPU 观察：无人时 `Tps::Unlimited` 地皮不应把单核占满（解除门禁后需确认休眠路径仍生效）

**实际改动（0.3.0-beta）**

| 文件 | 改动 |
| --- | --- |
| `crates/core/src/plot/mod.rs` | `update()` 里 `if !self.players.is_empty() {` 改为**裸块** `{`（无条件执行 tick 主体）；`last_player_time` 改为**仅在有玩家时刷新**；原 `} else { 卸载 }` 拆成块后的独立 `if self.players.is_empty() && !self.always_running { ... }` |
| `crates/core/src/plot/mod.rs` | 删除 `self.timings.set_ticking(false)` 调用（现在永远 tick） |

**未做**：没有改 `run()` 里的快速路径条件（`sleep_time <= 5ms && !players.is_empty()`），Unlimited 且无人时仍是 `yield_now()` 死转 —— 用户已明确接受"单核跑满"。


---

### BUG-010　只保留 p0,0，其余地皮一律不加载（空岛模型）

**状态**：✅ **已实施**（0.3.0-beta，✅ release 编译通过，待运行验证）——见文末「实际改动」
**登记日期**：2026-10-25
**来源**：用户 2026-10-25 提出——"我需要只保留 p0,0 地块，其他地块一律不许加载，玩家不许进入，如果玩家进入到 p0,0 外面，还是加载 p0,0，也就是说，p0,0 永远加载"
**性质**：架构性收窄（从「任意地皮」变为「单一地皮服务器」）
**与 BUG-009 的关系**：本条闭合了 BUG-009 的卸载策略待定项（唯一地皮 + 永不卸载 = A1），且让 BUG-009 的验证场景简化为只有一块地皮

**用户的目标（已澄清）**

- 整台服务器**只有 `p0,0` 一块地皮**，其它地皮**永不加载**
- p0,0 **永远加载、永远 tick**（不因无人而卸载）
- 玩家走到 p0,0 外面时：**不拉回、不阻挡、不加载新地皮**，就是**空岛模型**——岛外面是空无一物，玩家可以飘出去，回头仍能看见这座岛
- **不是**"掉进虚空摔死"那种：本仓**完全没有**生命值 / 坠落伤害 / 虚空伤害系统（全仓搜不到 `health`、`fall_damage`），`pos.y < 0` 只被读成空气
- 玩家登录 / 重连时若存档位置在 p0,0 之外 → **传送到 p0,0 中心**
- 自动化协议里 `PIN` / `POUT` 用了 p0,0 以外的坐标 → **保持现状返回 `Inactive`**（不 clamp）

**已确认的决策**（用户 2026-10-25 逐条选择）

| 议题 | 决定 |
| --- | --- |
| 越界处理 | **不拉回、不阻挡**，外面是空的，玩家自由活动（空岛模型） |
| 越界后是否仍看见 p0,0 | 是，这是空岛模型的自然结果（具体区块发送行为见"待确认风险"） |
| 是否允许其它地皮加载 | **彻底删除**"按需加载地皮"路径（`server.rs:275`），任何原因都不再产生第二块地皮 |
| `remove_oob_players` | 不再把越界的玩家踢出地皮（现行为会导致服务器去加载新地皮），改为不作处理 |
| 登录 / 重连位置在岛外 | **传送到 p0,0 中心** |
| 自动化越界坐标 | 保持 `Inactive`，不 clamp |
| p0,0 生命周期 | **永远加载、永不卸载**（等价 BUG-009 的 A1） |

**代码现状（已核实）**

| 事实 | 位置 |
| --- | --- |
| `PLOT_SCALE = 5` → 地皮 32×32 chunk = **512×512** 方块；p0,0 覆盖 x,z ∈ [0, 512) | `plot/mod.rs:48` |
| 出生点 p0,0 启动时硬编码加载，`always_running = true` | `server.rs:194` |
| **唯一**的按需加载入口（全仓只有两处 `Plot::load_and_run` 调用，另一处是出生点） | `server.rs:275` |
| 越界玩家现在会被 `PlayerLeavePlot` 踢出 → 服务器加载新地皮 | `plot/mod.rs:1349` `remove_oob_players` |
| 无人 600 秒卸载地皮 | `plot/mod.rs:1506` |
| 无生命值 / 伤害系统 | 全仓无 `health` / `fall_damage` |
| 未加载地皮的自动化请求返回 `Inactive(pos)` | `server.rs:323-325` |

**拟定方案（未实施，仅备查）**

| 文件 | 改动要点 |
| --- | --- |
| `crates/core/src/server.rs` | 删除 `send_player_to_plot` 中的按需加载分支（:275 起的 `if !plot_loaded { Plot::load_and_run(..) }`），地皮永远只有启动时加载的 p0,0 |
| `crates/core/src/server.rs` | `route_automation_request` 的 `Inactive` 分支保留（地皮外坐标即无效） |
| `crates/core/src/server.rs` | 玩家加入时若 `pos.plot_pos() != (0,0)`，传送到 p0,0 中心 |
| `crates/core/src/plot/mod.rs` | `remove_oob_players`：改为不移除玩家（可保留函数但不再对越界玩家 `leave_plot`） |
| `crates/core/src/plot/mod.rs` | 卸载分支（:1506）对 p0,0 永不触发（`always_running = true` 已足够，或直接删除该分支） |
| 与 BUG-009 | 解除 `update()` 的 `players.is_empty()` 门禁（BUG-009 方案 A），使无人时也 tick |

**待确认风险（写代码前必须想清楚）**

1. **区块发送**：玩家在岛外时，`update_view_pos_for_player`（`plot/mod.rs:513`）会按他的新位置给客户端发区块。岛外的区块在 p0,0 的 `world` 里可能存在（全部空气）也可能不存在，取决于 p0,0 的 chunk 分配方式。若发送了岛外空区块，客户端可能按视距把 p0,0 的区块裁掉 → 与"回头仍能看见岛"冲突，需实测或读代码确认
2. **跨地皮坐标系**：`get_block` 用 `pos.x & 0xF` 之类的局部坐标（`plot/mod.rs:188-208`），越界坐标是否会串到别的 chunk / 越界 panic，需要确认
3. **worldedit / 其它命令**：WorldEdit 选区若跨出 512 边界，是裁剪还是越界 panic，需要确认
4. **自动化 `Inactive` 与 `PLOT_SCALE` 判定**：`server.rs` 用 `pos.x >> 4 >> PLOT_SCALE` 算地皮号，p0,0 之外的坐标会得到非 0 → 走 `Inactive`，符合决策

**验证方式（待实施后由用户执行）**

1. 启动服务器，不启动任何 MC 客户端 → p0,0 仍在 tick（配合 BUG-009 一起验证）
2. 玩家走到 x=513 / z=-1 → **不被拉回、不被传送**，周围是空，回头能看见岛
3. 玩家越界后，服务器**不产生**新地皮（`world/plots/` 下不应出现 `p1,0` 之类目录，`running_plots` 长度恒为 1）
4. 玩家在岛外重连 → 落到 p0,0 中心
5. 自动化脚本 `PIN` 一个 x=600 的坐标 → 收到 `Inactive`
6. 原有功能回归：`components`（18 例）+ `timings`（6 例）+ `ril_tests`（9 例）全绿

**实际改动（0.3.0-beta）**

| 文件 | 改动 |
| --- | --- |
| `crates/core/src/server.rs` | `send_player_to_plot`：签名改 `mut player`；若 `plot_pos() != (0,0)` 则 `player.teleport(PlotPos::center)`（实际用 `Plot::get_center(0,0)` + `PlayerPos::new(x, 64.0, z)`）；**删除**按需 `Plot::load_and_run` 分支，改为只在 `running_plots` 里查找，找不到就 `warn!` |
| `crates/core/src/server.rs` | 新增 `use crate::player::PlayerPos` |
| `crates/core/src/plot/mod.rs` | **删除** `remove_oob_players` 整个函数与 `update()` 中的调用，原地留注释说明原因 |

**行为**：玩家走到 p0,0 外面不再被踢出（原来会触发加载新地皮）；越界后的区块由既有的 `set_chunk_loaded_at_player` 发送空区块（`Chunk::encode_empty_packet`），因此"空岛外"是安全的，**不会 panic**。

**已知影响（2026-10-25 补充，已写入两份 README）**

- 依赖「切换地块」的命令（`/plot auto`、`/plot visit`、`/plot claim` 等）**不再能把玩家送到别的地皮**：`send_player_to_plot` 现在只会把非 p0,0 的位置拉回中心，而其它地皮永不加载
- `/teleport` 仍可把玩家移出岛外（它不经过 `send_player_to_plot`），外面是空的
- `get_center` 的 `y` 固定为 `64.0`（沿用 `send_player_away` 的写法），落地后由客户端自行调整


---

### BUG-011　编译崩溃时保存存档 → 终端打印崩溃信息 → 进程退出

**状态**：✅ **已实施**（0.3.0-beta，✅ release 编译通过，待运行验证）——见文末「实际改动」
**登记日期**：2026-10-25
**来源**：用户 2026-10-25 提出——"如果编译崩溃会怎么样，我需要服务器保存当前地块存档后再退出，在终端里打印崩溃信息"
**性质**：崩溃处理策略变更；**收窄**了 §6 中"其它 panic 崩溃点保持原样、不引入 `catch_unwind`"那条

**用户要求**

编译过程中发生不可恢复崩溃时：**先保存当前地块存档 → 在终端打印崩溃信息 → 然后退出**。

**已确认的决策**（用户 2026-10-25 逐条选择）

| 议题 | 决定 | 备注 |
| --- | --- | --- |
| "崩溃"的范围 | **只算真 panic**（不可恢复的 Rust panic） | BUG-004 那种**优雅中止**（`Stopped`，如 `Unsupported` 方块）**不退服**，保持现状 |
| 保存范围 | **保存崩溃的那块地皮** | BUG-010 落地后只有 p0,0，两者等价 |
| 退出行为 | 保存完 → 打印 → **整个进程 `exit(1)`** | 不是只结束地皮线程 |
| 终端信息 | **时间戳 + 地皮坐标 + 原始 panic 消息 + 存档路径 + 退出码** | 形如 `[CRASH] plot (0,0) redpiler panic at 2026-10-25T21:40: panic message... / saved to ./world/plots/p0,0 / exiting with code 1` |

**代码现状（已核实）**

| 事实 | 位置 |
| --- | --- |
| 本仓**未**设置 `panic = "abort"`（Cargo.toml 里无 panic 配置）→ 默认 **unwind** | `Cargo.toml` |
| `start_redpiler` 对非 `CompileError` 的 panic 载荷执行 `resume_unwind(payload)`，保留原行为 | `plot/mod.rs`（`match compile_result` 的 `Err(payload)` 分支） |
| `impl Drop for Plot` 里**本来就有** `self.save()`，unwind 时 `Plot` 被 drop → **存档其实已经会写**、`PlotUnload` 也会发 | `plot/mod.rs:1746+` |
| 地皮线程 panic 后，服务器**不会退出**：`handle_plot_unload` 只是把该地皮从 `running_plots` 列表里移除，进程继续跑 | `server.rs:226-234` |
| 默认 panic hook 会向 **stderr** 打印 `thread 'p0,0' panicked at src/...` | Rust 标准库默认行为 |
| 存在已知会在编译中 panic 的点，例如入边 ≥ 255：`backend/direct/compile.rs:47-58` | 见 §6 清单 |

**因此真正的缺口是**

1. **进程不退出**（地皮死了但服务器还在跑，玩家会被往已卸载的地皮送）
2. 终端信息**只有** Rust 默认 panic 输出，没有时间戳 / 地皮坐标 / 存档路径 / 退出码
3. 存档是否**确实写成**没有保证性验证（Drop 里的 `save()` 若在 panic 中再次 panic 会 abort，例如 `save_to_file` 的 `.unwrap()`）

**拟定方案（未实施，仅备查）**

| 方案 | 做法 | 评价 |
| --- | --- | --- |
| B1 | 在 `Plot::run` / `start_redpiler` 的崩溃路径显式捕获：`catch_unwind` 包住编译调用 → 记录 panic 载荷 → 显式 `self.save()` → 打印 `[CRASH] ...` → `std::process::exit(1)` | **推荐**：控制点明确，能保证保存后再退出 |
| B2 | 安装全局 `std::panic::set_hook`，在 hook 里保存 + 打印 + `exit(1)` | 覆盖所有 panic 点，但在 hook 里做带锁的存档再 exit 风险高（重入 / double panic） |
| B3 | 沿用 Drop 里的 `save()`，只在 `handle_plot_unload` 之后加"若 p0,0 消失则 exit(1)" | 改动最小，但存档失败无法保证，且拿不到 panic 原文 |

**与 §6 的关系（需同步）**

§6 现有条目「其它 panic 崩溃点 / plot 崩溃全局兜底」写的是"用户明确**不改**……不引入 `catch_unwind`，也不做全局 panic 静音"。BUG-011 只覆盖**编译路径的 panic**（保存 + 打印 + 退出），并非"静音"，也不是对所有 panic 点兜底。落地时需要在 §6 该行补一句收窄说明。

**验证方式（待实施后由用户执行）**

1. 人为构造一个必然 panic 的编译（例如让某节点入边 ≥ 255）→ 观察：
   - 终端出现一行 `[CRASH] plot (0,0) ... panic message ... saved to ./world/plots/p0,0 ... exiting with code 1`
   - 进程**退出**，退出码 **1**
   - `./world/plots/p0,0` 文件**已更新**（mtime 变化、内容可重新加载）
2. 重启服务器 → p0,0 能正常加载，崩溃前最后一次 tick 的状态还在
3. 回归：BUG-004 的优雅中止（`Unsupported` 方块）**不再退服**，仍是 `Stopped` + 红字报错
4. 原有测试全绿

**实际改动（0.3.0-beta）**

| 文件 | 改动 |
| --- | --- |
| `crates/core/src/plot/mod.rs` | `start_redpiler` 的 `Err(payload)` 分支：不再 `resume_unwind`，改为 `crash_save()` → `error!` 打印 → `stdout/stderr` flush → `std::process::exit(1)` |
| `crates/core/src/plot/mod.rs` | 新增 `Plot::crash_save()`：`catch_unwind(AssertUnwindSafe(|| self.save()))`，成功返回 `./world/plots/p{x},{z}`，失败返回带 `(SAVE FAILED)` 的标记（避免二次 panic 导致 abort 而无日志） |
| `crates/core/src/plot/mod.rs` | 新增自由函数 `panic_payload_message()`：从 `&dyn Any` 提取 `&str` / `String`，否则 `<non-string panic payload>` |
| `crates/core/src/plot/mod.rs` | 新增 `use std::io::Write;`、`use std::panic::AssertUnwindSafe;` |

**日志行**（`error!` → 同时进 stdout 与 `./logs/mchprs.log`，`tracing_subscriber::fmt` 默认带时间戳）：

```
[CRASH] plot (0, 0) redpiler panic: <原始 panic 消息> — saved to ./world/plots/p0,0 — exiting with code 1
```


---

### BUG-012　无玩家时无法触发编译 → 自动化脚本直接发送命令

**状态**：✅ **已实施**（0.3.0-beta，✅ release 编译通过，待运行验证）——见文末「实际改动」与「实施偏差」
**登记日期**：2026-10-25
**来源**：用户 2026-10-25 先指出"无玩家时自动编译条件过窄"，随后给出原始想法：
> 自动化脚本能发送 `/` 开头的数据，然后能实现执行玩家才能执行的命令，和玩家输入命令一样。

**用户目标**

让自动化协议成为**第二条命令入口**：脚本发一行以 `/` 开头的数据，服务器按玩家敲命令的方式执行它。这样无玩家时脚本也能自己触发 `/redpiler compile`，从而"无玩家 + 电路正常运行 + 脚本可用"整条链路闭合。

**已确认的决策**（用户 2026-10-25 逐条选择）

| 议题 | 决定 |
| --- | --- |
| 回显 | **只要一行确认 `OK CMD`**；命令自身的聊天输出**不回传** |
| 权限身份 | **等同 OP**（最高权限，不走 `plots.*` 判定） |
| 支持范围 | **只支持"不依赖玩家状态"的命令**；`~ ^` 相对坐标、`//load` `//save` 等 worldedit 命令返回 `ERR` |
| 作用于哪块地皮 | **固定 p0,0**（配合 BUG-010 单地皮） |
| 危险命令 | **不限制**，`/stop` 也可以发 |

**关键技术约束（已核实，决定了实现方式）**

`Player.client` 的类型是 `PlayerConn`（`crates/network/src/lib.rs:81`），内部直接持有 `TcpStream`：

```rust
pub struct PlayerConn {
    client: NetworkClient,   // 内部 stream: TcpStream
    alive: bool,
}
```

**因此无法伪造一个"虚拟玩家"** 来当命令执行者（需要一条真 TCP 连接，且该玩家会进 `players` 列表参与 tick、存档、广播）。只能把命令处理里对 `self.players[player]` 的依赖**抽象出来**。

`handle_command` 内 `self.players[player]` 出现 **78 次**，所以**不能整体改通用**；只对白名单命令做小范围抽象。

**方案：白名单命令通道**

### 第 1 层：协议（`crates/core/src/automation.rs`）

1. `parse_request`：在 `let op = parts.next()?.to_ascii_uppercase()` **之前**先判断 `line.starts_with('/')` → 返回新的 `ParseResult::Command(line.to_string())`（保留原始大小写，不能走 `to_ascii_uppercase`，否则玩家名/参数会被改写）
2. `poll()`：新增分支 → `AutomationAction::Command { conn_id, line }`
3. `AutomationAction` 新增变体 `Command { conn_id, line: String }`
4. 新响应类型（写回连接）：
   - 成功：`OK CMD`
   - 失败：`ERR CMD <CODE>`，`CODE` ∈ {`INACTIVE`, `UNKNOWN`, `ARG`, `ERR`}
   - 说明：用户要求"只要一行 OK CMD"，但**失败也必须回一行**，否则脚本会死等。故保留最小错误码集合

### 第 2 层：路由（`crates/core/src/server.rs`）

1. `handle_message` 里新增 `AutomationAction::Command` 分支
2. 新增 `route_automation_command(conn_id, line)`：**固定找 `plot_x == 0 && plot_z == 0`**
   - 找不到（p0,0 未加载）→ `ERR CMD INACTIVE`
   - 找到 → `PrivMessage::AutomationCommand { conn_id, line }`
3. 注意：**不走** `route_automation_request`（那个按坐标算地皮号，命令没有坐标）

### 第 3 层：执行（`crates/core/src/plot/mod.rs` + `plot/commands.rs`）

1. `PrivMessage` 新增 `AutomationCommand { conn_id, line: String }`，在 `handle_messages` 的 priv 分支里处理
2. `commands.rs` 新增：

```rust
#[derive(Clone, Copy)]
enum CommandTarget { Player(usize), Automation }
```

   并加辅助：

```rust
fn notify(&mut self, target: CommandTarget, msg: &str)      // Player → send_system_message；Automation → 丢弃
fn notify_err(&mut self, target: CommandTarget, msg: &str)  // Player → send_error_message；Automation → 丢弃
fn target_name(&self, target: CommandTarget) -> &str        // 日志用："automation" / 用户名
```

3. 白名单命令体里的 `self.players[player].send_*(...)` 改成 `self.notify(target, ...)`（**只改白名单命令，约 15 处**）
4. 新增入口：

```rust
fn handle_automation_command(&mut self, line: &str) -> AutomationCmdOutcome
```

   流程：去掉前导 `/` → 按空格切 `command` + `args` → `info!` 记日志（`automation issued command: ...`）→ 匹配白名单 → 执行 → 返回 `Ok` / `Err(code)`
5. **遇到 `~` / `^` 开头的坐标参数**（或命令本身在白名单外）→ `Err(UNSUPPORTED / UNKNOWN)`

**白名单（初稿，待用户确认）**

| 命令 | 纳入 | 理由 |
| --- | --- | --- |
| `/redpiler compile`（`/rp c`） | ✅ **核心** | 无玩家时唯一能触发编译的途径，本条的立项目的 |
| `/redpiler reset`（`/rp r`） | ✅ | 只操作电路状态，无玩家依赖 |
| `/stop` | ✅ | 用户明确不限制；内部只是发 `Message::Shutdown` |
| `/radvance <n>`（`/radv`） | ✅ | 手动推进 tick，只依赖 `tickn` |
| `/toggleautorp` | ✅ | 切 `auto_redpiler` 开关 |
| `/worldsendrate <hz>`（`/wsr`） | ✅ | 只改 `world.world_send_rate` |
| `/rtps` | ❌ | 输出被丢弃后**毫无意义**（报告只发给玩家） |
| `/version` | ❌ | 同上 |
| `/redpiler inspect` | ❌ | 依赖玩家视线射线检测（`ray_trace_block`） |
| `/plot *`（info/claim/auto/middle/visit/teleport/lock/unlock/select） | ❌ | `handle_plot_command` 第一行就是 `self.players[player].pos.plot_pos()`，全部玩家依赖 |
| `/speed`、`/gmc`、`/gmsp`、`/gamemode`、`/container`、`/teleport` | ❌ | 直接操作玩家自身状态 |
| `//load`、`//save`、`//paste`、`//undo` 等 worldedit | ❌ | 依赖玩家剪贴板 / 选区 / 视线 |

**已知取舍（需要你知晓）**

1. **编译失败脚本无法区分**：用户选择"只要 `OK CMD`"，因此 `/redpiler compile` 无论是编译成功还是被 `Unsupported` 中止，脚本都只收到 `OK CMD`。若将来需要，可扩展 `ERR CMD COMPILE <reason>`（本轮不做）
2. `/rtps`、`/version` 被排除，因为它们的全部价值就是输出文本
3. 自动化命令**等同 OP**，且端口只绑 `127.0.0.1`（`automation.rs`），所以本机任何进程都能关服/重置——这是用户明确接受的

**涉及文件**

| 文件 | 改动 |
| --- | --- |
| `crates/core/src/automation.rs` | `parse_request` 前置 `/` 判断；`AutomationAction::Command`；`poll()` 分支；响应行 `OK CMD` / `ERR CMD <CODE>` |
| `crates/core/src/server.rs` | 处理 `AutomationAction::Command`；新增 `route_automation_command`（固定 p0,0） |
| `crates/core/src/plot/mod.rs` | `PrivMessage::AutomationCommand`；在 priv 分支调用 `handle_automation_command` |
| `crates/core/src/plot/commands.rs` | `CommandTarget` + `notify` / `notify_err` / `target_name`；白名单命令体改造（约 15 处）；`handle_automation_command` |
| `src/protocol.txt`、`src/protocol_zh.txt` | 新增 `/` 命令行语法与 `OK CMD` / `ERR CMD` 说明 |
| `README.md`、`README_zh.md` | 协议版本与能力表更新 |

**协议版本**：这是**协议新增指令** → 建议随本次升到 **`0.2.3-beta`**（`automation.rs` 的 HELLO 字面量 + 两份 protocol 文档 + 两份 README）。

**验证方式（待实施后）**

1. 无任何 MC 客户端，脚本发 `/redpiler compile` → 收到 `OK CMD`；随后 `SUB`/`PIN`/`POUT` 不再返回 `INACTIVE`，电路开始 tick
2. 发 `/stop` → 服务器正常关闭（进程退出、地皮存档落盘）
3. 发 `/plot info` → 收到 `ERR CMD UNKNOWN`（不在白名单）
4. 发 `/redpiler compile ~ ~ ~` → 收到 `ERR CMD UNSUPPORTED`（相对坐标不支持）
5. 发不存在的命令 `/foobar` → `ERR CMD UNKNOWN`
6. p0,0 未加载时发命令 → `ERR CMD INACTIVE`
7. 玩家侧回归：所有原有命令行为不变（含 `/rtps`、`/plot *`、worldedit）
8. 自动化原有动词（`PIN`/`POUT`/`INPUT`/`SUB`/`READ`/`USE`/`SET`）行为与回包格式不变

**与其它条目的关系**

- 本条是"无玩家跑电路"链路的**最后一环**：BUG-009 让 tick 转起来、BUG-010 让地皮常在、本条让脚本能触发编译
- 落地后 `auto_redpiler = true` 不再是必须项（脚本可自行编译）；但两者不冲突，可同时开启
- 改动点与 BUG-009 都在 `update()` / `handle_commands()` 附近，建议 **BUG-009 → BUG-010 → 本条** 的顺序

**实际改动（0.3.0-beta）**

| 文件 | 改动 |
| --- | --- |
| `crates/core/src/automation.rs` | `parse_request` **在 `to_ascii_uppercase()` 之前**拦截 `line.starts_with('/')` → `ParseResult::Command(line)`（保留大小写）；新增 `ParseResult::Command`、`AutomationAction::Command { conn_id, line }`、`AutomationResponse::{CommandOk, CommandError(&'static str)}`；`poll()` 新增分支；`handle_reply` 新增 `OK CMD` / `ERR CMD <CODE>` |
| `crates/core/src/server.rs` | `PrivMessage::AutomationCommand { conn_id, line }`；`AutomationAction::Command` 分派；新增 `route_automation_command()`，固定路由 p0,0，未加载时回 `ERR CMD INACTIVE` |
| `crates/core/src/plot/mod.rs` | priv 分支调用 `handle_automation_command(&line)`，结果走 `Message::IoResponse` |
| `crates/core/src/plot/commands.rs` | 新增 `handle_automation_command()` + `execute_automation_command()`：剥掉 `/`、按空白切分、拒绝 `~`/`^`、`info!` 记录、白名单分派 |
| `src/protocol.txt`、`src/protocol_zh.txt` | 新增「命令」小节与 `OK CMD` / `ERR CMD CODE` 响应说明 |
| `README.md`、`README_zh.md` | 能力列表新增一条 |

**实施偏差（与 §「方案」不同，需记录）**

方案里写的是「引入 `CommandTarget` 枚举 + `notify/notify_err/target_name` 辅助，改造白名单命令体约 15 处 `send_*`」。实际改为**独立分派器**：新增 `execute_automation_command()` 直接调用同一批内部方法（`reset_redpiler` / `start_redpiler` / `tickn` / `publish_world` / `Message::Shutdown`），**完全不碰 `handle_command` 的 78 处 `self.players[player]`**。

理由：在白名单只有 6 个命令、且输出本就不回传（只要 `OK CMD`）的前提下，改 78 处调用点的风险远大于收益。代价是**两处逻辑需要同步维护**（玩家路径与自动化路径各一份），已在代码注释中写明。

**支持的子命令**（实际落地）：`/redpiler compile|c`、`/redpiler reset|r`、`/stop`、`/radv|radvance <n>`、`/toggleautorp`、`/worldsendrate|wsr <hz>`。其余一律 `ERR CMD UNKNOWN`。


---

### BUG-013　自动化端口绑定失败被静默吞掉

**状态**：✅ **已实施**（0.3.0-beta，✅ release 编译通过，待运行验证）——用户 2026-10-25 在大更新中一并授权
**登记日期**：2026-10-25
**影响文件**：`crates/core/src/server.rs`（启动处，约 :188）

**现象**

`AutomationServer::bind(CONFIG.automation_port)` 失败时，错误被 `.ok()` 直接丢弃：

```rust
automation: AutomationServer::bind(CONFIG.automation_port).ok(),
```

后果：

- 自动化桥接被**静默禁用**（`self.automation == None`）
- 服务器**照常启动**，其余功能看起来完全正常
- 日志里**没有任何提示**
- 脚本连 `127.0.0.1:25585` 连不上，或连上后所有请求都得不到应答，排查时毫无线索

**触发场景**：端口被其它进程占用、端口号非法（已有 `u16::try_from(...).unwrap_or(25585)` 兜底）、权限问题等。

**与 BUG-005 的相似性**：同一类"失败无反馈"毛病——BUG-005 是 `/whitelist` 失败无反馈导致误删白名单，本条是绑定失败无反馈导致自动化整体消失。

**方案（已规划，未实施）**

原则：**只加日志，不改变"绑定失败仍启动服务器"的行为**。自动化是可选组件，不应因为端口占用就让游戏服务器起不来。

### 改动 1：`crates/core/src/server.rs`（约 :188）

现状：

```rust
automation: AutomationServer::bind(CONFIG.automation_port).ok(),
```

改为显式匹配并把错误打出来：

```rust
let automation = match AutomationServer::bind(CONFIG.automation_port) {
    Ok(server) => Some(server),
    Err(err) => {
        warn!(
            "Failed to bind the automation bridge on 127.0.0.1:{}: {}. \
             MCHPRS_IO automation protocol is DISABLED; the game server keeps running.",
            CONFIG.automation_port, err
        );
        None
    }
};
```

### 改动 2：`crates/core/src/automation.rs` 的 `bind`（约 :216）

现状会把非法端口**静默**回退到 25585：

```rust
let port = u16::try_from(port).unwrap_or(25585);
```

配置成 `-1` 或 `70000` 时没有任何提示，最后连的是 25585，排查同样困难。改为：

```rust
let port = match u16::try_from(port) {
    Ok(port) => port,
    Err(_) => {
        warn!(
            "automation_port {} is not a valid port number, falling back to 25585",
            port
        );
        25585
    }
};
```

### 不做的事

- 不做"端口被占用就换端口重试"（会让脚本无法预期端口）
- 不把警告升级成 **fatal**（保持服务器可用）
- 不在成功路径加日志（避免噪音）

**验证方式（待实施后）**

1. 先占用 25585（例如 `python3 -m http.server 25585`），再启动服务器 → 终端出现一条 `Failed to bind the automation bridge on 127.0.0.1:25585: ... DISABLED ...`，服务器**照常启动**
2. 把 `Config.toml` 的 `automation_port` 改成 `70000` → 终端出现 `not a valid port number, falling back to 25585`，且实际监听 25585
3. 正常情况（未被占用、端口合法）→ **零日志噪音**，自动化正常连上

**实际改动（0.3.0-beta）**

| 文件 | 改动 |
| --- | --- |
| `crates/core/src/server.rs` | `automation: AutomationServer::bind(...).ok()` 改为 `match`：`Err` 时 `warn!` 打印端口 + 错误原因 + 「automation DISABLED，游戏服务器继续运行」，仍存 `None` |
| `crates/core/src/automation.rs` | `bind()` 里 `u16::try_from(port).unwrap_or(25585)` 改为显式 `match`，非法端口时 `warn!` 再回退 |



---

### BUG-014　空容器警告改为红色，并一次列出全部坐标

**状态**：✅ **已实施**（0.4.0-beta，✅ release 编译通过，待运行验证）——用户 2026-10-26 授权"写代码+编译+更新文档"
**登记日期**：2026-10-26
**影响文件**：`crates/redpiler/src/task_monitor.rs`、`crates/redpiler/src/passes/frontend/identify_nodes.rs`（:113）、`crates/core/src/plot/mod.rs`（:979-989）

**用户原话（2026-10-26）**

> 现在如果遇到空容器是聊天框弹黄色警告，是吧，改成红色警告，还有，如果有多个坐标是空的话，一起列出来，例如，Fedpiler compile warning: comparator at (143,89,367),(126,69,127),(... reads some empty containers 不用考虑单复数英文语法规则

**现状（0.3.0-beta 已实现的 BUG-001 修订）**

一次编译里**第一个**空容器 → 聊天框黄色一条：

```
Redpiler compile warning: comparator at (x, 64, z) reads an empty barrel
```

链路：

| 位置 | 现状 |
| --- | --- |
| `comparator.rs:70` | 空容器 → `Err(OverrideError::EmptyContainer { pos, kind })` |
| `identify_nodes.rs:113` | 命中 `EmptyContainer` → `monitor.set_warning(error)`，按 `NodeType::Constant` / `ss(0)` 继续编译 |
| `task_monitor.rs:set_warning` | **只保留第一条**（`if slot.is_none()`），后来的全部丢弃 |
| `plot/mod.rs:979` | 编译成功后，`if let (Some(player), Some(EmptyContainer { pos, kind }))` → `send_system_message`（`ColorCode::Yellow`） |

三个毛病：

1. **颜色是黄色**，用户要求红色。
2. **后续空容器被丢弃** → 玩家修掉一个再编译，又冒出一个，得反复来回。
3. 只报一个坐标，**不知道总共有几处**。

**拟定方案（未实施，仅备查）**

1. `TaskMonitor::warning` 单槽 → 列表：`warnings: Mutex<Vec<OverrideError>>`；`set_warning` 改为追加（同一 `pos` 去重），`warnings()` 返回克隆。`set_error` 保持"只留第一条"不变（那是中止路径）。
2. `identify_nodes.rs:113` 调用点语义不变，只换成追加版。
3. `plot/mod.rs:979` 成功后取出全部空容器坐标，拼成**一条**消息：

   ```
   Redpiler compile warning: comparator at (143,89,367),(126,69,127) reads some empty containers
   ```

   - 坐标之间用 `,` 连接，坐标内部 `(x,y,z)` 无空格（照用户示例）
   - 末尾固定 `reads some empty containers`，单复数不做区分（用户明确豁免）
4. 红色：`send_system_message` 是 Yellow、`send_error_message` 是 Red（`player.rs:629-644`）。实现取**最小改动**——直接改用 `send_error_message`（红），文案仍写 `warning`；暂不新增 `send_warning_message` 变体。

**已确认的细节（用户 2026-10-26 逐项选择）**

| 问题 | 结论 |
| --- | --- |
| 坐标语义 | **空容器自身坐标**（保持现状），文案沿用 `comparator at` |
| 容器种类 | **丢掉**，不写 `barrel`/`hopper`，只留 `(x,y,z)` |
| 数量上限 | **超过 10 个截断**：只列前 10 个，末尾接 `,(...)` |
| 单个时文案 | **统一** `reads some empty containers`，不区分单复数 |

最终形态：

```
Redpiler compile warning: comparator at (143,89,367),(126,69,127) reads some empty containers
```

> 超过 10 个时空容器：列出前 10 个，第 11 个起用 `,(...)` 概括；**不报总数**（`(...)` 已表示"还有"；若要"共 27 处"这种计数，需另行提出）。
> 坐标格式统一成 `(x,y,z)` **无空格**（照用户示例；`BlockPos` 的 `Display` 自带空格，需手动格式化，不走 `Display`）。

**验证方式**

p0,0 摆两个比较器分别读空木桶 / 空漏斗 → `/redpiler compile` → 聊天框**一条红色**消息含两个坐标；自动编译（`source == None`）仍静默；只放一个空容器时文案符合第 4 点的决定。

**备注**

- 本条与前 13 条无代码耦合，可独立实施。
- 颜色改动只影响聊天框展示，协议层与 `EVENT` 无关。

### BUG-015　从文件读取 ROM（Logisim v3.0 hex）驱动红石 IO

**状态**：✅ **已实施**（0.4.0-beta，✅ release 编译通过，待运行验证）；`*.ram.txt` 语义仍待用户补充，本期未实现
**登记日期**：2026-10-26
**目标版本**：**0.4.0-beta**（用户 2026-10-26 确认：内核版本与协议版本**都**升到 0.4.0-beta）
**影响文件（预判）**：新增 `crates/core/src/memory.rs`（解析 + 每 tick 读写）；`crates/core/src/plot/mod.rs`（tick 循环里挂 ROM 采样）；`crates/core/src/server.rs`（启动加载 `memory/`）；`crates/core/src/commands.rs`（`/memory reload`）；`crates/core/src/plot/commands.rs`（自动化命令白名单加 `memory reload`）；`crates/core/src/automation.rs`（HELLO 版本号）；`Cargo.toml`（内核版本）

**草案原文（2026-10-26，逐字；另存 `~/mchprs/drafts/rom-draft-2026-10-26.txt`）**

```
xxx.rom.txt
v0.3.0-beta hex words addressed
clk = 0/具体坐标
//这里的时钟触发才会读取对应地址的数据
address coordinate = [
12, 6, 7
13, 4, 5
12, 6, 6
13, 4, 6
14, 5, 5
12, 6, 8
]  //这个不限制位宽
data coordinate = [
12, 6, 7
13, 4, 5
12, 6, 6
13, 4, 6
14, 5, 5
12, 6, 8
13, 4, 4
]  //坐标不重要，这个是演示，位宽也不限制
//下面的格式按照logisim的格式读取，只能读取，不能生成
```

**关键发现：现有 PIN / POUT 端口就是同一套语义（可直接复用）**

| 现有机制 | 点必须是什么 | 时钟必须是什么 | 服务器行为 |
| --- | --- | --- | --- |
| `PIN` + `INPUT` | 拉杆 / 踩踏板 | 按钮 | **写**：`redpiler.try_set_powered(pos, bool)` |
| `POUT` + `OUTPUT` | 红石灯 / 活板门 / 音符盒 | 同类输出元件 | **读**：`redpiler.query_io(pos) -> (IoElement, powered, output_power)` |

校验函数在 `crates/core/src/plot/mod.rs`：`validate_pin_port`（:884）、`validate_pout_port`（:903）；写入在 :858 的 `AutomationRequest::Input` 分支。

用户的 ROM 需求正好是它的镜像：

- `address coordinate` + `clk` → 按 **POUT 规则**读（灯 / 活板门 / 音符盒）
- `data coordinate` → 按 **PIN 规则**写（拉杆）

`POUT` 已实现上升沿检测（`PlotDataPort.clk_prev`，:1112-1125）与位向量打包，可直接照搬。

**"这是编译后的行为"的确切原因**：`query_io()` 只有在 redpiler 编译完成、后端图存在时才返回 `Some`；未编译时 PIN/POUT 一律回 `INACTIVE`。所以元件类型校验天然发生在编译之后——与用户说法一致，不需要额外设计。

**已确认的需求（用户 2026-10-26 逐项选择）**

| # | 项 | 结论 |
| --- | --- | --- |
| 1 | 归属 | **内核内置**，启动读文件、每 tick 自己采样，不依赖脚本 |
| 2 | 首行版本 | **严格**：必须等于当前内核版本，即 `v0.4.0-beta hex words addressed`，不符拒绝加载 |
| 3 | 文件目录 | 服务器顶层新建 `memory/`，与 `Config.toml` 同级（`/storage/emulated/0/termux/MCHPRS_IO/memory/`） |
| 4 | 文件识别 | **只认 `*.rom.txt`**；`*.ram.txt` 用户表示"还没想好"，先记着，本期不实现 |
| 5 | 加载范围 | `memory/` 下所有 `*.rom.txt` **全部加载**，一个文件 = 一个 ROM |
| 6 | 冲突 | **地址线、数据线允许冲突**（多个 ROM 共用引脚）；**时钟不允许冲突** |
| 7 | 基准坐标 | 新增字段 `origin`。`origin = 0` 与 `origin = 0, 0, 0` **都表示世界绝对坐标**（加零等价，仅解析分支不同）；否则文件里所有坐标 = 文件值 + origin，**x/y/z 三个方向都加** |
| 8 | 位序 | 数组**第 1 个元素 = 最高位**（MSB first），地址线、数据线同理 |
| 9 | address / clk 元件 | 只能是**输出元件**：红石灯 / 活板门 / 音符盒（与现有 POUT 校验一致） |
| 10 | data 元件 | 只能是**拉杆**（现有 PIN 还允许踩踏板，本条只接受拉杆） |
| 11 | 输入判定 | 信号 **> 0 即视为 1**（读元件的 powered / lit 布尔） |
| 12 | 输出电气 | 写拉杆：1 = 通电（信号 15），0 = 断电 |
| 13 | 写入方式 | **全量写**：每次输出把整条数据总线的所有位（含 0）都写一遍 |
| 14 | 触发 | `clk` 为具体坐标时：**上升沿 0→1 读一次地址并锁存输出** |
| 15 | 无时钟 | `clk = 0` 表示没有时钟 → **用户明确"先等会，还有其他问题"**，本期先不定 |
| 16 | 地址越界 | 超出文件出现的最大地址 → **输出 0**（与 Logisim 一致） |
| 17 | 字宽 | 以 `data coordinate` 条目数为准；word 更宽时**取低位**（低 7 位式，与 Logisim 一致） |
| 18 | 出错处理 | **一律不阻断**：解析失败 / 元件类型不符 → 只报错（日志 + 红色聊天），服务器继续跑 |
| 19 | 重载 | 支持 `/memory reload`，**玩家与自动化命令通道都能发**（需加进 BUG-012 的命令白名单） |
| 20 | 版本 | 内核、协议**都升 0.4.0-beta** |

**补充确认（用户 2026-10-26 第二批选择）**

| # | 项 | 结论 |
| --- | --- | --- |
| 21 | 时钟冲突 | **两个都拒绝**：两只 ROM 的 `clk` 撞同一坐标时，两个都不生效并各自报错 |
| 22 | 踩踏板 | data **拉杆和踩踏板都行**（与现有 PIN 的 `Lever | PressurePlate` 一致） |
| 23 | 时序 | 接受 **1 tick 延迟**：clk 上升沿所在 tick 结束后读地址、写数据，下一 tick 红石才看到变化 |

| 24 | 地址/数据重合 | **不做检测，允许重合**；但"无时钟 + 重合 = 每 tick 振荡"的风险必须写进 `protocol*.txt` 与 `README*.md` |
| 25 | `clk = 0` 语义 | **地址值变化的那一刻才写一次**，同样 1 tick 延迟（本期实现） |
| 26 | 脚本 vs ROM | **ROM 赢**：脚本在 `handle_messages()`（tick 前）写，ROM 在采样点（tick 后）写，同一 tick 里 ROM 最终生效 |
| 27 | 玩家 vs ROM | **ROM 赢**：ROM 每采样点全量重写，玩家手动拨 data 拉杆会被改回去；文档写明"data 拉杆归 ROM 控制" |

| 28 | 注释语法 | `#`、`//`、`/* */` **三种全部接受**，配置区与数据区通用；`/* */` 可跨行；行尾注释也允许 |

| 29 | 空格 / 制表符 | 任意数量都只是分隔符；数字按十六进制**整数**解析，前导零无意义（`0f` = `f` = `000f` = `0x0f`），大小写均可 |
| 30 | 折行 | **换行也当分隔符，可以折行**：不写地址的行继续上一个地址 |
| 31 | word 超宽 | 爱写多长写多长，**只取低 N 位**（N = data 坐标数），不报错 |
| 32 | 地址写法 | 宽松：`00:` / `1f:` / `0x1f:` / `1F:` 都行，前导零随意 |
| 33 | word 分隔 | **必须用空格（或换行）分隔**，不做"连续 hex 自动切片"：`7f7f7f7f` 是**一个** 64 位 word，不是多个 |

| 34 | 元数 | **严格**：`clk` / `origin` 只接受 1 个数（哨兵 `0`）或 3 个数（坐标）；列表项必须是 3 个数。其他元数一律报错，不猜、不补 |
| 35 | `origin` 作用范围 | **`clk` 也加 `origin`**，所有坐标统一处理 |
| 36 | 行可省略性 | `clk` 与 `origin` **两行必须写全**，缺一行即报错 |

| 37 | 列表开括号位置 | `address coordinate =` / `data coordinate =` 后的 `[` **允许写在下一行**；✅ 0.4.1-beta 已实现（`parse_list` 找不到 `[` 就继续读下一行） |

**用户主动指出的细节（原话）**

> "PIN/POUT和这个文件的解析不一样，额，具体来说，是十六进制编码不一样，在这个阶段应该没有影响"

核实结论：**确实没有影响，而且不需要对齐**。

- 现有 `PIN`/`INPUT` 走的是**协议 hex**：`automation.rs` 里 `pack_hex` / 解包约定 **bit 0 = 最高位**，一条端口状态打成 `ceil(N/4)` 个 hex 字符。
- ROM 文件走的是**Logisim 每 word 一个 hex 数**：word 是有数值意义的整数，data 总线按"数组第 1 个 = 最高位"逐位摆放，word 比总线宽时取低位。
- 两者是**两条互不相干的通路**：ROM 直接读写方块状态，完全不经过 hex 打包。只要实现时不复用 `pack_hex`，就不存在一致性问题。

**实现细节（拟定，待用户认可）**

1. **生效前提**：ROM 只在 redpiler 编译完成、后端图存在时生效（`query_io` 才有值）。未编译 / 编译中 / 已停止 → ROM 静默不动作，与 PIN/POUT 的 `INACTIVE` 一致。
2. **类型校验时机**：加载文件时**不做**方块类型检查（那时地皮可能还没加载）；redpiler 每次编译成功后，逐条 ROM 校验全部 `address`/`clk`/`data` 坐标，失败的那条 ROM 不生效并报红色聊天消息，其余 ROM 照常。
3. **坐标范围**：经 `origin` 换算后必须落在已加载地皮内；越界坐标等同类型校验失败（报错、该 ROM 不生效）。

**坐标与哨兵值（已确认）**

| 写法 | 含义 |
| --- | --- |
| `clk = 0` | **没有时钟**（哨兵值，1 个数） |
| `clk = 0, 0, 0` | **真实坐标**：世界原点处的那个方块（若设了 `origin`，则指向 `origin` 位置） |
| `origin = 0` | 没有基准 → 所有坐标是世界绝对坐标 |
| `origin = 0, 0, 0` | 基准在世界原点 → 加零，结果同样是世界绝对坐标 |

判据：**单个 `0` = 哨兵（不存在）；逗号分开的三个数 = 坐标。**

- 元数严格：`clk` / `origin` 接受 1 个或 3 个数；坐标列表每项必须 3 个数。其他情况报错。
- `clk` 与 `origin` 两行**必须写全**，不能省略。
- `origin` 对 `clk` 同样生效。

**数据区解析规则（已确认）**

| 情况 | 处理 |
| --- | --- |
| word 值 < 2^N | 高位补 0 |
| word 值 >= 2^N | **只取低 N 位**，高位丢弃，不报错 |
| word 只写 1 个 hex 字符（`f`） | 合法，按整数 15 处理 |
| `7f7f7f7f` 这种连写 | **一个** word（64 位），取低 N 位；要多个 word 必须空格分隔 |
| 一行写不完 | 可以折行，换行等价于空格 |
| 没列出的地址 | 读作 0 |

N = 6 时的换算示例：

| word | 值 | 取低 6 位 | 总线状态（第一个坐标是 bit5） |
| --- | --- | --- | --- |
| `2d` | 45 | `101101` | `1 0 1 1 0 1` |
| `ff` | 255 | `111111` | `1 1 1 1 1 1` |
| `a5` | 165 | `100101` | `1 0 0 1 0 1` |
| `f` | 15 | `001111` | `0 0 1 1 1 1` |

7 位数据总线的写法：一个 word 是 0..127，两位 hex 足够，如 `00: 7f` → `1111111`；要写 8 个 word 就 `00: 7f 7f 7f 7f 7f 7f 7f 7f`。

**执行时序（已核实代码，`plot/mod.rs`）**

一次 `Plot::update()` 的固定顺序：

| 步骤 | 位置 | 发生什么 |
| --- | --- | --- |
| 1 | `handle_messages()`（:1418） | 排空私有通道（`while let Ok(...)`，:1344），**脚本的 `INPUT`/`SET` 在这里按到达顺序全部应用**（tick 之前） |
| 2 | tick 批次 + 采样（:1480 / :1499） | 红石传播；`sample_pout_ports()`；**ROM 的读地址 / 写数据挂在这里**（tick 之后） |
| 3 | `update_players()`（:1539） | 玩家交互（手动拨拉杆、放方块）生效 |
| 4 | `handle_commands()` | 聊天命令 |
| 5 | `remove_dc_players()` | 掉线清理 |

关键事实：

- **脚本与 ROM 不是同一个通道**：脚本走 TCP → `server.rs:363` → mpsc；ROM 在地皮线程内部直接调 `try_set_powered`。两者唯一的交点是 `try_set_powered(pos, bit)`。
- **写操作不触发 tick**：`set_node`（`redpiler/backend/direct/mod.rs:137`）立即改本节点、立即改下游输入和（`ss_counts` 加减）、立即就地重算相邻节点（`update_node`，`update.rs:7`）——灯亮立即生效、活板门立即翻转、灯灭排 2 tick、中继器/火把/比较器走 pending tick。
- **脚本连写同一元件**：同一批排空内后者覆盖前者，中间值不会被任何 tick 看到，但会收到两条 `OK`；跨批次则中间值会被 tick 采纳。
- **ROM 的反馈环**：写 data → 影响地址（若重合）→ 下一个采样点读到新地址，每 tick 迭代一次，**不会死循环，但也不会收敛**。代码里没有任何重合检测。

**文件语法（拟定，未定稿）**

```
v0.4.0-beta hex words addressed
clk = 0                          // 0 = 无时钟（行为待定）；否则写具体坐标
origin = 0                       // 0 或 0, 0, 0 = 世界绝对坐标；否则为基准偏移
// 配置区注释用 //
address coordinate = [
12, 6, 7
13, 4, 5
]
data coordinate = [
12, 6, 7
13, 4, 5
]

// 数据区：Logisim v3.0 hex words addressed 规则，# 注释
00: 000 00a 00a 00a fdd 077 fc0 008
08: 070 05a fe4 fcd 076 ff0 fec 067
```

分区方式：**靠首行自动识别**（用户选 1）。首行之后到 `hex words addressed` 结束为配置区；其后遇到第一条 `<十六进制地址>:` 形式的行即进入数据区。

注释：`#`、`//`、`/* */` 三种**全部接受**（用户 2026-10-26 明确），配置区与数据区通用，可跨行、可出现在行尾。注意 Logisim 原生只认 `#`，所以带 `//` / `/* */` 的文件不能直接喂给 Logisim —— 本服务器只读、不生成，不受影响。

> 用户问过"自动识别会有问题吗"。风险点：配置区不能出现以 `xx:` 开头的行；空行 / `#` 注释在数据区要跳过；地址行允许前导空白。这些都会显式处理，风险可控。

**验证方式（草案澄清后定稿）**

8 位地址 × 16 位数据的 ROM，`00`..`07` 写不同值；地址线 0/1/2/3；`clk` 打上升沿；核对数据总线数值与文件一致；越界地址输出 0；类型不匹配时红色聊天提示且服务器不崩。

**备注**

- 0.4.0-beta 的第一个条目，编号 BUG-015。
- 与 BUG-014（空容器警告红色）无耦合。
- 用户明确"还有其他问题"，本条会继续追加。
- **待补充**：`*.ram.txt`（可写 RAM）的语义——用户 2026-10-26 表示"先记着，我还没想好"，本期不实现。


### BUG-016　`INPUT` / `OUTPUT` 的 hex 改为"整条数据前端补零"（原记于 `PIN` / `POUT`）

**状态**：✅ **已实施**（0.4.0-beta，✅ release 编译通过，待运行验证）
**登记日期**：2026-10-26
**目标版本**：0.4.0-beta（与 BUG-015 同批）
**影响文件**：`crates/core/src/plot/mod.rs`（`pack_hex`，:130）、`crates/core/src/automation.rs`（`parse_hex_bits`，:870；`hex_len` 不变，:85）；`src/protocol.txt` / `src/protocol_zh.txt`
**破坏性**：**是**（与 BUG-003 同类）。位宽不是 4 的倍数时 hex 会变，老脚本必须跟着改。

**用户原话（2026-10-26）**

> 啊，还有，PIN和POUT的数据改成在前面填充0，现在是不是在后面填充0计算十六进制？

> 不对，是0010 1101，在整个数据的前面补0，和rom的行为一样

**语义（用户最终口径）**

把整条端口的位向量当成一个**无符号整数**（第 1 个点是最高位），按 `ceil(N/4)` 个十六进制字符输出，补零补在**整条数据的最高位端**（前面），和 Logisim / ROM 对 word 的处理一致。

**现状（已核实）**

`pack_hex`（`plot/mod.rs:130`）按 4 位一组**从左往右**切，最后一组不足 4 位时把已有位左对齐、在低位补零：

```rust
value = (value << 1) | u8::from(*bit);
value <<= 4 - chunk.len();      // ← 补零补在最后一位半字节的后面
```

`parse_hex_bits`（`automation.rs:870`）对称：每个 hex 字符拆 4 位，从头取前 N 位，末尾多余丢弃。

例：N = 6，位序 `1 0 1 1 0 1`（值 = 45 = `0x2d`）

| | hex | 拆解 |
| --- | --- | --- |
| 现在 | `b4` | `1011` + `0100`，补的零在**后面** |
| 改为前端补零 | `2d` | `0010` + `1101`，补的零在**前面** |

**改动要点**

1. `pack_hex`：改为**从右往左**切组——最前面一组只含 `N % 4` 位（`rem == 0` 时含 4 位），后面每组 4 位；补零自然落在最前面那一组的**高位**（即整条数据的最前端）。等价说法：位向量按大端整数编码，左侧补零到 `ceil(N/4)` 位 hex。
   - 例：N = 6，`rem = 2` → 第 1 个字符 = 低 2 位 `10` = `2`，第 2 个字符 = `1101` = `d` → `2d`
2. `parse_hex_bits`：对称——**第 1 个 hex 字符只取低 `N % 4` 位**（跳过它高位的补零），其余字符各取 4 位。
   - 不能简单"取前 N 位"或"取后 N 位"，两种都会错。
3. `hex_len()` 不变，仍是 `ceil(N / 4)`。
4. `N % 4 == 0` 时行为完全不变。
5. 大位宽不能靠 `u128` 聚合，必须逐组处理（位宽无上限）。

**已确认**

- `INPUT` 解析侧**一起改**，两边严格对称。
- **不做兼容**：旧格式（`b4`）直接报错，脚本必须跟着改；protocol 文档写明 breaking change。

**验证方式**

- N = 6 的 `PIN`：`INPUT name 2d` → 6 个点状态为 `1 0 1 1 0 1`；旧格式 `b4` → 报错。
- N = 6 的 `POUT`：`OUTPUT` 行输出 `2d`（原 `b4`）。
- N = 5：位 `1 0 1 1 0` → `16`。
- N = 8：与改动前完全一致。


### BUG-017　data 拉杆与 address 元件相邻 → 禁用该存储器

**状态**：✅ **已实施**（0.4.1-beta，✅ release 编译通过，待运行验证）——用户 2026-10-26 授权 0.4.1-beta 一并实施
**登记日期**：2026-10-26
**目标版本**：0.4.0-beta（与 BUG-015 同批发布）
**影响文件**：`crates/core/src/memory.rs`（`Memory::validate`、`Invalid`、`describe_invalid`）、`README.md` / `README_zh.md` / `src/protocol.txt` / `src/protocol_zh.txt`

**现象（用户 2026-10-26 实测）**

ROM 里写的是固定序列，输出却一直变，数据拉杆持续抖动。

用户当时的文件（`memory/test.rom.txt`）：

```
clk = 0
address coordinate = [
258, 10, 263
256, 8, 263
]
data coordinate = [
256, 14, 264
256, 12, 264
256, 10, 264
256, 8, 264
]
0: 1 2 3 4
```

**根因：电平耦合，不是 ROM 逻辑错**

数据第 4 位（最低位）的拉杆在 `(256, 8, 264)`，地址第 2 位（最低位）的灯在
`(256, 8, 263)`——**z 差 1，直接相邻**。拉杆通电会点亮这盏灯。

于是形成 1 tick 反馈环：

```
地址 0 → word 1 = 0001 → 最低位拉杆通电 → 相邻的地址灯亮 → 地址变成 1
地址 1 → word 2 = 0010 → 最低位拉杆断电 → 地址灯灭   → 地址回到 0
```

`clk = 0`（无时钟）让这个环没有任何刹车：地址一变立刻重写数据，数据又改变地址。

这正是 BUG-015 第 24 条记录的"无时钟 + 地址数据耦合 = 每 tick 振荡，服务器不检测"
——只是耦合方式是**方块相邻**，比坐标重合更隐蔽。

**方案（拟定，未实施）**

在 `Memory::validate` 里增加相邻检查：任意一对 `data` 坐标与 `address` / `clk`
坐标**曼哈顿距离为 1**（共面相邻）时，返回新的 `Invalid::Adjacent`，那条存储器
被禁用并给出红色提示，其余存储器照常工作。

- 只挡面相邻，挡不住"绕一段红石线再回来"这类间接耦合——文档里保留这条说明。
- 检查发生在编译成功后（与元件类型校验同一时机），因为只有那时才知道方块确实存在。

**验证方式**

用户原文件直接加载 → 编译成功后应看到红色
`Memory test.rom.txt disabled: data lever at (256,8,264) is adjacent to address element (256,8,263)`；
把数据排挪到 `z = 266` 后应恢复正常，地址固定时输出不再变化。

**备注**

- 与 BUG-015 同一文件，一起发布。
- 文档需同步：`README*`、`protocol*` 里"服务器不检测重合"的说法要改成
  "检测相邻耦合，但不检测间接耦合"。

### BUG-018　暴露点状红石粉的 0–15 信号强度

**状态**：📝 **已登记，待澄清**（用户 2026-10-26 提出并同意立项；未授权写代码）
**登记日期**：2026-10-26
**目标版本**：拟定 0.4.x（不与 0.4.0-beta 混发）
**影响文件（预判）**：`crates/redpiler/src/backend/mod.rs`（`IoElement`）、`crates/redpiler/src/backend/direct/mod.rs`（`io_element`、`query_io`、`try_set_power`）、`crates/redpiler/src/backend/direct/compile.rs`（入边统计）、`crates/redpiler/src/backend/direct/node.rs`（`Node` 标记位）、`crates/redpiler/src/backend/direct/update.rs`（`IoEvent.powered` 合成）

**需求（用户原话，2026-10-26）**

> 就是只识别点状红石粉，写入的话也只能写入点状红石粉，并且这个点状红石粉没有输入，也没有被别的信号覆盖，这样是否可行？

> power是信号强度，红石粉必须检测入边为0，才能做输入

**核心思路（已与用户对齐）**

1. **只识别孤点**：`wire::is_dot(block)`（四面都不连）才纳入，普通红石粉不暴露。
2. **`powered` 是派生值**：红石粉节点的 `powered` 字段恒为 `false`（`NodeState::ss()` 不设它，`update.rs` 的 Wire 分支只更新 `output_power`），所以对外必须合成，算法规格为 `powered = output_power > 0`。
3. **`power` 才是本体**：0–15 的信号强度，即 `node.output_power`。
4. **写入只允许入边为 0 的孤点**：判据不能用"当前输入和为 0"（驱动源熄灭时也是 0，但入边还在），必须用**图里没有任何 forward link 指向该节点**。
   - 依据：`update_node` 只有两个调用点——`set_node`（更新自己的**后继**）与 `tick_node`（只处理 repeater / torch / comparator / lamp 等）。红石粉不会给自己排 tick。因此无入边的点永远不会被重算，写进去的 `output_power` 会一直保持。
   - 有入边的孤点：可以读，但写入会被下一次传播覆盖，应当拒绝写入。

**为什么可行（已核实代码）**

| 事实 | 位置 |
| --- | --- |
| `query_io` 本来就返回 `(IoElement, powered, u8 output_power)` | `backend/direct/mod.rs:312` |
| `IoEvent` 自带 `output_power` | `backend/mod.rs:40` |
| `flush` 会把强度写回世界方块 `wire.power = node.output_power` | `backend/direct/mod.rs:283` |
| `io_element()` 目前对 `NodeType::Wire` 返回 `None`（唯一真正的门禁） | `backend/direct/mod.rs:365` |
| 后端保留原方块 `blocks: Vec<SmallVec<[(BlockPos, Block); 1]>>`，可用于判定是否孤点 | `backend/direct/mod.rs:123` |
| `try_set_powered` 只接受 `Lever / PressurePlate`，需要新增写强度的入口 | `backend/direct/mod.rs:341` |

**拟定实现（未实施）**

1. 编译期：`wire::is_dot` 判定孤点；遍历所有节点的 forward link 统计**入边**，给 `Node` 打标记（如 `wire_dot_input_free: bool`）。
2. `IoElement` 增加 `WireDot` 变体，`code()` 加 `6`。
3. `io_element()` 对 `Wire` 返回 `Some(WireDot)`（仅当孤点）。
4. `query_io` / `IoEvent`：`powered` 合成为 `output_power > 0`。
5. 新增 `try_set_power(pos, u8)`：只对"孤点 + 入边 0"生效。
6. `flush` 无需改动（已写回强度）。

**风险与边界（需写进文档）**

- **必须带 `-d`（`--wire-dot-out`）编译**：否则 `--optimize` 会丢掉非 IO 的线，`prune_orphans` 也只保留输出节点及其上游，孤点不在图里，`query_io` 返回 `None`。
- **写入值活到下次编译**：编译从世界方块重建图，之前写入的强度会丢。
- **入边是编译期算的**：之后在附近放方块产生新连接，需重新编译；重编译后若入边不为 0，该点不再接受写入。
- **只对 `-d` 保留的孤点有效**，不承诺对普通导线生效。
- **`-i`（io-only）模式正好合适**：flush 只写 IO 节点，孤点被标为 output，会连同强度一起刷给客户端。

**待澄清的 4 点**

1. **`powered` 的合成规则**：`powered = output_power > 0`（强度为 0 时返回 0）？还是别的口径？
2. **读取是否也要求入边为 0**：有入边的孤点能不能读？（写入肯定不行）
3. **是否需要接入 `PIN` / `POUT` 数据端口**：即一个点贡献 4 bit（一位 hex），还是先只做单点 `query_io` / `try_set_power` 这一层。
4. **能否接受"必须 `-d` 编译"**，还是希望不倚赖这个开关（那就要在编译期强制保留孤点）。

### BUG-019　红石粉相关的兼容性问题

**状态**：📝 **已登记，待用户补充细节**（未授权写代码）
**登记日期**：2026-10-26
**目标版本**：待定
**影响文件**：待定（预判涉及 `crates/redstone/src/wire/`、`crates/redpiler/src/passes/frontend/identify_nodes.rs`、`crates/redpiler/src/backend/direct/update.rs`）

**用户原话（2026-10-26）**

> 先记着，红石粉会导致一大堆问题不兼容，具体情况以后再说，记录bug19

**当前已知**

- 用户预期红石粉在相当多场景下与上游 / 原版行为不兼容，但具体现象、复现步骤、影响范围**尚未提供**。
- 与之相关的独立条目：**BUG-018**（暴露点状红石粉 0–15 强度）——那条是功能需求，本条是兼容性缺陷，不要混在一起。
- 本条暂时**不做任何分析或猜测**：等用户给出具体现象再说。

**待补充**

1. 具体现象（什么电路、预期什么、实际什么）
2. 是否与 BUG-007（`coalesce`）、BUG-008（信号强度分析）同源
3. 是否涉及 `-d` / `-l` / `-c` 这几个 wire 相关编译开关

**备注**

- 编号 019，接在 BUG-018 之后。
- 本条只登记，不实施；分析要等用户提供现象后再做。

### BUG-020　自动化协议重写（端口定义改由 `io.txt` 声明）

**状态**：📝 **整体重写，目前只定稿“十六进制序列规则”；协议关键字与 `OK` / `ERR` 格式全部待重定**（未授权写代码）
**登记日期**：2026-10-26　**更新**：2026-10-27（并入原 BUG-024 的 `io.txt` 设计）
**目标版本**：待定（大概率是协议版本大跳，需用户确认）
**影响文件**：`crates/core/src/automation.rs`、`crates/core/src/plot/mod.rs`（数据端口与采样）、`crates/core/src/server.rs`（路由）、新增 `io.txt` 加载器、`src/protocol.txt` / `src/protocol_zh.txt`，并同步 `MCHPRS_IO/test.py`

> ### ⚠ 重写范围（2026-10-27 用户明确）
>
> **重写就是全部重写**：不只是端口定义方式，连 `INPUT` / `OUTPUT` / `READ` / `USE` / `SET` / `SUB` / `UNSUB` / `EVENT` 这些**关键字本身**、以及 `OK` / `ERR` **响应格式**，全部重新设计。
>
> **目前只确定了一件事：十六进制序列的规则**（整字前端补零 / `matrix` 逐行补零 / 位数不符严格报错）。
>
> 下面规则表中标注「作废」的条目，是早期按“只改端口定义、其余保持原样”记下的，现已失效。

**用户原话**

> 2026-10-26：还有，目前的自动化协议我要重写，先记着，bug20，具体情况以后再说
>
> 2026-10-27：新增 io.txt 文件，你应该能看见，在 mchprs.sh 指向的文件夹

---

## 设计核心：端口定义从协议移到 `io.txt`

**现状**：脚本用 `PIN NAME EN_CLK CX CY CZ X0 Y0 Z0 NX NY NZ DX DY DZ` 现场定义一片规则点阵，坐标散落在脚本里。

**之后**：端口组在 `io.txt` 里声明（逐点坐标 / BUG-022 的步进展开 / `matrix`），加载后**自动可用**，脚本直接用组名访问。

`io.txt` 位置：服务端工作目录根下，与 `Config.toml`、`memory/` 同级（`/storage/emulated/0/termux/MCHPRS_IO/io.txt`）。

**用户给的格式草图（逐字保留）**

```
input name = [
origin = 
clk =
width =
……
……
]

input name = [
origin =
clk = 
width =
matrix: ……
]

output name = [

]

input name = [

]
```

**结构**

- 文件由若干**具名块**组成，块头两种：`input name = [` 与 `output name = [`
- 每个块内**必须**有 `origin`、`clk`、`width` 三个键
- 块内端口既可以逐行写坐标（`……`），也可以用 BUG-022 的 `matrix:` 一行写法
- 块内可以为空（草图中两个空块）

**已确认的规则**（用户 2026-10-26 / 2026-10-27 答复）

| # | 项 | 结论 |
| --- | --- | --- |
| 1 | 与 memory 文件的关系 | **没有任何关系**。`io.txt` 不参与 `.rom.txt` / `.ram.txt` 的端口定义，两者互不引用 |
| 2 | `input` / `output` 的方向 | **`input` ↔ 输入数据端口**（拉杆 / 压力板，脚本写入电路）；**`output` ↔ 输出数据端口**（灯 / 活板门 / 音符盒，脚本读出） |
| 3 | 组内 `origin` / `clk` / `width` | **三个都必填**；没有时钟就写 `clk = 0` |
| 4 | 组名唯一性 | **全局唯一**，重名报错 |
| 5 | `io.txt` 加载时机 | **启动时加载**，并提供**重载命令**（类似 `/memory reload`） |
| 6 | 组内 `width` 含义 | **该组端口个数（位宽）**，与 BUG-023 同义：校验块内展开出的坐标数 |
| 7 | 组内 `clk` 含义 | 沿用**现有协议的时钟语义**（原 `EN_CLK + CX CY CZ`）。`clk = 0` 表示没有时钟元件；无时钟端口在任何点阵位变化时触发，有时钟端口仅在时钟元件上升沿触发 |
| 8 | 是否需要注册 | `io.txt` 里的组**加载后自动可用**，脚本**不再发注册指令** |
| 9 | `PIN` / `POUT` 指令 | **完全废弃**（整条指令删除，不是改成只认组名） |
| 10 | 旧的带坐标写法 | **废止**，不再支持 `NAME EN_CLK CX CY CZ X0 Y0 Z0 NX NY NZ DX DY DZ` 这类参数 |
| 11 | 时钟元件位置 | **不再受“必须在点阵包围盒之外”的限制**，时钟可以放在任意位置 |
| 12 | 时钟与端口坐标重合 | **报错**：时钟可以放在任意位置，但不允许与任何端口坐标撞位 |
| 13 | ~~协议其余部分~~ | ❌ **作废（2026-10-27）**：原记“除端口定义外与当前 TCP 协议保持一致”，实际是**全部重写** |
| 14 | ~~`INPUT` / `OUTPUT` 指令语法~~ | ❌ **作废（2026-10-27）**：这两个关键字**本身也要重写**，不再沿用现语法 |
| 15 | 重载时已有连接 | **断开所有自动化连接**，强制脚本重连后使用新定义 |
| 16 | 与 BUG-018（红石粉 0–15 强度）的关系 | **本次不纳入**，强度仍由 BUG-018 单独立项实施 |
| 17 | `io.txt` 是否支持 `matrix` 关键字 | **支持**，并**遵守 BUG-022 的全部 matrix 规则**：块内只能一行、1～3 段、行优先（第一段最快）、逐行按 4 对齐补零 |
| 18 | `INPUT` / `OUTPUT` 的数据格式 | **按 matrix 格式处理**：使用了 `matrix` 的组，其数据在线上使用的 hex 采用**逐行补零**后的位数（`ceil(第一段数量/4) × 剩余段数量之积`），而不是 `ceil(width/4)` |
| 19 | **非 `matrix` 组**的 `INPUT` / `OUTPUT` 数据 | 仍按 `ceil(width/4)` 个 hex 数字处理，**维持 BUG-016 现有行为** |
| 20 | `matrix` 组的 `width` 取值 | **展开后的有效端口数**（逐行补的零不计入），与 BUG-023 规则 6 一致；hex 位数另外按逐行补零计算 |
| 21 | 端口点数上限 | **取消**：不再沿用旧协议的“单个端口最多 16000 点”，只受每段数量 ≤ 1024 与坐标范围约束 |
| 22 | 空组是否允许 | **不允许**：`input` / `output` 组内至少要有 1 个端口，空方块括号报错（草图里的空块只是占位示意） |
| 23 | 校验时机 | **语法与重名在加载期查**（`width` 一致性、`matrix` 格式、组名重复）；**元件类型与坐标是否存在，等 `/redpiler compile` 之后校验**，错误集中列出 |
| 24 | 时钟唯一性范围 | **全局唯一，包含 `io.txt` 的组**：io 组的 `clk` 不得与任何 `.rom.txt` / `.ram.txt` 的 `clk` 相同，也不得与其它 io 组相同，撞了双方都拒载 |
| 25 | `INPUT` / `OUTPUT` 的 hex 长度不符 | **严格报错**（与现有协议 `parse_hex_bits` 一致），不自动补齐、不截断；仅 `matrix` 组按逐行补零后的位数校验 |
| 26 | `matrix` 组 hex 里的**补零位** | 必须是 0；发现 1 即**报错**（该位不对应任何端口，出现 1 说明脚本位序算错了） |
| 27 | `matrix` 组的时钟脉冲 | **沿用现有语义，不新增规则**：先把全部位写完，再给时钟元件打一次脉冲；与位序、是否 `matrix` 无关 |

**`io.txt` 各键与现有协议参数的对应**

| `io.txt` | 现有参数 | 说明 |
| --- | --- | --- |
| `origin` | `X0 Y0 Z0` | 原点 |
| `clk` | `EN_CLK CX CY CZ` | `clk = 0` ↔ `EN_CLK 0` |
| `width` | 由 `NX NY NZ` 推出的总点数 | 组内端口个数 |
| 块内坐标行 / `matrix:` | 由 `NX NY NZ DX DY DZ` 描述的规则点阵 | 新写法逐点 / 步进 / 矩阵展开，比规则点阵更灵活 |
| `input name` | （原 `PIN` 的职责） | 输入数据端口 |
| `output name` | （原 `POUT` 的职责） | 输出数据端口 |

---

**现有协议现状（仅作对照，不代表保留）**

- 明文 TCP，默认 25585，按行处理
- 动词：`PIN` / `POUT`（**将废弃**）、`INPUT` / `OUTPUT` / `READ` / `USE` / `SET` / `SUB` / `UNSUB` / `EVENT`（保留），以及 `/` 开头的命令通道
- `PIN`/`POUT` 是按二进制位定义的数据端口；hex 在 0.4.0-beta 改成“整条位向量当一个整数、前端补零”（BUG-016）
- 只有六种元件可访问：按钮 / 拉杆 / 踩踏板 / 灯 / 活板门 / 音符盒；`power`（0–15）不对外暴露
- ~~单个端口最多 16000 点~~ **已于 2026-10-27 取消该上限（见规则 21）**；点与时钟仍须位于同一 plot

**实现备注**

- 时钟位置限制解除后，需在加载期校验“时钟不与任何端口坐标重合”
- 组自动可用意味着**加载期**就要完成全部端口校验（元件类型、坐标存在性、`width` 一致性等），错误集中报出
- 重载命令需处理“已有连接正在使用旧组”的情况（是否断开、是否保留旧定义到连接结束）
- 时钟位置自由后，`input` 组的时钟脉冲（模拟右键）仍按现有语义执行

**实现备注（续）**

- 重载时**断开所有自动化连接**：需要先把待发送的 `OUTPUT` / `EVENT` 冲出去再断开，避免脚本读到半截数据
- `INPUT` / `OUTPUT` 语法不变意味着 `test.py` 与 `io_element.py` 只需要把坐标参数换成组名，改动面比预期小
- **`matrix` 组改变了线上 hex 的长度**：脚本按 `width` 生成的 hex 位数与线上实际要求的位数会不一致，`test.py` 与 `io_element.py` 必须跟着改；错误码需明确是“位数不符”还是“位序不符”
- `matrix` 的逐行补零在 memory 文件与 `io.txt` 里是**同一套规则**，应共用同一份编解码实现

**备注**

- 本条吸收原 **BUG-024**（`io.txt` 具名 IO 组定义）；该编号已于 2026-10-27 回收并重新分配给「告示牌朝向」缺陷
- 与 BUG-022（步进 / `matrix` 展开）、BUG-023（`width`）在语法形态上相关，但语义无关（那两条只管 memory 文件）
- 本条只登记规则，不实施；重写会牵动协议文档、`test.py`、`io_element.py` 等周边
- 台账状态：**规则已全部定稿（27 条），未写代码**

### BUG-021　可写存储器（`.ram.txt`）

**状态**：✅ **已实施**（0.4.1-beta，✅ release 编译通过，待运行验证）
**登记日期**：2026-10-26
**目标版本**：待定（用户 2026-10-26 曾说 `.ram.txt`"
先记着""，现在开始定需求）
**影响文件（预判）**：`crates/core/src/memory.rs`（解析 + 读写循环）、`README*`、`protocol*`

**用户原话（2026-10-26）**

> 现在要实现.ram.txt
> 输入端口有时钟，地址，写数据线写使能，读使能，字节写使能（这个具体规则我一会再说）输入只能是红石的O端口
> 输出端口只有读数据线，只能是红石I端口

**需求（已知）**

| 端口 | 方向 | 元件类型 | 说明 |
| --- | --- | --- | --- |
| 时钟 `clk` | 服务器读 | O 端口（灯/活板门/音符盒） | 触发时机（上升沿？待确认） |
| 地址 `address` | 服务器读 | O 端口 | 位宽待定 |
| 写数据 `write data` | 服务器读 | O 端口 | 位宽 = 存储字宽 |
| 写使能 `WE` | 服务器读 | O 端口 | 规则待确认 |
| 读使能 `RE` | 服务器读 | O 端口 | 规则待确认 |
| 字节写使能 | 服务器读 | O 端口 | **具体规则用户"一会再说"** |
| 读数据 `read data` | 服务器写 | I 端口（拉杆/踩踏板） | 位宽 = 存储字宽 |

**我的理解**

- 与 `.rom.txt` 相反：ROM 的 address/clk 走"O 端口、服务器读"，data 走"I 端口、服务器写"；RAM 把"写数据 / WE / RE / 字节写使能"也放进读侧（电路驱动、服务器采样），把"读数据"放进写侧（服务器驱动拉杆）。
- 因此 RAM 需要**两组数据线**：写数据（灯）与读数据（拉杆），坐标必须分开。

**已确认（用户 2026-10-26 第一轮选择）**

| # | 项 | 结论 |
| --- | --- | --- |
| 1 | 时序 | **全同步**：时钟上升沿那一 tick 采样 address / write data / WE / RE / 字节写使能，写入与输出都在 tick 后，固定 1 tick 延迟 |
| 2 | 持久化 | **需要持久化**（具体方案待定，见下） |
| 3 | 同时读写 | **读优先**：同一沿同一地址 WE 与 RE 都有效时，读数据输出旧值 |
| 4 | 校验 | **全部校验**：读写数据线不得重叠、读数据拉杆不得有入边、时钟全局唯一、元件类型正确 |
| 5 | 容量上限 | **不设业务上限**（但地址位宽受 `usize` 限制，见备注） |

**已确认（用户 2026-10-26 第二轮选择）**

| # | 项 | 结论 |
| --- | --- | --- |
| 6 | 持久化位置 | **写回同一个 `.ram.txt`**（一个文件既是初值也是当前值） |
| 7 | 落盘时机 | ~~定时/批量：每 2 秒或每 4096 次写入落一次~~ **2026-10-27 改为仅事件驱动**（含规则 7b 的玩家退出）：只在**退出服务器**、**`/redpiler reset`**、**编译崩溃退出**、**`/memory reload` 之前** 四个时机落盘；**运行期不再自动写盘**，避免 tick 突发卡顿 |
| 7b | 玩家退出服务器时落盘（2026-10-27 用户新增） | 追加一个落盘时机：**玩家退出服务器时回写 RAM**。现状：p0,0 标记 `always_running`（BUG-010），地皮永不卸载，因此 `Drop` 的兜底落盘在运行期永远不会触发；玩家走光后 RAM 内容一直只存在内存里。**已确认（2026-10-27）：任意一个玩家退出即落盘**——用户认为该时机正好取代被取消的定时/计数回写机制 |
| 7a | 退出信号覆盖范围 | `ctrlc` 已启用 `termination` feature（`crates/core/Cargo.toml:35`），因此 **SIGINT / SIGTERM / SIGHUP 都会走 `Message::Shutdown` 并落盘**；只有 **SIGKILL / 断电 / OOM 强杀**无法兜底（任何程序都抓不住） |
| 8 | 写回格式 | **稀疏**，只写"有值/写过"的地址（`地址: word` 行） |
| 9 | 容量保护 | **不设保护**（写满存储也认） |

**持久化方案的连带问题（待确认）**

1. **文件会被程序改写**：用户手写的注释、排版、配置区会被覆盖吗？建议"保留头部（首行+配置区+注释），只替换数据区"。
2. **原子写入**：必须先写临时文件再 `rename`，否则崩溃时留下半截文件，下次启动直接拒载。
3. **版本迁移**：首行严格匹配当前内核版本；程序升级后旧 `.ram.txt` 会被拒载。写回时要不要顺手把首行更新成当前版本？
4. **`/memory reload` 与未落盘改动**：reload 时内存里的脏数据是落盘后再读，还是直接丢弃？
5. **不设保护的风险**：脚本写满地址空间会让 HashMap 与文件无限增长，可能写满手机存储。

**已确认（用户 2026-10-26 第三轮选择）**

| # | 项 | 结论 |
| --- | --- | --- |
| 10 | 落盘格式 | **保留头部**：首行、配置区、注释原样保留，只重写 `地址: word` 数据区 |
| 11 | `/memory reload` | **先落盘再重读**，不丢内存里的脏数据 |
| 12 | 版本号 | 加载时**严格匹配、不匹配即拒载**；落盘时首行**写成当前内核版本**（用户 2026-10-26 确认） |

**文件格式（用户 2026-10-26 给出的样例，`test.ram.txt`）**

```text
v0.4.0-beta hex words addressed
origin = 0 //和rom一样
clk = 12, 37, 76 //时钟必须存在
write enable = 43, 46, 25
read enable = 81, 66, 48
byte write enable = [
26, 67, 94
26, 67, 97
]
//byte write enable = []
//没有字节写使能按照上面的格式
address coordinate = [
258, 10, 263
258, 8, 263
]
write data coordinate = [
256, 14, 264
256, 12, 264
256, 10, 264
256, 8, 264
]
read data coordinate = [
256, 14, 264
256, 12, 264
256, 10, 264
256, 8, 264
]
0: f b a 7
```

**从样例读出的规则**

- 首行与 `.rom.txt` 相同（`v<内核版本> hex words addressed`）→ RAM 初值也走 Logisim 格式
- 新增键：`write enable`、`read enable`、`byte write enable`、`write data coordinate`、`read data coordinate`
- `byte write enable` 是**列表**，允许为空 `[]`（表示没有字节写使能）
- `clk` **必须存在**（用户注释"时钟必须存在"），即 RAM 不支持无时钟模式
- 用户要求：**`.rom.txt` 的 `origin` 与 `clk` 也要换位置**（顺序改为 `origin` 在前、`clk` 在后）

**从样例看出的疑点（已向用户提问）**

1. ~~样例里读写数据坐标相同~~ → 用户 2026-10-26 确认：**笔误**，实际必须分成两组不同的坐标（与"全部校验"里的"读写线不得重叠"一致）。
2. ~~`byte write enable` 映射规则~~ → 用户 2026-10-26 给出：

   - **RAM 的时钟必须存在，同时管读和写**（`clk = 0` 直接报错）
   - 字节写使能数量 = `ceil(数据位宽 / 8)`
   - 每一组按**高到低**分：例如 15 位数据 → 共 2 组，最高组 = 高 7 位，次组 = 低 8 位
   - 使能列表**第一个元素对应最高组**（按顺序从高到低）
   - 用户提供的 `test.ram.txt` 里 2 条使能对 4 位数据是**错的**，以本条规则为准
3. ~~`origin` / `clk` 换位置~~ → 用户 2026-10-26 确认：**只统一示例与文档，解析器不强制顺序**。已执行：`example.rom.txt`、`README.md`、`README_zh.md` 的 ROM 示例改成 `origin` 在前、`clk` 在后。

**已确认（用户 2026-10-26 第四轮选择）**

| # | 项 | 结论 |
| --- | --- | --- |
| 13 | 空列表 | ~~`byte write enable = []` = 没有字节使能~~ **已于 BUG-023 废止**：`[]` 不再合法，必须写 `byte write enable = [ width = 0 ]` |
| 14 | 条数校验 | 必须严格等于 `ceil(数据位宽 / 8)`，否则报错并禁用该 RAM |
| 15 | 写粒度 | 使能 = 1 的组写入新值，使能 = 0 的组**保持原值** |
| 16 | 时钟 | RAM 的 `clk` **必须存在**，同时管读和写；`clk = 0` 直接报错 |
| 17 | 字节分组的位序 | 按**高到低**分组：使能列表第一个元素对应最高组；15 位数据 = 高 7 位 + 低 8 位两组 |

**字节写使能的位映射（用户 2026-10-26 确认）**

- 组数 = `ceil(N / 8)`，`N` = 数据位宽（`write data coordinate` 的条目数）
- **使能列表第 1 项 = 最高的那一组**，从高到低排列
- **最后一个使能永远管低 8 位**
- 最高组可能不足 8 位；其余各组都是 8 位
- 使能[0] 对应的正是 `data coordinate` 的前 8 个坐标（列表第一个元素本来就代表最高位）

| N | 组数 | 使能[0] | 使能[1] | 使能[2] |
| --- | --- | --- | --- | --- |
| 4 | 1 | bit 3..0 | — | — |
| 8 | 1 | bit 7..0 | — | — |
| 15 | 2 | bit 14..8（7 位） | bit 7..0 | — |
| 16 | 2 | bit 15..8 | bit 7..0 | — |
| 20 | 3 | bit 19..16（4 位） | bit 15..8 | bit 7..0 |

通式：使能[i] 覆盖 `[N-1-8i, max(0, N-8(i+1))]`。

- `[]` 空列表 = 没有字节使能，`WE=1` 时整字写入
- 条数 ≠ `ceil(N/8)` → 报错并禁用该 RAM
- 读（`RE`）不受字节使能影响，永远输出整字

**拟定但未确认的两条（按此实现，若不对请纠正）**

- `write data coordinate` 与 `read data coordinate` 的位宽必须相等，即同一个字宽 N；不相等则报错。
- 时钟唯一性是全局的：`.rom.txt` 与 `.ram.txt` 的 `clk` 不得相同，撞了双方都拒载；**2026-10-27 起该约束扩展到 `io.txt` 的组（见 BUG-020 规则 24）**。

**待澄清（已向用户提问）**

1. 时钟是上升沿触发，还是电平有效？WE / RE / 字节写使能是在时钟沿采样，还是组合有效？
2. 同一 tick 同时读和写同一地址时，读数据输出旧值还是新值？
3. 字节写使能的位宽规则：字宽 N 不是 8 的倍数时，最后一组怎么算？
4. RAM 上电初值：全 0，还是 `.ram.txt` 里带 Logisim 初值？
5. **持久化**：RAM 内容是否需要写回文件或存档？重启 `/stop` 后是否保留？（当前地皮存档只存方块，不存内存内容）
6. 地址位宽上限（2^N 的内存占用）。
7. 文件头格式是否与 `.rom.txt` 完全一致（`v<kernel> hex words addressed`）。
8. 读数据拉杆是否要求"无入边"（与 BUG-017 同类问题：被外部驱动的拉杆写入会打架）。

**备注**

- 地址位宽"不设上限"存在实现上限：地址值用一个 `usize` 聚合，**地址线最多 63～64 根**，再宽就无法表示，代码里必须有这条硬上限。
- 持久化与"不设上限"组合有风险：脚本若把地址空间写满会爆内存/爆文件，需要至少一个字数或文件大小保护（待用户定）。
- 本条取代 BUG-015 里挂着的"`*.ram.txt` 语义待补充"。
- 与 BUG-018（红石粉强度）、BUG-019（红石粉兼容性）、BUG-020（协议重写）无直接耦合。



#### 回写格式修正（原 BUG-028，2026-10-27 并入）

> 本条原独立登记为 `BUG-028`，用户 2026-10-27 指出“不应单独列一条”，故并入 BUG-021（RAM 回写属于可写存储器的一部分）。**原编号停用**。

**状态**：✅ **已实施**（0.4.3-beta，✅ release 编译通过，⏳ 待运行验证）　·　**登记日期**：2026-10-27
**目标版本**：待定（0.4.2-beta 已发布的二进制含此问题）

**实施记录（0.4.3-beta，2026-10-27）**

- `render()` 重写为按 8 字块输出；`FLUSH_GROUP` 改名 `WORDS_PER_LINE`
- 行首地址位数取 `(self.memory.address.len() + 3) / 4`，最少 1 位
- 空格子走 `pack_word_hex(&vec![false; bits], matrix)`，天然定宽
- ✅ release 编译通过（0.4.3-beta），⏳ 待运行验证
**影响文件**：`crates/core/src/memory.rs`（`render()`）

**用户报告（2026-10-27）**

只写了几个字，`test.ram.txt` 回写出来是这样：

```
0: f 1 a a
b: c
e: 6
```

而 Logisim 的格式是（每行 8 个 word、地址按 8 对齐、缺的补 0）：

```
00000: 00000413 00051137 00010113 22c00093 000080e7 fea12823 fe812e23 40800413
00008: ff014183 003401b3 0001c183 ff114203 00440233 00024203 004181b3 ff214203
```

用户期望的回写结果：

```
0: f 1 a a 0 0 0 0
8: 0 0 0 0 c 0 0 6 0
```

**根因**

`render()` 只遍历**实际存在的地址**，把连续且在 8 个以内的地址拼成一行：

```rust
while end + 1 < addresses.len()
    && addresses[end + 1] == addresses[end] + 1
    && end + 1 - index < FLUSH_GROUP
```

因此：

- 行首地址不按 8 对齐，而是从“第一个存在的地址”开始
- 行内不存在的地址**直接跳过**，不写 0
- 结果人类与 Logisim 都读不出各字的原始地址意图（虽然按格式语义读取时仍是等价的：未列出的地址读作 0）

**影响范围**

- 功能上读取等价（未列出地址读 0），但**与 Logisim 的工具链不兼容**，也不好人工核对
- 只影响 RAM 回写；ROM 是只读的，不会被改写

**已确认的规则**（用户 2026-10-27 答复）

| # | 项 | 结论 |
| --- | --- | --- |
| 1 | 每行 word 数 | **固定 8 个**，不足的用 0 补齐（用户示例第二行的 9 个是笔误） |
| 2 | 行首地址 | **按 8 对齐**（0、8、10、18…）；因空块被跳过，地址会出现跳跃，例如 `08` 直接跳到 `30`（用户原话：“行内补齐8个数据，地址从08跳到30都可以”） |
| 3 | 全 0 块 | **省略**：只有含至少一个已存字的块才写出；被跳过的地址读作 0 |
| 4 | 空字写法 | **按位宽定宽补零**，即每个字总是 `ceil(bits/4)` 个十六进制数字（4 位字写 `0`，8 位字写 `00`，32 位字写 `00000000`）；`matrix` 字按逐行补零后的位数 |
| 5 | 行首地址的位数 | **由 `address coordinate` 列表里声明的 `width` 决定**：写成 `ceil(width/4)` 个十六进制位。用户 2026-10-27 原话“按照地址位宽决定”“地址位宽就显式写在 ram.txt 里”。4 位地址 → 1 位（`0:` / `8:`）；6 位 → 2 位（`08:` / `30:`）；8 位 → 2 位；12 位 → 3 位 |
| 6 | 显式写入的 0 | **算“已存字”**：往某地址显式写过 0，该块仍要写出（该字按位宽写成 `0` / `00` …），只有**从未写过**的块才省略（用户 2026-10-27 确认） |

**备注**

- 只改 `render()` 的排版，**不影响落盘时机**（落盘时机见 BUG-021 规则 7 / 7a）
- 与 BUG-022（matrix 逐行补零）不同：那条管**单个 word 内部**的位对齐，本条管**行内多个 word 的排布**
- **实现要点（2026-10-27 已核对）**：`Memory.address` 存的是展开后的点列表，BUG-023 的校验保证其长度等于声明的 `width`，所以行首地址位数可以直接取 `(memory.address.len() + 3) / 4`，**不需要新增字段**
- 空字的定宽补零可直接复用 `pack_word_hex(&vec![false; memory.bits], memory.matrix)`，它本来就输出满位数（含 `matrix` 逐行补零）
- 数据段里的注释在回写时会丢失（`header` 只保留第一条数据行之前的内容）—— 这是 0.4.1 起的既有行为，本次不改

### BUG-022　存储器端口坐标支持“步进展开”与 `matrix` 矩阵展开

**需求**（2026-10-26 用户提出）

原来端口列表**一行一个坐标 = 一位**，8 位总线要写 8 行。改为支持两种增量写法：单行步进展开、以及按 hex 对齐的 `matrix` 矩阵展开。

---

#### A. 步进展开（一行多个坐标）

一行 5 段：`x, y, z, 步长, 数量`

- 前 3 段是**起点坐标**
- 第 4 段是**单轴步长**，形如 `<符号><整数><轴>`，例如 `+2x`、`-3y`
- 第 5 段是**展开总数，包含起点**

```
256, 12, 264, +2x, 4
```

展开为 `(256,12,264)` `(258,12,264)` `(260,12,264)` `(262,12,264)`。

---
#### B. `matrix` 矩阵展开（按 hex 数据对齐）

关键字 `matrix` + 冒号开头，后接起点与 **1～3 段** `步长, 数量`：

```
matrix: 256, 12, 264, +2x, 8
matrix: 0, 0, 0, +1x, 3, +1y, 5
matrix: 0, 0, 0, +1x, 3, +1y, 4, +1z, 5
```

**写法约束**

- 写了 `matrix` 的列表**只能有一行**，多写一行直接报错
- 一段到三段，最多三个方向
- **行优先：第一段变化最快**；三段时第二段比第三段快
- 关键字**不区分大小写**，冒号后空格可有可无

**hex 对齐的补零规则（本条的核心）**

有效位宽 = 各段数量之积。但写进文件的 hex 要按 **4 位一组**切，所以**每一行各自在行首补零**，补到 4 的倍数：

- 每行补零位数 = `(4 - 第一段数量 % 4) % 4`
- 每行占 `ceil(第一段数量 / 4)` 个 hex 数字
- 每个字的 hex 数字个数 = `ceil(第一段数量 / 4) × 第二段数量 × 第三段数量`
- 总 hex 位宽 ≥ 有效位宽，多出来的全是补的零，**不代表任何坐标**

**完整示例**：`matrix: 0, 0, 0, +1x, 3, +1y, 5`

- 有效位 = 3 × 5 = **15**
- 每行 3 个位置 → 行首补 1 个 0 → 每行 4 bit = 1 个 hex 数字
- 共 5 行 → **5 个 hex 数字 = 20 bit**
- 全部点亮写成 `77777`（多出的 5 个 bit 全是补的零）

| hex | 二进制 | 补的位 | 有效位 → 坐标 |
| --- | --- | --- | --- |
| 第 1 个 `7` | `0` `111` | 最高位 0 | (0,0,0) (1,0,0) (2,0,0) |
| 第 2 个 `7` | `0` `111` | 最高位 0 | (0,1,0) (1,1,0) (2,1,0) |
| … | | | 依次到 y=4 |

**与普通存储器的差别**：普通存储器只在整字前面补零一次（`ceil(bits/4)` 个数字）；`matrix` 是**逐行补零**，所以 hex 数字个数比 `ceil(bits/4)` 多。

**hex 数字个数不符时的处理**（与现有 `parse_word()` 右对齐行为一致）

- 写得比期望**少**：在前面补**整数字**的 0。例：该写 5 个数字而只写了 `7FFF` → 按 `07FFF` 处理
- 写得比期望**多**：从**前端截断**，保留低位数字。例：写了 6 个 → 保留最后 5 个

---
#### 已确认的规则总表（用户 2026-10-26 分三轮答复）

| # | 问题 | 结论 |
| --- | --- | --- |
| 1 | 符号位能否省略 | **不能省**，必须写 `+` 或 `-`；轴名**大小写均可**（`x` / `X` 等价） |
| 2 | 多轴 / 对角线写法 | **不支持**，每段只能是一个轴上的步进 |
| 3 | 一行内多个步进段 | 普通写法**不支持**；要矩阵式展开必须用 `matrix:` 前缀 |
| 4 | 展开后越界（x / z 超出 −256…255、y 超出世界高度） | **解析期不报错**，交给编译期“端口不存在”报错 |
| 5 | 展开后两个 bit 落在同一坐标 | **算错**，报错并禁用该存储器 |
| 6 | 数量写 `1` | 等同旧写法，只有起点，合法 |
| 7 | 数量写 `0` | **报错**（带行号） |
| 8 | 单段数量上限 | **1024**，超出报错 |
| 9 | 步长数值范围 | **±255 以内**，超出报错 |
| 10 | 步长写 0（如 `+0x`） | **一律报错** |
| 11 | 数量段写法 | **必须纯数字**（`4`），不接受 `+4` |
| 12 | 重复检测范围 | **同一存储器内所有列表互相查**：`address` / `data` / `write data` / `read data` / `byte write enable` 任意两个 bit 撞同一坐标即报错 |
| 13 | 重复时报错详细度 | **行号 + 展开序号 + 坐标**，例 `line 3: data coordinate: element 4 of 5 expands to 256, 3, 264, which duplicates line 5` |
| 14 | 旧 3 段写法 | **必须继续兼容**，本条属增量语法 |
| 15 | `matrix` 适用范围 | **所有端口列表都能用**（ROM 与 RAM 全部键） |
| 16 | 非 hex 数据的列表用 `matrix`（如 `address coordinate`、`byte write enable`） | **不做任何补零**，就是纯坐标展开 |
| 17 | `matrix` 关键字 | **不区分大小写**，冒号后空格可有可无 |
| 18 | 第一段数量必须 4 的倍数吗 | **不要求**，只是“数据按 4 的倍数切分处理”，逐行补零即可 |
| 19 | `matrix` 段数 | **1～3 段都可**；但无论几段，`matrix` 列表**只能有一行**（只写 1 段也照样只能一行，且逐行补零规则照常生效） |
| 20 | 展开顺序 | **行优先，第一段变化最快** |
| 21 | `matrix` 总点数上限 | **只限每段（1…1024）**，不额外限制乘积 |
| 22 | hex 数字个数不符 | 少 → 前端补整数字 0；多 → 前端截断保留低位（与现有 `parse_word()` 一致） |
| 23 | RAM 回写格式 | 按同一套逐行补零规则写回，保证读写往返一致 |
| 24 | `matrix` 字组的**补零位**写了 1 | **忽略，按 0 处理，不报错**（手写文件从宽）。注意这与 `io.txt` 的 `INPUT`/`OUTPUT` **有意不同**：那边在协议层，严格报错（BUG-020 规则 26） |

---
#### 实现要点（待实施）

- `parse_list()` 现在每行 `split(,)` 后要求恰好 3 段，需扩展为：
  - 3 段（旧写法）
  - 5 段（单段步进）
  - `matrix:` 前缀 + 3 + 2n 段（n = 1～3）
- 第 4 段正则 `[+-]<数字>[xyzXYZ]`，数值绝对值 ∈ 1…255
- 数量段纯数字整数 ∈ 1…1024
- 展开在 `origin` 偏移之前完成，展开后对**每个**坐标施加 `origin`
- 重复检测在展开完成后统一做，范围是整个存储器
- **新增一层 hex 编解码**：`matrix` 存储器需要“逐行去补零/加补零”，与现有 `parse_word()` / `pack_word_hex()` 的单次补齐逻辑不同，需单独处理
- 错误信息沿用现行风格：带**行号**和**键名**，重复类错误额外带展开序号

#### 实现备注

- 第一段数量正好是 4 的倍数时无补零，逐行结果与普通存储器一致，无需特殊分支
- `matrix` 只写 1 段时，等价于“一行展开成 N 个坐标”，但仍须按 4 对齐逐行补零（只有一行）

#### 备注

- 与 BUG-015（ROM 读取）、BUG-021（RAM）耦合——改的是两者**共用**的 `parse_list()` 与 hex 编解码
- 与 BUG-017（data 与 address 相邻校验）无直接关系
- 本条**不改**文件头格式
- **补零位的处理在两条线上有意不同**：memory 文件按 0 忽略（规则 24），`io.txt` / 协议层严格报错（BUG-020 规则 26）。不要试图统一，这是刻意分层
- 台账状态：✅ **已实施**（0.4.2-beta，release 编译通过，待运行验证）；逐条核对见 §7 的「代码与台账一致性审计」

### BUG-023　端口列表支持 `width` 声明，并校验展开后的坐标个数

**需求**（2026-10-26 用户提出）

> 添加 width 选项，检查写出来坐标的个数和预期的位宽是否一致

在端口列表的方括号内支持 `width = N`，声明该列表**预期**展开出多少个坐标；解析时校验实际数量，不一致即报错。

**用户提供的示例**

`test.rom.txt`：

```
address coordinate = [
width = 2
258, 10, 263
258, 8, 263
]
data coordinate = [
width = 11
256, 14, 264
256, 12, 264, +2x, 8
256, 10, 264
256, 8, 264
]
0: 157 7c4 3db 62c
```

`test.ram.txt`：

```
byte write enable = [
width = 1
26, 67, 94
]
byte write enable = [
width = 0
]
address coordinate = [
width = 2
258, 10, 263
258, 8, 263
]
write data coordinate = [
width = 4
…4 个坐标…
]
read data coordinate = [
width = 4
…4 个坐标…
]
```

**已确认的规则**（用户 2026-10-26 答复）

| # | 问题 | 结论 |
| --- | --- | --- |
| 1 | `width` 的位置 | **必须是列表第一项**，写在开方括号后的第一行，坐标从其下一行开始 |
| 2 | 不写 `width` 时 | **报错**——`width` 为**必填项**，所有端口列表都必须声明 |
| 3 | 实际坐标数 ≠ `width` | **报错并禁用该存储器**，错误信息带行号 |
| 4 | 示例里两个 `byte write enable` 块 | **两种写法任选其一**（有端口写坐标、无端口写 `width = 0`）；现行“重复键报错 `defined twice`”规则不变 |
| 5 | `width = 0` 的适用范围 | **仅 `byte write enable` 可用**，表示“没有字节使能”；其它列表写 0 报错 |
| 6 | 与 `matrix:` 联用时 `width` 的含义 | **展开后的有效坐标总数**（如 `+1x,3, +1y,5` 写 `width = 15`），逐行补的零**不计入** |
| 7 | `write data` 与 `read data` 的 `width` | **必须相等**，不等报错（与现行“两边坐标数必须相等”一致） |
| 8 | 旧的空列表 `[ ]` | **不再允许**，必须写成 `[ width = 0 ]`；空方括号直接报错 |
| 9 | 单点端口 `clk` / `origin` / `write enable` / `read enable` | **不需要也不能写** `width` |
| 10 | 用户现有 `memory/*.rom.txt` / `*.ram.txt` | 由**我在实现时**统一补上 `width` 并升级首行版本号 |


**兼容性影响（重要）**

- `width` 必填意味着**破坏性变更**：现有所有 `.rom.txt` / `.ram.txt` 都会因缺少 `width` 被判非法
- 用户现有的 `memory/test.rom.txt`、`memory/test.ram.txt` 需要手工补上 `width`（且它们的首行还是 `v0.4.0-beta`，也要一并升级）
- README / protocol 文档里的全部示例都要同步补 `width`

**实现要点（待实施）**

- `parse_list()` 需先识别方括号内第一行的 `width = N`，再解析其余坐标行
- `width` 与展开后的实际数量比对，不一致报错带行号
- `Memory::bits` 改由 `width` 声明值参与推导（数据列表的 `width` 即字宽）
- `matrix` 与普通步进两种写法都要参与同一套校验

**备注**

- 与 BUG-022（步进 / matrix 展开）强相关：`width` 校验的对象是**展开之后**的数量
- 台账状态：✅ **已实施**（0.4.2-beta，release 编译通过，待运行验证）；逐条核对见 §7 的「代码与台账一致性审计」

### BUG-025　报错出口统一：脚本只收 `OK` / `ERR`，详情走终端与玩家聊天框

**状态**：📝 **规则已定（4 条），未写代码**
**登记日期**：2026-10-27
**目标版本**：待定

**用户原话（2026-10-27）**

> 我的想法是自动化脚本尽量简单，接受一个 ERR，在终端和玩家聊天框里详细

---

## 现状审计（2026-10-27 逐处核对代码）

现有三条报错通道互不重叠：**终端日志**（`warn!` / `error!`）、**游戏内聊天**（`send_error_message` / `send_system_message`）、**自动化 TCP**（`ERR` / `WARN` 行）。

| 场景 | 终端 | 玩家聊天 | 脚本 |
| --- | --- | --- | --- |
| 存储器文件加载失败（启动时） | ✅ `warn!` | ❌ | ❌ |
| 存储器校验失败（编译后） | ✅ `warn!` | ⚠️ 仅玩家触发编译时 | ❌ |
| `/memory reload`（玩家发） | — | ✅ 汇总 + 最多 5 条红色细节 | — |
| `/memory reload`（脚本发） | ✅ | — | ❌ 完全无返回（`let _ =`） |
| 编译警告：比较器读空容器 | ❌ 终端不记 | ⚠️ 仅玩家触发编译时 | ❌ |
| 编译中止：空容器 / 无 override | ❌ 终端不记 | ⚠️ 仅玩家触发时 | ❌ |
| 编译 panic | ✅ `[CRASH]` + 退出 1 | ❌ | ❌ |
| 协议参数错（`ERR TYPE` / `CLK` / `FAILED` / `HEX`…） | ❌ | — | ✅ |
| 协议命令错（`ERR CMD …`） | ❌ | — | ✅ |
| 无效 / 畸形请求 | ❌ | — | ✅ |
| `SUB` 相关 `WARN INACTIVE` / `NO_IO` / `NOT_OUTPUT` / `FAILED` | ❌ | — | ✅ |
| schematic 存读失败 | ✅ `error!` | ❌ | ❌ |
| 玩家数据加载失败 | ✅ `warn!` | ❌ | — |
| 客户端版本不匹配 | ✅ `warn!` | ❌ | — |

## 已识别的四个缺口

1. **自动化触发编译时失败原因全静默**：`start_redpiler(options, None)` 的 `source` 为 `None`，编译警告与编译中止既不发给脚本，也不写终端（只有存储器校验会 `warn!`）
2. **脚本发 `/memory reload` 完全无反馈**：`commands.rs:291` 是 `let _ = self.reload_memories();`
3. **启动时存储器加载失败只有终端可见**，玩家进游戏后没有任何提示
4. **终端与协议两侧互不覆盖**：协议错误不写终端，编译错误不写终端

---

## 设计原则（用户 2026-10-27 定）

- **自动化脚本保持极简**：只接受成功（`OK`）或失败（`ERR`），不承载详细原因
- **详细情况写到两处**：**终端日志** 与 **玩家聊天框**

## 需要调整的方向

- 协议层不再返回细节，只回 `OK` / `ERR`
- 所有错误在**终端**至少记一条（含协议错误、编译错误）
- 所有错误在**玩家聊天框**可见（含自动化触发的操作）
- 自动化触发的编译 / 重载，结果要能反映到 `OK` / `ERR` 上（现状是 `OK` 无条件返回）

## 已确认的规则（用户 2026-10-27 答复）

| # | 项 | 结论 |
| --- | --- | --- |
| 1 | 自动化触发时详细错误的去向 | **广播给所有在线玩家**；无人在线则只写终端 |
| 2 | 协议响应格式 | **`OK` 就是裸 `OK`**，不附加任何内容；**`ERR` 必须带短码，且只带短码**（不带文字描述），细节只出现在终端与聊天框。原则上是**尽量每个错误都有短码** |
| 3 | 自动化编译失败 | **回 `ERR`**（成功回 `OK`）；不再无条件回 `OK` |
| 4 | `WARN` 系列 | **全部并入 `ERR`**，协议层只剩 `OK` / `ERR` 两种响应前缀，不再有 `WARN` 行 |

**实现备注**

- 现行私有的两条错误行 `ERR invalid request: <line>` 与 `ERR malformed request: <line>` **没有短码**，按“尽量都带短码”的原则需要各补一个（具体名字**暂不定**，随 BUG-020 的码表一起定）
- `WARN` 并入 `ERR` 改变了线上响应类型，属**协议破坏性变更**，应与 BUG-020 重写同批落地
- **BUG-020 目前只确定了 hex 序列规则，`OK` / `ERR` 的格式本身也在重写范围内**；本条的规则 2、3、4 是**方向性约束**（裸 `OK`、`ERR` 只带短码、`WARN` 并入 `ERR`），具体表达随 BUG-020 定
- 本条不提前推演具体错误码清单
- `send_error_message` / `send_system_message` 已经是玩家聊天通道，广播给所有玩家时要用现行 `broadcast_chat_message` 类接口，避免逐玩家重复拼接

**已知文案问题（2026-10-27 审计发现，待随报错统一一起改）**

- BUG-023 规则 5 要求“`width = 0` 仅 `byte write enable` 可用”，当前实现是通过“其它列表的 `width` 必须大于 0”间接拦下，提示语为 `width must be greater than 0`，**没有专门的“仅字节使能可写 0”文案**。行为正确，文案待改。

**备注**

- 本条目是**跨切面改进**，影响 `automation.rs`、`plot/mod.rs`、`plot/commands.rs`、`memory.rs`、`server.rs`
- 与 BUG-020（协议重写）有交集：若协议层要改成“只回 ERR”，应在重写时一并落地，避免改两遍
- 台账状态：**规则已定（4 条），未写代码**

### BUG-026　存储器性能与容量风险

**状态**：✅ **0.4.2 已实施其中两项**（「只写变化的位」+「去掉总线 clone」，release 编译通过）；容量保护 / 按位写节点 / 落盘阻塞 tick 等余项维持不做
**登记日期**：2026-10-27
**目标版本**：待定
**影响文件**：`crates/core/src/memory.rs`（`run_rom` / `run_ram` / `save` / `render` / `flush_due`）、`crates/core/src/plot/mod.rs`（`run_memories`）

**用户提问（2026-10-27）**

> rom 和 ram 的响应时间问题，数据量过大会不会导致响应时间过长？

---

## 结论速览

| 维度 | 是否随数据量变差 |
| --- | --- |
| **延迟**（地址变化 → 数据输出） | ❌ **不会**，恒定 1 tick，与字数、位宽无关 |
| **每 tick 工作量** | ⚠️ 与**数据位宽**线性相关 |
| **周期落盘** | ⚠️ 与**字总数**线性相关，最危险 |
| **内存占用** | ⚠️ 无上限，可被脚本写爆 |

延迟恒定是因为存储器统一在 `tickn()` 之后采样（`run_memories`），与数据规模无关。

---

## 四个热点（均已定位到代码）

### 热点 1：每 tick 无条件采样地址总线

`run_rom` 每 tick 都执行 `read_address(redpiler, &self.memory.address)`，每个地址位一次 `query_io`（HashMap 查找）。63 位地址 = 63 次/tick。**量级很小，一般可忽略。**

### 热点 2：RAM 每个时钟边沿整条 clone 总线

`run_ram` 在边沿处把 `address` / `write_data` / `read_data` / `byte_enable` 四条总线**各复制一份**（为了绕开 `self.memory.ports` 与 `self.memory.words` 的借用冲突）。

- 代价：每 tick 4 次堆分配，复制量 ≈ (地址位 + 写位 + 读位 + 字节使能数) × 12 字节
- 例：63 + 1024 + 1024 + 128 ≈ **27 KB / tick** 的分配与释放
- **可完全避免**：把 `Memory` 拆字段分别借用（`let Memory { ports, words, .. } = &mut self.memory;`）即可，无需复制

### 热点 3：每次触发按位宽写节点

`try_set_powered` 每位一次，内部 `set_node` 还会遍历该节点**全部前向链接**做红石传播。

- 1024 位数据总线 = **一次触发 1024 次节点更新 + 各自扇出**
- 这是位宽影响性能的最直接路径

### 热点 4：落盘同步在 tick 线程上（**已于 2026-10-27 降级**）

`save()` → `render()`：排序全部地址、每字转 hex、拼出**整份文件文本**，再 `fs::write` + `rename`，全程跑在 tick 线程。

- 原触发条件 `FLUSH_INTERVAL = 2s` / `FLUSH_WRITES = 4096` 次写 —— **用户 2026-10-27 决定取消运行期定时落盘**
- 改为**仅事件驱动**：退出服务器、`/redpiler reset`、编译崩溃退出、`/memory reload` 之前
- 这些时机都是一次性事件（加载/退出/重置），短暂停顿可以接受；**运行期不再有任何磁盘写**

| 数据量 | 每次落盘写出 |
| --- | --- |
| 4K 字 × 64 位 | ~200 KB，影响小 |
| 100K 字 × 64 位 | ~5 MB，明显停顿 |
| 1M 字 | ~50 MB，tick 卡住数秒 |

### 热点 5：内存占用无上限

`Vec<bool>` 是位压缩的（1 位 ≈ 1/8 字节），但：

- 1M 字 × 1024 位 ≈ 128 MB 位图，加 `HashMap` 条目与 `Vec` 头部约 200 MB
- 当前**没有任何容量保护**（用户此前明确选择“不设保护”）

---

## 用户已选优先级（2026-10-27）

### 六项最终处置（2026-10-27 全部拍板）

| 项 | 处置 |
| --- | --- |
| **取消运行期定时落盘** | ✅ **已决议**，并入「BUG-021 落盘改动」，改为四事件时机落盘（退出 / `/redpiler reset` / 崩溃 / `/memory reload` 前） |
| **只写变化的位** | ✅ **0.4.2 实施**：位值未变则不调 `set_node`，减少无效传播 |
| **去掉每 tick 的总线 clone** | ✅ **0.4.2 实施**：拆字段借用（`let Memory { ports, words, .. } = &mut self.memory;`），去掉每 tick 约 4 次数组复制；纯收益、不改行为 |
| **落盘移出 tick 线程** | ❌ **正式关闭**：定时落盘已取消，只剩一次性事件，不再需要 |
| **强杀保护（SIGTERM / SIGHUP）** | ❌ **正式关闭**：已由 `ctrlc` 的 `termination` feature 覆盖（2026-10-27 核实），只有 SIGKILL 无法兜底 |
| **补容量保护** | ❌ **不做**：沿用用户此前“不设保护”的决定，写入量由使用者自己负责 |

## 实现思路（仅记录，未实施）

- **落盘**：`save()` 改为把 `render()` 的结果交给后台线程写盘，tick 线程只做 `try_send`；或改为“标记待写 + 在 tick 间隙写”
- **只写变化的位**：`run_rom` / `run_ram` 写出前先 `query_io` 取当前值，相同则跳过
- **去 clone**：拆字段借用，无需复制

**备注**

- 本条只做分析登记，**未跑任何基准测试**，上表数字是按位宽与字数推算的量级
- 若将来补容量保护，需与 BUG-021「RAM 持久化」的写入策略一起考虑
- 台账状态：✅ 0.4.2 已实施其中两项（只写变化的位 / 去掉总线 clone）

### BUG-027　`/radvance` 与批量 tick 跳过存储器 / IO 采样

**状态**：✅ **已实施**（0.4.2-beta，release 编译通过，待运行验证）；`automation_per_tick_push` 与无限制 TPS 的互斥关系维持原样
**登记日期**：2026-10-27
**目标版本**：待定
**影响文件**：`crates/core/src/plot/commands.rs`（`radv` 两处入口）、`crates/core/src/plot/mod.rs`（`tickn` / `update` 批量路径）

**用户提问（2026-10-27）**

> radv 指令会导致丢数据吗

---

## 结论

`/radvance` 的实现只有两步，**不落盘、也不改内存里的字**：

```rust
self.tickn(u64::from(ticks));
self.publish_world();
```

因此：

| 问题 | 答案 |
| --- | --- |
| 会丢磁盘文件里的数据吗 | ❌ **不会** |
| 会丢 RAM 内存里的内容吗 | ❌ **不会** |
| 会丢 RAM 的**写入**吗 | ✅ **会丢** |
| 会丢 `EVENT` 吗 | ❌ 不丢，但只报最终状态 |
| 会丢 `OUTPUT` 的中间状态吗 | ✅ **会丢** |

## 三条通道的采集方式不同

| 通道 | 采集方式 | `radv` 期间 |
| --- | --- | --- |
| RAM 写入 | 主动采样时钟边沿 | ❌ 边沿全丢，写不会发生 |
| `EVENT` | `flush()` 时遍历 `changed` 节点收集并清标志 | ✅ 不丢（`publish_world()` 内部 flush + drain），但每个节点只报**最终值** |
| `OUTPUT` | `sample_pout_ports()` 主动轮询 | ❌ 中间状态全丢，下次正常 update 只补最终值 |

## 代码依据

- `tickn()` 只跑红石 tick，**没有 `run_memories()`**：

```rust
fn tickn(&mut self, ticks: u64) {
    if self.redpiler.is_active() {
        self.redpiler.tickn(ticks);
        return;              // ← 存储器一次都不采样
    }
    ...
}
```

- `publish_world()` **会** flush + drain 事件，所以 `EVENT` 不丢：

```rust
self.redpiler.flush(&mut self.world);
let events = self.redpiler.take_io_events();
```

- 事件在 `DirectBackend::flush()` 中按 `node.changed` 推送，每个节点每个 flush 周期只报一次最终值——**中间折返不可见**。

## 副作用：假边沿

`prev_clk` / `prev_address` 停在 `radv` 之前的值。若 `radv` 结束时时钟为高、之前为低，`radv` 之后第一个正常 tick 会**误判一次上升沿**，补做一次读写。净效果是多个边沿被压缩成最多一次。

## 批量路径同样存在（不限于无限制 TPS）

`update()` 的批量分支是：

```rust
self.tickn(batch_size as u64);   // 一次推进整批
...
self.sample_pout_ports();        // 每批只采一次
self.run_memories();             // 每批只采一次
```

`batch_size > 1` 的触发条件（**不限于无限制 TPS**，2026-10-27 用户确认后订正）：

| 情况 | `batch_size` |
| --- | --- |
| `Tps::Unlimited` | `= max_batch_size`，最大可达 `MAX_BATCH_SIZE = 50_000` |
| `Tps::Limited(n)` 正常跟得上 | 约 1，无影响 |
| **`Tps::Limited(n)` 但服务器落后** | **`ticks_behind` 可为 2、5、10…**，同样进入批量路径 |
| `batch_size = 0` | 本帧不推进 |

- 默认 20 TPS **稳定运行时** `batch_size == 1`，无影响
- 但只要出现一次卡顿（大 RAM 加载、系统调度抖动、Android 后台限流、世界写盘等），`ticks_behind` 立即 > 1，该瞬间进入批量路径，卡顿期间的时钟边沿可能被合并丢失
- 因此本缺陷**不是无限制 TPS 专属**，只是无限制 TPS 下必然发生、限制 TPS 下偶发发生
- 批量路径**不调用 `sample_sub_watches()`**，只有 `automation_per_tick_push = true`（默认 `false`，见 `config.rs:89`）才逐 tick 运行

### `batch_size > 1` 的成因（2026-10-27 补充说明）

主循环（`plot/mod.rs:1829` 附近）目标为「每 `sleep_time` 跑一帧」，20 TPS 时 `sleep_time = 50ms`：

```rust
let before = Instant::now();
self.update();
let delta = Instant::now().duration_since(before);
if delta < self.sleep_time { thread::sleep(self.sleep_time - delta); }
else { thread::yield_now(); }
```

`update()` 内部按墙钟欠账计算本帧 tick 数：`lag_time += now - last_update_time`，`ticks_behind = lag_time / dur_per_tick`。

**只要两次 `update()` 的间隔超过一个 tick 周期，就会批量补跑。**

常见成因：

| 类别 | 例子 |
| --- | --- |
| 单帧自身慢 | 大电路 tick、加载大型存储器文件、世界 flush、编译 |
| 线程被挂起 | Android 后台限流、CPU 降频、调度抖动、设备休眠 / doze |
| IO 抖动 | 世界存档写盘、sdcard 模拟存储（FUSE）延迟 |
| 内存压力 | 大 RAM 占用导致的分页与分配抖动 |

**重要区分**：`max_batch_size = batch_interval / last_nspt`，被 `clamp(1, MAX_BATCH_SIZE)` 限制。

- **单 tick 本身慢** → `ticks_fit` 可能算成 0，被压到 1 → 表现为**TPS 下降**，不发生批量合并
- **tick 很快但帧间隔被外部拉长** → `batch_size` 变大 → **才会批量合并、丢边沿**

因此本缺陷最典型的发生场景是「电路很轻、但服务器被外部原因打断」。

## 修法（用户 2026-10-27 决定）

**核心分工：ROM / RAM 是服务器内部管理的，`automation_per_tick_push` 管不到它们；该开关只管脚本可见的 IO。**

| # | 项 | 结论 |
| --- | --- | --- |
| 1 | **`run_memories()`（ROM / RAM）** | **一律逐 tick 采样**，**不受 `automation_per_tick_push` 影响**；`radv` 与批量路径都照此执行 |
| 2 | `automation_per_tick_push = false` | **按原路径跑**：批量推进，每批采样一次 `POUT`，`EVENT` 在 flush 时 drain |
| 3 | `automation_per_tick_push = true` | **逐 tick 跑**：每 tick 采 `POUT` + `SUB`，并丢弃 flush-time 事件 |
| 4 | 开关管辖范围 | **`SUB` / `EVENT` / `POUT`** 三项，因为它们由自动化脚本直接控制 |
| 5 | `/radvance` | 与正常运行时逻辑一致：**存储器每 tick 都跑**；`POUT` / `SUB` 按开关决定 |
| 6 | **落后时（`ticks_behind > 1`）的处理方式** | **逐 tick 快速追帧，不再合并**：用循环 `tickn(1)` 取代 `tickn(batch_size)`，每个 tick 后调用 `run_memories()`；`POUT` / `SUB` 仍按开关决定 |
| 7 | 与旧行为的区别 | 旧：`tickn(batch_size)` 一次性推进，批内边沿被合并；新：N 个 tick 逐个跑完，边沿一个不漏 |
| 8 | 性能 | 单 tick 调用的额外开销仅为函数调用与 `run_memories()`；`run_memories()` 在无触发器时只做地址位数次哈希查找，可忽略 |


**性能影响评估**

- 逐 tick 的只有 `run_memories()`，其开销为「地址位数次哈希查找 + 触发时数据位数次 `set_node`」，量级很小
- `POUT` / `SUB` 仍受开关约束，因此**本次改动本身**不会引入新的开销爆炸

**既有事实：`automation_per_tick_push = true` 会拖慢无限制 TPS（2026-10-27 用户指出，必须记录）**

- 开关为 `true` 时，**每个 tick** 都要遍历全部 `POUT` 端口与全部 `SUB` 订阅各采样一次
- 无限制 TPS 下 `batch_size` 最大可达 `MAX_BATCH_SIZE = 50_000`，即**单批最多做 5 万次全端口遍历**
- 后果：服务器**跑不满无限制 TPS**，会明显变慢；tick 越多、端口/订阅越多，退化越严重
- 也就是说，**`automation_per_tick_push = true` 与“无限制 TPS 跑满”实际互斥**，需要按场景二选一：
  - 要极致 tick 速率 → 保持 `false`（按批采样，`OUTPUT` / `EVENT` 只看每批的最终状态）
  - 要每个 tick 的精确 IO 变化 → 设 `true`，但接受 tick 速率下降
- 本条不是缺陷，属于**既有的设计取舍**，登记在此以免以后误以为“开了无限制 TPS 也照样逐 tick 采样、性能无损”

## 正确性结论（2026-10-27 用户追问后确认）

**现在（0.4.1 已发布状态）不能保证 ROM / RAM 的正确性**：`run_memories()` 的调用位置导致 `radv` 期间与 `batch_size > 1` 时存在漏采样，与 `automation_per_tick_push` 的取值无关。

**按本条修法（`run_memories()` 一律逐 tick）之后可以保证**：

| 场景 | 修复后 |
| --- | --- |
| `automation_per_tick_push = false` | 每个 tick 都采存储器 ✅ |
| `automation_per_tick_push = true` | 每个 tick 都采存储器 ✅ |
| `/radvance N` | N 个 tick 逐个采样 ✅ |
| 无限制 TPS 大批次 | 批内逐 tick 采样 ✅ |

因为 `prev_clk` / `prev_address` 每个 tick 都会更新：时钟边沿在 tick 粒度上**一个不漏**，`radv` 之后的「假边沿」也自然消失。

## 修不掉的固有限制（**用户 2026-10-27 明确：0-tick 脉冲不用管**）

**同一 tick 内“先升后降”的脉冲采不到。** 存储器在 tick 结束后采样，只能看到该 tick 的最终电平；若时钟在一个 tick 的传播过程中上升又回落（0-tick 脉冲一类），采样时与上一 tick 电平相同，判定为无边沿。

此问题与开关、`radv` 均无关，需要更深的改动（在 redpiler 传播过程中挂钩）才能解决。

**用户 2026-10-27 明确：0-tick 脉冲不用管**，本条不处理、也不立项。

## 附带行为变更

修复后，批内某个 tick 的 RAM 写入会**影响同一批内的后续 tick**（现在要等整批结束才生效）。方向更正确，但属于可观察的行为变化。

**备注**

- 本条与 BUG-021（RAM 持久化）无耦合：不落盘，故不涉及丢盘
- 与 BUG-020（协议重写）有交集：`OUTPUT` / `EVENT` 的语义会在重写中重新定义
- 台账状态：✅ **已实施**（0.4.2-beta）

### BUG-024　告示牌放置时朝向固定偏向一侧

**状态**：✅ **已修复**（0.4.3-beta，收包处统一归一化）；✅ release 编译通过，⏳ 待运行验证
**登记日期**：2026-10-27
**编号说明**：**征用已停用的 BUG-024**（用户 2026-10-27 明确“征用 bug24”）。原 BUG-024（`io.txt`）已于同日并入 BUG-020，编号回收后可重新分配。
**目标版本**：待定
**影响文件**：`crates/core/src/interaction.rs`（`get_sign_placement`）

**用户报告（2026-10-27）**

> 放置告示牌时，只会固定的偏向一侧，这是为什么呢

**代码现状**

朝向在两处决定（`interaction.rs:64`）：

```rust
fn get_sign_placement(context, standard: fn(u8) -> Block, wall: fn(BlockDirection) -> Block) -> Block {
    let rotation = (((180.0 + context.player.yaw) * 16.0 / 360.0) + 0.5).floor() as u8 & 15;
    match context.block_face {
        BlockFace::Bottom => Block::Air,
        BlockFace::Top => standard(rotation),              // 立牌：用玩家 yaw
        _ => wall(context.block_face.unwrap_direction()),  // 墙牌：用被点击的面
    }
}
```

- **立牌**（放在方块顶面）朝向取决于 `context.player.yaw`
- **墙牌**（贴在方块侧面）朝向取决于客户端上传的 `block_face`

`player.yaw` 来自 `PlayerPosition` / `PlayerRotation` 包（`packet_handlers.rs:258`、`:308`），放置走 `plot/mod.rs:702` 的玩家上下文，因此**真玩家手动放置时 yaw 应当是实时值**。

**可能的原因（待澄清后确定）**

1. 通过**自动化 / 机器人**放置：合成上下文里 yaw 恒为 0，于是立牌永远朝同一个方向（rotation = 8）
2. 通过 **WorldEdit / 投影粘贴**放置：这类路径不走 `get_sign_placement`，块状态取自投影，可能全部同一朝向
3. 真玩家手动放置也偏：那说明 `yaw` 没被正确更新，或 `block_face` 解析/换算有误

**用户答复（2026-10-27）**

- 放置方式：**玩家客户端手动放置**
- 现象对象：**立牌**（放在方块顶上的那种）

**静态排查结论（2026-10-27）**

已排除以下可能性，代码路径在纸面上是正确的：

| 检查项 | 结论 |
| --- | --- |
| 朝向计算 | `interaction.rs:69` 与 MC 原版公式一致：`floor((yaw + 180) * 16 / 360 + 0.5) & 15` |
| `block_face` 解析 | `BlockFace::from_id` 与协议 id（0=下 1=上 2=北 3=南 4=西 5=东）完全对应 |
| `yaw` 是否被更新 | `PlayerPositionAndRotation`（`packet_handlers.rs:258`）与 `PlayerRotation`（`:308`）都会写入 `player.yaw`；仅位置的包不动 `yaw`，无误清零 |
| 放置入口 | 唯一的 `get_state_for_placement` 调用点在 `use_item_on_block`（`interaction.rs:481`），使用真实玩家上下文 |
| 状态编码 | `generated.rs` 中 `OakSign { rotation }` 的 encode / decode 均存在，属性名 `rotation` 正确 |
| 落块流程 | `place_in_world` 直接 `world.set_block(block)`，不改写状态 |

因此**大概率不是朝向计算本身的问题**，而需要运行期数据定位。两种可能：

1. 放置那一刻服务端看到的 `player.yaw` 恒为 0（例如旋转包未按预期到达或被覆盖）
2. 服务端算对了，但下发给客户端的方块状态（state id）里 `rotation` 丢失，客户端一律按默认朝向渲染

**运行期诊断（用户 2026-10-27 授权，临时构建代号 `0.4.3-alpha`）**

在 `interaction.rs` 的 `use_item_on_block` 里加了临时日志（**未改任何逻辑**）：

- 放置前：打印 `pos` / `block_face` / `player.yaw` / `player.pitch` / 算出的 `block`（含 rotation 或 facing）/ `block.get_id()`（要下发的 state id）/ 放置前该位置的原状态
- 放置后：打印 `world.get_block_raw(pos)` 与实际存下的 `block`

日志前缀 `[SIGN-DIAG]`，便于过滤。

**判读方法**

| 现象 | 结论 |
| --- | --- |
| `yaw` 随转身变化，且 `state_id` 也不同，但游戏里朝向仍固定 | **下发/渲染**问题（state id → 客户端显示） |
| `yaw` 恒为同一个值（尤其 0） | **收包**问题：旋转包未生效或被覆盖 |
| `yaw` 在变、但 `block=` 里的 rotation 始终相同 | 朝向计算本身有误 |
| `state_id` 与 `raw_after` 不一致 | 落块或状态编码不一致 |

**根因（2026-10-27 由诊断日志确认）**

日志显示玩家 `yaw` 是 **9820.04 / 9755.24 / 9810.59 / 9775.04**（客户端未归一化，累加了约 27 圈），而四次放置的 `block` 全部是 `rotation: 15`、`state_id` 全部是 `4397`。

原因在 `interaction.rs:69`：

```rust
let rotation = (((180.0 + context.player.yaw) * 16.0 / 360.0) + 0.5).floor() as u8 & 15;
```

- 代入 yaw≈9800 后括号内结果约 **440**
- Rust 的 `as u8` 是**饱和转换**：大于 255 一律变 **255**
- `255 & 15 == 15`，于是任何朝向都得到 `rotation = 15`

归一化后正确值应为 12 / 10 / 12 / 10（四个朝向不同），与用户观察到的“全部同一侧”一致。

**上游同源性（2026-10-27 核实）**

- 上游副本 `~/mchprs/source/MCHPRS`（2026-09-28 快照）的 `get_sign_placement` 与我们的**逐字节完全相同**，同样有 `as u8` 饱和问题
- 上游 `player.rs` 的 `get_facing()` **做了** `yaw.rem_euclid(360.0)` 归一化，但 `get_direction()` 与 `get_sign_placement()` **没做**，属上游自身的不一致
- 结论：**这是上游缺陷，不是本 fork 引入的**

**处置（用户 2026-10-27 更新：解禁）**

> 不管上游，只管自己，有bug就记录下来

- 初次决定是“暂不修复，只记录”；**用户随后解禁**，要求按下面的方案修
- 因此本条**不再是“仅记录”**，进入“待写代码”状态，等用户“写代码”口令

**实施记录（0.4.3-beta，2026-10-27）**

- `plot/packet_handlers.rs` 两个处理函数都加了 `rem_euclid(360.0)`，回显包统一用归一化后的 `yaw`
- `interaction.rs` 的两处 `[SIGN-DIAG]` 日志与 `use tracing::info;` 已删除，该文件回到与提交 `f685cfc` 逐字一致的状态
- 修正旧判断：`get_direction()` 不需要改（`& 3` 对 360 的整数倍天然免疫）
- ✅ release 编译通过（0.4.3-beta），⏳ 待运行验证

**修复方案（用户 2026-10-27 选定：收包处统一归一化）**

1. `plot/packet_handlers.rs` 的 `handle_set_player_position_and_rotation`：先用 `yaw.rem_euclid(360.0)` 归一化，再写入 `players[player].yaw`；回显包 `CTeleportEntity` / `CUpdateEntityPositionAndRotation` / `CSetHeadRotation` 一律用归一化后的值
2. 同文件 `handle_player_rotation` 同样处理（`CEntityRotation` / `CSetHeadRotation` 用归一化值）
3. `interaction.rs` 的 `get_sign_placement` **不再单独归一化**：yaw 进服务端时已在 `[0, 360)`，`as u8` 不会再饱和（最大值约 24）
4. 删除两处 `[SIGN-DIAG]` 日志与 `use tracing::info;` 导入（下次正式编译时一并做）

**连带影响（2026-10-27 已核对，均无害）**

| 位置 | 影响 |
| --- | --- |
| `player.rs:361` `get_direction()` | 行为不变。原先看着可疑，实际加 360 度正好让商加 4，`& 3` 天然免疫；只有 \|yaw\| 超过 `i32` 上限（21 亿）才会饱和，不可能出现 |
| `player.rs:371` `get_facing()` | 自带的 `rem_euclid` 变成冗余，结果不变 |
| `player.rs:319` `save()` 的 `rotation` | 写盘的 yaw 从 9820 变成 100，只改数值表达，不影响朝向 |
| `server.rs:409`、`worldedit/execute.rs:468`、`:482`、`commands.rs:199` | 只当角度用，归一化后结果一致 |

**构建信息**

- 内核与协议版本**均保持 `0.4.2-beta` 不变**（用户要求，避免现有 `v0.4.2-beta` 的 memory 文件被拒载）
- 临时构建在对话与台账中称 **`0.4.3-alpha`**
- 产物 `target/release/mchprs`，8,476,168 字节，sha256 `45bfda0d7908be4cf8742364131b3e7146a6c7314120ea3c2dc0d8164170e11b`

**收尾要求**

- 诊断结束后**必须删除**这两处 `[SIGN-DIAG]` 日志与 `use tracing::info;` 导入，再出正式版本

**备注**

- 本条曾只登记、未改代码；2026-10-27 解禁，等“写代码”口令
- 与 BUG-022 / BUG-023 / BUG-021 无关，属独立缺陷

## 4. 候选条目

（暂无独立编号的候选。历史：原候选 A 立项为 **BUG-006**，已明确不做（见 §6）；原候选 B 立项为 **BUG-007**，仍待实施。详见 §3。）

### 4.1 观察：自动化 `EVENT` 推送的完整性（未立项，待实测）

**记录日期**：2026-10-25
**背景**：用户 2026-10-25 追问"`automation_per_tick_push` 开启后，`EVENT` 会不会漏掉某个 tick 的变化"，并回忆"redpiler 内部有个 2ms"。
**用户处置**：不立项、不改代码，**先记着**，等真正遇到现象时再处理。

**已查清的事实（代码层面）**

1. **redpiler 内部没有 2ms**：`crates/redpiler/src/` 里 `Duration` 一次都没出现，只有 3 处 `Instant::now()` 用于 timings 计时（`passes/mod.rs:216`、`lib.rs:167`、`lib.rs:204`）。
2. **全仓唯一的 2ms** 是 `server.rs:213` 的主循环 `sleep(2ms)`（另一处 `:246` 在优雅关闭里）。它只造成 **≤2ms 延迟 + 多条行合并成一次 socket write**，不丢行：`conn.pending` 是纯追加字节缓冲（`queue_line`），mpsc 无界，`flush` 遇 `WouldBlock` 留到下一轮。
3. **逐 tick 模式下每个 tick 都采样**（`plot/mod.rs:1448-1458`）：`tickn(1)` → `sample_sub_watches()` → `sample_pout_ports()`，且比对基线 `sub_prev` 每 tick 前进 → 不会把多个 tick 合并成一条。
4. `query_io` 读的是**后端节点表**（`node.powered` / `node.output_power`），不读世界方块，与"世界每批 flush 一次"无关。

**会漏的例外（代码推断，未实测）**

| # | 场景 | 后果 | 位置 |
| --- | --- | --- | --- |
| 1 | `/radvance N` 手动推进 | 只在最后 `publish_world()` 发一次 flush 期事件 → 中间 tick 的变化全丢 | `commands.rs:291` |
| 2 | `query_io(pos)` 返回 `None` | 该 tick 跳过该位置，且 `sub_prev` 不更新 | `plot/mod.rs:1037` |
| 3 | `redpiler.is_active() == false` | 采样直接 `return` | `plot/mod.rs:1014` |
| 4 | `Tps::Limited` 跟不上 | `ticks_behind.min(max_batch_size)` 丢弃多余 tick —— tick 没执行，非事件漏 | `plot/mod.rs:1430` |
| 5 | 同一 tick 同一点 | 最多一条（`pushed` 去重），设计如此 | `plot/mod.rs:1046` |
| 6 | 订阅之前的变化 | 注册只 seed 基线，不推送 | `plot/mod.rs:797` |

**会重复的情况**：批中途若有人调用 `publish_world()`（玩家跨区块移动 `:514`、右键交互 `:611`、压力板 `:417/:433`、发送方块纠正 `:359`、命令 `commands.rs:188/209/292`），它会 `flush()` 并发出 flush 期事件，与逐 tick 采样流重叠 → 同一次变化可能收到两条 `EVENT`。纯无人脚本场景基本不触发。

**待办（仅在用户真的遇到问题时）**

- 构造"每 tick 翻转一次"的电路，`SUB` 后统计 `EVENT` 行数并与 tick 数对账；再单独验证 `/radvance` 与"有玩家移动"两个重复/丢失场景
- 该验证需要**编译 + 运行**授权，当前**未**进行

---

## 5. 待添加功能

（暂无）

---

## 6. 已明确不做

| 条目 | 说明 | 确认时间 |
| --- | --- | --- |
| 岩浆锅 / 细雪锅的比较器支持 | 用户明确只需识别水锅 | 2026-09-30 |
| **BUG-006：编译失败后回填 `to_be_ticked`** | 用户 2026-10-25 明确："如果编译崩溃，tick 丢失不用管"。原方案 A1（编译前 `ticks.clone()`、失败时回填）**不实施**；BUG-001 的「后续修订」（空容器降级）落地后，最常见的失败源也已消失 | 2026-10-25 |
| **其它 panic 崩溃点 / plot 崩溃全局兜底** | 用户明确**不改**。BUG-004 只做比较器取值这一处点状修复，**不引入** `catch_unwind`，也不做全局 panic 静音。**〔2026-10-25 由 BUG-011 收窄〕**编译路径的 panic 改为「保存存档 + 打印 + `exit(1)`」，这不是静音也不是全局兜底，其余 panic 点仍按本行处理。以下已查实、运行时可达的 panic 点**保持原样**：`identify_nodes.rs:217`、`backend/direct/compile.rs:47-58`（≥255 入边）、`plot/mod.rs:1683`（plot 加载失败）、`redstone/src/lib.rs:260-262`（音符盒缓存不一致）、`compile_graph/stable_graph.rs`（4 处图不变量）、`redpiler/src/lib.rs:226`、`plot/worldedit/mod.rs:217-262`、`plot/monitor.rs:32`。 | 2026-09-30 |

> 注：原清单中的 `passes/analysis/ss_range_analysis.rs:313` 已随 BUG-008 删除该文件而消失（上游版本是穷尽 match，无 `unreachable!`），故不再列出。

---

## 7. 版本升级记录

| 版本 | 内核 | 协议 | 条目 | 状态 |
| --- | --- | --- | --- | --- |
| **0.4.3-beta** | 0.4.2-beta → **0.4.3-beta** | 0.4.2-beta → **0.4.3-beta** | BUG-021 回写格式（规则 1–6）+ BUG-021 规则 7b（玩家退出落盘）+ BUG-024（收包处统一归一化）+ 删除 `[SIGN-DIAG]` 诊断日志 | ✅ **release 编译通过**，⏳ 未运行验证 |
| **0.4.2-beta** | 0.4.1-beta → **0.4.2-beta** | 0.4.0-beta → **0.4.2-beta（统一升版）** | BUG-022 / 023 / 027 + BUG-021 落盘改动 + BUG-026 项下「只写变化的位」与「去掉每 tick 总线 clone」 | ✅ 六项已实施并 release 编译通过；第 7 项 7b（玩家退出落盘）已写未编译 |
| **0.4.1-beta** | 0.4.0-beta → **0.4.1-beta** | 0.4.0-beta（不变） | BUG-017 / 021 + BUG-015 §37 | ✅ 已实施，**`cargo build --release` 通过**，⏳ 待运行验证 |
| **0.4.0-beta** | 0.3.0-beta → **0.4.0-beta** | 0.3.0-beta → **0.4.0-beta** | BUG-014 / 015 / 016 | ✅ 已实施，**`cargo build --release` 通过**，⏳ 待运行验证 |
| **0.3.0-beta** | 0.2.0-beta → **0.3.0-beta** | 0.2.1-beta → **0.3.0-beta** | BUG-001 修订 / 007 / 009 / 010 / 011 / 012 / 013 | ✅ 已实施，**`cargo build --release` 通过**，⏳ 待运行验证 |
| 0.2.2-beta | — | — | BUG-008 | ⚠️ **已并入 0.3.0-beta，不再单独发布**（原拟定未落地） |
| 0.2.1-beta | 0.2.0-beta（不变） | 0.2.0-beta → 0.2.1-beta | BUG-005 | ✅ 已实施，待运行验证 |
| 0.2.0-beta | 0.1.0-beta → 0.2.0-beta | 0.1.2-beta → 0.2.0-beta | BUG-001~004 | ✅ 已实施，待运行验证 |

### 0.4.3-beta（2026-10-27）——回写格式、告示牌朝向与退出落盘

**包含条目**

| 条目 | 内容 |
| --- | --- |
| **BUG-021 回写格式** | `render()` 改为 Logisim 兼容排版（规则 1–6） |
| **BUG-021 规则 7b** | 任意玩家退出服务器即落盘 |
| **BUG-024** | 收包处统一归一化 yaw，修复告示牌朝向恒为 15 |
| **清理** | 删除两处 `[SIGN-DIAG]` 临时日志与 `use tracing::info;` |

**版本号**（用户 2026-10-27 指示：内核与协议都升）

- **内核**：`0.4.2-beta` → **`0.4.3-beta`**
- **协议**：`0.4.2-beta` → **`0.4.3-beta`**（只有 `HELLO` 里的版本串变化，语法与语义不变）

**变更摘要**

- **BUG-021 回写格式**：`render()` 不再按"连续存在的地址"拼行，改为按 8 字块输出——每行固定 8 个 word、行首地址按 8 对齐、按地址总线宽度补足十六进制位数（位数取 `(address.len() + 3) / 4`，最少 1 位）、不含已存字的块整块省略、空格子用 `pack_word_hex(&vec![false; bits], matrix)` 输出定宽零（4 位字 `0`、8 位字 `00`、32 位字 `00000000`，`matrix` 按逐行补零后的位数）。常量 `FLUSH_GROUP` 更名 `WORDS_PER_LINE`
- **BUG-024**：`handle_set_player_position_and_rotation` 与 `handle_player_rotation` 在写入 `players[].yaw` 之前先做 `rem_euclid(360.0)`，回显包（`CTeleportEntity` / `CUpdateEntityPositionAndRotation` / `CSetHeadRotation` / `CEntityRotation`）一律使用归一化后的值；`get_sign_placement` 的公式保持原样不动
- **7b**：`remove_dc_players()` 在检测到有人断开时调用 `flush_memories()`（复核表见 0.4.2 节末）

**文件清单**

`crates/core/src/memory.rs`、`crates/core/src/plot/packet_handlers.rs`、`crates/core/src/interaction.rs`、`crates/core/src/automation.rs`、`Cargo.toml`、`Cargo.lock`、`README.md`、`README_zh.md`、`src/protocol.txt`、`src/protocol_zh.txt`

**编译结果**

- ✅ `cargo build --release` **通过**（2026-10-27 09:29，1m 49s，exit 0，首次编译即通过）
- 产物 `target/release/mchprs`，8,451,776 字节，sha256 `d029909f7dbfa6440a769b066c6cccef6867627a4480c17565f6794fd66d53e9`
- `./target/release/mchprs --version` → `mchprs_core v0.4.3-beta`（提交 `f685cfc`）
- 警告 2 条，均为改动前既有：`redpiler` 的 `AnalysisUsage::reset` 未使用、`monitor.rs:181` 的 `fetch_update` 已弃用（另有 1 条 `proc-macro-error2` 的 future-incompat 提示，来自依赖）
- 备份：`~/.ssset/backup/mchprs-0.4.3-beta`（新产物）、`mchprs-0.4.3-alpha`（诊断版 8,476,168 字节）
- ✅ 线上 `~/mchprs/mchprs` **已替换**为 0.4.3-beta（2026-10-27 09:32，8,451,776 字节，sha256 `d029909f7dbfa6440a769b066c6cccef6867627a4480c17565f6794fd66d53e9`），`--version` 自检为 `mchprs_core v0.4.3-beta`
  - 替换前线上是诊断版 0.4.3-alpha（8,476,168 字节，sha256 `45bfda0d…`），已备份
  - 替换那一刻服务器进程未运行；替换后需要用户自行启动
  - 中途 Termux SSH 隧道断过一次（`Connection timed out during banner exchange`），当时的 `cp` 未执行，恢复后按实际状态核对再重做，不存在重复替换

**注意**

- 内核版本变化后，`memory/*.rom.txt` 与 `*.ram.txt` 的**首行必须写成 `v0.4.3-beta`**，否则会被拒载
- 已同步更新用户文件：`MCHPRS_IO/memory/test.ram.txt` 与 `MCHPRS_IO/test.rom.txt`（后者当前在 `MCHPRS_IO/` 而不是 `memory/` 下，不会被加载）
- 回写格式变化**向后兼容**：解析器仍接受旧的稀疏写法（未列出的地址读作 0），旧文件不用改
- 行为变化：`player.yaw` 现在恒在 `[0, 360)`，写进玩家存档的 `rotation` 数值随之变化（不影响朝向）
- 文档同步：两份 README 的内存章节改写为"每行 8 字、按 8 对齐、按地址总线补位、空块省略、五个落盘时机"；协议文档第 12 节补上 7b 与新的回写格式，并顺手修掉"`*.ram.txt` 已预留、当前忽略"的过期描述

**验证状态**

- ✅ 编译通过；⏳ 未运行验证（还没在真实服务器上跑过告示牌朝向与 RAM 回写格式）

### 0.4.2-beta（2026-10-27）——端口语法展开、逐 tick 追帧与落盘时机

**用户决定的范围**（2026-10-27 多选）

| 条目 | 内容 |
| --- | --- |
| **BUG-022** | 端口坐标步进展开 + `matrix` 固定矩阵展开（含逐行补零） |
| **BUG-023** | 端口列表 `width` 必填 + 展开数量校验（**破坏性**） |
| **BUG-027** | 落后时逐 tick 追帧，`run_memories()` 每 tick（`radv` 一并修正） |
| **BUG-021 改动** | 取消运行期定时落盘，改为四事件落盘（退出 / `/redpiler reset` / 崩溃 / `/memory reload` 前） |
| **BUG-026 项下两项** | ① 只写变化的位：位值未变则跳过 `set_node`；② 去掉每 tick 的总线 clone：拆字段借用 |

**本次明确不做**

- BUG-020（协议整体重写）：规则尚未定完，留到后续版本
- BUG-025（报错出口统一）：错误码表依赖 BUG-020

**依赖与顺序**

1. **BUG-022 + BUG-023 必须一起做**：改的是同一处 `parse_list()`；`width` 是破坏性变更，旧文件全部需要补 `width`
2. **BUG-027**：改 `tickn` / `update` 的 tick 循环，与存储器的解析逻辑无耦合，可独立进行
3. **BUG-021 改动**：删除 `FLUSH_INTERVAL` / `FLUSH_WRITES` 定时触发，补 `/redpiler reset` 时的落盘调用
4. **BUG-026 只写变化的位**：改 `run_rom` / `run_ram` 的写出路径

**影响面**

- `crates/core/src/memory.rs`（解析、校验、hex 编解码、落盘、写出路径）
- `crates/core/src/plot/mod.rs`（`tickn` / `update` / `run_memories` / `flush_memories`）
- `crates/core/src/plot/commands.rs`（`radv`、`/redpiler reset`）
- `Cargo.toml`（版本号 → `0.4.2-beta`）
- `README.md` / `README_zh.md` / `src/protocol.txt` / `src/protocol_zh.txt`（语法与落盘时机）
- 用户现有 `memory/*.rom.txt` / `*.ram.txt`：补 `width`、升级首行版本号

**实施进度**

| # | 方案 | 状态 |
| --- | --- | --- |
| 1 | BUG-022 + BUG-023（解析 / 校验 / hex 编解码） | ✅ **已实施，release 编译通过** |
| 2 | BUG-027（逐 tick 追帧） | ✅ **已实施，release 编译通过** |
| 3 | BUG-021 落盘改动 | ✅ **已实施，release 编译通过** |
| 4 | BUG-026（只写变化的位 + 去掉 clone） | ✅ **已实施，release 编译通过** |
| 5 | 版本号统一升 0.4.2-beta | ✅ **已实施，release 编译通过** |
| 6 | 用户 memory 文件补 `width` / 升首行 | ✅ 已更新磁盘文件 |
| 7 | 玩家退出服务器时回写 RAM（BUG-021 规则 7b） | 🟡 代码已写入并逐行复核通过，**未编译、未运行验证**（复核记录见本节末） |

**7b 代码复核（2026-10-27，逐行核对 `remove_dc_players` / `flush_memories`）**

| 检查项 | 结论 |
| --- | --- |
| 调用点 | `remove_dc_players()` 在 `update()` 末尾无条件调用，每个循环周期都跑，断线当周期内即落盘 |
| 触发条件 | `had_disconnections` 取自 `retain` 前后差集，**任意一人**下线为真；同周期多人断线只落盘一次 |
| 落盘范围 | `flush_memories()` 只写 `is_dirty()` 的 RAM；ROM 与尚未写入过的 RAM 不碰盘 |
| 与关服路径的关系 | `BroadcastMessage::Shutdown` 走 `players.drain()`，不进 `remove_dc_players`，但它自己有 `flush_memories()`，不遗漏也不重复 |
| `/tp` 跨地皮 | `plot/commands.rs` 的 `leave_plot` 不落盘；单地皮模型下目标玩家恒在 p0,0，走同地皮分支，`leave_plot` 分支实际不可达 |
| 借用关系 | `message_sender` 的借用止于 `retain` 调用，之后 `destroy_entity` / `flush_memories` 取 `&mut self` 不冲突 |
| 未验证项 | **编译**与**运行**均未做（用户 2026-10-27 决定先不编译） |


**代码与台账一致性审计（2026-10-27）**

| 规则组 | 结论 |
| --- | --- |
| BUG-022 规则 1–24 | ✅ 全部落地（符号必填 / 单轴 / 步长 ±255 / 数量 1–1024 纯数字 / 零步长报错 / matrix 1–3 段单行 / 行优先 / 逐行补零 / 重复检测 / 补零位忽略） |
| BUG-023 规则 1–10 | ✅ 全部落地（width 必填且为第一项 / 数量校验 / 仅字节使能可 0 / 读写宽度相等 / 空列表报错 / 单点端口无 width） |
| BUG-021 规则 7 / 7a | ✅ 四个落盘时机全部存在：`reload_memories` 开头、编译崩溃分支、`reset_redpiler` 开头、`BroadcastMessage::Shutdown`；`Drop for Plot` 仍作兜底；SIGINT/SIGTERM/SIGHUP 由 `ctrlc` 的 `termination` feature 覆盖（未改动） |
| BUG-026 决策 | ✅ 定时落盘已删（`FLUSH_INTERVAL` / `FLUSH_WRITES` / `flush_due` 均已移除）；`run_rom` / `run_ram` 内已无 `clone()`，改为借用 `&self.memory` 的字段；写总线前先比较位值 |
| BUG-027 决策 | ✅ `update()` 批量分支改为 `for _ in 0..batch_size { tickn(1); run_memories(); }`；两条 `/radvance` 入口都走 `advance_ticks()`；`POUT` / `SUB` 仍只受 `automation_per_tick_push` 控制 |
| 版本号 | ✅ `Cargo.toml`、`HELLO`、`protocol*.txt`、`README*.md` 均为 `0.4.2-beta`；历史叙述（“0.4.0-beta 起”等）保留 |

**差异的处置（用户 2026-10-27 决定）**

- 差异 1（越界回绕）：**不修**，保持现状
- 差异 2（`width = 0` 文案）：**记入 BUG-025** 的“已知文案问题”，随报错统一一起改
- 差异 3（`EVENT` 数量减少）：**无影响**，不处理

**发现的差异（均不影响编译）**

1. **越界坐标会回绕**：`shift_axis()` 用 `i64` 累加后 `as i32` 强转，超出 i32 的展开值会回绕而不是原样保留。台账规则 4 写的是“解析期不报错、交给编译期”，回绕与此略有出入（实际使用中步长 ≤255、段数 ≤3，几乎不可能触发）。
2. **`width = 0` 只在字节使能上被接受，但通过间接报错实现**：其它列表写 `width = 0` 会得到 “`width` must be greater than 0”，而不是专门的“仅字节使能可写 0”提示。行为符合台账，文案不同。
3. **可观察行为变化（非缺陷）**：由于“只写变化的位”，数据总线上位值未变时不再调用 `try_set_powered`，因此不会再为“值没变的节点”产生 `EVENT`。旧实现会把每次写入都标记为 `changed`，从而可能推送重复事件。这属于预期的收益，但脚本若依赖重复事件需知悉。

**编译结果（2026-10-27 08:04）**

- `cargo build --release` **通过**，耗时 1m 36s
- 产物 `target/release/mchprs`，8,451,576 字节，sha256 `f6b5611bf54c561cc6b2a735465c341470970ba0ca338318e49563c4b42148fe`
- 仅剩两条既有警告（`redpiler` 未使用的 `AnalysisUsage::reset`、`monitor.rs` 的 `fetch_update` 已弃用），与本次改动无关
- 第一次编译失败的原因：通过 shell 写入源码时**单引号被剥离**，`split_once(=)` 等字符字面量变成裸符号，共 7 处；已修复后重新编译
- 线上二进制 `~/mchprs/mchprs` **尚未替换**，仍为 0.4.1-beta

**方案 2–6 已落地的内容（2026-10-27）**

- **BUG-027**：新增 `Plot::advance_ticks()`，`/radvance`（玩家与自动化两条入口）改为走它；`update()` 的批量分支不再 `tickn(batch_size)` 一次推进，而是逐 tick `tickn(1)` + `run_memories()`；`POUT` / `SUB` 仍按 `automation_per_tick_push`
- **BUG-021**：删除 `FLUSH_INTERVAL` / `FLUSH_WRITES` / `flush_due()` / `writes` / `last_flush`；`run_memories()` 不再落盘；`reset_redpiler()` 开头新增 `flush_memories()`，与退出 / 崩溃 / reload 一起构成四个落盘时机
- **BUG-026**：`run_rom` / `run_ram` 改为按字段借用 `&self.memory`（去掉每次时钟边沿 4 条总线的 `clone`）；写数据总线前先 `query_io` 比较，位值未变则跳过 `try_set_powered`
- **版本号**：`Cargo.toml`、`automation.rs` 的 `HELLO`、`protocol*.txt`、`README*.md` 全部升到 `0.4.2-beta`（历史叙述保留）
- **文档**：README 内存章节与 protocol 第 12 节补充 `width` 必填、步进展开、`matrix` 逐行补零、落盘时机变更
- **用户 memory 文件**：`test.rom.txt` / `test.ram.txt` 首行升到 `v0.4.2-beta`；`test.ram.txt` 里重复的第二个 `byte write enable` 块改为注释（新规则下重复键报错）

**方案 1 已落地的内容（2026-10-27）**

- 新增 `MatrixShape` / `PortList` / `Provenance` 结构
- `parse_list()` 重写：`width` 必填且必须是列表第一项；支持 `width = 0`（仅字节使能）；空列表 `[]` 报错
- 坐标行支持 `x, y, z` 与 `x, y, z, ±步长, 数量` 两种形态；步长符号必填、|值| 1–255、数量 1–1024 且必须纯数字
- `matrix:` 支持 1–3 段、只能单行、大小写不敏感、行优先（第一段最快）
- 所有列表内跨键重复坐标检测，报错带行号 + 展开序号 + 坐标 + 冲突行
- hex 编解码改为 `matrix` 感知：写入按逐行补零、读取先补齐再去掉补零位
- `Memory` 新增 `matrix: Option<MatrixShape>`

**本次明确不改（用户 2026-10-27 确认）**

| 项 | 归属 | 0.4.2 是否包含 |
| --- | --- | --- |
| **`io.txt`** | BUG-020（原 BUG-024 并入） | ❌ **不做** |
| **协议格式修改** | BUG-020 | ❌ **不做**：`INPUT` / `OUTPUT` / `PIN` / `POUT` / `ERR` / `OK` 等全部保持原样 |

本次改动全部集中在**文件层与内部实现**：存储器文件语法（BUG-022 / 023）、tick 循环（BUG-027）、落盘时机（BUG-021 改动）、写出路径与借用方式（BUG-026）。**没有一条触碰协议线。**

**版本号（用户 2026-10-27 更新：统一升版）**

- **内核版本**：`0.4.1-beta` → **`0.4.2-beta`**
- **协议版本**：`0.4.0-beta` → **`0.4.2-beta`**（**不再保持 0.4.0-beta**，两处版本号统一为 `0.4.2-beta`）
- 需要同步的位置：`Cargo.toml`、`automation.rs` 的 `HELLO MCHPRS AUTOMATION <版本>`、`src/protocol.txt` / `src/protocol_zh.txt`、`README.md` / `README_zh.md`
- 注意：`HELLO` 字符串变化属于**线协议可观察变更**，但协议**语法与语义本身不变**（本次仍不做 BUG-020）

**工作方式（用户 2026-10-27 指示）**

本次任务较复杂，采用任务清单逐项跟踪：**每完成一个方案就记入台账**，不攒到最后统一写。

**备注**

- 协议**语法与语义**保持原样；仅 `HELLO` 中的版本字符串随升版变化
- **间接行为变化（非协议变更）**：BUG-027 修好后，存储器在 `/radvance` 期间会真正工作，同一脚本读到的 `OUTPUT` 可能与旧版不同——因为旧版 radv 期间 ROM / RAM 完全没运行。属于修复后的正确行为，不是协议改动
- 台账状态：✅ 六项已实施并编译通过；7b 已写、未编译（用户 2026-10-27：确保代码正确即可，暂不编译）

---

### 0.4.1-beta（2026-10-26）——可写存储器与存储器校验

- **包含条目**：BUG-021（`.ram.txt`）、BUG-017（data 拉杆与 address 元件相邻校验）、BUG-015 §37（列表开括号可换行）
- **版本号**：内核 `0.4.0-beta` → **`0.4.1-beta`**；协议**保持 `0.4.0-beta`**（本次没有报文变更，RAM 是服务器侧功能）
- **变更摘要**
  - **BUG-021**：`memory/*.ram.txt` 可写存储器。新键 `write enable` / `read enable` / `byte write enable` / `write data coordinate` / `read data coordinate`；`clk` 必须存在；上升沿锁存、1 tick 延迟、读优先；字节写使能按 `ceil(N/8)` 从高到低分组、最后一条管低 8 位；内容稀疏写回同一文件（保留头部、首行写当前版本、tmp+rename 原子替换），落盘（0.4.1 原为每 2 秒或 4096 次写入；**2026-10-27 改为仅事件驱动**：退出 / `/redpiler reset` / 崩溃 / reload 前，运行期不写盘）
  - **BUG-017**：`Memory::validate` 增加面相邻检查；ROM 的 data 拉杆与 address/时钟相邻即禁用该条
  - **BUG-015 §37**：`parse_list` 起改为"若本行没有 `[` 就继续读下一行"，与注释一致
  - **`crates/redpiler`**：新增 `JITBackend::has_inputs` / `Compiler::has_inputs`，用 forward link 反查某节点是否有入边（RAM 校验"读数据拉杆不得被驱动"用）
  - **报错信息补全**（用户 2026-10-26 要求）：解析错误统一带行号与键名（如 ``address coordinate`: line 8: coordinate `12, 6` must be a triple`）；RAM 缺时钟有专属文案；`/memory reload` 把被拒文件与原因（前 5 条）以红色发到玩家聊天框；校验错误拆成"方块类型不对（并指出实际是什么）"与"该位置不在编译结果里"两种
- **文件清单**：`Cargo.toml`、`crates/core/src/memory.rs`（重写，1138 行）、`crates/core/src/plot/mod.rs`、`crates/redpiler/src/{lib.rs, backend/mod.rs, backend/direct/mod.rs}`、`README.md`、`README_zh.md`、`src/protocol.txt`、`src/protocol_zh.txt`
- **编译结果**：✅ `cargo build --release` 通过（2026-10-26 15:08，产物 8,434,832 字节）
  - 编译期修正 2 处：`fwd_link_range` 不是 `Copy`（改 `&node.fwd_link_range`）；`at_line` 的 `impl Into<String>` 不能直接 `{}` 格式化（补 `.into()`）
  - 警告 2 条，均为改动前既有
- **注意**：ROM/RAM 首行版本改成 `v0.4.1-beta` 后，**旧的 `v0.4.0-beta` 文件会被拒载**（这是用户定的严格匹配）；RAM 落盘时会自动把首行更新成当前版本
- **验证状态**：✅ 编译通过；⏳ **未运行验证**

### 0.4.0-beta（2026-10-26）——内存文件 + 协议格式变更

- **包含条目**：BUG-014、BUG-015、BUG-016
- **版本号**：内核 `0.3.0-beta` → **`0.4.0-beta`**；协议 `0.3.0-beta` → **`0.4.0-beta`**（用户 2026-10-26 决定：内核与协议都升）
- **变更摘要**
  - **BUG-014**：空容器警告由黄色 `send_system_message` 改为红色 `send_error_message`；`TaskMonitor` 保留全部位置（按位置去重），消息格式 `Redpiler compile warning: comparator at (x,y,z),(x,y,z) reads some empty containers`，超过 10 个用 `,(...)` 收尾，不再区分单复数
  - **BUG-015**：新增模块 `crates/core/src/memory.rs`；`memory/*.rom.txt`（Logisim `v3.0 hex words addressed`）定义只读存储器；`clk` 上升沿或有/无时钟的地址变化触发；address/clk 读红石灯·活板门·音符盒，data 写拉杆·踩踏板；tick 后读写（1 tick 延迟）；编译成功后逐条校验，失败仅禁用该条并红色提示；`/memory reload`（玩家 + 自动化命令通道）
  - **BUG-016**：`pack_hex` / `parse_hex_bits` 由"末位半字节后端补零"改为"整条位向量当一个无符号整数、前端补零"；`N % 4 != 0` 时 hex 变化，**破坏性变更**，旧格式返回 `ERR INPUT HEX`
- **文件清单**：`Cargo.toml`、`crates/core/src/{lib.rs, memory.rs（新增）, plot/mod.rs, plot/commands.rs, automation.rs}`、`crates/redpiler/src/task_monitor.rs`、`src/protocol.txt`、`src/protocol_zh.txt`、`README.md`、`README_zh.md`、`BACKLOG.md`
- **回滚点**：`~/.ssset/backup/code-before-0.4.0/`（6 个文件）、`~/.ssset/backup/docs-before-0.4.0-readme/`、`~/.ssset/backup/docs-before-0.4.0-proto/`
- **编译结果**：✅ `cargo build --release` 通过（2026-10-26，1m51s，exit 0，产物 `mchprs v0.4.0-beta`）
  - 编译期修正 1 处：`memory.rs` 需要 `use std::ops::Add;` 才能用 `BlockPos` 的 `+`
  - 警告 2 条，均为改动前既有（`AnalysisUsage::reset` 未使用、`plot/monitor.rs:181` 的 deprecated `fetch_update`）
  - 新增文件零警告
- **文档更新（2026-10-26）**
  - `README.md` / `README_zh.md`：运行模型标题升 0.4.0-beta；特性列表更新（hex 前端补零、空容器红色合并警告、文件 ROM）；「空容器」段落改写；新增「Memory files (ROM) / 内存文件（ROM）」整节；脚本命令表与常用命令表加入 `/memory reload`；演示输出示例版本号同步
  - `src/protocol.txt` / `src/protocol_zh.txt`：版本号与 HELLO 同步；`INPUT`/`OUTPUT` 的 hex 说明改为整数前端补零并标注破坏性变更；命令白名单加入 `/memory reload`；第 11 节空容器条目改写；新增 **第 12 节「Memory files / 内存文件」**，含"无时钟 + 地址数据重合 = 1 tick 振荡、服务器不检测"的风险说明
- **验证状态**：✅ 编译通过；⏳ **未运行验证**（用户尚未启动 0.4.0-beta 做端到端测试）

### 0.3.0-beta（2026-10-25）——大更新

- **包含条目**：BUG-001 后续修订、BUG-007、BUG-009、BUG-010、BUG-011、BUG-012、BUG-013
- **版本号**：内核 `0.2.0-beta` → **`0.3.0-beta`**；协议 `0.2.1-beta` → **`0.3.0-beta`**（用户 2026-10-25 决定：这是大更新，两者同时升级）
- **变更摘要**
  - **BUG-001 修订**：空容器不再中止编译；按 0 参与编译，并向触发编译的玩家发黄色 `Redpiler compile warning: comparator at (x, y, z) reads an empty <kind>`（自动编译静默）
  - **BUG-007**：`coalesce` 补齐 `dest.state == into.state` 与 `graph[edge_idx].ty == LinkType::Default`，与上游一致
  - **BUG-009**：`update()` 的「无人不 tick」门禁解除，无人时电路照常 tick、采样、推送
  - **BUG-010**：只保留 p0,0；删除按需加载地皮的整条路径；玩家越界不再被踢出（空岛模型）；登录/重连位置在岛外时传送到 p0,0 中心
  - **BUG-011**：编译 panic → `crash_save()` 保存地皮 → 打印 `[CRASH] ...` → `exit(1)`
  - **BUG-012**：新增 `/` 命令行，协议升 `0.3.0-beta`；脚本可直接执行白名单命令（等同 OP、作用于 p0,0、只回 `OK CMD` / `ERR CMD <CODE>`）
  - **BUG-013**：自动化端口绑定失败与非法端口回退都会打日志
- **文件清单**：`Cargo.toml`、`crates/core/src/{server.rs, automation.rs, plot/mod.rs, plot/commands.rs}`、`crates/redpiler/src/{task_monitor.rs, passes/frontend/identify_nodes.rs, passes/opt/coalesce.rs}`、`src/protocol.txt`、`src/protocol_zh.txt`、`README.md`、`README_zh.md`
- **回滚点**：`~/.ssset/backup/code-before-0.3.0/`（按仓库相对路径存放改动前的原文件，共 12 个）
- **编译结果**：✅ `cargo build --release` **通过**（2026-10-25，1m44s，exit 0，产物 `target/release/mchprs` 8.3 MB）
  - 编译期修正 **1 处**：`plot/mod.rs` 的 `let monitor = Default::default()` 因改成 `monitor.clone()` 后无法推断类型（E0282），已显式标注 `Arc<TaskMonitor>`
  - 警告 **2 条，均为改动前既有**：`mchprs_redpiler` 的 `AnalysisUsage::reset` 未使用；`mchprs_core` 的 `plot/monitor.rs:181` 用了 deprecated 的 `fetch_update`
  - 改动涉及的 7 个文件**零警告**
- **文档更新（2026-10-25）**
  - `README.md` / `README_zh.md`：新增「Runtime Model / 运行模型（0.3.0-beta）」整章（单地皮、无玩家运行、脚本命令、崩溃处理、空容器）；特性列表新增 4 条；`auto_redpiler` 与 `automation_port` 两行配置说明补上触发条件与失败告警；常用命令下补「哪些命令可由脚本发送」与「单地皮下跨地皮命令无效」的提示
  - `src/protocol.txt` / `src/protocol_zh.txt`：配置段补充非法端口/绑定失败的告警行为；新增 **第 11 节「Runtime model / 运行模型」**
- **验证状态**：✅ 编译通过；⏳ **未运行验证**

### 0.2.2-beta（已并入 0.3.0-beta，不再单独发布）

- **包含条目**：BUG-008
- **版本号**：内核保持 `0.2.0-beta` 不变；协议 `0.2.1-beta` → `0.2.2-beta`
- **说明**：本次改动**不触及自动化协议本身**，版本号提升仅作发布标记（与 0.2.1-beta 的处理方式一致）
- **落地时需改**：`crates/core/src/automation.rs` 的 HELLO 串、`src/protocol.txt`、`src/protocol_zh.txt`、`README.md`、`README_zh.md`
- **验证状态**：⏳ 端到端未验证（用户指示暂不做）

### 0.2.1-beta（2026-09-30）

- **包含条目**：BUG-005（删除整个白名单功能）
- **版本号**：内核不变；协议 `0.2.0-beta` → `0.2.1-beta`
- **变更摘要**
  - 功能移除：白名单整体删除（`profile.rs` 整文件、`/whitelist` 命令、相关消息/结构体/字段、`whitelist.json` 读写、登录校验、`config.rs` 字段）
  - 依赖移除：`reqwest` + `rustls` 整体删除（全仓唯一一处 HTTPS 调用随之消失）
  - 命令树：仅移除根 children 里的 `30`（甲方案，节点 30–33 成为游离节点）
  - 文档：`Config.toml` 与两份 README 配置表移除 `whitelist` 行
- **行为后果**：不再有任何访问控制
- **验证状态**：⏳ 待用户编译验证

### 0.2.0-beta（2026-09-30）

- **包含条目**：BUG-001、BUG-002、BUG-003、BUG-004
- **版本号**：内核 `0.1.0-beta` → `0.2.0-beta`；协议 `0.1.2-beta` → `0.2.0-beta`（两者同步）
- **变更摘要**
  - BUG-001 + BUG-004：`comparator.rs` 补 `WaterCauldron`、新增 `OverrideError`、拆分 `try_get_override` / `get_override`、删除 `eprintln!`；`task_monitor.rs` 错误槽；`lib.rs` 新增 `CompileError` 与 `Result` 返回；`identify_nodes` 返回 `Result`；`run_passes` 提前返回；`plot/mod.rs` 分级处理结果 + 修掉"无条件设 Running"
  - BUG-002：`clientbound.rs` 新增 `GameMode` parser 变体；命令树新增节点 53–56 并挂根；39/40 补进 `/plot` children
  - BUG-003：位序号改 `j + k*ny + i*ny*nz`、循环改 `for i { for k { for j } }`、`dy` 校验改 `ny>1 → dy<0`；两份协议文档与 `test.py` 同步（原点、`dy`、位映射，移除 `_REV8` / `_REV12`）
  - 文档：两份 README 协议版本号同步
- **验证状态**：⏳ 待用户编译验证（当时按铁律未执行任何 cargo 操作）
- **注意**：BUG-003 是破坏性协议变更，依赖旧位序的外部脚本必须按新规则改写

---

## 8. 上游溯源（MCHPR/MCHPRS）

**对比基线**：上游 `master` @ `d492e43`，同为 Minecraft 1.20.4 / 协议 765。本分支历史被压成 `Initial commit`，无法从 git 追溯基点，故以当前上游 `master` 为准。

- Android 侧上游对照副本：`~/mchprs/source/MCHPRS`（只读比对用）
- 重新获取（Windows）：`& 'C:\Program Files\Git\cmd\git.exe' clone --depth 1 --single-branch --branch master https://github.com/MCHPR/MCHPRS.git "$env:TEMP\MCHPRS-upstream"`
- 重新获取（Android）：`git clone --depth 1 --single-branch --branch master https://github.com/MCHPR/MCHPRS.git ~/mchprs/source/MCHPRS`

> ⚠️ **基线不是完全一致**：本分支的红石分析层原本**落后**于该基线（旧区间版 vs 上游位集版），BUG-008 才补齐。对比时若发现分析相关代码差异，先确认是不是这一层。

**用户报的问题，上游是否存在**

| 条目 | 上游 `master` 的实现 | 归属 |
| --- | --- | --- |
| BUG-001 比较器 | `has_override` 含 `Cauldron`、不含 `WaterCauldron`；`get_override` 兜底 `unreachable!()` | **上游固有** |
| BUG-004 编译崩溃 | 同款调用 → 同样 panic；上游**无**优雅中止 | **上游固有**；优雅中止为本分支新增 |
| BUG-002 Tab 补全 | 上游命令树同样无 gamemode 节点；`/plot` children 同样漏掉 39/40 | **上游固有**（两处） |
| BUG-005 whitelist | `profile.rs` 与本分支修改前逐字相同，同样只写 `debug!` | **上游固有** |
| BUG-003 数据端口位序 | 上游**没有** `automation.rs` 与 `src/protocol.txt` | **本分支独有设计** |
| BUG-008 分析层 | 上游已是 `SSSetAnalysis` + pending tick | **本分支补齐对齐** |

**⚠️ 有意偏离上游之处（对比时勿误判为改错）**

- **BUG-001 的语义**：本分支把空锅 / 空容器 / 空堆肥桶改判为**错误并中止编译**；上游把它们当**合法值 0** 返回。这是用户明确要求的行为差异
- 本分支新增：`Compiler::compile` 返回 `Result`、失败时置 `Stopped` 并向玩家报坐标、整套自动化协议、以及 pending tick 在编译图里的落地方式
- 上游 `start_redpiler` 同样**无条件**设 `Running`，本分支已顺手修掉
