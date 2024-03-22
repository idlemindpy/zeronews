use crate::domain::sub_email::SubscriberEmail;
use crate::domain::sub_name::SubscriberName;

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
