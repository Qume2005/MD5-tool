use std::path::PathBuf;
use iced::{window::Id, Event};

use super::Target;

#[derive(Debug, Clone)]
pub enum Message {
    CalculateMD5,
    MD5Calculated(String),
    SetTarget(Target),
    OnEvent(Id, Event),
    EnableEditMode,
    DisableEditMode,
    WindowResize,
    FileDropped(PathBuf),
    EnterKeyPressed,
    ArrowRightPressed,
}