use std::fmt::Debug;

use serde::de::{Deserialize, Deserializer};

use crate::errors::*;
use crate::types::*;

macro_rules! rtd_enum_deserialize {
  ($type_name:ident, $(($td_name:ident, $enum_item:ident));*;) => {
    // example json
    // {"@type":"authorizationStateWaitEncryptionKey","is_encrypted":false}
    |deserializer: D| -> Result<$type_name, D::Error> {
      let rtd_trait_value: serde_json::Value = Deserialize::deserialize(deserializer)?;
      // the `rtd_trait_value` variable type is &serde_json::Value, tdlib trait will return a object, convert this type to object `&Map<String, Value>`
      let rtd_trait_map = match rtd_trait_value.as_object() {
        Some(map) => map,
        None => return Err(D::Error::unknown_field(stringify!($type_name), &[stringify!("{} is not the correct type", $type_name)])) // &format!("{} is not the correct type", stringify!($field))[..]
      };
      // get `@type` value, detect specific types
      let rtd_trait_type = match rtd_trait_map.get("@type") {
        // the `t` variable type is `serde_json::Value`, convert `t` to str
        Some(t) => match t.as_str() {
          Some(s) => s,
          None => return Err(D::Error::unknown_field(stringify!("{} -> @type", $field), &[stringify!("{} -> @type is not the correct type", $type_name)])) // &format!("{} -> @type is not the correct type", stringify!($field))[..]
        },
        None => return Err(D::Error::missing_field(stringify!("{} -> @type", $field)))
      };

      let obj = match rtd_trait_type {
        $(
          stringify!($td_name) => $type_name::$enum_item(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::unknown_field(stringify!("{} can't deserialize to {}::{}", $td_name, $type_name, $enum_item, _e), &[stringify!("{:?}", _e)]))
          }),
        )*
        _ => return Err(D::Error::missing_field(stringify!($field)))
      };
      Ok(obj)
    }
  }
}


///// tuple enum is field
//macro_rules! tuple_enum_is {
//  ($enum_name:ident, $field:ident) => {
//    |o: &$enum_name| {
//      if let $enum_name::$field(_) = o { true } else { false }
//    }
//  };
////  ($e:ident, $t:ident, $namespace:ident) => {
////    Box::new(|t: &$e| {
////      match t {
////        $namespace::$e::$t(_) => true,
////        _ => false
////      }
////    })
////  };
//}
//
//macro_rules! tuple_enum_on {
//  ($enum_name:ident, $field:ident, $fnc:expr) => {
//    |o: &$enum_name| {
//      if let $enum_name::$field(t) = o { $fnc(t) }
//    }
//  };
//}

pub fn detect_td_type<S: AsRef<str>>(json: S) -> Option<String> {
  let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str::<serde_json::Value>(json.as_ref());
  if let Err(_) = result { return None }
  let value = result.unwrap();
  value.as_object().map_or(None, |v| {
    v.get("@type").map_or(None, |t| t.as_str().map_or(None, |t| {
      Some(t.to_string())
    }))
  })
}

pub fn detect_td_type_and_extra<S: AsRef<str>>(json: S) -> (Option<String>, Option<String>) {
  let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str::<serde_json::Value>(json.as_ref());
  if let Err(_) = result { return (None, None) }
  let value = result.unwrap();
  let mut type_ = None;
  let mut extra = None;
  if let Some(map) = value.as_object() {
    map.get("@type").map(|v| v.as_str().map(|t| type_.replace(t.to_string())));
    map.get("@extra").map(|v| v.as_str().map(|t| extra.replace(t.to_string())));
  }
  (type_, extra)
}

pub fn from_json<'a, T>(json: &'a str) -> RTDResult<T> where T: serde::de::Deserialize<'a>, {
  Ok(serde_json::from_str(json.as_ref())?)
}

/// All tdlib type abstract class defined the same behavior
pub trait RObject: Debug {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str;
  #[doc(hidden)]
  fn extra(&self) -> Option<String>;
  /// Return td type to json string
  fn to_json(&self) -> RTDResult<String>;
}

pub trait RFunction: Debug + RObject {}


impl<'a, RObj: RObject> RObject for &'a RObj {
  fn td_name(&self) -> &'static str { (*self).td_name() }
  fn to_json(&self) -> RTDResult<String> { (*self).to_json() }
  fn extra(&self) -> Option<String> { (*self).extra() }
}

