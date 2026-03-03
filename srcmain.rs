//! Apex Kernel - Minimal self-healing execution kernel
//! Core architecture: Supervisor pattern with process isolation and health monitoring

use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Mutex, RwLock};
use tracing::{error, info, warn, Level};
use uuid::Uuid;

mod firebase_state;
mod health_monitor;
mod task_manager;

use firebase_state::FirebaseStateManager;
use health_monitor::{HealthMonitor, SystemMetrics};
use task_manager::{Task, TaskManager, TaskStatus};

#[derive(Debug, Clone)]
pub struct KernelConfig {
    pub max_restart_attempts: u32,
    pub health_check_interval_secs: u64,
    pub task_timeout_secs: u64,
    pub max_concurrent_tasks: usize,
    pub firebase_project_id: String,
    pub firebase_credentials_path: String,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            max_restart_attempts: 3,
            health_check_interval_secs: 30,
            task_timeout_secs: 300,
            max_concurrent_tasks: 10,
            firebase_project_id: "apex-kernel".to_string(),
            fire