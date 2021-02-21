// This file is auto-generated do not edit manually.
use serde;
use serde::{Deserialize, Serialize};

pub mod accessibility;
pub mod animation;
pub mod application_cache;
pub mod audits;
pub mod background_service;
pub mod browser;
pub mod cache_storage;
pub mod cast;
pub mod console;
pub mod css;
pub mod database;
pub mod debugger;
pub mod device_orientation;
pub mod dom;
pub mod dom_debugger;
pub mod emulation;
pub mod fetch;
pub mod headless_experimental;
pub mod heap_profiler;
pub mod indexed_db;
pub mod input;
pub mod inspector;
pub mod io;
pub mod log;
pub mod media;
pub mod memory;
pub mod network;
pub mod overlay;
pub mod page;
pub mod performance;
pub mod profiler;
pub mod runtime;
pub mod schema;
pub mod security;
pub mod service_worker;
pub mod storage;
pub mod system_info;
pub mod target;
pub mod tethering;
pub mod tracing;
pub mod web_audio;
pub mod web_authn;

pub(crate) trait Command {
    const NAME: &'static str;

    type ReturnObject: serde::de::DeserializeOwned;

    fn to_command_call(self, call_id: u64) -> CommandCall<Self>
    where
        Self: std::marker::Sized,
    {
        CommandCall {
            id: call_id,
            params: self,
            method: Self::NAME,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CommandCall<T> {
    method: &'static str,
    pub id: u64,
    params: T,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "method")]
pub enum Event {
    #[serde(rename = "Console.messageAdded")]
    ConsoleMessageAdded(console::MessageAddedEvent),
    #[serde(rename = "Debugger.breakpointResolved")]
    DebuggerBreakpointResolved(debugger::BreakpointResolvedEvent),
    #[serde(rename = "Debugger.paused")]
    DebuggerPaused(debugger::PausedEvent),
    #[serde(rename = "Debugger.resumed")]
    DebuggerResumed(debugger::ResumedEvent),
    #[serde(rename = "Debugger.scriptFailedToParse")]
    DebuggerScriptFailedToParse(debugger::ScriptFailedToParseEvent),
    #[serde(rename = "Debugger.scriptParsed")]
    DebuggerScriptParsed(debugger::ScriptParsedEvent),
    #[serde(rename = "HeapProfiler.addHeapSnapshotChunk")]
    HeapProfilerAddHeapSnapshotChunk(heap_profiler::AddHeapSnapshotChunkEvent),
    #[serde(rename = "HeapProfiler.heapStatsUpdate")]
    HeapProfilerHeapStatsUpdate(heap_profiler::HeapStatsUpdateEvent),
    #[serde(rename = "HeapProfiler.lastSeenObjectId")]
    HeapProfilerLastSeenObjectId(heap_profiler::LastSeenObjectIdEvent),
    #[serde(rename = "HeapProfiler.reportHeapSnapshotProgress")]
    HeapProfilerReportHeapSnapshotProgress(heap_profiler::ReportHeapSnapshotProgressEvent),
    #[serde(rename = "HeapProfiler.resetProfiles")]
    HeapProfilerResetProfiles(heap_profiler::ResetProfilesEvent),
    #[serde(rename = "Profiler.consoleProfileFinished")]
    ProfilerConsoleProfileFinished(profiler::ConsoleProfileFinishedEvent),
    #[serde(rename = "Profiler.consoleProfileStarted")]
    ProfilerConsoleProfileStarted(profiler::ConsoleProfileStartedEvent),
    #[serde(rename = "Profiler.preciseCoverageDeltaUpdate")]
    ProfilerPreciseCoverageDeltaUpdate(profiler::PreciseCoverageDeltaUpdateEvent),
    #[serde(rename = "Runtime.bindingCalled")]
    RuntimeBindingCalled(runtime::BindingCalledEvent),
    #[serde(rename = "Runtime.consoleAPICalled")]
    RuntimeConsoleAPICalled(runtime::ConsoleAPICalledEvent),
    #[serde(rename = "Runtime.exceptionRevoked")]
    RuntimeExceptionRevoked(runtime::ExceptionRevokedEvent),
    #[serde(rename = "Runtime.exceptionThrown")]
    RuntimeExceptionThrown(runtime::ExceptionThrownEvent),
    #[serde(rename = "Runtime.executionContextCreated")]
    RuntimeExecutionContextCreated(runtime::ExecutionContextCreatedEvent),
    #[serde(rename = "Runtime.executionContextDestroyed")]
    RuntimeExecutionContextDestroyed(runtime::ExecutionContextDestroyedEvent),
    #[serde(rename = "Runtime.executionContextsCleared")]
    RuntimeExecutionContextsCleared(runtime::ExecutionContextsClearedEvent),
    #[serde(rename = "Runtime.inspectRequested")]
    RuntimeInspectRequested(runtime::InspectRequestedEvent),
    #[serde(rename = "Animation.animationCanceled")]
    AnimationAnimationCanceled(animation::AnimationCanceledEvent),
    #[serde(rename = "Animation.animationCreated")]
    AnimationAnimationCreated(animation::AnimationCreatedEvent),
    #[serde(rename = "Animation.animationStarted")]
    AnimationAnimationStarted(animation::AnimationStartedEvent),
    #[serde(rename = "ApplicationCache.applicationCacheStatusUpdated")]
    ApplicationCacheApplicationCacheStatusUpdated(
        application_cache::ApplicationCacheStatusUpdatedEvent,
    ),
    #[serde(rename = "ApplicationCache.networkStateUpdated")]
    ApplicationCacheNetworkStateUpdated(application_cache::NetworkStateUpdatedEvent),
    #[serde(rename = "Audits.issueAdded")]
    AuditsIssueAdded(audits::IssueAddedEvent),
    #[serde(rename = "BackgroundService.recordingStateChanged")]
    BackgroundServiceRecordingStateChanged(background_service::RecordingStateChangedEvent),
    #[serde(rename = "BackgroundService.backgroundServiceEventReceived")]
    BackgroundServiceBackgroundServiceEventReceived(
        background_service::BackgroundServiceEventReceivedEvent,
    ),
    #[serde(rename = "CSS.fontsUpdated")]
    CSSFontsUpdated(css::FontsUpdatedEvent),
    #[serde(rename = "CSS.mediaQueryResultChanged")]
    CSSMediaQueryResultChanged(css::MediaQueryResultChangedEvent),
    #[serde(rename = "CSS.styleSheetAdded")]
    CSSStyleSheetAdded(css::StyleSheetAddedEvent),
    #[serde(rename = "CSS.styleSheetChanged")]
    CSSStyleSheetChanged(css::StyleSheetChangedEvent),
    #[serde(rename = "CSS.styleSheetRemoved")]
    CSSStyleSheetRemoved(css::StyleSheetRemovedEvent),
    #[serde(rename = "Cast.sinksUpdated")]
    CastSinksUpdated(cast::SinksUpdatedEvent),
    #[serde(rename = "Cast.issueUpdated")]
    CastIssueUpdated(cast::IssueUpdatedEvent),
    #[serde(rename = "DOM.attributeModified")]
    DOMAttributeModified(dom::AttributeModifiedEvent),
    #[serde(rename = "DOM.attributeRemoved")]
    DOMAttributeRemoved(dom::AttributeRemovedEvent),
    #[serde(rename = "DOM.characterDataModified")]
    DOMCharacterDataModified(dom::CharacterDataModifiedEvent),
    #[serde(rename = "DOM.childNodeCountUpdated")]
    DOMChildNodeCountUpdated(dom::ChildNodeCountUpdatedEvent),
    #[serde(rename = "DOM.childNodeInserted")]
    DOMChildNodeInserted(dom::ChildNodeInsertedEvent),
    #[serde(rename = "DOM.childNodeRemoved")]
    DOMChildNodeRemoved(dom::ChildNodeRemovedEvent),
    #[serde(rename = "DOM.distributedNodesUpdated")]
    DOMDistributedNodesUpdated(dom::DistributedNodesUpdatedEvent),
    #[serde(rename = "DOM.documentUpdated")]
    DOMDocumentUpdated(dom::DocumentUpdatedEvent),
    #[serde(rename = "DOM.inlineStyleInvalidated")]
    DOMInlineStyleInvalidated(dom::InlineStyleInvalidatedEvent),
    #[serde(rename = "DOM.pseudoElementAdded")]
    DOMPseudoElementAdded(dom::PseudoElementAddedEvent),
    #[serde(rename = "DOM.pseudoElementRemoved")]
    DOMPseudoElementRemoved(dom::PseudoElementRemovedEvent),
    #[serde(rename = "DOM.setChildNodes")]
    DOMSetChildNodes(dom::SetChildNodesEvent),
    #[serde(rename = "DOM.shadowRootPopped")]
    DOMShadowRootPopped(dom::ShadowRootPoppedEvent),
    #[serde(rename = "DOM.shadowRootPushed")]
    DOMShadowRootPushed(dom::ShadowRootPushedEvent),
    #[serde(rename = "Database.addDatabase")]
    DatabaseAddDatabase(database::AddDatabaseEvent),
    #[serde(rename = "Emulation.virtualTimeBudgetExpired")]
    EmulationVirtualTimeBudgetExpired(emulation::VirtualTimeBudgetExpiredEvent),
    #[serde(rename = "HeadlessExperimental.needsBeginFramesChanged")]
    HeadlessExperimentalNeedsBeginFramesChanged(
        headless_experimental::NeedsBeginFramesChangedEvent,
    ),
    #[serde(rename = "Inspector.detached")]
    InspectorDetached(inspector::DetachedEvent),
    #[serde(rename = "Inspector.targetCrashed")]
    InspectorTargetCrashed(inspector::TargetCrashedEvent),
    #[serde(rename = "Inspector.targetReloadedAfterCrash")]
    InspectorTargetReloadedAfterCrash(inspector::TargetReloadedAfterCrashEvent),
    #[serde(rename = "Log.entryAdded")]
    LogEntryAdded(log::EntryAddedEvent),
    #[serde(rename = "Network.dataReceived")]
    NetworkDataReceived(network::DataReceivedEvent),
    #[serde(rename = "Network.eventSourceMessageReceived")]
    NetworkEventSourceMessageReceived(network::EventSourceMessageReceivedEvent),
    #[serde(rename = "Network.loadingFailed")]
    NetworkLoadingFailed(network::LoadingFailedEvent),
    #[serde(rename = "Network.loadingFinished")]
    NetworkLoadingFinished(network::LoadingFinishedEvent),
    #[serde(rename = "Network.requestIntercepted")]
    NetworkRequestIntercepted(network::RequestInterceptedEvent),
    #[serde(rename = "Network.requestServedFromCache")]
    NetworkRequestServedFromCache(network::RequestServedFromCacheEvent),
    #[serde(rename = "Network.requestWillBeSent")]
    NetworkRequestWillBeSent(network::RequestWillBeSentEvent),
    #[serde(rename = "Network.resourceChangedPriority")]
    NetworkResourceChangedPriority(network::ResourceChangedPriorityEvent),
    #[serde(rename = "Network.signedExchangeReceived")]
    NetworkSignedExchangeReceived(network::SignedExchangeReceivedEvent),
    #[serde(rename = "Network.responseReceived")]
    NetworkResponseReceived(network::ResponseReceivedEvent),
    #[serde(rename = "Network.webSocketClosed")]
    NetworkWebSocketClosed(network::WebSocketClosedEvent),
    #[serde(rename = "Network.webSocketCreated")]
    NetworkWebSocketCreated(network::WebSocketCreatedEvent),
    #[serde(rename = "Network.webSocketFrameError")]
    NetworkWebSocketFrameError(network::WebSocketFrameErrorEvent),
    #[serde(rename = "Network.webSocketFrameReceived")]
    NetworkWebSocketFrameReceived(network::WebSocketFrameReceivedEvent),
    #[serde(rename = "Network.webSocketFrameSent")]
    NetworkWebSocketFrameSent(network::WebSocketFrameSentEvent),
    #[serde(rename = "Network.webSocketHandshakeResponseReceived")]
    NetworkWebSocketHandshakeResponseReceived(network::WebSocketHandshakeResponseReceivedEvent),
    #[serde(rename = "Network.webSocketWillSendHandshakeRequest")]
    NetworkWebSocketWillSendHandshakeRequest(network::WebSocketWillSendHandshakeRequestEvent),
    #[serde(rename = "Network.requestWillBeSentExtraInfo")]
    NetworkRequestWillBeSentExtraInfo(network::RequestWillBeSentExtraInfoEvent),
    #[serde(rename = "Network.responseReceivedExtraInfo")]
    NetworkResponseReceivedExtraInfo(network::ResponseReceivedExtraInfoEvent),
    #[serde(rename = "Overlay.inspectNodeRequested")]
    OverlayInspectNodeRequested(overlay::InspectNodeRequestedEvent),
    #[serde(rename = "Overlay.nodeHighlightRequested")]
    OverlayNodeHighlightRequested(overlay::NodeHighlightRequestedEvent),
    #[serde(rename = "Overlay.screenshotRequested")]
    OverlayScreenshotRequested(overlay::ScreenshotRequestedEvent),
    #[serde(rename = "Overlay.inspectModeCanceled")]
    OverlayInspectModeCanceled(overlay::InspectModeCanceledEvent),
    #[serde(rename = "Page.domContentEventFired")]
    PageDomContentEventFired(page::DomContentEventFiredEvent),
    #[serde(rename = "Page.fileChooserOpened")]
    PageFileChooserOpened(page::FileChooserOpenedEvent),
    #[serde(rename = "Page.frameAttached")]
    PageFrameAttached(page::FrameAttachedEvent),
    #[serde(rename = "Page.frameClearedScheduledNavigation")]
    PageFrameClearedScheduledNavigation(page::FrameClearedScheduledNavigationEvent),
    #[serde(rename = "Page.frameDetached")]
    PageFrameDetached(page::FrameDetachedEvent),
    #[serde(rename = "Page.frameNavigated")]
    PageFrameNavigated(page::FrameNavigatedEvent),
    #[serde(rename = "Page.frameResized")]
    PageFrameResized(page::FrameResizedEvent),
    #[serde(rename = "Page.frameRequestedNavigation")]
    PageFrameRequestedNavigation(page::FrameRequestedNavigationEvent),
    #[serde(rename = "Page.frameScheduledNavigation")]
    PageFrameScheduledNavigation(page::FrameScheduledNavigationEvent),
    #[serde(rename = "Page.frameStartedLoading")]
    PageFrameStartedLoading(page::FrameStartedLoadingEvent),
    #[serde(rename = "Page.frameStoppedLoading")]
    PageFrameStoppedLoading(page::FrameStoppedLoadingEvent),
    #[serde(rename = "Page.downloadWillBegin")]
    PageDownloadWillBegin(page::DownloadWillBeginEvent),
    #[serde(rename = "Page.downloadProgress")]
    PageDownloadProgress(page::DownloadProgressEvent),
    #[serde(rename = "Page.interstitialHidden")]
    PageInterstitialHidden(page::InterstitialHiddenEvent),
    #[serde(rename = "Page.interstitialShown")]
    PageInterstitialShown(page::InterstitialShownEvent),
    #[serde(rename = "Page.javascriptDialogClosed")]
    PageJavascriptDialogClosed(page::JavascriptDialogClosedEvent),
    #[serde(rename = "Page.javascriptDialogOpening")]
    PageJavascriptDialogOpening(page::JavascriptDialogOpeningEvent),
    #[serde(rename = "Page.lifecycleEvent")]
    PageLifecycleEvent(page::LifecycleEventEvent),
    #[serde(rename = "Page.loadEventFired")]
    PageLoadEventFired(page::LoadEventFiredEvent),
    #[serde(rename = "Page.navigatedWithinDocument")]
    PageNavigatedWithinDocument(page::NavigatedWithinDocumentEvent),
    #[serde(rename = "Page.screencastFrame")]
    PageScreencastFrame(page::ScreencastFrameEvent),
    #[serde(rename = "Page.screencastVisibilityChanged")]
    PageScreencastVisibilityChanged(page::ScreencastVisibilityChangedEvent),
    #[serde(rename = "Page.windowOpen")]
    PageWindowOpen(page::WindowOpenEvent),
    #[serde(rename = "Page.compilationCacheProduced")]
    PageCompilationCacheProduced(page::CompilationCacheProducedEvent),
    #[serde(rename = "Performance.metrics")]
    PerformanceMetrics(performance::MetricsEvent),
    #[serde(rename = "Security.certificateError")]
    SecurityCertificateError(security::CertificateErrorEvent),
    #[serde(rename = "Security.visibleSecurityStateChanged")]
    SecurityVisibleSecurityStateChanged(security::VisibleSecurityStateChangedEvent),
    #[serde(rename = "Security.securityStateChanged")]
    SecuritySecurityStateChanged(security::SecurityStateChangedEvent),
    #[serde(rename = "ServiceWorker.workerErrorReported")]
    ServiceWorkerWorkerErrorReported(service_worker::WorkerErrorReportedEvent),
    #[serde(rename = "ServiceWorker.workerRegistrationUpdated")]
    ServiceWorkerWorkerRegistrationUpdated(service_worker::WorkerRegistrationUpdatedEvent),
    #[serde(rename = "ServiceWorker.workerVersionUpdated")]
    ServiceWorkerWorkerVersionUpdated(service_worker::WorkerVersionUpdatedEvent),
    #[serde(rename = "Storage.cacheStorageContentUpdated")]
    StorageCacheStorageContentUpdated(storage::CacheStorageContentUpdatedEvent),
    #[serde(rename = "Storage.cacheStorageListUpdated")]
    StorageCacheStorageListUpdated(storage::CacheStorageListUpdatedEvent),
    #[serde(rename = "Storage.indexedDBContentUpdated")]
    StorageIndexedDBContentUpdated(storage::IndexedDBContentUpdatedEvent),
    #[serde(rename = "Storage.indexedDBListUpdated")]
    StorageIndexedDBListUpdated(storage::IndexedDBListUpdatedEvent),
    #[serde(rename = "Target.attachedToTarget")]
    TargetAttachedToTarget(target::AttachedToTargetEvent),
    #[serde(rename = "Target.detachedFromTarget")]
    TargetDetachedFromTarget(target::DetachedFromTargetEvent),
    #[serde(rename = "Target.receivedMessageFromTarget")]
    TargetReceivedMessageFromTarget(target::ReceivedMessageFromTargetEvent),
    #[serde(rename = "Target.targetCreated")]
    TargetTargetCreated(target::TargetCreatedEvent),
    #[serde(rename = "Target.targetDestroyed")]
    TargetTargetDestroyed(target::TargetDestroyedEvent),
    #[serde(rename = "Target.targetCrashed")]
    TargetTargetCrashed(target::TargetCrashedEvent),
    #[serde(rename = "Target.targetInfoChanged")]
    TargetTargetInfoChanged(target::TargetInfoChangedEvent),
    #[serde(rename = "Tethering.accepted")]
    TetheringAccepted(tethering::AcceptedEvent),
    #[serde(rename = "Tracing.bufferUsage")]
    TracingBufferUsage(tracing::BufferUsageEvent),
    #[serde(rename = "Tracing.dataCollected")]
    TracingDataCollected(tracing::DataCollectedEvent),
    #[serde(rename = "Tracing.tracingComplete")]
    TracingTracingComplete(tracing::TracingCompleteEvent),
    #[serde(rename = "Fetch.requestPaused")]
    FetchRequestPaused(fetch::RequestPausedEvent),
    #[serde(rename = "Fetch.authRequired")]
    FetchAuthRequired(fetch::AuthRequiredEvent),
    #[serde(rename = "WebAudio.contextCreated")]
    WebAudioContextCreated(web_audio::ContextCreatedEvent),
    #[serde(rename = "WebAudio.contextWillBeDestroyed")]
    WebAudioContextWillBeDestroyed(web_audio::ContextWillBeDestroyedEvent),
    #[serde(rename = "WebAudio.contextChanged")]
    WebAudioContextChanged(web_audio::ContextChangedEvent),
    #[serde(rename = "WebAudio.audioListenerCreated")]
    WebAudioAudioListenerCreated(web_audio::AudioListenerCreatedEvent),
    #[serde(rename = "WebAudio.audioListenerWillBeDestroyed")]
    WebAudioAudioListenerWillBeDestroyed(web_audio::AudioListenerWillBeDestroyedEvent),
    #[serde(rename = "WebAudio.audioNodeCreated")]
    WebAudioAudioNodeCreated(web_audio::AudioNodeCreatedEvent),
    #[serde(rename = "WebAudio.audioNodeWillBeDestroyed")]
    WebAudioAudioNodeWillBeDestroyed(web_audio::AudioNodeWillBeDestroyedEvent),
    #[serde(rename = "WebAudio.audioParamCreated")]
    WebAudioAudioParamCreated(web_audio::AudioParamCreatedEvent),
    #[serde(rename = "WebAudio.audioParamWillBeDestroyed")]
    WebAudioAudioParamWillBeDestroyed(web_audio::AudioParamWillBeDestroyedEvent),
    #[serde(rename = "WebAudio.nodesConnected")]
    WebAudioNodesConnected(web_audio::NodesConnectedEvent),
    #[serde(rename = "WebAudio.nodesDisconnected")]
    WebAudioNodesDisconnected(web_audio::NodesDisconnectedEvent),
    #[serde(rename = "WebAudio.nodeParamConnected")]
    WebAudioNodeParamConnected(web_audio::NodeParamConnectedEvent),
    #[serde(rename = "WebAudio.nodeParamDisconnected")]
    WebAudioNodeParamDisconnected(web_audio::NodeParamDisconnectedEvent),
    #[serde(rename = "Media.playerPropertiesChanged")]
    MediaPlayerPropertiesChanged(media::PlayerPropertiesChangedEvent),
    #[serde(rename = "Media.playerEventsAdded")]
    MediaPlayerEventsAdded(media::PlayerEventsAddedEvent),
    #[serde(rename = "Media.playerMessagesLogged")]
    MediaPlayerMessagesLogged(media::PlayerMessagesLoggedEvent),
    #[serde(rename = "Media.playerErrorsRaised")]
    MediaPlayerErrorsRaised(media::PlayerErrorsRaisedEvent),
    #[serde(rename = "Media.playersCreated")]
    MediaPlayersCreated(media::PlayersCreatedEvent),
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct RemoteError {
    pub code: u64,
    pub message: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Response {
    #[serde(rename(deserialize = "id"))]
    pub call_id: u64,
    pub result: Option<serde_json::Value>,
    pub error: Option<RemoteError>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Message {
    Event(Event),
    Response(Response),
    ConnectionShutdown,
}
