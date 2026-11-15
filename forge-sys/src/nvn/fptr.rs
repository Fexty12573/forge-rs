use core::ffi::{c_char, c_uchar, c_void};

use crate::nvn::types::*;

pub type FnGenericFuncPtr = unsafe extern "C" fn();
pub type FnBootstrapLoader = unsafe extern "C" fn(*const c_char) -> Option<FnGenericFuncPtr>;
pub type FnCommandBufferMemoryCallback =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNcommandBufferMemoryEvent, usize, *mut c_void);
pub type FnDebugCallback = unsafe extern "C" fn(
    NVNdebugCallbackSource,
    NVNdebugCallbackType,
    i32,
    NVNdebugCallbackSeverity,
    *const c_uchar,
    *const c_void,
);
pub type FnWalkDebugDatabaseCallback = unsafe extern "C" fn(*mut c_void, *mut c_void);
pub type FnDeviceBuilderSetDefaults = unsafe extern "C" fn(*mut NVNdeviceBuilder);
pub type FnDeviceBuilderSetFlags = unsafe extern "C" fn(*mut NVNdeviceBuilder, NVNdeviceFlagBits_);
pub type FnDeviceInitialize = unsafe extern "C" fn(*mut NVNdevice, *const NVNdeviceBuilder) -> bool;
pub type FnDeviceFinalize = unsafe extern "C" fn(*mut NVNdevice);
pub type FnDeviceSetDebugLabel = unsafe extern "C" fn(*mut NVNdevice, *const c_char);
pub type FnDeviceGetProcAddress = unsafe extern "C" fn(*const NVNdevice, *const c_char) -> Option<FnGenericFuncPtr>;
pub type FnDeviceGetInteger = unsafe extern "C" fn(*const NVNdevice, NVNdeviceInfo, *mut i32);
pub type FnDeviceGetCurrentTimestampInNanoseconds = unsafe extern "C" fn(*const NVNdevice) -> u64;
pub type FnDeviceSetIntermediateShaderCache = unsafe extern "C" fn(*mut NVNdevice, i32);
pub type FnDeviceGetTextureHandle = unsafe extern "C" fn(*const NVNdevice, i32, i32) -> NVNtextureHandle;
pub type FnDeviceGetTexelFetchHandle = unsafe extern "C" fn(*const NVNdevice, i32) -> NVNtextureHandle;
pub type FnDeviceGetImageHandle = unsafe extern "C" fn(*const NVNdevice, i32) -> NVNimageHandle;
pub type FnDeviceInstallDebugCallback = unsafe extern "C" fn(*mut NVNdevice, FnDebugCallback, *mut c_void, bool);
pub type FnDeviceGenerateDebugDomainId = unsafe extern "C" fn(*const NVNdevice, *const c_char) -> i32;
pub type FnDeviceSetWindowOriginMode = unsafe extern "C" fn(*mut NVNdevice, NVNwindowOriginMode);
pub type FnDeviceSetDepthMode = unsafe extern "C" fn(*mut NVNdevice, NVNdepthMode);
pub type FnDeviceRegisterFastClearColor = unsafe extern "C" fn(*mut NVNdevice, *const f32, NVNformat) -> bool;
pub type FnDeviceRegisterFastClearColori = unsafe extern "C" fn(*mut NVNdevice, *const i32, NVNformat) -> bool;
pub type FnDeviceRegisterFastClearColorui = unsafe extern "C" fn(*mut NVNdevice, *const u32, NVNformat) -> bool;
pub type FnDeviceRegisterFastClearDepth = unsafe extern "C" fn(*mut NVNdevice, f32) -> bool;
pub type FnDeviceGetWindowOriginMode = unsafe extern "C" fn(*const NVNdevice) -> NVNwindowOriginMode;
pub type FnDeviceGetDepthMode = unsafe extern "C" fn(*const NVNdevice) -> NVNdepthMode;
pub type FnDeviceGetTimestampInNanoseconds = unsafe extern "C" fn(*const NVNdevice, *const NVNcounterData) -> u64;
pub type FnDeviceApplyDeferredFinalizes = unsafe extern "C" fn(*mut NVNdevice, i32);
pub type FnDeviceFinalizeCommandHandle = unsafe extern "C" fn(*mut NVNdevice, NVNcommandHandle);
pub type FnDeviceWalkDebugDatabase =
    unsafe extern "C" fn(*const NVNdevice, NVNdebugObjectType, FnWalkDebugDatabaseCallback, *mut c_void);
pub type FnDeviceGetSeparateTextureHandle = unsafe extern "C" fn(*const NVNdevice, i32) -> NVNseparateTextureHandle;
pub type FnDeviceGetSeparateSamplerHandle = unsafe extern "C" fn(*const NVNdevice, i32) -> NVNseparateSamplerHandle;
pub type FnDeviceIsExternalDebuggerAttached = unsafe extern "C" fn(*const NVNdevice) -> bool;

pub type FnQueueGetError = unsafe extern "C" fn(*mut NVNqueue, *mut NVNqueueErrorInfo) -> NVNqueueGetErrorResult;
pub type FnQueueGetTotalCommandMemoryUsed = unsafe extern "C" fn(*const NVNqueue) -> usize;
pub type FnQueueGetTotalControlMemoryUsed = unsafe extern "C" fn(*const NVNqueue) -> usize;
pub type FnQueueGetTotalComputeMemoryUsed = unsafe extern "C" fn(*const NVNqueue) -> usize;
pub type FnQueueResetMemoryUsageCounts = unsafe extern "C" fn(*mut NVNqueue);
pub type FnQueueBuilderSetDevice = unsafe extern "C" fn(*mut NVNqueueBuilder, *mut NVNdevice);
pub type FnQueueBuilderSetDefaults = unsafe extern "C" fn(*mut NVNqueueBuilder);
pub type FnQueueBuilderSetFlags = unsafe extern "C" fn(*mut NVNqueueBuilder, NVNqueueBuilderFlags_);
pub type FnQueueBuilderSetCommandMemorySize = unsafe extern "C" fn(*mut NVNqueueBuilder, usize);
pub type FnQueueBuilderSetControlMemorySize = unsafe extern "C" fn(*mut NVNqueueBuilder, usize);
pub type FnQueueBuilderSetComputeMemorySize = unsafe extern "C" fn(*mut NVNqueueBuilder, usize);
pub type FnQueueBuilderGetQueueMemorySize = unsafe extern "C" fn(*const NVNqueueBuilder) -> usize;
pub type FnQueueBuilderSetQueueMemory = unsafe extern "C" fn(*mut NVNqueueBuilder, *mut c_void, usize);
pub type FnQueueBuilderSetCommandFlushThreshold = unsafe extern "C" fn(*mut NVNqueueBuilder, usize);
pub type FnQueueBuilderGetDevice = unsafe extern "C" fn(*const NVNqueueBuilder) -> *const NVNdevice;
pub type FnQueueBuilderGetFlags = unsafe extern "C" fn(*const NVNqueueBuilder) -> NVNqueueBuilderFlags_;
pub type FnQueueBuilderGetCommandMemorySize = unsafe extern "C" fn(*const NVNqueueBuilder) -> usize;
pub type FnQueueInitialize = unsafe extern "C" fn(*mut NVNqueue, *const NVNqueueBuilder) -> bool;
pub type FnQueueFinalize = unsafe extern "C" fn(*mut NVNqueue);
pub type FnQueueSetDebugLabel = unsafe extern "C" fn(*mut NVNqueue, *const c_char);
pub type FnQueueSubmitCommands = unsafe extern "C" fn(*mut NVNqueue, i32, *const NVNcommandHandle);
pub type FnQueueFlush = unsafe extern "C" fn(*mut NVNqueue);
pub type FnQueueFinish = unsafe extern "C" fn(*mut NVNqueue);
pub type FnQueuePresentTexture = unsafe extern "C" fn(*mut NVNqueue, *mut NVNwindow, i32);
pub type FnQueueAcquireTexture = unsafe extern "C" fn(*mut NVNqueue, *mut NVNwindow, *mut i32) -> NVNqueueAcquireTextureResult;

pub type FnWindowBuilderSetDevice = unsafe extern "C" fn(*mut NVNwindowBuilder, *mut NVNdevice);
pub type FnWindowBuilderSetDefaults = unsafe extern "C" fn(*mut NVNwindowBuilder);
pub type FnWindowBuilderSetNativeWindow = unsafe extern "C" fn(*mut NVNwindowBuilder, NVNnativeWindow);
pub type FnWindowBuilderSetTextures = unsafe extern "C" fn(*mut NVNwindowBuilder, i32, *const *mut NVNtexture);
pub type FnWindowBuilderSetPresentInterval = unsafe extern "C" fn(*mut NVNwindowBuilder, i32);
pub type FnWindowBuilderGetNativeWindow = unsafe extern "C" fn(*const NVNwindowBuilder) -> NVNnativeWindow;
pub type FnWindowBuilderGetPresentInterval = unsafe extern "C" fn(*const NVNwindowBuilder) -> i32;
pub type FnWindowInitialize = unsafe extern "C" fn(*mut NVNwindow, *const NVNwindowBuilder) -> bool;
pub type FnWindowFinalize = unsafe extern "C" fn(*mut NVNwindow);
pub type FnWindowSetDebugLabel = unsafe extern "C" fn(*mut NVNwindow, *const c_char);
pub type FnWindowAcquireTexture = unsafe extern "C" fn(*mut NVNwindow, *mut NVNsync, *mut i32) -> NVNwindowAcquireTextureResult;
pub type FnWindowGetNativeWindow = unsafe extern "C" fn(*const NVNwindow) -> NVNnativeWindow;
pub type FnWindowGetPresentInterval = unsafe extern "C" fn(*const NVNwindow) -> i32;
pub type FnWindowSetPresentInterval = unsafe extern "C" fn(*mut NVNwindow, i32);
pub type FnWindowSetCrop = unsafe extern "C" fn(*mut NVNwindow, i32, i32, i32, i32);
pub type FnWindowGetCrop = unsafe extern "C" fn(*const NVNwindow, *mut NVNrectangle);

pub type FnProgramInitialize = unsafe extern "C" fn(*mut NVNprogram, *mut NVNdevice) -> bool;
pub type FnProgramFinalize = unsafe extern "C" fn(*mut NVNprogram);
pub type FnProgramSetDebugLabel = unsafe extern "C" fn(*mut NVNprogram, *const c_char);
pub type FnProgramSetShaders = unsafe extern "C" fn(*mut NVNprogram, i32, *const NVNshaderData) -> bool;

pub type FnMemoryPoolBuilderSetDevice = unsafe extern "C" fn(*mut NVNmemoryPoolBuilder, *mut NVNdevice);
pub type FnMemoryPoolBuilderSetDefaults = unsafe extern "C" fn(*mut NVNmemoryPoolBuilder);
pub type FnMemoryPoolBuilderSetStorage = unsafe extern "C" fn(*mut NVNmemoryPoolBuilder, *mut c_void, usize);
pub type FnMemoryPoolBuilderSetFlags = unsafe extern "C" fn(*mut NVNmemoryPoolBuilder, NVNmemoryPoolFlags_);
pub type FnMemoryPoolBuilderGetMemory = unsafe extern "C" fn(*const NVNmemoryPoolBuilder) -> *mut c_void;
pub type FnMemoryPoolBuilderGetSize = unsafe extern "C" fn(*const NVNmemoryPoolBuilder) -> usize;
pub type FnMemoryPoolBuilderGetFlags = unsafe extern "C" fn(*const NVNmemoryPoolBuilder) -> NVNmemoryPoolFlags_;
pub type FnMemoryPoolInitialize = unsafe extern "C" fn(*mut NVNmemoryPool, *const NVNmemoryPoolBuilder) -> bool;
pub type FnMemoryPoolSetDebugLabel = unsafe extern "C" fn(*mut NVNmemoryPool, *const c_char);
pub type FnMemoryPoolFinalize = unsafe extern "C" fn(*mut NVNmemoryPool);
pub type FnMemoryPoolMap = unsafe extern "C" fn(*const NVNmemoryPool) -> *mut c_void;
pub type FnMemoryPoolFlushMappedRange = unsafe extern "C" fn(*const NVNmemoryPool, isize, usize);
pub type FnMemoryPoolInvalidateMappedRange = unsafe extern "C" fn(*const NVNmemoryPool, isize, usize);
pub type FnMemoryPoolGetBufferAddress = unsafe extern "C" fn(*const NVNmemoryPool) -> NVNbufferAddress;
pub type FnMemoryPoolMapVirtual = unsafe extern "C" fn(*mut NVNmemoryPool, i32, *const NVNmappingRequest) -> bool;
pub type FnMemoryPoolGetSize = unsafe extern "C" fn(*const NVNmemoryPool) -> usize;
pub type FnMemoryPoolGetFlags = unsafe extern "C" fn(*const NVNmemoryPool) -> NVNmemoryPoolFlags_;

pub type FnTexturePoolInitialize = unsafe extern "C" fn(*mut NVNtexturePool, *const NVNmemoryPool, isize, i32) -> bool;
pub type FnTexturePoolSetDebugLabel = unsafe extern "C" fn(*mut NVNtexturePool, *const c_char);
pub type FnTexturePoolFinalize = unsafe extern "C" fn(*mut NVNtexturePool);
pub type FnTexturePoolRegisterTexture =
    unsafe extern "C" fn(*const NVNtexturePool, i32, *const NVNtexture, *const NVNtextureView);
pub type FnTexturePoolRegisterImage = unsafe extern "C" fn(*const NVNtexturePool, i32, *const NVNtexture, *const NVNtextureView);
pub type FnTexturePoolGetMemoryPool = unsafe extern "C" fn(*const NVNtexturePool) -> *const NVNmemoryPool;
pub type FnTexturePoolGetMemoryOffset = unsafe extern "C" fn(*const NVNtexturePool) -> isize;
pub type FnTexturePoolGetSize = unsafe extern "C" fn(*const NVNtexturePool) -> i32;

pub type FnSamplerPoolInitialize = unsafe extern "C" fn(*mut NVNsamplerPool, *const NVNmemoryPool, isize, i32) -> bool;
pub type FnSamplerPoolSetDebugLabel = unsafe extern "C" fn(*mut NVNsamplerPool, *const c_char);
pub type FnSamplerPoolFinalize = unsafe extern "C" fn(*mut NVNsamplerPool);
pub type FnSamplerPoolRegisterSampler = unsafe extern "C" fn(*const NVNsamplerPool, i32, *const NVNsampler);
pub type FnSamplerPoolRegisterSamplerBuilder = unsafe extern "C" fn(*const NVNsamplerPool, i32, *const NVNsamplerBuilder);
pub type FnSamplerPoolGetMemoryPool = unsafe extern "C" fn(*const NVNsamplerPool) -> *const NVNmemoryPool;
pub type FnSamplerPoolGetMemoryOffset = unsafe extern "C" fn(*const NVNsamplerPool) -> isize;
pub type FnSamplerPoolGetSize = unsafe extern "C" fn(*const NVNsamplerPool) -> i32;

