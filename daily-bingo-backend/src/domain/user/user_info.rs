use chrono::DateTime;
use chrono::Utc;

pub struct UserInfo {
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}
