# Tech Pulse v2: 产品重新设计方案

## 愿景

**Tech Pulse 是一个能学习你知道什么、并主动展示你遗漏了什么的科技新闻产品。**

所有现有产品（Feedly、HN、Google News）都在强化你已有的阅读习惯。Tech Pulse 是唯一一个会主动告诉你"科技圈正在发生你不知道的事"并帮你填补盲区的产品。它是一个**反信息茧房**产品。

---

## 架构升级：引入后端

后端做三件客户端做不到的事：
1. **持续数据采集** — 定时任务每 30 分钟抓取所有来源，消除 CORS 代理依赖，获得统计学上有意义的趋势数据
2. **跨用户聚合** — 基于数千篇文章做趋势检测，而非每次用户访问时的 150 篇
3. **持久化用户画像** — 不再依赖 localStorage，支持跨设备

**技术栈**: Node/Express + SQLite（Express 已在 package.json 中）

---

## 四个互联视图（不再是孤岛）

```
               +---------------------+
               |     知识画像         |
               |   （共享中枢）       |
               +---------------------+
              /    |         |        \
         今日   探索     知识盲区    时间线
```

每个视图都从知识画像**读取**和**写入**数据。这是与当前设计最根本的区别——当前的 Feed、盲区、时间线是三个互不关联的独立小应用。

### 视图 1: 今日（每日简报） — 休闲用户默认页
- "自昨天下午 6:23 以来有 42 条新消息"
- 排名最高的 5 条新闻
- 涨幅最大的 3 个趋势话题
- 1-2 条盲区提醒："本周 [安全] 领域有 12 条新闻你没看过"
- 设计目标：2 分钟内看完

### 视图 2: 探索（智能信息流） — 重度用户主界面
- 文章按智能算法排序（质量 + 时效 + 趋势 + 盲区加权）
- 来源/分类过滤器、搜索、趋势栏
- 每隔 10-15 条文章插入盲区推荐卡片
- 文章卡片上标记"你的新话题"
- "上次访问以来的新内容"分隔线

### 视图 3: 知识盲区（知识画像） — 核心差异化功能
- 雷达图/蛛网图展示所有技术领域覆盖度
- 按严重程度排列的盲区列表（领域活跃度高 + 你的覆盖度低 = 严重盲区）
- 每个盲区都有"去探索"按钮，点击跳转到该话题的过滤信息流
- 优势区域展示（正向激励）
- 手动标记话题（"我已了解" / "想学习"）

### 视图 4: 时间线 — 与所有功能联动
- 自动从排名前 0.5% 的高影响力文章中填充里程碑
- 在探索视图中可手动置顶到时间线
- 可按技术领域过滤
- 高亮你的盲区领域中的里程碑："这件事发生时你没在关注 [安全] 领域"

---

## 智能排序算法

```
最终分数(文章, 用户) =
    0.35 * 百分位分数          // 跨来源公平比较
  + 0.20 * 时效衰减            // 6 小时半衰期
  + 0.15 * 趋势动量            // 文章标签的 z-score
  + 0.20 * 盲区加权            // 用户薄弱领域的文章分数更高
  + 0.10 * 来源多样性奖励      // 防止 HN 一家独大
```

**基于百分位的归一化**取代现有的各来源临时公式：
- 后端维护每个来源 7 天内的分数分布
- "HN 排名前 5%" 和 "Reddit 排名前 5%" 真正具有可比性
- ArXiv/Product Hunt 等缺少互动指标的来源使用代理评分（时效性 + 趋势关键词重叠度）

---

## 趋势检测（统计学上有意义）

后端驱动，非客户端：
- 6 小时为窗口（而非按天），实现更精细的动量检测
- 基于 Z-score 的检测：`z = (当前频率 - 基准频率) / 标准差`
- 满足 z > 2.0 且绝对出现次数 > 5 才算"趋势"
- 14 天基准线（比现有 7 天更稳定）
- "常驻高频"排除名单（避免"javascript"永远显示为趋势）

---

## 知识画像（核心引擎）

### 多信号追踪（不再只追踪点击）