pub type FnBufferBuilderSetDevice = unsafe extern "C" fn(*mut NVNbufferBuilder, *mut NVNdevice);
pub type FnBufferBuilderSetDefaults = unsafe extern "C" fn(*mut NVNbufferBuilder);
pub type FnBufferBuilderSetStorage = unsafe extern "C" fn(*mut NVNbufferBuilder, *mut NVNmemoryPool, isize, usize);
pub type FnBufferBuilderGetMemoryPool = unsafe extern "C" fn(*const NVNbufferBuilder) -> *mut NVNmemoryPool;
pub type FnBufferBuilderGetMemoryOffset = unsafe extern "C" fn(*const NVNbufferBuilder) -> isize;
pub type FnBufferBuilderGetSize = unsafe extern "C" fn(*const NVNbufferBuilder) -> usize;
pub type FnBufferInitialize = unsafe extern "C" fn(*mut NVNbuffer, *const NVNbufferBuilder) -> bool;
pub type FnBufferSetDebugLabel = unsafe extern "C" fn(*mut NVNbuffer, *const c_char);
pub type FnBufferFinalize = unsafe extern "C" fn(*mut NVNbuffer);
pub type FnBufferMap = unsafe extern "C" fn(*const NVNbuffer) -> *mut c_void;
pub type FnBufferGetAddress = unsafe extern "C" fn(*const NVNbuffer) -> NVNbufferAddress;
pub type FnBufferFlushMappedRange = unsafe extern "C" fn(*const NVNbuffer, isize, usize);
pub type FnBufferInvalidateMappedRange = unsafe extern "C" fn(*const NVNbuffer, isize, usize);
pub type FnBufferGetMemoryPool = unsafe extern "C" fn(*const NVNbuffer) -> *mut NVNmemoryPool;
pub type FnBufferGetMemoryOffset = unsafe extern "C" fn(*const NVNbuffer) -> isize;
pub type FnBufferGetSize = unsafe extern "C" fn(*const NVNbuffer) -> usize;
pub type FnBufferGetDebugID = unsafe extern "C" fn(*const NVNbuffer) -> u64;

pub type FnTextureBuilderSetDevice = unsafe extern "C" fn(*mut NVNtextureBuilder, *mut NVNdevice);
pub type FnTextureBuilderSetDefaults = unsafe extern "C" fn(*mut NVNtextureBuilder);
pub type FnTextureBuilderSetFlags = unsafe extern "C" fn(*mut NVNtextureBuilder, NVNtextureFlags_);
pub type FnTextureBuilderSetTarget = unsafe extern "C" fn(*mut NVNtextureBuilder, NVNtextureTarget);
pub type FnTextureBuilderSetWidth = unsafe extern "C" fn(*mut NVNtextureBuilder, i32);
pub type FnTextureBuilderSetHeight = unsafe extern "C" fn(*mut NVNtextureBuilder, i32);
pub type FnTextureBuilderSetDepth = unsafe extern "C" fn(*mut NVNtextureBuilder, i32);
pub type FnTextureBuilderSetSize1D = unsafe extern "C" fn(*mut NVNtextureBuilder, i32);
pub type FnTextureBuilderSetSize2D = unsafe extern "C" fn(*mut NVNtextureBuilder, i32, i32);
pub type FnTextureBuilderSetSize3D = unsafe extern "C" fn(*mut NVNtextureBuilder, i32, i32, i32);
pub type FnTextureBuilderSetLevels = unsafe extern "C" fn(*mut NVNtextureBuilder, i32);
pub type FnTextureBuilderSetFormat = unsafe extern "C" fn(*mut NVNtextureBuilder, NVNformat);
pub type FnTextureBuilderSetSamples = unsafe extern "C" fn(*mut NVNtextureBuilder, i32);
pub type FnTextureBuilderSetSwizzle =
    unsafe extern "C" fn(*mut NVNtextureBuilder, NVNtextureSwizzle, NVNtextureSwizzle, NVNtextureSwizzle, NVNtextureSwizzle);
pub type FnTextureBuilderSetDepthStencilMode = unsafe extern "C" fn(*mut NVNtextureBuilder, NVNtextureDepthStencilMode);
pub type FnTextureBuilderGetStorageSize = unsafe extern "C" fn(*const NVNtextureBuilder) -> usize;
pub type FnTextureBuilderGetStorageAlignment = unsafe extern "C" fn(*const NVNtextureBuilder) -> usize;
pub type FnTextureBuilderSetStorage = unsafe extern "C" fn(*mut NVNtextureBuilder, *mut NVNmemoryPool, isize);
pub type FnTextureBuilderSetPackagedTextureData = unsafe extern "C" fn(*mut NVNtextureBuilder, *const c_void);
pub type FnTextureBuilderSetPackagedTextureLayout = unsafe extern "C" fn(*mut NVNtextureBuilder, *const NVNpackagedTextureLayout);
pub type FnTextureBuilderSetStride = unsafe extern "C" fn(*mut NVNtextureBuilder, isize);
pub type FnTextureBuilderSetGLTextureName = unsafe extern "C" fn(*mut NVNtextureBuilder, u32);
pub type FnTextureBuilderGetStorageClass = unsafe extern "C" fn(*const NVNtextureBuilder) -> NVNstorageClass;
pub type FnTextureBuilderGetFlags = unsafe extern "C" fn(*const NVNtextureBuilder) -> NVNtextureFlags_;
pub type FnTextureBuilderGetTarget = unsafe extern "C" fn(*const NVNtextureBuilder) -> NVNtextureTarget;
pub type FnTextureBuilderGetWidth = unsafe extern "C" fn(*const NVNtextureBuilder) -> i32;
pub type FnTextureBuilderGetHeight = unsafe extern "C" fn(*const NVNtextureBuilder) -> i32;
pub type FnTextureBuilderGetDepth = unsafe extern "C" fn(*const NVNtextureBuilder) -> i32;
pub type FnTextureBuilderGetLevels = unsafe extern "C" fn(*const NVNtextureBuilder) -> i32;
pub type FnTextureBuilderGetFormat = unsafe extern "C" fn(*const NVNtextureBuilder) -> NVNformat;
pub type FnTextureBuilderGetSamples = unsafe extern "C" fn(*const NVNtextureBuilder) -> i32;
pub type FnTextureBuilderGetSwizzle = unsafe extern "C" fn(
    *const NVNtextureBuilder,
    *mut NVNtextureSwizzle,
    *mut NVNtextureSwizzle,
    *mut NVNtextureSwizzle,
    *mut NVNtextureSwizzle,
);
pub type FnTextureBuilderGetDepthStencilMode = unsafe extern "C" fn(*const NVNtextureBuilder) -> NVNtextureDepthStencilMode;
pub type FnTextureBuilderGetPackagedTextureData = unsafe extern "C" fn(*const NVNtextureBuilder) -> *const c_void;
pub type FnTextureBuilderGetStride = unsafe extern "C" fn(*const NVNtextureBuilder) -> isize;
pub type FnTextureBuilderGetSparseTileLayout = unsafe extern "C" fn(*const NVNtextureBuilder, *mut NVNtextureSparseTileLayout);
pub type FnTextureBuilderGetGLTextureName = unsafe extern "C" fn(*const NVNtextureBuilder) -> u32;
pub type FnTextureBuilderGetZCullStorageSize = unsafe extern "C" fn(*const NVNtextureBuilder) -> usize;
pub type FnTextureBuilderGetMemoryPool = unsafe extern "C" fn(*const NVNtextureBuilder) -> *mut NVNmemoryPool;
pub type FnTextureBuilderGetMemoryOffset = unsafe extern "C" fn(*const NVNtextureBuilder) -> isize;
pub type FnTextureViewSetDefaults = unsafe extern "C" fn(*mut NVNtextureView);
pub type FnTextureViewSetLevels = unsafe extern "C" fn(*mut NVNtextureView, i32, i32);
pub type FnTextureViewSetLayers = unsafe extern "C" fn(*mut NVNtextureView, i32, i32);
pub type FnTextureViewSetFormat = unsafe extern "C" fn(*mut NVNtextureView, NVNformat);
pub type FnTextureViewSetSwizzle =
    unsafe extern "C" fn(*mut NVNtextureView, NVNtextureSwizzle, NVNtextureSwizzle, NVNtextureSwizzle, NVNtextureSwizzle);
pub type FnTextureViewSetDepthStencilMode = unsafe extern "C" fn(*mut NVNtextureView, NVNtextureDepthStencilMode);
pub type FnTextureViewSetTarget = unsafe extern "C" fn(*mut NVNtextureView, NVNtextureTarget);
pub type FnTextureViewGetLevels = unsafe extern "C" fn(*const NVNtextureView, *mut i32, *mut i32) -> bool;
pub type FnTextureViewGetLayers = unsafe extern "C" fn(*const NVNtextureView, *mut i32, *mut i32) -> bool;
pub type FnTextureViewGetFormat = unsafe extern "C" fn(*const NVNtextureView, *mut NVNformat) -> bool;
pub type FnTextureViewGetSwizzle = unsafe extern "C" fn(
    *const NVNtextureView,
    *mut NVNtextureSwizzle,
    *mut NVNtextureSwizzle,
    *mut NVNtextureSwizzle,
    *mut NVNtextureSwizzle,
) -> bool;
pub type FnTextureViewGetDepthStencilMode = unsafe extern "C" fn(*const NVNtextureView, *mut NVNtextureDepthStencilMode) -> bool;
pub type FnTextureViewGetTarget = unsafe extern "C" fn(*const NVNtextureView, *mut NVNtextureTarget) -> bool;
pub type FnTextureViewCompare = unsafe extern "C" fn(*const NVNtextureView, *const NVNtextureView) -> bool;
pub type FnTextureInitialize = unsafe extern "C" fn(*mut NVNtexture, *const NVNtextureBuilder) -> bool;
pub type FnTextureGetZCullStorageSize = unsafe extern "C" fn(*const NVNtexture) -> usize;
pub type FnTextureFinalize = unsafe extern "C" fn(*mut NVNtexture);
pub type FnTextureSetDebugLabel = unsafe extern "C" fn(*mut NVNtexture, *const c_char);
pub type FnTextureGetStorageClass = unsafe extern "C" fn(*const NVNtexture) -> NVNstorageClass;
pub type FnTextureGetViewOffset = unsafe extern "C" fn(*const NVNtexture, *const NVNtextureView) -> isize;
pub type FnTextureGetFlags = unsafe extern "C" fn(*const NVNtexture) -> NVNtextureFlags_;
pub type FnTextureGetTarget = unsafe extern "C" fn(*const NVNtexture) -> NVNtextureTarget;
pub type FnTextureGetWidth = unsafe extern "C" fn(*const NVNtexture) -> i32;
pub type FnTextureGetHeight = unsafe extern "C" fn(*const NVNtexture) -> i32;
pub type FnTextureGetDepth = unsafe extern "C" fn(*const NVNtexture) -> i32;
pub type FnTextureGetLevels = unsafe extern "C" fn(*const NVNtexture) -> i32;
pub type FnTextureGetFormat = unsafe extern "C" fn(*const NVNtexture) -> NVNformat;
pub type FnTextureGetSamples = unsafe extern "C" fn(*const NVNtexture) -> i32;
pub type FnTextureGetSwizzle = unsafe extern "C" fn(
    *const NVNtexture,
    *mut NVNtextureSwizzle,
    *mut NVNtextureSwizzle,
    *mut NVNtextureSwizzle,
    *mut NVNtextureSwizzle,
);
pub type FnTextureGetDepthStencilMode = unsafe extern "C" fn(*const NVNtexture) -> NVNtextureDepthStencilMode;
pub type FnTextureGetStride = unsafe extern "C" fn(*const NVNtexture) -> isize;
pub type FnTextureGetTextureAddress = unsafe extern "C" fn(*const NVNtexture) -> NVNtextureAddress;
pub type FnTextureGetSparseTileLayout = unsafe extern "C" fn(*const NVNtexture, *mut NVNtextureSparseTileLayout);
pub type FnTextureWriteTexels =
    unsafe extern "C" fn(*const NVNtexture, *const NVNtextureView, *const NVNcopyRegion, *const c_void);
pub type FnTextureWriteTexelsStrided =
    unsafe extern "C" fn(*const NVNtexture, *const NVNtextureView, *const NVNcopyRegion, *const c_void, isize, isize);
pub type FnTextureReadTexels = unsafe extern "C" fn(*const NVNtexture, *const NVNtextureView, *const NVNcopyRegion, *mut c_void);
pub type FnTextureReadTexelsStrided =
    unsafe extern "C" fn(*const NVNtexture, *const NVNtextureView, *const NVNcopyRegion, *mut c_void, isize, isize);
pub type FnTextureFlushTexels = unsafe extern "C" fn(*const NVNtexture, *const NVNtextureView, *const NVNcopyRegion);
pub type FnTextureInvalidateTexels = unsafe extern "C" fn(*const NVNtexture, *const NVNtextureView, *const NVNcopyRegion);
pub type FnTextureGetMemoryPool = unsafe extern "C" fn(*const NVNtexture) -> *mut NVNmemoryPool;
pub type FnTextureGetMemoryOffset = unsafe extern "C" fn(*const NVNtexture) -> isize;
pub type FnTextureGetStorageSize = unsafe extern "C" fn(*const NVNtexture) -> i32;
pub type FnTextureCompare = unsafe extern "C" fn(*const NVNtexture, *const NVNtexture) -> bool;
pub type FnTextureGetDebugID = unsafe extern "C" fn(*const NVNtexture) -> u64;

pub type FnSamplerBuilderSetDevice = unsafe extern "C" fn(*mut NVNsamplerBuilder, *mut NVNdevice);
pub type FnSamplerBuilderSetDefaults = unsafe extern "C" fn(*mut NVNsamplerBuilder);
pub type FnSamplerBuilderSetMinMagFilter = unsafe extern "C" fn(*mut NVNsamplerBuilder, NVNminFilter, NVNmagFilter);
pub type FnSamplerBuilderSetWrapMode = unsafe extern "C" fn(*mut NVNsamplerBuilder, NVNwrapMode, NVNwrapMode, NVNwrapMode);
pub type FnSamplerBuilderSetLodClamp = unsafe extern "C" fn(*mut NVNsamplerBuilder, f32, f32);
pub type FnSamplerBuilderSetLodBias = unsafe extern "C" fn(*mut NVNsamplerBuilder, f32);
pub type FnSamplerBuilderSetCompare = unsafe extern "C" fn(*mut NVNsamplerBuilder, NVNcompareMode, NVNcompareFunc);
pub type FnSamplerBuilderSetBorderColor = unsafe extern "C" fn(*mut NVNsamplerBuilder, *const f32);
pub type FnSamplerBuilderSetBorderColori = unsafe extern "C" fn(*mut NVNsamplerBuilder, *const i32);
pub type FnSamplerBuilderSetBorderColorui = unsafe extern "C" fn(*mut NVNsamplerBuilder, *const u32);
pub type FnSamplerBuilderSetMaxAnisotropy = unsafe extern "C" fn(*mut NVNsamplerBuilder, f32);
pub type FnSamplerBuilderSetReductionFilter = unsafe extern "C" fn(*mut NVNsamplerBuilder, NVNsamplerReduction);
pub type FnSamplerBuilderSetLodSnap = unsafe extern "C" fn(*mut NVNsamplerBuilder, f32);
pub type FnSamplerBuilderGetMinMagFilter = unsafe extern "C" fn(*const NVNsamplerBuilder, *mut NVNminFilter, *mut NVNmagFilter);
pub type FnSamplerBuilderGetWrapMode =
    unsafe extern "C" fn(*const NVNsamplerBuilder, *mut NVNwrapMode, *mut NVNwrapMode, *mut NVNwrapMode);
