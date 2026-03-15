use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy, PartialEq)]
pub enum WebsiteStatus {
    Up,
    Down,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Monitor {
    pub id: Uuid,
    pub url: String,
    pub user_id: Uuid,
    pub period: i32, // Frequency of checks in seconds
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub paused: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct MonitorTick {
    pub id: Uuid,
    pub monitor_id: Uuid,
    pub response_time_ms: i32,
    pub status: WebsiteStatus,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpData {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckPeriod {
    OneMin,
    ThreeMin,
}

impl CheckPeriod {
    pub fn as_seconds(&self) -> i32 {
        match self {
            CheckPeriod::OneMin => 60,
            CheckPeriod::ThreeMin => 180,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMonitorRequest {
    pub url: String,
    pub period: CheckPeriod,
}
