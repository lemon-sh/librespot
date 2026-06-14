use crate::{
    deserialize_with::*,
    protocol::{
        context::Context,
        context_player_options::ContextPlayerOptionOverrides,
        player::{PlayOrigin, ProvidedTrack},
        transfer_state::TransferState,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

/// A dealer request containing a message ID, sender device ID, and command.
#[derive(Clone, Debug, Deserialize)]
pub struct Request {
    /// The unique message identifier.
    pub message_id: u32,
    // todo: did only send target_alias_id: null so far, maybe we just ignore it, will see
    // pub target_alias_id: Option<()>,
    /// The device ID of the sender.
    pub sent_by_device_id: String,
    /// The command to execute.
    pub command: Command,
}

/// A dealer command representing a playback action.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "endpoint", rename_all = "snake_case")]
pub enum Command {
    /// Transfer playback to another device.
    Transfer(TransferCommand),
    /// Start or resume playback.
    #[serde(deserialize_with = "boxed")]
    Play(Box<PlayCommand>),
    /// Pause playback.
    Pause(PauseCommand),
    /// Seek to a position.
    SeekTo(SeekToCommand),
    /// Set shuffle state.
    SetShufflingContext(SetValueCommand),
    /// Set repeat-track state.
    SetRepeatingTrack(SetValueCommand),
    /// Set repeat-context state.
    SetRepeatingContext(SetValueCommand),
    /// Add a track to the queue.
    AddToQueue(AddToQueueCommand),
    /// Replace the queue.
    SetQueue(SetQueueCommand),
    /// Update playback options.
    SetOptions(SetOptionsCommand),
    /// Update the playback context.
    UpdateContext(UpdateContextCommand),
    /// Skip to the next track.
    SkipNext(SkipNextCommand),
    // commands that don't send any context (at least not usually...)
    /// Skip to the previous track.
    SkipPrev(GenericCommand),
    /// Resume playback.
    Resume(GenericCommand),
    // catch unknown commands, so that we can implement them later
    /// An unrecognized command.
    #[serde(untagged)]
    Unknown(Value),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Command::*;

        write!(
            f,
            "endpoint: {}{}",
            matches!(self, Unknown(_))
                .then_some("unknown ")
                .unwrap_or_default(),
            match self {
                Transfer(_) => "transfer",
                Play(_) => "play",
                Pause(_) => "pause",
                SeekTo(_) => "seek_to",
                SetShufflingContext(_) => "set_shuffling_context",
                SetRepeatingContext(_) => "set_repeating_context",
                SetRepeatingTrack(_) => "set_repeating_track",
                AddToQueue(_) => "add_to_queue",
                SetQueue(_) => "set_queue",
                SetOptions(_) => "set_options",
                UpdateContext(_) => "update_context",
                SkipNext(_) => "skip_next",
                SkipPrev(_) => "skip_prev",
                Resume(_) => "resume",
                Unknown(json) => {
                    json.as_object()
                        .and_then(|obj| obj.get("endpoint").map(|v| v.as_str()))
                        .flatten()
                        .unwrap_or("???")
                }
            }
        )
    }
}

/// A command to transfer playback between devices.
#[derive(Clone, Debug, Deserialize)]
pub struct TransferCommand {
    /// The optional transfer state data.
    #[serde(default, deserialize_with = "base64_proto")]
    pub data: Option<TransferState>,
    /// Transfer options.
    pub options: TransferOptions,
    /// The source device identifier.
    pub from_device_identifier: String,
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// A command to start or resume playback.
#[derive(Clone, Debug, Deserialize)]
pub struct PlayCommand {
    /// The playback context.
    #[serde(deserialize_with = "json_proto")]
    pub context: Context,
    /// The play origin.
    #[serde(deserialize_with = "json_proto")]
    pub play_origin: PlayOrigin,
    /// Play options.
    pub options: PlayOptions,
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// A command to pause playback.
#[derive(Clone, Debug, Deserialize)]
pub struct PauseCommand {
    // does send options with it, but seems to be empty, investigate which options are send here
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// A command to seek to a position in the current track.
#[derive(Clone, Debug, Deserialize)]
pub struct SeekToCommand {
    /// The target position in milliseconds.
    pub value: u32,
    /// The current position.
    pub position: u32,
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// A command to skip to the next track.
#[derive(Clone, Debug, Deserialize)]
pub struct SkipNextCommand {
    /// An optional track to skip to.
    #[serde(default, deserialize_with = "option_json_proto")]
    pub track: Option<ProvidedTrack>,
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// A command to set a boolean value (e.g., shuffle, repeat).
#[derive(Clone, Debug, Deserialize)]
pub struct SetValueCommand {
    /// The boolean value.
    pub value: bool,
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// A command to add a track to the queue.
#[derive(Clone, Debug, Deserialize)]
pub struct AddToQueueCommand {
    /// The track to add.
    #[serde(deserialize_with = "json_proto")]
    pub track: ProvidedTrack,
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// A command to replace the queue.
#[derive(Clone, Debug, Deserialize)]
pub struct SetQueueCommand {
    /// The next tracks in the queue.
    #[serde(deserialize_with = "vec_json_proto")]
    pub next_tracks: Vec<ProvidedTrack>,
    /// The previous tracks.
    #[serde(deserialize_with = "vec_json_proto")]
    pub prev_tracks: Vec<ProvidedTrack>,
    // this queue revision is actually the last revision, so using it will not update the web ui
    // might be that internally they use the last revision to create the next revision
    /// The queue revision identifier.
    pub queue_revision: String,
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// A command to update playback options.
#[derive(Clone, Debug, Deserialize)]
pub struct SetOptionsCommand {
    /// Whether to shuffle the context.
    pub shuffling_context: Option<bool>,
    /// Whether to repeat the context.
    pub repeating_context: Option<bool>,
    /// Whether to repeat the current track.
    pub repeating_track: Option<bool>,
    /// Additional options.
    pub options: Option<OptionsOptions>,
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// A command to update the playback context.
#[derive(Clone, Debug, Deserialize)]
pub struct UpdateContextCommand {
    /// The new context.
    #[serde(deserialize_with = "json_proto")]
    pub context: Context,
    /// The optional session ID.
    pub session_id: Option<String>,
}

/// A generic command with only logging parameters.
#[derive(Clone, Debug, Deserialize)]
pub struct GenericCommand {
    /// Logging parameters.
    pub logging_params: LoggingParams,
}

/// Options for controlling playback transfer.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TransferOptions {
    /// Whether to restore the paused state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_paused: Option<String>,
    /// Whether to restore the playback position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_position: Option<String>,
    /// Whether to restore the current track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_track: Option<String>,
    /// Whether to retain the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_session: Option<String>,
}

/// Options for controlling playback.
#[derive(Clone, Debug, Deserialize)]
pub struct PlayOptions {
    /// Skip to a specific track.
    pub skip_to: Option<SkipTo>,
    /// Player option overrides.
    #[serde(default, deserialize_with = "option_json_proto")]
    pub player_options_override: Option<ContextPlayerOptionOverrides>,
    /// The license type.
    pub license: Option<String>,
    // possible to send wie web-api
    /// Position to seek to in milliseconds.
    pub seek_to: Option<u32>,
    // mobile
    /// Whether to always play something.
    pub always_play_something: Option<bool>,
    /// The audio stream type.
    pub audio_stream: Option<String>,
    /// Whether playback starts paused.
    pub initially_paused: Option<bool>,
    /// The prefetch level.
    pub prefetch_level: Option<String>,
    /// Whether the command was system-initiated.
    pub system_initiated: Option<bool>,
}

/// Additional options for the `set_options` command.
#[derive(Clone, Debug, Deserialize)]
pub struct OptionsOptions {
    only_for_local_device: bool,
    override_restrictions: bool,
    system_initiated: bool,
}

/// A target track to skip to.
#[derive(Clone, Debug, Deserialize, Default)]
pub struct SkipTo {
    /// The track UID.
    pub track_uid: Option<String>,
    /// The track URI.
    pub track_uri: Option<String>,
    /// The track index in the context.
    pub track_index: Option<u32>,
}

/// Logging parameters for a command.
#[derive(Clone, Debug, Deserialize)]
pub struct LoggingParams {
    /// Interaction IDs for analytics.
    pub interaction_ids: Option<Vec<String>>,
    /// The device identifier.
    pub device_identifier: Option<String>,
    /// When the command was initiated.
    pub command_initiated_time: Option<i64>,
    /// Page instance IDs for analytics.
    pub page_instance_ids: Option<Vec<String>>,
    /// The command identifier.
    pub command_id: Option<String>,
}
