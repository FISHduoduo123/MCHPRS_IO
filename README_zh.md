# MCHPRS_IO

[MCHPRS](https://github.com/MCHPR/MCHPRS) 的衍生版——一个为计算红石打造的 Minecraft 1.20.4 创造模式多线程服务器，新增 TCP 明文自动化协议，并附带一套开箱即跑的演示程序。

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 衍生版声明

本项目是 [MCHPRS](https://github.com/MCHPR/MCHPRS) 的**衍生版（fork）**。

- 原项目地址：<https://github.com/MCHPR/MCHPRS>
- 原项目协议：**MIT** — Copyright (c) 2020 Ojas Landge
- 本衍生版同样采用 **MIT 协议**（见 [LICENSE](./LICENSE)）。

## 简介

MCHPRS_IO 保留了 MCHPRS 面向计算红石的全部核心能力：

- 地块在独立线程上运行——更少的卡顿、更高的并发。本衍生版只运行 `p0,0` 一块
  512x512 地皮，详见下文「运行模型」。
- [Redpiler](docs/Redpiler.md)（"红石编译器"）把红石电路编译为高速仿真。
- 一套基于 TCP 的**明文自动化协议**（默认端口 25585），让外部程序可以直接读取、驱动红石机器。

仓库根目录的 `MCHPRS_IO/` 文件夹就是这套协议的完整演示，几分钟即可看到运行结果。

## 特性

- Minecraft 1.20.4 创造模式多线程红石服务器
- Redpiler 红石编译器
- 自动化协议（TCP 25585 端口，明文，协议版本 0.4.0-beta）：
  - `PIN` / `POUT` 数据端口，把红石区域以十六进制值对外暴露；十六进制表示
    是整条位向量当作一个整数、在前面补零（0.4.0-beta 起变更）
  - `INPUT` / `OUTPUT` 读写端口数值
  - `EVENT` 推送，把变化通知给订阅方
  - 端口名大小写敏感，按注册名原样推送
  - `/` 开头的数据行可让脚本在服务器唯一的地皮上执行受支持的玩家命令
    （例如 `/redpiler compile`），无需玩家在线
- **无人值守**：没有玩家在线时电路照常运行，适合脚本挂机跑电路
- **单地皮 + 空岛模型**：只保留 `p0,0`，其余地皮永不加载
- **编译崩溃可控**：先保存地皮存档、打印 `[CRASH]`，再以退出码 1 退出
- **空容器不中止编译**：按 0 处理，并把全部位置合并成一条红色警告
- **文件存储器**：`memory/*.rom.txt` 只读 ROM，`memory/*.ram.txt` 可写 RAM
  （Logisim `v3.0 hex words addressed`），由红石地址线、时钟线与使能线驱动
- WorldEdit 风格建筑指令、Velocity 转发、LuckPerms 支持

## 运行模型（0.4.0-beta）

### 只有一块地皮

服务器只保留出生点 `p0,0`（512x512）。其它地皮**永不加载**，玩家也无法进入。
走到 `p0,0` 之外不会被拉回：外面是空的（空岛模型），可以自由飘出去，回头仍能
看见岛。登录或重连时若存档位置在岛外，会被传送到 `p0,0` 中心。

### 无玩家也运行

红石模拟、redpiler 刷新与自动化采样都**不再依赖在线玩家**。没有客户端连接时，
`p0,0` 依旧持续 tick，`EVENT` / `OUTPUT` 照常推送，`PIN` / `INPUT` / `SUB`
照常应答。配合下面的 `/` 命令，脚本可以完全无人值守地驱动电路。

### 脚本可以执行命令

自动化连接发送以 `/` 开头的整行数据，服务器会像玩家敲命令一样执行它。自动化
调用方没有玩家实体，因此**等同 OP 权限**、命令**固定作用于 `p0,0`**、相对坐标
（`~` / `^`）不被接受。支持的命令：

| 命令 | 说明 |
| --- | --- |
| `/redpiler compile [参数]` | 触发编译 —— 无人在线时脚本自行启动电路的关键 |
| `/redpiler reset` | 重置电路 |
| `/radvance <ticks>` | 手动推进 tick |
| `/toggleautorp` | 切换自动编译 |
| `/worldsendrate <hz>` | 设置方块更新频率 |
| `/memory reload` | 从磁盘重读 `memory/*.rom.txt` |
| `/stop` | 关闭服务器 |

成功回 `OK CMD`；失败回 `ERR CMD <INACTIVE|UNKNOWN|ARG>`。命令自身的聊天输出
**不回传**。其余命令（`/plot`、`/gamemode`、`//load` 等）一律 `ERR CMD UNKNOWN`。

### 崩溃处理

redpiler 编译过程中发生不可恢复错误时，服务器会**先保存该地皮存档**，在终端
打印一行 `[CRASH]`（含时间戳、地皮坐标、错误信息、存档路径、退出码）：

```text
[CRASH] plot (0, 0) redpiler panic: <错误信息> — saved to ./world/plots/p0,0 — exiting with code 1
```

然后以退出码 **1** 终止进程。

### 空容器

比较器读到空容器（空锅 / 空木桶 / 空熔炉 / 空漏斗 / 空堆肥桶）**不再中止编译**：
该方块按 0 参与编译，并在触发编译的玩家聊天窗里发**一条红色警告**，列出全部受
影响的位置，例如：

```text
Redpiler compile warning: comparator at (143,89,367),(126,69,127) reads some empty containers
```

只列前 10 个位置，后面接 `,(...)`。服务器没有规则的方块（`Unsupported`）仍会
中止编译。

### 内存文件（ROM 与 RAM）

存储器用文本文件定义，放在与 `Config.toml` 同级的 `memory/` 目录里。
`*.rom.txt` 是只读的 ROM，`*.ram.txt` 是可写的 RAM；启动时加载该目录下所有
文件。

```text
v0.4.1-beta hex words addressed
origin = 0
clk = 0
address coordinate = [
-8, 64, 0
-7, 64, 0
]
data coordinate = [
2, 64, 0
3, 64, 0
]
00: 0f 3c 69 96
```

- 首行必须与当前内核版本一致，否则整份文件被拒载。
- `clk = 0` 表示**没有时钟**：地址值一变就重写数据总线。`clk = x, y, z`
  表示该元件的每个上升沿读一次。
- `origin = 0`（或 `0, 0, 0`）表示坐标就是世界绝对坐标；其它三元组会加到
  所有坐标上，`clk` 也算。键的顺序任意。
- `address coordinate` 必须是红石灯 / 活板门 / 音符盒；`data coordinate`
  必须是拉杆 / 踩踏板。两组列表的第一个元素都是**最高位**，位宽不要求是 4 的
  倍数。
- 数据区是 Logisim 的 `v3.0 hex words addressed` 格式：`地址: word word ...`。
  word 之间用空格或换行分隔，`#` / `//` / `/* */` 注释处处可用，未列出的地址
  读作 0，比数据总线宽的 word 只取低位。
- 每次编译成功后逐条校验：坐标解析不出的那一条被禁用并给出红色聊天提示，其余
  照常工作。读写都在 tick 之后执行，因此固定有 **1 tick 延迟**。
- data 拉杆与 address / 时钟元件**面贴面相邻**会被拒载：那等于用自己的输出去
  驱动自己读的信号（1 tick 反馈环）。经红石线的间接耦合不检测。
- 多条存储器可以共用地址线与数据线；共用**时钟**元件则两块一起失效。
- `/memory reload` 无需重启即可重读 `memory/`，玩家和脚本（`/` 行）都能发。

### 可写存储器（`*.ram.txt`）

RAM 的内容保存在服务器内存里，并写回同一个文件。

```text
v0.4.1-beta hex words addressed
origin = 0
clk = 12, 37, 76                  // 必须存在，同时驱动读和写
write enable = 43, 46, 25
read enable = 81, 66, 48
byte write enable = [             // 空列表 = 整字写入
26, 67, 94
26, 67, 97
]
address coordinate = [
258, 10, 263
258, 8, 263
]
write data coordinate = [         // 电路驱动（灯）
256, 14, 264
256, 12, 264
]
read data coordinate = [          // 服务器驱动（拉杆）
256, 34, 264
256, 32, 264
]
00: f b
```

- 时钟必须存在，同时驱动读和写；所有信号在上升沿锁存，因此读写都有 1 tick
  延迟。
- `write data coordinate` 与 `read data coordinate` 必须是**两组不同的方块**，
  位宽必须相等。
- 字节写使能从最高位开始每 8 位一组：共 `ceil(N/8)` 条，第 1 条管最高组，
  **最后一条永远管低 8 位**。使能为 1 的组写入新值，为 0 的组保持原值。
- 读优先：同一个沿上读写使能同时有效时，读数据线输出**旧值**，写入照常进行。
- 内容会写回同一个文件（保留头部与注释，首行更新为当前版本）：每 2 秒或每
  4096 次写入落盘一次，`/stop`、编译崩溃、`/memory reload` 之前强制落盘。
- 读数据拉杆不能被电路驱动，否则会和存储器互相覆盖；校验时会检查。

## 快速开始

构建服务器：

```shell
cargo build --release
```

优化后的可执行文件位于 `./target/release/mchprs`。

使用演示配置启动服务器（绑定 `127.0.0.1:25565`，自动化端口 `25585`）：

```shell
cd MCHPRS_IO
/path/to/target/release/mchprs
```

在第二个终端里运行演示客户端：

```shell
python3 MCHPRS_IO/test.py
```

演示会通过自动化协议注册一个 8 位加法器（`PIN add_in` 输入端口、`POUT add_out` 输出端口）并逐组验证，你会看到类似输出：

```text
<- HELLO MCHPRS AUTOMATION 0.4.0-beta
[25] passed=25 failed=0 rate=10/s
...
done. total=500 passed=500 failed=0 elapsed=50.1s rate=10/s
```

把 `test.py` 顶部的 `VERBOSE = True` 打开，可以看到每一行协议交互，例如：

```text
-> INPUT add_in a0ab
<- OK INPUT add_in
<- OUTPUT add_out df0
```

### 演示文件夹

| 路径 | 用途 |
| --- | --- |
| `MCHPRS_IO/Config.toml` | 演示服务器配置（自动化端口 25585） |
| `MCHPRS_IO/test.py` | 演示客户端——仅依赖 Python 3 标准库 |
| `MCHPRS_IO/schems/cca_test.schem` | 演示用的 8 位加法器原理图 |
| `MCHPRS_IO/world/` | 预置好的演示世界 |

### 演示测试模式

修改 `test.py` 顶部的 `TEST_MODE`：

- `EXHAUSTIVE` — 穷举全部输入组合（256 x 256 x 2 = 131,072 组）
- `RANDOM` — 500 组固定种子的随机向量（默认）
- `EDGE` — 边界用例（0x00、0x01、0x7F、0x80、0xFF 等）

## 自动化协议

完整协议规范：

- 英文：[src/protocol.txt](src/protocol.txt)
- 中文：[src/protocol_zh.txt](src/protocol_zh.txt)

## 配置

服务器首次启动时会在工作目录生成 `Config.toml`。

| 字段 | 说明 | 默认值 |
| --- | --- | --- |
| `bind_address` | 绑定地址与端口 | `0.0.0.0:25565` |
| `motd` | 服务器 MOTD | `"Minecraft High Performance Redstone Server"` |
| `chat_format` | 聊天格式，使用 `{username}` / `{message}` | `<{username}> {message}` |
| `max_players` | 最大同时在线人数 | `99999` |
| `view_distance` | 玩家周围加载区块的最大距离（区块） | `8` |
| `schemati` | 模拟 ORE Schemati 插件的目录布局 | `false` |
| `block_in_hitbox` | 允许把方块放在玩家内部 | `true` |
| `auto_redpiler` | 自动编译（**仅当 rtps 为 `unlimited` 或地皮卡顿时才触发**；无人值守建议改用脚本发 `/redpiler compile`） | `false` |
| `automation_port` | 自动化协议 TCP 端口（非法值回退 `25585` 并打印警告；绑定失败会打印含原因的警告并禁用自动化） | `25585` |
| `automation_data_port_events` | 数据端口变化时推送 `EVENT` 行 | `false` |
| `automation_per_tick_push` | 每 tick 采样一次数据端口 | `false` |

支持 Velocity 转发与 LuckPerms，配置方法见原项目 README。

## 常用命令

| 命令 | 别名 | 说明 |
| --- | --- | --- |
| `/rtps [rtps\|unlimited]` | — | 设置红石刻/秒（默认 10） |
| `/radvance <ticks>` | `/radv` | 将地块推进 `<ticks>` 个红石刻 |
| `/redpiler compile` | `/rp c` | 开始 redpiler 编译 |
| `/redpiler reset` | `/rp r` | 停止 redpiler |
| `/toggleautorp` | — | 切换自动 redpiler 编译 |
| `/teleport <x> <y> <z>` | `/tp` | 传送（支持相对坐标） |
| `/speed <speed>` | — | 设置飞行速度 |
| `/gamemode <mode>` | `/gmc`、`/gmsp` | 切换游戏模式 |
| `/plot claim` | `/p c` | 认领所在的地块 |
| `/plot auto` | `/p a` | 自动寻找并认领空闲地块 |
| `/memory reload` | — | 从 `memory/` 重读内存定义 |
| `/stop` | — | 停止服务器 |

其中 `/redpiler compile`、`/redpiler reset`、`/radvance`、`/toggleautorp`、
`/worldsendrate`、`/memory reload`、`/stop` 也可以由自动化脚本以 `/` 开头的行
发送，无需玩家在线（见上文「运行模型」）。

> **注意**：本版只运行 `p0,0`，因此依赖「切换地块」的命令（`/plot auto`、
> `/plot visit`、`/plot claim` 等）不会把玩家送到别的地皮——那些地皮永不加载。
> `/teleport` 仍可把你移出岛外，但外面是空的。

地块、WorldEdit 与 Redpiler 的完整命令列表可在游戏内（`//help`、`/redpiler compile --help`）或原项目 README 中查阅。

## 许可证

本衍生版采用 **MIT 协议**——与原项目 MCHPRS 相同（Copyright (c) 2020 Ojas Landge）。详见 [LICENSE](./LICENSE)。

## 致谢

本项目是 [MCHPRS](https://github.com/MCHPR/MCHPRS)（作者 [StackDoubleFlow](https://github.com/StackDoubleFlow) 及贡献者）的衍生版。感谢上游作者：

- [@AL1L](https://github.com/AL1L)：WorldEdit 与其他特性的贡献。
- [@DavidGarland](https://github.com/DavidGarland)：更快的内存 `get_entry` 实现。
