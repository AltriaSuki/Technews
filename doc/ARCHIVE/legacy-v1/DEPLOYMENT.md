# 部署指南 (Deployment Guide)

## 推荐方案：GitHub Pages (配合本地网络工具)

既然你的电脑上有网络工具（梯子），那么 **GitHub Pages** 是最简单、免费且稳定的方案。

### 部署步骤

1.  **确保本地项目准备就绪**：
    你已经在本地拉取了最新代码（包含我已经为你配置好的 `deploy` 脚本）。

2.  **一键部署**：
    在项目根目录运行：
    ```bash
    npm run deploy
    ```
    *这个命令会自动构建项目并将 `dist` 文件夹推送到 `gh-pages` 分支。*

3.  **开启 GitHub Pages**：
    - 去你的 GitHub 仓库页面 (`AltriaSuki/Technews`)
    - 点击 **Settings** -> **Pages** (侧边栏)
    - 在 **Source** 下选择 `Deploy from a branch`
    - Branch 选择 `gh-pages` / `/ (root)`
    - 点击 **Save**

4.  **访问**：
    几分钟后，你就可以通过 `https://altriasuki.github.io/Technews/` 访问了。

---

## 备选方案：VPS (如果你想用 VPS 也可以)

因为代码已经改回了“纯前端”模式，你现在的 VPS 部署也变得更简单了。

1.  在 VPS 上安装 Nginx/Caddy/Apache 任意一个 Web 服务器。
2.  构建代码 `npm run build`。
3.  把 `dist` 文件夹里的文件扔进 Web 服务器的目录（例如 `/var/www/html`）。
4.  直接访问 IP 即可。
