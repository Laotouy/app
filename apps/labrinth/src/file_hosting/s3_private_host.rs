//! S3 Private Host - 用于付费插件的私有桶存储
//!
//! 支持功能:
//! - 文件上传到私有桶
//! - Presigned URL 生成（临时访问链接）
//! - 文件存在检查
//! - 文件删除

use crate::file_hosting::{
    DeleteFileData, FileHostingError, UploadFileData,
};
use bytes::Bytes;
use chrono::Utc;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use sha2::Digest;
use std::collections::HashMap;

/// 私有桶存储主机
/// 用于存储付费插件的文件，通过 Presigned URL 提供临时访问
pub struct S3PrivateHost {
    /// 用于上传/删除等操作的 bucket（使用 S3 端点）
    bucket: Bucket,
    /// 用于生成 Presigned URL 的 bucket（使用 CDN 端点，可选）
    cdn_bucket: Option<Bucket>,
}

impl S3PrivateHost {
    /// 创建新的 S3PrivateHost 实例
    ///
    /// # Arguments
    /// * `bucket_name` - 私有桶名称
    /// * `url` - S3/MinIO 端点 URL
    /// * `access_token` - 访问密钥
    /// * `secret` - 密钥
    pub fn new(
        bucket_name: &str,
        url: &str,
        access_token: &str,
        secret: &str,
    ) -> Result<S3PrivateHost, FileHostingError> {
        Self::new_with_cdn(bucket_name, url, access_token, secret, None)
    }

    /// 创建带 CDN 支持的 S3PrivateHost 实例
    ///
    /// # Arguments
    /// * `bucket_name` - 私有桶名称
    /// * `url` - S3/MinIO 端点 URL（用于上传/删除）
    /// * `access_token` - 访问密钥
    /// * `secret` - 密钥
    /// * `cdn_url` - CDN URL（用于生成 Presigned URL），如果为 None 则使用 S3 URL
    pub fn new_with_cdn(
        bucket_name: &str,
        url: &str,
        access_token: &str,
        secret: &str,
        cdn_url: Option<&str>,
    ) -> Result<S3PrivateHost, FileHostingError> {
        let credentials = Credentials::new(
            Some(access_token),
            Some(secret),
            None,
            None,
            None,
        )
        .map_err(|_| {
            FileHostingError::S3Error("创建凭证时出错".to_string())
        })?;

        // 创建用于操作的 bucket
        let mut bucket = Bucket::new(
            bucket_name,
            Region::Custom {
                region: "".to_owned(),
                endpoint: url.to_string(),
            },
            credentials.clone(),
        )
        .map_err(|_| {
            FileHostingError::S3Error("创建 Bucket 实例时出错".to_string())
        })?;
        bucket.set_path_style();
        bucket.set_request_timeout(None);

        // 如果提供了 CDN URL，创建用于生成 Presigned URL 的 bucket
        let cdn_bucket = if let Some(cdn) = cdn_url {
            let mut cdn_bucket = Bucket::new(
                bucket_name,
                Region::Custom {
                    region: "".to_owned(),
                    endpoint: cdn.to_string(),
                },
                credentials,
            )
            .map_err(|_| {
                FileHostingError::S3Error("创建 CDN Bucket 实例时出错".to_string())
            })?;
            cdn_bucket.set_path_style();
            cdn_bucket.set_request_timeout(None);
            Some(*cdn_bucket)
        } else {
            None
        };

        Ok(S3PrivateHost {
            bucket: *bucket,
            cdn_bucket,
        })
    }

    /// 上传文件到私有桶
    pub async fn upload_file(
        &self,
        content_type: &str,
        file_name: &str,
        file_bytes: Bytes,
    ) -> Result<UploadFileData, FileHostingError> {
        let content_sha1 = format!("{:x}", sha1::Sha1::digest(&file_bytes));
        let content_sha512 = format!("{:x}", sha2::Sha512::digest(&file_bytes));
        let file_size = file_bytes.len();

        // 根据文件大小设置超时时间
        let timeout_seconds =
            std::cmp::max(30, (file_size / (1024 * 1024)) + 60);

        let upload_future = self.bucket.put_object_with_content_type(
            format!("/{file_name}"),
            &file_bytes,
            content_type,
        );

        match tokio::time::timeout(
            std::time::Duration::from_secs(timeout_seconds as u64),
            upload_future,
        )
        .await
        {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                return Err(FileHostingError::S3Error(format!(
                    "S3 上传错误: {:?}",
                    e
                )));
            }
            Err(_) => {
                return Err(FileHostingError::S3Error(format!(
                    "上传超时: {} 秒, 文件大小: {} 字节",
                    timeout_seconds, file_size
                )));
            }
        }

