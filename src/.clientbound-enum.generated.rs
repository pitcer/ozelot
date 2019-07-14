/* This file is automatically generated by packets.clj
Do not manually edit this file, if you wish to make
changes here, then edit and rerun packets.clj */

/// Represents a single packet
#[derive(Debug, PartialEq, Clone)]
pub enum ClientboundPacket {
    StatusResponse(StatusResponse),
    StatusPong(StatusPong),
    LoginDisconnect(LoginDisconnect),
    EncryptionRequest(EncryptionRequest),
    LoginSuccess(LoginSuccess),
    SetCompression(SetCompression),
    LoginPluginRequest(LoginPluginRequest),
    SpawnObject(SpawnObject),
    SpawnExperienceOrb(SpawnExperienceOrb),
    SpawnGlobalEntity(SpawnGlobalEntity),
    SpawnMob(SpawnMob),
    SpawnPainting(SpawnPainting),
    SpawnPlayer(SpawnPlayer),
    ClientboundAnimation(ClientboundAnimation),
    Statistics(Statistics),
    BlockBreakAnimation(BlockBreakAnimation),
    UpdateBlockEntity(UpdateBlockEntity),
    BlockAction(BlockAction),
    BlockChange(BlockChange),
    BossBar(BossBar),
    ServerDifficulty(ServerDifficulty),
    ChatMessage(ChatMessage),
    MultiBlockChange(MultiBlockChange),
    ClientboundTabComplete(ClientboundTabComplete),
    DeclareCommands(DeclareCommands),
    ClientboundConfirmTransaction(ClientboundConfirmTransaction),
    ClientboundCloseWindow(ClientboundCloseWindow),
    OpenWindow(OpenWindow),
    WindowItems(WindowItems),
    WindowProperty(WindowProperty),
    SetSlot(SetSlot),
    SetCooldown(SetCooldown),
    ClientboundPluginMessage(ClientboundPluginMessage),
    NamedSoundEffect(NamedSoundEffect),
    PlayDisconnect(PlayDisconnect),
    EntityStatus(EntityStatus),
    NBTQueryResponse(NBTQueryResponse),
    Explosion(Explosion),
    UnloadChunk(UnloadChunk),
    ChangeGameState(ChangeGameState),
    KeepAlive(KeepAlive),
    ChunkData(ChunkData),
    Effect(Effect),
    Particle(Particle),
    UpdateLight(UpdateLight),
    JoinGame(JoinGame),
    Map(Map),
    Entity(Entity),
    EntityRelativeMove(EntityRelativeMove),
    EntityLookRelativeMove(EntityLookRelativeMove),
    EntityLook(EntityLook),
    ClientboundVehicleMove(ClientboundVehicleMove),
    OpenSignEditor(OpenSignEditor),
    CraftRecipeResponse(CraftRecipeResponse),
    PlayerAbilities(PlayerAbilities),
    CombatEvent(CombatEvent),
    PlayerInfo(PlayerInfo),
    FacePlayer(FacePlayer),
    PlayerPositionAndLook(PlayerPositionAndLook),
    UnlockRecipes(UnlockRecipes),
    DestroyEntities(DestroyEntities),
    RemoveEntityEffect(RemoveEntityEffect),
    ResourcePackSend(ResourcePackSend),
    Respawn(Respawn),
    EntityHeadLook(EntityHeadLook),
    SelectAdvancementTab(SelectAdvancementTab),
    WorldBorder(WorldBorder),
    Camera(Camera),
    ClientboundHeldItemChange(ClientboundHeldItemChange),
    UpdateViewPosition(UpdateViewPosition),
    UpdateViewDistance(UpdateViewDistance),
    DisplayScoreboard(DisplayScoreboard),
    EntityMetadata(EntityMetadata),
    AttachEntity(AttachEntity),
    EntityVelocity(EntityVelocity),
    EntityEquipment(EntityEquipment),
    SetExperience(SetExperience),
    UpdateHealth(UpdateHealth),
    ScoreboardObjective(ScoreboardObjective),
    SetPassengers(SetPassengers),
    Teams(Teams),
    UpdateScore(UpdateScore),
    SpawnPosition(SpawnPosition),
    TimeUpdate(TimeUpdate),
    Title(Title),
    StopSound(StopSound),
    SoundEffect(SoundEffect),
    PlayerListHeaderFooter(PlayerListHeaderFooter),
    CollectItem(CollectItem),
    EntityTeleport(EntityTeleport),
    Advancements(Advancements),
    EntityProperties(EntityProperties),
    EntityEffect(EntityEffect),
    DeclareRecipes(DeclareRecipes),
    Tags(Tags),

}