impl<'a, RObj: RObject> RObject for &'a mut RObj {
  fn td_name(&self) -> &'static str { (**self).td_name() }
  fn to_json(&self) -> RTDResult<String> { (**self).to_json() }
  fn extra(&self) -> Option<String> { (**self).extra() }
}


impl<'a, Fnc: RFunction> RFunction for &'a Fnc {}
impl<'a, Fnc: RFunction> RFunction for &'a mut Fnc {}


impl<'a, AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType for &'a AUTHENTICATIONCODETYPE {}
impl<'a, AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType for &'a mut AUTHENTICATIONCODETYPE {}

impl<'a, AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState for &'a AUTHORIZATIONSTATE {}
impl<'a, AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState for &'a mut AUTHORIZATIONSTATE {}

impl<'a, CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &'a CALLDISCARDREASON {}
impl<'a, CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &'a mut CALLDISCARDREASON {}

impl<'a, CALLSTATE: TDCallState> TDCallState for &'a CALLSTATE {}
impl<'a, CALLSTATE: TDCallState> TDCallState for &'a mut CALLSTATE {}

impl<'a, CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload for &'a CALLBACKQUERYPAYLOAD {}
impl<'a, CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload for &'a mut CALLBACKQUERYPAYLOAD {}

impl<'a, CHATACTION: TDChatAction> TDChatAction for &'a CHATACTION {}
impl<'a, CHATACTION: TDChatAction> TDChatAction for &'a mut CHATACTION {}

impl<'a, CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &'a CHATEVENTACTION {}
impl<'a, CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &'a mut CHATEVENTACTION {}

impl<'a, CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &'a CHATMEMBERSTATUS {}
impl<'a, CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &'a mut CHATMEMBERSTATUS {}

impl<'a, CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &'a CHATMEMBERSFILTER {}
impl<'a, CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &'a mut CHATMEMBERSFILTER {}

impl<'a, CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &'a CHATREPORTREASON {}
impl<'a, CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &'a mut CHATREPORTREASON {}

impl<'a, CHATTYPE: TDChatType> TDChatType for &'a CHATTYPE {}
impl<'a, CHATTYPE: TDChatType> TDChatType for &'a mut CHATTYPE {}

impl<'a, CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult for &'a CHECKCHATUSERNAMERESULT {}
impl<'a, CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult for &'a mut CHECKCHATUSERNAMERESULT {}

impl<'a, CONNECTIONSTATE: TDConnectionState> TDConnectionState for &'a CONNECTIONSTATE {}
impl<'a, CONNECTIONSTATE: TDConnectionState> TDConnectionState for &'a mut CONNECTIONSTATE {}

impl<'a, DEVICETOKEN: TDDeviceToken> TDDeviceToken for &'a DEVICETOKEN {}
impl<'a, DEVICETOKEN: TDDeviceToken> TDDeviceToken for &'a mut DEVICETOKEN {}

impl<'a, FILETYPE: TDFileType> TDFileType for &'a FILETYPE {}
impl<'a, FILETYPE: TDFileType> TDFileType for &'a mut FILETYPE {}

impl<'a, INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType for &'a INLINEKEYBOARDBUTTONTYPE {}
impl<'a, INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType for &'a mut INLINEKEYBOARDBUTTONTYPE {}

impl<'a, INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &'a INLINEQUERYRESULT {}
impl<'a, INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &'a mut INLINEQUERYRESULT {}

impl<'a, INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &'a INPUTCREDENTIALS {}
impl<'a, INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &'a mut INPUTCREDENTIALS {}

impl<'a, INPUTFILE: TDInputFile> TDInputFile for &'a INPUTFILE {}
impl<'a, INPUTFILE: TDInputFile> TDInputFile for &'a mut INPUTFILE {}

impl<'a, INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult for &'a INPUTINLINEQUERYRESULT {}
impl<'a, INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult for &'a mut INPUTINLINEQUERYRESULT {}

impl<'a, INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent for &'a INPUTMESSAGECONTENT {}
impl<'a, INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent for &'a mut INPUTMESSAGECONTENT {}

impl<'a, INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement for &'a INPUTPASSPORTELEMENT {}
impl<'a, INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement for &'a mut INPUTPASSPORTELEMENT {}

