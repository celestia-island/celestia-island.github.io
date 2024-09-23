use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/**
 *  The design here is a flexible permission system, and the fields are designed to
 *  match the user group list in the global configuration.
 *  The same user can have multiple identities, which allows users to have different
 *  permissions in different scenarios.
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission(pub Vec<PermissionItem>);

impl Iterator for Permission {
    type Item = PermissionItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

impl Permission {
    pub fn append(&mut self, item: &PermissionItem) {
        if let Some(other_item) = self.get_mut(item.group.as_str(), item.name.as_str()) {
            if let Some(score) = item.score {
                // If there is already a permission item of the same score type,
                // add the new score value to the original score value.
                if let Some(ref mut old_score) = other_item.score {
                    *old_score += score;
                } else {
                    other_item.score = Some(score);
                }
            } else if let Some(expired) = item.expired {
                // The new expiration time must be later than the current time.
                if expired < Utc::now() {
                    return;
                }

                // 如果已经存在相同的限时类型权限项，替换为新的过期时间
                // If there is already a permission item of the same expiration type,
                // replace it with the new expiration time.
                if let Some(ref mut old_expired) = other_item.expired {
                    *old_expired = expired;
                } else {
                    other_item.expired = Some(expired);
                }
            } else {
                // If there is already a permission item of the same type,
                // do nothing.
            }
        } else {
            self.0.push(item.clone());
        }
    }

    pub fn remove(&mut self, item: &PermissionItem) {
        if let Some(other_item) = self.get_mut(item.group.as_str(), item.name.as_str()) {
            if let Some(score) = item.score {
                // If there is already a permission item of the same score type,
                // subtract the new score value from the original score value.
                if let Some(ref mut old_score) = other_item.score {
                    *old_score -= score;
                }
                return;
            }
        }

        // In other cases, delete the permission item directly.
        self.0
            .retain(|other_item| item.group != other_item.group || item.name != other_item.name);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, prefix: &str, suffix: &str) -> Option<&PermissionItem> {
        self.0
            .iter()
            .find(|item| item.group == prefix && item.name == suffix)
    }

    pub fn get_mut(&mut self, prefix: &str, suffix: &str) -> Option<&mut PermissionItem> {
        self.0
            .iter_mut()
            .find(|item| item.group == prefix && item.name == suffix)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionItem {
    /// User group
    pub group: String,
    /// Permission name
    pub name: String,
    /// Optional score value, only recommended for sensitive score records
    pub score: Option<i64>,
    /// Optional expiration time to limit the validity period of permissions (UTC time)
    pub expired: Option<DateTime<Utc>>,
}

impl TryFrom<String> for PermissionItem {
    type Error = serde_json::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&s)
    }
}

impl TryFrom<&str> for PermissionItem {
    type Error = serde_json::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s)
    }
}

impl TryInto<String> for PermissionItem {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        serde_json::to_string(&self)
    }
}

impl std::cmp::PartialOrd for PermissionItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.group == other.group && self.name == other.name {
            match (self.score, other.score) {
                (Some(self_score), Some(other_score)) => self_score.partial_cmp(&other_score),
                (Some(_), None) => Some(std::cmp::Ordering::Greater),
                (None, Some(_)) => Some(std::cmp::Ordering::Less),
                _ => Some(std::cmp::Ordering::Equal),
            }
        } else {
            // Sort by dictionary order of group and name
            Some(
                format!("{}::{}", self.group, self.name)
                    .cmp(&format!("{}::{}", other.group, other.name)),
            )
        }
    }
}

impl std::cmp::Ord for PermissionItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&str> for Permission {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

impl From<String> for Permission {
    fn from(s: String) -> Self {
        serde_json::from_str(&s).unwrap()
    }
}

impl Into<String> for Permission {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub const PREFIX_ROOT: &'static str = "root";
pub const SUFFIX_ADMIN: &'static str = "admin";
pub const SUFFIX_USER: &'static str = "user";

pub const ROOT_ADMIN: Lazy<PermissionItem> = Lazy::new(|| PermissionItem {
    group: PREFIX_ROOT.to_string(),
    name: SUFFIX_ADMIN.to_string(),
    score: None,
    expired: None,
});
pub const ROOT_USER: Lazy<PermissionItem> = Lazy::new(|| PermissionItem {
    group: PREFIX_ROOT.to_string(),
    name: SUFFIX_USER.to_string(),
    score: None,
    expired: None,
});

impl Default for Permission {
    fn default() -> Self {
        Permission(vec![ROOT_USER.clone()])
    }
}

/**
 * To facilitate the comparison of permissions and determine whether basic control rights are held,
 * two built-in user groups root::admin and root::user are designed here.
 *
 * A user with root::admin permission means that he has the highest permission and can operate the system at will.
 *
 * root::admin is the highest permission of the system. Users with this permission can operate the system at will.
 * It is not recommended to issue this permission to other non-essential users.
 *
 * root::user should be owned by every user who can log in normally, because it is the only permission checked by the authentication middleware during login.
 * Losing this permission means that the account is banned.
 */

impl Permission {
    pub fn is_root(&self) -> bool {
        self.0.iter().any(|item| item == &*ROOT_ADMIN)
    }

    pub fn is_user(&self) -> bool {
        self.0.iter().any(|item| item == &*ROOT_USER)
    }

    pub fn include(&self, what: &PermissionItem) -> bool {
        self.0.iter().any(|item| item == what)
    }
}

impl ToString for Permission {
    fn to_string(&self) -> String {
        self.clone().into()
    }
}

/// Provide default root::admin and root::user corresponding constants
impl Permission {
    pub const ROOT_ADMIN: Lazy<Permission> = Lazy::new(|| {
        Permission(vec![PermissionItem {
            group: PREFIX_ROOT.to_string(),
            name: SUFFIX_ADMIN.to_string(),
            score: None,
            expired: None,
        }])
    });
    pub const ROOT_USER: Lazy<Permission> = Lazy::new(|| {
        Permission(vec![PermissionItem {
            group: PREFIX_ROOT.to_string(),
            name: SUFFIX_USER.to_string(),
            score: None,
            expired: None,
        }])
    });
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub password_hash: String,

    pub permission: Permission,
    pub email: String,
    pub extra_profile: Option<String>,
}