pub type FnSamplerBuilderGetLodClamp = unsafe extern "C" fn(*const NVNsamplerBuilder, *mut f32, *mut f32);
pub type FnSamplerBuilderGetLodBias = unsafe extern "C" fn(*const NVNsamplerBuilder) -> f32;
pub type FnSamplerBuilderGetCompare = unsafe extern "C" fn(*const NVNsamplerBuilder, *mut NVNcompareMode, *mut NVNcompareFunc);
pub type FnSamplerBuilderGetBorderColor = unsafe extern "C" fn(*const NVNsamplerBuilder, *mut f32);
pub type FnSamplerBuilderGetBorderColori = unsafe extern "C" fn(*const NVNsamplerBuilder, *mut i32);
pub type FnSamplerBuilderGetBorderColorui = unsafe extern "C" fn(*const NVNsamplerBuilder, *mut u32);
pub type FnSamplerBuilderGetMaxAnisotropy = unsafe extern "C" fn(*const NVNsamplerBuilder) -> f32;
pub type FnSamplerBuilderGetReductionFilter = unsafe extern "C" fn(*const NVNsamplerBuilder) -> NVNsamplerReduction;
pub type FnSamplerBuilderGetLodSnap = unsafe extern "C" fn(*const NVNsamplerBuilder) -> f32;
pub type FnSamplerInitialize = unsafe extern "C" fn(*mut NVNsampler, *const NVNsamplerBuilder) -> bool;
pub type FnSamplerFinalize = unsafe extern "C" fn(*mut NVNsampler);
pub type FnSamplerSetDebugLabel = unsafe extern "C" fn(*mut NVNsampler, *const c_char);
pub type FnSamplerGetMinMagFilter = unsafe extern "C" fn(*const NVNsampler, *mut NVNminFilter, *mut NVNmagFilter);
pub type FnSamplerGetWrapMode = unsafe extern "C" fn(*const NVNsampler, *mut NVNwrapMode, *mut NVNwrapMode, *mut NVNwrapMode);
pub type FnSamplerGetLodClamp = unsafe extern "C" fn(*const NVNsampler, *mut f32, *mut f32);
pub type FnSamplerGetLodBias = unsafe extern "C" fn(*const NVNsampler) -> f32;
pub type FnSamplerGetCompare = unsafe extern "C" fn(*const NVNsampler, *mut NVNcompareMode, *mut NVNcompareFunc);
pub type FnSamplerGetBorderColor = unsafe extern "C" fn(*const NVNsampler, *mut f32);
pub type FnSamplerGetBorderColori = unsafe extern "C" fn(*const NVNsampler, *mut i32);
pub type FnSamplerGetBorderColorui = unsafe extern "C" fn(*const NVNsampler, *mut u32);
pub type FnSamplerGetMaxAnisotropy = unsafe extern "C" fn(*const NVNsampler) -> f32;
pub type FnSamplerGetReductionFilter = unsafe extern "C" fn(*const NVNsampler) -> NVNsamplerReduction;
pub type FnSamplerCompare = unsafe extern "C" fn(*const NVNsampler, *const NVNsampler) -> bool;
pub type FnSamplerGetDebugID = unsafe extern "C" fn(*const NVNsampler) -> u64;

pub type FnBlendStateSetDefaults = unsafe extern "C" fn(*mut NVNblendState);
pub type FnBlendStateSetBlendTarget = unsafe extern "C" fn(*mut NVNblendState, i32);
pub type FnBlendStateSetBlendFunc =
    unsafe extern "C" fn(*mut NVNblendState, NVNblendFunc, NVNblendFunc, NVNblendFunc, NVNblendFunc);
pub type FnBlendStateSetBlendEquation = unsafe extern "C" fn(*mut NVNblendState, NVNblendEquation, NVNblendEquation);
pub type FnBlendStateSetAdvancedMode = unsafe extern "C" fn(*mut NVNblendState, NVNblendAdvancedMode);
pub type FnBlendStateSetAdvancedOverlap = unsafe extern "C" fn(*mut NVNblendState, NVNblendAdvancedOverlap);
pub type FnBlendStateSetAdvancedPremultipliedSrc = unsafe extern "C" fn(*mut NVNblendState, bool);
pub type FnBlendStateSetAdvancedNormalizedDst = unsafe extern "C" fn(*mut NVNblendState, bool);
pub type FnBlendStateGetBlendTarget = unsafe extern "C" fn(*const NVNblendState) -> i32;
pub type FnBlendStateGetBlendFunc =
    unsafe extern "C" fn(*const NVNblendState, *mut NVNblendFunc, *mut NVNblendFunc, *mut NVNblendFunc, *mut NVNblendFunc);
pub type FnBlendStateGetBlendEquation = unsafe extern "C" fn(*const NVNblendState, *mut NVNblendEquation, *mut NVNblendEquation);
pub type FnBlendStateGetAdvancedMode = unsafe extern "C" fn(*const NVNblendState) -> NVNblendAdvancedMode;
pub type FnBlendStateGetAdvancedOverlap = unsafe extern "C" fn(*const NVNblendState) -> NVNblendAdvancedOverlap;
pub type FnBlendStateGetAdvancedPremultipliedSrc = unsafe extern "C" fn(*const NVNblendState) -> bool;
pub type FnBlendStateGetAdvancedNormalizedDst = unsafe extern "C" fn(*const NVNblendState) -> bool;

pub type FnColorStateSetDefaults = unsafe extern "C" fn(*mut NVNcolorState);
pub type FnColorStateSetBlendEnable = unsafe extern "C" fn(*mut NVNcolorState, i32, bool);
pub type FnColorStateSetLogicOp = unsafe extern "C" fn(*mut NVNcolorState, NVNlogicOp);
pub type FnColorStateSetAlphaTest = unsafe extern "C" fn(*mut NVNcolorState, NVNalphaFunc);
pub type FnColorStateGetBlendEnable = unsafe extern "C" fn(*const NVNcolorState, i32) -> bool;
pub type FnColorStateGetLogicOp = unsafe extern "C" fn(*const NVNcolorState) -> NVNlogicOp;
pub type FnColorStateGetAlphaTest = unsafe extern "C" fn(*const NVNcolorState) -> NVNalphaFunc;

pub type FnChannelMaskStateSetDefaults = unsafe extern "C" fn(*mut NVNchannelMaskState);
pub type FnChannelMaskStateSetChannelMask = unsafe extern "C" fn(*mut NVNchannelMaskState, i32, bool, bool, bool, bool);
pub type FnChannelMaskStateGetChannelMask =
    unsafe extern "C" fn(*const NVNchannelMaskState, i32, *mut bool, *mut bool, *mut bool, *mut bool);

pub type FnMultisampleStateSetDefaults = unsafe extern "C" fn(*mut NVNmultisampleState);
pub type FnMultisampleStateSetMultisampleEnable = unsafe extern "C" fn(*mut NVNmultisampleState, bool);
pub type FnMultisampleStateSetSamples = unsafe extern "C" fn(*mut NVNmultisampleState, i32);
pub type FnMultisampleStateSetAlphaToCoverageEnable = unsafe extern "C" fn(*mut NVNmultisampleState, bool);
pub type FnMultisampleStateSetAlphaToCoverageDither = unsafe extern "C" fn(*mut NVNmultisampleState, bool);
pub type FnMultisampleStateGetMultisampleEnable = unsafe extern "C" fn(*const NVNmultisampleState) -> bool;
pub type FnMultisampleStateGetSamples = unsafe extern "C" fn(*const NVNmultisampleState) -> i32;
pub type FnMultisampleStateGetAlphaToCoverageEnable = unsafe extern "C" fn(*const NVNmultisampleState) -> bool;
pub type FnMultisampleStateGetAlphaToCoverageDither = unsafe extern "C" fn(*const NVNmultisampleState) -> bool;
pub type FnMultisampleStateSetRasterSamples = unsafe extern "C" fn(*mut NVNmultisampleState, i32);
pub type FnMultisampleStateGetRasterSamples = unsafe extern "C" fn(*mut NVNmultisampleState) -> i32;
pub type FnMultisampleStateSetCoverageModulationMode = unsafe extern "C" fn(*mut NVNmultisampleState, NVNcoverageModulationMode);
pub type FnMultisampleStateGetCoverageModulationMode =
    unsafe extern "C" fn(*const NVNmultisampleState) -> NVNcoverageModulationMode;
pub type FnMultisampleStateSetCoverageToColorEnable = unsafe extern "C" fn(*mut NVNmultisampleState, bool);
pub type FnMultisampleStateGetCoverageToColorEnable = unsafe extern "C" fn(*const NVNmultisampleState) -> bool;
pub type FnMultisampleStateSetCoverageToColorOutput = unsafe extern "C" fn(*mut NVNmultisampleState, i32);
pub type FnMultisampleStateGetCoverageToColorOutput = unsafe extern "C" fn(*const NVNmultisampleState) -> i32;
pub type FnMultisampleStateSetSampleLocationsEnable = unsafe extern "C" fn(*mut NVNmultisampleState, bool);
pub type FnMultisampleStateGetSampleLocationsEnable = unsafe extern "C" fn(*const NVNmultisampleState) -> bool;
pub type FnMultisampleStateGetSampleLocationsGrid = unsafe extern "C" fn(*mut NVNmultisampleState, *mut i32, *mut i32);
pub type FnMultisampleStateSetSampleLocationsGridEnable = unsafe extern "C" fn(*mut NVNmultisampleState, bool);
pub type FnMultisampleStateGetSampleLocationsGridEnable = unsafe extern "C" fn(*const NVNmultisampleState) -> bool;
pub type FnMultisampleStateSetSampleLocations = unsafe extern "C" fn(*mut NVNmultisampleState, i32, i32, *const f32);

pub type FnPolygonStateSetDefaults = unsafe extern "C" fn(*mut NVNpolygonState);
pub type FnPolygonStateSetCullFace = unsafe extern "C" fn(*mut NVNpolygonState, NVNface);
pub type FnPolygonStateSetFrontFace = unsafe extern "C" fn(*mut NVNpolygonState, NVNfrontFace);
pub type FnPolygonStateSetPolygonMode = unsafe extern "C" fn(*mut NVNpolygonState, NVNpolygonMode);
pub type FnPolygonStateSetPolygonOffsetEnables = unsafe extern "C" fn(*mut NVNpolygonState, i32);
pub type FnPolygonStateGetCullFace = unsafe extern "C" fn(*const NVNpolygonState) -> NVNface;
pub type FnPolygonStateGetFrontFace = unsafe extern "C" fn(*const NVNpolygonState) -> NVNfrontFace;
pub type FnPolygonStateGetPolygonMode = unsafe extern "C" fn(*const NVNpolygonState) -> NVNpolygonMode;
pub type FnPolygonStateGetPolygonOffsetEnables = unsafe extern "C" fn(*const NVNpolygonState) -> NVNpolygonOffsetEnable;

pub type FnDepthStencilStateSetDefaults = unsafe extern "C" fn(*mut NVNdepthStencilState);
pub type FnDepthStencilStateSetDepthTestEnable = unsafe extern "C" fn(*mut NVNdepthStencilState, bool);
pub type FnDepthStencilStateSetDepthWriteEnable = unsafe extern "C" fn(*mut NVNdepthStencilState, bool);
pub type FnDepthStencilStateSetDepthFunc = unsafe extern "C" fn(*mut NVNdepthStencilState, NVNdepthFunc);
pub type FnDepthStencilStateSetStencilTestEnable = unsafe extern "C" fn(*mut NVNdepthStencilState, bool);
pub type FnDepthStencilStateSetStencilFunc = unsafe extern "C" fn(*mut NVNdepthStencilState, NVNface, NVNstencilFunc);
pub type FnDepthStencilStateSetStencilOp =
    unsafe extern "C" fn(*mut NVNdepthStencilState, NVNface, NVNstencilOp, NVNstencilOp, NVNstencilOp);
pub type FnDepthStencilStateGetDepthTestEnable = unsafe extern "C" fn(*const NVNdepthStencilState) -> bool;
pub type FnDepthStencilStateGetDepthWriteEnable = unsafe extern "C" fn(*const NVNdepthStencilState) -> bool;
pub type FnDepthStencilStateGetDepthFunc = unsafe extern "C" fn(*const NVNdepthStencilState) -> NVNdepthFunc;
pub type FnDepthStencilStateGetStencilTestEnable = unsafe extern "C" fn(*const NVNdepthStencilState) -> bool;
pub type FnDepthStencilStateGetStencilFunc = unsafe extern "C" fn(*const NVNdepthStencilState, NVNface) -> NVNstencilFunc;
pub type FnDepthStencilStateGetStencilOp =
    unsafe extern "C" fn(*const NVNdepthStencilState, NVNface, *mut NVNstencilOp, *mut NVNstencilOp, *mut NVNstencilOp);

pub type FnVertexAttribStateSetDefaults = unsafe extern "C" fn(*mut NVNvertexAttribState);
pub type FnVertexAttribStateSetFormat = unsafe extern "C" fn(*mut NVNvertexAttribState, NVNformat, isize);
pub type FnVertexAttribStateSetStreamIndex = unsafe extern "C" fn(*mut NVNvertexAttribState, i32);
pub type FnVertexAttribStateGetFormat = unsafe extern "C" fn(*const NVNvertexAttribState, *mut NVNformat, *mut isize);
pub type FnVertexAttribStateGetStreamIndex = unsafe extern "C" fn(*const NVNvertexAttribState) -> i32;
pub type FnVertexStreamStateSetDefaults = unsafe extern "C" fn(*mut NVNvertexStreamState);
pub type FnVertexStreamStateSetStride = unsafe extern "C" fn(*mut NVNvertexStreamState, isize);
pub type FnVertexStreamStateSetDivisor = unsafe extern "C" fn(*mut NVNvertexStreamState, i32);
pub type FnVertexStreamStateGetStride = unsafe extern "C" fn(*const NVNvertexStreamState) -> isize;
pub type FnVertexStreamStateGetDivisor = unsafe extern "C" fn(*const NVNvertexStreamState) -> i32;