impl<'a, INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource> TDInputPassportElementErrorSource for &'a INPUTPASSPORTELEMENTERRORSOURCE {}
impl<'a, INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource> TDInputPassportElementErrorSource for &'a mut INPUTPASSPORTELEMENTERRORSOURCE {}

impl<'a, KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType for &'a KEYBOARDBUTTONTYPE {}
impl<'a, KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType for &'a mut KEYBOARDBUTTONTYPE {}

impl<'a, LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue for &'a LANGUAGEPACKSTRINGVALUE {}
impl<'a, LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue for &'a mut LANGUAGEPACKSTRINGVALUE {}

impl<'a, LINKSTATE: TDLinkState> TDLinkState for &'a LINKSTATE {}
impl<'a, LINKSTATE: TDLinkState> TDLinkState for &'a mut LINKSTATE {}

impl<'a, MASKPOINT: TDMaskPoint> TDMaskPoint for &'a MASKPOINT {}
impl<'a, MASKPOINT: TDMaskPoint> TDMaskPoint for &'a mut MASKPOINT {}

impl<'a, MESSAGECONTENT: TDMessageContent> TDMessageContent for &'a MESSAGECONTENT {}
impl<'a, MESSAGECONTENT: TDMessageContent> TDMessageContent for &'a mut MESSAGECONTENT {}

impl<'a, MESSAGEFORWARDINFO: TDMessageForwardInfo> TDMessageForwardInfo for &'a MESSAGEFORWARDINFO {}
impl<'a, MESSAGEFORWARDINFO: TDMessageForwardInfo> TDMessageForwardInfo for &'a mut MESSAGEFORWARDINFO {}

impl<'a, MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState for &'a MESSAGESENDINGSTATE {}
impl<'a, MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState for &'a mut MESSAGESENDINGSTATE {}

impl<'a, NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry for &'a NETWORKSTATISTICSENTRY {}
impl<'a, NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry for &'a mut NETWORKSTATISTICSENTRY {}

impl<'a, NETWORKTYPE: TDNetworkType> TDNetworkType for &'a NETWORKTYPE {}
impl<'a, NETWORKTYPE: TDNetworkType> TDNetworkType for &'a mut NETWORKTYPE {}

impl<'a, NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope for &'a NOTIFICATIONSETTINGSSCOPE {}
impl<'a, NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope for &'a mut NOTIFICATIONSETTINGSSCOPE {}

impl<'a, OPTIONVALUE: TDOptionValue> TDOptionValue for &'a OPTIONVALUE {}
impl<'a, OPTIONVALUE: TDOptionValue> TDOptionValue for &'a mut OPTIONVALUE {}

impl<'a, PAGEBLOCK: TDPageBlock> TDPageBlock for &'a PAGEBLOCK {}
impl<'a, PAGEBLOCK: TDPageBlock> TDPageBlock for &'a mut PAGEBLOCK {}

impl<'a, PASSPORTELEMENT: TDPassportElement> TDPassportElement for &'a PASSPORTELEMENT {}
impl<'a, PASSPORTELEMENT: TDPassportElement> TDPassportElement for &'a mut PASSPORTELEMENT {}

impl<'a, PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource for &'a PASSPORTELEMENTERRORSOURCE {}
impl<'a, PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource for &'a mut PASSPORTELEMENTERRORSOURCE {}

impl<'a, PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType for &'a PASSPORTELEMENTTYPE {}
impl<'a, PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType for &'a mut PASSPORTELEMENTTYPE {}

impl<'a, PROXYTYPE: TDProxyType> TDProxyType for &'a PROXYTYPE {}
impl<'a, PROXYTYPE: TDProxyType> TDProxyType for &'a mut PROXYTYPE {}

impl<'a, REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &'a REPLYMARKUP {}
impl<'a, REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &'a mut REPLYMARKUP {}

impl<'a, RICHTEXT: TDRichText> TDRichText for &'a RICHTEXT {}
impl<'a, RICHTEXT: TDRichText> TDRichText for &'a mut RICHTEXT {}

impl<'a, SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter for &'a SEARCHMESSAGESFILTER {}
impl<'a, SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter for &'a mut SEARCHMESSAGESFILTER {}

impl<'a, SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &'a SECRETCHATSTATE {}
impl<'a, SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &'a mut SECRETCHATSTATE {}

