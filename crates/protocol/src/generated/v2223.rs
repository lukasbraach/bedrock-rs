#![allow(unused)]
#[cfg(feature = "v2223")]
mod inner {
    use crate::ProtoVersion;
    use crate::ProtoVersionEnums;
    use crate::ProtoVersionPackets;
    use crate::ProtoVersionTypes;
    #[derive(Clone, std::fmt::Debug)]
    pub enum V2223 {
        ActorEventPacket(Box<<Self as ProtoVersionPackets>::ActorEventPacket>),
        ActorPickRequestPacket(Box<<Self as ProtoVersionPackets>::ActorPickRequestPacket>),
        AddActorPacket(Box<<Self as ProtoVersionPackets>::AddActorPacket>),
        AddBehaviourTreePacket(Box<<Self as ProtoVersionPackets>::AddBehaviourTreePacket>),
        AddItemActorPacket(Box<<Self as ProtoVersionPackets>::AddItemActorPacket>),
        AddPaintingPacket(Box<<Self as ProtoVersionPackets>::AddPaintingPacket>),
        AddPlayerPacket(Box<<Self as ProtoVersionPackets>::AddPlayerPacket>),
        AddVolumeEntityPacket(Box<<Self as ProtoVersionPackets>::AddVolumeEntityPacket>),
        AgentActionEventPacket(Box<<Self as ProtoVersionPackets>::AgentActionEventPacket>),
        AgentAnimationPacket(Box<<Self as ProtoVersionPackets>::AgentAnimationPacket>),
        AnimateEntityPacket(Box<<Self as ProtoVersionPackets>::AnimateEntityPacket>),
        AnimatePacket(Box<<Self as ProtoVersionPackets>::AnimatePacket>),
        AnvilDamagePacket(Box<<Self as ProtoVersionPackets>::AnvilDamagePacket>),
        AutomationClientConnectPacket(
            Box<<Self as ProtoVersionPackets>::AutomationClientConnectPacket>,
        ),
        AvailableActorIdentifiersPacket(
            Box<<Self as ProtoVersionPackets>::AvailableActorIdentifiersPacket>,
        ),
        AvailableCommandsPacket(Box<<Self as ProtoVersionPackets>::AvailableCommandsPacket>),
        AwardAchievementPacket(Box<<Self as ProtoVersionPackets>::AwardAchievementPacket>),
        BiomeDefinitionListPacket(Box<<Self as ProtoVersionPackets>::BiomeDefinitionListPacket>),
        BlockActorDataPacket(Box<<Self as ProtoVersionPackets>::BlockActorDataPacket>),
        BlockEventPacket(Box<<Self as ProtoVersionPackets>::BlockEventPacket>),
        BlockPickRequestPacket(Box<<Self as ProtoVersionPackets>::BlockPickRequestPacket>),
        BookEditPacket(Box<<Self as ProtoVersionPackets>::BookEditPacket>),
        BossEventPacket(Box<<Self as ProtoVersionPackets>::BossEventPacket>),
        CameraAimAssistActorPriorityPacket(
            Box<<Self as ProtoVersionPackets>::CameraAimAssistActorPriorityPacket>,
        ),
        CameraAimAssistInstructionPacket(
            Box<<Self as ProtoVersionPackets>::CameraAimAssistInstructionPacket>,
        ),
        CameraAimAssistPacket(Box<<Self as ProtoVersionPackets>::CameraAimAssistPacket>),
        CameraAimAssistPresetsPacket(
            Box<<Self as ProtoVersionPackets>::CameraAimAssistPresetsPacket>,
        ),
        CameraInstructionPacket(Box<<Self as ProtoVersionPackets>::CameraInstructionPacket>),
        CameraPacket(Box<<Self as ProtoVersionPackets>::CameraPacket>),
        CameraPresetsPacket(Box<<Self as ProtoVersionPackets>::CameraPresetsPacket>),
        CameraShakePacket(Box<<Self as ProtoVersionPackets>::CameraShakePacket>),
        CameraSplinePacket(Box<<Self as ProtoVersionPackets>::CameraSplinePacket>),
        ChangeDimensionPacket(Box<<Self as ProtoVersionPackets>::ChangeDimensionPacket>),
        ChangeMobPropertyPacket(Box<<Self as ProtoVersionPackets>::ChangeMobPropertyPacket>),
        ChunkRadiusUpdatedPacket(Box<<Self as ProtoVersionPackets>::ChunkRadiusUpdatedPacket>),
        ClientBoundAttributeLayerSyncPacket(
            Box<<Self as ProtoVersionPackets>::ClientBoundAttributeLayerSyncPacket>,
        ),
        ClientBoundCloseFormPacket(Box<<Self as ProtoVersionPackets>::ClientBoundCloseFormPacket>),
        ClientBoundControlSchemeSetPacket(
            Box<<Self as ProtoVersionPackets>::ClientBoundControlSchemeSetPacket>,
        ),
        ClientBoundDataDrivenUICloseScreenPacket(
            Box<<Self as ProtoVersionPackets>::ClientBoundDataDrivenUICloseScreenPacket>,
        ),
        ClientBoundDataDrivenUIReloadPacket(
            Box<<Self as ProtoVersionPackets>::ClientBoundDataDrivenUIReloadPacket>,
        ),
        ClientBoundDataDrivenUIShowScreenPacket(
            Box<<Self as ProtoVersionPackets>::ClientBoundDataDrivenUIShowScreenPacket>,
        ),
        ClientBoundDataStorePacket(Box<<Self as ProtoVersionPackets>::ClientBoundDataStorePacket>),
        ClientBoundDebugRendererPacket(
            Box<<Self as ProtoVersionPackets>::ClientBoundDebugRendererPacket>,
        ),
        ClientBoundMapItemDataPacket(
            Box<<Self as ProtoVersionPackets>::ClientBoundMapItemDataPacket>,
        ),
        ClientBoundTextureShiftPacket(
            Box<<Self as ProtoVersionPackets>::ClientBoundTextureShiftPacket>,
        ),
        ClientBoundUpdateSoundDataPacket(
            Box<<Self as ProtoVersionPackets>::ClientBoundUpdateSoundDataPacket>,
        ),
        ClientCacheBlobStatusPacket(
            Box<<Self as ProtoVersionPackets>::ClientCacheBlobStatusPacket>,
        ),
        ClientCacheMissResponsePacket(
            Box<<Self as ProtoVersionPackets>::ClientCacheMissResponsePacket>,
        ),
        ClientCacheStatusPacket(Box<<Self as ProtoVersionPackets>::ClientCacheStatusPacket>),
        ClientToServerHandshakePacket(
            Box<<Self as ProtoVersionPackets>::ClientToServerHandshakePacket>,
        ),
        CodeBuilderPacket(Box<<Self as ProtoVersionPackets>::CodeBuilderPacket>),
        CodeBuilderSourcePacket(Box<<Self as ProtoVersionPackets>::CodeBuilderSourcePacket>),
        CommandBlockUpdatePacket(Box<<Self as ProtoVersionPackets>::CommandBlockUpdatePacket>),
        CommandOutputPacket(Box<<Self as ProtoVersionPackets>::CommandOutputPacket>),
        CommandRequestPacket(Box<<Self as ProtoVersionPackets>::CommandRequestPacket>),
        CompletedUsingItemPacket(Box<<Self as ProtoVersionPackets>::CompletedUsingItemPacket>),
        ContainerClosePacket(Box<<Self as ProtoVersionPackets>::ContainerClosePacket>),
        ContainerOpenPacket(Box<<Self as ProtoVersionPackets>::ContainerOpenPacket>),
        ContainerRegistryCleanupPacket(
            Box<<Self as ProtoVersionPackets>::ContainerRegistryCleanupPacket>,
        ),
        ContainerSetDataPacket(Box<<Self as ProtoVersionPackets>::ContainerSetDataPacket>),
        CorrectPlayerMovePredictionPacket(
            Box<<Self as ProtoVersionPackets>::CorrectPlayerMovePredictionPacket>,
        ),
        CraftingDataPacket(Box<<Self as ProtoVersionPackets>::CraftingDataPacket>),
        CreatePhotoPacket(Box<<Self as ProtoVersionPackets>::CreatePhotoPacket>),
        CreativeContentPacket(Box<<Self as ProtoVersionPackets>::CreativeContentPacket>),
        CurrentStructureFeaturePacket(
            Box<<Self as ProtoVersionPackets>::CurrentStructureFeaturePacket>,
        ),
        DeathInfoPacket(Box<<Self as ProtoVersionPackets>::DeathInfoPacket>),
        DebugDrawerPacket(Box<<Self as ProtoVersionPackets>::DebugDrawerPacket>),
        DebugInfoPacket(Box<<Self as ProtoVersionPackets>::DebugInfoPacket>),
        DimensionDataPacket(Box<<Self as ProtoVersionPackets>::DimensionDataPacket>),
        DisconnectPacket(Box<<Self as ProtoVersionPackets>::DisconnectPacket>),
        EditorNetworkPacket(Box<<Self as ProtoVersionPackets>::EditorNetworkPacket>),
        EduUriResourcePacket(Box<<Self as ProtoVersionPackets>::EduUriResourcePacket>),
        EducationSettingsPacket(Box<<Self as ProtoVersionPackets>::EducationSettingsPacket>),
        EmoteListPacket(Box<<Self as ProtoVersionPackets>::EmoteListPacket>),
        EmotePacket(Box<<Self as ProtoVersionPackets>::EmotePacket>),
        FeatureRegistryPacket(Box<<Self as ProtoVersionPackets>::FeatureRegistryPacket>),
        GameRulesChangedPacket(Box<<Self as ProtoVersionPackets>::GameRulesChangedPacket>),
        GameTestRequestPacket(Box<<Self as ProtoVersionPackets>::GameTestRequestPacket>),
        GameTestResultsPacket(Box<<Self as ProtoVersionPackets>::GameTestResultsPacket>),
        GraphicsParameterOverridePacket(
            Box<<Self as ProtoVersionPackets>::GraphicsParameterOverridePacket>,
        ),
        GuiDataPickItemPacket(Box<<Self as ProtoVersionPackets>::GuiDataPickItemPacket>),
        HurtArmorPacket(Box<<Self as ProtoVersionPackets>::HurtArmorPacket>),
        InteractPacket(Box<<Self as ProtoVersionPackets>::InteractPacket>),
        InventoryContentPacket(Box<<Self as ProtoVersionPackets>::InventoryContentPacket>),
        InventorySlotPacket(Box<<Self as ProtoVersionPackets>::InventorySlotPacket>),
        InventoryTransactionPacket(Box<<Self as ProtoVersionPackets>::InventoryTransactionPacket>),
        ItemComponentPacket(Box<<Self as ProtoVersionPackets>::ItemComponentPacket>),
        ItemStackRequestPacket(Box<<Self as ProtoVersionPackets>::ItemStackRequestPacket>),
        ItemStackResponsePacket(Box<<Self as ProtoVersionPackets>::ItemStackResponsePacket>),
        JigsawStructureDataPacket(Box<<Self as ProtoVersionPackets>::JigsawStructureDataPacket>),
        LabTablePacket(Box<<Self as ProtoVersionPackets>::LabTablePacket>),
        LecternUpdatePacket(Box<<Self as ProtoVersionPackets>::LecternUpdatePacket>),
        LegacyTelemetryEventPacket(Box<<Self as ProtoVersionPackets>::LegacyTelemetryEventPacket>),
        LessonProgressPacket(Box<<Self as ProtoVersionPackets>::LessonProgressPacket>),
        LevelChunkPacket(Box<<Self as ProtoVersionPackets>::LevelChunkPacket>),
        LevelEventGenericPacket(Box<<Self as ProtoVersionPackets>::LevelEventGenericPacket>),
        LevelEventPacket(Box<<Self as ProtoVersionPackets>::LevelEventPacket>),
        LevelSoundEventPacket(Box<<Self as ProtoVersionPackets>::LevelSoundEventPacket>),
        LocatorBarPacket(Box<<Self as ProtoVersionPackets>::LocatorBarPacket>),
        LoginPacket(Box<<Self as ProtoVersionPackets>::LoginPacket>),
        MapCreateLockedCopyPacket(Box<<Self as ProtoVersionPackets>::MapCreateLockedCopyPacket>),
        MapInfoRequestPacket(Box<<Self as ProtoVersionPackets>::MapInfoRequestPacket>),
        MobArmorEquipmentPacket(Box<<Self as ProtoVersionPackets>::MobArmorEquipmentPacket>),
        MobEffectPacket(Box<<Self as ProtoVersionPackets>::MobEffectPacket>),
        MobEquipmentPacket(Box<<Self as ProtoVersionPackets>::MobEquipmentPacket>),
        ModalFormRequestPacket(Box<<Self as ProtoVersionPackets>::ModalFormRequestPacket>),
        ModalFormResponsePacket(Box<<Self as ProtoVersionPackets>::ModalFormResponsePacket>),
        MotionPredictionHintsPacket(
            Box<<Self as ProtoVersionPackets>::MotionPredictionHintsPacket>,
        ),
        MoveActorAbsolutePacket(Box<<Self as ProtoVersionPackets>::MoveActorAbsolutePacket>),
        MoveActorDeltaPacket(Box<<Self as ProtoVersionPackets>::MoveActorDeltaPacket>),
        MovePlayerPacket(Box<<Self as ProtoVersionPackets>::MovePlayerPacket>),
        MovementEffectPacket(Box<<Self as ProtoVersionPackets>::MovementEffectPacket>),
        MovementPredictionSyncPacket(
            Box<<Self as ProtoVersionPackets>::MovementPredictionSyncPacket>,
        ),
        MultiplayerSettingsPacket(Box<<Self as ProtoVersionPackets>::MultiplayerSettingsPacket>),
        NetworkChunkPublisherUpdatePacket(
            Box<<Self as ProtoVersionPackets>::NetworkChunkPublisherUpdatePacket>,
        ),
        NetworkSettingsPacket(Box<<Self as ProtoVersionPackets>::NetworkSettingsPacket>),
        NetworkStackLatencyPacket(Box<<Self as ProtoVersionPackets>::NetworkStackLatencyPacket>),
        NpcDialoguePacket(Box<<Self as ProtoVersionPackets>::NpcDialoguePacket>),
        NpcRequestPacket(Box<<Self as ProtoVersionPackets>::NpcRequestPacket>),
        OnScreenTextureAnimationPacket(
            Box<<Self as ProtoVersionPackets>::OnScreenTextureAnimationPacket>,
        ),
        OpenSignPacket(Box<<Self as ProtoVersionPackets>::OpenSignPacket>),
        PacketViolationWarningPacket(
            Box<<Self as ProtoVersionPackets>::PacketViolationWarningPacket>,
        ),
        PartyChangedPacket(Box<<Self as ProtoVersionPackets>::PartyChangedPacket>),
        PartyDestinationCookieResponsePacket(
            Box<<Self as ProtoVersionPackets>::PartyDestinationCookieResponsePacket>,
        ),
        PhotoTransferPacket(Box<<Self as ProtoVersionPackets>::PhotoTransferPacket>),
        PlaySoundPacket(Box<<Self as ProtoVersionPackets>::PlaySoundPacket>),
        PlayStatusPacket(Box<<Self as ProtoVersionPackets>::PlayStatusPacket>),
        PlayerActionPacket(Box<<Self as ProtoVersionPackets>::PlayerActionPacket>),
        PlayerArmorDamagePacket(Box<<Self as ProtoVersionPackets>::PlayerArmorDamagePacket>),
        PlayerAuthInputPacket(Box<<Self as ProtoVersionPackets>::PlayerAuthInputPacket>),
        PlayerEnchantOptionsPacket(Box<<Self as ProtoVersionPackets>::PlayerEnchantOptionsPacket>),
        PlayerFogPacket(Box<<Self as ProtoVersionPackets>::PlayerFogPacket>),
        PlayerHotbarPacket(Box<<Self as ProtoVersionPackets>::PlayerHotbarPacket>),
        PlayerListPacket(Box<<Self as ProtoVersionPackets>::PlayerListPacket>),
        PlayerLocationPacket(Box<<Self as ProtoVersionPackets>::PlayerLocationPacket>),
        PlayerSkinPacket(Box<<Self as ProtoVersionPackets>::PlayerSkinPacket>),
        PlayerStartItemCooldownPacket(
            Box<<Self as ProtoVersionPackets>::PlayerStartItemCooldownPacket>,
        ),
        PlayerToggleCrafterSlotRequestPacket(
            Box<<Self as ProtoVersionPackets>::PlayerToggleCrafterSlotRequestPacket>,
        ),
        PlayerUpdateEntityOverridesPacket(
            Box<<Self as ProtoVersionPackets>::PlayerUpdateEntityOverridesPacket>,
        ),
        PlayerVideoCapturePacket(Box<<Self as ProtoVersionPackets>::PlayerVideoCapturePacket>),
        PositionTrackingDBClientRequestPacket(
            Box<<Self as ProtoVersionPackets>::PositionTrackingDBClientRequestPacket>,
        ),
        PositionTrackingDBServerBroadcastPacket(
            Box<<Self as ProtoVersionPackets>::PositionTrackingDBServerBroadcastPacket>,
        ),
        PurchaseReceiptPacket(Box<<Self as ProtoVersionPackets>::PurchaseReceiptPacket>),
        RecordStartedPacket(Box<<Self as ProtoVersionPackets>::RecordStartedPacket>),
        RefreshEntitlementsPacket(Box<<Self as ProtoVersionPackets>::RefreshEntitlementsPacket>),
        RemoveActorPacket(Box<<Self as ProtoVersionPackets>::RemoveActorPacket>),
        RemoveObjectivePacket(Box<<Self as ProtoVersionPackets>::RemoveObjectivePacket>),
        RemoveVolumeEntityPacket(Box<<Self as ProtoVersionPackets>::RemoveVolumeEntityPacket>),
        RequestAbilityPacket(Box<<Self as ProtoVersionPackets>::RequestAbilityPacket>),
        RequestChunkRadiusPacket(Box<<Self as ProtoVersionPackets>::RequestChunkRadiusPacket>),
        RequestNetworkSettingsPacket(
            Box<<Self as ProtoVersionPackets>::RequestNetworkSettingsPacket>,
        ),
        RequestPermissionsPacket(Box<<Self as ProtoVersionPackets>::RequestPermissionsPacket>),
        ResourcePackChunkDataPacket(
            Box<<Self as ProtoVersionPackets>::ResourcePackChunkDataPacket>,
        ),
        ResourcePackChunkRequestPacket(
            Box<<Self as ProtoVersionPackets>::ResourcePackChunkRequestPacket>,
        ),
        ResourcePackClientResponsePacket(
            Box<<Self as ProtoVersionPackets>::ResourcePackClientResponsePacket>,
        ),
        ResourcePackDataInfoPacket(Box<<Self as ProtoVersionPackets>::ResourcePackDataInfoPacket>),
        ResourcePackStackPacket(Box<<Self as ProtoVersionPackets>::ResourcePackStackPacket>),
        ResourcePacksInfoPacket(Box<<Self as ProtoVersionPackets>::ResourcePacksInfoPacket>),
        ResourcePacksReadyForValidationPacket(
            Box<<Self as ProtoVersionPackets>::ResourcePacksReadyForValidationPacket>,
        ),
        RespawnPacket(Box<<Self as ProtoVersionPackets>::RespawnPacket>),
        ScriptMessagePacket(Box<<Self as ProtoVersionPackets>::ScriptMessagePacket>),
        SendPartyDestinationCookiePacket(
            Box<<Self as ProtoVersionPackets>::SendPartyDestinationCookiePacket>,
        ),
        ServerBoundDataDrivenClosedPacket(
            Box<<Self as ProtoVersionPackets>::ServerBoundDataDrivenClosedPacket>,
        ),
        ServerBoundDataStorePacket(Box<<Self as ProtoVersionPackets>::ServerBoundDataStorePacket>),
        ServerBoundDiagnosticsPacket(
            Box<<Self as ProtoVersionPackets>::ServerBoundDiagnosticsPacket>,
        ),
        ServerBoundLoadingScreenPacket(
            Box<<Self as ProtoVersionPackets>::ServerBoundLoadingScreenPacket>,
        ),
        ServerBoundPackSettingChangePacket(
            Box<<Self as ProtoVersionPackets>::ServerBoundPackSettingChangePacket>,
        ),
        ServerPlayerPostMovePositionPacket(
            Box<<Self as ProtoVersionPackets>::ServerPlayerPostMovePositionPacket>,
        ),
        ServerPresenceInfoPacket(Box<<Self as ProtoVersionPackets>::ServerPresenceInfoPacket>),
        ServerSettingsRequestPacket(
            Box<<Self as ProtoVersionPackets>::ServerSettingsRequestPacket>,
        ),
        ServerSettingsResponsePacket(
            Box<<Self as ProtoVersionPackets>::ServerSettingsResponsePacket>,
        ),
        ServerStatsPacket(Box<<Self as ProtoVersionPackets>::ServerStatsPacket>),
        ServerStoreInfoPacket(Box<<Self as ProtoVersionPackets>::ServerStoreInfoPacket>),
        ServerToClientHandshakePacket(
            Box<<Self as ProtoVersionPackets>::ServerToClientHandshakePacket>,
        ),
        SetActorDataPacket(Box<<Self as ProtoVersionPackets>::SetActorDataPacket>),
        SetActorLinkPacket(Box<<Self as ProtoVersionPackets>::SetActorLinkPacket>),
        SetActorMotionPacket(Box<<Self as ProtoVersionPackets>::SetActorMotionPacket>),
        SetCommandsEnabledPacket(Box<<Self as ProtoVersionPackets>::SetCommandsEnabledPacket>),
        SetDefaultGameTypePacket(Box<<Self as ProtoVersionPackets>::SetDefaultGameTypePacket>),
        SetDifficultyPacket(Box<<Self as ProtoVersionPackets>::SetDifficultyPacket>),
        SetDisplayObjectivePacket(Box<<Self as ProtoVersionPackets>::SetDisplayObjectivePacket>),
        SetHealthPacket(Box<<Self as ProtoVersionPackets>::SetHealthPacket>),
        SetHudPacket(Box<<Self as ProtoVersionPackets>::SetHudPacket>),
        SetLastHurtByPacket(Box<<Self as ProtoVersionPackets>::SetLastHurtByPacket>),
        SetLocalPlayerAsInitializedPacket(
            Box<<Self as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket>,
        ),
        SetPlayerFurnaceOptionsPacket(
            Box<<Self as ProtoVersionPackets>::SetPlayerFurnaceOptionsPacket>,
        ),
        SetPlayerGameTypePacket(Box<<Self as ProtoVersionPackets>::SetPlayerGameTypePacket>),
        SetPlayerInventoryOptionsPacket(
            Box<<Self as ProtoVersionPackets>::SetPlayerInventoryOptionsPacket>,
        ),
        SetScorePacket(Box<<Self as ProtoVersionPackets>::SetScorePacket>),
        SetScoreboardIdentityPacket(
            Box<<Self as ProtoVersionPackets>::SetScoreboardIdentityPacket>,
        ),
        SetSpawnPositionPacket(Box<<Self as ProtoVersionPackets>::SetSpawnPositionPacket>),
        SetTimePacket(Box<<Self as ProtoVersionPackets>::SetTimePacket>),
        SetTitlePacket(Box<<Self as ProtoVersionPackets>::SetTitlePacket>),
        SettingsCommandPacket(Box<<Self as ProtoVersionPackets>::SettingsCommandPacket>),
        ShowCreditsPacket(Box<<Self as ProtoVersionPackets>::ShowCreditsPacket>),
        ShowProfilePacket(Box<<Self as ProtoVersionPackets>::ShowProfilePacket>),
        ShowStoreOfferPacket(Box<<Self as ProtoVersionPackets>::ShowStoreOfferPacket>),
        SimpleEventPacket(Box<<Self as ProtoVersionPackets>::SimpleEventPacket>),
        SimulationTypePacket(Box<<Self as ProtoVersionPackets>::SimulationTypePacket>),
        SpawnExperienceOrbPacket(Box<<Self as ProtoVersionPackets>::SpawnExperienceOrbPacket>),
        SpawnParticleEffectPacket(Box<<Self as ProtoVersionPackets>::SpawnParticleEffectPacket>),
        StartGamePacket(Box<<Self as ProtoVersionPackets>::StartGamePacket>),
        StopSoundPacket(Box<<Self as ProtoVersionPackets>::StopSoundPacket>),
        StructureBlockUpdatePacket(Box<<Self as ProtoVersionPackets>::StructureBlockUpdatePacket>),
        StructureDataRequestPacket(Box<<Self as ProtoVersionPackets>::StructureDataRequestPacket>),
        StructureDataResponsePacket(
            Box<<Self as ProtoVersionPackets>::StructureDataResponsePacket>,
        ),
        SubChunkPacket(Box<<Self as ProtoVersionPackets>::SubChunkPacket>),
        SubChunkRequestPacket(Box<<Self as ProtoVersionPackets>::SubChunkRequestPacket>),
        SubClientLoginPacket(Box<<Self as ProtoVersionPackets>::SubClientLoginPacket>),
        SyncActorPropertyPacket(Box<<Self as ProtoVersionPackets>::SyncActorPropertyPacket>),
        SyncWorldClocksPacket(Box<<Self as ProtoVersionPackets>::SyncWorldClocksPacket>),
        TakeItemActorPacket(Box<<Self as ProtoVersionPackets>::TakeItemActorPacket>),
        TextPacket(Box<<Self as ProtoVersionPackets>::TextPacket>),
        TickingAreaLoadStatusPacket(
            Box<<Self as ProtoVersionPackets>::TickingAreaLoadStatusPacket>,
        ),
        ToastRequestPacket(Box<<Self as ProtoVersionPackets>::ToastRequestPacket>),
        TransferPlayerPacket(Box<<Self as ProtoVersionPackets>::TransferPlayerPacket>),
        TrimDataPacket(Box<<Self as ProtoVersionPackets>::TrimDataPacket>),
        UnlockedRecipesPacket(Box<<Self as ProtoVersionPackets>::UnlockedRecipesPacket>),
        UpdateAbilitiesPacket(Box<<Self as ProtoVersionPackets>::UpdateAbilitiesPacket>),
        UpdateAdventureSettingsPacket(
            Box<<Self as ProtoVersionPackets>::UpdateAdventureSettingsPacket>,
        ),
        UpdateAttributesPacket(Box<<Self as ProtoVersionPackets>::UpdateAttributesPacket>),
        UpdateBlockPacket(Box<<Self as ProtoVersionPackets>::UpdateBlockPacket>),
        UpdateBlockSyncedPacket(Box<<Self as ProtoVersionPackets>::UpdateBlockSyncedPacket>),
        UpdateClientInputLocksPacket(
            Box<<Self as ProtoVersionPackets>::UpdateClientInputLocksPacket>,
        ),
        UpdateClientOptionsPacket(Box<<Self as ProtoVersionPackets>::UpdateClientOptionsPacket>),
        UpdateEquipPacket(Box<<Self as ProtoVersionPackets>::UpdateEquipPacket>),
        UpdatePlayerGameTypePacket(Box<<Self as ProtoVersionPackets>::UpdatePlayerGameTypePacket>),
        UpdateSoftEnumPacket(Box<<Self as ProtoVersionPackets>::UpdateSoftEnumPacket>),
        UpdateSubChunkBlocksPacket(Box<<Self as ProtoVersionPackets>::UpdateSubChunkBlocksPacket>),
        UpdateTradePacket(Box<<Self as ProtoVersionPackets>::UpdateTradePacket>),
        VoxelShapesPacket(Box<<Self as ProtoVersionPackets>::VoxelShapesPacket>),
        Unknown(Box<bedrock_protocol_core::UnknownPacket>),
    }
    impl bedrock_protocol_core::Packets for V2223 {
        #[inline]
        fn serialize<W: std::io::Write>(
            &self,
            header: &bedrock_protocol_core::PacketHeader,
            stream: &mut W,
        ) -> Result<(), bedrock_protocol_core::error::PacketCodecError> {
            <bedrock_protocol_core::PacketHeader as bedrock_protocol_core::ProtoCodec>::serialize(
                header, stream,
            )
            .map_err(bedrock_protocol_core::error::PacketCodecError::InvalidHeader)?;
            match self {
                V2223::ActorEventPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ActorEventPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ActorEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ActorEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ActorPickRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ActorPickRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ActorPickRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ActorPickRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AddActorPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AddActorPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddActorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddActorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AddBehaviourTreePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AddBehaviourTreePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddBehaviourTreePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddBehaviourTreePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AddItemActorPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AddItemActorPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddItemActorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddItemActorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AddPaintingPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AddPaintingPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddPaintingPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddPaintingPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AddPlayerPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AddPlayerPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddPlayerPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddPlayerPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AddVolumeEntityPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AddVolumeEntityPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddVolumeEntityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddVolumeEntityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AgentActionEventPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AgentActionEventPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AgentActionEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AgentActionEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AgentAnimationPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AgentAnimationPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AgentAnimationPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AgentAnimationPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AnimateEntityPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AnimateEntityPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AnimateEntityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AnimateEntityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AnimatePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AnimatePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AnimatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AnimatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AnvilDamagePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AnvilDamagePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AnvilDamagePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AnvilDamagePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AutomationClientConnectPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AutomationClientConnectPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AutomationClientConnectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AutomationClientConnectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AvailableActorIdentifiersPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AvailableActorIdentifiersPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AvailableActorIdentifiersPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AvailableActorIdentifiersPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AvailableCommandsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AvailableCommandsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AvailableCommandsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AvailableCommandsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::AwardAchievementPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::AwardAchievementPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AwardAchievementPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AwardAchievementPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::BiomeDefinitionListPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::BiomeDefinitionListPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BiomeDefinitionListPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BiomeDefinitionListPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::BlockActorDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::BlockActorDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BlockActorDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BlockActorDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::BlockEventPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::BlockEventPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BlockEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BlockEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::BlockPickRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::BlockPickRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BlockPickRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BlockPickRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::BookEditPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::BookEditPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BookEditPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BookEditPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::BossEventPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::BossEventPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BossEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BossEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CameraAimAssistActorPriorityPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CameraAimAssistActorPriorityPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraAimAssistActorPriorityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraAimAssistActorPriorityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CameraAimAssistInstructionPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CameraAimAssistInstructionPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraAimAssistInstructionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraAimAssistInstructionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CameraAimAssistPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CameraAimAssistPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraAimAssistPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraAimAssistPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CameraAimAssistPresetsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CameraAimAssistPresetsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraAimAssistPresetsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraAimAssistPresetsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CameraInstructionPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CameraInstructionPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraInstructionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraInstructionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CameraPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CameraPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CameraPresetsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CameraPresetsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraPresetsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraPresetsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CameraShakePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CameraShakePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraShakePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraShakePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CameraSplinePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CameraSplinePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraSplinePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraSplinePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ChangeDimensionPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ChangeDimensionPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ChangeDimensionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ChangeDimensionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ChangeMobPropertyPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ChangeMobPropertyPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ChangeMobPropertyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ChangeMobPropertyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ChunkRadiusUpdatedPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ChunkRadiusUpdatedPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ChunkRadiusUpdatedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ChunkRadiusUpdatedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundAttributeLayerSyncPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundAttributeLayerSyncPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ClientBoundAttributeLayerSyncPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundAttributeLayerSyncPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundCloseFormPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundCloseFormPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundCloseFormPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundCloseFormPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundControlSchemeSetPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundControlSchemeSetPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundControlSchemeSetPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundControlSchemeSetPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundDataDrivenUICloseScreenPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUICloseScreenPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ClientBoundDataDrivenUICloseScreenPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUICloseScreenPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundDataDrivenUIReloadPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIReloadPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ClientBoundDataDrivenUIReloadPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIReloadPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundDataDrivenUIShowScreenPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIShowScreenPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ClientBoundDataDrivenUIShowScreenPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIShowScreenPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundDataStorePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDataStorePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundDataStorePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDataStorePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundDebugRendererPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDebugRendererPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundDebugRendererPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDebugRendererPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundMapItemDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundMapItemDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundMapItemDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundMapItemDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundTextureShiftPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundTextureShiftPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundTextureShiftPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundTextureShiftPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientBoundUpdateSoundDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundUpdateSoundDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundUpdateSoundDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundUpdateSoundDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientCacheBlobStatusPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientCacheBlobStatusPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientCacheBlobStatusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientCacheBlobStatusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientCacheMissResponsePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientCacheMissResponsePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientCacheMissResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientCacheMissResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientCacheStatusPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientCacheStatusPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientCacheStatusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientCacheStatusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ClientToServerHandshakePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ClientToServerHandshakePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientToServerHandshakePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientToServerHandshakePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CodeBuilderPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CodeBuilderPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CodeBuilderPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CodeBuilderPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CodeBuilderSourcePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CodeBuilderSourcePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CodeBuilderSourcePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CodeBuilderSourcePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CommandBlockUpdatePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CommandBlockUpdatePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CommandBlockUpdatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CommandBlockUpdatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CommandOutputPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CommandOutputPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CommandOutputPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CommandOutputPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CommandRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CommandRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CommandRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CommandRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CompletedUsingItemPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CompletedUsingItemPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CompletedUsingItemPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CompletedUsingItemPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ContainerClosePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ContainerClosePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ContainerClosePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ContainerClosePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ContainerOpenPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ContainerOpenPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ContainerOpenPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ContainerOpenPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ContainerRegistryCleanupPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ContainerRegistryCleanupPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ContainerRegistryCleanupPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ContainerRegistryCleanupPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ContainerSetDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ContainerSetDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ContainerSetDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ContainerSetDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CorrectPlayerMovePredictionPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CorrectPlayerMovePredictionPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CorrectPlayerMovePredictionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CorrectPlayerMovePredictionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CraftingDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CraftingDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CraftingDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CraftingDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CreatePhotoPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CreatePhotoPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CreatePhotoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CreatePhotoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CreativeContentPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CreativeContentPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CreativeContentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CreativeContentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::CurrentStructureFeaturePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::CurrentStructureFeaturePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CurrentStructureFeaturePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CurrentStructureFeaturePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::DeathInfoPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::DeathInfoPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DeathInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DeathInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::DebugDrawerPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::DebugDrawerPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DebugDrawerPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DebugDrawerPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::DebugInfoPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::DebugInfoPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DebugInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DebugInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::DimensionDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::DimensionDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DimensionDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DimensionDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::DisconnectPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::DisconnectPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DisconnectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DisconnectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::EditorNetworkPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::EditorNetworkPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EditorNetworkPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EditorNetworkPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::EduUriResourcePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::EduUriResourcePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EduUriResourcePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EduUriResourcePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::EducationSettingsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::EducationSettingsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EducationSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EducationSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::EmoteListPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::EmoteListPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EmoteListPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EmoteListPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::EmotePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::EmotePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EmotePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EmotePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::FeatureRegistryPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::FeatureRegistryPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(FeatureRegistryPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::FeatureRegistryPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::GameRulesChangedPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::GameRulesChangedPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GameRulesChangedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GameRulesChangedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::GameTestRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::GameTestRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GameTestRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GameTestRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::GameTestResultsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::GameTestResultsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GameTestResultsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GameTestResultsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::GraphicsParameterOverridePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::GraphicsParameterOverridePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GraphicsParameterOverridePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GraphicsParameterOverridePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::GuiDataPickItemPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::GuiDataPickItemPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GuiDataPickItemPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GuiDataPickItemPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::HurtArmorPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::HurtArmorPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(HurtArmorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::HurtArmorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::InteractPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::InteractPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(InteractPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::InteractPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::InventoryContentPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::InventoryContentPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(InventoryContentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::InventoryContentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::InventorySlotPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::InventorySlotPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(InventorySlotPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::InventorySlotPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::InventoryTransactionPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::InventoryTransactionPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(InventoryTransactionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::InventoryTransactionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ItemComponentPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ItemComponentPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ItemComponentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ItemComponentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ItemStackRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ItemStackRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ItemStackRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ItemStackRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ItemStackResponsePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ItemStackResponsePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ItemStackResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ItemStackResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::JigsawStructureDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::JigsawStructureDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(JigsawStructureDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::JigsawStructureDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LabTablePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LabTablePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LabTablePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LabTablePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LecternUpdatePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LecternUpdatePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LecternUpdatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LecternUpdatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LegacyTelemetryEventPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LegacyTelemetryEventPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LegacyTelemetryEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LegacyTelemetryEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LessonProgressPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LessonProgressPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LessonProgressPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LessonProgressPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LevelChunkPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LevelChunkPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LevelChunkPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LevelChunkPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LevelEventGenericPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LevelEventGenericPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LevelEventGenericPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LevelEventGenericPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LevelEventPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LevelEventPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LevelEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LevelEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LevelSoundEventPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LevelSoundEventPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LevelSoundEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LevelSoundEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LocatorBarPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LocatorBarPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LocatorBarPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LocatorBarPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::LoginPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::LoginPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LoginPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LoginPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MapCreateLockedCopyPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MapCreateLockedCopyPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MapCreateLockedCopyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MapCreateLockedCopyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MapInfoRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MapInfoRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MapInfoRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MapInfoRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MobArmorEquipmentPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MobArmorEquipmentPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MobArmorEquipmentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MobArmorEquipmentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MobEffectPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MobEffectPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MobEffectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MobEffectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MobEquipmentPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MobEquipmentPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MobEquipmentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MobEquipmentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ModalFormRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ModalFormRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ModalFormRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ModalFormRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ModalFormResponsePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ModalFormResponsePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ModalFormResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ModalFormResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MotionPredictionHintsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MotionPredictionHintsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MotionPredictionHintsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MotionPredictionHintsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MoveActorAbsolutePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MoveActorAbsolutePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MoveActorAbsolutePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MoveActorAbsolutePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MoveActorDeltaPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MoveActorDeltaPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MoveActorDeltaPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MoveActorDeltaPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MovePlayerPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MovePlayerPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MovePlayerPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MovePlayerPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MovementEffectPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MovementEffectPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MovementEffectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MovementEffectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MovementPredictionSyncPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MovementPredictionSyncPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MovementPredictionSyncPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MovementPredictionSyncPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::MultiplayerSettingsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::MultiplayerSettingsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MultiplayerSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MultiplayerSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::NetworkChunkPublisherUpdatePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::NetworkChunkPublisherUpdatePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NetworkChunkPublisherUpdatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NetworkChunkPublisherUpdatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::NetworkSettingsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::NetworkSettingsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NetworkSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NetworkSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::NetworkStackLatencyPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::NetworkStackLatencyPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NetworkStackLatencyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NetworkStackLatencyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::NpcDialoguePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::NpcDialoguePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NpcDialoguePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NpcDialoguePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::NpcRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::NpcRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NpcRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NpcRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::OnScreenTextureAnimationPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::OnScreenTextureAnimationPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(OnScreenTextureAnimationPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::OnScreenTextureAnimationPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::OpenSignPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::OpenSignPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(OpenSignPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::OpenSignPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PacketViolationWarningPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PacketViolationWarningPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PacketViolationWarningPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PacketViolationWarningPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PartyChangedPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PartyChangedPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PartyChangedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PartyChangedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PartyDestinationCookieResponsePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PartyDestinationCookieResponsePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    PartyDestinationCookieResponsePacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::PartyDestinationCookieResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PhotoTransferPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PhotoTransferPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PhotoTransferPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PhotoTransferPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlaySoundPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlaySoundPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlaySoundPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlaySoundPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayStatusPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayStatusPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayStatusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayStatusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerActionPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerActionPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerActionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerActionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerArmorDamagePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerArmorDamagePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerArmorDamagePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerArmorDamagePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerAuthInputPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerAuthInputPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerAuthInputPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerAuthInputPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerEnchantOptionsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerEnchantOptionsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerEnchantOptionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerEnchantOptionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerFogPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerFogPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerFogPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerFogPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerHotbarPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerHotbarPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerHotbarPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerHotbarPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerListPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerListPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerListPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerListPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerLocationPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerLocationPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerLocationPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerLocationPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerSkinPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerSkinPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerSkinPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerSkinPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerStartItemCooldownPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerStartItemCooldownPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerStartItemCooldownPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerStartItemCooldownPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerToggleCrafterSlotRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerToggleCrafterSlotRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    PlayerToggleCrafterSlotRequestPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerToggleCrafterSlotRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerUpdateEntityOverridesPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerUpdateEntityOverridesPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerUpdateEntityOverridesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerUpdateEntityOverridesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PlayerVideoCapturePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PlayerVideoCapturePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerVideoCapturePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerVideoCapturePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PositionTrackingDBClientRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PositionTrackingDBClientRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    PositionTrackingDBClientRequestPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::PositionTrackingDBClientRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PositionTrackingDBServerBroadcastPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PositionTrackingDBServerBroadcastPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    PositionTrackingDBServerBroadcastPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::PositionTrackingDBServerBroadcastPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::PurchaseReceiptPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::PurchaseReceiptPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PurchaseReceiptPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PurchaseReceiptPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RecordStartedPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RecordStartedPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RecordStartedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RecordStartedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RefreshEntitlementsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RefreshEntitlementsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RefreshEntitlementsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RefreshEntitlementsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RemoveActorPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RemoveActorPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RemoveActorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RemoveActorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RemoveObjectivePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RemoveObjectivePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RemoveObjectivePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RemoveObjectivePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RemoveVolumeEntityPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RemoveVolumeEntityPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RemoveVolumeEntityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RemoveVolumeEntityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RequestAbilityPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RequestAbilityPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RequestAbilityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RequestAbilityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RequestChunkRadiusPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RequestChunkRadiusPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RequestChunkRadiusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RequestChunkRadiusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RequestNetworkSettingsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RequestNetworkSettingsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RequestNetworkSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RequestNetworkSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RequestPermissionsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RequestPermissionsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RequestPermissionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RequestPermissionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ResourcePackChunkDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackChunkDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackChunkDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackChunkDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ResourcePackChunkRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackChunkRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackChunkRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackChunkRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ResourcePackClientResponsePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackClientResponsePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackClientResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackClientResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ResourcePackDataInfoPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackDataInfoPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackDataInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackDataInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ResourcePackStackPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackStackPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackStackPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackStackPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ResourcePacksInfoPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePacksInfoPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePacksInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePacksInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ResourcePacksReadyForValidationPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePacksReadyForValidationPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ResourcePacksReadyForValidationPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePacksReadyForValidationPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::RespawnPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::RespawnPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RespawnPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RespawnPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ScriptMessagePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ScriptMessagePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ScriptMessagePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ScriptMessagePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SendPartyDestinationCookiePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SendPartyDestinationCookiePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SendPartyDestinationCookiePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SendPartyDestinationCookiePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerBoundDataDrivenClosedPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundDataDrivenClosedPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundDataDrivenClosedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundDataDrivenClosedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerBoundDataStorePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundDataStorePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundDataStorePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundDataStorePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerBoundDiagnosticsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundDiagnosticsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundDiagnosticsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundDiagnosticsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerBoundLoadingScreenPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundLoadingScreenPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundLoadingScreenPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundLoadingScreenPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerBoundPackSettingChangePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundPackSettingChangePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundPackSettingChangePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundPackSettingChangePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerPlayerPostMovePositionPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerPlayerPostMovePositionPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerPlayerPostMovePositionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerPlayerPostMovePositionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerPresenceInfoPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerPresenceInfoPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerPresenceInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerPresenceInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerSettingsRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerSettingsRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerSettingsRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerSettingsRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerSettingsResponsePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerSettingsResponsePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerSettingsResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerSettingsResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerStatsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerStatsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerStatsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerStatsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerStoreInfoPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerStoreInfoPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerStoreInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerStoreInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ServerToClientHandshakePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ServerToClientHandshakePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerToClientHandshakePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerToClientHandshakePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetActorDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetActorDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetActorDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetActorDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetActorLinkPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetActorLinkPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetActorLinkPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetActorLinkPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetActorMotionPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetActorMotionPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetActorMotionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetActorMotionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetCommandsEnabledPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetCommandsEnabledPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetCommandsEnabledPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetCommandsEnabledPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetDefaultGameTypePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetDefaultGameTypePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetDefaultGameTypePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetDefaultGameTypePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetDifficultyPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetDifficultyPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetDifficultyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetDifficultyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetDisplayObjectivePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetDisplayObjectivePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetDisplayObjectivePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetDisplayObjectivePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetHealthPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetHealthPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetHealthPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetHealthPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetHudPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetHudPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetHudPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetHudPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetLastHurtByPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetLastHurtByPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetLastHurtByPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetLastHurtByPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetLocalPlayerAsInitializedPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetLocalPlayerAsInitializedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetPlayerFurnaceOptionsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetPlayerFurnaceOptionsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetPlayerFurnaceOptionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetPlayerFurnaceOptionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetPlayerGameTypePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetPlayerGameTypePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetPlayerGameTypePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetPlayerGameTypePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetPlayerInventoryOptionsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetPlayerInventoryOptionsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetPlayerInventoryOptionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetPlayerInventoryOptionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetScorePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetScorePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetScorePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetScorePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetScoreboardIdentityPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetScoreboardIdentityPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetScoreboardIdentityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetScoreboardIdentityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetSpawnPositionPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetSpawnPositionPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetSpawnPositionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetSpawnPositionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetTimePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetTimePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetTimePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetTimePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SetTitlePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SetTitlePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetTitlePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetTitlePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SettingsCommandPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SettingsCommandPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SettingsCommandPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SettingsCommandPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ShowCreditsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ShowCreditsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ShowCreditsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ShowCreditsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ShowProfilePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ShowProfilePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ShowProfilePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ShowProfilePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ShowStoreOfferPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ShowStoreOfferPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ShowStoreOfferPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ShowStoreOfferPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SimpleEventPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SimpleEventPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SimpleEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SimpleEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SimulationTypePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SimulationTypePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SimulationTypePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SimulationTypePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SpawnExperienceOrbPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SpawnExperienceOrbPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SpawnExperienceOrbPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SpawnExperienceOrbPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SpawnParticleEffectPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SpawnParticleEffectPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SpawnParticleEffectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SpawnParticleEffectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::StartGamePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::StartGamePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StartGamePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StartGamePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::StopSoundPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::StopSoundPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StopSoundPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StopSoundPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::StructureBlockUpdatePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::StructureBlockUpdatePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StructureBlockUpdatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StructureBlockUpdatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::StructureDataRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::StructureDataRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StructureDataRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StructureDataRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::StructureDataResponsePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::StructureDataResponsePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StructureDataResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StructureDataResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SubChunkPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SubChunkPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SubChunkPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SubChunkPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SubChunkRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SubChunkRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SubChunkRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SubChunkRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SubClientLoginPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SubClientLoginPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SubClientLoginPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SubClientLoginPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SyncActorPropertyPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SyncActorPropertyPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SyncActorPropertyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SyncActorPropertyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::SyncWorldClocksPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::SyncWorldClocksPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SyncWorldClocksPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SyncWorldClocksPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::TakeItemActorPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::TakeItemActorPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TakeItemActorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TakeItemActorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::TextPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::TextPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TextPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TextPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::TickingAreaLoadStatusPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::TickingAreaLoadStatusPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TickingAreaLoadStatusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TickingAreaLoadStatusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::ToastRequestPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::ToastRequestPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ToastRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ToastRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::TransferPlayerPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::TransferPlayerPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TransferPlayerPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TransferPlayerPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::TrimDataPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::TrimDataPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TrimDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TrimDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UnlockedRecipesPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UnlockedRecipesPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UnlockedRecipesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UnlockedRecipesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateAbilitiesPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateAbilitiesPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateAbilitiesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateAbilitiesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateAdventureSettingsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateAdventureSettingsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateAdventureSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateAdventureSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateAttributesPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateAttributesPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateAttributesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateAttributesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateBlockPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateBlockPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateBlockPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateBlockPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateBlockSyncedPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateBlockSyncedPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateBlockSyncedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateBlockSyncedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateClientInputLocksPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateClientInputLocksPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateClientInputLocksPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateClientInputLocksPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateClientOptionsPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateClientOptionsPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateClientOptionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateClientOptionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateEquipPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateEquipPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateEquipPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateEquipPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdatePlayerGameTypePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdatePlayerGameTypePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdatePlayerGameTypePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdatePlayerGameTypePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateSoftEnumPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateSoftEnumPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateSoftEnumPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateSoftEnumPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateSubChunkBlocksPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateSubChunkBlocksPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateSubChunkBlocksPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateSubChunkBlocksPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::UpdateTradePacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::UpdateTradePacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateTradePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateTradePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::VoxelShapesPacket(pk) => {
                    match <<V2223 as ProtoVersionPackets>::VoxelShapesPacket as bedrock_protocol_core::ProtoCodec>::serialize(
                        pk.as_ref(),
                        stream,
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(VoxelShapesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::VoxelShapesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    };
                }
                V2223::Unknown(pk) => stream.write_all(pk.buf.as_ref()).map_err(|e| {
                    bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                        packet_name: "Unknown",
                        packet_id: header.packet_id,
                        error: e.into(),
                    }
                })?,
            };
            Ok(())
        }
        #[inline]
        fn deserialize<R: std::io::Read>(
            stream: &mut R,
        ) -> Result<
            (Self, bedrock_protocol_core::PacketHeader),
            bedrock_protocol_core::error::PacketCodecError,
        > {
            let header = <bedrock_protocol_core::PacketHeader as bedrock_protocol_core::ProtoCodec>::deserialize(
                    stream,
                )
                .map_err(bedrock_protocol_core::error::PacketCodecError::InvalidHeader)?;
            let packet = match header.packet_id {
                <<V2223 as ProtoVersionPackets>::ActorEventPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ActorEventPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ActorEventPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ActorEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ActorEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ActorPickRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ActorPickRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ActorPickRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ActorPickRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ActorPickRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AddActorPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AddActorPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AddActorPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddActorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddActorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AddBehaviourTreePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AddBehaviourTreePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AddBehaviourTreePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddBehaviourTreePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddBehaviourTreePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AddItemActorPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AddItemActorPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AddItemActorPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddItemActorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddItemActorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AddPaintingPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AddPaintingPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AddPaintingPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddPaintingPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddPaintingPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AddPlayerPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AddPlayerPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AddPlayerPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddPlayerPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddPlayerPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AddVolumeEntityPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AddVolumeEntityPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AddVolumeEntityPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AddVolumeEntityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AddVolumeEntityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AgentActionEventPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AgentActionEventPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AgentActionEventPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AgentActionEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AgentActionEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AgentAnimationPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AgentAnimationPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AgentAnimationPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AgentAnimationPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AgentAnimationPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AnimateEntityPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AnimateEntityPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AnimateEntityPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AnimateEntityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AnimateEntityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AnimatePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AnimatePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AnimatePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AnimatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AnimatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AnvilDamagePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AnvilDamagePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AnvilDamagePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AnvilDamagePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AnvilDamagePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AutomationClientConnectPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AutomationClientConnectPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AutomationClientConnectPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AutomationClientConnectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AutomationClientConnectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AvailableActorIdentifiersPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AvailableActorIdentifiersPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AvailableActorIdentifiersPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AvailableActorIdentifiersPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AvailableActorIdentifiersPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AvailableCommandsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AvailableCommandsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AvailableCommandsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AvailableCommandsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AvailableCommandsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::AwardAchievementPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::AwardAchievementPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::AwardAchievementPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(AwardAchievementPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::AwardAchievementPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::BiomeDefinitionListPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::BiomeDefinitionListPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::BiomeDefinitionListPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BiomeDefinitionListPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BiomeDefinitionListPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::BlockActorDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::BlockActorDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::BlockActorDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BlockActorDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BlockActorDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::BlockEventPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::BlockEventPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::BlockEventPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BlockEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BlockEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::BlockPickRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::BlockPickRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::BlockPickRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BlockPickRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BlockPickRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::BookEditPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::BookEditPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::BookEditPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BookEditPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BookEditPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::BossEventPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::BossEventPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::BossEventPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(BossEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::BossEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CameraAimAssistActorPriorityPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CameraAimAssistActorPriorityPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CameraAimAssistActorPriorityPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraAimAssistActorPriorityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraAimAssistActorPriorityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CameraAimAssistInstructionPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CameraAimAssistInstructionPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CameraAimAssistInstructionPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraAimAssistInstructionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraAimAssistInstructionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CameraAimAssistPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CameraAimAssistPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CameraAimAssistPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraAimAssistPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraAimAssistPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CameraAimAssistPresetsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CameraAimAssistPresetsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CameraAimAssistPresetsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraAimAssistPresetsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraAimAssistPresetsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CameraInstructionPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CameraInstructionPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CameraInstructionPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraInstructionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraInstructionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CameraPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CameraPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CameraPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CameraPresetsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CameraPresetsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CameraPresetsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraPresetsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraPresetsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CameraShakePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CameraShakePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CameraShakePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraShakePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraShakePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CameraSplinePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CameraSplinePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CameraSplinePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CameraSplinePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CameraSplinePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ChangeDimensionPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ChangeDimensionPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ChangeDimensionPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ChangeDimensionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ChangeDimensionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ChangeMobPropertyPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ChangeMobPropertyPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ChangeMobPropertyPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ChangeMobPropertyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ChangeMobPropertyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ChunkRadiusUpdatedPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ChunkRadiusUpdatedPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ChunkRadiusUpdatedPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ChunkRadiusUpdatedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ChunkRadiusUpdatedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundAttributeLayerSyncPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundAttributeLayerSyncPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => {
                            V2223::ClientBoundAttributeLayerSyncPacket(Box::new(pk))
                        }
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ClientBoundAttributeLayerSyncPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundAttributeLayerSyncPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundCloseFormPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundCloseFormPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientBoundCloseFormPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundCloseFormPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundCloseFormPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundControlSchemeSetPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundControlSchemeSetPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientBoundControlSchemeSetPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundControlSchemeSetPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundControlSchemeSetPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUICloseScreenPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUICloseScreenPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => {
                            V2223::ClientBoundDataDrivenUICloseScreenPacket(Box::new(pk))
                        }
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ClientBoundDataDrivenUICloseScreenPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUICloseScreenPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIReloadPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIReloadPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => {
                            V2223::ClientBoundDataDrivenUIReloadPacket(Box::new(pk))
                        }
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ClientBoundDataDrivenUIReloadPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIReloadPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIShowScreenPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIShowScreenPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => {
                            V2223::ClientBoundDataDrivenUIShowScreenPacket(Box::new(pk))
                        }
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ClientBoundDataDrivenUIShowScreenPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIShowScreenPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundDataStorePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDataStorePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientBoundDataStorePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundDataStorePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDataStorePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundDebugRendererPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundDebugRendererPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientBoundDebugRendererPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundDebugRendererPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundDebugRendererPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundMapItemDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundMapItemDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientBoundMapItemDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundMapItemDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundMapItemDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundTextureShiftPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundTextureShiftPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientBoundTextureShiftPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundTextureShiftPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundTextureShiftPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientBoundUpdateSoundDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientBoundUpdateSoundDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientBoundUpdateSoundDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientBoundUpdateSoundDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientBoundUpdateSoundDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientCacheBlobStatusPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientCacheBlobStatusPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientCacheBlobStatusPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientCacheBlobStatusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientCacheBlobStatusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientCacheMissResponsePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientCacheMissResponsePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientCacheMissResponsePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientCacheMissResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientCacheMissResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientCacheStatusPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientCacheStatusPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientCacheStatusPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientCacheStatusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientCacheStatusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ClientToServerHandshakePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ClientToServerHandshakePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ClientToServerHandshakePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ClientToServerHandshakePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ClientToServerHandshakePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CodeBuilderPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CodeBuilderPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CodeBuilderPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CodeBuilderPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CodeBuilderPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CodeBuilderSourcePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CodeBuilderSourcePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CodeBuilderSourcePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CodeBuilderSourcePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CodeBuilderSourcePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CommandBlockUpdatePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CommandBlockUpdatePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CommandBlockUpdatePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CommandBlockUpdatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CommandBlockUpdatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CommandOutputPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CommandOutputPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CommandOutputPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CommandOutputPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CommandOutputPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CommandRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CommandRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CommandRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CommandRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CommandRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CompletedUsingItemPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CompletedUsingItemPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CompletedUsingItemPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CompletedUsingItemPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CompletedUsingItemPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ContainerClosePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ContainerClosePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ContainerClosePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ContainerClosePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ContainerClosePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ContainerOpenPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ContainerOpenPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ContainerOpenPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ContainerOpenPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ContainerOpenPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ContainerRegistryCleanupPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ContainerRegistryCleanupPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ContainerRegistryCleanupPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ContainerRegistryCleanupPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ContainerRegistryCleanupPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ContainerSetDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ContainerSetDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ContainerSetDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ContainerSetDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ContainerSetDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CorrectPlayerMovePredictionPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CorrectPlayerMovePredictionPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CorrectPlayerMovePredictionPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CorrectPlayerMovePredictionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CorrectPlayerMovePredictionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CraftingDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CraftingDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CraftingDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CraftingDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CraftingDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CreatePhotoPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CreatePhotoPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CreatePhotoPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CreatePhotoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CreatePhotoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CreativeContentPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CreativeContentPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CreativeContentPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CreativeContentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CreativeContentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::CurrentStructureFeaturePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::CurrentStructureFeaturePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::CurrentStructureFeaturePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(CurrentStructureFeaturePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::CurrentStructureFeaturePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::DeathInfoPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::DeathInfoPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::DeathInfoPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DeathInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DeathInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::DebugDrawerPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::DebugDrawerPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::DebugDrawerPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DebugDrawerPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DebugDrawerPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::DebugInfoPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::DebugInfoPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::DebugInfoPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DebugInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DebugInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::DimensionDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::DimensionDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::DimensionDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DimensionDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DimensionDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::DisconnectPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::DisconnectPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::DisconnectPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(DisconnectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::DisconnectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::EditorNetworkPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::EditorNetworkPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::EditorNetworkPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EditorNetworkPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EditorNetworkPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::EduUriResourcePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::EduUriResourcePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::EduUriResourcePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EduUriResourcePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EduUriResourcePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::EducationSettingsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::EducationSettingsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::EducationSettingsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EducationSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EducationSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::EmoteListPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::EmoteListPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::EmoteListPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EmoteListPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EmoteListPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::EmotePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::EmotePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::EmotePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(EmotePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::EmotePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::FeatureRegistryPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::FeatureRegistryPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::FeatureRegistryPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(FeatureRegistryPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::FeatureRegistryPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::GameRulesChangedPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::GameRulesChangedPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::GameRulesChangedPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GameRulesChangedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GameRulesChangedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::GameTestRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::GameTestRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::GameTestRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GameTestRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GameTestRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::GameTestResultsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::GameTestResultsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::GameTestResultsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GameTestResultsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GameTestResultsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::GraphicsParameterOverridePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::GraphicsParameterOverridePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::GraphicsParameterOverridePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GraphicsParameterOverridePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GraphicsParameterOverridePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::GuiDataPickItemPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::GuiDataPickItemPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::GuiDataPickItemPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(GuiDataPickItemPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::GuiDataPickItemPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::HurtArmorPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::HurtArmorPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::HurtArmorPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(HurtArmorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::HurtArmorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::InteractPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::InteractPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::InteractPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(InteractPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::InteractPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::InventoryContentPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::InventoryContentPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::InventoryContentPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(InventoryContentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::InventoryContentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::InventorySlotPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::InventorySlotPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::InventorySlotPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(InventorySlotPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::InventorySlotPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::InventoryTransactionPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::InventoryTransactionPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::InventoryTransactionPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(InventoryTransactionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::InventoryTransactionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ItemComponentPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ItemComponentPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ItemComponentPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ItemComponentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ItemComponentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ItemStackRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ItemStackRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ItemStackRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ItemStackRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ItemStackRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ItemStackResponsePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ItemStackResponsePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ItemStackResponsePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ItemStackResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ItemStackResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::JigsawStructureDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::JigsawStructureDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::JigsawStructureDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(JigsawStructureDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::JigsawStructureDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LabTablePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LabTablePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LabTablePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LabTablePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LabTablePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LecternUpdatePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LecternUpdatePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LecternUpdatePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LecternUpdatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LecternUpdatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LegacyTelemetryEventPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LegacyTelemetryEventPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LegacyTelemetryEventPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LegacyTelemetryEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LegacyTelemetryEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LessonProgressPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LessonProgressPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LessonProgressPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LessonProgressPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LessonProgressPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LevelChunkPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LevelChunkPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LevelChunkPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LevelChunkPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LevelChunkPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LevelEventGenericPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LevelEventGenericPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LevelEventGenericPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LevelEventGenericPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LevelEventGenericPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LevelEventPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LevelEventPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LevelEventPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LevelEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LevelEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LevelSoundEventPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LevelSoundEventPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LevelSoundEventPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LevelSoundEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LevelSoundEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LocatorBarPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LocatorBarPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LocatorBarPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LocatorBarPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LocatorBarPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::LoginPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::LoginPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::LoginPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(LoginPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::LoginPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MapCreateLockedCopyPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MapCreateLockedCopyPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MapCreateLockedCopyPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MapCreateLockedCopyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MapCreateLockedCopyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MapInfoRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MapInfoRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MapInfoRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MapInfoRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MapInfoRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MobArmorEquipmentPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MobArmorEquipmentPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MobArmorEquipmentPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MobArmorEquipmentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MobArmorEquipmentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MobEffectPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MobEffectPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MobEffectPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MobEffectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MobEffectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MobEquipmentPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MobEquipmentPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MobEquipmentPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MobEquipmentPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MobEquipmentPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ModalFormRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ModalFormRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ModalFormRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ModalFormRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ModalFormRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ModalFormResponsePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ModalFormResponsePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ModalFormResponsePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ModalFormResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ModalFormResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MotionPredictionHintsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MotionPredictionHintsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MotionPredictionHintsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MotionPredictionHintsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MotionPredictionHintsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MoveActorAbsolutePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MoveActorAbsolutePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MoveActorAbsolutePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MoveActorAbsolutePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MoveActorAbsolutePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MoveActorDeltaPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MoveActorDeltaPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MoveActorDeltaPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MoveActorDeltaPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MoveActorDeltaPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MovePlayerPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MovePlayerPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MovePlayerPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MovePlayerPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MovePlayerPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MovementEffectPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MovementEffectPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MovementEffectPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MovementEffectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MovementEffectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MovementPredictionSyncPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MovementPredictionSyncPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MovementPredictionSyncPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MovementPredictionSyncPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MovementPredictionSyncPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::MultiplayerSettingsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::MultiplayerSettingsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::MultiplayerSettingsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(MultiplayerSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::MultiplayerSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::NetworkChunkPublisherUpdatePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::NetworkChunkPublisherUpdatePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::NetworkChunkPublisherUpdatePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NetworkChunkPublisherUpdatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NetworkChunkPublisherUpdatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::NetworkSettingsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::NetworkSettingsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::NetworkSettingsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NetworkSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NetworkSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::NetworkStackLatencyPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::NetworkStackLatencyPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::NetworkStackLatencyPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NetworkStackLatencyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NetworkStackLatencyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::NpcDialoguePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::NpcDialoguePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::NpcDialoguePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NpcDialoguePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NpcDialoguePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::NpcRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::NpcRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::NpcRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(NpcRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::NpcRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::OnScreenTextureAnimationPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::OnScreenTextureAnimationPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::OnScreenTextureAnimationPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(OnScreenTextureAnimationPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::OnScreenTextureAnimationPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::OpenSignPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::OpenSignPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::OpenSignPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(OpenSignPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::OpenSignPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PacketViolationWarningPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PacketViolationWarningPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PacketViolationWarningPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PacketViolationWarningPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PacketViolationWarningPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PartyChangedPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PartyChangedPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PartyChangedPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PartyChangedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PartyChangedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PartyDestinationCookieResponsePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PartyDestinationCookieResponsePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => {
                            V2223::PartyDestinationCookieResponsePacket(Box::new(pk))
                        }
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    PartyDestinationCookieResponsePacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::PartyDestinationCookieResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PhotoTransferPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PhotoTransferPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PhotoTransferPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PhotoTransferPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PhotoTransferPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlaySoundPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlaySoundPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlaySoundPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlaySoundPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlaySoundPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayStatusPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayStatusPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayStatusPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayStatusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayStatusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerActionPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerActionPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerActionPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerActionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerActionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerArmorDamagePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerArmorDamagePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerArmorDamagePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerArmorDamagePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerArmorDamagePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerAuthInputPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerAuthInputPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerAuthInputPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerAuthInputPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerAuthInputPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerEnchantOptionsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerEnchantOptionsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerEnchantOptionsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerEnchantOptionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerEnchantOptionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerFogPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerFogPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerFogPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerFogPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerFogPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerHotbarPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerHotbarPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerHotbarPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerHotbarPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerHotbarPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerListPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerListPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerListPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerListPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerListPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerLocationPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerLocationPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerLocationPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerLocationPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerLocationPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerSkinPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerSkinPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerSkinPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerSkinPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerSkinPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerStartItemCooldownPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerStartItemCooldownPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerStartItemCooldownPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerStartItemCooldownPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerStartItemCooldownPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerToggleCrafterSlotRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerToggleCrafterSlotRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => {
                            V2223::PlayerToggleCrafterSlotRequestPacket(Box::new(pk))
                        }
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    PlayerToggleCrafterSlotRequestPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerToggleCrafterSlotRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerUpdateEntityOverridesPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerUpdateEntityOverridesPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerUpdateEntityOverridesPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerUpdateEntityOverridesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerUpdateEntityOverridesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PlayerVideoCapturePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PlayerVideoCapturePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PlayerVideoCapturePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PlayerVideoCapturePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PlayerVideoCapturePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PositionTrackingDBClientRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PositionTrackingDBClientRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => {
                            V2223::PositionTrackingDBClientRequestPacket(Box::new(pk))
                        }
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    PositionTrackingDBClientRequestPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::PositionTrackingDBClientRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PositionTrackingDBServerBroadcastPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PositionTrackingDBServerBroadcastPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => {
                            V2223::PositionTrackingDBServerBroadcastPacket(Box::new(pk))
                        }
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    PositionTrackingDBServerBroadcastPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::PositionTrackingDBServerBroadcastPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::PurchaseReceiptPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::PurchaseReceiptPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::PurchaseReceiptPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(PurchaseReceiptPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::PurchaseReceiptPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RecordStartedPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RecordStartedPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RecordStartedPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RecordStartedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RecordStartedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RefreshEntitlementsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RefreshEntitlementsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RefreshEntitlementsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RefreshEntitlementsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RefreshEntitlementsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RemoveActorPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RemoveActorPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RemoveActorPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RemoveActorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RemoveActorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RemoveObjectivePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RemoveObjectivePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RemoveObjectivePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RemoveObjectivePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RemoveObjectivePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RemoveVolumeEntityPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RemoveVolumeEntityPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RemoveVolumeEntityPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RemoveVolumeEntityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RemoveVolumeEntityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RequestAbilityPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RequestAbilityPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RequestAbilityPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RequestAbilityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RequestAbilityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RequestChunkRadiusPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RequestChunkRadiusPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RequestChunkRadiusPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RequestChunkRadiusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RequestChunkRadiusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RequestNetworkSettingsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RequestNetworkSettingsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RequestNetworkSettingsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RequestNetworkSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RequestNetworkSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RequestPermissionsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RequestPermissionsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RequestPermissionsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RequestPermissionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RequestPermissionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ResourcePackChunkDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackChunkDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ResourcePackChunkDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackChunkDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackChunkDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ResourcePackChunkRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackChunkRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ResourcePackChunkRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackChunkRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackChunkRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ResourcePackClientResponsePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackClientResponsePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ResourcePackClientResponsePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackClientResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackClientResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ResourcePackDataInfoPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackDataInfoPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ResourcePackDataInfoPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackDataInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackDataInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ResourcePackStackPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePackStackPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ResourcePackStackPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePackStackPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePackStackPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ResourcePacksInfoPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePacksInfoPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ResourcePacksInfoPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ResourcePacksInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePacksInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ResourcePacksReadyForValidationPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ResourcePacksReadyForValidationPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => {
                            V2223::ResourcePacksReadyForValidationPacket(Box::new(pk))
                        }
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(
                                    ResourcePacksReadyForValidationPacket
                                ),
                                packet_id: <<V2223 as ProtoVersionPackets>::ResourcePacksReadyForValidationPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::RespawnPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::RespawnPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::RespawnPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(RespawnPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::RespawnPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ScriptMessagePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ScriptMessagePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ScriptMessagePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ScriptMessagePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ScriptMessagePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SendPartyDestinationCookiePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SendPartyDestinationCookiePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SendPartyDestinationCookiePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SendPartyDestinationCookiePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SendPartyDestinationCookiePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerBoundDataDrivenClosedPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundDataDrivenClosedPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerBoundDataDrivenClosedPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundDataDrivenClosedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundDataDrivenClosedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerBoundDataStorePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundDataStorePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerBoundDataStorePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundDataStorePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundDataStorePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerBoundDiagnosticsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundDiagnosticsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerBoundDiagnosticsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundDiagnosticsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundDiagnosticsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerBoundLoadingScreenPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundLoadingScreenPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerBoundLoadingScreenPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundLoadingScreenPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundLoadingScreenPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerBoundPackSettingChangePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerBoundPackSettingChangePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerBoundPackSettingChangePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerBoundPackSettingChangePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerBoundPackSettingChangePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerPlayerPostMovePositionPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerPlayerPostMovePositionPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerPlayerPostMovePositionPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerPlayerPostMovePositionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerPlayerPostMovePositionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerPresenceInfoPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerPresenceInfoPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerPresenceInfoPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerPresenceInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerPresenceInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerSettingsRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerSettingsRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerSettingsRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerSettingsRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerSettingsRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerSettingsResponsePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerSettingsResponsePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerSettingsResponsePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerSettingsResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerSettingsResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerStatsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerStatsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerStatsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerStatsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerStatsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerStoreInfoPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerStoreInfoPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerStoreInfoPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerStoreInfoPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerStoreInfoPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ServerToClientHandshakePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ServerToClientHandshakePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ServerToClientHandshakePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ServerToClientHandshakePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ServerToClientHandshakePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetActorDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetActorDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetActorDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetActorDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetActorDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetActorLinkPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetActorLinkPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetActorLinkPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetActorLinkPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetActorLinkPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetActorMotionPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetActorMotionPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetActorMotionPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetActorMotionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetActorMotionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetCommandsEnabledPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetCommandsEnabledPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetCommandsEnabledPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetCommandsEnabledPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetCommandsEnabledPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetDefaultGameTypePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetDefaultGameTypePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetDefaultGameTypePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetDefaultGameTypePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetDefaultGameTypePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetDifficultyPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetDifficultyPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetDifficultyPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetDifficultyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetDifficultyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetDisplayObjectivePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetDisplayObjectivePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetDisplayObjectivePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetDisplayObjectivePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetDisplayObjectivePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetHealthPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetHealthPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetHealthPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetHealthPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetHealthPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetHudPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetHudPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetHudPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetHudPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetHudPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetLastHurtByPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetLastHurtByPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetLastHurtByPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetLastHurtByPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetLastHurtByPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetLocalPlayerAsInitializedPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetLocalPlayerAsInitializedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetPlayerFurnaceOptionsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetPlayerFurnaceOptionsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetPlayerFurnaceOptionsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetPlayerFurnaceOptionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetPlayerFurnaceOptionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetPlayerGameTypePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetPlayerGameTypePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetPlayerGameTypePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetPlayerGameTypePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetPlayerGameTypePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetPlayerInventoryOptionsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetPlayerInventoryOptionsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetPlayerInventoryOptionsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetPlayerInventoryOptionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetPlayerInventoryOptionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetScorePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetScorePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetScorePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetScorePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetScorePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetScoreboardIdentityPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetScoreboardIdentityPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetScoreboardIdentityPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetScoreboardIdentityPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetScoreboardIdentityPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetSpawnPositionPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetSpawnPositionPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetSpawnPositionPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetSpawnPositionPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetSpawnPositionPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetTimePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetTimePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetTimePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetTimePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetTimePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SetTitlePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SetTitlePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SetTitlePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SetTitlePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SetTitlePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SettingsCommandPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SettingsCommandPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SettingsCommandPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SettingsCommandPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SettingsCommandPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ShowCreditsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ShowCreditsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ShowCreditsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ShowCreditsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ShowCreditsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ShowProfilePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ShowProfilePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ShowProfilePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ShowProfilePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ShowProfilePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ShowStoreOfferPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ShowStoreOfferPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ShowStoreOfferPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ShowStoreOfferPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ShowStoreOfferPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SimpleEventPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SimpleEventPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SimpleEventPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SimpleEventPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SimpleEventPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SimulationTypePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SimulationTypePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SimulationTypePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SimulationTypePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SimulationTypePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SpawnExperienceOrbPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SpawnExperienceOrbPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SpawnExperienceOrbPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SpawnExperienceOrbPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SpawnExperienceOrbPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SpawnParticleEffectPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SpawnParticleEffectPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SpawnParticleEffectPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SpawnParticleEffectPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SpawnParticleEffectPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::StartGamePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::StartGamePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::StartGamePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StartGamePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StartGamePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::StopSoundPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::StopSoundPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::StopSoundPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StopSoundPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StopSoundPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::StructureBlockUpdatePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::StructureBlockUpdatePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::StructureBlockUpdatePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StructureBlockUpdatePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StructureBlockUpdatePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::StructureDataRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::StructureDataRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::StructureDataRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StructureDataRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StructureDataRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::StructureDataResponsePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::StructureDataResponsePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::StructureDataResponsePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(StructureDataResponsePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::StructureDataResponsePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SubChunkPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SubChunkPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SubChunkPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SubChunkPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SubChunkPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SubChunkRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SubChunkRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SubChunkRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SubChunkRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SubChunkRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SubClientLoginPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SubClientLoginPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SubClientLoginPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SubClientLoginPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SubClientLoginPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SyncActorPropertyPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SyncActorPropertyPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SyncActorPropertyPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SyncActorPropertyPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SyncActorPropertyPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::SyncWorldClocksPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::SyncWorldClocksPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::SyncWorldClocksPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(SyncWorldClocksPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::SyncWorldClocksPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::TakeItemActorPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::TakeItemActorPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::TakeItemActorPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TakeItemActorPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TakeItemActorPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::TextPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::TextPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::TextPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TextPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TextPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::TickingAreaLoadStatusPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::TickingAreaLoadStatusPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::TickingAreaLoadStatusPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TickingAreaLoadStatusPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TickingAreaLoadStatusPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::ToastRequestPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::ToastRequestPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::ToastRequestPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(ToastRequestPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::ToastRequestPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::TransferPlayerPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::TransferPlayerPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::TransferPlayerPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TransferPlayerPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TransferPlayerPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::TrimDataPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::TrimDataPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::TrimDataPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(TrimDataPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::TrimDataPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UnlockedRecipesPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UnlockedRecipesPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UnlockedRecipesPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UnlockedRecipesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UnlockedRecipesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateAbilitiesPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateAbilitiesPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateAbilitiesPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateAbilitiesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateAbilitiesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateAdventureSettingsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateAdventureSettingsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateAdventureSettingsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateAdventureSettingsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateAdventureSettingsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateAttributesPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateAttributesPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateAttributesPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateAttributesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateAttributesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateBlockPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateBlockPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateBlockPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateBlockPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateBlockPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateBlockSyncedPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateBlockSyncedPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateBlockSyncedPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateBlockSyncedPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateBlockSyncedPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateClientInputLocksPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateClientInputLocksPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateClientInputLocksPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateClientInputLocksPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateClientInputLocksPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateClientOptionsPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateClientOptionsPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateClientOptionsPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateClientOptionsPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateClientOptionsPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateEquipPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateEquipPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateEquipPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateEquipPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateEquipPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdatePlayerGameTypePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdatePlayerGameTypePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdatePlayerGameTypePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdatePlayerGameTypePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdatePlayerGameTypePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateSoftEnumPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateSoftEnumPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateSoftEnumPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateSoftEnumPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateSoftEnumPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateSubChunkBlocksPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateSubChunkBlocksPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateSubChunkBlocksPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateSubChunkBlocksPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateSubChunkBlocksPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::UpdateTradePacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::UpdateTradePacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::UpdateTradePacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(UpdateTradePacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::UpdateTradePacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                <<V2223 as ProtoVersionPackets>::VoxelShapesPacket as bedrock_protocol_core::Packet>::ID => {
                    match <<V2223 as ProtoVersionPackets>::VoxelShapesPacket as bedrock_protocol_core::ProtoCodec>::deserialize(
                        stream,
                    ) {
                        Ok(pk) => V2223::VoxelShapesPacket(Box::new(pk)),
                        Err(err) => {
                            return Err(bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                                packet_name: stringify!(VoxelShapesPacket),
                                packet_id: <<V2223 as ProtoVersionPackets>::VoxelShapesPacket as bedrock_protocol_core::Packet>::ID,
                                error: err,
                            });
                        }
                    }
                }
                unknown => {
                    let mut buf = Vec::new();
                    stream
                        .read_to_end(&mut buf)
                        .map_err(|e| bedrock_protocol_core::error::PacketCodecError::InvalidPacket {
                            packet_name: "Unknown",
                            packet_id: header.packet_id,
                            error: e.into(),
                        })?;
                    V2223::Unknown(
                        Box::new(bedrock_protocol_core::UnknownPacket {
                            id: unknown,
                            buf: buf.into_boxed_slice(),
                        }),
                    )
                }
            };
            Ok((packet, header))
        }
        #[inline]
        fn size_hint(&self, header: &bedrock_protocol_core::PacketHeader) -> usize {
            <bedrock_protocol_core::PacketHeader as bedrock_protocol_core::ProtoCodec>::size_hint(
                header,
            )
                + match self {
                    V2223::ActorEventPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ActorEventPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ActorPickRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ActorPickRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AddActorPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AddActorPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AddBehaviourTreePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AddBehaviourTreePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AddItemActorPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AddItemActorPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AddPaintingPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AddPaintingPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AddPlayerPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AddPlayerPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AddVolumeEntityPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AddVolumeEntityPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AgentActionEventPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AgentActionEventPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AgentAnimationPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AgentAnimationPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AnimateEntityPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AnimateEntityPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AnimatePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AnimatePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AnvilDamagePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AnvilDamagePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AutomationClientConnectPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AutomationClientConnectPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AvailableActorIdentifiersPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AvailableActorIdentifiersPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AvailableCommandsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AvailableCommandsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::AwardAchievementPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::AwardAchievementPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::BiomeDefinitionListPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::BiomeDefinitionListPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::BlockActorDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::BlockActorDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::BlockEventPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::BlockEventPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::BlockPickRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::BlockPickRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::BookEditPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::BookEditPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::BossEventPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::BossEventPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CameraAimAssistActorPriorityPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CameraAimAssistActorPriorityPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CameraAimAssistInstructionPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CameraAimAssistInstructionPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CameraAimAssistPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CameraAimAssistPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CameraAimAssistPresetsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CameraAimAssistPresetsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CameraInstructionPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CameraInstructionPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CameraPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CameraPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CameraPresetsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CameraPresetsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CameraShakePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CameraShakePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CameraSplinePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CameraSplinePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ChangeDimensionPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ChangeDimensionPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ChangeMobPropertyPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ChangeMobPropertyPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ChunkRadiusUpdatedPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ChunkRadiusUpdatedPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundAttributeLayerSyncPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundAttributeLayerSyncPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundCloseFormPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundCloseFormPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundControlSchemeSetPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundControlSchemeSetPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundDataDrivenUICloseScreenPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUICloseScreenPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundDataDrivenUIReloadPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIReloadPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundDataDrivenUIShowScreenPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIShowScreenPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundDataStorePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundDataStorePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundDebugRendererPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundDebugRendererPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundMapItemDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundMapItemDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundTextureShiftPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundTextureShiftPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientBoundUpdateSoundDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientBoundUpdateSoundDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientCacheBlobStatusPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientCacheBlobStatusPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientCacheMissResponsePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientCacheMissResponsePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientCacheStatusPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientCacheStatusPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ClientToServerHandshakePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ClientToServerHandshakePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CodeBuilderPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CodeBuilderPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CodeBuilderSourcePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CodeBuilderSourcePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CommandBlockUpdatePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CommandBlockUpdatePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CommandOutputPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CommandOutputPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CommandRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CommandRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CompletedUsingItemPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CompletedUsingItemPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ContainerClosePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ContainerClosePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ContainerOpenPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ContainerOpenPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ContainerRegistryCleanupPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ContainerRegistryCleanupPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ContainerSetDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ContainerSetDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CorrectPlayerMovePredictionPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CorrectPlayerMovePredictionPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CraftingDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CraftingDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CreatePhotoPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CreatePhotoPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CreativeContentPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CreativeContentPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::CurrentStructureFeaturePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::CurrentStructureFeaturePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::DeathInfoPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::DeathInfoPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::DebugDrawerPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::DebugDrawerPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::DebugInfoPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::DebugInfoPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::DimensionDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::DimensionDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::DisconnectPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::DisconnectPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::EditorNetworkPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::EditorNetworkPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::EduUriResourcePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::EduUriResourcePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::EducationSettingsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::EducationSettingsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::EmoteListPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::EmoteListPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::EmotePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::EmotePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::FeatureRegistryPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::FeatureRegistryPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::GameRulesChangedPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::GameRulesChangedPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::GameTestRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::GameTestRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::GameTestResultsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::GameTestResultsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::GraphicsParameterOverridePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::GraphicsParameterOverridePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::GuiDataPickItemPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::GuiDataPickItemPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::HurtArmorPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::HurtArmorPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::InteractPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::InteractPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::InventoryContentPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::InventoryContentPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::InventorySlotPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::InventorySlotPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::InventoryTransactionPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::InventoryTransactionPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ItemComponentPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ItemComponentPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ItemStackRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ItemStackRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ItemStackResponsePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ItemStackResponsePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::JigsawStructureDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::JigsawStructureDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LabTablePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LabTablePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LecternUpdatePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LecternUpdatePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LegacyTelemetryEventPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LegacyTelemetryEventPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LessonProgressPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LessonProgressPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LevelChunkPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LevelChunkPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LevelEventGenericPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LevelEventGenericPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LevelEventPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LevelEventPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LevelSoundEventPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LevelSoundEventPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LocatorBarPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LocatorBarPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::LoginPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::LoginPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MapCreateLockedCopyPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MapCreateLockedCopyPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MapInfoRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MapInfoRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MobArmorEquipmentPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MobArmorEquipmentPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MobEffectPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MobEffectPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MobEquipmentPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MobEquipmentPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ModalFormRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ModalFormRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ModalFormResponsePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ModalFormResponsePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MotionPredictionHintsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MotionPredictionHintsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MoveActorAbsolutePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MoveActorAbsolutePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MoveActorDeltaPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MoveActorDeltaPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MovePlayerPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MovePlayerPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MovementEffectPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MovementEffectPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MovementPredictionSyncPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MovementPredictionSyncPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::MultiplayerSettingsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::MultiplayerSettingsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::NetworkChunkPublisherUpdatePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::NetworkChunkPublisherUpdatePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::NetworkSettingsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::NetworkSettingsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::NetworkStackLatencyPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::NetworkStackLatencyPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::NpcDialoguePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::NpcDialoguePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::NpcRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::NpcRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::OnScreenTextureAnimationPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::OnScreenTextureAnimationPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::OpenSignPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::OpenSignPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PacketViolationWarningPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PacketViolationWarningPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PartyChangedPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PartyChangedPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PartyDestinationCookieResponsePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PartyDestinationCookieResponsePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PhotoTransferPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PhotoTransferPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlaySoundPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlaySoundPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayStatusPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayStatusPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerActionPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerActionPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerArmorDamagePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerArmorDamagePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerAuthInputPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerAuthInputPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerEnchantOptionsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerEnchantOptionsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerFogPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerFogPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerHotbarPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerHotbarPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerListPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerListPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerLocationPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerLocationPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerSkinPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerSkinPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerStartItemCooldownPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerStartItemCooldownPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerToggleCrafterSlotRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerToggleCrafterSlotRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerUpdateEntityOverridesPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerUpdateEntityOverridesPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PlayerVideoCapturePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PlayerVideoCapturePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PositionTrackingDBClientRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PositionTrackingDBClientRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PositionTrackingDBServerBroadcastPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PositionTrackingDBServerBroadcastPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::PurchaseReceiptPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::PurchaseReceiptPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RecordStartedPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RecordStartedPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RefreshEntitlementsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RefreshEntitlementsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RemoveActorPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RemoveActorPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RemoveObjectivePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RemoveObjectivePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RemoveVolumeEntityPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RemoveVolumeEntityPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RequestAbilityPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RequestAbilityPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RequestChunkRadiusPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RequestChunkRadiusPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RequestNetworkSettingsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RequestNetworkSettingsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RequestPermissionsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RequestPermissionsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ResourcePackChunkDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ResourcePackChunkDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ResourcePackChunkRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ResourcePackChunkRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ResourcePackClientResponsePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ResourcePackClientResponsePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ResourcePackDataInfoPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ResourcePackDataInfoPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ResourcePackStackPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ResourcePackStackPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ResourcePacksInfoPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ResourcePacksInfoPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ResourcePacksReadyForValidationPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ResourcePacksReadyForValidationPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::RespawnPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::RespawnPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ScriptMessagePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ScriptMessagePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SendPartyDestinationCookiePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SendPartyDestinationCookiePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerBoundDataDrivenClosedPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerBoundDataDrivenClosedPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerBoundDataStorePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerBoundDataStorePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerBoundDiagnosticsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerBoundDiagnosticsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerBoundLoadingScreenPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerBoundLoadingScreenPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerBoundPackSettingChangePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerBoundPackSettingChangePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerPlayerPostMovePositionPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerPlayerPostMovePositionPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerPresenceInfoPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerPresenceInfoPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerSettingsRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerSettingsRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerSettingsResponsePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerSettingsResponsePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerStatsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerStatsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerStoreInfoPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerStoreInfoPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ServerToClientHandshakePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ServerToClientHandshakePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetActorDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetActorDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetActorLinkPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetActorLinkPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetActorMotionPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetActorMotionPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetCommandsEnabledPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetCommandsEnabledPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetDefaultGameTypePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetDefaultGameTypePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetDifficultyPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetDifficultyPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetDisplayObjectivePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetDisplayObjectivePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetHealthPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetHealthPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetHudPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetHudPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetLastHurtByPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetLastHurtByPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetLocalPlayerAsInitializedPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetPlayerFurnaceOptionsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetPlayerFurnaceOptionsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetPlayerGameTypePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetPlayerGameTypePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetPlayerInventoryOptionsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetPlayerInventoryOptionsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetScorePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetScorePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetScoreboardIdentityPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetScoreboardIdentityPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetSpawnPositionPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetSpawnPositionPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetTimePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetTimePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SetTitlePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SetTitlePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SettingsCommandPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SettingsCommandPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ShowCreditsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ShowCreditsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ShowProfilePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ShowProfilePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ShowStoreOfferPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ShowStoreOfferPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SimpleEventPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SimpleEventPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SimulationTypePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SimulationTypePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SpawnExperienceOrbPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SpawnExperienceOrbPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SpawnParticleEffectPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SpawnParticleEffectPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::StartGamePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::StartGamePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::StopSoundPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::StopSoundPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::StructureBlockUpdatePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::StructureBlockUpdatePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::StructureDataRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::StructureDataRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::StructureDataResponsePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::StructureDataResponsePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SubChunkPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SubChunkPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SubChunkRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SubChunkRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SubClientLoginPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SubClientLoginPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SyncActorPropertyPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SyncActorPropertyPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::SyncWorldClocksPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::SyncWorldClocksPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::TakeItemActorPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::TakeItemActorPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::TextPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::TextPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::TickingAreaLoadStatusPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::TickingAreaLoadStatusPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::ToastRequestPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::ToastRequestPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::TransferPlayerPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::TransferPlayerPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::TrimDataPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::TrimDataPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UnlockedRecipesPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UnlockedRecipesPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateAbilitiesPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateAbilitiesPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateAdventureSettingsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateAdventureSettingsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateAttributesPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateAttributesPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateBlockPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateBlockPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateBlockSyncedPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateBlockSyncedPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateClientInputLocksPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateClientInputLocksPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateClientOptionsPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateClientOptionsPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateEquipPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateEquipPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdatePlayerGameTypePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdatePlayerGameTypePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateSoftEnumPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateSoftEnumPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateSubChunkBlocksPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateSubChunkBlocksPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::UpdateTradePacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::UpdateTradePacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::VoxelShapesPacket(pk) => {
                        <<V2223 as ProtoVersionPackets>::VoxelShapesPacket as bedrock_protocol_core::ProtoCodec>::size_hint(
                            pk.as_ref(),
                        )
                    }
                    V2223::Unknown(pk) => pk.buf.len(),
                }
        }
        #[inline]
        fn id(&self) -> u16 {
            match self {
                V2223::ActorEventPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ActorEventPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ActorPickRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ActorPickRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AddActorPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AddActorPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AddBehaviourTreePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AddBehaviourTreePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AddItemActorPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AddItemActorPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AddPaintingPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AddPaintingPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AddPlayerPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AddPlayerPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AddVolumeEntityPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AddVolumeEntityPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AgentActionEventPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AgentActionEventPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AgentAnimationPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AgentAnimationPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AnimateEntityPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AnimateEntityPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AnimatePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AnimatePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AnvilDamagePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AnvilDamagePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AutomationClientConnectPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AutomationClientConnectPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AvailableActorIdentifiersPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AvailableActorIdentifiersPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AvailableCommandsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AvailableCommandsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::AwardAchievementPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::AwardAchievementPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::BiomeDefinitionListPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::BiomeDefinitionListPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::BlockActorDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::BlockActorDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::BlockEventPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::BlockEventPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::BlockPickRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::BlockPickRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::BookEditPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::BookEditPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::BossEventPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::BossEventPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CameraAimAssistActorPriorityPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CameraAimAssistActorPriorityPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CameraAimAssistInstructionPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CameraAimAssistInstructionPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CameraAimAssistPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CameraAimAssistPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CameraAimAssistPresetsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CameraAimAssistPresetsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CameraInstructionPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CameraInstructionPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CameraPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CameraPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CameraPresetsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CameraPresetsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CameraShakePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CameraShakePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CameraSplinePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CameraSplinePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ChangeDimensionPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ChangeDimensionPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ChangeMobPropertyPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ChangeMobPropertyPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ChunkRadiusUpdatedPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ChunkRadiusUpdatedPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundAttributeLayerSyncPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundAttributeLayerSyncPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundCloseFormPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundCloseFormPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundControlSchemeSetPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundControlSchemeSetPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundDataDrivenUICloseScreenPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUICloseScreenPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundDataDrivenUIReloadPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIReloadPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundDataDrivenUIShowScreenPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundDataDrivenUIShowScreenPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundDataStorePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundDataStorePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundDebugRendererPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundDebugRendererPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundMapItemDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundMapItemDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundTextureShiftPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundTextureShiftPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientBoundUpdateSoundDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientBoundUpdateSoundDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientCacheBlobStatusPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientCacheBlobStatusPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientCacheMissResponsePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientCacheMissResponsePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientCacheStatusPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientCacheStatusPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ClientToServerHandshakePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ClientToServerHandshakePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CodeBuilderPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CodeBuilderPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CodeBuilderSourcePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CodeBuilderSourcePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CommandBlockUpdatePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CommandBlockUpdatePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CommandOutputPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CommandOutputPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CommandRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CommandRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CompletedUsingItemPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CompletedUsingItemPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ContainerClosePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ContainerClosePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ContainerOpenPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ContainerOpenPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ContainerRegistryCleanupPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ContainerRegistryCleanupPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ContainerSetDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ContainerSetDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CorrectPlayerMovePredictionPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CorrectPlayerMovePredictionPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CraftingDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CraftingDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CreatePhotoPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CreatePhotoPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CreativeContentPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CreativeContentPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::CurrentStructureFeaturePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::CurrentStructureFeaturePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::DeathInfoPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::DeathInfoPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::DebugDrawerPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::DebugDrawerPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::DebugInfoPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::DebugInfoPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::DimensionDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::DimensionDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::DisconnectPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::DisconnectPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::EditorNetworkPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::EditorNetworkPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::EduUriResourcePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::EduUriResourcePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::EducationSettingsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::EducationSettingsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::EmoteListPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::EmoteListPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::EmotePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::EmotePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::FeatureRegistryPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::FeatureRegistryPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::GameRulesChangedPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::GameRulesChangedPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::GameTestRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::GameTestRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::GameTestResultsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::GameTestResultsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::GraphicsParameterOverridePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::GraphicsParameterOverridePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::GuiDataPickItemPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::GuiDataPickItemPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::HurtArmorPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::HurtArmorPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::InteractPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::InteractPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::InventoryContentPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::InventoryContentPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::InventorySlotPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::InventorySlotPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::InventoryTransactionPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::InventoryTransactionPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ItemComponentPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ItemComponentPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ItemStackRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ItemStackRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ItemStackResponsePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ItemStackResponsePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::JigsawStructureDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::JigsawStructureDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LabTablePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LabTablePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LecternUpdatePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LecternUpdatePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LegacyTelemetryEventPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LegacyTelemetryEventPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LessonProgressPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LessonProgressPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LevelChunkPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LevelChunkPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LevelEventGenericPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LevelEventGenericPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LevelEventPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LevelEventPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LevelSoundEventPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LevelSoundEventPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LocatorBarPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LocatorBarPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::LoginPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::LoginPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MapCreateLockedCopyPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MapCreateLockedCopyPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MapInfoRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MapInfoRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MobArmorEquipmentPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MobArmorEquipmentPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MobEffectPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MobEffectPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MobEquipmentPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MobEquipmentPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ModalFormRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ModalFormRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ModalFormResponsePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ModalFormResponsePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MotionPredictionHintsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MotionPredictionHintsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MoveActorAbsolutePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MoveActorAbsolutePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MoveActorDeltaPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MoveActorDeltaPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MovePlayerPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MovePlayerPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MovementEffectPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MovementEffectPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MovementPredictionSyncPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MovementPredictionSyncPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::MultiplayerSettingsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::MultiplayerSettingsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::NetworkChunkPublisherUpdatePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::NetworkChunkPublisherUpdatePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::NetworkSettingsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::NetworkSettingsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::NetworkStackLatencyPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::NetworkStackLatencyPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::NpcDialoguePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::NpcDialoguePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::NpcRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::NpcRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::OnScreenTextureAnimationPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::OnScreenTextureAnimationPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::OpenSignPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::OpenSignPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PacketViolationWarningPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PacketViolationWarningPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PartyChangedPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PartyChangedPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PartyDestinationCookieResponsePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PartyDestinationCookieResponsePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PhotoTransferPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PhotoTransferPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlaySoundPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlaySoundPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayStatusPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayStatusPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerActionPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerActionPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerArmorDamagePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerArmorDamagePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerAuthInputPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerAuthInputPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerEnchantOptionsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerEnchantOptionsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerFogPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerFogPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerHotbarPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerHotbarPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerListPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerListPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerLocationPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerLocationPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerSkinPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerSkinPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerStartItemCooldownPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerStartItemCooldownPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerToggleCrafterSlotRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerToggleCrafterSlotRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerUpdateEntityOverridesPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerUpdateEntityOverridesPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PlayerVideoCapturePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PlayerVideoCapturePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PositionTrackingDBClientRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PositionTrackingDBClientRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PositionTrackingDBServerBroadcastPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PositionTrackingDBServerBroadcastPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::PurchaseReceiptPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::PurchaseReceiptPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RecordStartedPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RecordStartedPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RefreshEntitlementsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RefreshEntitlementsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RemoveActorPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RemoveActorPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RemoveObjectivePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RemoveObjectivePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RemoveVolumeEntityPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RemoveVolumeEntityPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RequestAbilityPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RequestAbilityPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RequestChunkRadiusPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RequestChunkRadiusPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RequestNetworkSettingsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RequestNetworkSettingsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RequestPermissionsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RequestPermissionsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ResourcePackChunkDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ResourcePackChunkDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ResourcePackChunkRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ResourcePackChunkRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ResourcePackClientResponsePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ResourcePackClientResponsePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ResourcePackDataInfoPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ResourcePackDataInfoPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ResourcePackStackPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ResourcePackStackPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ResourcePacksInfoPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ResourcePacksInfoPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ResourcePacksReadyForValidationPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ResourcePacksReadyForValidationPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::RespawnPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::RespawnPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ScriptMessagePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ScriptMessagePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SendPartyDestinationCookiePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SendPartyDestinationCookiePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerBoundDataDrivenClosedPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerBoundDataDrivenClosedPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerBoundDataStorePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerBoundDataStorePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerBoundDiagnosticsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerBoundDiagnosticsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerBoundLoadingScreenPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerBoundLoadingScreenPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerBoundPackSettingChangePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerBoundPackSettingChangePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerPlayerPostMovePositionPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerPlayerPostMovePositionPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerPresenceInfoPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerPresenceInfoPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerSettingsRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerSettingsRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerSettingsResponsePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerSettingsResponsePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerStatsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerStatsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerStoreInfoPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerStoreInfoPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ServerToClientHandshakePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ServerToClientHandshakePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetActorDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetActorDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetActorLinkPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetActorLinkPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetActorMotionPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetActorMotionPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetCommandsEnabledPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetCommandsEnabledPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetDefaultGameTypePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetDefaultGameTypePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetDifficultyPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetDifficultyPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetDisplayObjectivePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetDisplayObjectivePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetHealthPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetHealthPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetHudPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetHudPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetLastHurtByPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetLastHurtByPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetLocalPlayerAsInitializedPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetPlayerFurnaceOptionsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetPlayerFurnaceOptionsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetPlayerGameTypePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetPlayerGameTypePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetPlayerInventoryOptionsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetPlayerInventoryOptionsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetScorePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetScorePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetScoreboardIdentityPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetScoreboardIdentityPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetSpawnPositionPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetSpawnPositionPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetTimePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetTimePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SetTitlePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SetTitlePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SettingsCommandPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SettingsCommandPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ShowCreditsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ShowCreditsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ShowProfilePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ShowProfilePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ShowStoreOfferPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ShowStoreOfferPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SimpleEventPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SimpleEventPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SimulationTypePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SimulationTypePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SpawnExperienceOrbPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SpawnExperienceOrbPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SpawnParticleEffectPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SpawnParticleEffectPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::StartGamePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::StartGamePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::StopSoundPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::StopSoundPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::StructureBlockUpdatePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::StructureBlockUpdatePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::StructureDataRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::StructureDataRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::StructureDataResponsePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::StructureDataResponsePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SubChunkPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SubChunkPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SubChunkRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SubChunkRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SubClientLoginPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SubClientLoginPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SyncActorPropertyPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SyncActorPropertyPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::SyncWorldClocksPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::SyncWorldClocksPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::TakeItemActorPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::TakeItemActorPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::TextPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::TextPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::TickingAreaLoadStatusPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::TickingAreaLoadStatusPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::ToastRequestPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::ToastRequestPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::TransferPlayerPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::TransferPlayerPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::TrimDataPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::TrimDataPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UnlockedRecipesPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UnlockedRecipesPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateAbilitiesPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateAbilitiesPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateAdventureSettingsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateAdventureSettingsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateAttributesPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateAttributesPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateBlockPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateBlockPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateBlockSyncedPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateBlockSyncedPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateClientInputLocksPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateClientInputLocksPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateClientOptionsPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateClientOptionsPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateEquipPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateEquipPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdatePlayerGameTypePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdatePlayerGameTypePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateSoftEnumPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateSoftEnumPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateSubChunkBlocksPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateSubChunkBlocksPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::UpdateTradePacket(_) => {
                    <<V2223 as ProtoVersionPackets>::UpdateTradePacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::VoxelShapesPacket(_) => {
                    <<V2223 as ProtoVersionPackets>::VoxelShapesPacket as bedrock_protocol_core::Packet>::ID
                }
                V2223::Unknown(pk) => pk.id,
            }
        }
    }
    impl ProtoVersionPackets for V2223 {
        type ActorEventPacket = crate::version::v975::packets::ActorEventPacket<Self>;
        type ActorPickRequestPacket = crate::version::v662::packets::ActorPickRequestPacket;
        type AddActorPacket = crate::version::v662::packets::AddActorPacket<Self>;
        type AddBehaviourTreePacket = crate::version::v662::packets::AddBehaviourTreePacket;
        type AddItemActorPacket = crate::version::v662::packets::AddItemActorPacket<Self>;
        type AddPaintingPacket = crate::version::v662::packets::AddPaintingPacket<Self>;
        type AddPlayerPacket = crate::version::v662::packets::AddPlayerPacket<Self>;
        type AddVolumeEntityPacket = crate::version::v662::packets::AddVolumeEntityPacket<Self>;
        type AgentActionEventPacket = crate::version::v662::packets::AgentActionEventPacket<Self>;
        type AgentAnimationPacket = crate::version::v662::packets::AgentAnimationPacket<Self>;
        type AnimateEntityPacket = crate::version::v662::packets::AnimateEntityPacket<Self>;
        type AnimatePacket = crate::version::v898::packets::AnimatePacket<Self>;
        type AnvilDamagePacket = crate::version::v2168::packets::AnvilDamagePacket<Self>;
        type AutomationClientConnectPacket =
            crate::version::v662::packets::AutomationClientConnectPacket<Self>;
        type AvailableActorIdentifiersPacket =
            crate::version::v662::packets::AvailableActorIdentifiersPacket;
        type AvailableCommandsPacket = crate::version::v898::packets::AvailableCommandsPacket;
        type AwardAchievementPacket = crate::version::v685::packets::AwardAchievementPacket;
        type BiomeDefinitionListPacket =
            crate::version::v800::packets::BiomeDefinitionListPacket<Self>;
        type BlockActorDataPacket = crate::version::v662::packets::BlockActorDataPacket<Self>;
        type BlockEventPacket = crate::version::v662::packets::BlockEventPacket<Self>;
        type BlockPickRequestPacket = crate::version::v662::packets::BlockPickRequestPacket<Self>;
        type BookEditPacket = crate::version::v924::packets::BookEditPacket<Self>;
        type BossEventPacket = crate::version::v2193::packets::BossEventPacket<Self>;
        type CameraAimAssistActorPriorityPacket =
            crate::version::v924::packets::CameraAimAssistActorPriorityPacket;
        type CameraAimAssistInstructionPacket =
            crate::version::v776::packets::CameraAimAssistInstructionPacket<Self>;
        type CameraAimAssistPacket = crate::version::v827::packets::CameraAimAssistPacket<Self>;
        type CameraAimAssistPresetsPacket =
            crate::version::v800::packets::CameraAimAssistPresetsPacket<Self>;
        type CameraInstructionPacket = crate::version::v712::packets::CameraInstructionPacket<Self>;
        type CameraPacket = crate::version::v662::packets::CameraPacket<Self>;
        type CameraPresetsPacket = crate::version::v662::packets::CameraPresetsPacket<Self>;
        type CameraShakePacket = crate::version::v662::packets::CameraShakePacket<Self>;
        type CameraSplinePacket = crate::version::v924::packets::CameraSplinePacket<Self>;
        type ChangeDimensionPacket = crate::version::v712::packets::ChangeDimensionPacket;
        type ChangeMobPropertyPacket = crate::version::v662::packets::ChangeMobPropertyPacket<Self>;
        type ChunkRadiusUpdatedPacket = crate::version::v662::packets::ChunkRadiusUpdatedPacket;
        type ClientBoundAttributeLayerSyncPacket =
            crate::version::v1001::packets::ClientBoundAttributeLayerSyncPacket;
        type ClientBoundCloseFormPacket = crate::version::v686::packets::ClientBoundCloseFormPacket;
        type ClientBoundControlSchemeSetPacket =
            crate::version::v800::packets::ClientBoundControlSchemeSetPacket<Self>;
        type ClientBoundDataDrivenUICloseAllScreensPacket = ();
        type ClientBoundDataDrivenUICloseScreenPacket =
            crate::version::v944::packets::ClientBoundDataDrivenUICloseScreenPacket;
        type ClientBoundDataDrivenUIReloadPacket =
            crate::version::v924::packets::ClientBoundDataDrivenUIReloadPacket;
        type ClientBoundDataDrivenUIShowScreenPacket =
            crate::version::v944::packets::ClientBoundDataDrivenUIShowScreenPacket;
        type ClientBoundDataStorePacket = crate::version::v924::packets::ClientBoundDataStorePacket;
        type ClientBoundDebugRendererPacket =
            crate::version::v671::packets::ClientBoundDebugRendererPacket;
        type ClientBoundMapItemDataPacket =
            crate::version::v2168::packets::ClientBoundMapItemDataPacket<Self>;
        type ClientBoundTextureShiftPacket =
            crate::version::v924::packets::ClientBoundTextureShiftPacket;
        type ClientBoundUpdateSoundDataPacket =
            crate::version::v2193::packets::ClientBoundUpdateSoundDataPacket<Self>;
        type ClientCacheBlobStatusPacket =
            crate::version::v1001::packets::ClientCacheBlobStatusPacket;
        type ClientCacheMissResponsePacket =
            crate::version::v662::packets::ClientCacheMissResponsePacket;
        type ClientCacheStatusPacket = crate::version::v662::packets::ClientCacheStatusPacket;
        type ClientToServerHandshakePacket =
            crate::version::v662::packets::ClientToServerHandshakePacket;
        type CodeBuilderPacket = crate::version::v662::packets::CodeBuilderPacket;
        type CodeBuilderSourcePacket = crate::version::v685::packets::CodeBuilderSourcePacket<Self>;
        type CommandBlockUpdatePacket =
            crate::version::v776::packets::CommandBlockUpdatePacket<Self>;
        type CommandOutputPacket = crate::version::v898::packets::CommandOutputPacket<Self>;
        type CommandRequestPacket = crate::version::v898::packets::CommandRequestPacket<Self>;
        type CompletedUsingItemPacket =
            crate::version::v662::packets::CompletedUsingItemPacket<Self>;
        type CompressedBiomeDefinitionListPacket = ();
        type ContainerClosePacket = crate::version::v685::packets::ContainerClosePacket<Self>;
        type ContainerOpenPacket = crate::version::v662::packets::ContainerOpenPacket<Self>;
        type ContainerRegistryCleanupPacket =
            crate::version::v729::packets::ContainerRegistryCleanupPacket<Self>;
        type ContainerSetDataPacket = crate::version::v662::packets::ContainerSetDataPacket<Self>;
        type CorrectPlayerMovePredictionPacket =
            crate::version::v827::packets::CorrectPlayerMovePredictionPacket<Self>;
        type CraftingDataPacket = crate::version::v2168::packets::CraftingDataPacket<Self>;
        type CreatePhotoPacket = crate::version::v662::packets::CreatePhotoPacket;
        type CreativeContentPacket = crate::version::v2168::packets::CreativeContentPacket<Self>;
        type CurrentStructureFeaturePacket =
            crate::version::v712::packets::CurrentStructureFeaturePacket;
        type DeathInfoPacket = crate::version::v662::packets::DeathInfoPacket;
        type DebugDrawerPacket = crate::version::v818::packets::DebugDrawerPacket<Self>;
        type DebugInfoPacket = crate::version::v662::packets::DebugInfoPacket<Self>;
        type DimensionDataPacket = crate::version::v662::packets::DimensionDataPacket<Self>;
        type DisconnectPacket = crate::version::v712::packets::DisconnectPacket<Self>;
        type EditorNetworkPacket = crate::version::v662::packets::EditorNetworkPacket;
        type EduUriResourcePacket = crate::version::v662::packets::EduUriResourcePacket<Self>;
        type EducationSettingsPacket = crate::version::v662::packets::EducationSettingsPacket<Self>;
        type EmoteListPacket = crate::version::v662::packets::EmoteListPacket<Self>;
        type EmotePacket = crate::version::v729::packets::EmotePacket<Self>;
        type FeatureRegistryPacket = crate::version::v662::packets::FeatureRegistryPacket;
        type FilterTextPacket = ();
        type GameRulesChangedPacket = crate::version::v662::packets::GameRulesChangedPacket<Self>;
        type GameTestRequestPacket = crate::version::v662::packets::GameTestRequestPacket<Self>;
        type GameTestResultsPacket = crate::version::v662::packets::GameTestResultsPacket;
        type GraphicsParameterOverridePacket =
            crate::version::v1001::packets::GraphicsParameterOverridePacket;
        type GuiDataPickItemPacket = crate::version::v662::packets::GuiDataPickItemPacket;
        type HurtArmorPacket = crate::version::v662::packets::HurtArmorPacket;
        type InteractPacket = crate::version::v898::packets::InteractPacket<Self>;
        type InventoryContentPacket = crate::version::v1001::packets::InventoryContentPacket<Self>;
        type InventorySlotPacket = crate::version::v975::packets::InventorySlotPacket<Self>;
        type InventoryTransactionPacket =
            crate::version::v2193::packets::InventoryTransactionPacket<Self>;
        type ItemComponentPacket = crate::version::v776::packets::ItemComponentPacket<Self>;
        type ItemStackRequestPacket = crate::version::v662::packets::ItemStackRequestPacket<Self>;
        type ItemStackResponsePacket = crate::version::v662::packets::ItemStackResponsePacket<Self>;
        type JigsawStructureDataPacket = crate::version::v712::packets::JigsawStructureDataPacket;
        type LabTablePacket = crate::version::v662::packets::LabTablePacket<Self>;
        type LecternUpdatePacket = crate::version::v662::packets::LecternUpdatePacket<Self>;
        type LegacyTelemetryEventPacket =
            crate::version::v898::packets::LegacyTelemetryEventPacket<Self>;
        type LessonProgressPacket = crate::version::v662::packets::LessonProgressPacket<Self>;
        type LevelChunkPacket = crate::version::v2168::packets::LevelChunkPacket<Self>;
        type LevelEventGenericPacket = crate::version::v662::packets::LevelEventGenericPacket<Self>;
        type LevelEventPacket = crate::version::v662::packets::LevelEventPacket;
        type LevelSoundEventPacket = crate::version::v1001::packets::LevelSoundEventPacket;
        type LevelSoundEventV1Packet = ();
        type LevelSoundEventV2Packet = ();
        type LocatorBarPacket = crate::version::v975::packets::LocatorBarPacket;
        type LoginPacket = crate::version::v662::packets::LoginPacket;
        type MapCreateLockedCopyPacket =
            crate::version::v662::packets::MapCreateLockedCopyPacket<Self>;
        type MapInfoRequestPacket = crate::version::v662::packets::MapInfoRequestPacket<Self>;
        type MobArmorEquipmentPacket =
            crate::version::v1001::packets::MobArmorEquipmentPacket<Self>;
        type MobEffectPacket = crate::version::v898::packets::MobEffectPacket<Self>;
        type MobEquipmentPacket = crate::version::v975::packets::MobEquipmentPacket<Self>;
        type ModalFormRequestPacket = crate::version::v662::packets::ModalFormRequestPacket;
        type ModalFormResponsePacket = crate::version::v662::packets::ModalFormResponsePacket<Self>;
        type MotionPredictionHintsPacket =
            crate::version::v662::packets::MotionPredictionHintsPacket<Self>;
        type MoveActorAbsolutePacket = crate::version::v662::packets::MoveActorAbsolutePacket<Self>;
        type MoveActorDeltaPacket = crate::version::v662::packets::MoveActorDeltaPacket<Self>;
        type MovePlayerPacket = crate::version::v2168::packets::MovePlayerPacket<Self>;
        type MovementEffectPacket = crate::version::v748::packets::MovementEffectPacket<Self>;
        type MovementPredictionSyncPacket =
            crate::version::v975::packets::MovementPredictionSyncPacket<Self>;
        type MultiplayerSettingsPacket =
            crate::version::v662::packets::MultiplayerSettingsPacket<Self>;
        type NetworkChunkPublisherUpdatePacket =
            crate::version::v662::packets::NetworkChunkPublisherUpdatePacket<Self>;
        type NetworkSettingsPacket = crate::version::v662::packets::NetworkSettingsPacket<Self>;
        type NetworkStackLatencyPacket = crate::version::v662::packets::NetworkStackLatencyPacket;
        type NpcDialoguePacket = crate::version::v662::packets::NpcDialoguePacket;
        type NpcRequestPacket = crate::version::v662::packets::NpcRequestPacket<Self>;
        type OnScreenTextureAnimationPacket =
            crate::version::v662::packets::OnScreenTextureAnimationPacket;
        type OpenSignPacket = crate::version::v662::packets::OpenSignPacket<Self>;
        type PacketViolationWarningPacket =
            crate::version::v662::packets::PacketViolationWarningPacket<Self>;
        type PartyChangedPacket = crate::version::v975::packets::PartyChangedPacket;
        type PartyDestinationCookieResponsePacket =
            crate::version::v1001::packets::PartyDestinationCookieResponsePacket;
        type PassengerJumpPacket = ();
        type PhotoTransferPacket = crate::version::v662::packets::PhotoTransferPacket<Self>;
        type PlaySoundPacket = crate::version::v2193::packets::PlaySoundPacket<Self>;
        type PlayStatusPacket = crate::version::v662::packets::PlayStatusPacket<Self>;
        type PlayerActionPacket = crate::version::v662::packets::PlayerActionPacket<Self>;
        type PlayerArmorDamagePacket = crate::version::v844::packets::PlayerArmorDamagePacket;
        type PlayerAuthInputPacket = crate::version::v2193::packets::PlayerAuthInputPacket<Self>;
        type PlayerEnchantOptionsPacket =
            crate::version::v662::packets::PlayerEnchantOptionsPacket<Self>;
        type PlayerFogPacket = crate::version::v662::packets::PlayerFogPacket;
        type PlayerHotbarPacket = crate::version::v662::packets::PlayerHotbarPacket<Self>;
        type PlayerInputPacket = ();
        type PlayerListPacket = crate::version::v2168::packets::PlayerListPacket<Self>;
        type PlayerLocationPacket = crate::version::v2168::packets::PlayerLocationPacket;
        type PlayerSkinPacket = crate::version::v2168::packets::PlayerSkinPacket<Self>;
        type PlayerStartItemCooldownPacket =
            crate::version::v662::packets::PlayerStartItemCooldownPacket;
        type PlayerToggleCrafterSlotRequestPacket =
            crate::version::v662::packets::PlayerToggleCrafterSlotRequestPacket;
        type PlayerUpdateEntityOverridesPacket =
            crate::version::v2168::packets::PlayerUpdateEntityOverridesPacket<Self>;
        type PlayerVideoCapturePacket = crate::version::v786::packets::PlayerVideoCapturePacket;
        type PositionTrackingDBClientRequestPacket =
            crate::version::v662::packets::PositionTrackingDBClientRequestPacket<Self>;
        type PositionTrackingDBServerBroadcastPacket =
            crate::version::v662::packets::PositionTrackingDBServerBroadcastPacket<Self>;
        type PurchaseReceiptPacket = crate::version::v662::packets::PurchaseReceiptPacket;
        type RecordStartedPacket = crate::version::v2193::packets::RecordStartedPacket<Self>;
        type RefreshEntitlementsPacket = crate::version::v662::packets::RefreshEntitlementsPacket;
        type RemoveActorPacket = crate::version::v662::packets::RemoveActorPacket<Self>;
        type RemoveObjectivePacket = crate::version::v662::packets::RemoveObjectivePacket;
        type RemoveVolumeEntityPacket =
            crate::version::v662::packets::RemoveVolumeEntityPacket<Self>;
        type RequestAbilityPacket = crate::version::v662::packets::RequestAbilityPacket<Self>;
        type RequestChunkRadiusPacket = crate::version::v662::packets::RequestChunkRadiusPacket;
        type RequestNetworkSettingsPacket =
            crate::version::v662::packets::RequestNetworkSettingsPacket;
        type RequestPermissionsPacket =
            crate::version::v662::packets::RequestPermissionsPacket<Self>;
        type ResourcePackChunkDataPacket =
            crate::version::v662::packets::ResourcePackChunkDataPacket;
        type ResourcePackChunkRequestPacket =
            crate::version::v662::packets::ResourcePackChunkRequestPacket;
        type ResourcePackClientResponsePacket =
            crate::version::v2168::packets::ResourcePackClientResponsePacket;
        type ResourcePackDataInfoPacket =
            crate::version::v662::packets::ResourcePackDataInfoPacket<Self>;
        type ResourcePackStackPacket = crate::version::v898::packets::ResourcePackStackPacket<Self>;
        type ResourcePacksInfoPacket = crate::version::v2168::packets::ResourcePacksInfoPacket;
        type ResourcePacksReadyForValidationPacket =
            crate::version::v944::packets::ResourcePacksReadyForValidationPacket;
        type RespawnPacket = crate::version::v662::packets::RespawnPacket<Self>;
        type ScriptMessagePacket = crate::version::v662::packets::ScriptMessagePacket;
        type SendPartyDestinationCookiePacket =
            crate::version::v1001::packets::SendPartyDestinationCookiePacket;
        type ServerBoundDataDrivenClosedPacket =
            crate::version::v944::packets::ServerBoundDataDrivenClosedPacket;
        type ServerBoundDataStorePacket = crate::version::v924::packets::ServerBoundDataStorePacket;
        type ServerBoundDiagnosticsPacket =
            crate::version::v2193::packets::ServerBoundDiagnosticsPacket;
        type ServerBoundLoadingScreenPacket =
            crate::version::v712::packets::ServerBoundLoadingScreenPacket;
        type ServerBoundPackSettingChangePacket =
            crate::version::v2193::packets::ServerBoundPackSettingChangePacket;
        type ServerPlayerPostMovePositionPacket =
            crate::version::v662::packets::ServerPlayerPostMovePositionPacket;
        type ServerPresenceInfoPacket = crate::version::v2168::packets::ServerPresenceInfoPacket;
        type ServerSettingsRequestPacket =
            crate::version::v662::packets::ServerSettingsRequestPacket;
        type ServerSettingsResponsePacket =
            crate::version::v662::packets::ServerSettingsResponsePacket;
        type ServerStatsPacket = crate::version::v662::packets::ServerStatsPacket;
        type ServerStoreInfoPacket = crate::version::v975::packets::ServerStoreInfoPacket;
        type ServerToClientHandshakePacket =
            crate::version::v662::packets::ServerToClientHandshakePacket;
        type SetActorDataPacket = crate::version::v662::packets::SetActorDataPacket<Self>;
        type SetActorLinkPacket = crate::version::v662::packets::SetActorLinkPacket<Self>;
        type SetActorMotionPacket = crate::version::v662::packets::SetActorMotionPacket<Self>;
        type SetCommandsEnabledPacket = crate::version::v662::packets::SetCommandsEnabledPacket;
        type SetDefaultGameTypePacket =
            crate::version::v662::packets::SetDefaultGameTypePacket<Self>;
        type SetDifficultyPacket = crate::version::v662::packets::SetDifficultyPacket;
        type SetDisplayObjectivePacket =
            crate::version::v662::packets::SetDisplayObjectivePacket<Self>;
        type SetHealthPacket = crate::version::v662::packets::SetHealthPacket;
        type SetHudPacket = crate::version::v662::packets::SetHudPacket<Self>;
        type SetLastHurtByPacket = crate::version::v662::packets::SetLastHurtByPacket<Self>;
        type SetLocalPlayerAsInitializedPacket =
            crate::version::v662::packets::SetLocalPlayerAsInitializedPacket<Self>;
        type SetMovementAuthorityPacket = ();
        type SetPlayerFurnaceOptionsPacket =
            crate::version::v2193::packets::SetPlayerFurnaceOptionsPacket;
        type SetPlayerGameTypePacket = crate::version::v662::packets::SetPlayerGameTypePacket<Self>;
        type SetPlayerInventoryOptionsPacket =
            crate::version::v662::packets::SetPlayerInventoryOptionsPacket<Self>;
        type SetScorePacket = crate::version::v2168::packets::SetScorePacket<Self>;
        type SetScoreboardIdentityPacket =
            crate::version::v2168::packets::SetScoreboardIdentityPacket<Self>;
        type SetSpawnPositionPacket = crate::version::v662::packets::SetSpawnPositionPacket<Self>;
        type SetTimePacket = crate::version::v662::packets::SetTimePacket;
        type SetTitlePacket = crate::version::v712::packets::SetTitlePacket;
        type SettingsCommandPacket = crate::version::v662::packets::SettingsCommandPacket;
        type ShowCreditsPacket = crate::version::v662::packets::ShowCreditsPacket<Self>;
        type ShowProfilePacket = crate::version::v662::packets::ShowProfilePacket;
        type ShowStoreOfferPacket = crate::version::v859::packets::ShowStoreOfferPacket<Self>;
        type SimpleEventPacket = crate::version::v662::packets::SimpleEventPacket;
        type SimulationTypePacket = crate::version::v662::packets::SimulationTypePacket<Self>;
        type SpawnExperienceOrbPacket = crate::version::v662::packets::SpawnExperienceOrbPacket;
        type SpawnParticleEffectPacket =
            crate::version::v662::packets::SpawnParticleEffectPacket<Self>;
        type StartGamePacket = crate::version::v2168::packets::StartGamePacket<Self>;
        type StopSoundPacket = crate::version::v712::packets::StopSoundPacket;
        type StructureBlockUpdatePacket =
            crate::version::v662::packets::StructureBlockUpdatePacket<Self>;
        type StructureDataRequestPacket =
            crate::version::v662::packets::StructureDataRequestPacket<Self>;
        type StructureDataResponsePacket =
            crate::version::v662::packets::StructureDataResponsePacket<Self>;
        type SubChunkPacket = crate::version::v2193::packets::SubChunkPacket<Self>;
        type SubChunkRequestPacket = crate::version::v1001::packets::SubChunkRequestPacket<Self>;
        type SubClientLoginPacket = crate::version::v662::packets::SubClientLoginPacket;
        type SyncActorPropertyPacket = crate::version::v662::packets::SyncActorPropertyPacket;
        type SyncWorldClocksPacket = crate::version::v944::packets::SyncWorldClocksPacket;
        type TakeItemActorPacket = crate::version::v662::packets::TakeItemActorPacket<Self>;
        type TextPacket = crate::version::v898::packets::TextPacket<Self>;
        type TickSyncPacket = ();
        type TickingAreaLoadStatusPacket =
            crate::version::v662::packets::TickingAreaLoadStatusPacket;
        type ToastRequestPacket = crate::version::v662::packets::ToastRequestPacket;
        type TransferPlayerPacket = crate::version::v2168::packets::TransferPlayerPacket<Self>;
        type TrimDataPacket = crate::version::v662::packets::TrimDataPacket;
        type UnlockedRecipesPacket = crate::version::v662::packets::UnlockedRecipesPacket;
        type UpdateAbilitiesPacket = crate::version::v662::packets::UpdateAbilitiesPacket<Self>;
        type UpdateAdventureSettingsPacket =
            crate::version::v662::packets::UpdateAdventureSettingsPacket<Self>;
        type UpdateAttributesPacket = crate::version::v729::packets::UpdateAttributesPacket<Self>;
        type UpdateBlockPacket = crate::version::v662::packets::UpdateBlockPacket<Self>;
        type UpdateBlockSyncedPacket = crate::version::v662::packets::UpdateBlockSyncedPacket<Self>;
        type UpdateClientInputLocksPacket =
            crate::version::v944::packets::UpdateClientInputLocksPacket;
        type UpdateClientOptionsPacket = crate::version::v975::packets::UpdateClientOptionsPacket;
        type UpdateEquipPacket = crate::version::v662::packets::UpdateEquipPacket<Self>;
        type UpdatePlayerGameTypePacket =
            crate::version::v671::packets::UpdatePlayerGameTypePacket<Self>;
        type UpdateSoftEnumPacket = crate::version::v662::packets::UpdateSoftEnumPacket<Self>;
        type UpdateSubChunkBlocksPacket =
            crate::version::v662::packets::UpdateSubChunkBlocksPacket<Self>;
        type UpdateTradePacket = crate::version::v662::packets::UpdateTradePacket<Self>;
        type VoxelShapesPacket = crate::version::v944::packets::VoxelShapesPacket;
    }
    impl ProtoVersionTypes for V2223 {
        type ActorLink = crate::version::v712::types::ActorLink<Self>;
        type ActorRuntimeID = crate::version::v662::types::ActorRuntimeID;
        type ActorUniqueID = crate::version::v662::types::ActorUniqueID;
        type AdventureSettings = crate::version::v662::types::AdventureSettings;
        type BaseDescription = crate::version::v662::types::BaseDescription<Self>;
        type BaseGameVersion = crate::version::v662::types::BaseGameVersion;
        type BiomeCappedSurfaceData = crate::version::v800::types::BiomeCappedSurfaceData;
        type BiomeClimateData = crate::version::v844::types::BiomeClimateData;
        type BiomeConditionalTransformationData =
            crate::version::v800::types::BiomeConditionalTransformationData<Self>;
        type BiomeConsolidatedFeatureList =
            crate::version::v800::types::BiomeConsolidatedFeatureList<Self>;
        type BiomeCoordinateData = crate::version::v800::types::BiomeCoordinateData;
        type BiomeDefinition = crate::version::v844::types::BiomeDefinition<Self>;
        type BiomeDefinitionChunkGenData =
            crate::version::v975::types::BiomeDefinitionChunkGenData<Self>;
        type BiomeElementData = crate::version::v800::types::BiomeElementData<Self>;
        type BiomeLegacyWorldGenRulesData =
            crate::version::v800::types::BiomeLegacyWorldGenRulesData<Self>;
        type BiomeMesaSurfaceData = crate::version::v800::types::BiomeMesaSurfaceData;
        type BiomeMountainParamsData = crate::version::v800::types::BiomeMountainParamsData;
        type BiomeMultinoiseGenRulesData = crate::version::v800::types::BiomeMultinoiseGenRulesData;
        type BiomeNoiseGradientSurfaceData =
            crate::version::v1001::types::BiomeNoiseGradientSurfaceData;
        type BiomeOverworldGenRulesData =
            crate::version::v800::types::BiomeOverworldGenRulesData<Self>;
        type BiomeReplacementData = crate::version::v859::types::BiomeReplacementData;
        type BiomeScatterParamData = crate::version::v800::types::BiomeScatterParamData<Self>;
        type BiomeSurfaceBuilderData = crate::version::v975::types::BiomeSurfaceBuilderData<Self>;
        type BiomeSurfaceMaterialAdjustmentData =
            crate::version::v800::types::BiomeSurfaceMaterialAdjustmentData<Self>;
        type BiomeSurfaceMaterialData = crate::version::v800::types::BiomeSurfaceMaterialData;
        type BiomeWeightedData = crate::version::v800::types::BiomeWeightedData;
        type BiomeWeightedTemperatureData =
            crate::version::v800::types::BiomeWeightedTemperatureData;
        type BlockPos = crate::version::v662::types::BlockPos;
        type CameraAimAssistCategories =
            crate::version::v766::types::CameraAimAssistCategories<Self>;
        type CameraAimAssistCategory = crate::version::v924::types::CameraAimAssistCategory<Self>;
        type CameraAimAssistItemSettings = crate::version::v766::types::CameraAimAssistItemSettings;
        type CameraAimAssistPreset = crate::version::v766::types::CameraAimAssistPreset;
        type CameraAimAssistPresetDefinition =
            crate::version::v924::types::CameraAimAssistPresetDefinition<Self>;
        type CameraAimAssistPriority = crate::version::v766::types::CameraAimAssistPriority;
        type CameraInstruction = crate::version::v924::types::CameraInstruction<Self>;
        type CameraPreset = crate::version::v2193::types::CameraPreset<Self>;
        type CameraPresets = crate::version::v662::types::CameraPresets<Self>;
        type CameraSplineInstruction = crate::version::v944::types::CameraSplineInstruction<Self>;
        type ChunkPos = crate::version::v662::types::ChunkPos;
        type Color = crate::version::v800::types::Color;
        type CommandOriginData = crate::version::v898::types::CommandOriginData<Self>;
        type ContainerMixDataEntry = crate::version::v662::types::ContainerMixDataEntry;
        type CraftingDataEntry = crate::version::v662::types::CraftingDataEntry<Self>;
        type CraftingRecipeIngredient = crate::version::v2168::types::CraftingRecipeIngredient;
        type DataItem = crate::version::v662::types::DataItem<Self>;
        type DebugShape = crate::version::v2193::types::DebugShape<Self>;
        type DimensionDefinitionGroup = crate::version::v2193::types::DimensionDefinitionGroup;
        type EduSharedUriResource = crate::version::v662::types::EduSharedUriResource;
        type EducationLevelSettings = crate::version::v662::types::EducationLevelSettings;
        type EntityNetID = crate::version::v662::types::EntityNetID;
        type Experiments = crate::version::v662::types::Experiments;
        type FullContainerName = crate::version::v729::types::FullContainerName<Self>;
        type GameRulesChangedPacketData = crate::version::v844::types::GameRulesChangedPacketData;
        type GatheringsConfig = crate::version::v2168::types::GatheringsConfig;
        type InventoryAction = crate::version::v1001::types::InventoryAction<Self>;
        type InventorySource = crate::version::v662::types::InventorySource<Self>;
        type InventoryTransaction = crate::version::v662::types::InventoryTransaction<Self>;
        type ItemData = crate::version::v662::types::ItemData;
        type ItemEnchants = crate::version::v662::types::ItemEnchants<Self>;
        type ItemStackRequestNetworkItemInstanceDescriptor =
            crate::version::v2168::types::ItemStackRequestNetworkItemInstanceDescriptor<Self>;
        type ItemStackRequestSlotInfo =
            crate::version::v2168::types::ItemStackRequestSlotInfo<Self>;
        type ItemStackResponseContainerInfo =
            crate::version::v712::types::ItemStackResponseContainerInfo<Self>;
        type ItemStackResponseInfo = crate::version::v2193::types::ItemStackResponseInfo<Self>;
        type ItemStackResponseSlotInfo =
            crate::version::v2193::types::ItemStackResponseSlotInfo<Self>;
        type LevelSettings = crate::version::v2216::types::LevelSettings<Self>;
        type MapDecoration = crate::version::v2168::types::MapDecoration;
        type MapItemTrackedActorUniqueID =
            crate::version::v2168::types::MapItemTrackedActorUniqueID<Self>;
        type MaterialReducerDataEntry = crate::version::v662::types::MaterialReducerDataEntry;
        type MolangVariableMap = crate::version::v662::types::MolangVariableMap;
        type MoveActorAbsoluteData = crate::version::v662::types::MoveActorAbsoluteData<Self>;
        type MoveActorDeltaData = crate::version::v2193::types::MoveActorDeltaData<Self>;
        type MovePlayerTeleportData = crate::version::v2168::types::MovePlayerTeleportData;
        type MultiRecipe = crate::version::v2168::types::MultiRecipe;
        type NetworkBlockPosition = crate::version::v944::types::NetworkBlockPosition;
        type NetworkItemInstanceDescriptor =
            crate::version::v2168::types::NetworkItemInstanceDescriptor;
        type NetworkItemStackDescriptor = crate::version::v2168::types::NetworkItemStackDescriptor;
        type NetworkItemStackDescriptorV2 =
            crate::version::v2168::types::NetworkItemStackDescriptorV2;
        type NetworkPermissions = crate::version::v662::types::NetworkPermissions;
        type PackedItemUseLegacyInventoryTransaction =
            crate::version::v2193::types::PackedItemUseLegacyInventoryTransaction<Self>;
        type PlayerBlockActionData = crate::version::v2168::types::PlayerBlockActionData<Self>;
        type PositionTrackingId = crate::version::v662::types::PositionTrackingId;
        type PotionMixDataEntry = crate::version::v662::types::PotionMixDataEntry;
        type PropertySyncData = crate::version::v662::types::PropertySyncData;
        type RecipeIngredient = crate::version::v2168::types::RecipeIngredient<Self>;
        type RecipeUnlockingRequirement =
            crate::version::v2168::types::RecipeUnlockingRequirement<Self>;
        type RedactableString = crate::version::v2168::types::RedactableString;
        type ScoreboardId = crate::version::v662::types::ScoreboardId;
        type SerializedAbilitiesData = crate::version::v776::types::SerializedAbilitiesData<Self>;
        type SerializedSkin = crate::version::v2168::types::SerializedSkin<Self>;
        type ShapedRecipe = crate::version::v2168::types::ShapedRecipe<Self>;
        type ShapelessRecipe = crate::version::v2168::types::ShapelessRecipe<Self>;
        type ShulkerBoxRecipe = crate::version::v748::types::ShulkerBoxRecipe<Self>;
        type SmithingTransformRecipe = crate::version::v2168::types::SmithingTransformRecipe<Self>;
        type SmithingTrimRecipe = crate::version::v2168::types::SmithingTrimRecipe<Self>;
        type SoundData = crate::version::v2168::types::SoundData;
        type SpawnSettings = crate::version::v662::types::SpawnSettings<Self>;
        type StructureEditorData = crate::version::v2168::types::StructureEditorData<Self>;
        type StructureSettings = crate::version::v662::types::StructureSettings<Self>;
        type SubChunkPos = crate::version::v2168::types::SubChunkPos;
        type SubChunkPosOffset = crate::version::v662::types::SubChunkPosOffset;
        type SyncedPlayerMovementSettings =
            crate::version::v818::types::SyncedPlayerMovementSettings;
        type WebSocketPacketData = crate::version::v662::types::WebSocketPacketData;
    }
    impl ProtoVersionEnums for V2223 {
        type AbilitiesIndex = crate::version::v776::enums::AbilitiesIndex;
        type ActorBlockSyncMessageID = crate::version::v662::enums::ActorBlockSyncMessageID;
        type ActorDamageCause = crate::version::v662::enums::ActorDamageCause;
        type ActorDataIDs = crate::version::v924::enums::ActorDataIDs;
        type ActorEvent = crate::version::v975::enums::ActorEvent;
        type ActorFlags = crate::version::v2168::enums::ActorFlags;
        type ActorLinkType = crate::version::v662::enums::ActorLinkType;
        type ActorType = crate::version::v662::enums::ActorType;
        type AgentActionType = crate::version::v662::enums::AgentActionType;
        type AimAssistAction = crate::version::v729::enums::AimAssistAction;
        type AnimatedTextureType = crate::version::v2168::enums::AnimatedTextureType;
        type AnimationExpression = crate::version::v2168::enums::AnimationExpression;
        type AnimationMode = crate::version::v662::enums::AnimationMode;
        type ArmSizeType = crate::version::v2168::enums::ArmSizeType;
        type AttributeModifierOperation = crate::version::v662::enums::AttributeModifierOperation;
        type AttributeOperands = crate::version::v662::enums::AttributeOperands;
        type AuthoritativeMovementMode = crate::version::v748::enums::AuthoritativeMovementMode;
        type BookEditAction = crate::version::v924::enums::BookEditAction;
        type BossEventUpdateType = crate::version::v776::enums::BossEventUpdateType<Self>;
        type BuildPlatform = crate::version::v662::enums::BuildPlatform;
        type CameraAimAssistOperation = crate::version::v776::enums::CameraAimAssistOperation;
        type CameraShakeAction = crate::version::v662::enums::CameraShakeAction;
        type CameraShakeType = crate::version::v662::enums::CameraShakeType;
        type CameraSplineEaseType = crate::version::v924::enums::CameraSplineEaseType;
        type CameraSplineType = crate::version::v859::enums::CameraSplineType;
        type ChatRestrictionLevel = crate::version::v662::enums::ChatRestrictionLevel;
        type CodeBuilderCodeStatus = crate::version::v685::enums::CodeBuilderCodeStatus;
        type CodeBuilderStorageCategory = crate::version::v662::enums::CodeBuilderStorageCategory;
        type CodeBuilderStorageOperation = crate::version::v662::enums::CodeBuilderStorageOperation;
        type CommandBlockMode = crate::version::v662::enums::CommandBlockMode;
        type CommandOriginType = crate::version::v898::enums::CommandOriginType;
        type CommandOutputType = crate::version::v898::enums::CommandOutputType;
        type CommandParameterOption = crate::version::v662::enums::CommandParameterOption;
        type CommandPermissionLevel = crate::version::v662::enums::CommandPermissionLevel;
        type ComplexInventoryTransactionType =
            crate::version::v662::enums::ComplexInventoryTransactionType;
        type ConnectionFailReason = crate::version::v2193::enums::ConnectionFailReason;
        type ContainerEnumName = crate::version::v944::enums::ContainerEnumName;
        type ContainerID = crate::version::v662::enums::ContainerID;
        type ContainerType = crate::version::v662::enums::ContainerType;
        type ControlScheme = crate::version::v800::enums::ControlScheme;
        type CraftingDataEntryType = crate::version::v662::enums::CraftingDataEntryType<Self>;
        type DataItemType = crate::version::v2168::enums::DataItemType<Self>;
        type Difficulty = crate::version::v662::enums::Difficulty;
        type EasingType = crate::version::v662::enums::EasingType;
        type EditorWorldType = crate::version::v662::enums::EditorWorldType;
        type EducationEditionOffer = crate::version::v2168::enums::EducationEditionOffer;
        type EnchantType = crate::version::v662::enums::EnchantType;
        type GamePublishSetting = crate::version::v662::enums::GamePublishSetting;
        type GameType = crate::version::v662::enums::GameType;
        type GeneratorType = crate::version::v662::enums::GeneratorType;
        type HudElement = crate::version::v786::enums::HudElement;
        type HudVisibility = crate::version::v786::enums::HudVisibility;
        type IdentityDefinitionType = crate::version::v662::enums::IdentityDefinitionType<Self>;
        type InputMode = crate::version::v662::enums::InputMode;
        type InteractionType = crate::version::v662::enums::InteractionType;
        type InventoryLayout = crate::version::v662::enums::InventoryLayout;
        type InventoryLeftTabIndex = crate::version::v662::enums::InventoryLeftTabIndex;
        type InventoryRightTabIndex = crate::version::v662::enums::InventoryRightTabIndex;
        type InventorySourceFlags = crate::version::v662::enums::InventorySourceFlags;
        type InventorySourceType = crate::version::v662::enums::InventorySourceType<Self>;
        type ItemDescriptorType = crate::version::v2168::enums::ItemDescriptorType;
        type ItemReleaseInventoryTransactionType =
            crate::version::v662::enums::ItemReleaseInventoryTransactionType;
        type ItemStackNetResult = crate::version::v2168::enums::ItemStackNetResult;
        type ItemStackRequestActionType =
            crate::version::v2168::enums::ItemStackRequestActionType<Self>;
        type ItemUseInventoryTransactionType =
            crate::version::v662::enums::ItemUseInventoryTransactionType;
        type ItemUseMethod = crate::version::v662::enums::ItemUseMethod;
        type ItemUseOnActorInventoryTransactionType =
            crate::version::v662::enums::ItemUseOnActorInventoryTransactionType;
        type ItemVersion = crate::version::v776::enums::ItemVersion;
        type LabTableReactionType = crate::version::v662::enums::LabTableReactionType;
        type LessonAction = crate::version::v662::enums::LessonAction;
        type LevelEvent = crate::version::v766::enums::LevelEvent;
        type LevelSoundEventType = crate::version::v2168::enums::LevelSoundEventType;
        type MinecraftPacketIds = crate::version::v662::enums::MinecraftPacketIds;
        type Mirror = crate::version::v662::enums::Mirror;
        type ModalFormCancelReason = crate::version::v662::enums::ModalFormCancelReason;
        type MolangVersion = crate::version::v662::enums::MolangVersion;
        type MovementEffectType = crate::version::v1001::enums::MovementEffectType;
        type MultiplayerSettingsPacketType =
            crate::version::v662::enums::MultiplayerSettingsPacketType;
        type NewInteractionModel = crate::version::v2168::enums::NewInteractionModel;
        type ObjectiveSortOrder = crate::version::v662::enums::ObjectiveSortOrder;
        type POIBlockInteractionType = crate::version::v662::enums::POIBlockInteractionType;
        type PackType = crate::version::v662::enums::PackType;
        type PacketCompressionAlgorithm = crate::version::v662::enums::PacketCompressionAlgorithm;
        type PacketViolationSeverity = crate::version::v662::enums::PacketViolationSeverity;
        type PacketViolationType = crate::version::v662::enums::PacketViolationType;
        type ParticleType = crate::version::v2168::enums::ParticleType;
        type PersonaPieceType = crate::version::v2193::enums::PersonaPieceType;
        type PhotoType = crate::version::v662::enums::PhotoType;
        type PlayStatus = crate::version::v662::enums::PlayStatus;
        type PlayerAuthInputData = crate::version::v2168::enums::PlayerAuthInputData;
        type PlayerPermissionLevel = crate::version::v662::enums::PlayerPermissionLevel;
        type PlayerPositionMode = crate::version::v2168::enums::PlayerPositionMode;
        type PlayerRespawnState = crate::version::v662::enums::PlayerRespawnState;
        type PredictionType = crate::version::v827::enums::PredictionType;
        type ResourcePackResponse = crate::version::v662::enums::ResourcePackResponse;
        type Rotation = crate::version::v662::enums::Rotation;
        type ServerAuthMovementMode = ();
        type ShowStoreOfferRedirectType = crate::version::v662::enums::ShowStoreOfferRedirectType;
        type SimulationType = crate::version::v662::enums::SimulationType;
        type SoftEnumUpdateType = crate::version::v662::enums::SoftEnumUpdateType;
        type SpawnBiomeType = crate::version::v662::enums::SpawnBiomeType;
        type SpawnPositionType = crate::version::v662::enums::SpawnPositionType;
        type StructureBlockType = crate::version::v662::enums::StructureBlockType;
        type StructureRedstoneSaveMode = crate::version::v2168::enums::StructureRedstoneSaveMode;
        type StructureTemplateRequestOperation =
            crate::version::v662::enums::StructureTemplateRequestOperation;
        type StructureTemplateResponseType =
            crate::version::v662::enums::StructureTemplateResponseType;
        type TeleportationCause = crate::version::v662::enums::TeleportationCause;
        type TextPacketType = crate::version::v924::enums::TextPacketType;
        type TextProcessingEventOrigin = crate::version::v662::enums::TextProcessingEventOrigin;
        type UIProfile = crate::version::v662::enums::UIProfile;
    }
    impl ProtoVersion for V2223 {
        const PROTOCOL_VERSION: u32 = 2223u32;
        const PROTOCOL_BRANCH: &str = "r/26_u6";
        const GAME_VERSION: &str = "1.26.60";
        const RAKNET_VERSION: u8 = 11u8;
    }
    #[cfg(feature = "packet-dyn")]
    impl AsRef<dyn bedrock_protocol_core::PacketDyn> for V2223 {
        fn as_ref(&self) -> &dyn bedrock_protocol_core::PacketDyn {
            match self {
                V2223::ActorEventPacket(pk) => pk.as_ref(),
                V2223::ActorPickRequestPacket(pk) => pk.as_ref(),
                V2223::AddActorPacket(pk) => pk.as_ref(),
                V2223::AddBehaviourTreePacket(pk) => pk.as_ref(),
                V2223::AddItemActorPacket(pk) => pk.as_ref(),
                V2223::AddPaintingPacket(pk) => pk.as_ref(),
                V2223::AddPlayerPacket(pk) => pk.as_ref(),
                V2223::AddVolumeEntityPacket(pk) => pk.as_ref(),
                V2223::AgentActionEventPacket(pk) => pk.as_ref(),
                V2223::AgentAnimationPacket(pk) => pk.as_ref(),
                V2223::AnimateEntityPacket(pk) => pk.as_ref(),
                V2223::AnimatePacket(pk) => pk.as_ref(),
                V2223::AnvilDamagePacket(pk) => pk.as_ref(),
                V2223::AutomationClientConnectPacket(pk) => pk.as_ref(),
                V2223::AvailableActorIdentifiersPacket(pk) => pk.as_ref(),
                V2223::AvailableCommandsPacket(pk) => pk.as_ref(),
                V2223::AwardAchievementPacket(pk) => pk.as_ref(),
                V2223::BiomeDefinitionListPacket(pk) => pk.as_ref(),
                V2223::BlockActorDataPacket(pk) => pk.as_ref(),
                V2223::BlockEventPacket(pk) => pk.as_ref(),
                V2223::BlockPickRequestPacket(pk) => pk.as_ref(),
                V2223::BookEditPacket(pk) => pk.as_ref(),
                V2223::BossEventPacket(pk) => pk.as_ref(),
                V2223::CameraAimAssistActorPriorityPacket(pk) => pk.as_ref(),
                V2223::CameraAimAssistInstructionPacket(pk) => pk.as_ref(),
                V2223::CameraAimAssistPacket(pk) => pk.as_ref(),
                V2223::CameraAimAssistPresetsPacket(pk) => pk.as_ref(),
                V2223::CameraInstructionPacket(pk) => pk.as_ref(),
                V2223::CameraPacket(pk) => pk.as_ref(),
                V2223::CameraPresetsPacket(pk) => pk.as_ref(),
                V2223::CameraShakePacket(pk) => pk.as_ref(),
                V2223::CameraSplinePacket(pk) => pk.as_ref(),
                V2223::ChangeDimensionPacket(pk) => pk.as_ref(),
                V2223::ChangeMobPropertyPacket(pk) => pk.as_ref(),
                V2223::ChunkRadiusUpdatedPacket(pk) => pk.as_ref(),
                V2223::ClientBoundAttributeLayerSyncPacket(pk) => pk.as_ref(),
                V2223::ClientBoundCloseFormPacket(pk) => pk.as_ref(),
                V2223::ClientBoundControlSchemeSetPacket(pk) => pk.as_ref(),
                V2223::ClientBoundDataDrivenUICloseScreenPacket(pk) => pk.as_ref(),
                V2223::ClientBoundDataDrivenUIReloadPacket(pk) => pk.as_ref(),
                V2223::ClientBoundDataDrivenUIShowScreenPacket(pk) => pk.as_ref(),
                V2223::ClientBoundDataStorePacket(pk) => pk.as_ref(),
                V2223::ClientBoundDebugRendererPacket(pk) => pk.as_ref(),
                V2223::ClientBoundMapItemDataPacket(pk) => pk.as_ref(),
                V2223::ClientBoundTextureShiftPacket(pk) => pk.as_ref(),
                V2223::ClientBoundUpdateSoundDataPacket(pk) => pk.as_ref(),
                V2223::ClientCacheBlobStatusPacket(pk) => pk.as_ref(),
                V2223::ClientCacheMissResponsePacket(pk) => pk.as_ref(),
                V2223::ClientCacheStatusPacket(pk) => pk.as_ref(),
                V2223::ClientToServerHandshakePacket(pk) => pk.as_ref(),
                V2223::CodeBuilderPacket(pk) => pk.as_ref(),
                V2223::CodeBuilderSourcePacket(pk) => pk.as_ref(),
                V2223::CommandBlockUpdatePacket(pk) => pk.as_ref(),
                V2223::CommandOutputPacket(pk) => pk.as_ref(),
                V2223::CommandRequestPacket(pk) => pk.as_ref(),
                V2223::CompletedUsingItemPacket(pk) => pk.as_ref(),
                V2223::ContainerClosePacket(pk) => pk.as_ref(),
                V2223::ContainerOpenPacket(pk) => pk.as_ref(),
                V2223::ContainerRegistryCleanupPacket(pk) => pk.as_ref(),
                V2223::ContainerSetDataPacket(pk) => pk.as_ref(),
                V2223::CorrectPlayerMovePredictionPacket(pk) => pk.as_ref(),
                V2223::CraftingDataPacket(pk) => pk.as_ref(),
                V2223::CreatePhotoPacket(pk) => pk.as_ref(),
                V2223::CreativeContentPacket(pk) => pk.as_ref(),
                V2223::CurrentStructureFeaturePacket(pk) => pk.as_ref(),
                V2223::DeathInfoPacket(pk) => pk.as_ref(),
                V2223::DebugDrawerPacket(pk) => pk.as_ref(),
                V2223::DebugInfoPacket(pk) => pk.as_ref(),
                V2223::DimensionDataPacket(pk) => pk.as_ref(),
                V2223::DisconnectPacket(pk) => pk.as_ref(),
                V2223::EditorNetworkPacket(pk) => pk.as_ref(),
                V2223::EduUriResourcePacket(pk) => pk.as_ref(),
                V2223::EducationSettingsPacket(pk) => pk.as_ref(),
                V2223::EmoteListPacket(pk) => pk.as_ref(),
                V2223::EmotePacket(pk) => pk.as_ref(),
                V2223::FeatureRegistryPacket(pk) => pk.as_ref(),
                V2223::GameRulesChangedPacket(pk) => pk.as_ref(),
                V2223::GameTestRequestPacket(pk) => pk.as_ref(),
                V2223::GameTestResultsPacket(pk) => pk.as_ref(),
                V2223::GraphicsParameterOverridePacket(pk) => pk.as_ref(),
                V2223::GuiDataPickItemPacket(pk) => pk.as_ref(),
                V2223::HurtArmorPacket(pk) => pk.as_ref(),
                V2223::InteractPacket(pk) => pk.as_ref(),
                V2223::InventoryContentPacket(pk) => pk.as_ref(),
                V2223::InventorySlotPacket(pk) => pk.as_ref(),
                V2223::InventoryTransactionPacket(pk) => pk.as_ref(),
                V2223::ItemComponentPacket(pk) => pk.as_ref(),
                V2223::ItemStackRequestPacket(pk) => pk.as_ref(),
                V2223::ItemStackResponsePacket(pk) => pk.as_ref(),
                V2223::JigsawStructureDataPacket(pk) => pk.as_ref(),
                V2223::LabTablePacket(pk) => pk.as_ref(),
                V2223::LecternUpdatePacket(pk) => pk.as_ref(),
                V2223::LegacyTelemetryEventPacket(pk) => pk.as_ref(),
                V2223::LessonProgressPacket(pk) => pk.as_ref(),
                V2223::LevelChunkPacket(pk) => pk.as_ref(),
                V2223::LevelEventGenericPacket(pk) => pk.as_ref(),
                V2223::LevelEventPacket(pk) => pk.as_ref(),
                V2223::LevelSoundEventPacket(pk) => pk.as_ref(),
                V2223::LocatorBarPacket(pk) => pk.as_ref(),
                V2223::LoginPacket(pk) => pk.as_ref(),
                V2223::MapCreateLockedCopyPacket(pk) => pk.as_ref(),
                V2223::MapInfoRequestPacket(pk) => pk.as_ref(),
                V2223::MobArmorEquipmentPacket(pk) => pk.as_ref(),
                V2223::MobEffectPacket(pk) => pk.as_ref(),
                V2223::MobEquipmentPacket(pk) => pk.as_ref(),
                V2223::ModalFormRequestPacket(pk) => pk.as_ref(),
                V2223::ModalFormResponsePacket(pk) => pk.as_ref(),
                V2223::MotionPredictionHintsPacket(pk) => pk.as_ref(),
                V2223::MoveActorAbsolutePacket(pk) => pk.as_ref(),
                V2223::MoveActorDeltaPacket(pk) => pk.as_ref(),
                V2223::MovePlayerPacket(pk) => pk.as_ref(),
                V2223::MovementEffectPacket(pk) => pk.as_ref(),
                V2223::MovementPredictionSyncPacket(pk) => pk.as_ref(),
                V2223::MultiplayerSettingsPacket(pk) => pk.as_ref(),
                V2223::NetworkChunkPublisherUpdatePacket(pk) => pk.as_ref(),
                V2223::NetworkSettingsPacket(pk) => pk.as_ref(),
                V2223::NetworkStackLatencyPacket(pk) => pk.as_ref(),
                V2223::NpcDialoguePacket(pk) => pk.as_ref(),
                V2223::NpcRequestPacket(pk) => pk.as_ref(),
                V2223::OnScreenTextureAnimationPacket(pk) => pk.as_ref(),
                V2223::OpenSignPacket(pk) => pk.as_ref(),
                V2223::PacketViolationWarningPacket(pk) => pk.as_ref(),
                V2223::PartyChangedPacket(pk) => pk.as_ref(),
                V2223::PartyDestinationCookieResponsePacket(pk) => pk.as_ref(),
                V2223::PhotoTransferPacket(pk) => pk.as_ref(),
                V2223::PlaySoundPacket(pk) => pk.as_ref(),
                V2223::PlayStatusPacket(pk) => pk.as_ref(),
                V2223::PlayerActionPacket(pk) => pk.as_ref(),
                V2223::PlayerArmorDamagePacket(pk) => pk.as_ref(),
                V2223::PlayerAuthInputPacket(pk) => pk.as_ref(),
                V2223::PlayerEnchantOptionsPacket(pk) => pk.as_ref(),
                V2223::PlayerFogPacket(pk) => pk.as_ref(),
                V2223::PlayerHotbarPacket(pk) => pk.as_ref(),
                V2223::PlayerListPacket(pk) => pk.as_ref(),
                V2223::PlayerLocationPacket(pk) => pk.as_ref(),
                V2223::PlayerSkinPacket(pk) => pk.as_ref(),
                V2223::PlayerStartItemCooldownPacket(pk) => pk.as_ref(),
                V2223::PlayerToggleCrafterSlotRequestPacket(pk) => pk.as_ref(),
                V2223::PlayerUpdateEntityOverridesPacket(pk) => pk.as_ref(),
                V2223::PlayerVideoCapturePacket(pk) => pk.as_ref(),
                V2223::PositionTrackingDBClientRequestPacket(pk) => pk.as_ref(),
                V2223::PositionTrackingDBServerBroadcastPacket(pk) => pk.as_ref(),
                V2223::PurchaseReceiptPacket(pk) => pk.as_ref(),
                V2223::RecordStartedPacket(pk) => pk.as_ref(),
                V2223::RefreshEntitlementsPacket(pk) => pk.as_ref(),
                V2223::RemoveActorPacket(pk) => pk.as_ref(),
                V2223::RemoveObjectivePacket(pk) => pk.as_ref(),
                V2223::RemoveVolumeEntityPacket(pk) => pk.as_ref(),
                V2223::RequestAbilityPacket(pk) => pk.as_ref(),
                V2223::RequestChunkRadiusPacket(pk) => pk.as_ref(),
                V2223::RequestNetworkSettingsPacket(pk) => pk.as_ref(),
                V2223::RequestPermissionsPacket(pk) => pk.as_ref(),
                V2223::ResourcePackChunkDataPacket(pk) => pk.as_ref(),
                V2223::ResourcePackChunkRequestPacket(pk) => pk.as_ref(),
                V2223::ResourcePackClientResponsePacket(pk) => pk.as_ref(),
                V2223::ResourcePackDataInfoPacket(pk) => pk.as_ref(),
                V2223::ResourcePackStackPacket(pk) => pk.as_ref(),
                V2223::ResourcePacksInfoPacket(pk) => pk.as_ref(),
                V2223::ResourcePacksReadyForValidationPacket(pk) => pk.as_ref(),
                V2223::RespawnPacket(pk) => pk.as_ref(),
                V2223::ScriptMessagePacket(pk) => pk.as_ref(),
                V2223::SendPartyDestinationCookiePacket(pk) => pk.as_ref(),
                V2223::ServerBoundDataDrivenClosedPacket(pk) => pk.as_ref(),
                V2223::ServerBoundDataStorePacket(pk) => pk.as_ref(),
                V2223::ServerBoundDiagnosticsPacket(pk) => pk.as_ref(),
                V2223::ServerBoundLoadingScreenPacket(pk) => pk.as_ref(),
                V2223::ServerBoundPackSettingChangePacket(pk) => pk.as_ref(),
                V2223::ServerPlayerPostMovePositionPacket(pk) => pk.as_ref(),
                V2223::ServerPresenceInfoPacket(pk) => pk.as_ref(),
                V2223::ServerSettingsRequestPacket(pk) => pk.as_ref(),
                V2223::ServerSettingsResponsePacket(pk) => pk.as_ref(),
                V2223::ServerStatsPacket(pk) => pk.as_ref(),
                V2223::ServerStoreInfoPacket(pk) => pk.as_ref(),
                V2223::ServerToClientHandshakePacket(pk) => pk.as_ref(),
                V2223::SetActorDataPacket(pk) => pk.as_ref(),
                V2223::SetActorLinkPacket(pk) => pk.as_ref(),
                V2223::SetActorMotionPacket(pk) => pk.as_ref(),
                V2223::SetCommandsEnabledPacket(pk) => pk.as_ref(),
                V2223::SetDefaultGameTypePacket(pk) => pk.as_ref(),
                V2223::SetDifficultyPacket(pk) => pk.as_ref(),
                V2223::SetDisplayObjectivePacket(pk) => pk.as_ref(),
                V2223::SetHealthPacket(pk) => pk.as_ref(),
                V2223::SetHudPacket(pk) => pk.as_ref(),
                V2223::SetLastHurtByPacket(pk) => pk.as_ref(),
                V2223::SetLocalPlayerAsInitializedPacket(pk) => pk.as_ref(),
                V2223::SetPlayerFurnaceOptionsPacket(pk) => pk.as_ref(),
                V2223::SetPlayerGameTypePacket(pk) => pk.as_ref(),
                V2223::SetPlayerInventoryOptionsPacket(pk) => pk.as_ref(),
                V2223::SetScorePacket(pk) => pk.as_ref(),
                V2223::SetScoreboardIdentityPacket(pk) => pk.as_ref(),
                V2223::SetSpawnPositionPacket(pk) => pk.as_ref(),
                V2223::SetTimePacket(pk) => pk.as_ref(),
                V2223::SetTitlePacket(pk) => pk.as_ref(),
                V2223::SettingsCommandPacket(pk) => pk.as_ref(),
                V2223::ShowCreditsPacket(pk) => pk.as_ref(),
                V2223::ShowProfilePacket(pk) => pk.as_ref(),
                V2223::ShowStoreOfferPacket(pk) => pk.as_ref(),
                V2223::SimpleEventPacket(pk) => pk.as_ref(),
                V2223::SimulationTypePacket(pk) => pk.as_ref(),
                V2223::SpawnExperienceOrbPacket(pk) => pk.as_ref(),
                V2223::SpawnParticleEffectPacket(pk) => pk.as_ref(),
                V2223::StartGamePacket(pk) => pk.as_ref(),
                V2223::StopSoundPacket(pk) => pk.as_ref(),
                V2223::StructureBlockUpdatePacket(pk) => pk.as_ref(),
                V2223::StructureDataRequestPacket(pk) => pk.as_ref(),
                V2223::StructureDataResponsePacket(pk) => pk.as_ref(),
                V2223::SubChunkPacket(pk) => pk.as_ref(),
                V2223::SubChunkRequestPacket(pk) => pk.as_ref(),
                V2223::SubClientLoginPacket(pk) => pk.as_ref(),
                V2223::SyncActorPropertyPacket(pk) => pk.as_ref(),
                V2223::SyncWorldClocksPacket(pk) => pk.as_ref(),
                V2223::TakeItemActorPacket(pk) => pk.as_ref(),
                V2223::TextPacket(pk) => pk.as_ref(),
                V2223::TickingAreaLoadStatusPacket(pk) => pk.as_ref(),
                V2223::ToastRequestPacket(pk) => pk.as_ref(),
                V2223::TransferPlayerPacket(pk) => pk.as_ref(),
                V2223::TrimDataPacket(pk) => pk.as_ref(),
                V2223::UnlockedRecipesPacket(pk) => pk.as_ref(),
                V2223::UpdateAbilitiesPacket(pk) => pk.as_ref(),
                V2223::UpdateAdventureSettingsPacket(pk) => pk.as_ref(),
                V2223::UpdateAttributesPacket(pk) => pk.as_ref(),
                V2223::UpdateBlockPacket(pk) => pk.as_ref(),
                V2223::UpdateBlockSyncedPacket(pk) => pk.as_ref(),
                V2223::UpdateClientInputLocksPacket(pk) => pk.as_ref(),
                V2223::UpdateClientOptionsPacket(pk) => pk.as_ref(),
                V2223::UpdateEquipPacket(pk) => pk.as_ref(),
                V2223::UpdatePlayerGameTypePacket(pk) => pk.as_ref(),
                V2223::UpdateSoftEnumPacket(pk) => pk.as_ref(),
                V2223::UpdateSubChunkBlocksPacket(pk) => pk.as_ref(),
                V2223::UpdateTradePacket(pk) => pk.as_ref(),
                V2223::VoxelShapesPacket(pk) => pk.as_ref(),
                V2223::Unknown(pk) => pk.as_ref(),
            }
        }
    }
    #[cfg(feature = "packet-dyn")]
    impl From<V2223> for Box<dyn bedrock_protocol_core::PacketDyn> {
        fn from(val: V2223) -> Box<dyn bedrock_protocol_core::PacketDyn> {
            match val {
                V2223::ActorEventPacket(pk) => pk,
                V2223::ActorPickRequestPacket(pk) => pk,
                V2223::AddActorPacket(pk) => pk,
                V2223::AddBehaviourTreePacket(pk) => pk,
                V2223::AddItemActorPacket(pk) => pk,
                V2223::AddPaintingPacket(pk) => pk,
                V2223::AddPlayerPacket(pk) => pk,
                V2223::AddVolumeEntityPacket(pk) => pk,
                V2223::AgentActionEventPacket(pk) => pk,
                V2223::AgentAnimationPacket(pk) => pk,
                V2223::AnimateEntityPacket(pk) => pk,
                V2223::AnimatePacket(pk) => pk,
                V2223::AnvilDamagePacket(pk) => pk,
                V2223::AutomationClientConnectPacket(pk) => pk,
                V2223::AvailableActorIdentifiersPacket(pk) => pk,
                V2223::AvailableCommandsPacket(pk) => pk,
                V2223::AwardAchievementPacket(pk) => pk,
                V2223::BiomeDefinitionListPacket(pk) => pk,
                V2223::BlockActorDataPacket(pk) => pk,
                V2223::BlockEventPacket(pk) => pk,
                V2223::BlockPickRequestPacket(pk) => pk,
                V2223::BookEditPacket(pk) => pk,
                V2223::BossEventPacket(pk) => pk,
                V2223::CameraAimAssistActorPriorityPacket(pk) => pk,
                V2223::CameraAimAssistInstructionPacket(pk) => pk,
                V2223::CameraAimAssistPacket(pk) => pk,
                V2223::CameraAimAssistPresetsPacket(pk) => pk,
                V2223::CameraInstructionPacket(pk) => pk,
                V2223::CameraPacket(pk) => pk,
                V2223::CameraPresetsPacket(pk) => pk,
                V2223::CameraShakePacket(pk) => pk,
                V2223::CameraSplinePacket(pk) => pk,
                V2223::ChangeDimensionPacket(pk) => pk,
                V2223::ChangeMobPropertyPacket(pk) => pk,
                V2223::ChunkRadiusUpdatedPacket(pk) => pk,
                V2223::ClientBoundAttributeLayerSyncPacket(pk) => pk,
                V2223::ClientBoundCloseFormPacket(pk) => pk,
                V2223::ClientBoundControlSchemeSetPacket(pk) => pk,
                V2223::ClientBoundDataDrivenUICloseScreenPacket(pk) => pk,
                V2223::ClientBoundDataDrivenUIReloadPacket(pk) => pk,
                V2223::ClientBoundDataDrivenUIShowScreenPacket(pk) => pk,
                V2223::ClientBoundDataStorePacket(pk) => pk,
                V2223::ClientBoundDebugRendererPacket(pk) => pk,
                V2223::ClientBoundMapItemDataPacket(pk) => pk,
                V2223::ClientBoundTextureShiftPacket(pk) => pk,
                V2223::ClientBoundUpdateSoundDataPacket(pk) => pk,
                V2223::ClientCacheBlobStatusPacket(pk) => pk,
                V2223::ClientCacheMissResponsePacket(pk) => pk,
                V2223::ClientCacheStatusPacket(pk) => pk,
                V2223::ClientToServerHandshakePacket(pk) => pk,
                V2223::CodeBuilderPacket(pk) => pk,
                V2223::CodeBuilderSourcePacket(pk) => pk,
                V2223::CommandBlockUpdatePacket(pk) => pk,
                V2223::CommandOutputPacket(pk) => pk,
                V2223::CommandRequestPacket(pk) => pk,
                V2223::CompletedUsingItemPacket(pk) => pk,
                V2223::ContainerClosePacket(pk) => pk,
                V2223::ContainerOpenPacket(pk) => pk,
                V2223::ContainerRegistryCleanupPacket(pk) => pk,
                V2223::ContainerSetDataPacket(pk) => pk,
                V2223::CorrectPlayerMovePredictionPacket(pk) => pk,
                V2223::CraftingDataPacket(pk) => pk,
                V2223::CreatePhotoPacket(pk) => pk,
                V2223::CreativeContentPacket(pk) => pk,
                V2223::CurrentStructureFeaturePacket(pk) => pk,
                V2223::DeathInfoPacket(pk) => pk,
                V2223::DebugDrawerPacket(pk) => pk,
                V2223::DebugInfoPacket(pk) => pk,
                V2223::DimensionDataPacket(pk) => pk,
                V2223::DisconnectPacket(pk) => pk,
                V2223::EditorNetworkPacket(pk) => pk,
                V2223::EduUriResourcePacket(pk) => pk,
                V2223::EducationSettingsPacket(pk) => pk,
                V2223::EmoteListPacket(pk) => pk,
                V2223::EmotePacket(pk) => pk,
                V2223::FeatureRegistryPacket(pk) => pk,
                V2223::GameRulesChangedPacket(pk) => pk,
                V2223::GameTestRequestPacket(pk) => pk,
                V2223::GameTestResultsPacket(pk) => pk,
                V2223::GraphicsParameterOverridePacket(pk) => pk,
                V2223::GuiDataPickItemPacket(pk) => pk,
                V2223::HurtArmorPacket(pk) => pk,
                V2223::InteractPacket(pk) => pk,
                V2223::InventoryContentPacket(pk) => pk,
                V2223::InventorySlotPacket(pk) => pk,
                V2223::InventoryTransactionPacket(pk) => pk,
                V2223::ItemComponentPacket(pk) => pk,
                V2223::ItemStackRequestPacket(pk) => pk,
                V2223::ItemStackResponsePacket(pk) => pk,
                V2223::JigsawStructureDataPacket(pk) => pk,
                V2223::LabTablePacket(pk) => pk,
                V2223::LecternUpdatePacket(pk) => pk,
                V2223::LegacyTelemetryEventPacket(pk) => pk,
                V2223::LessonProgressPacket(pk) => pk,
                V2223::LevelChunkPacket(pk) => pk,
                V2223::LevelEventGenericPacket(pk) => pk,
                V2223::LevelEventPacket(pk) => pk,
                V2223::LevelSoundEventPacket(pk) => pk,
                V2223::LocatorBarPacket(pk) => pk,
                V2223::LoginPacket(pk) => pk,
                V2223::MapCreateLockedCopyPacket(pk) => pk,
                V2223::MapInfoRequestPacket(pk) => pk,
                V2223::MobArmorEquipmentPacket(pk) => pk,
                V2223::MobEffectPacket(pk) => pk,
                V2223::MobEquipmentPacket(pk) => pk,
                V2223::ModalFormRequestPacket(pk) => pk,
                V2223::ModalFormResponsePacket(pk) => pk,
                V2223::MotionPredictionHintsPacket(pk) => pk,
                V2223::MoveActorAbsolutePacket(pk) => pk,
                V2223::MoveActorDeltaPacket(pk) => pk,
                V2223::MovePlayerPacket(pk) => pk,
                V2223::MovementEffectPacket(pk) => pk,
                V2223::MovementPredictionSyncPacket(pk) => pk,
                V2223::MultiplayerSettingsPacket(pk) => pk,
                V2223::NetworkChunkPublisherUpdatePacket(pk) => pk,
                V2223::NetworkSettingsPacket(pk) => pk,
                V2223::NetworkStackLatencyPacket(pk) => pk,
                V2223::NpcDialoguePacket(pk) => pk,
                V2223::NpcRequestPacket(pk) => pk,
                V2223::OnScreenTextureAnimationPacket(pk) => pk,
                V2223::OpenSignPacket(pk) => pk,
                V2223::PacketViolationWarningPacket(pk) => pk,
                V2223::PartyChangedPacket(pk) => pk,
                V2223::PartyDestinationCookieResponsePacket(pk) => pk,
                V2223::PhotoTransferPacket(pk) => pk,
                V2223::PlaySoundPacket(pk) => pk,
                V2223::PlayStatusPacket(pk) => pk,
                V2223::PlayerActionPacket(pk) => pk,
                V2223::PlayerArmorDamagePacket(pk) => pk,
                V2223::PlayerAuthInputPacket(pk) => pk,
                V2223::PlayerEnchantOptionsPacket(pk) => pk,
                V2223::PlayerFogPacket(pk) => pk,
                V2223::PlayerHotbarPacket(pk) => pk,
                V2223::PlayerListPacket(pk) => pk,
                V2223::PlayerLocationPacket(pk) => pk,
                V2223::PlayerSkinPacket(pk) => pk,
                V2223::PlayerStartItemCooldownPacket(pk) => pk,
                V2223::PlayerToggleCrafterSlotRequestPacket(pk) => pk,
                V2223::PlayerUpdateEntityOverridesPacket(pk) => pk,
                V2223::PlayerVideoCapturePacket(pk) => pk,
                V2223::PositionTrackingDBClientRequestPacket(pk) => pk,
                V2223::PositionTrackingDBServerBroadcastPacket(pk) => pk,
                V2223::PurchaseReceiptPacket(pk) => pk,
                V2223::RecordStartedPacket(pk) => pk,
                V2223::RefreshEntitlementsPacket(pk) => pk,
                V2223::RemoveActorPacket(pk) => pk,
                V2223::RemoveObjectivePacket(pk) => pk,
                V2223::RemoveVolumeEntityPacket(pk) => pk,
                V2223::RequestAbilityPacket(pk) => pk,
                V2223::RequestChunkRadiusPacket(pk) => pk,
                V2223::RequestNetworkSettingsPacket(pk) => pk,
                V2223::RequestPermissionsPacket(pk) => pk,
                V2223::ResourcePackChunkDataPacket(pk) => pk,
                V2223::ResourcePackChunkRequestPacket(pk) => pk,
                V2223::ResourcePackClientResponsePacket(pk) => pk,
                V2223::ResourcePackDataInfoPacket(pk) => pk,
                V2223::ResourcePackStackPacket(pk) => pk,
                V2223::ResourcePacksInfoPacket(pk) => pk,
                V2223::ResourcePacksReadyForValidationPacket(pk) => pk,
                V2223::RespawnPacket(pk) => pk,
                V2223::ScriptMessagePacket(pk) => pk,
                V2223::SendPartyDestinationCookiePacket(pk) => pk,
                V2223::ServerBoundDataDrivenClosedPacket(pk) => pk,
                V2223::ServerBoundDataStorePacket(pk) => pk,
                V2223::ServerBoundDiagnosticsPacket(pk) => pk,
                V2223::ServerBoundLoadingScreenPacket(pk) => pk,
                V2223::ServerBoundPackSettingChangePacket(pk) => pk,
                V2223::ServerPlayerPostMovePositionPacket(pk) => pk,
                V2223::ServerPresenceInfoPacket(pk) => pk,
                V2223::ServerSettingsRequestPacket(pk) => pk,
                V2223::ServerSettingsResponsePacket(pk) => pk,
                V2223::ServerStatsPacket(pk) => pk,
                V2223::ServerStoreInfoPacket(pk) => pk,
                V2223::ServerToClientHandshakePacket(pk) => pk,
                V2223::SetActorDataPacket(pk) => pk,
                V2223::SetActorLinkPacket(pk) => pk,
                V2223::SetActorMotionPacket(pk) => pk,
                V2223::SetCommandsEnabledPacket(pk) => pk,
                V2223::SetDefaultGameTypePacket(pk) => pk,
                V2223::SetDifficultyPacket(pk) => pk,
                V2223::SetDisplayObjectivePacket(pk) => pk,
                V2223::SetHealthPacket(pk) => pk,
                V2223::SetHudPacket(pk) => pk,
                V2223::SetLastHurtByPacket(pk) => pk,
                V2223::SetLocalPlayerAsInitializedPacket(pk) => pk,
                V2223::SetPlayerFurnaceOptionsPacket(pk) => pk,
                V2223::SetPlayerGameTypePacket(pk) => pk,
                V2223::SetPlayerInventoryOptionsPacket(pk) => pk,
                V2223::SetScorePacket(pk) => pk,
                V2223::SetScoreboardIdentityPacket(pk) => pk,
                V2223::SetSpawnPositionPacket(pk) => pk,
                V2223::SetTimePacket(pk) => pk,
                V2223::SetTitlePacket(pk) => pk,
                V2223::SettingsCommandPacket(pk) => pk,
                V2223::ShowCreditsPacket(pk) => pk,
                V2223::ShowProfilePacket(pk) => pk,
                V2223::ShowStoreOfferPacket(pk) => pk,
                V2223::SimpleEventPacket(pk) => pk,
                V2223::SimulationTypePacket(pk) => pk,
                V2223::SpawnExperienceOrbPacket(pk) => pk,
                V2223::SpawnParticleEffectPacket(pk) => pk,
                V2223::StartGamePacket(pk) => pk,
                V2223::StopSoundPacket(pk) => pk,
                V2223::StructureBlockUpdatePacket(pk) => pk,
                V2223::StructureDataRequestPacket(pk) => pk,
                V2223::StructureDataResponsePacket(pk) => pk,
                V2223::SubChunkPacket(pk) => pk,
                V2223::SubChunkRequestPacket(pk) => pk,
                V2223::SubClientLoginPacket(pk) => pk,
                V2223::SyncActorPropertyPacket(pk) => pk,
                V2223::SyncWorldClocksPacket(pk) => pk,
                V2223::TakeItemActorPacket(pk) => pk,
                V2223::TextPacket(pk) => pk,
                V2223::TickingAreaLoadStatusPacket(pk) => pk,
                V2223::ToastRequestPacket(pk) => pk,
                V2223::TransferPlayerPacket(pk) => pk,
                V2223::TrimDataPacket(pk) => pk,
                V2223::UnlockedRecipesPacket(pk) => pk,
                V2223::UpdateAbilitiesPacket(pk) => pk,
                V2223::UpdateAdventureSettingsPacket(pk) => pk,
                V2223::UpdateAttributesPacket(pk) => pk,
                V2223::UpdateBlockPacket(pk) => pk,
                V2223::UpdateBlockSyncedPacket(pk) => pk,
                V2223::UpdateClientInputLocksPacket(pk) => pk,
                V2223::UpdateClientOptionsPacket(pk) => pk,
                V2223::UpdateEquipPacket(pk) => pk,
                V2223::UpdatePlayerGameTypePacket(pk) => pk,
                V2223::UpdateSoftEnumPacket(pk) => pk,
                V2223::UpdateSubChunkBlocksPacket(pk) => pk,
                V2223::UpdateTradePacket(pk) => pk,
                V2223::VoxelShapesPacket(pk) => pk,
                V2223::Unknown(pk) => pk,
            }
        }
    }
}
#[cfg(feature = "v2223")]
pub use inner::*;
