
use crate::types::*;
use crate::errors::*;




/// Represents short information about a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerSetInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Identifier of the sticker set
  id: i64,
  /// Title of the sticker set
  title: String,
  /// Name of the sticker set
  name: String,
  /// True, if the sticker set has been installed by current user
  is_installed: bool,
  /// True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously
  is_archived: bool,
  /// True, if the sticker set is official
  is_official: bool,
  /// True, if the stickers in the set are masks
  is_masks: bool,
  /// True for already viewed trending sticker sets
  is_viewed: bool,
  /// Total number of stickers in the set
  size: i32,
  /// Contains up to the first 5 stickers from the set, depending on the context. If the client needs more stickers the full set should be requested
  covers: Vec<Sticker>,
  
}

impl RObject for StickerSetInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickerSetInfo" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl StickerSetInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStickerSetInfoBuilder {
    let mut inner = StickerSetInfo::default();
    inner.td_name = "stickerSetInfo".to_string();
    RTDStickerSetInfoBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn title(&self) -> &String { &self.title }

  pub fn name(&self) -> &String { &self.name }

  pub fn is_installed(&self) -> bool { self.is_installed }

  pub fn is_archived(&self) -> bool { self.is_archived }

  pub fn is_official(&self) -> bool { self.is_official }

  pub fn is_masks(&self) -> bool { self.is_masks }

  pub fn is_viewed(&self) -> bool { self.is_viewed }

  pub fn size(&self) -> i32 { self.size }

  pub fn covers(&self) -> &Vec<Sticker> { &self.covers }

}

#[doc(hidden)]
pub struct RTDStickerSetInfoBuilder {
  inner: StickerSetInfo
}

impl RTDStickerSetInfoBuilder {
  pub fn build(&self) -> StickerSetInfo { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn is_installed(&mut self, is_installed: bool) -> &mut Self {
    self.inner.is_installed = is_installed;
    self
  }

   
  pub fn is_archived(&mut self, is_archived: bool) -> &mut Self {
    self.inner.is_archived = is_archived;
    self
  }

   
  pub fn is_official(&mut self, is_official: bool) -> &mut Self {
    self.inner.is_official = is_official;
    self
  }

   
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.is_masks = is_masks;
    self
  }

   
  pub fn is_viewed(&mut self, is_viewed: bool) -> &mut Self {
    self.inner.is_viewed = is_viewed;
    self
  }

   
  pub fn size(&mut self, size: i32) -> &mut Self {
    self.inner.size = size;
    self
  }

   
  pub fn covers(&mut self, covers: Vec<Sticker>) -> &mut Self {
    self.inner.covers = covers;
    self
  }

}

impl AsRef<StickerSetInfo> for StickerSetInfo {
  fn as_ref(&self) -> &StickerSetInfo { self }
}

impl AsRef<StickerSetInfo> for RTDStickerSetInfoBuilder {
  fn as_ref(&self) -> &StickerSetInfo { &self.inner }
}