impl<'a, SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter for &'a SUPERGROUPMEMBERSFILTER {}
impl<'a, SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter for &'a mut SUPERGROUPMEMBERSFILTER {}

impl<'a, TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &'a TMEURLTYPE {}
impl<'a, TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &'a mut TMEURLTYPE {}

impl<'a, TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &'a TEXTENTITYTYPE {}
impl<'a, TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &'a mut TEXTENTITYTYPE {}

impl<'a, TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &'a TEXTPARSEMODE {}
impl<'a, TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &'a mut TEXTPARSEMODE {}

impl<'a, TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &'a TOPCHATCATEGORY {}
impl<'a, TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &'a mut TOPCHATCATEGORY {}

impl<'a, UPDATE: TDUpdate> TDUpdate for &'a UPDATE {}
impl<'a, UPDATE: TDUpdate> TDUpdate for &'a mut UPDATE {}

impl<'a, USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting for &'a USERPRIVACYSETTING {}
impl<'a, USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting for &'a mut USERPRIVACYSETTING {}

impl<'a, USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule for &'a USERPRIVACYSETTINGRULE {}
impl<'a, USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule for &'a mut USERPRIVACYSETTINGRULE {}

impl<'a, USERSTATUS: TDUserStatus> TDUserStatus for &'a USERSTATUS {}
impl<'a, USERSTATUS: TDUserStatus> TDUserStatus for &'a mut USERSTATUS {}

impl<'a, USERTYPE: TDUserType> TDUserType for &'a USERTYPE {}
impl<'a, USERTYPE: TDUserType> TDUserType for &'a mut USERTYPE {}


#[derive(Debug, Clone)]
pub enum TdType {
  TestUseUpdate(TestUseUpdate),
  UpdateAuthorizationState(UpdateAuthorizationState),
  UpdateBasicGroup(UpdateBasicGroup),
  UpdateBasicGroupFullInfo(UpdateBasicGroupFullInfo),
  UpdateCall(UpdateCall),
  UpdateChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
  UpdateChatDraftMessage(UpdateChatDraftMessage),
  UpdateChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
  UpdateChatIsPinned(UpdateChatIsPinned),
  UpdateChatIsSponsored(UpdateChatIsSponsored),
  UpdateChatLastMessage(UpdateChatLastMessage),
  UpdateChatNotificationSettings(UpdateChatNotificationSettings),
  UpdateChatOrder(UpdateChatOrder),
  UpdateChatPhoto(UpdateChatPhoto),
  UpdateChatReadInbox(UpdateChatReadInbox),
  UpdateChatReadOutbox(UpdateChatReadOutbox),
  UpdateChatReplyMarkup(UpdateChatReplyMarkup),
  UpdateChatTitle(UpdateChatTitle),
  UpdateChatUnreadMentionCount(UpdateChatUnreadMentionCount),
  UpdateConnectionState(UpdateConnectionState),
  UpdateDeleteMessages(UpdateDeleteMessages),
  UpdateFavoriteStickers(UpdateFavoriteStickers),
  UpdateFile(UpdateFile),
  UpdateFileGenerationStart(UpdateFileGenerationStart),
  UpdateFileGenerationStop(UpdateFileGenerationStop),
  UpdateInstalledStickerSets(UpdateInstalledStickerSets),
  UpdateLanguagePackStrings(UpdateLanguagePackStrings),
  UpdateMessageContent(UpdateMessageContent),
  UpdateMessageContentOpened(UpdateMessageContentOpened),
  UpdateMessageEdited(UpdateMessageEdited),
  UpdateMessageMentionRead(UpdateMessageMentionRead),
  UpdateMessageSendAcknowledged(UpdateMessageSendAcknowledged),
  UpdateMessageSendFailed(UpdateMessageSendFailed),
  UpdateMessageSendSucceeded(UpdateMessageSendSucceeded),
  UpdateMessageViews(UpdateMessageViews),
  UpdateNewCallbackQuery(UpdateNewCallbackQuery),
  UpdateNewChat(UpdateNewChat),
  UpdateNewChosenInlineResult(UpdateNewChosenInlineResult),
  UpdateNewCustomEvent(UpdateNewCustomEvent),
  UpdateNewCustomQuery(UpdateNewCustomQuery),
  UpdateNewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
  UpdateNewInlineQuery(UpdateNewInlineQuery),
  UpdateNewMessage(UpdateNewMessage),
  UpdateNewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
  UpdateNewShippingQuery(UpdateNewShippingQuery),
  UpdateOption(UpdateOption),
  UpdateRecentStickers(UpdateRecentStickers),
  UpdateSavedAnimations(UpdateSavedAnimations),
  UpdateScopeNotificationSettings(UpdateScopeNotificationSettings),
  UpdateSecretChat(UpdateSecretChat),
  UpdateServiceNotification(UpdateServiceNotification),
  UpdateSupergroup(UpdateSupergroup),
  UpdateSupergroupFullInfo(UpdateSupergroupFullInfo),
  UpdateTermsOfService(UpdateTermsOfService),
  UpdateTrendingStickerSets(UpdateTrendingStickerSets),
  UpdateUnreadChatCount(UpdateUnreadChatCount),
  UpdateUnreadMessageCount(UpdateUnreadMessageCount),
  UpdateUser(UpdateUser),
  UpdateUserChatAction(UpdateUserChatAction),
  UpdateUserFullInfo(UpdateUserFullInfo),
  UpdateUserPrivacySettingRules(UpdateUserPrivacySettingRules),
  UpdateUserStatus(UpdateUserStatus),

