# angelfish
Antimatter Powered Server Core For Minecraft

# 架构
## 综述
angelfish分为browser和server两部分。browser将不同的客户端协议（bedrock和java）转码为类似于websocket的spp协议，和server沟通；server处理spp协议，实现游戏逻辑。spp协议基于url，实现动态资源（地图运行时图层）的调配。另外，browser还通过spp协议，从静态资源服务器上下载toml格式的静态资源表（类似于html网页），实现静态资源（资源包材质包、静态地图）的调配；spp协议的通讯地址通过静态资源表传输。

## 登录流程
C代表客户端，B代表Browser，S代表Server。
事实上表格内顺序无关的消息是并发处理的。

### 预处理
| 时间顺序 | 处理         | 内容                               |
| ---- | ---------- | -------------------------------- |
| 1    | B->S spp协议 | 我要所有的静态页面和地图，我已有的资源url有这些，哈希是这样的 |
| 2    | S->B spp协议 | 传输哈希有变化的或者浏览器没有的所有资源             |

### 客户端登录
| 时间顺序 | 处理         | 内容                                       |
| ---- | ---------- | ---------------------------------------- |
| 1    | C->B 多种协议  | 我要登录play.birs.tech:19132                 |
| 2    | B          | 读取设置，发现play.birs.tech:19132对应spp://localhost |
| 3    | B          | 访问缓存，找spp://localhost/index.toml         |
| 4    | B          | 读取index.toml，发现需要连接到动态地图spp://localhost/static/example_world |
| 5    | B->S spp协议 | 我要连接到地图spp://localhost/static/example_world |
| 6    | S->B spp协议 | 打开管道，开始传输视界内地图区块                         |
| 6    | B->C 多种协议  | 打开管道传输地图，同时B缓存地图                         |
| 7    | B->C 多种协议  | 传输完成，游戏开始                                |
| 7    | B          | 读取index.toml，发现在on_join时需要发送一个提示信息       |
| 8    | B->C 多种协议  | 发送提示信息：欢迎进入游戏！                           |

### 放置并点燃TNT

| 时间顺序 | 处理         | 内容                           |
| ---- | ---------- | ---------------------------- |
| 1    | C->B 多种协议  | 控制器尝试在pos位置放置方块              |
| 2    | B->S spp协议 | 客户端client尝试放置方块到某位置          |
| 3    | S          | 反作弊、权限等系统计算并处理，发现客户端可以放置这个方块 |
| 4    | S->B spp协议 | 发送方块放置数据报                    |
| 5    | B->C 多种协议  | 给可以看见方块[1]的玩家发送方块放置数据报       |

| 时间顺序 | 处理         | 内容                                    |
| ---- | ---------- | ------------------------------------- |
| 1    | C->B 多种协议  | 控制器与pos位置的方块互动                        |
| 2    | B->S spp协议 | 客户端client尝试与pos位置方块互动                 |
| 3    | S          | 处理计算，发现client手中拿的是打火石，pos位置方块是TNT     |
| 4    | S->B spp协议 | 对可以看见方块的玩家，发送生成TNT实体数据报，并更新实体过期（消失）时间 |
| 5    | B->C 多种协议  | 分发TNT实体生成数据报到客户端                      |
| 6    | S          | 时间到之后，计算TNT需要破坏哪些方块                   |
| 7    | S->B spp协议 | 发送对应的方块破坏和物品掉落                        |
| 8    | B->C 多种协议  | 分发TNT实体消失数据报到客户端                      |
| 8    | B->C 多种协议  | 分发方块和物品数据报到客户端                        |

[1] 由browser计算玩家视距，决定是否发送

### 例程

```rust
use angelfish::prelude::*;

server::new()
    .route("/", homepageHandle)
    .route("/query", pqHandle.query)
    .route("/ping", pqHandle.ping)
    .spp("localhost")
    .unwrap();

browser::new()
    .homepage(ReqType::Join, "spp://localhost/")
    .homepage(ReqType::Query, "spp://localhost/query")
    .homepage(ReqType::OfflinePing, "spp://localhost/ping")
    .adapt_mc_bedrock("localhost:19132")
    .adapt_mc_java("localhost:25565") // only for example
    .unwrap();
```