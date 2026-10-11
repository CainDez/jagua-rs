# PandaNest 上游兼容回迁

本 fork 以 `64a774f97eba88ca1ea047fc0eef8e0b428625df` 为定制基线；共同上游祖先为
`ef83b08840fbca2a7eab2f164af9c45d86e085b0`。本批评估冻结上游 main
`9e00fd867be0fc493002aea68a1c6613d5b833d6`，采用兼容回迁，不能视为已合并完整 main。
保留 MPL-2.0 许可证及源文件署名。

| 上游来源 | 处理 | 验证或约束 |
| --- | --- | --- |
| `824ab31c` / #87 | 复用 quadtree debug assertion 的 f64 保守参照与测试 | 生产 f32 碰撞谓词不变；允许 f32 保守假阳性 |
| `dea6f872` / #89 | 修正变换顺序文档 | 只改说明 |
| `bfdeca7a` / #90 | 复用 SVG 孔洞和低质量区碰撞线 | 离线诊断显示，不改产品请求契约 |
| `a696c1d6` / #91 | 容器 density 使用 gross area | 统一有缺陷/孔洞容器的诊断指标口径 |
| `a15ddd49` / #102 | 复用缩短 polygon 错误信息；保留本地等价且更保守的 offset 修复 | 继续使用 geo-buffer 快路径与拓扑安全 geo 回退，保留圆弧 sagitta 补偿；970 顶点真实轮廓测试守门 |
| `0058d418` / #107 | pier rays 以质心相对坐标构造 | 用非原点形状平移不变性反例验证 |
| `c7c15b78` / #119 | 回迁简化候选不得扩大 bbox 的保护 | V-notch 反例必须仍简化、保守包含且 bbox 不增大 |
| `9e00fd86` / #121 | 不采用 multipart offset 只保留最大轮廓的行为 | 避免静默丢失其他轮廓；保留多部分/孔洞保守拒绝契约 |

本 fork crate 为 `0.8.3`，继续提供 PandaNest 与 Sparrow 已使用的 0.8 类型契约。
上游 1.0/1.1 的 Arc<Item>、独立 ID/index、orientation/reflection、泛型碰撞引擎及
collector/filter 重构需要另一个完整 API 迁移批次，本批不启用这些新契约。

Native Jagua 单测保留 debug assertions；产品 dev/test 对 Jagua 的既有关闭策略不变。
最终布局仍由父仓库原始几何校验守门，不能用代理形状或 solver 自报密度替代合法性验证。