  AuthorizationState(AuthorizationState),
  CheckChatUsernameResult(CheckChatUsernameResult),
  LanguagePackStringValue(LanguagePackStringValue),
  OptionValue(OptionValue),
  PassportElement(PassportElement),
  Update(Update),
  AccountTtl(AccountTtl),
  Animations(Animations),
  AuthenticationCodeInfo(AuthenticationCodeInfo),
  BasicGroup(BasicGroup),
  BasicGroupFullInfo(BasicGroupFullInfo),
  CallId(CallId),
  CallbackQueryAnswer(CallbackQueryAnswer),
  Chat(Chat),
  ChatEvents(ChatEvents),
  ChatInviteLink(ChatInviteLink),
  ChatInviteLinkInfo(ChatInviteLinkInfo),
  ChatMember(ChatMember),
  ChatMembers(ChatMembers),
  ChatReportSpamState(ChatReportSpamState),
  Chats(Chats),
  ConnectedWebsites(ConnectedWebsites),
  Count(Count),
  CustomRequestResult(CustomRequestResult),
  DeepLinkInfo(DeepLinkInfo),
  EmailAddressAuthenticationCodeInfo(EmailAddressAuthenticationCodeInfo),
  Error(Error),
  File(File),
  FormattedText(FormattedText),
  FoundMessages(FoundMessages),
  GameHighScores(GameHighScores),
  Hashtags(Hashtags),
  ImportedContacts(ImportedContacts),
  InlineQueryResults(InlineQueryResults),
  LanguagePackStrings(LanguagePackStrings),
  LocalizationTargetInfo(LocalizationTargetInfo),
  Message(Message),
  Messages(Messages),
  NetworkStatistics(NetworkStatistics),
  Ok(Ok),
  OrderInfo(OrderInfo),
  PassportAuthorizationForm(PassportAuthorizationForm),
  PassportElements(PassportElements),
  PasswordState(PasswordState),
  PaymentForm(PaymentForm),
  PaymentReceipt(PaymentReceipt),
  PaymentResult(PaymentResult),
  Proxies(Proxies),
  Proxy(Proxy),
  PublicMessageLink(PublicMessageLink),
  RecoveryEmailAddress(RecoveryEmailAddress),
  ScopeNotificationSettings(ScopeNotificationSettings),
  Seconds(Seconds),
  SecretChat(SecretChat),
  Sessions(Sessions),
  StickerEmojis(StickerEmojis),
  StickerSet(StickerSet),
  StickerSets(StickerSets),
  Stickers(Stickers),
  StorageStatistics(StorageStatistics),
  StorageStatisticsFast(StorageStatisticsFast),
  Supergroup(Supergroup),
  SupergroupFullInfo(SupergroupFullInfo),
  TMeUrls(TMeUrls),
  TemporaryPasswordState(TemporaryPasswordState),
  TestBytes(TestBytes),
  TestInt(TestInt),
  TestString(TestString),
  TestVectorInt(TestVectorInt),
  TestVectorIntObject(TestVectorIntObject),
  TestVectorString(TestVectorString),
  TestVectorStringObject(TestVectorStringObject),
  Text(Text),
  TextEntities(TextEntities),
  User(User),
  UserFullInfo(UserFullInfo),
  UserPrivacySettingRules(UserPrivacySettingRules),
  UserProfilePhotos(UserProfilePhotos),
  Users(Users),
  ValidatedOrderInfo(ValidatedOrderInfo),
  Wallpapers(Wallpapers),
  WebPage(WebPage),
  WebPageInstantView(WebPageInstantView),

}
impl<'de> Deserialize<'de> for TdType {
fn deserialize<D>(deserializer: D) -> Result<TdType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TdType,
  (testUseUpdate, TestUseUpdate);
  (updateAuthorizationState, UpdateAuthorizationState);
  (updateBasicGroup, UpdateBasicGroup);
  (updateBasicGroupFullInfo, UpdateBasicGroupFullInfo);
  (updateCall, UpdateCall);
  (updateChatDefaultDisableNotification, UpdateChatDefaultDisableNotification);
  (updateChatDraftMessage, UpdateChatDraftMessage);
  (updateChatIsMarkedAsUnread, UpdateChatIsMarkedAsUnread);
  (updateChatIsPinned, UpdateChatIsPinned);
  (updateChatIsSponsored, UpdateChatIsSponsored);
  (updateChatLastMessage, UpdateChatLastMessage);
  (updateChatNotificationSettings, UpdateChatNotificationSettings);
  (updateChatOrder, UpdateChatOrder);
  (updateChatPhoto, UpdateChatPhoto);
  (updateChatReadInbox, UpdateChatReadInbox);
  (updateChatReadOutbox, UpdateChatReadOutbox);
  (updateChatReplyMarkup, UpdateChatReplyMarkup);
  (updateChatTitle, UpdateChatTitle);
  (updateChatUnreadMentionCount, UpdateChatUnreadMentionCount);
  (updateConnectionState, UpdateConnectionState);
  (updateDeleteMessages, UpdateDeleteMessages);
  (updateFavoriteStickers, UpdateFavoriteStickers);
  (updateFile, UpdateFile);
  (updateFileGenerationStart, UpdateFileGenerationStart);
  (updateFileGenerationStop, UpdateFileGenerationStop);
  (updateInstalledStickerSets, UpdateInstalledStickerSets);
  (updateLanguagePackStrings, UpdateLanguagePackStrings);
  (updateMessageContent, UpdateMessageContent);
  (updateMessageContentOpened, UpdateMessageContentOpened);
  (updateMessageEdited, UpdateMessageEdited);
  (updateMessageMentionRead, UpdateMessageMentionRead);
  (updateMessageSendAcknowledged, UpdateMessageSendAcknowledged);
  (updateMessageSendFailed, UpdateMessageSendFailed);
  (updateMessageSendSucceeded, UpdateMessageSendSucceeded);
  (updateMessageViews, UpdateMessageViews);
  (updateNewCallbackQuery, UpdateNewCallbackQuery);
  (updateNewChat, UpdateNewChat);
  (updateNewChosenInlineResult, UpdateNewChosenInlineResult);
  (updateNewCustomEvent, UpdateNewCustomEvent);
  (updateNewCustomQuery, UpdateNewCustomQuery);
  (updateNewInlineCallbackQuery, UpdateNewInlineCallbackQuery);
  (updateNewInlineQuery, UpdateNewInlineQuery);
  (updateNewMessage, UpdateNewMessage);
  (updateNewPreCheckoutQuery, UpdateNewPreCheckoutQuery);
  (updateNewShippingQuery, UpdateNewShippingQuery);
  (updateOption, UpdateOption);
  (updateRecentStickers, UpdateRecentStickers);
  (updateSavedAnimations, UpdateSavedAnimations);
  (updateScopeNotificationSettings, UpdateScopeNotificationSettings);
  (updateSecretChat, UpdateSecretChat);
  (updateServiceNotification, UpdateServiceNotification);
  (updateSupergroup, UpdateSupergroup);
  (updateSupergroupFullInfo, UpdateSupergroupFullInfo);
  (updateTermsOfService, UpdateTermsOfService);
  (updateTrendingStickerSets, UpdateTrendingStickerSets);
  (updateUnreadChatCount, UpdateUnreadChatCount);
  (updateUnreadMessageCount, UpdateUnreadMessageCount);
  (updateUser, UpdateUser);
  (updateUserChatAction, UpdateUserChatAction);
  (updateUserFullInfo, UpdateUserFullInfo);
  (updateUserPrivacySettingRules, UpdateUserPrivacySettingRules);
  (updateUserStatus, UpdateUserStatus);

