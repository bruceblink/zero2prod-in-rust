//邮件发送组件

use reqwest::Client;
use crate::domain::SubscriberEmail;
pub struct EmailClient {
    http_client: Client,  // 存储Client实例
    base_url: String,     // 用于存储发送API请求的URL
    sender: SubscriberEmail,
}

impl EmailClient {

    // 增加EmailClient的构造方法
    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str
    )  -> Result<(), String>{
        todo!()
    }
}