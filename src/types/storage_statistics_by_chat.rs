
use crate::types::*;
use crate::errors::*;




/// Contains the storage usage statistics for a specific chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageStatisticsByChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Chat identifier; 0 if none
  chat_id: i32,
  /// Total size of the files in the chat
  size: i32,
  /// Total number of files in the chat
  count: i32,
  /// Statistics split by file types
  by_file_type: Vec<StorageStatisticsByFileType>,
  
}

impl RObject for StorageStatisticsByChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "storageStatisticsByChat" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl StorageStatisticsByChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStorageStatisticsByChatBuilder {
    let mut inner = StorageStatisticsByChat::default();
    inner.td_name = "storageStatisticsByChat".to_string();
    RTDStorageStatisticsByChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i32 { self.chat_id }

  pub fn size(&self) -> i32 { self.size }

  pub fn count(&self) -> i32 { self.count }

  pub fn by_file_type(&self) -> &Vec<StorageStatisticsByFileType> { &self.by_file_type }

}

#[doc(hidden)]
pub struct RTDStorageStatisticsByChatBuilder {
  inner: StorageStatisticsByChat
}

impl RTDStorageStatisticsByChatBuilder {
  pub fn build(&self) -> StorageStatisticsByChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i32) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn size(&mut self, size: i32) -> &mut Self {
    self.inner.size = size;
    self
  }

   
  pub fn count(&mut self, count: i32) -> &mut Self {
    self.inner.count = count;
    self
  }

   
  pub fn by_file_type(&mut self, by_file_type: Vec<StorageStatisticsByFileType>) -> &mut Self {
    self.inner.by_file_type = by_file_type;
    self
  }

}

impl AsRef<StorageStatisticsByChat> for StorageStatisticsByChat {
  fn as_ref(&self) -> &StorageStatisticsByChat { self }
}

impl AsRef<StorageStatisticsByChat> for RTDStorageStatisticsByChatBuilder {
  fn as_ref(&self) -> &StorageStatisticsByChat { &self.inner }
}