  (AuthorizationState, AuthorizationState);
  (CheckChatUsernameResult, CheckChatUsernameResult);
  (LanguagePackStringValue, LanguagePackStringValue);
  (OptionValue, OptionValue);
  (PassportElement, PassportElement);
  (Update, Update);
  (accountTtl, AccountTtl);
  (animations, Animations);
  (authenticationCodeInfo, AuthenticationCodeInfo);
  (basicGroup, BasicGroup);
  (basicGroupFullInfo, BasicGroupFullInfo);
  (callId, CallId);
  (callbackQueryAnswer, CallbackQueryAnswer);
  (chat, Chat);
  (chatEvents, ChatEvents);
  (chatInviteLink, ChatInviteLink);
  (chatInviteLinkInfo, ChatInviteLinkInfo);
  (chatMember, ChatMember);
  (chatMembers, ChatMembers);
  (chatReportSpamState, ChatReportSpamState);
  (chats, Chats);
  (connectedWebsites, ConnectedWebsites);
  (count, Count);
  (customRequestResult, CustomRequestResult);
  (deepLinkInfo, DeepLinkInfo);
  (emailAddressAuthenticationCodeInfo, EmailAddressAuthenticationCodeInfo);
  (error, Error);
  (file, File);
  (formattedText, FormattedText);
  (foundMessages, FoundMessages);
  (gameHighScores, GameHighScores);
  (hashtags, Hashtags);
  (importedContacts, ImportedContacts);
  (inlineQueryResults, InlineQueryResults);
  (languagePackStrings, LanguagePackStrings);
  (localizationTargetInfo, LocalizationTargetInfo);
  (message, Message);
  (messages, Messages);
  (networkStatistics, NetworkStatistics);
  (ok, Ok);
  (orderInfo, OrderInfo);
  (passportAuthorizationForm, PassportAuthorizationForm);
  (passportElements, PassportElements);
  (passwordState, PasswordState);
  (paymentForm, PaymentForm);
  (paymentReceipt, PaymentReceipt);
  (paymentResult, PaymentResult);
  (proxies, Proxies);
  (proxy, Proxy);
  (publicMessageLink, PublicMessageLink);
  (recoveryEmailAddress, RecoveryEmailAddress);
  (scopeNotificationSettings, ScopeNotificationSettings);
  (seconds, Seconds);
  (secretChat, SecretChat);
  (sessions, Sessions);
  (stickerEmojis, StickerEmojis);
  (stickerSet, StickerSet);
  (stickerSets, StickerSets);
  (stickers, Stickers);
  (storageStatistics, StorageStatistics);
  (storageStatisticsFast, StorageStatisticsFast);
  (supergroup, Supergroup);
  (supergroupFullInfo, SupergroupFullInfo);
  (tMeUrls, TMeUrls);
  (temporaryPasswordState, TemporaryPasswordState);
  (testBytes, TestBytes);
  (testInt, TestInt);
  (testString, TestString);
  (testVectorInt, TestVectorInt);
  (testVectorIntObject, TestVectorIntObject);
  (testVectorString, TestVectorString);
  (testVectorStringObject, TestVectorStringObject);
  (text, Text);
  (textEntities, TextEntities);
  (user, User);
  (userFullInfo, UserFullInfo);
  (userPrivacySettingRules, UserPrivacySettingRules);
  (userProfilePhotos, UserProfilePhotos);
  (users, Users);
  (validatedOrderInfo, ValidatedOrderInfo);
  (wallpapers, Wallpapers);
  (webPage, WebPage);
  (webPageInstantView, WebPageInstantView);

 )(deserializer)

 }
}



#[cfg(test)]
mod tests {
  use crate::types::{TdType, from_json, UpdateAuthorizationState};

  #[test]
  fn test_deserialize_enum() {
    match from_json::<UpdateAuthorizationState>(r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#) {
      Ok(t) => {},
      Err(e) => {panic!("{}", e)}
    };

    match from_json::<TdType>(r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#) {
      Ok(t) => {
        match t {
          TdType::UpdateAuthorizationState(v) => {},
          _ => panic!("from_json failed: {:?}", t)
        }
      },
      Err(e) => {panic!("{}", e)}
    };
  }
}