        Ok(UploadFileData {
            file_id: file_name.to_string(),
            file_name: file_name.to_string(),
            content_length: file_bytes.len() as u32,
            content_sha512,
            content_sha1,
            content_md5: None,
            content_type: content_type.to_string(),
            upload_timestamp: Utc::now().timestamp() as u64,
        })
    }

    /// 生成文件的 Presigned GET URL
    ///
    /// # Arguments
    /// * `file_path` - 文件路径（例如 "/paid/version_id/file.jar"）
    /// * `expiry_secs` - URL 有效期（秒），建议 900 秒（15分钟）
    /// * `filename` - 可选，下载时显示的文件名
    ///
    /// # Returns
    /// 返回临时访问 URL，过期后无法访问
    /// 如果配置了 CDN URL，则返回 CDN 域名的 URL
    pub async fn presign_get(
        &self,
        file_path: &str,
        expiry_secs: u32,
        filename: Option<&str>,
    ) -> Result<String, FileHostingError> {
        let custom_queries = filename.map(|name| {
            let mut queries = HashMap::new();
            queries.insert(
                "response-content-disposition".to_string(),
                format!("attachment; filename=\"{}\"", name),
            );
            queries
        });

        // 优先使用 CDN bucket 生成 URL
        let bucket = self.cdn_bucket.as_ref().unwrap_or(&self.bucket);

        bucket
            .presign_get(file_path, expiry_secs, custom_queries)
            .await
            .map_err(|e| {
                FileHostingError::S3Error(format!(
                    "生成 Presigned URL 失败: {:?}",
                    e
                ))
            })
    }

    /// 检查文件是否存在
    pub async fn object_exists(&self, file_path: &str) -> Result<bool, FileHostingError> {
        self.bucket
            .object_exists(file_path)
            .await
            .map_err(|e| {
                FileHostingError::S3Error(format!(
                    "检查文件存在失败: {:?}",
                    e
                ))
            })
    }

    /// 删除文件
    pub async fn delete_file(
        &self,
        file_path: &str,
    ) -> Result<DeleteFileData, FileHostingError> {
        self.bucket
            .delete_object(file_path)
            .await
            .map_err(|e| {
                FileHostingError::S3Error(format!(
                    "删除文件失败: {:?}",
                    e
                ))
            })?;

        Ok(DeleteFileData {
            file_id: file_path.to_string(),
            file_name: file_path.to_string(),
        })
    }

    /// 获取文件内容（用于内部操作，如病毒扫描等）
    pub async fn get_object(&self, file_path: &str) -> Result<Vec<u8>, FileHostingError> {
        let response = self.bucket
            .get_object(file_path)
            .await
            .map_err(|e| {
                FileHostingError::S3Error(format!(
                    "获取文件失败: {:?}",
                    e
                ))
            })?;

        Ok(response.to_vec())
    }

    /// 获取桶名称
    pub fn bucket_name(&self) -> &str {
        &self.bucket.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 S3PrivateHost 的基本功能
    /// 需要设置以下环境变量才能运行：
    /// - S3_PRIVATE_BUCKET_NAME
    /// - S3_URL
    /// - S3_ACCESS_TOKEN
    /// - S3_SECRET
    #[tokio::test]
    #[ignore] // 默认忽略，需要手动运行
    async fn test_s3_private_host_integration() {
        let bucket_name = std::env::var("S3_PRIVATE_BUCKET_NAME")
            .unwrap_or_else(|_| "bbsmc-private".to_string());
        let url = std::env::var("S3_URL")
            .unwrap_or_else(|_| "http://localhost:9000".to_string());
        let access_token = std::env::var("S3_ACCESS_TOKEN")
            .expect("S3_ACCESS_TOKEN not set");
        let secret = std::env::var("S3_SECRET")
            .expect("S3_SECRET not set");

        // 创建 host
        let host = S3PrivateHost::new(&bucket_name, &url, &access_token, &secret)
            .expect("Failed to create S3PrivateHost");

        // 测试文件路径
        let test_path = "/test/integration_test.txt";
        let test_content = b"Hello, S3PrivateHost!";

        // 1. 上传文件
        let upload_result = host
            .upload_file(
                "text/plain",
                &test_path[1..], // 去掉开头的 /
                Bytes::from_static(test_content),
            )
            .await
            .expect("Failed to upload file");

        println!("Upload result: {:?}", upload_result);
        assert_eq!(upload_result.content_length, test_content.len() as u32);

        // 2. 检查文件存在
        let exists = host
            .object_exists(test_path)
            .await
            .expect("Failed to check existence");
        assert!(exists, "File should exist after upload");

        // 3. 生成 Presigned URL
        let presigned_url = host
            .presign_get(test_path, 300, Some("test.txt"))
            .await
            .expect("Failed to generate presigned URL");

        println!("Presigned URL: {}", presigned_url);
        assert!(presigned_url.contains(&bucket_name));

        // 4. 获取文件内容
        let content = host
            .get_object(test_path)
            .await
            .expect("Failed to get object");
        assert_eq!(content, test_content);

        // 5. 删除文件
        let delete_result = host
            .delete_file(test_path)
            .await
            .expect("Failed to delete file");
        println!("Delete result: {:?}", delete_result);

        // 6. 确认删除成功
        let exists_after_delete = host
            .object_exists(test_path)
            .await
            .expect("Failed to check existence after delete");
        assert!(!exists_after_delete, "File should not exist after delete");

        println!("All tests passed!");
    }
}

