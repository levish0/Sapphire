use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;

/// Moderation Action enum (moderation_logs.action 필드에 저장됨)
/// 포맷: "{resource}:{operation}"
///
/// 관리자/중재자의 제재·관리 행동을 기록
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ModerationAction {
    // ==================== User Actions ====================
    #[serde(rename = "user:ban")]
    UserBan,
    #[serde(rename = "user:unban")]
    UserUnban,
    #[serde(rename = "user:warn")]
    UserWarn,
    // ==================== Post Actions ====================
    #[serde(rename = "post:delete")]
    PostDelete,
    #[serde(rename = "post:hide")]
    PostHide,
    #[serde(rename = "post:unhide")]
    PostUnhide,
    // ==================== Comment Actions ====================
    #[serde(rename = "comment:delete")]
    CommentDelete,
    #[serde(rename = "comment:hide")]
    CommentHide,
    #[serde(rename = "comment:unhide")]
    CommentUnhide,
}

impl ModerationAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            // User
            ModerationAction::UserBan => "user:ban",
            ModerationAction::UserUnban => "user:unban",
            ModerationAction::UserWarn => "user:warn",
            // Post
            ModerationAction::PostDelete => "post:delete",
            ModerationAction::PostHide => "post:hide",
            ModerationAction::PostUnhide => "post:unhide",
            // Comment
            ModerationAction::CommentDelete => "comment:delete",
            ModerationAction::CommentHide => "comment:hide",
            ModerationAction::CommentUnhide => "comment:unhide",
        }
    }
}

impl fmt::Display for ModerationAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ModerationAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // User
            "user:ban" => Ok(ModerationAction::UserBan),
            "user:unban" => Ok(ModerationAction::UserUnban),
            "user:warn" => Ok(ModerationAction::UserWarn),
            // Post
            "post:delete" => Ok(ModerationAction::PostDelete),
            "post:hide" => Ok(ModerationAction::PostHide),
            "post:unhide" => Ok(ModerationAction::PostUnhide),
            // Comment
            "comment:delete" => Ok(ModerationAction::CommentDelete),
            "comment:hide" => Ok(ModerationAction::CommentHide),
            "comment:unhide" => Ok(ModerationAction::CommentUnhide),
            _ => Err(format!("Unknown moderation action: {}", s)),
        }
    }
}

/// Convert ModerationAction to String for DB storage
pub fn moderation_action_to_string(action: ModerationAction) -> String {
    action.as_str().to_string()
}

/// Convert String from DB to ModerationAction
pub fn string_to_moderation_action(s: &str) -> Option<ModerationAction> {
    s.parse().ok()
}
