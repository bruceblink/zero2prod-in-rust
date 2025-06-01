use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        // 判断字符串是否为空的
        let is_empty_or_whitespace = s.trim().is_empty();
        // 判断字符串是否长度
        let is_too_long = s.graphemes(true).count() > 256;
        // 特殊字符
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{','}'];
        let contains_forbidden_characters = s.chars().any(|x| forbidden_characters.contains(&x));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("{} is not a valid subscriber name.", s)
        }else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}