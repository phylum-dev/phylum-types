use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub group_name: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct CreateGroupResponse {
    pub group_name: String,
    pub owner_email: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct AddUserToGroupRequest {
    pub user_email: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct KickUserFromGroupRequest {
    pub user_email: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct UserGroup {
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub owner_email: String,
    pub group_name: String,

    //only present in the relatively uncommon case that the user is an admin
    #[serde(default)]
    pub is_admin: bool,

    //only present in the relatively uncommon case that the user is the owner
    #[serde(default)]
    pub is_owner: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct ListUserGroupsResponse {
    pub groups: Vec<UserGroup>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct GroupMember {
    pub user_email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct ListGroupMembersResponse {
    pub members: Vec<GroupMember>,
}