pub type FnCommandBufferInitialize = unsafe extern "C" fn(*mut NVNcommandBuffer, *mut NVNdevice) -> bool;
pub type FnCommandBufferFinalize = unsafe extern "C" fn(*mut NVNcommandBuffer);
pub type FnCommandBufferSetDebugLabel = unsafe extern "C" fn(*mut NVNcommandBuffer, *const c_char);
pub type FnCommandBufferSetMemoryCallback = unsafe extern "C" fn(*mut NVNcommandBuffer, FnCommandBufferMemoryCallback);
pub type FnCommandBufferSetMemoryCallbackData = unsafe extern "C" fn(*mut NVNcommandBuffer, *mut c_void);
pub type FnCommandBufferAddCommandMemory = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNmemoryPool, isize, usize);
pub type FnCommandBufferAddControlMemory = unsafe extern "C" fn(*mut NVNcommandBuffer, *mut c_void, usize);
pub type FnCommandBufferGetCommandMemorySize = unsafe extern "C" fn(*const NVNcommandBuffer) -> usize;
pub type FnCommandBufferGetCommandMemoryUsed = unsafe extern "C" fn(*const NVNcommandBuffer) -> usize;
pub type FnCommandBufferGetCommandMemoryFree = unsafe extern "C" fn(*const NVNcommandBuffer) -> usize;
pub type FnCommandBufferGetControlMemorySize = unsafe extern "C" fn(*const NVNcommandBuffer) -> usize;
pub type FnCommandBufferGetControlMemoryUsed = unsafe extern "C" fn(*const NVNcommandBuffer) -> usize;
pub type FnCommandBufferGetControlMemoryFree = unsafe extern "C" fn(*const NVNcommandBuffer) -> usize;
pub type FnCommandBufferBeginRecording = unsafe extern "C" fn(*mut NVNcommandBuffer);
pub type FnCommandBufferEndRecording = unsafe extern "C" fn(*mut NVNcommandBuffer) -> NVNcommandHandle;
pub type FnCommandBufferCallCommands = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, *const NVNcommandHandle);
pub type FnCommandBufferCopyCommands = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, *const NVNcommandHandle);
pub type FnCommandBufferBindBlendState = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNblendState);
pub type FnCommandBufferBindChannelMaskState = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNchannelMaskState);
pub type FnCommandBufferBindColorState = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNcolorState);
pub type FnCommandBufferBindMultisampleState = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNmultisampleState);
pub type FnCommandBufferBindPolygonState = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNpolygonState);
pub type FnCommandBufferBindDepthStencilState = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNdepthStencilState);
pub type FnCommandBufferBindVertexAttribState = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, *const NVNvertexAttribState);
pub type FnCommandBufferBindVertexStreamState = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, *const NVNvertexStreamState);
pub type FnCommandBufferBindProgram = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNprogram, i32);
pub type FnCommandBufferBindVertexBuffer = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, NVNbufferAddress, usize);
pub type FnCommandBufferBindVertexBuffers = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32, *const NVNbufferRange);
pub type FnCommandBufferBindUniformBuffer =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, NVNbufferAddress, usize);
pub type FnCommandBufferBindUniformBuffers =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, i32, *const NVNbufferRange);
pub type FnCommandBufferBindTransformFeedbackBuffer = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, NVNbufferAddress, usize);
pub type FnCommandBufferBindTransformFeedbackBuffers =
    unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32, *const NVNbufferRange);
pub type FnCommandBufferBindStorageBuffer =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, NVNbufferAddress, usize);
pub type FnCommandBufferBindStorageBuffers =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, i32, *const NVNbufferRange);
pub type FnCommandBufferBindTexture = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, NVNtextureHandle);
pub type FnCommandBufferBindTextures =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, i32, *const NVNtextureHandle);
pub type FnCommandBufferBindImage = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, NVNimageHandle);
pub type FnCommandBufferBindImages = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, i32, *const NVNimageHandle);
pub type FnCommandBufferSetPatchSize = unsafe extern "C" fn(*mut NVNcommandBuffer, i32);
pub type FnCommandBufferSetInnerTessellationLevels = unsafe extern "C" fn(*mut NVNcommandBuffer, *const f32);
pub type FnCommandBufferSetOuterTessellationLevels = unsafe extern "C" fn(*mut NVNcommandBuffer, *const f32);
pub type FnCommandBufferSetPrimitiveRestart = unsafe extern "C" fn(*mut NVNcommandBuffer, bool, i32);
pub type FnCommandBufferBeginTransformFeedback = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress);
pub type FnCommandBufferEndTransformFeedback = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress);
pub type FnCommandBufferPauseTransformFeedback = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress);
pub type FnCommandBufferResumeTransformFeedback = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress);
pub type FnCommandBufferDrawTransformFeedback = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNdrawPrimitive, NVNbufferAddress);
pub type FnCommandBufferDrawArrays = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNdrawPrimitive, i32, i32);
pub type FnCommandBufferDrawElements =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNdrawPrimitive, NVNindexType, i32, NVNbufferAddress);
pub type FnCommandBufferDrawElementsBaseVertex =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNdrawPrimitive, NVNindexType, i32, NVNbufferAddress, i32);
pub type FnCommandBufferDrawArraysInstanced = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNdrawPrimitive, i32, i32, i32, i32);
pub type FnCommandBufferDrawElementsInstanced =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNdrawPrimitive, NVNindexType, i32, NVNbufferAddress, i32, i32, i32);
pub type FnCommandBufferDrawArraysIndirect = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNdrawPrimitive, NVNbufferAddress);
pub type FnCommandBufferDrawElementsIndirect =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNdrawPrimitive, NVNindexType, NVNbufferAddress, NVNbufferAddress);
pub type FnCommandBufferMultiDrawArraysIndirectCount =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNdrawPrimitive, NVNbufferAddress, NVNbufferAddress, i32, isize);
pub type FnCommandBufferMultiDrawElementsIndirectCount = unsafe extern "C" fn(
    *mut NVNcommandBuffer,
    NVNdrawPrimitive,
    NVNindexType,
    NVNbufferAddress,
    NVNbufferAddress,
    NVNbufferAddress,
    i32,
    isize,
);
pub type FnCommandBufferClearColor = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, *const f32, NVNclearColorMask_);
pub type FnCommandBufferClearColori = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, *const i32, NVNclearColorMask_);
pub type FnCommandBufferClearColorui = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, *const u32, NVNclearColorMask_);
pub type FnCommandBufferClearDepthStencil = unsafe extern "C" fn(*mut NVNcommandBuffer, f32, bool, i32, i32);
pub type FnCommandBufferDispatchCompute = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32, i32);
pub type FnCommandBufferDispatchComputeIndirect = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress);
pub type FnCommandBufferSetViewport = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32, i32, i32);
pub type FnCommandBufferSetViewports = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32, *const f32);
pub type FnCommandBufferSetViewportSwizzles = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32, *const NVNviewportSwizzle);
pub type FnCommandBufferSetScissor = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32, i32, i32);
pub type FnCommandBufferSetScissors = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32, *const i32);
pub type FnCommandBufferSetDepthRange = unsafe extern "C" fn(*mut NVNcommandBuffer, f32, f32);
pub type FnCommandBufferSetDepthBounds = unsafe extern "C" fn(*mut NVNcommandBuffer, bool, f32, f32);
pub type FnCommandBufferSetDepthRanges = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32, *const f32);
pub type FnCommandBufferSetTiledCacheAction = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNtiledCacheAction);
pub type FnCommandBufferSetTiledCacheTileSize = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32);
pub type FnCommandBufferBindSeparateTexture =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, NVNseparateTextureHandle);
pub type FnCommandBufferBindSeparateSampler =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, NVNseparateSamplerHandle);
pub type FnCommandBufferBindSeparateTextures =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, i32, *const NVNseparateTextureHandle);
pub type FnCommandBufferBindSeparateSamplers =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNshaderStage, i32, i32, *const NVNseparateSamplerHandle);
pub type FnCommandBufferSetStencilValueMask = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNface, i32);
pub type FnCommandBufferSetStencilMask = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNface, i32);
pub type FnCommandBufferSetStencilRef = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNface, i32);
pub type FnCommandBufferSetBlendColor = unsafe extern "C" fn(*mut NVNcommandBuffer, *const f32);
pub type FnCommandBufferSetPointSize = unsafe extern "C" fn(*mut NVNcommandBuffer, f32);
pub type FnCommandBufferSetLineWidth = unsafe extern "C" fn(*mut NVNcommandBuffer, f32);
pub type FnCommandBufferSetPolygonOffsetClamp = unsafe extern "C" fn(*mut NVNcommandBuffer, f32, f32, f32);
pub type FnCommandBufferSetAlphaRef = unsafe extern "C" fn(*mut NVNcommandBuffer, f32);
pub type FnCommandBufferSetSampleMask = unsafe extern "C" fn(*mut NVNcommandBuffer, i32);
pub type FnCommandBufferSetRasterizerDiscard = unsafe extern "C" fn(*mut NVNcommandBuffer, bool);
pub type FnCommandBufferSetDepthClamp = unsafe extern "C" fn(*mut NVNcommandBuffer, bool);
pub type FnCommandBufferSetConservativeRasterEnable = unsafe extern "C" fn(*mut NVNcommandBuffer, bool);
pub type FnCommandBufferSetConservativeRasterDilate = unsafe extern "C" fn(*mut NVNcommandBuffer, f32);
pub type FnCommandBufferSetSubpixelPrecisionBias = unsafe extern "C" fn(*mut NVNcommandBuffer, i32, i32);
pub type FnCommandBufferCopyBufferToTexture = unsafe extern "C" fn(
    *mut NVNcommandBuffer,
    NVNbufferAddress,
    *const NVNtexture,
    *const NVNtextureView,
    *const NVNcopyRegion,
    NVNcopyFlags_,
);
pub type FnCommandBufferCopyTextureToBuffer = unsafe extern "C" fn(
    *mut NVNcommandBuffer,
    *const NVNtexture,
    *const NVNtextureView,
    *const NVNcopyRegion,
    NVNbufferAddress,
    NVNcopyFlags_,
);
pub type FnCommandBufferCopyTextureToTexture = unsafe extern "C" fn(
    *mut NVNcommandBuffer,
    *const NVNtexture,
    *const NVNtextureView,
    *const NVNcopyRegion,
    *const NVNtexture,
    *const NVNtextureView,
    *const NVNcopyRegion,
    NVNcopyFlags_,
);
pub type FnCommandBufferCopyBufferToBuffer =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress, NVNbufferAddress, usize, i32);
pub type FnCommandBufferClearBuffer = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress, usize, u32);
pub type FnCommandBufferClearTexture =
    unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNtexture, *const NVNtextureView, *const NVNcopyRegion, *const f32, i32);
pub type FnCommandBufferClearTexturei =
    unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNtexture, *const NVNtextureView, *const NVNcopyRegion, *const i32, i32);
pub type FnCommandBufferClearTextureui =
    unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNtexture, *const NVNtextureView, *const NVNcopyRegion, *const u32, i32);
pub type FnCommandBufferUpdateUniformBuffer =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress, usize, isize, usize, *const c_void);
pub type FnCommandBufferReportCounter = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNcounterType, NVNbufferAddress);
pub type FnCommandBufferResetCounter = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNcounterType);
pub type FnCommandBufferReportValue = unsafe extern "C" fn(*mut NVNcommandBuffer, u32, NVNbufferAddress);
pub type FnCommandBufferSetRenderEnable = unsafe extern "C" fn(*mut NVNcommandBuffer, bool);
pub type FnCommandBufferSetRenderEnableConditional =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNconditionalRenderMode, NVNbufferAddress);
pub type FnCommandBufferSetRenderTargets = unsafe extern "C" fn(
    *mut NVNcommandBuffer,
    i32,
    *const *const NVNtexture,
    *const *const NVNtextureView,
    *const NVNtexture,
    *const NVNtextureView,
);
pub type FnCommandBufferDiscardColor = unsafe extern "C" fn(*mut NVNcommandBuffer, i32);
pub type FnCommandBufferDiscardDepthStencil = unsafe extern "C" fn(*mut NVNcommandBuffer);
pub type FnCommandBufferDownsample = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNtexture, *const NVNtexture);
pub type FnCommandBufferTiledDownsample = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNtexture, *const NVNtexture);
pub type FnCommandBufferDownsampleTextureView = unsafe extern "C" fn(
    *mut NVNcommandBuffer,
    *const NVNtexture,
    *const NVNtextureView,
    *const NVNtexture,
    *const NVNtextureView,
);
pub type FnCommandBufferTiledDownsampleTextureView = unsafe extern "C" fn(
    *mut NVNcommandBuffer,
    *const NVNtexture,
    *const NVNtextureView,
    *const NVNtexture,
    *const NVNtextureView,
);
pub type FnCommandBufferBarrier = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbarrierBits_);
pub type FnCommandBufferWaitSync = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNsync);
pub type FnCommandBufferFenceSync = unsafe extern "C" fn(*mut NVNcommandBuffer, *mut NVNsync, NVNsyncCondition, i32);
pub type FnCommandBufferSetTexturePool = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNtexturePool);
pub type FnCommandBufferSetSamplerPool = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNsamplerPool);
pub type FnCommandBufferSetShaderScratchMemory = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNmemoryPool, isize, usize);
pub type FnCommandBufferSaveZCullData = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress, usize);
pub type FnCommandBufferRestoreZCullData = unsafe extern "C" fn(*mut NVNcommandBuffer, NVNbufferAddress, usize);
pub type FnCommandBufferSetCopyRowStride = unsafe extern "C" fn(*mut NVNcommandBuffer, isize);
pub type FnCommandBufferSetCopyImageStride = unsafe extern "C" fn(*mut NVNcommandBuffer, isize);
pub type FnCommandBufferGetCopyRowStride = unsafe extern "C" fn(*const NVNcommandBuffer) -> isize;
pub type FnCommandBufferGetCopyImageStride = unsafe extern "C" fn(*const NVNcommandBuffer) -> isize;
pub type FnCommandBufferDrawTexture =
    unsafe extern "C" fn(*mut NVNcommandBuffer, NVNtextureHandle, *const NVNdrawTextureRegion, *const NVNdrawTextureRegion);

pub type FnProgramSetSubroutineLinkage = unsafe extern "C" fn(*mut NVNprogram, i32, *const NVNsubroutineLinkageMapPtr) -> bool;

pub type FnCommandBufferSetProgramSubroutines =
    unsafe extern "C" fn(*mut NVNcommandBuffer, *mut NVNprogram, NVNshaderStage, i32, i32, *const i32);
pub type FnCommandBufferBindCoverageModulationTable = unsafe extern "C" fn(*mut NVNcommandBuffer, *const f32);
pub type FnCommandBufferResolveDepthBuffer = unsafe extern "C" fn(*mut NVNcommandBuffer);
pub type FnCommandBufferPushDebugGroupStatic = unsafe extern "C" fn(*mut NVNcommandBuffer, u32, *const c_char);
pub type FnCommandBufferPushDebugGroupDynamic = unsafe extern "C" fn(*mut NVNcommandBuffer, u32, *const c_char);
pub type FnCommandBufferPushDebugGroup = unsafe extern "C" fn(*mut NVNcommandBuffer, u32, *const c_char);
pub type FnCommandBufferPopDebugGroup = unsafe extern "C" fn(*mut NVNcommandBuffer);
pub type FnCommandBufferPopDebugGroupId = unsafe extern "C" fn(*mut NVNcommandBuffer, u32);
pub type FnCommandBufferInsertDebugMarkerStatic = unsafe extern "C" fn(*mut NVNcommandBuffer, u32, *const c_char);
pub type FnCommandBufferInsertDebugMarkerDynamic = unsafe extern "C" fn(*mut NVNcommandBuffer, u32, *const c_char);
pub type FnCommandBufferInsertDebugMarker = unsafe extern "C" fn(*mut NVNcommandBuffer, *const c_char);
pub type FnCommandBufferGetMemoryCallback =
    unsafe extern "C" fn(*const NVNcommandBuffer) -> Option<FnCommandBufferMemoryCallback>;
pub type FnCommandBufferGetMemoryCallbackData = unsafe extern "C" fn(*const NVNcommandBuffer) -> *mut c_void;
pub type FnCommandBufferIsRecording = unsafe extern "C" fn(*const NVNcommandBuffer) -> bool;

pub type FnSyncInitialize = unsafe extern "C" fn(*mut NVNsync, *mut NVNdevice) -> bool;
pub type FnSyncFinalize = unsafe extern "C" fn(*mut NVNsync);
pub type FnSyncSetDebugLabel = unsafe extern "C" fn(*mut NVNsync, *const c_char);

pub type FnQueueFenceSync = unsafe extern "C" fn(*mut NVNqueue, *mut NVNsync, NVNsyncCondition, i32);

pub type FnSyncWait = unsafe extern "C" fn(*const NVNsync, u64) -> NVNsyncWaitResult;

pub type FnQueueWaitSync = unsafe extern "C" fn(*mut NVNqueue, *const NVNsync) -> bool;

