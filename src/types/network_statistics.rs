
use crate::types::*;
use crate::errors::*;




/// A full list of available network statistic entries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Point in time (Unix timestamp) when the app began collecting statistics
  since_date: i32,
  /// Network statistics entries
  entries: Vec<NetworkStatisticsEntry>,
  
}

impl RObject for NetworkStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkStatistics" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl NetworkStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNetworkStatisticsBuilder {
    let mut inner = NetworkStatistics::default();
    inner.td_name = "networkStatistics".to_string();
    RTDNetworkStatisticsBuilder { inner }
  }

  pub fn since_date(&self) -> i32 { self.since_date }

  pub fn entries(&self) -> &Vec<NetworkStatisticsEntry> { &self.entries }

}

#[doc(hidden)]
pub struct RTDNetworkStatisticsBuilder {
  inner: NetworkStatistics
}

impl RTDNetworkStatisticsBuilder {
  pub fn build(&self) -> NetworkStatistics { self.inner.clone() }

   
  pub fn since_date(&mut self, since_date: i32) -> &mut Self {
    self.inner.since_date = since_date;
    self
  }

   
  pub fn entries(&mut self, entries: Vec<NetworkStatisticsEntry>) -> &mut Self {
    self.inner.entries = entries;
    self
  }

}

impl AsRef<NetworkStatistics> for NetworkStatistics {
  fn as_ref(&self) -> &NetworkStatistics { self }
}

impl AsRef<NetworkStatistics> for RTDNetworkStatisticsBuilder {
  fn as_ref(&self) -> &NetworkStatistics { &self.inner }
}



