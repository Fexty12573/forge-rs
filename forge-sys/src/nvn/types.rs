use core::ffi::c_void;
use core::mem::MaybeUninit;

macro_rules! opaque_struct {
    ($name:ident, $size:expr) => {
        #[repr(C)]
        pub struct $name {
            _reserved: MaybeUninit<[u8; $size]>,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    _reserved: MaybeUninit::uninit(),
                }
            }
        }
    };
}

pub type NVNtextureHandle = u64;
pub type NVNimageHandle = u64;
pub type NVNcommandHandle = u64;
pub type NVNbufferAddress = u64;
pub type NVNtextureAddress = u64;
pub type NVNdebugDomainId = i32;
pub type NVNstorageClass = i32;
pub type NVNsubroutineLinkageMapPtr = *mut c_void;
pub type NVNnativeWindow = *mut c_void;

opaque_struct!(NVNdevice, 0x3000);
opaque_struct!(NVNdeviceBuilder, 0x40);
opaque_struct!(NVNtextureSparseTileLayout, 0x1); // TODO
opaque_struct!(NVNdrawTextureRegion, 0x1); // TODO
opaque_struct!(NVNmultisampleState, 0x18);
opaque_struct!(NVNdepthStencilState, 0x8);
opaque_struct!(NVNchannelMaskState, 0x4);
opaque_struct!(NVNpolygonState, 0x4);
#[repr(C)]
pub struct NVNbufferRange {
    pub address: u64,
    pub size: u64,
}
opaque_struct!(NVNcolorState, 0x4);
#[repr(C)]
pub struct NVNcounterData {
    pub counter: u64,
    pub timestamp: u64,
}
#[repr(C)]
pub struct NVNseparateTextureHandle {
    pub value: u64,
}
#[repr(C)]
pub struct NVNseparateSamplerHandle {
    pub value: u64,
}
opaque_struct!(NVNqueue, 0x2000);
opaque_struct!(NVNqueueErrorInfo, 0x1); // TODO
opaque_struct!(NVNqueueBuilder, 0x40);
opaque_struct!(NVNwindow, 0x180);
opaque_struct!(NVNtexture, 0xC0);
opaque_struct!(NVNtextureView, 0x28);
opaque_struct!(NVNcommandBuffer, 0xA0);
opaque_struct!(NVNsync, 0x40);
#[repr(C)]
pub struct NVNrectangle {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
opaque_struct!(NVNprogram, 0xC0);
#[repr(C)]
pub struct NVNshaderData {
    pub data: u64,
    pub control: *const c_void,
}
opaque_struct!(NVNmemoryPool, 0x100);
opaque_struct!(NVNtexturePool, 0x20);
opaque_struct!(NVNsamplerPool, 0x20);
opaque_struct!(NVNblendState, 0x8);
opaque_struct!(NVNvertexAttribState, 0x4);
opaque_struct!(NVNvertexStreamState, 0x8);
opaque_struct!(NVNmemoryPoolBuilder, 0x40);
opaque_struct!(NVNmappingRequest, 0x1); // TODO
opaque_struct!(NVNsamplerBuilder, 0x60);
opaque_struct!(NVNsampler, 0x30);
opaque_struct!(NVNbufferBuilder, 0x40);
opaque_struct!(NVNbuffer, 0x30);
opaque_struct!(NVNevent, 0x40);
opaque_struct!(NVNeventBuilder, 0x20);
#[repr(C)]
pub struct NVNcopyRegion {
    pub xoff: i32,
    pub yoff: i32,
    pub zoff: i32,
    pub w: i32,
    pub h: i32,
    pub d: i32,
}
opaque_struct!(NVNwindowBuilder, 0x40);
opaque_struct!(NVNtextureBuilder, 0x80);
opaque_struct!(NVNpackagedTextureLayout, 0x8);

#[repr(i32)]
pub enum NVNdeviceInfo {
    ApiMajorVersion = 0,
    ApiMinorVersion,
    UniformBufferBindingsPerStage,
    MaxUniformBufferSize,
    UniformBufferAlignment,
    ColorBufferBindings,
    VertexBufferBindings,
    TransformFeedbackBufferBindings,
    ShaderStorageBufferBindingsPerStage,
    TextureBindingsPerStage,
    CounterAlignment,
    TransformFeedbackBufferAlignment,
    TransformFeedbackControlAlignment,
    IndirectDrawAlignment,
    VertexAttributes,
    TextureDescriptorSize,
    SamplerDescriptorSize,
    ReservedTextureDescriptors,
    ReservedSamplerDescriptors,
    CommandBufferCommandAlignment,
    CommandBufferControlAlignment,
    CommandBufferMinCommandSize,
    CommandBufferMinControlSize,
    ShaderScratchMemoryScaleFactorMinimum,
    ShaderScratchMemoryScaleFactorRecommended,
    ShaderScratchMemoryAlignment,
    ShaderScratchMemoryGranularity,
    MaxTextureAnisotropy,
    MaxComputeWorkGroupSizeX,
    MaxComputeWorkGroupSizeY,
    MaxComputeWorkGroupSizeZ,
    MaxComputeWorkGroupSizeThreads,
    MaxComputeDispatchWorkGroupsX,
    MaxComputeDispatchWorkGroupsY,
    MaxComputeDispatchWorkGroupsZ,
    ImageBindingsPerStage,
    MaxTexturePoolSize,
    MaxSamplerPoolSize,
    MaxViewports,
    MempoolTextureObjectPageAlignment,
    SupportsMinMaxFiltering,
    SupportsStencil8Format,
    SupportsAstcFormats,
    L2Size,
    MaxTextureLevels,
    MaxTextureLayers,
    GlslcMaxSupportedGpuCodeMajorVersion,
    GlslcMinSupportedGpuCodeMajorVersion,
    GlslcMaxSupportedGpuCodeMinorVersion,
    GlslcMinSupportedGpuCodeMinorVersion,
    SupportsConservativeRaster,
    SubpixelBits,
    MaxSubpixelBiasBits,
    IndirectDispatchAlignment,
    ZCullSaveRestoreAlignment,
    ShaderScratchMemoryComputeScaleFactorMinimum,
    LinearTextureStrideAlignment,
    LinearRenderTargetStrideAlignment,
    MemoryPoolPageSize,
    SupportsZeroFromUnmappedVirtualPoolPages,
    UniformBufferUpdateAlignment,
    MaxTextureSize,
    MaxBufferTextureSize,
    Max3DTextureSize,
    MaxCubeMapTextureSize,
    MaxRectangleTextureSize,
    SupportsPassthroughGeometryShaders,
    SupportsViewportSwizzle,
    SupportsSparseTiledPackagedTextures,
    SupportsAdvancedBlendModes,
    MaxPresentInterval,
    SupportsDrawTexture,
    SupportsTargetIndependentRasterization,
    SupportsFragmentCoverageToColor,
    SupportsPostDepthCoverage,
    SupportsImageUsingTextureHandles,
    SupportsSampleLocations,
    MaxSampleLocationTableEntries,
    ShaderCodeMemoryPoolPaddingSize,
    MaxPatchSize,
    QueueCommandMemoryGranularity,
    QueueCommandMemoryMinSize,
    QueueCommandMemoryDefaultSize,
    QueueComputeMemoryGranularity,
    QueueComputeMemoryMinSize,
    QueueComputeMemoryDefaultSize,
    QueueCommandMemoryMinFlushThreshold,
    SupportsFragmentShaderInterlock,
    MaxTexturesPerWindow,
    MinTexturesPerWindow,
    SupportsDebugLayer,
    QueueControlMemoryMinSize,
    QueueControlMemoryDefaultSize,
    QueueControlMemoryGranularity,
    SeparateTextureBindingsPerStage,
    SeparateSamplerBindingsPerStage,
    DebugGroupsMaxDomainId,
    EventsSupportReductionOperations,
}

pub type NVNdeviceFlagBits_ = i32;
pub struct NVNdeviceFlagBits;
impl NVNdeviceFlagBits {
    pub const DEBUG_ENABLE: i32 = 1 << 0;
    pub const DEBUG_SKIP_CALLS_ON_ERROR: i32 = 1 << 1;
    pub const DEBUG_DRAW_VALIDATION: i32 = 1 << 2;
    pub const DEFERRED_FINALIZE: i32 = 1 << 3;
    pub const DEBUG_DRAW_VALIDATION_HEAVY: i32 = 1 << 4;
    pub const DEBUG_ENABLE_LEVEL_0: i32 = 1 << 5;
    pub const DEBUG_ENABLE_LEVEL_1: i32 = 1 << 6;
    pub const DEBUG_ENABLE_LEVEL_2: i32 = 1 << 0;
    pub const DEBUG_ENABLE_LEVEL_3: i32 = 1 << 2;
    pub const DEBUG_ENABLE_LEVEL_4: i32 = 1 << 4;
    pub const DEFERRED_FIRMWARE_MEMORY_RECLAIM: i32 = 1 << 7;
    pub const ENABLE_SEPARATE_SAMPLER_TEXTURE_SUPPORT: i32 = 1 << 8;
}

pub type NVNclearColorMask_ = i32;
pub struct NVNclearColorMask;
impl NVNclearColorMask {
    pub const R_BIT: i32 = 0x1;
    pub const G_BIT: i32 = 0x2;
    pub const B_BIT: i32 = 0x4;
    pub const A_BIT: i32 = 0x8;
    pub const RGBA: i32 = 0xF;
}

#[repr(i32)]
pub enum NVNdebugCallbackSource {
    API = 0,
}

#[repr(i32)]
pub enum NVNdebugCallbackType {
    Error = 0,
    Warning,
}

#[repr(i32)]
pub enum NVNdebugCallbackSeverity {
    High = 0,
    Medium,
    Low,
    Notification,
}

#[repr(i32)]
pub enum NVNwindowOriginMode {
    LowerLeft = 0,
    UpperLeft,
}

#[repr(i32)]
pub enum NVNdepthMode {
    NearIsMinusW = 0,
    NearIsZero,
}

#[repr(i32)]
pub enum NVNformat {
    None,
    R8,
    R8SN,
    R8UI,
    R8I,
    R16F,
    R16,
    R16SN,
    R16UI,
    R16I,
    R32F,
    R32UI,
    R32I,
    RG8,
    RG8SN,
    RG8UI,
    RG8I,
    RG16F,
    RG16,
    RG16SN,
    RG16UI,
    RG16I,
    RG32F,
    RG32UI,
    RG32I,
    RGB8,
    RGB8SN,
    RGB8UI,
    RGB8I,
    RGB16F,
    RGB16,
    RGB16SN,
    RGB16UI,
    RGB16I,
    RGB32F,
    RGB32UI,
    RGB32I,
    RGBA8,
    RGBA8SN,
    RGBA8UI,
    RGBA8I,
    RGBA16F,
    RGBA16,
    RGBA16SN,
    RGBA16UI,
    RGBA16I,
    RGBA32F,
    RGBA32UI,
    RGBA32I,
    S8,
    D16,
    D24,
    D32F,
    D24S8,
    D32FS8,
    RGBX8_SRGB,
    RGBA8_SRGB,
    RGBA4,
    RGB5,
    RGB5A1,
    RGB565,
    RGB10A2,
    RGB10A2UI,
    R11G11B10F,
    RGB9E5F,
    RGB_DXT1,
    RGBA_DXT1,
    RGBA_DXT3,
    RGBA_DXT5,
    RGB_DXT1_SRGB,
    RGBA_DXT1_SRGB,
    RGBA_DXT3_SRGB,
    RGBA_DXT5_SRGB,
    RGTC1_UNORN,
    RGTC1_SNORM,
    RGTC2_UNORM,
    RGTC2_SNORM,
    BPTC_UNORM,
    BPTC_UNORM_SRGB,
    BPTC_SFLOAT,
    BPTC_UFLOAT,
    R8_UI2F,
    R8_I2F,
    R16_UI2F,
    R16_I2F,
    R32_UI2F,
    R32_I2F,
    RG8_UI2F,
    RG8_I2F,
    RG16_UI2F,
    RG16_I2F,
    RG32_UI2F,
    RG32_I2F,
    RGB8_UI2F,
    RGB8_I2F,
    RGB16_UI2F,
    RGB16_I2F,
    RGB32_UI2F,
    RGB32_I2F,
    RGBA8_UI2F,
    RGBA8_I2F,
    RGBA16_UI2F,
    RGBA16_I2F,
    RGBA32_UI2F,
    RGBA32_I2F,
    RGB10A2SN,
    RGB10A2I,
    RGB10A2_UI2F,
    RGB10A2_I2F,
    RGBX8,
    RGBX8SN,
    RGBX8UI,
    RGBX8I,
    RGBX16F,
    RGBX16,
    RGBX16SN,
    RGBX16UI,
    RGBX16I,
    RGBX32F,
    RGBX32UI,
    RGBX32I,
    RGBA_ASTC_4x4,
    RGBA_ASTC_5x4,
    RGBA_ASTC_5x5,
    RGBA_ASTC_6x5,
    RGBA_ASTC_6x6,
    RGBA_ASTC_8x5,
    RGBA_ASTC_8x6,
    RGBA_ASTC_8x8,
    RGBA_ASTC_10x5,
    RGBA_ASTC_10x6,
    RGBA_ASTC_10x8,
    RGBA_ASTC_10x10,
    RGBA_ASTC_12x10,
    RGBA_ASTC_12x12,
    RGBA_ASTC_4x4_SRGB,
    RGBA_ASTC_5x4_SRGB,
    RGBA_ASTC_5x5_SRGB,
    RGBA_ASTC_6x5_SRGB,
    RGBA_ASTC_6x6_SRGB,
    RGBA_ASTC_8x5_SRGB,
    RGBA_ASTC_8x6_SRGB,
    RGBA_ASTC_8x8_SRGB,
    RGBA_ASTC_10x5_SRGB,
    RGBA_ASTC_10x6_SRGB,
    RGBA_ASTC_10x8_SRGB,
    RGBA_ASTC_10x10_SRGB,
    RGBA_ASTC_12x10_SRGB,
    RGBA_ASTC_12x12_SRGB,
    BGR565,
    BGR5,
    BGR5A1,
    A1BGR5,
    BGRX8,
    BGRA8,
    BGRX8_SRGB,
    BGRA8_SRGB,
}

#[repr(i32)]
pub enum NVNdebugObjectType {
    Window = 0,
    Queue,
    CommandBuffer,
    MemoryPool,
    Sync,
    Program,
    TexturePool,
    SamplerPool,
}

#[repr(i32)]
pub enum NVNqueueGetErrorResult {
    NoError = 0,
    Unknown,
    MmuFault,
    PbdmaException,
    EngineException,
    Timeout,
}

pub type NVNqueueBuilderFlags_ = i32;

pub struct NVNqueueBuilderFlags;
impl NVNqueueBuilderFlags {
    pub const NONE: i32 = 0 << 0;
    pub const NO_FRAGMENT_INTERLOCK: i32 = 1 << 0;
    pub const NO_Z_CULL: i32 = 1 << 2;
}

#[repr(i32)]
pub enum NVNacquireTextureResult {
    Success = 0,
    NativeError,
}

pub type NVNqueueAcquireTextureResult = NVNacquireTextureResult;
pub type NVNwindowAcquireTextureResult = NVNacquireTextureResult;

pub type NVNmemoryPoolFlags_ = i32;
pub struct NVNmemoryPoolFlags;
impl NVNmemoryPoolFlags {
    pub const CPU_NO_ACCESS: i32 = 1 << 0;
    pub const CPU_UNCACHED: i32 = 1 << 1;
    pub const CPU_CACHED: i32 = 1 << 2;
    pub const GPU_NO_ACCESS: i32 = 1 << 3;
    pub const GPU_UNCACHED: i32 = 1 << 4;
    pub const GPU_CACHED: i32 = 1 << 5;
    pub const SHADER_CODE: i32 = 1 << 6;
    pub const COMPRESSIBLE: i32 = 1 << 7;
    pub const PHYSICAL: i32 = 1 << 8;
    pub const VIRTUAL: i32 = 1 << 9;
}

#[repr(i32)]
pub enum NVNtextureSwizzle {
    Zero = 0,
    One,
    R,
    G,
    B,
    A,
}

#[repr(i32)]
pub enum NVNshaderStage {
    Vertex = 0,
    Fragment,
    Geometry,
    TessControl,
    TessEvaluation,
    Compute,
}

pub type NVNshaderStageBits_ = i32;
pub struct NVNshaderStageBits;
impl NVNshaderStageBits {
    pub const VERTEX_BIT: i32 = 1 << 0;
    pub const FRAGMENT_BIT: i32 = 1 << 1;
    pub const GEOMETRY_BIT: i32 = 1 << 2;
    pub const TESS_CONTROL_BIT: i32 = 1 << 3;
    pub const TESS_EVALUATION_BIT: i32 = 1 << 4;
    pub const COMPUTE_BIT: i32 = 1 << 4;
    pub const ALL_GRAPHICS_BITS: i32 = 0x1F;
    pub const ALL_BITS: i32 = 0x3F;
}

#[repr(i32)]
pub enum NVNeventWaitMode {
    Equal = 0,
    GEqualWrap,
}

#[repr(i32)]
pub enum NVNeventSignalMode {
    Write = 0,
    Add,
}

#[repr(i32)]
pub enum NVNeventSignalLocation {
    Top = 0,
    VertexPipe,
    Bottom,
}

#[repr(i32)]
pub enum NVNsyncCondition {
    AllGpuCommandsComplete = 0,
    GraphicsWorldSpaceComplete,
}

#[repr(i32)]
pub enum NVNcounterType {
    Timestamp = 0,
    SamplesPassed,
    InputVertices,
    InputPrimitives,
    VertexShaderInvocations,
    TessControlShaderInvocations,
    TessEvaluationShaderInvocations,
    GeometryShaderInvocations,
    FragmentShaderInvocations,
    TessEvaluationShaderPrimitives,
    GeometryShaderPrimitives,
    ClipperInputPrimitives,
    ClipperOutputPrimitives,
    PrimitivesGenerated,
    TransformFeedbackPrimitivesWritten,
    ZCullStats,
    TimestampTop,
}

#[repr(i32)]
pub enum NVNface {
    None = 0,
    Front,
    Back,
    FrontAndBack,
}

#[repr(i32)]
pub enum NVNcommandBufferMemoryEvent {
    OutOfCommandMemory = 0,
    OutOfControlMemory,
}

#[repr(i32)]
pub enum NVNdrawPrimitive {
    Points = 0,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
    Quads,
    QuadStrip,
    Polygon,
    LinesAdjancency,
    LineStripAdjancency,
    TrianglesAdjacency,
    TriangleStripAdjancency,
    Patches,
}

#[repr(i32)]
pub enum NVNindexType {
    UnsignedByte = 0,
    UnsignedShort,
    UnsignedInt,
}

pub type NVNtextureFlags_ = i32;
pub struct NVNtextureFlags;
impl NVNtextureFlags {
    pub const DISPLAY: i32 = 1 << 0;
    pub const VIDEO_DECODE: i32 = 1 << 1;
    pub const IMAGE: i32 = 1 << 2;
    pub const COMPRESSIBLE: i32 = 1 << 3;
    pub const LINEAR: i32 = 1 << 4;
    pub const SPARSE: i32 = 1 << 5;
    pub const LINEAR_RENDER_TARGET: i32 = 1 << 6;
    pub const ADAPTIVE_ZCULL: i32 = 1 << 7;
    pub const DEPTH16_PREFER_FAST_CLEAR: i32 = 1 << 8;
    pub const MINIMAL_LAYOUT: i32 = 1 << 9;
    pub const ZCULL_SUPPORT_STENCIL: i32 = 1 << 10;
}

pub type NVNbarrierBits_ = i32;
pub struct NVNbarrierBits;
impl NVNbarrierBits {
    pub const ORDER_PRIMITIVES_BIT: i32 = 1 << 0;
    pub const ORDER_FRAGMENTS_BIT: i32 = 1 << 1;
    pub const ORDER_FRAGMENTS_TILED_BIT: i32 = 1 << 2;
    pub const ORDER_INDIRECT_DATA_BIT: i32 = 1 << 3;
    pub const INVALIDATE_TEXTURE_BIT: i32 = 1 << 4;
    pub const INVALIDATE_SHADER_BIT: i32 = 1 << 5;
    pub const INVALIDATE_TEXTURE_DESCRIPTOR_BIT: i32 = 1 << 6;
    pub const INVALIDATE_ZCULL_BIT: i32 = 1 << 7;
}

#[repr(i32)]
pub enum NVNtextureTarget {
    T1D = 0,
    T2D,
    T3D,
    T1DArray,
    T2DArray,
    T2DMultisample,
    T2DMultisampleArray,
    Rectangle,
    Cubemap,
    CubemapArray,
    Buffer,
}

#[repr(i32)]
pub enum NVNtextureDepthStencilMode {
    Depth = 0,
    Stencil,
}

#[repr(i32)]
pub enum NVNlogicOp {
    Clear = 0,
    And,
    AndReverse,
    Copy,
    AndInverted,
    NoOp,
    Xor,
    Or,
    Nor,
    Equiv,
    Invert,
    OrReverse,
    CopyInverted,
    OrInverted,
    Nand,
    Set,
}

#[repr(i32)]
pub enum NVNalphaFunc {
    Never = 1,
    Less,
    Equal,
    LEqual,
    Greater,
    NotEqual,
    GEqual,
    Always,
}

#[repr(i32)]
pub enum NVNblendAdvancedMode {
    None = 0,
    Zero,
    Src,
    Dst,
    SrcOver,
    DstOver,
    SrcIn,
    DstIn,
    SrcOut,
    DstOut,
    SrcAtop,
    DstAtop,
    Xor,
    Plus,
    PlusClamped,
    PlusClampedAlpha,
    PlusDarker,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Minus,
    MinusClamped,
    Exclusion,
    Contrast,
    Invert,
    InvertRgb,
    InvertOvg,
    LinearDodge,
    LinearBurn,
    VividLight,
    LinearLight,
    PinLight,
    HardMix,
    Red,
    Green,
    Blue,
    HslHue,
    HslSaturation,
    HslColor,
    HslLuminosity,
}

#[repr(i32)]
pub enum NVNminFilter {
    Nearest = 0,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

#[repr(i32)]
pub enum NVNmagFilter {
    Nearest = 0,
    Linear,
}

#[repr(i32)]
pub enum NVNwrapMode {
    Clamp = 0,
    Repeat,
    MirrorClamp,
    MirrorClampToEdge,
    MirrorClampToBorder,
    ClampToBorder,
    MirroredRepeat,
    ClampToEdge,
}

#[repr(i32)]
pub enum NVNcompareMode {
    None = 0,
    CompareRToTexture,
}

pub type NVNcompareFunc = NVNalphaFunc;

#[repr(i32)]
pub enum NVNsamplerReduction {
    ReductionAverage = 0,
    ReductionMin,
    ReductionMax,
}

#[repr(i32)]
pub enum NVNblendFunc {
    Zero = 1,
    One = 2,
    SrcColor = 3,
    OneMinusSrcColor = 4,
    SrcAlpha = 5,
    OneMinusSrcAlpha = 6,
    DstAlpha = 7,
    OneMinusDstAlpha = 8,
    DstColor = 9,
    OneMinusDstColor = 10,
    SrcAlphaSaturate = 11,
    Src1Color = 16,
    OneMinusSrc1Color = 17,
    Src1Alpha = 18,
    OneMinusSrc1Alpha = 19,
    ConstantColor = 97,
    OneMinusConstantColor = 98,
    ConstantAlpha = 99,
    OneMinusConstantAlpha = 100,
}

#[repr(i32)]
pub enum NVNblendEquation {
    Add = 1,
    Sub,
    ReverseSub,
    Min,
    Max,
}

#[repr(i32)]
pub enum NVNblendAdvancedOverlap {
    Uncorrelated = 0,
    Disjoint,
    Conjoint,
}

#[repr(i32)]
pub enum NVNcoverageModulationMode {
    None = 0,
    Rgb,
    Alpha,
    Rgba,
}

pub type NVNdepthFunc = NVNalphaFunc;
pub type NVNstencilFunc = NVNalphaFunc;

#[repr(i32)]
pub enum NVNstencilOp {
    Keep = 1,
    Zero,
    Replace,
    Increment,
    Decrement,
    Invert,
    IncrementWrap,
    DecrementWrap,
}

#[repr(i32)]
pub enum NVNfrontFace {
    Cw = 0,
    Ccw,
}

#[repr(i32)]
pub enum NVNsyncWaitResult {
    AlreadySignaled,
    ConditionSatisfied,
    TimeoutExpired,
    Failed,
}

#[repr(i32)]
pub enum NVNpolygonOffsetEnable {
    None = 0,
    Point,
    Line,
    Fill,
}

#[repr(i32)]
pub enum NVNpolygonMode {
    Point = 0,
    Line,
    Fill,
}

#[repr(i32)]
pub enum NVNviewportSwizzle {
    PosX = 0,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
    PosW,
    NegW,
}

#[repr(i32)]
pub enum NVNtiledCacheAction {
    Enable = 1,
    Disable,
    Flush,
    FlushNoTiling,
    EnableRenderTargetBinning,
    DisableRenderTargetBinning,
}

#[repr(i32)]
pub enum NVNconditionalRenderMode {
    IfEqual = 0,
    IfNotEqual,
}

pub type NVNcopyFlags_ = i32;
pub struct NVNcopyFlags;
impl NVNcopyFlags {
    pub const NONE: i32 = 0;
    pub const LINEAR_FILTER_BIT: i32 = 1 << 0;
    pub const ENGINE_2D_BIT: i32 = 1 << 1;
    pub const MIRROR_X_BIT: i32 = 1 << 2;
    pub const MIRROR_Y_BIT: i32 = 1 << 3;
    pub const MIRROR_Z_BIT: i32 = 1 << 4;
}
