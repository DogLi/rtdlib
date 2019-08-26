
use crate::types::*;
use crate::errors::*;




/// Contains approximate storage usage statistics, excluding files of unknown file type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageStatisticsFast {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Approximate total size of files
  files_size: i32,
  /// Approximate number of files
  file_count: i32,
  /// Size of the database
  database_size: i32,
  /// Size of the language pack database
  language_pack_database_size: i32,
  /// Size of the TDLib internal log
  log_size: i32,
  
}

impl RObject for StorageStatisticsFast {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "storageStatisticsFast" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl StorageStatisticsFast {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStorageStatisticsFastBuilder {
    let mut inner = StorageStatisticsFast::default();
    inner.td_name = "storageStatisticsFast".to_string();
    RTDStorageStatisticsFastBuilder { inner }
  }

  pub fn files_size(&self) -> i32 { self.files_size }

  pub fn file_count(&self) -> i32 { self.file_count }

  pub fn database_size(&self) -> i32 { self.database_size }

  pub fn language_pack_database_size(&self) -> i32 { self.language_pack_database_size }

  pub fn log_size(&self) -> i32 { self.log_size }

}

#[doc(hidden)]
pub struct RTDStorageStatisticsFastBuilder {
  inner: StorageStatisticsFast
}

impl RTDStorageStatisticsFastBuilder {
  pub fn build(&self) -> StorageStatisticsFast { self.inner.clone() }

   
  pub fn files_size(&mut self, files_size: i32) -> &mut Self {
    self.inner.files_size = files_size;
    self
  }

   
  pub fn file_count(&mut self, file_count: i32) -> &mut Self {
    self.inner.file_count = file_count;
    self
  }

   
  pub fn database_size(&mut self, database_size: i32) -> &mut Self {
    self.inner.database_size = database_size;
    self
  }

   
  pub fn language_pack_database_size(&mut self, language_pack_database_size: i32) -> &mut Self {
    self.inner.language_pack_database_size = language_pack_database_size;
    self
  }

   
  pub fn log_size(&mut self, log_size: i32) -> &mut Self {
    self.inner.log_size = log_size;
    self
  }

}

impl AsRef<StorageStatisticsFast> for StorageStatisticsFast {
  fn as_ref(&self) -> &StorageStatisticsFast { self }
}

impl AsRef<StorageStatisticsFast> for RTDStorageStatisticsFastBuilder {
  fn as_ref(&self) -> &StorageStatisticsFast { &self.inner }
}



