//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task info in it's lifecycle
    pub task_info: TaskInfo,
    /// The task context
    pub task_cx: TaskContext,
}

/// The status of a task
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// Task information
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TaskInfo {
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
    /// Task status in it's life cycle
    pub status: TaskStatus,
}

impl TaskInfo {
    /// Create a new task info
    pub fn new() -> Self {
        Self {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}