impl Packet for ClientboundPacket {
    fn deserialize<R: Read>(r: &mut R, state: &ClientState) -> Result<Self> {
        let packet_id = read_varint(r)?;
        match state {
        &ClientState::Handshake => {
            Err("No packet available in this state".into())

        },
        &ClientState::Status => {
            match packet_id {
            0 => Ok(StatusResponse::deserialize(r)?),
            1 => Ok(StatusPong::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },
        &ClientState::Login => {
            match packet_id {
            0 => Ok(LoginDisconnect::deserialize(r)?),
            1 => Ok(EncryptionRequest::deserialize(r)?),
            2 => Ok(LoginSuccess::deserialize(r)?),
            3 => Ok(SetCompression::deserialize(r)?),
            4 => Ok(LoginPluginRequest::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },
        &ClientState::Play => {
            match packet_id {
            0 => Ok(SpawnObject::deserialize(r)?),
            1 => Ok(SpawnExperienceOrb::deserialize(r)?),
            2 => Ok(SpawnGlobalEntity::deserialize(r)?),
            3 => Ok(SpawnMob::deserialize(r)?),
            4 => Ok(SpawnPainting::deserialize(r)?),
            5 => Ok(SpawnPlayer::deserialize(r)?),
            6 => Ok(ClientboundAnimation::deserialize(r)?),
            7 => Ok(Statistics::deserialize(r)?),
            8 => Ok(BlockBreakAnimation::deserialize(r)?),
            9 => Ok(UpdateBlockEntity::deserialize(r)?),
            10 => Ok(BlockAction::deserialize(r)?),
            11 => Ok(BlockChange::deserialize(r)?),
            12 => Ok(BossBar::deserialize(r)?),
            13 => Ok(ServerDifficulty::deserialize(r)?),
            14 => Ok(ChatMessage::deserialize(r)?),
            15 => Ok(MultiBlockChange::deserialize(r)?),
            16 => Ok(ClientboundTabComplete::deserialize(r)?),
            17 => Ok(DeclareCommands::deserialize(r)?),
            18 => Ok(ClientboundConfirmTransaction::deserialize(r)?),
            19 => Ok(ClientboundCloseWindow::deserialize(r)?),
            46 => Ok(OpenWindow::deserialize(r)?),
            20 => Ok(WindowItems::deserialize(r)?),
            21 => Ok(WindowProperty::deserialize(r)?),
            22 => Ok(SetSlot::deserialize(r)?),
            23 => Ok(SetCooldown::deserialize(r)?),
            24 => Ok(ClientboundPluginMessage::deserialize(r)?),
            25 => Ok(NamedSoundEffect::deserialize(r)?),
            26 => Ok(PlayDisconnect::deserialize(r)?),
            27 => Ok(EntityStatus::deserialize(r)?),
            84 => Ok(NBTQueryResponse::deserialize(r)?),
            28 => Ok(Explosion::deserialize(r)?),
            29 => Ok(UnloadChunk::deserialize(r)?),
            30 => Ok(ChangeGameState::deserialize(r)?),
            32 => Ok(KeepAlive::deserialize(r)?),
            33 => Ok(ChunkData::deserialize(r)?),
            34 => Ok(Effect::deserialize(r)?),
            35 => Ok(Particle::deserialize(r)?),
            36 => Ok(UpdateLight::deserialize(r)?),
            37 => Ok(JoinGame::deserialize(r)?),
            38 => Ok(Map::deserialize(r)?),
            43 => Ok(Entity::deserialize(r)?),
            40 => Ok(EntityRelativeMove::deserialize(r)?),
            41 => Ok(EntityLookRelativeMove::deserialize(r)?),
            42 => Ok(EntityLook::deserialize(r)?),
            44 => Ok(ClientboundVehicleMove::deserialize(r)?),
            47 => Ok(OpenSignEditor::deserialize(r)?),
            48 => Ok(CraftRecipeResponse::deserialize(r)?),
            49 => Ok(PlayerAbilities::deserialize(r)?),
            50 => Ok(CombatEvent::deserialize(r)?),
            51 => Ok(PlayerInfo::deserialize(r)?),
            52 => Ok(FacePlayer::deserialize(r)?),
            53 => Ok(PlayerPositionAndLook::deserialize(r)?),
            54 => Ok(UnlockRecipes::deserialize(r)?),
            55 => Ok(DestroyEntities::deserialize(r)?),
            56 => Ok(RemoveEntityEffect::deserialize(r)?),
            57 => Ok(ResourcePackSend::deserialize(r)?),
            58 => Ok(Respawn::deserialize(r)?),
            59 => Ok(EntityHeadLook::deserialize(r)?),
            60 => Ok(SelectAdvancementTab::deserialize(r)?),
            61 => Ok(WorldBorder::deserialize(r)?),
            62 => Ok(Camera::deserialize(r)?),
            63 => Ok(ClientboundHeldItemChange::deserialize(r)?),
            64 => Ok(UpdateViewPosition::deserialize(r)?),
            65 => Ok(UpdateViewDistance::deserialize(r)?),
            66 => Ok(DisplayScoreboard::deserialize(r)?),
            67 => Ok(EntityMetadata::deserialize(r)?),
            68 => Ok(AttachEntity::deserialize(r)?),
            69 => Ok(EntityVelocity::deserialize(r)?),
            70 => Ok(EntityEquipment::deserialize(r)?),
            71 => Ok(SetExperience::deserialize(r)?),
            72 => Ok(UpdateHealth::deserialize(r)?),
            73 => Ok(ScoreboardObjective::deserialize(r)?),
            74 => Ok(SetPassengers::deserialize(r)?),
            75 => Ok(Teams::deserialize(r)?),
            76 => Ok(UpdateScore::deserialize(r)?),
            77 => Ok(SpawnPosition::deserialize(r)?),
            78 => Ok(TimeUpdate::deserialize(r)?),
            79 => Ok(Title::deserialize(r)?),
            82 => Ok(StopSound::deserialize(r)?),
            81 => Ok(SoundEffect::deserialize(r)?),
            83 => Ok(PlayerListHeaderFooter::deserialize(r)?),
            85 => Ok(CollectItem::deserialize(r)?),
            86 => Ok(EntityTeleport::deserialize(r)?),
            87 => Ok(Advancements::deserialize(r)?),
            88 => Ok(EntityProperties::deserialize(r)?),
            89 => Ok(EntityEffect::deserialize(r)?),
            90 => Ok(DeclareRecipes::deserialize(r)?),
            91 => Ok(Tags::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },

        }
    }
    fn get_packet_name(&self) -> &str {
        match self {
        &ClientboundPacket::StatusResponse(..) => "StatusResponse",
        &ClientboundPacket::StatusPong(..) => "StatusPong",
        &ClientboundPacket::LoginDisconnect(..) => "LoginDisconnect",
        &ClientboundPacket::EncryptionRequest(..) => "EncryptionRequest",
        &ClientboundPacket::LoginSuccess(..) => "LoginSuccess",
        &ClientboundPacket::SetCompression(..) => "SetCompression",
        &ClientboundPacket::LoginPluginRequest(..) => "LoginPluginRequest",
        &ClientboundPacket::SpawnObject(..) => "SpawnObject",
        &ClientboundPacket::SpawnExperienceOrb(..) => "SpawnExperienceOrb",
        &ClientboundPacket::SpawnGlobalEntity(..) => "SpawnGlobalEntity",
        &ClientboundPacket::SpawnMob(..) => "SpawnMob",
        &ClientboundPacket::SpawnPainting(..) => "SpawnPainting",
        &ClientboundPacket::SpawnPlayer(..) => "SpawnPlayer",
        &ClientboundPacket::ClientboundAnimation(..) => "ClientboundAnimation",
        &ClientboundPacket::Statistics(..) => "Statistics",
        &ClientboundPacket::BlockBreakAnimation(..) => "BlockBreakAnimation",
        &ClientboundPacket::UpdateBlockEntity(..) => "UpdateBlockEntity",
        &ClientboundPacket::BlockAction(..) => "BlockAction",
        &ClientboundPacket::BlockChange(..) => "BlockChange",
        &ClientboundPacket::BossBar(..) => "BossBar",
        &ClientboundPacket::ServerDifficulty(..) => "ServerDifficulty",
        &ClientboundPacket::ChatMessage(..) => "ChatMessage",
        &ClientboundPacket::MultiBlockChange(..) => "MultiBlockChange",
        &ClientboundPacket::ClientboundTabComplete(..) => "ClientboundTabComplete",
        &ClientboundPacket::DeclareCommands(..) => "DeclareCommands",
        &ClientboundPacket::ClientboundConfirmTransaction(..) => "ClientboundConfirmTransaction",
        &ClientboundPacket::ClientboundCloseWindow(..) => "ClientboundCloseWindow",
        &ClientboundPacket::OpenWindow(..) => "OpenWindow",
        &ClientboundPacket::WindowItems(..) => "WindowItems",
        &ClientboundPacket::WindowProperty(..) => "WindowProperty",
        &ClientboundPacket::SetSlot(..) => "SetSlot",
        &ClientboundPacket::SetCooldown(..) => "SetCooldown",
        &ClientboundPacket::ClientboundPluginMessage(..) => "ClientboundPluginMessage",
        &ClientboundPacket::NamedSoundEffect(..) => "NamedSoundEffect",
        &ClientboundPacket::PlayDisconnect(..) => "PlayDisconnect",
        &ClientboundPacket::EntityStatus(..) => "EntityStatus",
        &ClientboundPacket::NBTQueryResponse(..) => "NBTQueryResponse",
        &ClientboundPacket::Explosion(..) => "Explosion",
        &ClientboundPacket::UnloadChunk(..) => "UnloadChunk",
        &ClientboundPacket::ChangeGameState(..) => "ChangeGameState",
        &ClientboundPacket::KeepAlive(..) => "KeepAlive",
        &ClientboundPacket::ChunkData(..) => "ChunkData",
        &ClientboundPacket::Effect(..) => "Effect",
        &ClientboundPacket::Particle(..) => "Particle",
        &ClientboundPacket::UpdateLight(..) => "UpdateLight",
        &ClientboundPacket::JoinGame(..) => "JoinGame",
        &ClientboundPacket::Map(..) => "Map",
        &ClientboundPacket::Entity(..) => "Entity",
        &ClientboundPacket::EntityRelativeMove(..) => "EntityRelativeMove",
        &ClientboundPacket::EntityLookRelativeMove(..) => "EntityLookRelativeMove",
        &ClientboundPacket::EntityLook(..) => "EntityLook",
        &ClientboundPacket::ClientboundVehicleMove(..) => "ClientboundVehicleMove",
        &ClientboundPacket::OpenSignEditor(..) => "OpenSignEditor",
        &ClientboundPacket::CraftRecipeResponse(..) => "CraftRecipeResponse",
        &ClientboundPacket::PlayerAbilities(..) => "PlayerAbilities",
        &ClientboundPacket::CombatEvent(..) => "CombatEvent",
        &ClientboundPacket::PlayerInfo(..) => "PlayerInfo",
        &ClientboundPacket::FacePlayer(..) => "FacePlayer",
        &ClientboundPacket::PlayerPositionAndLook(..) => "PlayerPositionAndLook",
        &ClientboundPacket::UnlockRecipes(..) => "UnlockRecipes",
        &ClientboundPacket::DestroyEntities(..) => "DestroyEntities",
        &ClientboundPacket::RemoveEntityEffect(..) => "RemoveEntityEffect",
        &ClientboundPacket::ResourcePackSend(..) => "ResourcePackSend",
        &ClientboundPacket::Respawn(..) => "Respawn",
        &ClientboundPacket::EntityHeadLook(..) => "EntityHeadLook",
        &ClientboundPacket::SelectAdvancementTab(..) => "SelectAdvancementTab",
        &ClientboundPacket::WorldBorder(..) => "WorldBorder",
        &ClientboundPacket::Camera(..) => "Camera",
        &ClientboundPacket::ClientboundHeldItemChange(..) => "ClientboundHeldItemChange",
        &ClientboundPacket::UpdateViewPosition(..) => "UpdateViewPosition",
        &ClientboundPacket::UpdateViewDistance(..) => "UpdateViewDistance",
        &ClientboundPacket::DisplayScoreboard(..) => "DisplayScoreboard",
        &ClientboundPacket::EntityMetadata(..) => "EntityMetadata",
        &ClientboundPacket::AttachEntity(..) => "AttachEntity",
        &ClientboundPacket::EntityVelocity(..) => "EntityVelocity",
        &ClientboundPacket::EntityEquipment(..) => "EntityEquipment",
        &ClientboundPacket::SetExperience(..) => "SetExperience",
        &ClientboundPacket::UpdateHealth(..) => "UpdateHealth",
        &ClientboundPacket::ScoreboardObjective(..) => "ScoreboardObjective",
        &ClientboundPacket::SetPassengers(..) => "SetPassengers",
        &ClientboundPacket::Teams(..) => "Teams",
        &ClientboundPacket::UpdateScore(..) => "UpdateScore",
        &ClientboundPacket::SpawnPosition(..) => "SpawnPosition",
        &ClientboundPacket::TimeUpdate(..) => "TimeUpdate",
        &ClientboundPacket::Title(..) => "Title",
        &ClientboundPacket::StopSound(..) => "StopSound",
        &ClientboundPacket::SoundEffect(..) => "SoundEffect",
        &ClientboundPacket::PlayerListHeaderFooter(..) => "PlayerListHeaderFooter",
        &ClientboundPacket::CollectItem(..) => "CollectItem",
        &ClientboundPacket::EntityTeleport(..) => "EntityTeleport",
        &ClientboundPacket::Advancements(..) => "Advancements",
        &ClientboundPacket::EntityProperties(..) => "EntityProperties",
        &ClientboundPacket::EntityEffect(..) => "EntityEffect",
        &ClientboundPacket::DeclareRecipes(..) => "DeclareRecipes",
        &ClientboundPacket::Tags(..) => "Tags",

        }
    }
    fn get_clientstate(&self) -> ClientState {
        match self {
        &ClientboundPacket::StatusResponse(..) => ClientState::Status,
        &ClientboundPacket::StatusPong(..) => ClientState::Status,
        &ClientboundPacket::LoginDisconnect(..) => ClientState::Login,
        &ClientboundPacket::EncryptionRequest(..) => ClientState::Login,
        &ClientboundPacket::LoginSuccess(..) => ClientState::Login,
        &ClientboundPacket::SetCompression(..) => ClientState::Login,
        &ClientboundPacket::LoginPluginRequest(..) => ClientState::Login,
        &ClientboundPacket::SpawnObject(..) => ClientState::Play,
        &ClientboundPacket::SpawnExperienceOrb(..) => ClientState::Play,
        &ClientboundPacket::SpawnGlobalEntity(..) => ClientState::Play,
        &ClientboundPacket::SpawnMob(..) => ClientState::Play,
        &ClientboundPacket::SpawnPainting(..) => ClientState::Play,
        &ClientboundPacket::SpawnPlayer(..) => ClientState::Play,
        &ClientboundPacket::ClientboundAnimation(..) => ClientState::Play,
        &ClientboundPacket::Statistics(..) => ClientState::Play,
        &ClientboundPacket::BlockBreakAnimation(..) => ClientState::Play,
        &ClientboundPacket::UpdateBlockEntity(..) => ClientState::Play,
        &ClientboundPacket::BlockAction(..) => ClientState::Play,
        &ClientboundPacket::BlockChange(..) => ClientState::Play,
        &ClientboundPacket::BossBar(..) => ClientState::Play,
        &ClientboundPacket::ServerDifficulty(..) => ClientState::Play,
        &ClientboundPacket::ChatMessage(..) => ClientState::Play,
        &ClientboundPacket::MultiBlockChange(..) => ClientState::Play,
        &ClientboundPacket::ClientboundTabComplete(..) => ClientState::Play,
        &ClientboundPacket::DeclareCommands(..) => ClientState::Play,
        &ClientboundPacket::ClientboundConfirmTransaction(..) => ClientState::Play,
        &ClientboundPacket::ClientboundCloseWindow(..) => ClientState::Play,
        &ClientboundPacket::OpenWindow(..) => ClientState::Play,
        &ClientboundPacket::WindowItems(..) => ClientState::Play,
        &ClientboundPacket::WindowProperty(..) => ClientState::Play,
        &ClientboundPacket::SetSlot(..) => ClientState::Play,
        &ClientboundPacket::SetCooldown(..) => ClientState::Play,
        &ClientboundPacket::ClientboundPluginMessage(..) => ClientState::Play,
        &ClientboundPacket::NamedSoundEffect(..) => ClientState::Play,
        &ClientboundPacket::PlayDisconnect(..) => ClientState::Play,
        &ClientboundPacket::EntityStatus(..) => ClientState::Play,
        &ClientboundPacket::NBTQueryResponse(..) => ClientState::Play,
        &ClientboundPacket::Explosion(..) => ClientState::Play,
        &ClientboundPacket::UnloadChunk(..) => ClientState::Play,
        &ClientboundPacket::ChangeGameState(..) => ClientState::Play,
        &ClientboundPacket::KeepAlive(..) => ClientState::Play,
        &ClientboundPacket::ChunkData(..) => ClientState::Play,
        &ClientboundPacket::Effect(..) => ClientState::Play,
        &ClientboundPacket::Particle(..) => ClientState::Play,
        &ClientboundPacket::UpdateLight(..) => ClientState::Play,
        &ClientboundPacket::JoinGame(..) => ClientState::Play,
        &ClientboundPacket::Map(..) => ClientState::Play,
        &ClientboundPacket::Entity(..) => ClientState::Play,
        &ClientboundPacket::EntityRelativeMove(..) => ClientState::Play,
        &ClientboundPacket::EntityLookRelativeMove(..) => ClientState::Play,
        &ClientboundPacket::EntityLook(..) => ClientState::Play,
        &ClientboundPacket::ClientboundVehicleMove(..) => ClientState::Play,
        &ClientboundPacket::OpenSignEditor(..) => ClientState::Play,
        &ClientboundPacket::CraftRecipeResponse(..) => ClientState::Play,
        &ClientboundPacket::PlayerAbilities(..) => ClientState::Play,
        &ClientboundPacket::CombatEvent(..) => ClientState::Play,
        &ClientboundPacket::PlayerInfo(..) => ClientState::Play,
        &ClientboundPacket::FacePlayer(..) => ClientState::Play,
        &ClientboundPacket::PlayerPositionAndLook(..) => ClientState::Play,
        &ClientboundPacket::UnlockRecipes(..) => ClientState::Play,
        &ClientboundPacket::DestroyEntities(..) => ClientState::Play,
        &ClientboundPacket::RemoveEntityEffect(..) => ClientState::Play,
        &ClientboundPacket::ResourcePackSend(..) => ClientState::Play,
        &ClientboundPacket::Respawn(..) => ClientState::Play,
        &ClientboundPacket::EntityHeadLook(..) => ClientState::Play,
        &ClientboundPacket::SelectAdvancementTab(..) => ClientState::Play,
        &ClientboundPacket::WorldBorder(..) => ClientState::Play,
        &ClientboundPacket::Camera(..) => ClientState::Play,
        &ClientboundPacket::ClientboundHeldItemChange(..) => ClientState::Play,
        &ClientboundPacket::UpdateViewPosition(..) => ClientState::Play,
        &ClientboundPacket::UpdateViewDistance(..) => ClientState::Play,
        &ClientboundPacket::DisplayScoreboard(..) => ClientState::Play,
        &ClientboundPacket::EntityMetadata(..) => ClientState::Play,
        &ClientboundPacket::AttachEntity(..) => ClientState::Play,
        &ClientboundPacket::EntityVelocity(..) => ClientState::Play,
        &ClientboundPacket::EntityEquipment(..) => ClientState::Play,
        &ClientboundPacket::SetExperience(..) => ClientState::Play,
        &ClientboundPacket::UpdateHealth(..) => ClientState::Play,
        &ClientboundPacket::ScoreboardObjective(..) => ClientState::Play,
        &ClientboundPacket::SetPassengers(..) => ClientState::Play,
        &ClientboundPacket::Teams(..) => ClientState::Play,
        &ClientboundPacket::UpdateScore(..) => ClientState::Play,
        &ClientboundPacket::SpawnPosition(..) => ClientState::Play,
        &ClientboundPacket::TimeUpdate(..) => ClientState::Play,
        &ClientboundPacket::Title(..) => ClientState::Play,
        &ClientboundPacket::StopSound(..) => ClientState::Play,
        &ClientboundPacket::SoundEffect(..) => ClientState::Play,
        &ClientboundPacket::PlayerListHeaderFooter(..) => ClientState::Play,
        &ClientboundPacket::CollectItem(..) => ClientState::Play,
        &ClientboundPacket::EntityTeleport(..) => ClientState::Play,
        &ClientboundPacket::Advancements(..) => ClientState::Play,
        &ClientboundPacket::EntityProperties(..) => ClientState::Play,
        &ClientboundPacket::EntityEffect(..) => ClientState::Play,
        &ClientboundPacket::DeclareRecipes(..) => ClientState::Play,
        &ClientboundPacket::Tags(..) => ClientState::Play,

        }
    }
    fn get_id(&self) -> i32 {
        match self {
        &ClientboundPacket::StatusResponse(..) => 0,
        &ClientboundPacket::StatusPong(..) => 1,
        &ClientboundPacket::LoginDisconnect(..) => 0,
        &ClientboundPacket::EncryptionRequest(..) => 1,
        &ClientboundPacket::LoginSuccess(..) => 2,
        &ClientboundPacket::SetCompression(..) => 3,
        &ClientboundPacket::LoginPluginRequest(..) => 4,
        &ClientboundPacket::SpawnObject(..) => 0,
        &ClientboundPacket::SpawnExperienceOrb(..) => 1,
        &ClientboundPacket::SpawnGlobalEntity(..) => 2,
        &ClientboundPacket::SpawnMob(..) => 3,
        &ClientboundPacket::SpawnPainting(..) => 4,
        &ClientboundPacket::SpawnPlayer(..) => 5,
        &ClientboundPacket::ClientboundAnimation(..) => 6,
        &ClientboundPacket::Statistics(..) => 7,
        &ClientboundPacket::BlockBreakAnimation(..) => 8,
        &ClientboundPacket::UpdateBlockEntity(..) => 9,
        &ClientboundPacket::BlockAction(..) => 10,
        &ClientboundPacket::BlockChange(..) => 11,
        &ClientboundPacket::BossBar(..) => 12,
        &ClientboundPacket::ServerDifficulty(..) => 13,
        &ClientboundPacket::ChatMessage(..) => 14,
        &ClientboundPacket::MultiBlockChange(..) => 15,
        &ClientboundPacket::ClientboundTabComplete(..) => 16,
        &ClientboundPacket::DeclareCommands(..) => 17,
        &ClientboundPacket::ClientboundConfirmTransaction(..) => 18,
        &ClientboundPacket::ClientboundCloseWindow(..) => 19,
        &ClientboundPacket::OpenWindow(..) => 46,
        &ClientboundPacket::WindowItems(..) => 20,
        &ClientboundPacket::WindowProperty(..) => 21,
        &ClientboundPacket::SetSlot(..) => 22,
        &ClientboundPacket::SetCooldown(..) => 23,
        &ClientboundPacket::ClientboundPluginMessage(..) => 24,
        &ClientboundPacket::NamedSoundEffect(..) => 25,
        &ClientboundPacket::PlayDisconnect(..) => 26,
        &ClientboundPacket::EntityStatus(..) => 27,
        &ClientboundPacket::NBTQueryResponse(..) => 84,
        &ClientboundPacket::Explosion(..) => 28,
        &ClientboundPacket::UnloadChunk(..) => 29,
        &ClientboundPacket::ChangeGameState(..) => 30,
        &ClientboundPacket::KeepAlive(..) => 32,
        &ClientboundPacket::ChunkData(..) => 33,
        &ClientboundPacket::Effect(..) => 34,
        &ClientboundPacket::Particle(..) => 35,
        &ClientboundPacket::UpdateLight(..) => 36,
        &ClientboundPacket::JoinGame(..) => 37,
        &ClientboundPacket::Map(..) => 38,
        &ClientboundPacket::Entity(..) => 43,
        &ClientboundPacket::EntityRelativeMove(..) => 40,
        &ClientboundPacket::EntityLookRelativeMove(..) => 41,
        &ClientboundPacket::EntityLook(..) => 42,
        &ClientboundPacket::ClientboundVehicleMove(..) => 44,
        &ClientboundPacket::OpenSignEditor(..) => 47,
        &ClientboundPacket::CraftRecipeResponse(..) => 48,
        &ClientboundPacket::PlayerAbilities(..) => 49,
        &ClientboundPacket::CombatEvent(..) => 50,
        &ClientboundPacket::PlayerInfo(..) => 51,
        &ClientboundPacket::FacePlayer(..) => 52,
        &ClientboundPacket::PlayerPositionAndLook(..) => 53,
        &ClientboundPacket::UnlockRecipes(..) => 54,
        &ClientboundPacket::DestroyEntities(..) => 55,
        &ClientboundPacket::RemoveEntityEffect(..) => 56,
        &ClientboundPacket::ResourcePackSend(..) => 57,
        &ClientboundPacket::Respawn(..) => 58,
        &ClientboundPacket::EntityHeadLook(..) => 59,
        &ClientboundPacket::SelectAdvancementTab(..) => 60,
        &ClientboundPacket::WorldBorder(..) => 61,
        &ClientboundPacket::Camera(..) => 62,
        &ClientboundPacket::ClientboundHeldItemChange(..) => 63,
        &ClientboundPacket::UpdateViewPosition(..) => 64,
        &ClientboundPacket::UpdateViewDistance(..) => 65,
        &ClientboundPacket::DisplayScoreboard(..) => 66,
        &ClientboundPacket::EntityMetadata(..) => 67,
        &ClientboundPacket::AttachEntity(..) => 68,
        &ClientboundPacket::EntityVelocity(..) => 69,
        &ClientboundPacket::EntityEquipment(..) => 70,
        &ClientboundPacket::SetExperience(..) => 71,
        &ClientboundPacket::UpdateHealth(..) => 72,
        &ClientboundPacket::ScoreboardObjective(..) => 73,
        &ClientboundPacket::SetPassengers(..) => 74,
        &ClientboundPacket::Teams(..) => 75,
        &ClientboundPacket::UpdateScore(..) => 76,
        &ClientboundPacket::SpawnPosition(..) => 77,
        &ClientboundPacket::TimeUpdate(..) => 78,
        &ClientboundPacket::Title(..) => 79,
        &ClientboundPacket::StopSound(..) => 82,
        &ClientboundPacket::SoundEffect(..) => 81,
        &ClientboundPacket::PlayerListHeaderFooter(..) => 83,
        &ClientboundPacket::CollectItem(..) => 85,
        &ClientboundPacket::EntityTeleport(..) => 86,
        &ClientboundPacket::Advancements(..) => 87,
        &ClientboundPacket::EntityProperties(..) => 88,
        &ClientboundPacket::EntityEffect(..) => 89,
        &ClientboundPacket::DeclareRecipes(..) => 90,
        &ClientboundPacket::Tags(..) => 91,

        }
    }
    fn to_u8(&self) -> Result<Vec<u8>> {
        match self {
        &ClientboundPacket::StatusResponse(ref x) => x.to_u8(),
        &ClientboundPacket::StatusPong(ref x) => x.to_u8(),
        &ClientboundPacket::LoginDisconnect(ref x) => x.to_u8(),
        &ClientboundPacket::EncryptionRequest(ref x) => x.to_u8(),
        &ClientboundPacket::LoginSuccess(ref x) => x.to_u8(),
        &ClientboundPacket::SetCompression(ref x) => x.to_u8(),
        &ClientboundPacket::LoginPluginRequest(ref x) => x.to_u8(),
        &ClientboundPacket::SpawnObject(ref x) => x.to_u8(),
        &ClientboundPacket::SpawnExperienceOrb(ref x) => x.to_u8(),
        &ClientboundPacket::SpawnGlobalEntity(ref x) => x.to_u8(),
        &ClientboundPacket::SpawnMob(ref x) => x.to_u8(),
        &ClientboundPacket::SpawnPainting(ref x) => x.to_u8(),
        &ClientboundPacket::SpawnPlayer(ref x) => x.to_u8(),
        &ClientboundPacket::ClientboundAnimation(ref x) => x.to_u8(),
        &ClientboundPacket::Statistics(ref x) => x.to_u8(),
        &ClientboundPacket::BlockBreakAnimation(ref x) => x.to_u8(),
        &ClientboundPacket::UpdateBlockEntity(ref x) => x.to_u8(),
        &ClientboundPacket::BlockAction(ref x) => x.to_u8(),
        &ClientboundPacket::BlockChange(ref x) => x.to_u8(),
        &ClientboundPacket::BossBar(ref x) => x.to_u8(),
        &ClientboundPacket::ServerDifficulty(ref x) => x.to_u8(),
        &ClientboundPacket::ChatMessage(ref x) => x.to_u8(),
        &ClientboundPacket::MultiBlockChange(ref x) => x.to_u8(),
        &ClientboundPacket::ClientboundTabComplete(ref x) => x.to_u8(),
        &ClientboundPacket::DeclareCommands(ref x) => x.to_u8(),
        &ClientboundPacket::ClientboundConfirmTransaction(ref x) => x.to_u8(),
        &ClientboundPacket::ClientboundCloseWindow(ref x) => x.to_u8(),
        &ClientboundPacket::OpenWindow(ref x) => x.to_u8(),
        &ClientboundPacket::WindowItems(ref x) => x.to_u8(),
        &ClientboundPacket::WindowProperty(ref x) => x.to_u8(),
        &ClientboundPacket::SetSlot(ref x) => x.to_u8(),
        &ClientboundPacket::SetCooldown(ref x) => x.to_u8(),
        &ClientboundPacket::ClientboundPluginMessage(ref x) => x.to_u8(),
        &ClientboundPacket::NamedSoundEffect(ref x) => x.to_u8(),
        &ClientboundPacket::PlayDisconnect(ref x) => x.to_u8(),
        &ClientboundPacket::EntityStatus(ref x) => x.to_u8(),
        &ClientboundPacket::NBTQueryResponse(ref x) => x.to_u8(),
        &ClientboundPacket::Explosion(ref x) => x.to_u8(),
        &ClientboundPacket::UnloadChunk(ref x) => x.to_u8(),
        &ClientboundPacket::ChangeGameState(ref x) => x.to_u8(),
        &ClientboundPacket::KeepAlive(ref x) => x.to_u8(),
        &ClientboundPacket::ChunkData(ref x) => x.to_u8(),
        &ClientboundPacket::Effect(ref x) => x.to_u8(),
        &ClientboundPacket::Particle(ref x) => x.to_u8(),
        &ClientboundPacket::UpdateLight(ref x) => x.to_u8(),
        &ClientboundPacket::JoinGame(ref x) => x.to_u8(),
        &ClientboundPacket::Map(ref x) => x.to_u8(),
        &ClientboundPacket::Entity(ref x) => x.to_u8(),
        &ClientboundPacket::EntityRelativeMove(ref x) => x.to_u8(),
        &ClientboundPacket::EntityLookRelativeMove(ref x) => x.to_u8(),
        &ClientboundPacket::EntityLook(ref x) => x.to_u8(),
        &ClientboundPacket::ClientboundVehicleMove(ref x) => x.to_u8(),
        &ClientboundPacket::OpenSignEditor(ref x) => x.to_u8(),
        &ClientboundPacket::CraftRecipeResponse(ref x) => x.to_u8(),
        &ClientboundPacket::PlayerAbilities(ref x) => x.to_u8(),
        &ClientboundPacket::CombatEvent(ref x) => x.to_u8(),
        &ClientboundPacket::PlayerInfo(ref x) => x.to_u8(),
        &ClientboundPacket::FacePlayer(ref x) => x.to_u8(),
        &ClientboundPacket::PlayerPositionAndLook(ref x) => x.to_u8(),
        &ClientboundPacket::UnlockRecipes(ref x) => x.to_u8(),
        &ClientboundPacket::DestroyEntities(ref x) => x.to_u8(),
        &ClientboundPacket::RemoveEntityEffect(ref x) => x.to_u8(),
        &ClientboundPacket::ResourcePackSend(ref x) => x.to_u8(),
        &ClientboundPacket::Respawn(ref x) => x.to_u8(),
        &ClientboundPacket::EntityHeadLook(ref x) => x.to_u8(),
        &ClientboundPacket::SelectAdvancementTab(ref x) => x.to_u8(),
        &ClientboundPacket::WorldBorder(ref x) => x.to_u8(),
        &ClientboundPacket::Camera(ref x) => x.to_u8(),
        &ClientboundPacket::ClientboundHeldItemChange(ref x) => x.to_u8(),
        &ClientboundPacket::UpdateViewPosition(ref x) => x.to_u8(),
        &ClientboundPacket::UpdateViewDistance(ref x) => x.to_u8(),
        &ClientboundPacket::DisplayScoreboard(ref x) => x.to_u8(),
        &ClientboundPacket::EntityMetadata(ref x) => x.to_u8(),
        &ClientboundPacket::AttachEntity(ref x) => x.to_u8(),
        &ClientboundPacket::EntityVelocity(ref x) => x.to_u8(),
        &ClientboundPacket::EntityEquipment(ref x) => x.to_u8(),
        &ClientboundPacket::SetExperience(ref x) => x.to_u8(),
        &ClientboundPacket::UpdateHealth(ref x) => x.to_u8(),
        &ClientboundPacket::ScoreboardObjective(ref x) => x.to_u8(),
        &ClientboundPacket::SetPassengers(ref x) => x.to_u8(),
        &ClientboundPacket::Teams(ref x) => x.to_u8(),
        &ClientboundPacket::UpdateScore(ref x) => x.to_u8(),
        &ClientboundPacket::SpawnPosition(ref x) => x.to_u8(),
        &ClientboundPacket::TimeUpdate(ref x) => x.to_u8(),
        &ClientboundPacket::Title(ref x) => x.to_u8(),
        &ClientboundPacket::StopSound(ref x) => x.to_u8(),
        &ClientboundPacket::SoundEffect(ref x) => x.to_u8(),
        &ClientboundPacket::PlayerListHeaderFooter(ref x) => x.to_u8(),
        &ClientboundPacket::CollectItem(ref x) => x.to_u8(),
        &ClientboundPacket::EntityTeleport(ref x) => x.to_u8(),
        &ClientboundPacket::Advancements(ref x) => x.to_u8(),
        &ClientboundPacket::EntityProperties(ref x) => x.to_u8(),
        &ClientboundPacket::EntityEffect(ref x) => x.to_u8(),
        &ClientboundPacket::DeclareRecipes(ref x) => x.to_u8(),
        &ClientboundPacket::Tags(ref x) => x.to_u8(),

        }
    }
}
impl fmt::Display for ClientboundPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClientboundPacket of type {}", self.get_packet_name())
    }
}