pub type FnEventBuilderSetDefaults = unsafe extern "C" fn(*mut NVNeventBuilder);
pub type FnEventBuilderSetStorage = unsafe extern "C" fn(*mut NVNeventBuilder, *const NVNmemoryPool, i64);
pub type FnEventInitialize = unsafe extern "C" fn(*mut NVNevent, *const NVNeventBuilder) -> bool;
pub type FnEventFinalize = unsafe extern "C" fn(*mut NVNevent);
pub type FnEventGetValue = unsafe extern "C" fn(*const NVNevent) -> u32;
pub type FnEventSignal = unsafe extern "C" fn(*mut NVNevent, NVNeventSignalMode, u32);

pub type FnCommandBufferWaitEvent = unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNevent, NVNeventWaitMode, u32);
pub type FnCommandBufferSignalEvent =
    unsafe extern "C" fn(*mut NVNcommandBuffer, *const NVNevent, NVNeventSignalMode, NVNeventSignalLocation, i32, u32);

macro_rules! nvn_procs {
    ($($field:ident: $ty:ty = $name:literal,)*) => {
        /// Table of every NVN C entry point. Entries the driver doesn't export are `None`.
        #[derive(Clone, Copy, Default)]
        pub struct NVNprocs {
            $(pub $field: Option<$ty>,)*
        }

        impl NVNprocs {
            /// Resolves every entry point through `nvnDeviceGetProcAddress`.
            ///
            /// # Safety
            /// `get_proc_address` must be `nvnDeviceGetProcAddress`, and `device` must be null or
            /// point to an initialized device.
            pub unsafe fn load(device: *const NVNdevice, get_proc_address: FnDeviceGetProcAddress) -> Self {
                Self {
                    $($field: unsafe {
                        core::mem::transmute::<Option<FnGenericFuncPtr>, Option<$ty>>(get_proc_address(device, $name.as_ptr()))
                    },)*
                }
            }
        }
    };
}

