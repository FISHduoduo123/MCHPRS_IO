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
| **0.3.0-beta** | 0.2.0-beta → **0.3.0-beta** | 0.2.1-beta → **0.3.0-beta** | BUG-001 修订 / 007 / 009 / 010 / 011 / 012 / 013 | ✅ 已实施，**`cargo build --release` 通过**，⏳ 待运行验证 |
| 0.2.2-beta | — | — | BUG-008 | ⚠️ **已并入 0.3.0-beta，不再单独发布**（原拟定未落地） |
| 0.2.1-beta | 0.2.0-beta（不变） | 0.2.0-beta → 0.2.1-beta | BUG-005 | ✅ 已实施，待运行验证 |
| 0.2.0-beta | 0.1.0-beta → 0.2.0-beta | 0.1.2-beta → 0.2.0-beta | BUG-001~004 | ✅ 已实施，待运行验证 |

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