| 信号 | 权重 | 含义 |
|------|------|------|
| 点击（打开链接） | 0.3 | 表示有兴趣 |
| 页面停留 > 30 秒 | 0.7 | 真正阅读了 |
| 展开摘要 | 0.2 | 进一步探索 |
| 手动标记"已了解" | 1.0 | 用户自报 |
| 手动标记"想学习" | 0.5 | 用户自报 |
| 划走/不感兴趣 | -0.3 | 负面信号 |

### 动态话题分类体系（不再硬编码 7 个类别）

- 第一层（领域）：约 12 个人工策划的大领域（AI/ML、Web 开发、系统/基础设施、安全、移动端、数据/分析、DevOps/云、硬件、编程语言、区块链、科研、产品/创业）
- 第二层（话题）：动态生成——任何在 >N 篇文章中出现的关键词自动成为话题
- 第二层到第一层的映射通过扩展的 keyword_engine 别名系统实现

### 盲区严重度计算

```
严重度(领域) = 全局活跃度(领域) * (1 - 用户覆盖度(领域))
```
- 全局活跃度高 + 用户覆盖度低 = 严重盲区（大量动态你在错过）
- 全局活跃度低 + 用户覆盖度低 = 轻微盲区（本来就没什么大事）

### 反馈闭环（让一切生效的关键）

盲区数据不只是被展示——它**驱动整个产品体验**：
- 今日视图：简报中包含盲区提醒
- 探索视图：排序算法中盲区加权把你薄弱领域的文章推得更高
- 时间线：高亮你薄弱领域的里程碑事件
- 信息流：每 10-15 条文章插入一张盲区推荐卡

---

## 后端 API

```
POST /api/auth/token           → 匿名设备令牌（无需注册登录）
GET  /api/feed                 → 为该用户个性化排序的文章列表
GET  /api/trends               → 当前趋势关键词
GET  /api/briefing             → 每日简报数据
GET  /api/profile              → 用户知识画像
POST /api/interactions         → 批量上报交互事件
GET  /api/timeline             → 里程碑（自动 + 手动置顶）
POST /api/timeline/pin         → 将文章置顶到时间线
```

### 服务端结构
```
server/
  index.js                      — Express 应用 + 路由注册
  services/
    article_collector.js        — 定时任务：每 30 分钟抓取所有来源
    trend_detector.js           — 定时任务：计算 z-score 趋势
    ranking_engine.js           — FinalScore 排序计算
    profile_engine.js           — 从交互事件计算熟悉度分数
    milestone_detector.js       — 自动检测重大事件
  adapters/                     — 复用现有适配器（不再需要 CORS 代理）
  infrastructure/
    db.js                       — SQLite 连接 + 数据库迁移
    keyword_engine.js           — 与客户端共享
```

---

## 数据库表结构

```sql
articles（文章表）        — id, title, url, source, raw_score, normalized_score, tags, summary, published_at
keyword_windows（关键词窗口） — keyword, window_start（6小时窗口）, count, sources
trends_cache（趋势缓存）    — keyword, z_score, count, growth_rate, detected_at
user_profiles（用户档案）   — user_token, last_visit_at
user_interactions（交互记录）— user_token, article_id, type, timestamp, duration_seconds
user_topic_familiarity（话题熟悉度）— user_token, topic, domain, familiarity_score, last_interaction_at
milestones（里程碑）        — id, article_id, title, date, domain, is_auto, user_token
```

---

## 关键交互设计

1. **知识脉搏环** — 页面顶栏始终可见的 SVG 圆环，显示整体知识覆盖率百分比。阅读盲区内容时圆环会轻微填充，形成即时反馈。
2. **盲区推荐卡** — 在信息流中每 10-15 条文章插入一张："本周有 12 条 WebAssembly 相关文章，你看了 0 条。[去看看] [暂时不用]"
3. **"你的新话题"标签** — 文章卡片上的标签如果对应熟悉度 < 20 的话题，会有发光边框和"NEW"标记
4. **滑动操作**（移动端） — 右滑："我了解这个"，左滑："不感兴趣"
5. **"上次访问后的新内容"分隔线** — 在探索视图中清晰标记新旧分界

---

## 中文可读性层（自动翻译）

