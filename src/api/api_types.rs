
#![allow(dead_code, non_snake_case, non_camel_case_types)]
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum LolLobbyLobbyBotDifficulty {
    NONE,
    EASY,
    MEDIUM,
    HARD,
    UBER,
    TUTORIAL,
    INTRO
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LolLobbyLobbyInvitationState {
    Requested,
    Pending,
    Accepted,
    Joined,
    Declined,
    Kicked,
    OnHold,
    Error
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LolLobbyInvitationType {
    invalid,
    lobby,
    party
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LolLobbyQueueCustomGameSpectatorPolicy {
    NotAllowed,
    LobbyAllowed,
    FriendsAllowed,
    AllAllowed
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LolLobbyEligibilityRestrictionCode {
    QueueDisabled,
    QueueUnsupported,
    PlayerLevelRestriction,
    PlayerTimedRestriction,
    PlayerBannedRestriction,
    PlayerAvailableChampionRestriction,
    TeamDivisionRestriction,
    TeamSkillRestriction,
    TeamMaxSizeRestriction,
    TeamMinSizeRestriction,
    PlayerBingeRestriction,
    PlayerDodgeRestriction,
    PlayerInGameRestriction,
    PlayerLeaverBustedRestriction,
    PlayerLeaverQueueLockoutRestriction,
    PlayerLeaverTaintedWarningRestriction,
    PlayerMaxLevelRestriction,
    PlayerMinLevelRestriction,
    PlayerMinorRestriction,
    PlayerTimePlayedRestriction,
    PlayerRankSoloOnlyRestriction,
    PlayerRankedSuspensionRestriction,
    TeamHighMMRMaxSizeRestriction,
    TeamSizeRestriction,
    PrerequisiteQueuesNotPlayedRestriction,
    GameVersionMismatch,
    GameVersionMissing,
    GameVersionNotSupported,
    QueueEntryNotEntitledRestriction,
    UnknownRestriction,
    BanInfoNotAvailable,
    MinorInfoNotAvailable,
    SummonerInfoNotAvailable,
    LeaguesInfoNotAvailable,
    InventoryChampsInfoNotAvailable,
    InventoryQueuesInfoNotAvailable,
    MmrStandardDeviationTooLarge
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolLobbyEligibilityRestriction {
    pub expiredTimestamp: u64,
    pub restrictionArgs: Value,
    pub summonerIds: Vec<u64>,
    pub summonerIdsString: String,
    pub restrictionCode: LolLobbyEligibilityRestrictionCode
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolLobbyLobbyGameConfigDto {
    pub allowablePremadeSizes: Vec<i32>,
    pub customLobbyName: String,
    pub customMutatorName: String,
    pub customRewardsDisabledReasons: Vec<String>,
    pub customSpectators: Vec<LolLobbyLobbyParticipantDto>,
    pub customTeam100: Vec<LolLobbyLobbyParticipantDto>,
    pub customTeam200: Vec<LolLobbyLobbyParticipantDto>,
    pub gameMode: String,
    pub isCustom: bool,
    pub isLobbyFull: bool,
    pub isTeamBuilderManaged: bool,
    pub mapId: i32,
    pub maxHumanPlayers: i32,
    pub maxLobbySize: i32,
    pub maxTeamSize: i32,
    pub pickType: String,
    pub premadeSizeAllowed: bool,
    pub queueId: i32,
    pub showPositionSelector: bool,
    pub showPreferredChampions: bool,
    pub customSpectatorPolicy: LolLobbyQueueCustomGameSpectatorPolicy
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolLobbyLobbyInvitationDto {
    pub invitationId: String,
    pub timestamp: String,
    pub toSummonerId: u64,
    pub toSummonerName: String,
    pub invitationType: LolLobbyInvitationType,
    pub state: LolLobbyLobbyInvitationState
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolLobbyLobbyParticipantDto {
    pub allowedChangeActivity: bool,
    pub allowedInviteOthers: bool,
    pub allowedKickOthers: bool,
    pub allowedStartActivity: bool,
    pub allowedToggleInvite: bool,
    pub autoFillEligible: bool,
    pub autoFillProtectedForPromos: bool,
    pub autoFillProtectedForSoloing: bool,
    pub autoFillProtectedForStreaking: bool,
    pub botChampionId: i32,
    pub botId: String,
    pub firstPositionPreference: String,
    pub isBot: bool,
    pub isLeader: bool,
    pub isSpectator: bool,
    pub preferredChampions: Vec<i32>,
    pub puuid: String,
    pub ready: bool,
    pub secondPositionPreference: String,
    pub showGhostedBanner: bool,
    pub summonerIconId: i32,
    pub summonerId: u64,
    pub summonerInternalName: String,
    pub summonerLevel: u32,
    pub summonerName: String,
    pub teamId: i32,
    pub botDifficulty: LolLobbyLobbyBotDifficulty
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolLobbyLobbyDto {
    pub canStartActivity: bool,
    pub chatRoomId: String,
    pub chatRoomKey: String,
    pub gameConfig: LolLobbyLobbyGameConfigDto,
    pub invitations: Vec<LolLobbyLobbyInvitationDto>,
    pub localMember: LolLobbyLobbyParticipantDto,
    pub members: Vec<LolLobbyLobbyParticipantDto>,
    pub multiUserChatJWT: String,
    pub partyId: String,
    pub partyType: String,
    pub restrictions: Option<Vec<LolLobbyEligibilityRestriction>>,
    pub scarcePositions: Vec<String>,
    pub warnings: Option<Vec<LolLobbyEligibilityRestriction>>
}