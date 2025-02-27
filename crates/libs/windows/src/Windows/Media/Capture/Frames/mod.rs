#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct AudioMediaFrame(::windows::core::IUnknown);
impl AudioMediaFrame {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn AudioEncodingProperties(&self) -> ::windows::core::Result<super::super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn GetAudioFrame(&self) -> ::windows::core::Result<super::super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::AudioFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioMediaFrame {}
unsafe impl ::windows::core::RuntimeType for AudioMediaFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.AudioMediaFrame;{a3a9feff-8021-441b-9a46-e7f0137b7981})");
}
unsafe impl ::windows::core::Interface for AudioMediaFrame {
    type Vtable = IAudioMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a9feff_8021_441b_9a46_e7f0137b7981);
}
impl ::windows::core::RuntimeName for AudioMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.AudioMediaFrame";
}
impl ::core::convert::From<AudioMediaFrame> for ::windows::core::IUnknown {
    fn from(value: AudioMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioMediaFrame> for ::windows::core::IUnknown {
    fn from(value: &AudioMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AudioMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioMediaFrame> for ::windows::core::IInspectable {
    fn from(value: AudioMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioMediaFrame> for ::windows::core::IInspectable {
    fn from(value: &AudioMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AudioMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioMediaFrame {}
unsafe impl ::core::marker::Sync for AudioMediaFrame {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct BufferMediaFrame(::windows::core::IUnknown);
impl BufferMediaFrame {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for BufferMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BufferMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BufferMediaFrame {}
unsafe impl ::windows::core::RuntimeType for BufferMediaFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.BufferMediaFrame;{b5b153c7-9b84-4062-b79c-a365b2596854})");
}
unsafe impl ::windows::core::Interface for BufferMediaFrame {
    type Vtable = IBufferMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5b153c7_9b84_4062_b79c_a365b2596854);
}
impl ::windows::core::RuntimeName for BufferMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.BufferMediaFrame";
}
impl ::core::convert::From<BufferMediaFrame> for ::windows::core::IUnknown {
    fn from(value: BufferMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BufferMediaFrame> for ::windows::core::IUnknown {
    fn from(value: &BufferMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BufferMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BufferMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BufferMediaFrame> for ::windows::core::IInspectable {
    fn from(value: BufferMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BufferMediaFrame> for ::windows::core::IInspectable {
    fn from(value: &BufferMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BufferMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BufferMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BufferMediaFrame {}
unsafe impl ::core::marker::Sync for BufferMediaFrame {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct DepthMediaFrame(::windows::core::IUnknown);
impl DepthMediaFrame {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn VideoMediaFrame(&self) -> ::windows::core::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn DepthFormat(&self) -> ::windows::core::Result<DepthMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DepthMediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Media_Devices_Core', 'Perception_Spatial'*"]
    #[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial"))]
    pub fn TryCreateCoordinateMapper<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Core::CameraIntrinsics>, Param1: ::windows::core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, cameraintrinsics: Param0, coordinatesystem: Param1) -> ::windows::core::Result<super::super::Devices::Core::DepthCorrelatedCoordinateMapper> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cameraintrinsics.into_param().abi(), coordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::Devices::Core::DepthCorrelatedCoordinateMapper>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn MaxReliableDepth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IDepthMediaFrame2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn MinReliableDepth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IDepthMediaFrame2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for DepthMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DepthMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DepthMediaFrame {}
unsafe impl ::windows::core::RuntimeType for DepthMediaFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.DepthMediaFrame;{47135e4f-8549-45c0-925b-80d35efdb10a})");
}
unsafe impl ::windows::core::Interface for DepthMediaFrame {
    type Vtable = IDepthMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47135e4f_8549_45c0_925b_80d35efdb10a);
}
impl ::windows::core::RuntimeName for DepthMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.DepthMediaFrame";
}
impl ::core::convert::From<DepthMediaFrame> for ::windows::core::IUnknown {
    fn from(value: DepthMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthMediaFrame> for ::windows::core::IUnknown {
    fn from(value: &DepthMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DepthMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DepthMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DepthMediaFrame> for ::windows::core::IInspectable {
    fn from(value: DepthMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthMediaFrame> for ::windows::core::IInspectable {
    fn from(value: &DepthMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DepthMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DepthMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DepthMediaFrame {}
unsafe impl ::core::marker::Sync for DepthMediaFrame {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct DepthMediaFrameFormat(::windows::core::IUnknown);
impl DepthMediaFrameFormat {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn VideoFormat(&self) -> ::windows::core::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn DepthScaleInMeters(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for DepthMediaFrameFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DepthMediaFrameFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DepthMediaFrameFormat {}
unsafe impl ::windows::core::RuntimeType for DepthMediaFrameFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.DepthMediaFrameFormat;{c312cf40-d729-453e-8780-2e04f140d28e})");
}
unsafe impl ::windows::core::Interface for DepthMediaFrameFormat {
    type Vtable = IDepthMediaFrameFormatVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc312cf40_d729_453e_8780_2e04f140d28e);
}
impl ::windows::core::RuntimeName for DepthMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.DepthMediaFrameFormat";
}
impl ::core::convert::From<DepthMediaFrameFormat> for ::windows::core::IUnknown {
    fn from(value: DepthMediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthMediaFrameFormat> for ::windows::core::IUnknown {
    fn from(value: &DepthMediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DepthMediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DepthMediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DepthMediaFrameFormat> for ::windows::core::IInspectable {
    fn from(value: DepthMediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthMediaFrameFormat> for ::windows::core::IInspectable {
    fn from(value: &DepthMediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DepthMediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DepthMediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DepthMediaFrameFormat {}
unsafe impl ::core::marker::Sync for DepthMediaFrameFormat {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioMediaFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioMediaFrame {
    type Vtable = IAudioMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a9feff_8021_441b_9a46_e7f0137b7981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMediaFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBufferMediaFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBufferMediaFrame {
    type Vtable = IBufferMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5b153c7_9b84_4062_b79c_a365b2596854);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferMediaFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDepthMediaFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDepthMediaFrame {
    type Vtable = IDepthMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47135e4f_8549_45c0_925b_80d35efdb10a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthMediaFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cameraintrinsics: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "Perception_Spatial")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDepthMediaFrame2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDepthMediaFrame2 {
    type Vtable = IDepthMediaFrame2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cca473d_c4a4_4176_b0cd_33eae3b35aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthMediaFrame2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDepthMediaFrameFormat(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDepthMediaFrameFormat {
    type Vtable = IDepthMediaFrameFormatVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc312cf40_d729_453e_8780_2e04f140d28e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthMediaFrameFormatVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInfraredMediaFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInfraredMediaFrame {
    type Vtable = IInfraredMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fd13503_004b_4f0e_91ac_465299b41658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfraredMediaFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameArrivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameArrivedEventArgs {
    type Vtable = IMediaFrameArrivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b430add_a490_4435_ada1_9affd55239f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameArrivedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameFormat(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameFormat {
    type Vtable = IMediaFrameFormatVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71902b4e_b279_4a97_a9db_bd5a2fb78f39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameFormatVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameFormat2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameFormat2 {
    type Vtable = IMediaFrameFormat2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63856340_5e87_4c10_86d1_6df097a6c6a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameFormat2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameReader {
    type Vtable = IMediaFrameReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4c94395_2028_48ed_90b0_d1c1b162e24c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameReader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameReader2 {
    type Vtable = IMediaFrameReader2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x871127b3_8531_4050_87cc_a13733cf3e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReader2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameReference(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameReference {
    type Vtable = IMediaFrameReferenceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6b88641_f0dc_4044_8dc9_961cedd05bad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReferenceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameReference2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameReference2 {
    type Vtable = IMediaFrameReference2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddbc3ecc_d5b2_49ef_836a_947d989b80c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReference2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSource {
    type Vtable = IMediaFrameSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6782953_90db_46a8_8add_2aa884a8d253);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_Devices_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSourceController {
    type Vtable = IMediaFrameSourceControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d076635_316d_4b8f_b7b6_eeb04a8c6525);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceControllerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSourceController2 {
    type Vtable = IMediaFrameSourceController2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefc49fd4_fcf2_4a03_b4e4_ac9628739bee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceController2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceController3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSourceController3 {
    type Vtable = IMediaFrameSourceController3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f0cf815_2464_4651_b1e8_4a82dbdb54de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceController3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceGetPropertyResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSourceGetPropertyResult {
    type Vtable = IMediaFrameSourceGetPropertyResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x088616c2_3a64_4bd5_bd2b_e7c898d2f37a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceGetPropertyResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceGetPropertyStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSourceGroup {
    type Vtable = IMediaFrameSourceGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f605b87_4832_4b5f_ae3d_412faab37d34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceGroupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceGroupStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSourceGroupStatics {
    type Vtable = IMediaFrameSourceGroupStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c48bfc5_436f_4508_94cf_d5d8b7326445);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceGroupStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSourceInfo {
    type Vtable = IMediaFrameSourceInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87bdc9cd_4601_408f_91cf_038318cd0af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaStreamType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSourceInfo2 {
    type Vtable = IMediaFrameSourceInfo2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x195a7855_6457_42c6_a769_19b65bd32e6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceInfo3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaFrameSourceInfo3 {
    type Vtable = IMediaFrameSourceInfo3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca824ab6_66ea_5885_a2b6_26c0eeec3c7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayregion: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultiSourceMediaFrameArrivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMultiSourceMediaFrameArrivedEventArgs {
    type Vtable = IMultiSourceMediaFrameArrivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63115e01_cf51_48fd_aab0_6d693eb48127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameArrivedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMultiSourceMediaFrameReader {
    type Vtable = IMultiSourceMediaFrameReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d144402_f763_488d_98f2_b437bcf075e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMultiSourceMediaFrameReader2 {
    type Vtable = IMultiSourceMediaFrameReader2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef5c8abd_fc5c_4c6b_9d81_3cb9cc637c26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReader2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReference(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMultiSourceMediaFrameReference {
    type Vtable = IMultiSourceMediaFrameReferenceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21964b1a_7fe2_44d6_92e5_298e6d2810e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReferenceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoMediaFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoMediaFrame {
    type Vtable = IVideoMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00dd4ccb_32bd_4fe1_a013_7cc13cf5dbcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoMediaFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(feature = "Media_Devices_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoMediaFrameFormat(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoMediaFrameFormat {
    type Vtable = IVideoMediaFrameFormatVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46027fc0_d71b_45c7_8f14_6d9a0ae604e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoMediaFrameFormatVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct InfraredMediaFrame(::windows::core::IUnknown);
impl InfraredMediaFrame {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn VideoMediaFrame(&self) -> ::windows::core::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn IsIlluminated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InfraredMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InfraredMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InfraredMediaFrame {}
unsafe impl ::windows::core::RuntimeType for InfraredMediaFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.InfraredMediaFrame;{3fd13503-004b-4f0e-91ac-465299b41658})");
}
unsafe impl ::windows::core::Interface for InfraredMediaFrame {
    type Vtable = IInfraredMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fd13503_004b_4f0e_91ac_465299b41658);
}
impl ::windows::core::RuntimeName for InfraredMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.InfraredMediaFrame";
}
impl ::core::convert::From<InfraredMediaFrame> for ::windows::core::IUnknown {
    fn from(value: InfraredMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InfraredMediaFrame> for ::windows::core::IUnknown {
    fn from(value: &InfraredMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InfraredMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InfraredMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InfraredMediaFrame> for ::windows::core::IInspectable {
    fn from(value: InfraredMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InfraredMediaFrame> for ::windows::core::IInspectable {
    fn from(value: &InfraredMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InfraredMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InfraredMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InfraredMediaFrame {}
unsafe impl ::core::marker::Sync for InfraredMediaFrame {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameArrivedEventArgs(::windows::core::IUnknown);
impl MediaFrameArrivedEventArgs {}
impl ::core::clone::Clone for MediaFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameArrivedEventArgs {}
unsafe impl ::windows::core::RuntimeType for MediaFrameArrivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs;{0b430add-a490-4435-ada1-9affd55239f7})");
}
unsafe impl ::windows::core::Interface for MediaFrameArrivedEventArgs {
    type Vtable = IMediaFrameArrivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b430add_a490_4435_ada1_9affd55239f7);
}
impl ::windows::core::RuntimeName for MediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs";
}
impl ::core::convert::From<MediaFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameArrivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaFrameArrivedEventArgs {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameFormat(::windows::core::IUnknown);
impl MediaFrameFormat {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn MajorType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameRate(&self) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn VideoFormat(&self) -> ::windows::core::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn AudioEncodingProperties(&self) -> ::windows::core::Result<super::super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IMediaFrameFormat2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameFormat {}
unsafe impl ::windows::core::RuntimeType for MediaFrameFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameFormat;{71902b4e-b279-4a97-a9db-bd5a2fb78f39})");
}
unsafe impl ::windows::core::Interface for MediaFrameFormat {
    type Vtable = IMediaFrameFormatVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71902b4e_b279_4a97_a9db_bd5a2fb78f39);
}
impl ::windows::core::RuntimeName for MediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameFormat";
}
impl ::core::convert::From<MediaFrameFormat> for ::windows::core::IUnknown {
    fn from(value: MediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameFormat> for ::windows::core::IUnknown {
    fn from(value: &MediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameFormat> for ::windows::core::IInspectable {
    fn from(value: MediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameFormat> for ::windows::core::IInspectable {
    fn from(value: &MediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameFormat {}
unsafe impl ::core::marker::Sync for MediaFrameFormat {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameReader(::windows::core::IUnknown);
impl MediaFrameReader {
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn TryAcquireLatestFrame(&self) -> ::windows::core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameReaderStartStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameReaderStartStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaFrameReader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn AcquisitionMode(&self) -> ::windows::core::Result<MediaFrameReaderAcquisitionMode> {
        let this = &::windows::core::Interface::cast::<IMediaFrameReader2>(self)?;
        unsafe {
            let mut result__: MediaFrameReaderAcquisitionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReaderAcquisitionMode>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameReader {}
unsafe impl ::windows::core::RuntimeType for MediaFrameReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameReader;{e4c94395-2028-48ed-90b0-d1c1b162e24c})");
}
unsafe impl ::windows::core::Interface for MediaFrameReader {
    type Vtable = IMediaFrameReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4c94395_2028_48ed_90b0_d1c1b162e24c);
}
impl ::windows::core::RuntimeName for MediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameReader";
}
impl ::core::convert::From<MediaFrameReader> for ::windows::core::IUnknown {
    fn from(value: MediaFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameReader> for ::windows::core::IUnknown {
    fn from(value: &MediaFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameReader> for ::windows::core::IInspectable {
    fn from(value: MediaFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameReader> for ::windows::core::IInspectable {
    fn from(value: &MediaFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaFrameReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaFrameReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for MediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &MediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaFrameReader {}
unsafe impl ::core::marker::Sync for MediaFrameReader {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameReaderAcquisitionMode(pub i32);
impl MediaFrameReaderAcquisitionMode {
    pub const Realtime: Self = Self(0i32);
    pub const Buffered: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaFrameReaderAcquisitionMode {}
impl ::core::clone::Clone for MediaFrameReaderAcquisitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaFrameReaderAcquisitionMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaFrameReaderAcquisitionMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameReaderAcquisitionMode {}
unsafe impl ::windows::core::RuntimeType for MediaFrameReaderAcquisitionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameReaderAcquisitionMode;i4)");
}
impl ::windows::core::DefaultType for MediaFrameReaderAcquisitionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameReaderStartStatus(pub i32);
impl MediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const DeviceNotAvailable: Self = Self(2i32);
    pub const OutputFormatNotSupported: Self = Self(3i32);
    pub const ExclusiveControlNotAvailable: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaFrameReaderStartStatus {}
impl ::core::clone::Clone for MediaFrameReaderStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaFrameReaderStartStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaFrameReaderStartStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameReaderStartStatus {}
unsafe impl ::windows::core::RuntimeType for MediaFrameReaderStartStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameReaderStartStatus;i4)");
}
impl ::windows::core::DefaultType for MediaFrameReaderStartStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameReference(::windows::core::IUnknown);
impl MediaFrameReference {
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn SourceKind(&self) -> ::windows::core::Result<MediaFrameSourceKind> {
        let this = self;
        unsafe {
            let mut result__: MediaFrameSourceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceKind>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Format(&self) -> ::windows::core::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn BufferMediaFrame(&self) -> ::windows::core::Result<BufferMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BufferMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn VideoMediaFrame(&self) -> ::windows::core::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Perception_Spatial'*"]
    #[cfg(feature = "Perception_Spatial")]
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::Spatial::SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn AudioMediaFrame(&self) -> ::windows::core::Result<AudioMediaFrame> {
        let this = &::windows::core::Interface::cast::<IMediaFrameReference2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioMediaFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameReference {}
unsafe impl ::windows::core::RuntimeType for MediaFrameReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameReference;{f6b88641-f0dc-4044-8dc9-961cedd05bad})");
}
unsafe impl ::windows::core::Interface for MediaFrameReference {
    type Vtable = IMediaFrameReferenceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6b88641_f0dc_4044_8dc9_961cedd05bad);
}
impl ::windows::core::RuntimeName for MediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameReference";
}
impl ::core::convert::From<MediaFrameReference> for ::windows::core::IUnknown {
    fn from(value: MediaFrameReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameReference> for ::windows::core::IUnknown {
    fn from(value: &MediaFrameReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameReference> for ::windows::core::IInspectable {
    fn from(value: MediaFrameReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameReference> for ::windows::core::IInspectable {
    fn from(value: &MediaFrameReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaFrameReference> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaFrameReference) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaFrameReference> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaFrameReference) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for MediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &MediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaFrameReference {}
unsafe impl ::core::marker::Sync for MediaFrameReference {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameSource(::windows::core::IUnknown);
impl MediaFrameSource {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Info(&self) -> ::windows::core::Result<MediaFrameSourceInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceInfo>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Controller(&self) -> ::windows::core::Result<MediaFrameSourceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceController>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<MediaFrameFormat>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn CurrentFormat(&self) -> ::windows::core::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFormatAsync<'a, Param0: ::windows::core::IntoParam<'a, MediaFrameFormat>>(&self, format: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), format.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FormatChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<MediaFrameSource, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFormatChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Media_Devices_Core'*"]
    #[cfg(feature = "Media_Devices_Core")]
    pub fn TryGetCameraIntrinsics<'a, Param0: ::windows::core::IntoParam<'a, MediaFrameFormat>>(&self, format: Param0) -> ::windows::core::Result<super::super::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), format.into_param().abi(), &mut result__).from_abi::<super::super::Devices::Core::CameraIntrinsics>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSource {}
unsafe impl ::windows::core::RuntimeType for MediaFrameSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSource;{d6782953-90db-46a8-8add-2aa884a8d253})");
}
unsafe impl ::windows::core::Interface for MediaFrameSource {
    type Vtable = IMediaFrameSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6782953_90db_46a8_8add_2aa884a8d253);
}
impl ::windows::core::RuntimeName for MediaFrameSource {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSource";
}
impl ::core::convert::From<MediaFrameSource> for ::windows::core::IUnknown {
    fn from(value: MediaFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSource> for ::windows::core::IUnknown {
    fn from(value: &MediaFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaFrameSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaFrameSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSource> for ::windows::core::IInspectable {
    fn from(value: MediaFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSource> for ::windows::core::IInspectable {
    fn from(value: &MediaFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaFrameSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaFrameSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSource {}
unsafe impl ::core::marker::Sync for MediaFrameSource {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameSourceController(::windows::core::IUnknown);
impl MediaFrameSourceController {
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, propertyid: Param0, propertyvalue: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertyid.into_param().abi(), propertyvalue.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Media_Devices'*"]
    #[cfg(feature = "Media_Devices")]
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<super::super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::VideoDeviceController>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyByExtendedIdAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], maxpropertyvaluesize: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>> {
        let this = &::windows::core::Interface::cast::<IMediaFrameSourceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), extendedpropertyid.len() as u32, ::core::mem::transmute(extendedpropertyid.as_ptr()), maxpropertyvaluesize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPropertyByExtendedIdAsync(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], propertyvalue: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>> {
        let this = &::windows::core::Interface::cast::<IMediaFrameSourceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), extendedpropertyid.len() as u32, ::core::mem::transmute(extendedpropertyid.as_ptr()), propertyvalue.len() as u32, ::core::mem::transmute(propertyvalue.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Media_Devices'*"]
    #[cfg(feature = "Media_Devices")]
    pub fn AudioDeviceController(&self) -> ::windows::core::Result<super::super::Devices::AudioDeviceController> {
        let this = &::windows::core::Interface::cast::<IMediaFrameSourceController3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::AudioDeviceController>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameSourceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSourceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceController {}
unsafe impl ::windows::core::RuntimeType for MediaFrameSourceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceController;{6d076635-316d-4b8f-b7b6-eeb04a8c6525})");
}
unsafe impl ::windows::core::Interface for MediaFrameSourceController {
    type Vtable = IMediaFrameSourceControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d076635_316d_4b8f_b7b6_eeb04a8c6525);
}
impl ::windows::core::RuntimeName for MediaFrameSourceController {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceController";
}
impl ::core::convert::From<MediaFrameSourceController> for ::windows::core::IUnknown {
    fn from(value: MediaFrameSourceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceController> for ::windows::core::IUnknown {
    fn from(value: &MediaFrameSourceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaFrameSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaFrameSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSourceController> for ::windows::core::IInspectable {
    fn from(value: MediaFrameSourceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceController> for ::windows::core::IInspectable {
    fn from(value: &MediaFrameSourceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaFrameSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaFrameSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceController {}
unsafe impl ::core::marker::Sync for MediaFrameSourceController {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyResult(::windows::core::IUnknown);
impl MediaFrameSourceGetPropertyResult {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Status(&self) -> ::windows::core::Result<MediaFrameSourceGetPropertyStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaFrameSourceGetPropertyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceGetPropertyStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameSourceGetPropertyResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSourceGetPropertyResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceGetPropertyResult {}
unsafe impl ::windows::core::RuntimeType for MediaFrameSourceGetPropertyResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult;{088616c2-3a64-4bd5-bd2b-e7c898d2f37a})");
}
unsafe impl ::windows::core::Interface for MediaFrameSourceGetPropertyResult {
    type Vtable = IMediaFrameSourceGetPropertyResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x088616c2_3a64_4bd5_bd2b_e7c898d2f37a);
}
impl ::windows::core::RuntimeName for MediaFrameSourceGetPropertyResult {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult";
}
impl ::core::convert::From<MediaFrameSourceGetPropertyResult> for ::windows::core::IUnknown {
    fn from(value: MediaFrameSourceGetPropertyResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceGetPropertyResult> for ::windows::core::IUnknown {
    fn from(value: &MediaFrameSourceGetPropertyResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSourceGetPropertyResult> for ::windows::core::IInspectable {
    fn from(value: MediaFrameSourceGetPropertyResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceGetPropertyResult> for ::windows::core::IInspectable {
    fn from(value: &MediaFrameSourceGetPropertyResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceGetPropertyResult {}
unsafe impl ::core::marker::Sync for MediaFrameSourceGetPropertyResult {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyStatus(pub i32);
impl MediaFrameSourceGetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(4i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaFrameSourceGetPropertyStatus {}
impl ::core::clone::Clone for MediaFrameSourceGetPropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaFrameSourceGetPropertyStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaFrameSourceGetPropertyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceGetPropertyStatus {}
unsafe impl ::windows::core::RuntimeType for MediaFrameSourceGetPropertyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyStatus;i4)");
}
impl ::windows::core::DefaultType for MediaFrameSourceGetPropertyStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameSourceGroup(::windows::core::IUnknown);
impl MediaFrameSourceGroup {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceInfo>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceGroup>>> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceGroup>>>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGroup>> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGroup>>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaFrameSourceGroupStatics<R, F: FnOnce(&IMediaFrameSourceGroupStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaFrameSourceGroup, IMediaFrameSourceGroupStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaFrameSourceGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSourceGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceGroup {}
unsafe impl ::windows::core::RuntimeType for MediaFrameSourceGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceGroup;{7f605b87-4832-4b5f-ae3d-412faab37d34})");
}
unsafe impl ::windows::core::Interface for MediaFrameSourceGroup {
    type Vtable = IMediaFrameSourceGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f605b87_4832_4b5f_ae3d_412faab37d34);
}
impl ::windows::core::RuntimeName for MediaFrameSourceGroup {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceGroup";
}
impl ::core::convert::From<MediaFrameSourceGroup> for ::windows::core::IUnknown {
    fn from(value: MediaFrameSourceGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceGroup> for ::windows::core::IUnknown {
    fn from(value: &MediaFrameSourceGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaFrameSourceGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaFrameSourceGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSourceGroup> for ::windows::core::IInspectable {
    fn from(value: MediaFrameSourceGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceGroup> for ::windows::core::IInspectable {
    fn from(value: &MediaFrameSourceGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaFrameSourceGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaFrameSourceGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceGroup {}
unsafe impl ::core::marker::Sync for MediaFrameSourceGroup {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameSourceInfo(::windows::core::IUnknown);
impl MediaFrameSourceInfo {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn MediaStreamType(&self) -> ::windows::core::Result<super::MediaStreamType> {
        let this = self;
        unsafe {
            let mut result__: super::MediaStreamType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaStreamType>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn SourceKind(&self) -> ::windows::core::Result<MediaFrameSourceKind> {
        let this = self;
        unsafe {
            let mut result__: MediaFrameSourceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceKind>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn SourceGroup(&self) -> ::windows::core::Result<MediaFrameSourceGroup> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceGroup>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Devices_Enumeration'*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Perception_Spatial'*"]
    #[cfg(feature = "Perception_Spatial")]
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::Spatial::SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn ProfileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaFrameSourceInfo2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn VideoProfileMediaDescription(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>> {
        let this = &::windows::core::Interface::cast::<IMediaFrameSourceInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Devices_Enumeration', 'UI_WindowManagement'*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))]
    pub fn GetRelativePanel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::WindowManagement::DisplayRegion>>(&self, displayregion: Param0) -> ::windows::core::Result<super::super::super::Devices::Enumeration::Panel> {
        let this = &::windows::core::Interface::cast::<IMediaFrameSourceInfo3>(self)?;
        unsafe {
            let mut result__: super::super::super::Devices::Enumeration::Panel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), displayregion.into_param().abi(), &mut result__).from_abi::<super::super::super::Devices::Enumeration::Panel>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameSourceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSourceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceInfo {}
unsafe impl ::windows::core::RuntimeType for MediaFrameSourceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceInfo;{87bdc9cd-4601-408f-91cf-038318cd0af3})");
}
unsafe impl ::windows::core::Interface for MediaFrameSourceInfo {
    type Vtable = IMediaFrameSourceInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87bdc9cd_4601_408f_91cf_038318cd0af3);
}
impl ::windows::core::RuntimeName for MediaFrameSourceInfo {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceInfo";
}
impl ::core::convert::From<MediaFrameSourceInfo> for ::windows::core::IUnknown {
    fn from(value: MediaFrameSourceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceInfo> for ::windows::core::IUnknown {
    fn from(value: &MediaFrameSourceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaFrameSourceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaFrameSourceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSourceInfo> for ::windows::core::IInspectable {
    fn from(value: MediaFrameSourceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceInfo> for ::windows::core::IInspectable {
    fn from(value: &MediaFrameSourceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaFrameSourceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaFrameSourceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceInfo {}
unsafe impl ::core::marker::Sync for MediaFrameSourceInfo {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameSourceKind(pub i32);
impl MediaFrameSourceKind {
    pub const Custom: Self = Self(0i32);
    pub const Color: Self = Self(1i32);
    pub const Infrared: Self = Self(2i32);
    pub const Depth: Self = Self(3i32);
    pub const Audio: Self = Self(4i32);
    pub const Image: Self = Self(5i32);
    pub const Metadata: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaFrameSourceKind {}
impl ::core::clone::Clone for MediaFrameSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaFrameSourceKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaFrameSourceKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceKind {}
unsafe impl ::windows::core::RuntimeType for MediaFrameSourceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceKind;i4)");
}
impl ::windows::core::DefaultType for MediaFrameSourceKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MediaFrameSourceSetPropertyStatus(pub i32);
impl MediaFrameSourceSetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaFrameSourceSetPropertyStatus {}
impl ::core::clone::Clone for MediaFrameSourceSetPropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaFrameSourceSetPropertyStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaFrameSourceSetPropertyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceSetPropertyStatus {}
unsafe impl ::windows::core::RuntimeType for MediaFrameSourceSetPropertyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceSetPropertyStatus;i4)");
}
impl ::windows::core::DefaultType for MediaFrameSourceSetPropertyStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MultiSourceMediaFrameArrivedEventArgs(::windows::core::IUnknown);
impl MultiSourceMediaFrameArrivedEventArgs {}
impl ::core::clone::Clone for MultiSourceMediaFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MultiSourceMediaFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MultiSourceMediaFrameArrivedEventArgs {}
unsafe impl ::windows::core::RuntimeType for MultiSourceMediaFrameArrivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs;{63115e01-cf51-48fd-aab0-6d693eb48127})");
}
unsafe impl ::windows::core::Interface for MultiSourceMediaFrameArrivedEventArgs {
    type Vtable = IMultiSourceMediaFrameArrivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63115e01_cf51_48fd_aab0_6d693eb48127);
}
impl ::windows::core::RuntimeName for MultiSourceMediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs";
}
impl ::core::convert::From<MultiSourceMediaFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MultiSourceMediaFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MultiSourceMediaFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MultiSourceMediaFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MultiSourceMediaFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MultiSourceMediaFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MultiSourceMediaFrameArrivedEventArgs {}
unsafe impl ::core::marker::Sync for MultiSourceMediaFrameArrivedEventArgs {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MultiSourceMediaFrameReader(::windows::core::IUnknown);
impl MultiSourceMediaFrameReader {
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn TryAcquireLatestFrame(&self) -> ::windows::core::Result<MultiSourceMediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MultiSourceMediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMultiSourceMediaFrameReader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn AcquisitionMode(&self) -> ::windows::core::Result<MediaFrameReaderAcquisitionMode> {
        let this = &::windows::core::Interface::cast::<IMultiSourceMediaFrameReader2>(self)?;
        unsafe {
            let mut result__: MediaFrameReaderAcquisitionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReaderAcquisitionMode>(result__)
        }
    }
}
impl ::core::clone::Clone for MultiSourceMediaFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MultiSourceMediaFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MultiSourceMediaFrameReader {}
unsafe impl ::windows::core::RuntimeType for MultiSourceMediaFrameReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MultiSourceMediaFrameReader;{8d144402-f763-488d-98f2-b437bcf075e7})");
}
unsafe impl ::windows::core::Interface for MultiSourceMediaFrameReader {
    type Vtable = IMultiSourceMediaFrameReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d144402_f763_488d_98f2_b437bcf075e7);
}
impl ::windows::core::RuntimeName for MultiSourceMediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameReader";
}
impl ::core::convert::From<MultiSourceMediaFrameReader> for ::windows::core::IUnknown {
    fn from(value: MultiSourceMediaFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReader> for ::windows::core::IUnknown {
    fn from(value: &MultiSourceMediaFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MultiSourceMediaFrameReader> for ::windows::core::IInspectable {
    fn from(value: MultiSourceMediaFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReader> for ::windows::core::IInspectable {
    fn from(value: &MultiSourceMediaFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MultiSourceMediaFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MultiSourceMediaFrameReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MultiSourceMediaFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MultiSourceMediaFrameReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MultiSourceMediaFrameReader {}
unsafe impl ::core::marker::Sync for MultiSourceMediaFrameReader {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MultiSourceMediaFrameReaderStartStatus(pub i32);
impl MultiSourceMediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const NotSupported: Self = Self(1i32);
    pub const InsufficientResources: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for MultiSourceMediaFrameReaderStartStatus {}
impl ::core::clone::Clone for MultiSourceMediaFrameReaderStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MultiSourceMediaFrameReaderStartStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MultiSourceMediaFrameReaderStartStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MultiSourceMediaFrameReaderStartStatus {}
unsafe impl ::windows::core::RuntimeType for MultiSourceMediaFrameReaderStartStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MultiSourceMediaFrameReaderStartStatus;i4)");
}
impl ::windows::core::DefaultType for MultiSourceMediaFrameReaderStartStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct MultiSourceMediaFrameReference(::windows::core::IUnknown);
impl MultiSourceMediaFrameReference {
    #[doc = "*Required features: 'Media_Capture_Frames', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn TryGetFrameReferenceBySourceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, sourceid: Param0) -> ::windows::core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sourceid.into_param().abi(), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
}
impl ::core::clone::Clone for MultiSourceMediaFrameReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MultiSourceMediaFrameReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MultiSourceMediaFrameReference {}
unsafe impl ::windows::core::RuntimeType for MultiSourceMediaFrameReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MultiSourceMediaFrameReference;{21964b1a-7fe2-44d6-92e5-298e6d2810e9})");
}
unsafe impl ::windows::core::Interface for MultiSourceMediaFrameReference {
    type Vtable = IMultiSourceMediaFrameReferenceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21964b1a_7fe2_44d6_92e5_298e6d2810e9);
}
impl ::windows::core::RuntimeName for MultiSourceMediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameReference";
}
impl ::core::convert::From<MultiSourceMediaFrameReference> for ::windows::core::IUnknown {
    fn from(value: MultiSourceMediaFrameReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReference> for ::windows::core::IUnknown {
    fn from(value: &MultiSourceMediaFrameReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MultiSourceMediaFrameReference> for ::windows::core::IInspectable {
    fn from(value: MultiSourceMediaFrameReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReference> for ::windows::core::IInspectable {
    fn from(value: &MultiSourceMediaFrameReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MultiSourceMediaFrameReference> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MultiSourceMediaFrameReference) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MultiSourceMediaFrameReference> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MultiSourceMediaFrameReference) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MultiSourceMediaFrameReference {}
unsafe impl ::core::marker::Sync for MultiSourceMediaFrameReference {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct VideoMediaFrame(::windows::core::IUnknown);
impl VideoMediaFrame {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn VideoFormat(&self) -> ::windows::core::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Graphics_Imaging'*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Graphics_DirectX_Direct3D11'*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3DSurface(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames', 'Media_Devices_Core'*"]
    #[cfg(feature = "Media_Devices_Core")]
    pub fn CameraIntrinsics(&self) -> ::windows::core::Result<super::super::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Core::CameraIntrinsics>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn InfraredMediaFrame(&self) -> ::windows::core::Result<InfraredMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InfraredMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn DepthMediaFrame(&self) -> ::windows::core::Result<DepthMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DepthMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn GetVideoFrame(&self) -> ::windows::core::Result<super::super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::VideoFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoMediaFrame {}
unsafe impl ::windows::core::RuntimeType for VideoMediaFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.VideoMediaFrame;{00dd4ccb-32bd-4fe1-a013-7cc13cf5dbcf})");
}
unsafe impl ::windows::core::Interface for VideoMediaFrame {
    type Vtable = IVideoMediaFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00dd4ccb_32bd_4fe1_a013_7cc13cf5dbcf);
}
impl ::windows::core::RuntimeName for VideoMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.VideoMediaFrame";
}
impl ::core::convert::From<VideoMediaFrame> for ::windows::core::IUnknown {
    fn from(value: VideoMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoMediaFrame> for ::windows::core::IUnknown {
    fn from(value: &VideoMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VideoMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoMediaFrame> for ::windows::core::IInspectable {
    fn from(value: VideoMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoMediaFrame> for ::windows::core::IInspectable {
    fn from(value: &VideoMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VideoMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoMediaFrame {}
unsafe impl ::core::marker::Sync for VideoMediaFrame {}
#[doc = "*Required features: 'Media_Capture_Frames'*"]
#[repr(transparent)]
pub struct VideoMediaFrameFormat(::windows::core::IUnknown);
impl VideoMediaFrameFormat {
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn MediaFrameFormat(&self) -> ::windows::core::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn DepthFormat(&self) -> ::windows::core::Result<DepthMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DepthMediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Capture_Frames'*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoMediaFrameFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoMediaFrameFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoMediaFrameFormat {}
unsafe impl ::windows::core::RuntimeType for VideoMediaFrameFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.VideoMediaFrameFormat;{46027fc0-d71b-45c7-8f14-6d9a0ae604e4})");
}
unsafe impl ::windows::core::Interface for VideoMediaFrameFormat {
    type Vtable = IVideoMediaFrameFormatVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46027fc0_d71b_45c7_8f14_6d9a0ae604e4);
}
impl ::windows::core::RuntimeName for VideoMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.VideoMediaFrameFormat";
}
impl ::core::convert::From<VideoMediaFrameFormat> for ::windows::core::IUnknown {
    fn from(value: VideoMediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoMediaFrameFormat> for ::windows::core::IUnknown {
    fn from(value: &VideoMediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoMediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VideoMediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoMediaFrameFormat> for ::windows::core::IInspectable {
    fn from(value: VideoMediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoMediaFrameFormat> for ::windows::core::IInspectable {
    fn from(value: &VideoMediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoMediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VideoMediaFrameFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoMediaFrameFormat {}
unsafe impl ::core::marker::Sync for VideoMediaFrameFormat {}
