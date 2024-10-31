use std::path::PathBuf;
use iced::{window::Id, Event};

#[derive(Debug, Clone)]
pub enum Message {
    CalculateMD5,
    MD5Calculated(String),
    SetTarget(String),
    OnEvent(Id, Event),
    EnableEditMode,
    DisableEditMode,
    WindowResize,
    FileDropped(PathBuf),
    EnterKeyPressed,

}