
use crate::types::*;
use crate::errors::*;




/// Contains information about a notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Notification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Unique persistent identifier of this notification
  id: i32,
  /// Notification date
  date: i32,
  /// Notification type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: NotificationType,
  
}

impl RObject for Notification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notification" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Notification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNotificationBuilder {
    let mut inner = Notification::default();
    inner.td_name = "notification".to_string();
    RTDNotificationBuilder { inner }
  }

  pub fn id(&self) -> i32 { self.id }

  pub fn date(&self) -> i32 { self.date }

  pub fn type_(&self) -> &NotificationType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDNotificationBuilder {
  inner: Notification
}

impl RTDNotificationBuilder {
  pub fn build(&self) -> Notification { self.inner.clone() }

   
  pub fn id(&mut self, id: i32) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn date(&mut self, date: i32) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn type_<T: AsRef<NotificationType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<Notification> for Notification {
  fn as_ref(&self) -> &Notification { self }
}

impl AsRef<Notification> for RTDNotificationBuilder {
  fn as_ref(&self) -> &Notification { &self.inner }
}



