# monadclaw

一个简约的个人 AI Agent 框架，模块化、可扩展，用 Rust 编写。
个人项目，不面向生产使用。

**其他语言：** [English](../../README.md) · [Français](README.fr.md)

---

## 快速开始

### 前置条件

- Rust（stable）— [rustup.rs](https://rustup.rs)
- Node.js 18+（用于 dashboard）
- 支持的 LLM 提供商 API key（例如 [OpenRouter](https://openrouter.ai/keys)）

### 1. 配置

在项目根目录创建 `config.toml`（或放置于 `~/.config/monadclaw/config.toml`）：

```toml
active_provider = "openrouter"

[providers.openrouter]
model = "openai/gpt-4o-mini"
api_key_env = "OPENROUTER_API_KEY"
base_url = "https://openrouter.ai/api/v1/"
```

在项目根目录创建 `.env` 文件：

```env
OPENROUTER_API_KEY=sk-or-v1-...
```

### 2. 启动后端

```bash
source .env && MONADCLAW_CONFIG=./config.toml cargo run
# 服务启动在 http://0.0.0.0:3000
```

### 3. 启动 Dashboard

```bash
cd dashboard
npm install
npm run dev
# Dashboard 地址：http://localhost:5173
```

---

## 认证

Monadclaw 使用**三级访问模型**，根据连接来源和是否设置了密码决定访问权限。

| 连接来源 | 是否设置密码 | 结果 |
|---------|------------|------|
| 本地（loopback） | 否 | ✅ 直接允许，无需凭证 |
| 本地（loopback） | 是 | 🔑 需要 Bearer token |
| 远程 | 否 | ❌ 403 Forbidden |
| 远程 | 是 | 🔑 需要 Bearer token |

### 设置密码

在 `config.toml` 中添加 `dashboard_password` 字段，然后重启后端：

```toml
dashboard_password = "你的密码"
```

前端会自动跳转到登录页面，输入密码后进入 Dashboard。
Token 存储在 `localStorage` 中，无过期时间。清除浏览器存储即可退出登录。

### 远程访问

未设置密码时，远程访问**默认被拒绝**——这是有意为之的安全机制。
如需开启远程访问，在配置中设置 `dashboard_password`。

> 详见 [docs/auth.md](../auth.md)

---

## 项目结构

```
monadclaw/
├── apps/server/        # 二进制入口（Axum HTTP 服务器）
├── crates/
│   ├── api/            # Axum 路由、handlers、中间件
│   ├── chat/           # 聊天消息类型
│   ├── config/         # TOML 配置加载
│   └── providers/      # LLM 提供商抽象（genai）
├── dashboard/          # React 19 + TypeScript Dashboard
├── docs/               # 内部规范与文档
└── config.toml         # 本地配置（已加入 .gitignore）
```

---

## 路线图

| 功能 | 状态 |
|------|------|
| TOML 配置加载 + 环境变量解析 | ✅ 已完成 |
| LLM 提供商抽象（genai） | ✅ 已完成 |
| OpenAI 兼容自定义 endpoint（OpenRouter、Kimi 等） | ✅ 已完成 |
| 流式聊天 API（`POST /api/v1/chat`） | ✅ 已完成 |
| 状态 API（`GET /api/v1/status`） | ✅ 已完成 |
| Axum HTTP 服务器 + CORS | ✅ 已完成 |
| React Dashboard — Shell、侧边栏、导航 | ✅ 已完成 |
| 聊天页面（流式响应） | ✅ 已完成 |
| 三级认证中间件 | ✅ 已完成 |
| Dashboard 登录页面 + 路由守卫 | ✅ 已完成 |
| Agent 循环（工具调用、多步推理） | 🔄 计划中 |
| 短期记忆（对话窗口） | 🔄 计划中 |
| 长期记忆（持久化存储） | 🔄 计划中 |
| Discord Bot 接口 | 🔄 计划中 |
| 多 LLM 提供商（Anthropic、Gemini 等） | 🔄 计划中 |
| Dashboard 配置编辑器 | 🔄 计划中 |
| 会话历史 | 🔄 计划中 |
| 用量统计 | 🔄 计划中 |
| 日志查看器 | 🔄 计划中 |
| 定时任务 | 🔄 计划中 |
| Skills / 扩展系统 | 🔄 计划中 |

---

## 许可证

MIT
