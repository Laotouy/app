use log::{info, warn};
use std::sync::LazyLock;

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("Failed to create IndexNow HTTP client")
});

/// 向 Bing IndexNow API 提交 URL，通知搜索引擎内容已更新。
/// 使用 tokio::spawn 异步发送，不阻塞调用方。
/// 如果 INDEXNOW_KEY 未设置则静默跳过。
pub fn submit_urls(urls: Vec<String>) {
    let key = match dotenvy::var("INDEXNOW_KEY") {
        Ok(k) if !k.is_empty() => k,
        _ => return,
    };

    let site_url = match dotenvy::var("SITE_URL") {
        Ok(u) => u,
        _ => return,
    };

    let host = site_url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or_default()
        .to_string();

    let urls_debug = format!("{:?}", urls);

    tokio::spawn(async move {
        let body = serde_json::json!({
            "host": host,
            "key": key,
            "keyLocation": format!("https://{}/{}.txt", host, key),
            "urlList": urls,
        });

        match HTTP_CLIENT
            .post("https://api.indexnow.org/indexnow")
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => {
                info!("IndexNow 提交成功: status={}, urls={}", resp.status(), urls_debug);
            }
            Err(e) => {
                warn!("IndexNow 提交失败: {}, urls={}", e, urls_debug);
            }
        }
    });
}

/// 根据项目类型和 slug 构建项目 URL 并提交到 IndexNow。
pub fn notify_project(project_type: &str, slug: &str) {
    let site_url = match dotenvy::var("SITE_URL") {
        Ok(u) => u,
        _ => return,
    };

    let url = format!("{}/{}/{}", site_url, project_type, slug);
    submit_urls(vec![url]);
}
