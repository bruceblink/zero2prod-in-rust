use crate::domain::{SubscriberEmail, SubscriberName};

pub struct NewSubscriber {
    // We are not using `String` anymore!
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