nvn_procs! {
    device_builder_set_defaults: FnDeviceBuilderSetDefaults = c"nvnDeviceBuilderSetDefaults",
    device_builder_set_flags: FnDeviceBuilderSetFlags = c"nvnDeviceBuilderSetFlags",
    device_initialize: FnDeviceInitialize = c"nvnDeviceInitialize",
    device_finalize: FnDeviceFinalize = c"nvnDeviceFinalize",
    device_set_debug_label: FnDeviceSetDebugLabel = c"nvnDeviceSetDebugLabel",
    device_get_proc_address: FnDeviceGetProcAddress = c"nvnDeviceGetProcAddress",
    device_get_integer: FnDeviceGetInteger = c"nvnDeviceGetInteger",
    device_get_current_timestamp_in_nanoseconds: FnDeviceGetCurrentTimestampInNanoseconds = c"nvnDeviceGetCurrentTimestampInNanoseconds",
    device_set_intermediate_shader_cache: FnDeviceSetIntermediateShaderCache = c"nvnDeviceSetIntermediateShaderCache",
    device_get_texture_handle: FnDeviceGetTextureHandle = c"nvnDeviceGetTextureHandle",
    device_get_texel_fetch_handle: FnDeviceGetTexelFetchHandle = c"nvnDeviceGetTexelFetchHandle",
    device_get_image_handle: FnDeviceGetImageHandle = c"nvnDeviceGetImageHandle",
    device_install_debug_callback: FnDeviceInstallDebugCallback = c"nvnDeviceInstallDebugCallback",
    device_generate_debug_domain_id: FnDeviceGenerateDebugDomainId = c"nvnDeviceGenerateDebugDomainId",
    device_set_window_origin_mode: FnDeviceSetWindowOriginMode = c"nvnDeviceSetWindowOriginMode",
    device_set_depth_mode: FnDeviceSetDepthMode = c"nvnDeviceSetDepthMode",
    device_register_fast_clear_color: FnDeviceRegisterFastClearColor = c"nvnDeviceRegisterFastClearColor",
    device_register_fast_clear_colori: FnDeviceRegisterFastClearColori = c"nvnDeviceRegisterFastClearColori",
    device_register_fast_clear_colorui: FnDeviceRegisterFastClearColorui = c"nvnDeviceRegisterFastClearColorui",
    device_register_fast_clear_depth: FnDeviceRegisterFastClearDepth = c"nvnDeviceRegisterFastClearDepth",
    device_get_window_origin_mode: FnDeviceGetWindowOriginMode = c"nvnDeviceGetWindowOriginMode",
    device_get_depth_mode: FnDeviceGetDepthMode = c"nvnDeviceGetDepthMode",
    device_get_timestamp_in_nanoseconds: FnDeviceGetTimestampInNanoseconds = c"nvnDeviceGetTimestampInNanoseconds",
    device_apply_deferred_finalizes: FnDeviceApplyDeferredFinalizes = c"nvnDeviceApplyDeferredFinalizes",
    device_finalize_command_handle: FnDeviceFinalizeCommandHandle = c"nvnDeviceFinalizeCommandHandle",
    device_walk_debug_database: FnDeviceWalkDebugDatabase = c"nvnDeviceWalkDebugDatabase",
    device_get_separate_texture_handle: FnDeviceGetSeparateTextureHandle = c"nvnDeviceGetSeparateTextureHandle",
    device_get_separate_sampler_handle: FnDeviceGetSeparateSamplerHandle = c"nvnDeviceGetSeparateSamplerHandle",
    device_is_external_debugger_attached: FnDeviceIsExternalDebuggerAttached = c"nvnDeviceIsExternalDebuggerAttached",
    queue_get_error: FnQueueGetError = c"nvnQueueGetError",
    queue_get_total_command_memory_used: FnQueueGetTotalCommandMemoryUsed = c"nvnQueueGetTotalCommandMemoryUsed",
    queue_get_total_control_memory_used: FnQueueGetTotalControlMemoryUsed = c"nvnQueueGetTotalControlMemoryUsed",
    queue_get_total_compute_memory_used: FnQueueGetTotalComputeMemoryUsed = c"nvnQueueGetTotalComputeMemoryUsed",
    queue_reset_memory_usage_counts: FnQueueResetMemoryUsageCounts = c"nvnQueueResetMemoryUsageCounts",
    queue_builder_set_device: FnQueueBuilderSetDevice = c"nvnQueueBuilderSetDevice",
    queue_builder_set_defaults: FnQueueBuilderSetDefaults = c"nvnQueueBuilderSetDefaults",
    queue_builder_set_flags: FnQueueBuilderSetFlags = c"nvnQueueBuilderSetFlags",
    queue_builder_set_command_memory_size: FnQueueBuilderSetCommandMemorySize = c"nvnQueueBuilderSetCommandMemorySize",
    queue_builder_set_control_memory_size: FnQueueBuilderSetControlMemorySize = c"nvnQueueBuilderSetControlMemorySize",
    queue_builder_set_compute_memory_size: FnQueueBuilderSetComputeMemorySize = c"nvnQueueBuilderSetComputeMemorySize",
    queue_builder_get_queue_memory_size: FnQueueBuilderGetQueueMemorySize = c"nvnQueueBuilderGetQueueMemorySize",
    queue_builder_set_queue_memory: FnQueueBuilderSetQueueMemory = c"nvnQueueBuilderSetQueueMemory",
    queue_builder_set_command_flush_threshold: FnQueueBuilderSetCommandFlushThreshold = c"nvnQueueBuilderSetCommandFlushThreshold",
    queue_builder_get_device: FnQueueBuilderGetDevice = c"nvnQueueBuilderGetDevice",
    queue_builder_get_flags: FnQueueBuilderGetFlags = c"nvnQueueBuilderGetFlags",
    queue_builder_get_command_memory_size: FnQueueBuilderGetCommandMemorySize = c"nvnQueueBuilderGetCommandMemorySize",
    queue_initialize: FnQueueInitialize = c"nvnQueueInitialize",
    queue_finalize: FnQueueFinalize = c"nvnQueueFinalize",
    queue_set_debug_label: FnQueueSetDebugLabel = c"nvnQueueSetDebugLabel",
    queue_submit_commands: FnQueueSubmitCommands = c"nvnQueueSubmitCommands",
    queue_flush: FnQueueFlush = c"nvnQueueFlush",
    queue_finish: FnQueueFinish = c"nvnQueueFinish",
    queue_present_texture: FnQueuePresentTexture = c"nvnQueuePresentTexture",
    queue_acquire_texture: FnQueueAcquireTexture = c"nvnQueueAcquireTexture",
    window_builder_set_device: FnWindowBuilderSetDevice = c"nvnWindowBuilderSetDevice",
    window_builder_set_defaults: FnWindowBuilderSetDefaults = c"nvnWindowBuilderSetDefaults",
    window_builder_set_native_window: FnWindowBuilderSetNativeWindow = c"nvnWindowBuilderSetNativeWindow",
    window_builder_set_textures: FnWindowBuilderSetTextures = c"nvnWindowBuilderSetTextures",
    window_builder_set_present_interval: FnWindowBuilderSetPresentInterval = c"nvnWindowBuilderSetPresentInterval",
    window_builder_get_native_window: FnWindowBuilderGetNativeWindow = c"nvnWindowBuilderGetNativeWindow",
    window_builder_get_present_interval: FnWindowBuilderGetPresentInterval = c"nvnWindowBuilderGetPresentInterval",
    window_initialize: FnWindowInitialize = c"nvnWindowInitialize",
    window_finalize: FnWindowFinalize = c"nvnWindowFinalize",
    window_set_debug_label: FnWindowSetDebugLabel = c"nvnWindowSetDebugLabel",
    window_acquire_texture: FnWindowAcquireTexture = c"nvnWindowAcquireTexture",
    window_get_native_window: FnWindowGetNativeWindow = c"nvnWindowGetNativeWindow",
    window_get_present_interval: FnWindowGetPresentInterval = c"nvnWindowGetPresentInterval",
    window_set_present_interval: FnWindowSetPresentInterval = c"nvnWindowSetPresentInterval",
    window_set_crop: FnWindowSetCrop = c"nvnWindowSetCrop",
    window_get_crop: FnWindowGetCrop = c"nvnWindowGetCrop",
    program_initialize: FnProgramInitialize = c"nvnProgramInitialize",
    program_finalize: FnProgramFinalize = c"nvnProgramFinalize",
    program_set_debug_label: FnProgramSetDebugLabel = c"nvnProgramSetDebugLabel",
    program_set_shaders: FnProgramSetShaders = c"nvnProgramSetShaders",
    memory_pool_builder_set_device: FnMemoryPoolBuilderSetDevice = c"nvnMemoryPoolBuilderSetDevice",
    memory_pool_builder_set_defaults: FnMemoryPoolBuilderSetDefaults = c"nvnMemoryPoolBuilderSetDefaults",
    memory_pool_builder_set_storage: FnMemoryPoolBuilderSetStorage = c"nvnMemoryPoolBuilderSetStorage",
    memory_pool_builder_set_flags: FnMemoryPoolBuilderSetFlags = c"nvnMemoryPoolBuilderSetFlags",
    memory_pool_builder_get_memory: FnMemoryPoolBuilderGetMemory = c"nvnMemoryPoolBuilderGetMemory",
    memory_pool_builder_get_size: FnMemoryPoolBuilderGetSize = c"nvnMemoryPoolBuilderGetSize",
    memory_pool_builder_get_flags: FnMemoryPoolBuilderGetFlags = c"nvnMemoryPoolBuilderGetFlags",
    memory_pool_initialize: FnMemoryPoolInitialize = c"nvnMemoryPoolInitialize",
    memory_pool_set_debug_label: FnMemoryPoolSetDebugLabel = c"nvnMemoryPoolSetDebugLabel",
    memory_pool_finalize: FnMemoryPoolFinalize = c"nvnMemoryPoolFinalize",
    memory_pool_map: FnMemoryPoolMap = c"nvnMemoryPoolMap",
    memory_pool_flush_mapped_range: FnMemoryPoolFlushMappedRange = c"nvnMemoryPoolFlushMappedRange",
    memory_pool_invalidate_mapped_range: FnMemoryPoolInvalidateMappedRange = c"nvnMemoryPoolInvalidateMappedRange",
    memory_pool_get_buffer_address: FnMemoryPoolGetBufferAddress = c"nvnMemoryPoolGetBufferAddress",
    memory_pool_map_virtual: FnMemoryPoolMapVirtual = c"nvnMemoryPoolMapVirtual",
    memory_pool_get_size: FnMemoryPoolGetSize = c"nvnMemoryPoolGetSize",
    memory_pool_get_flags: FnMemoryPoolGetFlags = c"nvnMemoryPoolGetFlags",
    texture_pool_initialize: FnTexturePoolInitialize = c"nvnTexturePoolInitialize",
    texture_pool_set_debug_label: FnTexturePoolSetDebugLabel = c"nvnTexturePoolSetDebugLabel",
    texture_pool_finalize: FnTexturePoolFinalize = c"nvnTexturePoolFinalize",
    texture_pool_register_texture: FnTexturePoolRegisterTexture = c"nvnTexturePoolRegisterTexture",
    texture_pool_register_image: FnTexturePoolRegisterImage = c"nvnTexturePoolRegisterImage",
    texture_pool_get_memory_pool: FnTexturePoolGetMemoryPool = c"nvnTexturePoolGetMemoryPool",
    texture_pool_get_memory_offset: FnTexturePoolGetMemoryOffset = c"nvnTexturePoolGetMemoryOffset",
    texture_pool_get_size: FnTexturePoolGetSize = c"nvnTexturePoolGetSize",
    sampler_pool_initialize: FnSamplerPoolInitialize = c"nvnSamplerPoolInitialize",
    sampler_pool_set_debug_label: FnSamplerPoolSetDebugLabel = c"nvnSamplerPoolSetDebugLabel",
    sampler_pool_finalize: FnSamplerPoolFinalize = c"nvnSamplerPoolFinalize",
    sampler_pool_register_sampler: FnSamplerPoolRegisterSampler = c"nvnSamplerPoolRegisterSampler",
    sampler_pool_register_sampler_builder: FnSamplerPoolRegisterSamplerBuilder = c"nvnSamplerPoolRegisterSamplerBuilder",
    sampler_pool_get_memory_pool: FnSamplerPoolGetMemoryPool = c"nvnSamplerPoolGetMemoryPool",
    sampler_pool_get_memory_offset: FnSamplerPoolGetMemoryOffset = c"nvnSamplerPoolGetMemoryOffset",
    sampler_pool_get_size: FnSamplerPoolGetSize = c"nvnSamplerPoolGetSize",
    buffer_builder_set_device: FnBufferBuilderSetDevice = c"nvnBufferBuilderSetDevice",
    buffer_builder_set_defaults: FnBufferBuilderSetDefaults = c"nvnBufferBuilderSetDefaults",
    buffer_builder_set_storage: FnBufferBuilderSetStorage = c"nvnBufferBuilderSetStorage",
    buffer_builder_get_memory_pool: FnBufferBuilderGetMemoryPool = c"nvnBufferBuilderGetMemoryPool",
    buffer_builder_get_memory_offset: FnBufferBuilderGetMemoryOffset = c"nvnBufferBuilderGetMemoryOffset",
    buffer_builder_get_size: FnBufferBuilderGetSize = c"nvnBufferBuilderGetSize",
    buffer_initialize: FnBufferInitialize = c"nvnBufferInitialize",
    buffer_set_debug_label: FnBufferSetDebugLabel = c"nvnBufferSetDebugLabel",
    buffer_finalize: FnBufferFinalize = c"nvnBufferFinalize",
    buffer_map: FnBufferMap = c"nvnBufferMap",
    buffer_get_address: FnBufferGetAddress = c"nvnBufferGetAddress",
    buffer_flush_mapped_range: FnBufferFlushMappedRange = c"nvnBufferFlushMappedRange",
    buffer_invalidate_mapped_range: FnBufferInvalidateMappedRange = c"nvnBufferInvalidateMappedRange",
    buffer_get_memory_pool: FnBufferGetMemoryPool = c"nvnBufferGetMemoryPool",
    buffer_get_memory_offset: FnBufferGetMemoryOffset = c"nvnBufferGetMemoryOffset",
    buffer_get_size: FnBufferGetSize = c"nvnBufferGetSize",
    buffer_get_debug_id: FnBufferGetDebugID = c"nvnBufferGetDebugID",
    texture_builder_set_device: FnTextureBuilderSetDevice = c"nvnTextureBuilderSetDevice",
    texture_builder_set_defaults: FnTextureBuilderSetDefaults = c"nvnTextureBuilderSetDefaults",
    texture_builder_set_flags: FnTextureBuilderSetFlags = c"nvnTextureBuilderSetFlags",
    texture_builder_set_target: FnTextureBuilderSetTarget = c"nvnTextureBuilderSetTarget",
    texture_builder_set_width: FnTextureBuilderSetWidth = c"nvnTextureBuilderSetWidth",
    texture_builder_set_height: FnTextureBuilderSetHeight = c"nvnTextureBuilderSetHeight",
    texture_builder_set_depth: FnTextureBuilderSetDepth = c"nvnTextureBuilderSetDepth",
    texture_builder_set_size_1d: FnTextureBuilderSetSize1D = c"nvnTextureBuilderSetSize1D",
    texture_builder_set_size_2d: FnTextureBuilderSetSize2D = c"nvnTextureBuilderSetSize2D",
    texture_builder_set_size_3d: FnTextureBuilderSetSize3D = c"nvnTextureBuilderSetSize3D",
    texture_builder_set_levels: FnTextureBuilderSetLevels = c"nvnTextureBuilderSetLevels",
    texture_builder_set_format: FnTextureBuilderSetFormat = c"nvnTextureBuilderSetFormat",
    texture_builder_set_samples: FnTextureBuilderSetSamples = c"nvnTextureBuilderSetSamples",
    texture_builder_set_swizzle: FnTextureBuilderSetSwizzle = c"nvnTextureBuilderSetSwizzle",
    texture_builder_set_depth_stencil_mode: FnTextureBuilderSetDepthStencilMode = c"nvnTextureBuilderSetDepthStencilMode",
    texture_builder_get_storage_size: FnTextureBuilderGetStorageSize = c"nvnTextureBuilderGetStorageSize",
    texture_builder_get_storage_alignment: FnTextureBuilderGetStorageAlignment = c"nvnTextureBuilderGetStorageAlignment",
    texture_builder_set_storage: FnTextureBuilderSetStorage = c"nvnTextureBuilderSetStorage",
    texture_builder_set_packaged_texture_data: FnTextureBuilderSetPackagedTextureData = c"nvnTextureBuilderSetPackagedTextureData",
    texture_builder_set_packaged_texture_layout: FnTextureBuilderSetPackagedTextureLayout = c"nvnTextureBuilderSetPackagedTextureLayout",
    texture_builder_set_stride: FnTextureBuilderSetStride = c"nvnTextureBuilderSetStride",
    texture_builder_set_gl_texture_name: FnTextureBuilderSetGLTextureName = c"nvnTextureBuilderSetGLTextureName",
    texture_builder_get_storage_class: FnTextureBuilderGetStorageClass = c"nvnTextureBuilderGetStorageClass",
    texture_builder_get_flags: FnTextureBuilderGetFlags = c"nvnTextureBuilderGetFlags",
    texture_builder_get_target: FnTextureBuilderGetTarget = c"nvnTextureBuilderGetTarget",
    texture_builder_get_width: FnTextureBuilderGetWidth = c"nvnTextureBuilderGetWidth",
    texture_builder_get_height: FnTextureBuilderGetHeight = c"nvnTextureBuilderGetHeight",
    texture_builder_get_depth: FnTextureBuilderGetDepth = c"nvnTextureBuilderGetDepth",
    texture_builder_get_levels: FnTextureBuilderGetLevels = c"nvnTextureBuilderGetLevels",
    texture_builder_get_format: FnTextureBuilderGetFormat = c"nvnTextureBuilderGetFormat",
    texture_builder_get_samples: FnTextureBuilderGetSamples = c"nvnTextureBuilderGetSamples",
    texture_builder_get_swizzle: FnTextureBuilderGetSwizzle = c"nvnTextureBuilderGetSwizzle",
    texture_builder_get_depth_stencil_mode: FnTextureBuilderGetDepthStencilMode = c"nvnTextureBuilderGetDepthStencilMode",
    texture_builder_get_packaged_texture_data: FnTextureBuilderGetPackagedTextureData = c"nvnTextureBuilderGetPackagedTextureData",
    texture_builder_get_stride: FnTextureBuilderGetStride = c"nvnTextureBuilderGetStride",
    texture_builder_get_sparse_tile_layout: FnTextureBuilderGetSparseTileLayout = c"nvnTextureBuilderGetSparseTileLayout",
    texture_builder_get_gl_texture_name: FnTextureBuilderGetGLTextureName = c"nvnTextureBuilderGetGLTextureName",
    texture_builder_get_zcull_storage_size: FnTextureBuilderGetZCullStorageSize = c"nvnTextureBuilderGetZCullStorageSize",
    texture_builder_get_memory_pool: FnTextureBuilderGetMemoryPool = c"nvnTextureBuilderGetMemoryPool",
    texture_builder_get_memory_offset: FnTextureBuilderGetMemoryOffset = c"nvnTextureBuilderGetMemoryOffset",
    texture_view_set_defaults: FnTextureViewSetDefaults = c"nvnTextureViewSetDefaults",
    texture_view_set_levels: FnTextureViewSetLevels = c"nvnTextureViewSetLevels",
    texture_view_set_layers: FnTextureViewSetLayers = c"nvnTextureViewSetLayers",
    texture_view_set_format: FnTextureViewSetFormat = c"nvnTextureViewSetFormat",
    texture_view_set_swizzle: FnTextureViewSetSwizzle = c"nvnTextureViewSetSwizzle",
    texture_view_set_depth_stencil_mode: FnTextureViewSetDepthStencilMode = c"nvnTextureViewSetDepthStencilMode",
    texture_view_set_target: FnTextureViewSetTarget = c"nvnTextureViewSetTarget",
    texture_view_get_levels: FnTextureViewGetLevels = c"nvnTextureViewGetLevels",
    texture_view_get_layers: FnTextureViewGetLayers = c"nvnTextureViewGetLayers",
    texture_view_get_format: FnTextureViewGetFormat = c"nvnTextureViewGetFormat",
    texture_view_get_swizzle: FnTextureViewGetSwizzle = c"nvnTextureViewGetSwizzle",
    texture_view_get_depth_stencil_mode: FnTextureViewGetDepthStencilMode = c"nvnTextureViewGetDepthStencilMode",
    texture_view_get_target: FnTextureViewGetTarget = c"nvnTextureViewGetTarget",
    texture_view_compare: FnTextureViewCompare = c"nvnTextureViewCompare",
    texture_initialize: FnTextureInitialize = c"nvnTextureInitialize",
    texture_get_zcull_storage_size: FnTextureGetZCullStorageSize = c"nvnTextureGetZCullStorageSize",
    texture_finalize: FnTextureFinalize = c"nvnTextureFinalize",
    texture_set_debug_label: FnTextureSetDebugLabel = c"nvnTextureSetDebugLabel",
    texture_get_storage_class: FnTextureGetStorageClass = c"nvnTextureGetStorageClass",
    texture_get_view_offset: FnTextureGetViewOffset = c"nvnTextureGetViewOffset",
    texture_get_flags: FnTextureGetFlags = c"nvnTextureGetFlags",
    texture_get_target: FnTextureGetTarget = c"nvnTextureGetTarget",
    texture_get_width: FnTextureGetWidth = c"nvnTextureGetWidth",
    texture_get_height: FnTextureGetHeight = c"nvnTextureGetHeight",
    texture_get_depth: FnTextureGetDepth = c"nvnTextureGetDepth",
    texture_get_levels: FnTextureGetLevels = c"nvnTextureGetLevels",
    texture_get_format: FnTextureGetFormat = c"nvnTextureGetFormat",
    texture_get_samples: FnTextureGetSamples = c"nvnTextureGetSamples",
    texture_get_swizzle: FnTextureGetSwizzle = c"nvnTextureGetSwizzle",
    texture_get_depth_stencil_mode: FnTextureGetDepthStencilMode = c"nvnTextureGetDepthStencilMode",
    texture_get_stride: FnTextureGetStride = c"nvnTextureGetStride",
    texture_get_texture_address: FnTextureGetTextureAddress = c"nvnTextureGetTextureAddress",
    texture_get_sparse_tile_layout: FnTextureGetSparseTileLayout = c"nvnTextureGetSparseTileLayout",
    texture_write_texels: FnTextureWriteTexels = c"nvnTextureWriteTexels",
    texture_write_texels_strided: FnTextureWriteTexelsStrided = c"nvnTextureWriteTexelsStrided",
    texture_read_texels: FnTextureReadTexels = c"nvnTextureReadTexels",
    texture_read_texels_strided: FnTextureReadTexelsStrided = c"nvnTextureReadTexelsStrided",
    texture_flush_texels: FnTextureFlushTexels = c"nvnTextureFlushTexels",
    texture_invalidate_texels: FnTextureInvalidateTexels = c"nvnTextureInvalidateTexels",
    texture_get_memory_pool: FnTextureGetMemoryPool = c"nvnTextureGetMemoryPool",
    texture_get_memory_offset: FnTextureGetMemoryOffset = c"nvnTextureGetMemoryOffset",
    texture_get_storage_size: FnTextureGetStorageSize = c"nvnTextureGetStorageSize",
    texture_compare: FnTextureCompare = c"nvnTextureCompare",
    texture_get_debug_id: FnTextureGetDebugID = c"nvnTextureGetDebugID",
    sampler_builder_set_device: FnSamplerBuilderSetDevice = c"nvnSamplerBuilderSetDevice",
    sampler_builder_set_defaults: FnSamplerBuilderSetDefaults = c"nvnSamplerBuilderSetDefaults",
    sampler_builder_set_min_mag_filter: FnSamplerBuilderSetMinMagFilter = c"nvnSamplerBuilderSetMinMagFilter",
    sampler_builder_set_wrap_mode: FnSamplerBuilderSetWrapMode = c"nvnSamplerBuilderSetWrapMode",
    sampler_builder_set_lod_clamp: FnSamplerBuilderSetLodClamp = c"nvnSamplerBuilderSetLodClamp",
    sampler_builder_set_lod_bias: FnSamplerBuilderSetLodBias = c"nvnSamplerBuilderSetLodBias",
    sampler_builder_set_compare: FnSamplerBuilderSetCompare = c"nvnSamplerBuilderSetCompare",
    sampler_builder_set_border_color: FnSamplerBuilderSetBorderColor = c"nvnSamplerBuilderSetBorderColor",
    sampler_builder_set_border_colori: FnSamplerBuilderSetBorderColori = c"nvnSamplerBuilderSetBorderColori",
    sampler_builder_set_border_colorui: FnSamplerBuilderSetBorderColorui = c"nvnSamplerBuilderSetBorderColorui",
    sampler_builder_set_max_anisotropy: FnSamplerBuilderSetMaxAnisotropy = c"nvnSamplerBuilderSetMaxAnisotropy",
    sampler_builder_set_reduction_filter: FnSamplerBuilderSetReductionFilter = c"nvnSamplerBuilderSetReductionFilter",
    sampler_builder_set_lod_snap: FnSamplerBuilderSetLodSnap = c"nvnSamplerBuilderSetLodSnap",
    sampler_builder_get_min_mag_filter: FnSamplerBuilderGetMinMagFilter = c"nvnSamplerBuilderGetMinMagFilter",
    sampler_builder_get_wrap_mode: FnSamplerBuilderGetWrapMode = c"nvnSamplerBuilderGetWrapMode",
    sampler_builder_get_lod_clamp: FnSamplerBuilderGetLodClamp = c"nvnSamplerBuilderGetLodClamp",
    sampler_builder_get_lod_bias: FnSamplerBuilderGetLodBias = c"nvnSamplerBuilderGetLodBias",
    sampler_builder_get_compare: FnSamplerBuilderGetCompare = c"nvnSamplerBuilderGetCompare",
    sampler_builder_get_border_color: FnSamplerBuilderGetBorderColor = c"nvnSamplerBuilderGetBorderColor",
    sampler_builder_get_border_colori: FnSamplerBuilderGetBorderColori = c"nvnSamplerBuilderGetBorderColori",
    sampler_builder_get_border_colorui: FnSamplerBuilderGetBorderColorui = c"nvnSamplerBuilderGetBorderColorui",
    sampler_builder_get_max_anisotropy: FnSamplerBuilderGetMaxAnisotropy = c"nvnSamplerBuilderGetMaxAnisotropy",
    sampler_builder_get_reduction_filter: FnSamplerBuilderGetReductionFilter = c"nvnSamplerBuilderGetReductionFilter",
    sampler_builder_get_lod_snap: FnSamplerBuilderGetLodSnap = c"nvnSamplerBuilderGetLodSnap",
    sampler_initialize: FnSamplerInitialize = c"nvnSamplerInitialize",
    sampler_finalize: FnSamplerFinalize = c"nvnSamplerFinalize",
    sampler_set_debug_label: FnSamplerSetDebugLabel = c"nvnSamplerSetDebugLabel",
    sampler_get_min_mag_filter: FnSamplerGetMinMagFilter = c"nvnSamplerGetMinMagFilter",
    sampler_get_wrap_mode: FnSamplerGetWrapMode = c"nvnSamplerGetWrapMode",
    sampler_get_lod_clamp: FnSamplerGetLodClamp = c"nvnSamplerGetLodClamp",
    sampler_get_lod_bias: FnSamplerGetLodBias = c"nvnSamplerGetLodBias",
    sampler_get_compare: FnSamplerGetCompare = c"nvnSamplerGetCompare",
    sampler_get_border_color: FnSamplerGetBorderColor = c"nvnSamplerGetBorderColor",
    sampler_get_border_colori: FnSamplerGetBorderColori = c"nvnSamplerGetBorderColori",
    sampler_get_border_colorui: FnSamplerGetBorderColorui = c"nvnSamplerGetBorderColorui",
    sampler_get_max_anisotropy: FnSamplerGetMaxAnisotropy = c"nvnSamplerGetMaxAnisotropy",
    sampler_get_reduction_filter: FnSamplerGetReductionFilter = c"nvnSamplerGetReductionFilter",
    sampler_compare: FnSamplerCompare = c"nvnSamplerCompare",
    sampler_get_debug_id: FnSamplerGetDebugID = c"nvnSamplerGetDebugID",
    blend_state_set_defaults: FnBlendStateSetDefaults = c"nvnBlendStateSetDefaults",
    blend_state_set_blend_target: FnBlendStateSetBlendTarget = c"nvnBlendStateSetBlendTarget",
    blend_state_set_blend_func: FnBlendStateSetBlendFunc = c"nvnBlendStateSetBlendFunc",
    blend_state_set_blend_equation: FnBlendStateSetBlendEquation = c"nvnBlendStateSetBlendEquation",
    blend_state_set_advanced_mode: FnBlendStateSetAdvancedMode = c"nvnBlendStateSetAdvancedMode",
    blend_state_set_advanced_overlap: FnBlendStateSetAdvancedOverlap = c"nvnBlendStateSetAdvancedOverlap",
    blend_state_set_advanced_premultiplied_src: FnBlendStateSetAdvancedPremultipliedSrc = c"nvnBlendStateSetAdvancedPremultipliedSrc",
    blend_state_set_advanced_normalized_dst: FnBlendStateSetAdvancedNormalizedDst = c"nvnBlendStateSetAdvancedNormalizedDst",
    blend_state_get_blend_target: FnBlendStateGetBlendTarget = c"nvnBlendStateGetBlendTarget",
    blend_state_get_blend_func: FnBlendStateGetBlendFunc = c"nvnBlendStateGetBlendFunc",
    blend_state_get_blend_equation: FnBlendStateGetBlendEquation = c"nvnBlendStateGetBlendEquation",
    blend_state_get_advanced_mode: FnBlendStateGetAdvancedMode = c"nvnBlendStateGetAdvancedMode",
    blend_state_get_advanced_overlap: FnBlendStateGetAdvancedOverlap = c"nvnBlendStateGetAdvancedOverlap",
    blend_state_get_advanced_premultiplied_src: FnBlendStateGetAdvancedPremultipliedSrc = c"nvnBlendStateGetAdvancedPremultipliedSrc",
    blend_state_get_advanced_normalized_dst: FnBlendStateGetAdvancedNormalizedDst = c"nvnBlendStateGetAdvancedNormalizedDst",
    color_state_set_defaults: FnColorStateSetDefaults = c"nvnColorStateSetDefaults",
    color_state_set_blend_enable: FnColorStateSetBlendEnable = c"nvnColorStateSetBlendEnable",
    color_state_set_logic_op: FnColorStateSetLogicOp = c"nvnColorStateSetLogicOp",
    color_state_set_alpha_test: FnColorStateSetAlphaTest = c"nvnColorStateSetAlphaTest",
    color_state_get_blend_enable: FnColorStateGetBlendEnable = c"nvnColorStateGetBlendEnable",
    color_state_get_logic_op: FnColorStateGetLogicOp = c"nvnColorStateGetLogicOp",
    color_state_get_alpha_test: FnColorStateGetAlphaTest = c"nvnColorStateGetAlphaTest",
    channel_mask_state_set_defaults: FnChannelMaskStateSetDefaults = c"nvnChannelMaskStateSetDefaults",
    channel_mask_state_set_channel_mask: FnChannelMaskStateSetChannelMask = c"nvnChannelMaskStateSetChannelMask",
    channel_mask_state_get_channel_mask: FnChannelMaskStateGetChannelMask = c"nvnChannelMaskStateGetChannelMask",
    multisample_state_set_defaults: FnMultisampleStateSetDefaults = c"nvnMultisampleStateSetDefaults",
    multisample_state_set_multisample_enable: FnMultisampleStateSetMultisampleEnable = c"nvnMultisampleStateSetMultisampleEnable",
    multisample_state_set_samples: FnMultisampleStateSetSamples = c"nvnMultisampleStateSetSamples",
    multisample_state_set_alpha_to_coverage_enable: FnMultisampleStateSetAlphaToCoverageEnable = c"nvnMultisampleStateSetAlphaToCoverageEnable",
    multisample_state_set_alpha_to_coverage_dither: FnMultisampleStateSetAlphaToCoverageDither = c"nvnMultisampleStateSetAlphaToCoverageDither",
    multisample_state_get_multisample_enable: FnMultisampleStateGetMultisampleEnable = c"nvnMultisampleStateGetMultisampleEnable",
    multisample_state_get_samples: FnMultisampleStateGetSamples = c"nvnMultisampleStateGetSamples",
    multisample_state_get_alpha_to_coverage_enable: FnMultisampleStateGetAlphaToCoverageEnable = c"nvnMultisampleStateGetAlphaToCoverageEnable",
    multisample_state_get_alpha_to_coverage_dither: FnMultisampleStateGetAlphaToCoverageDither = c"nvnMultisampleStateGetAlphaToCoverageDither",
    multisample_state_set_raster_samples: FnMultisampleStateSetRasterSamples = c"nvnMultisampleStateSetRasterSamples",
    multisample_state_get_raster_samples: FnMultisampleStateGetRasterSamples = c"nvnMultisampleStateGetRasterSamples",
    multisample_state_set_coverage_modulation_mode: FnMultisampleStateSetCoverageModulationMode = c"nvnMultisampleStateSetCoverageModulationMode",
    multisample_state_get_coverage_modulation_mode: FnMultisampleStateGetCoverageModulationMode = c"nvnMultisampleStateGetCoverageModulationMode",
    multisample_state_set_coverage_to_color_enable: FnMultisampleStateSetCoverageToColorEnable = c"nvnMultisampleStateSetCoverageToColorEnable",
    multisample_state_get_coverage_to_color_enable: FnMultisampleStateGetCoverageToColorEnable = c"nvnMultisampleStateGetCoverageToColorEnable",
    multisample_state_set_coverage_to_color_output: FnMultisampleStateSetCoverageToColorOutput = c"nvnMultisampleStateSetCoverageToColorOutput",
    multisample_state_get_coverage_to_color_output: FnMultisampleStateGetCoverageToColorOutput = c"nvnMultisampleStateGetCoverageToColorOutput",
    multisample_state_set_sample_locations_enable: FnMultisampleStateSetSampleLocationsEnable = c"nvnMultisampleStateSetSampleLocationsEnable",
    multisample_state_get_sample_locations_enable: FnMultisampleStateGetSampleLocationsEnable = c"nvnMultisampleStateGetSampleLocationsEnable",
    multisample_state_get_sample_locations_grid: FnMultisampleStateGetSampleLocationsGrid = c"nvnMultisampleStateGetSampleLocationsGrid",
    multisample_state_set_sample_locations_grid_enable: FnMultisampleStateSetSampleLocationsGridEnable = c"nvnMultisampleStateSetSampleLocationsGridEnable",
    multisample_state_get_sample_locations_grid_enable: FnMultisampleStateGetSampleLocationsGridEnable = c"nvnMultisampleStateGetSampleLocationsGridEnable",
    multisample_state_set_sample_locations: FnMultisampleStateSetSampleLocations = c"nvnMultisampleStateSetSampleLocations",
    polygon_state_set_defaults: FnPolygonStateSetDefaults = c"nvnPolygonStateSetDefaults",
    polygon_state_set_cull_face: FnPolygonStateSetCullFace = c"nvnPolygonStateSetCullFace",
    polygon_state_set_front_face: FnPolygonStateSetFrontFace = c"nvnPolygonStateSetFrontFace",
    polygon_state_set_polygon_mode: FnPolygonStateSetPolygonMode = c"nvnPolygonStateSetPolygonMode",
    polygon_state_set_polygon_offset_enables: FnPolygonStateSetPolygonOffsetEnables = c"nvnPolygonStateSetPolygonOffsetEnables",
    polygon_state_get_cull_face: FnPolygonStateGetCullFace = c"nvnPolygonStateGetCullFace",
    polygon_state_get_front_face: FnPolygonStateGetFrontFace = c"nvnPolygonStateGetFrontFace",
    polygon_state_get_polygon_mode: FnPolygonStateGetPolygonMode = c"nvnPolygonStateGetPolygonMode",
    polygon_state_get_polygon_offset_enables: FnPolygonStateGetPolygonOffsetEnables = c"nvnPolygonStateGetPolygonOffsetEnables",
    depth_stencil_state_set_defaults: FnDepthStencilStateSetDefaults = c"nvnDepthStencilStateSetDefaults",
    depth_stencil_state_set_depth_test_enable: FnDepthStencilStateSetDepthTestEnable = c"nvnDepthStencilStateSetDepthTestEnable",
    depth_stencil_state_set_depth_write_enable: FnDepthStencilStateSetDepthWriteEnable = c"nvnDepthStencilStateSetDepthWriteEnable",
    depth_stencil_state_set_depth_func: FnDepthStencilStateSetDepthFunc = c"nvnDepthStencilStateSetDepthFunc",
    depth_stencil_state_set_stencil_test_enable: FnDepthStencilStateSetStencilTestEnable = c"nvnDepthStencilStateSetStencilTestEnable",
    depth_stencil_state_set_stencil_func: FnDepthStencilStateSetStencilFunc = c"nvnDepthStencilStateSetStencilFunc",
    depth_stencil_state_set_stencil_op: FnDepthStencilStateSetStencilOp = c"nvnDepthStencilStateSetStencilOp",
    depth_stencil_state_get_depth_test_enable: FnDepthStencilStateGetDepthTestEnable = c"nvnDepthStencilStateGetDepthTestEnable",
    depth_stencil_state_get_depth_write_enable: FnDepthStencilStateGetDepthWriteEnable = c"nvnDepthStencilStateGetDepthWriteEnable",
    depth_stencil_state_get_depth_func: FnDepthStencilStateGetDepthFunc = c"nvnDepthStencilStateGetDepthFunc",
    depth_stencil_state_get_stencil_test_enable: FnDepthStencilStateGetStencilTestEnable = c"nvnDepthStencilStateGetStencilTestEnable",
    depth_stencil_state_get_stencil_func: FnDepthStencilStateGetStencilFunc = c"nvnDepthStencilStateGetStencilFunc",
    depth_stencil_state_get_stencil_op: FnDepthStencilStateGetStencilOp = c"nvnDepthStencilStateGetStencilOp",
    vertex_attrib_state_set_defaults: FnVertexAttribStateSetDefaults = c"nvnVertexAttribStateSetDefaults",
    vertex_attrib_state_set_format: FnVertexAttribStateSetFormat = c"nvnVertexAttribStateSetFormat",
    vertex_attrib_state_set_stream_index: FnVertexAttribStateSetStreamIndex = c"nvnVertexAttribStateSetStreamIndex",
    vertex_attrib_state_get_format: FnVertexAttribStateGetFormat = c"nvnVertexAttribStateGetFormat",
    vertex_attrib_state_get_stream_index: FnVertexAttribStateGetStreamIndex = c"nvnVertexAttribStateGetStreamIndex",
    vertex_stream_state_set_defaults: FnVertexStreamStateSetDefaults = c"nvnVertexStreamStateSetDefaults",
    vertex_stream_state_set_stride: FnVertexStreamStateSetStride = c"nvnVertexStreamStateSetStride",
    vertex_stream_state_set_divisor: FnVertexStreamStateSetDivisor = c"nvnVertexStreamStateSetDivisor",
    vertex_stream_state_get_stride: FnVertexStreamStateGetStride = c"nvnVertexStreamStateGetStride",
    vertex_stream_state_get_divisor: FnVertexStreamStateGetDivisor = c"nvnVertexStreamStateGetDivisor",
    command_buffer_initialize: FnCommandBufferInitialize = c"nvnCommandBufferInitialize",
    command_buffer_finalize: FnCommandBufferFinalize = c"nvnCommandBufferFinalize",
    command_buffer_set_debug_label: FnCommandBufferSetDebugLabel = c"nvnCommandBufferSetDebugLabel",
    command_buffer_set_memory_callback: FnCommandBufferSetMemoryCallback = c"nvnCommandBufferSetMemoryCallback",
    command_buffer_set_memory_callback_data: FnCommandBufferSetMemoryCallbackData = c"nvnCommandBufferSetMemoryCallbackData",
    command_buffer_add_command_memory: FnCommandBufferAddCommandMemory = c"nvnCommandBufferAddCommandMemory",
    command_buffer_add_control_memory: FnCommandBufferAddControlMemory = c"nvnCommandBufferAddControlMemory",
    command_buffer_get_command_memory_size: FnCommandBufferGetCommandMemorySize = c"nvnCommandBufferGetCommandMemorySize",
    command_buffer_get_command_memory_used: FnCommandBufferGetCommandMemoryUsed = c"nvnCommandBufferGetCommandMemoryUsed",
    command_buffer_get_command_memory_free: FnCommandBufferGetCommandMemoryFree = c"nvnCommandBufferGetCommandMemoryFree",
    command_buffer_get_control_memory_size: FnCommandBufferGetControlMemorySize = c"nvnCommandBufferGetControlMemorySize",
    command_buffer_get_control_memory_used: FnCommandBufferGetControlMemoryUsed = c"nvnCommandBufferGetControlMemoryUsed",
    command_buffer_get_control_memory_free: FnCommandBufferGetControlMemoryFree = c"nvnCommandBufferGetControlMemoryFree",
    command_buffer_begin_recording: FnCommandBufferBeginRecording = c"nvnCommandBufferBeginRecording",
    command_buffer_end_recording: FnCommandBufferEndRecording = c"nvnCommandBufferEndRecording",
    command_buffer_call_commands: FnCommandBufferCallCommands = c"nvnCommandBufferCallCommands",
    command_buffer_copy_commands: FnCommandBufferCopyCommands = c"nvnCommandBufferCopyCommands",
    command_buffer_bind_blend_state: FnCommandBufferBindBlendState = c"nvnCommandBufferBindBlendState",
    command_buffer_bind_channel_mask_state: FnCommandBufferBindChannelMaskState = c"nvnCommandBufferBindChannelMaskState",
    command_buffer_bind_color_state: FnCommandBufferBindColorState = c"nvnCommandBufferBindColorState",
    command_buffer_bind_multisample_state: FnCommandBufferBindMultisampleState = c"nvnCommandBufferBindMultisampleState",
    command_buffer_bind_polygon_state: FnCommandBufferBindPolygonState = c"nvnCommandBufferBindPolygonState",
    command_buffer_bind_depth_stencil_state: FnCommandBufferBindDepthStencilState = c"nvnCommandBufferBindDepthStencilState",
    command_buffer_bind_vertex_attrib_state: FnCommandBufferBindVertexAttribState = c"nvnCommandBufferBindVertexAttribState",
    command_buffer_bind_vertex_stream_state: FnCommandBufferBindVertexStreamState = c"nvnCommandBufferBindVertexStreamState",
    command_buffer_bind_program: FnCommandBufferBindProgram = c"nvnCommandBufferBindProgram",
    command_buffer_bind_vertex_buffer: FnCommandBufferBindVertexBuffer = c"nvnCommandBufferBindVertexBuffer",
    command_buffer_bind_vertex_buffers: FnCommandBufferBindVertexBuffers = c"nvnCommandBufferBindVertexBuffers",
    command_buffer_bind_uniform_buffer: FnCommandBufferBindUniformBuffer = c"nvnCommandBufferBindUniformBuffer",
    command_buffer_bind_uniform_buffers: FnCommandBufferBindUniformBuffers = c"nvnCommandBufferBindUniformBuffers",
    command_buffer_bind_transform_feedback_buffer: FnCommandBufferBindTransformFeedbackBuffer = c"nvnCommandBufferBindTransformFeedbackBuffer",
    command_buffer_bind_transform_feedback_buffers: FnCommandBufferBindTransformFeedbackBuffers = c"nvnCommandBufferBindTransformFeedbackBuffers",
    command_buffer_bind_storage_buffer: FnCommandBufferBindStorageBuffer = c"nvnCommandBufferBindStorageBuffer",
    command_buffer_bind_storage_buffers: FnCommandBufferBindStorageBuffers = c"nvnCommandBufferBindStorageBuffers",
    command_buffer_bind_texture: FnCommandBufferBindTexture = c"nvnCommandBufferBindTexture",
    command_buffer_bind_textures: FnCommandBufferBindTextures = c"nvnCommandBufferBindTextures",
    command_buffer_bind_image: FnCommandBufferBindImage = c"nvnCommandBufferBindImage",
    command_buffer_bind_images: FnCommandBufferBindImages = c"nvnCommandBufferBindImages",
    command_buffer_set_patch_size: FnCommandBufferSetPatchSize = c"nvnCommandBufferSetPatchSize",
    command_buffer_set_inner_tessellation_levels: FnCommandBufferSetInnerTessellationLevels = c"nvnCommandBufferSetInnerTessellationLevels",
    command_buffer_set_outer_tessellation_levels: FnCommandBufferSetOuterTessellationLevels = c"nvnCommandBufferSetOuterTessellationLevels",
    command_buffer_set_primitive_restart: FnCommandBufferSetPrimitiveRestart = c"nvnCommandBufferSetPrimitiveRestart",
    command_buffer_begin_transform_feedback: FnCommandBufferBeginTransformFeedback = c"nvnCommandBufferBeginTransformFeedback",
    command_buffer_end_transform_feedback: FnCommandBufferEndTransformFeedback = c"nvnCommandBufferEndTransformFeedback",
    command_buffer_pause_transform_feedback: FnCommandBufferPauseTransformFeedback = c"nvnCommandBufferPauseTransformFeedback",
    command_buffer_resume_transform_feedback: FnCommandBufferResumeTransformFeedback = c"nvnCommandBufferResumeTransformFeedback",
    command_buffer_draw_transform_feedback: FnCommandBufferDrawTransformFeedback = c"nvnCommandBufferDrawTransformFeedback",
    command_buffer_draw_arrays: FnCommandBufferDrawArrays = c"nvnCommandBufferDrawArrays",
    command_buffer_draw_elements: FnCommandBufferDrawElements = c"nvnCommandBufferDrawElements",
    command_buffer_draw_elements_base_vertex: FnCommandBufferDrawElementsBaseVertex = c"nvnCommandBufferDrawElementsBaseVertex",
    command_buffer_draw_arrays_instanced: FnCommandBufferDrawArraysInstanced = c"nvnCommandBufferDrawArraysInstanced",
    command_buffer_draw_elements_instanced: FnCommandBufferDrawElementsInstanced = c"nvnCommandBufferDrawElementsInstanced",
    command_buffer_draw_arrays_indirect: FnCommandBufferDrawArraysIndirect = c"nvnCommandBufferDrawArraysIndirect",
    command_buffer_draw_elements_indirect: FnCommandBufferDrawElementsIndirect = c"nvnCommandBufferDrawElementsIndirect",
    command_buffer_multi_draw_arrays_indirect_count: FnCommandBufferMultiDrawArraysIndirectCount = c"nvnCommandBufferMultiDrawArraysIndirectCount",
    command_buffer_multi_draw_elements_indirect_count: FnCommandBufferMultiDrawElementsIndirectCount = c"nvnCommandBufferMultiDrawElementsIndirectCount",
    command_buffer_clear_color: FnCommandBufferClearColor = c"nvnCommandBufferClearColor",
    command_buffer_clear_colori: FnCommandBufferClearColori = c"nvnCommandBufferClearColori",
    command_buffer_clear_colorui: FnCommandBufferClearColorui = c"nvnCommandBufferClearColorui",
    command_buffer_clear_depth_stencil: FnCommandBufferClearDepthStencil = c"nvnCommandBufferClearDepthStencil",
    command_buffer_dispatch_compute: FnCommandBufferDispatchCompute = c"nvnCommandBufferDispatchCompute",
    command_buffer_dispatch_compute_indirect: FnCommandBufferDispatchComputeIndirect = c"nvnCommandBufferDispatchComputeIndirect",
    command_buffer_set_viewport: FnCommandBufferSetViewport = c"nvnCommandBufferSetViewport",
    command_buffer_set_viewports: FnCommandBufferSetViewports = c"nvnCommandBufferSetViewports",
    command_buffer_set_viewport_swizzles: FnCommandBufferSetViewportSwizzles = c"nvnCommandBufferSetViewportSwizzles",
    command_buffer_set_scissor: FnCommandBufferSetScissor = c"nvnCommandBufferSetScissor",
    command_buffer_set_scissors: FnCommandBufferSetScissors = c"nvnCommandBufferSetScissors",
    command_buffer_set_depth_range: FnCommandBufferSetDepthRange = c"nvnCommandBufferSetDepthRange",
    command_buffer_set_depth_bounds: FnCommandBufferSetDepthBounds = c"nvnCommandBufferSetDepthBounds",
    command_buffer_set_depth_ranges: FnCommandBufferSetDepthRanges = c"nvnCommandBufferSetDepthRanges",
    command_buffer_set_tiled_cache_action: FnCommandBufferSetTiledCacheAction = c"nvnCommandBufferSetTiledCacheAction",
    command_buffer_set_tiled_cache_tile_size: FnCommandBufferSetTiledCacheTileSize = c"nvnCommandBufferSetTiledCacheTileSize",
    command_buffer_bind_separate_texture: FnCommandBufferBindSeparateTexture = c"nvnCommandBufferBindSeparateTexture",
    command_buffer_bind_separate_sampler: FnCommandBufferBindSeparateSampler = c"nvnCommandBufferBindSeparateSampler",
    command_buffer_bind_separate_textures: FnCommandBufferBindSeparateTextures = c"nvnCommandBufferBindSeparateTextures",
    command_buffer_bind_separate_samplers: FnCommandBufferBindSeparateSamplers = c"nvnCommandBufferBindSeparateSamplers",
    command_buffer_set_stencil_value_mask: FnCommandBufferSetStencilValueMask = c"nvnCommandBufferSetStencilValueMask",
    command_buffer_set_stencil_mask: FnCommandBufferSetStencilMask = c"nvnCommandBufferSetStencilMask",
    command_buffer_set_stencil_ref: FnCommandBufferSetStencilRef = c"nvnCommandBufferSetStencilRef",
    command_buffer_set_blend_color: FnCommandBufferSetBlendColor = c"nvnCommandBufferSetBlendColor",
    command_buffer_set_point_size: FnCommandBufferSetPointSize = c"nvnCommandBufferSetPointSize",
    command_buffer_set_line_width: FnCommandBufferSetLineWidth = c"nvnCommandBufferSetLineWidth",
    command_buffer_set_polygon_offset_clamp: FnCommandBufferSetPolygonOffsetClamp = c"nvnCommandBufferSetPolygonOffsetClamp",
    command_buffer_set_alpha_ref: FnCommandBufferSetAlphaRef = c"nvnCommandBufferSetAlphaRef",
    command_buffer_set_sample_mask: FnCommandBufferSetSampleMask = c"nvnCommandBufferSetSampleMask",
    command_buffer_set_rasterizer_discard: FnCommandBufferSetRasterizerDiscard = c"nvnCommandBufferSetRasterizerDiscard",
    command_buffer_set_depth_clamp: FnCommandBufferSetDepthClamp = c"nvnCommandBufferSetDepthClamp",
    command_buffer_set_conservative_raster_enable: FnCommandBufferSetConservativeRasterEnable = c"nvnCommandBufferSetConservativeRasterEnable",
    command_buffer_set_conservative_raster_dilate: FnCommandBufferSetConservativeRasterDilate = c"nvnCommandBufferSetConservativeRasterDilate",
    command_buffer_set_subpixel_precision_bias: FnCommandBufferSetSubpixelPrecisionBias = c"nvnCommandBufferSetSubpixelPrecisionBias",
    command_buffer_copy_buffer_to_texture: FnCommandBufferCopyBufferToTexture = c"nvnCommandBufferCopyBufferToTexture",
    command_buffer_copy_texture_to_buffer: FnCommandBufferCopyTextureToBuffer = c"nvnCommandBufferCopyTextureToBuffer",
    command_buffer_copy_texture_to_texture: FnCommandBufferCopyTextureToTexture = c"nvnCommandBufferCopyTextureToTexture",
    command_buffer_copy_buffer_to_buffer: FnCommandBufferCopyBufferToBuffer = c"nvnCommandBufferCopyBufferToBuffer",
    command_buffer_clear_buffer: FnCommandBufferClearBuffer = c"nvnCommandBufferClearBuffer",
    command_buffer_clear_texture: FnCommandBufferClearTexture = c"nvnCommandBufferClearTexture",
    command_buffer_clear_texturei: FnCommandBufferClearTexturei = c"nvnCommandBufferClearTexturei",
    command_buffer_clear_textureui: FnCommandBufferClearTextureui = c"nvnCommandBufferClearTextureui",
    command_buffer_update_uniform_buffer: FnCommandBufferUpdateUniformBuffer = c"nvnCommandBufferUpdateUniformBuffer",
    command_buffer_report_counter: FnCommandBufferReportCounter = c"nvnCommandBufferReportCounter",
    command_buffer_reset_counter: FnCommandBufferResetCounter = c"nvnCommandBufferResetCounter",
    command_buffer_report_value: FnCommandBufferReportValue = c"nvnCommandBufferReportValue",
    command_buffer_set_render_enable: FnCommandBufferSetRenderEnable = c"nvnCommandBufferSetRenderEnable",
    command_buffer_set_render_enable_conditional: FnCommandBufferSetRenderEnableConditional = c"nvnCommandBufferSetRenderEnableConditional",
    command_buffer_set_render_targets: FnCommandBufferSetRenderTargets = c"nvnCommandBufferSetRenderTargets",
    command_buffer_discard_color: FnCommandBufferDiscardColor = c"nvnCommandBufferDiscardColor",
    command_buffer_discard_depth_stencil: FnCommandBufferDiscardDepthStencil = c"nvnCommandBufferDiscardDepthStencil",
    command_buffer_downsample: FnCommandBufferDownsample = c"nvnCommandBufferDownsample",
    command_buffer_tiled_downsample: FnCommandBufferTiledDownsample = c"nvnCommandBufferTiledDownsample",
    command_buffer_downsample_texture_view: FnCommandBufferDownsampleTextureView = c"nvnCommandBufferDownsampleTextureView",
    command_buffer_tiled_downsample_texture_view: FnCommandBufferTiledDownsampleTextureView = c"nvnCommandBufferTiledDownsampleTextureView",
    command_buffer_barrier: FnCommandBufferBarrier = c"nvnCommandBufferBarrier",
    command_buffer_wait_sync: FnCommandBufferWaitSync = c"nvnCommandBufferWaitSync",
    command_buffer_fence_sync: FnCommandBufferFenceSync = c"nvnCommandBufferFenceSync",
    command_buffer_set_texture_pool: FnCommandBufferSetTexturePool = c"nvnCommandBufferSetTexturePool",
    command_buffer_set_sampler_pool: FnCommandBufferSetSamplerPool = c"nvnCommandBufferSetSamplerPool",
    command_buffer_set_shader_scratch_memory: FnCommandBufferSetShaderScratchMemory = c"nvnCommandBufferSetShaderScratchMemory",
    command_buffer_save_zcull_data: FnCommandBufferSaveZCullData = c"nvnCommandBufferSaveZCullData",
    command_buffer_restore_zcull_data: FnCommandBufferRestoreZCullData = c"nvnCommandBufferRestoreZCullData",
    command_buffer_set_copy_row_stride: FnCommandBufferSetCopyRowStride = c"nvnCommandBufferSetCopyRowStride",
    command_buffer_set_copy_image_stride: FnCommandBufferSetCopyImageStride = c"nvnCommandBufferSetCopyImageStride",
    command_buffer_get_copy_row_stride: FnCommandBufferGetCopyRowStride = c"nvnCommandBufferGetCopyRowStride",
    command_buffer_get_copy_image_stride: FnCommandBufferGetCopyImageStride = c"nvnCommandBufferGetCopyImageStride",
    command_buffer_draw_texture: FnCommandBufferDrawTexture = c"nvnCommandBufferDrawTexture",
    program_set_subroutine_linkage: FnProgramSetSubroutineLinkage = c"nvnProgramSetSubroutineLinkage",
    command_buffer_set_program_subroutines: FnCommandBufferSetProgramSubroutines = c"nvnCommandBufferSetProgramSubroutines",
    command_buffer_bind_coverage_modulation_table: FnCommandBufferBindCoverageModulationTable = c"nvnCommandBufferBindCoverageModulationTable",
    command_buffer_resolve_depth_buffer: FnCommandBufferResolveDepthBuffer = c"nvnCommandBufferResolveDepthBuffer",
    command_buffer_push_debug_group_static: FnCommandBufferPushDebugGroupStatic = c"nvnCommandBufferPushDebugGroupStatic",
    command_buffer_push_debug_group_dynamic: FnCommandBufferPushDebugGroupDynamic = c"nvnCommandBufferPushDebugGroupDynamic",
    command_buffer_push_debug_group: FnCommandBufferPushDebugGroup = c"nvnCommandBufferPushDebugGroup",
    command_buffer_pop_debug_group: FnCommandBufferPopDebugGroup = c"nvnCommandBufferPopDebugGroup",
    command_buffer_pop_debug_group_id: FnCommandBufferPopDebugGroupId = c"nvnCommandBufferPopDebugGroupId",
    command_buffer_insert_debug_marker_static: FnCommandBufferInsertDebugMarkerStatic = c"nvnCommandBufferInsertDebugMarkerStatic",
    command_buffer_insert_debug_marker_dynamic: FnCommandBufferInsertDebugMarkerDynamic = c"nvnCommandBufferInsertDebugMarkerDynamic",
    command_buffer_insert_debug_marker: FnCommandBufferInsertDebugMarker = c"nvnCommandBufferInsertDebugMarker",
    command_buffer_get_memory_callback: FnCommandBufferGetMemoryCallback = c"nvnCommandBufferGetMemoryCallback",
    command_buffer_get_memory_callback_data: FnCommandBufferGetMemoryCallbackData = c"nvnCommandBufferGetMemoryCallbackData",
    command_buffer_is_recording: FnCommandBufferIsRecording = c"nvnCommandBufferIsRecording",
    sync_initialize: FnSyncInitialize = c"nvnSyncInitialize",
    sync_finalize: FnSyncFinalize = c"nvnSyncFinalize",
    sync_set_debug_label: FnSyncSetDebugLabel = c"nvnSyncSetDebugLabel",
    queue_fence_sync: FnQueueFenceSync = c"nvnQueueFenceSync",
    sync_wait: FnSyncWait = c"nvnSyncWait",
    queue_wait_sync: FnQueueWaitSync = c"nvnQueueWaitSync",
    event_builder_set_defaults: FnEventBuilderSetDefaults = c"nvnEventBuilderSetDefaults",
    event_builder_set_storage: FnEventBuilderSetStorage = c"nvnEventBuilderSetStorage",
    event_initialize: FnEventInitialize = c"nvnEventInitialize",
    event_finalize: FnEventFinalize = c"nvnEventFinalize",
    event_get_value: FnEventGetValue = c"nvnEventGetValue",
    event_signal: FnEventSignal = c"nvnEventSignal",
    command_buffer_wait_event: FnCommandBufferWaitEvent = c"nvnCommandBufferWaitEvent",
    command_buffer_signal_event: FnCommandBufferSignalEvent = c"nvnCommandBufferSignalEvent",
}
