use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;

/// Action Log Action enum (action_logs.action 필드에 저장됨)
/// 포맷: "{resource}:{operation}"
///
/// Moderation Actions vs Action Log Actions:
/// - Moderation Actions: 관리자/중재자 행동 로깅 (ban, delete, hide 등)
/// - Action Log Actions: 일반 사용자 활동 로깅 (create, edit 등)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ActionLogAction {
    // ==================== Post Actions (3) ====================
    #[serde(rename = "post:create")]
    PostCreate,
    #[serde(rename = "post:edit")]
    PostEdit,
    #[serde(rename = "post:delete")]
    PostDelete,
    // ==================== Comment Actions (3) ====================
    #[serde(rename = "comment:create")]
    CommentCreate,
    #[serde(rename = "comment:edit")]
    CommentEdit,
    #[serde(rename = "comment:delete")]
    CommentDelete,
}

impl ActionLogAction {
    /// Convert to database string value
    pub fn as_str(&self) -> &'static str {
        match self {
            // Post
            ActionLogAction::PostCreate => "post:create",
            ActionLogAction::PostEdit => "post:edit",
            ActionLogAction::PostDelete => "post:delete",
            // Comment
            ActionLogAction::CommentCreate => "comment:create",
            ActionLogAction::CommentEdit => "comment:edit",
            ActionLogAction::CommentDelete => "comment:delete",
        }
    }
}

impl fmt::Display for ActionLogAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ActionLogAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Post
            "post:create" => Ok(ActionLogAction::PostCreate),
            "post:edit" => Ok(ActionLogAction::PostEdit),
            "post:delete" => Ok(ActionLogAction::PostDelete),
            // Comment
            "comment:create" => Ok(ActionLogAction::CommentCreate),
            "comment:edit" => Ok(ActionLogAction::CommentEdit),
            "comment:delete" => Ok(ActionLogAction::CommentDelete),
            _ => Err(format!("Unknown action log action: {}", s)),
        }
    }
}

/// Convert ActionLogAction to String for DB storage
pub fn action_log_action_to_string(action: ActionLogAction) -> String {
    action.as_str().to_string()
}

/// Convert String from DB to ActionLogAction
pub fn string_to_action_log_action(s: &str) -> Option<ActionLogAction> {
    s.parse().ok()
}
