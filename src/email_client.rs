//邮件发送组件

use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use crate::domain::SubscriberEmail;

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,  // 存储Client实例
    base_url: String,     // 用于存储发送API请求的URL
    sender: SubscriberEmail,
    authorization_token: Secret<String>
}

impl EmailClient {

    // 增加EmailClient的构造方法
    pub fn new(base_url: String, sender: SubscriberEmail, authorization_token: Secret<String>) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            authorization_token
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str
    )  -> Result<(), reqwest::Error>{
        let url = format!("{}/email", self.base_url);
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text_body: text_content.to_owned()
        };
        self.http_client
            .post(&url)
            .header("X-Postmark-Server-Token", self.authorization_token.expose_secret())
            .json(&request_body)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

#[cfg(test)]
mod tests {
    use actix_web::web::method;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{header, header_exists, path};
    use crate::configuration::get_configuration;
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let configuration = get_configuration().expect("Failed to read configuration.");
        let email_client = EmailClient::new(mock_server.uri(), sender, configuration.email_client.authorization_token);

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST".parse().unwrap()))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;
        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content).await;
    }


}