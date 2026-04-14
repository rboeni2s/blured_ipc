use std::time::Duration;

use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstanceIdentifier
{
    #[default]
    All,
    Monitor(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action
{
    JumpToScene(String),
    NextScene,
    SetEffectOn(bool),
    ToggleEffect,
    SetSlideshowOn(bool),
    ToggleSlideshow,
    SetSlideshowInterval(Duration),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Message
{
    #[serde(default)]
    pub instance: InstanceIdentifier,
    pub action: Action,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OkResponse
{
    Ok,
    SwitchedScene(String),
    SetEffectOn(bool),
    SetSlideshowOn(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrResponse
{
    NoSuchScene(String),
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status
{
    Ok(OkResponse),
    Err(ErrResponse),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Response
{
    pub instance: InstanceIdentifier,
    pub status: Status,
}