目标：**让非中文来源不再是“纯英文阅读负担”**，而是默认呈现为可读的中文信息流。

### 产品体验
- 全局开关："中文优先 / Auto‑Translate"（默认关闭，用户显式开启）
- 文章卡片默认展示：**翻译后的标题 + 翻译后的摘要**
- 单条可切换："查看原文" / "显示中文"
- 翻译失败或不可用时自动回退为原文，并提示轻微状态标记（不打断阅读）

### 翻译触发逻辑
1. 先做语言检测（title + summary）
2. 非中文且翻译开启 → 进入翻译管线
3. 缓存命中直接返回；否则请求翻译服务

### 基础设施与成本控制
- 翻译在**服务端**完成，避免暴露 API Key
- 支持多 Provider（如 LibreTranslate / DeepL / OpenAI），用户可配置 Key
- 对每篇文章的 title+summary 做缓存（按 article_id + language + provider）
- 限制最大字符数与并发，降低成本与延迟

### 数据模型（概念层）
- `translated_title`
- `translated_summary`
- `translation_provider`
- `translation_status`（ok / failed / pending）

---

## 迁移路径（4 个阶段）

### 阶段 0: 后端基础
- 搭建 Express 服务器 + SQLite 数据库表
- 将现有适配器迁移到服务端（消除 corsproxy.io 依赖）
- 定时任务持续抓取数据
- GET /api/feed → 客户端改为从后端获取数据
- **涉及文件**: 新建 `server/` 目录，修改 `src/services/aggregator.js` 为 API 客户端

### 阶段 1: 趋势检测升级
- 采集器填充 keyword_windows 表
- Z-score 趋势检测定时任务
- GET /api/trends → 客户端 TrendingBar 从 API 读取数据
- **涉及文件**: 新建 `server/services/trend_detector.js`，修改 `src/components/TrendingBar.js`

### 阶段 2: 用户身份与交互追踪
- 匿名设备令牌（无需注册）
- 客户端捕获多信号交互（点击、阅读时长、展开摘要、划走）
- POST /api/interactions → profile_engine 计算熟悉度
- **涉及文件**: 新建 `src/services/interaction_tracker.js`，新建 `server/services/profile_engine.js`

### 阶段 3: 智能排序与盲区整合
- 带盲区加权的 FinalScore 排序
- 百分位归一化
- 新建今日/简报视图
- 重写知识盲区视图（雷达图、动态话题、反馈闭环）
- 重写信息流视图（推荐卡、"你的新话题"标签、"上次访问后的新内容"）
- **涉及文件**: 重写 `src/views/feed.js`、`src/views/blindspots.js`，新建 `src/views/today.js`

### 阶段 4: 联动时间线与打磨
- 自动里程碑检测
- 时间线与盲区联动
- 知识脉搏环指示器
- 移动端滑动操作
- 错误处理、性能优化、无障碍访问
- **涉及文件**: 重写 `src/views/timeline.js`，修改 `src/main.js`

---

## 保留与变更对照表

| 模块 | 决策 |
|------|------|
| `src/models/story.js` | 保留，新增 `isNew` + `blindSpotRelevance` 字段 |
| `src/infrastructure/keyword_engine.js` | 保留，客户端与服务端共享 |
| `src/infrastructure/storage.js` | 保留，仅用于离线缓存 |
| `src/adapters/*.js` | 迁移到服务端，移除 CORS hack |
| `src/components/NewsCard.js` | 修改：新增盲区指示器、"你的新话题"标签 |
| `src/components/TrendingBar.js` | 修改：改为从 API 读取 |
| `src/services/aggregator.js` | 重写：从适配器编排器变为轻量 API 客户端 |
| `src/services/trend_engine.js` | 迁移到服务端 |
| `src/services/knowledge_store.js` | 重写：变为向 API 上报交互事件的追踪器 |
| `src/views/feed.js` | 大幅重写：智能信息流 + 推荐卡 |
| `src/views/blindspots.js` | 完全重写 |
| `src/views/timeline.js` | 重写：自动填充、与盲区联动 |
| `src/main.js` | 新增今日视图、知识脉搏环、设备令牌管理 |
