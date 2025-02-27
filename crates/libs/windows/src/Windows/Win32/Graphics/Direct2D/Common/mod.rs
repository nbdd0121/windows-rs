#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_ALPHA_MODE = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = 0u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = 2u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = 3u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_ALPHA_MODE_FORCE_DWORD: D2D1_ALPHA_MODE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: D2D_POINT_2F,
    pub point2: D2D_POINT_2F,
    pub point3: D2D_POINT_2F,
}
impl ::core::marker::Copy for D2D1_BEZIER_SEGMENT {}
impl ::core::clone::Clone for D2D1_BEZIER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BEZIER_SEGMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D1_BEZIER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D1_BEZIER_SEGMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D1_BEZIER_SEGMENT {}
impl ::core::default::Default for D2D1_BEZIER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_BLEND_MODE = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_MULTIPLY: D2D1_BLEND_MODE = 0u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_SCREEN: D2D1_BLEND_MODE = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_DARKEN: D2D1_BLEND_MODE = 2u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_LIGHTEN: D2D1_BLEND_MODE = 3u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_DISSOLVE: D2D1_BLEND_MODE = 4u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_COLOR_BURN: D2D1_BLEND_MODE = 5u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_LINEAR_BURN: D2D1_BLEND_MODE = 6u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_DARKER_COLOR: D2D1_BLEND_MODE = 7u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_LIGHTER_COLOR: D2D1_BLEND_MODE = 8u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_COLOR_DODGE: D2D1_BLEND_MODE = 9u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_LINEAR_DODGE: D2D1_BLEND_MODE = 10u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_OVERLAY: D2D1_BLEND_MODE = 11u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_SOFT_LIGHT: D2D1_BLEND_MODE = 12u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_HARD_LIGHT: D2D1_BLEND_MODE = 13u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_VIVID_LIGHT: D2D1_BLEND_MODE = 14u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_LINEAR_LIGHT: D2D1_BLEND_MODE = 15u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_PIN_LIGHT: D2D1_BLEND_MODE = 16u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_HARD_MIX: D2D1_BLEND_MODE = 17u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_DIFFERENCE: D2D1_BLEND_MODE = 18u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_EXCLUSION: D2D1_BLEND_MODE = 19u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_HUE: D2D1_BLEND_MODE = 20u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_SATURATION: D2D1_BLEND_MODE = 21u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_COLOR: D2D1_BLEND_MODE = 22u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_LUMINOSITY: D2D1_BLEND_MODE = 23u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_SUBTRACT: D2D1_BLEND_MODE = 24u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_DIVISION: D2D1_BLEND_MODE = 25u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BLEND_MODE_FORCE_DWORD: D2D1_BLEND_MODE = 4294967295u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_BORDER_MODE = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BORDER_MODE_SOFT: D2D1_BORDER_MODE = 0u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BORDER_MODE_HARD: D2D1_BORDER_MODE = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_BORDER_MODE_FORCE_DWORD: D2D1_BORDER_MODE = 4294967295u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_COLORMATRIX_ALPHA_MODE = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COLORMATRIX_ALPHA_MODE_PREMULTIPLIED: D2D1_COLORMATRIX_ALPHA_MODE = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COLORMATRIX_ALPHA_MODE_STRAIGHT: D2D1_COLORMATRIX_ALPHA_MODE = 2u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COLORMATRIX_ALPHA_MODE_FORCE_DWORD: D2D1_COLORMATRIX_ALPHA_MODE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D1_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for D2D1_COLOR_F {}
impl ::core::clone::Clone for D2D1_COLOR_F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLOR_F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D1_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D1_COLOR_F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D1_COLOR_F {}
impl ::core::default::Default for D2D1_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_COMPOSITE_MODE = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_SOURCE_OVER: D2D1_COMPOSITE_MODE = 0u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_DESTINATION_OVER: D2D1_COMPOSITE_MODE = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_SOURCE_IN: D2D1_COMPOSITE_MODE = 2u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_DESTINATION_IN: D2D1_COMPOSITE_MODE = 3u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_SOURCE_OUT: D2D1_COMPOSITE_MODE = 4u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_DESTINATION_OUT: D2D1_COMPOSITE_MODE = 5u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_SOURCE_ATOP: D2D1_COMPOSITE_MODE = 6u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_DESTINATION_ATOP: D2D1_COMPOSITE_MODE = 7u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_XOR: D2D1_COMPOSITE_MODE = 8u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_PLUS: D2D1_COMPOSITE_MODE = 9u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_SOURCE_COPY: D2D1_COMPOSITE_MODE = 10u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY: D2D1_COMPOSITE_MODE = 11u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_MASK_INVERT: D2D1_COMPOSITE_MODE = 12u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_COMPOSITE_MODE_FORCE_DWORD: D2D1_COMPOSITE_MODE = 4294967295u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_FIGURE_BEGIN = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_FIGURE_BEGIN_FILLED: D2D1_FIGURE_BEGIN = 0u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_FIGURE_BEGIN_HOLLOW: D2D1_FIGURE_BEGIN = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_FIGURE_BEGIN_FORCE_DWORD: D2D1_FIGURE_BEGIN = 4294967295u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_FIGURE_END = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_FIGURE_END_OPEN: D2D1_FIGURE_END = 0u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_FIGURE_END_CLOSED: D2D1_FIGURE_END = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_FIGURE_END_FORCE_DWORD: D2D1_FIGURE_END = 4294967295u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_FILL_MODE = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_FILL_MODE_ALTERNATE: D2D1_FILL_MODE = 0u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_FILL_MODE_WINDING: D2D1_FILL_MODE = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_FILL_MODE_FORCE_DWORD: D2D1_FILL_MODE = 4294967295u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_PATH_SEGMENT = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_PATH_SEGMENT_NONE: D2D1_PATH_SEGMENT = 0u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_PATH_SEGMENT_FORCE_UNSTROKED: D2D1_PATH_SEGMENT = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN: D2D1_PATH_SEGMENT = 2u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_PATH_SEGMENT_FORCE_DWORD: D2D1_PATH_SEGMENT = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common', 'Win32_Graphics_Dxgi_Common'*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_PIXEL_FORMAT {
    pub format: super::super::Dxgi::Common::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D2D1_PIXEL_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D2D1_PIXEL_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D2D1_PIXEL_FORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D2D1_PIXEL_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D1_PIXEL_FORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D2D1_PIXEL_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D2D1_PIXEL_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub type D2D1_TURBULENCE_NOISE = u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_TURBULENCE_NOISE_FRACTAL_SUM: D2D1_TURBULENCE_NOISE = 0u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_TURBULENCE_NOISE_TURBULENCE: D2D1_TURBULENCE_NOISE = 1u32;
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub const D2D1_TURBULENCE_NOISE_FORCE_DWORD: D2D1_TURBULENCE_NOISE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for D2D_COLOR_F {}
impl ::core::clone::Clone for D2D_COLOR_F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_COLOR_F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_COLOR_F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_COLOR_F {}
impl ::core::default::Default for D2D_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_MATRIX_3X2_F {
    pub Anonymous: D2D_MATRIX_3X2_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_3X2_F {}
impl ::core::clone::Clone for D2D_MATRIX_3X2_F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_3X2_F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_3X2_F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_3X2_F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_3X2_F {}
impl ::core::default::Default for D2D_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub union D2D_MATRIX_3X2_F_0 {
    pub Anonymous1: D2D_MATRIX_3X2_F_0_0,
    pub Anonymous2: D2D_MATRIX_3X2_F_0_1,
    pub m: [f32; 6],
}
impl ::core::marker::Copy for D2D_MATRIX_3X2_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_3X2_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_3X2_F_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_3X2_F_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_3X2_F_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_3X2_F_0 {}
impl ::core::default::Default for D2D_MATRIX_3X2_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_MATRIX_3X2_F_0_0 {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_3X2_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_3X2_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_3X2_F_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_3X2_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_3X2_F_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_3X2_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_3X2_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_MATRIX_3X2_F_0_1 {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_3X2_F_0_1 {}
impl ::core::clone::Clone for D2D_MATRIX_3X2_F_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_3X2_F_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_3X2_F_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_3X2_F_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_3X2_F_0_1 {}
impl ::core::default::Default for D2D_MATRIX_3X2_F_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_MATRIX_4X3_F {
    pub Anonymous: D2D_MATRIX_4X3_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_4X3_F {}
impl ::core::clone::Clone for D2D_MATRIX_4X3_F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X3_F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X3_F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_4X3_F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X3_F {}
impl ::core::default::Default for D2D_MATRIX_4X3_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub union D2D_MATRIX_4X3_F_0 {
    pub Anonymous: D2D_MATRIX_4X3_F_0_0,
    pub m: [f32; 12],
}
impl ::core::marker::Copy for D2D_MATRIX_4X3_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X3_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X3_F_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X3_F_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_4X3_F_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X3_F_0 {}
impl ::core::default::Default for D2D_MATRIX_4X3_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_MATRIX_4X3_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_4X3_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X3_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X3_F_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X3_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_4X3_F_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X3_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_4X3_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_MATRIX_4X4_F {
    pub Anonymous: D2D_MATRIX_4X4_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_4X4_F {}
impl ::core::clone::Clone for D2D_MATRIX_4X4_F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X4_F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X4_F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_4X4_F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X4_F {}
impl ::core::default::Default for D2D_MATRIX_4X4_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub union D2D_MATRIX_4X4_F_0 {
    pub Anonymous: D2D_MATRIX_4X4_F_0_0,
    pub m: [f32; 16],
}
impl ::core::marker::Copy for D2D_MATRIX_4X4_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X4_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X4_F_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X4_F_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_4X4_F_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X4_F_0 {}
impl ::core::default::Default for D2D_MATRIX_4X4_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_MATRIX_4X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_4X4_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X4_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_4X4_F_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X4_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_4X4_F_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X4_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_4X4_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_MATRIX_5X4_F {
    pub Anonymous: D2D_MATRIX_5X4_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_5X4_F {}
impl ::core::clone::Clone for D2D_MATRIX_5X4_F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_5X4_F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_5X4_F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_5X4_F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_5X4_F {}
impl ::core::default::Default for D2D_MATRIX_5X4_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub union D2D_MATRIX_5X4_F_0 {
    pub Anonymous: D2D_MATRIX_5X4_F_0_0,
    pub m: [f32; 20],
}
impl ::core::marker::Copy for D2D_MATRIX_5X4_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_5X4_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_5X4_F_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_5X4_F_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_5X4_F_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_5X4_F_0 {}
impl ::core::default::Default for D2D_MATRIX_5X4_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_MATRIX_5X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
    pub _51: f32,
    pub _52: f32,
    pub _53: f32,
    pub _54: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_5X4_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_5X4_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_MATRIX_5X4_F_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_5X4_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_MATRIX_5X4_F_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_5X4_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_5X4_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_POINT_2F {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for D2D_POINT_2F {}
impl ::core::clone::Clone for D2D_POINT_2F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_POINT_2F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_POINT_2F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_POINT_2F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_POINT_2F {}
impl ::core::default::Default for D2D_POINT_2F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_POINT_2U {
    pub x: u32,
    pub y: u32,
}
impl ::core::marker::Copy for D2D_POINT_2U {}
impl ::core::clone::Clone for D2D_POINT_2U {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_POINT_2U {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_POINT_2U {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_POINT_2U>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_POINT_2U {}
impl ::core::default::Default for D2D_POINT_2U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl ::core::marker::Copy for D2D_RECT_F {}
impl ::core::clone::Clone for D2D_RECT_F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_RECT_F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_RECT_F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_RECT_F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_RECT_F {}
impl ::core::default::Default for D2D_RECT_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}
impl ::core::marker::Copy for D2D_RECT_U {}
impl ::core::clone::Clone for D2D_RECT_U {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_RECT_U {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_RECT_U {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_RECT_U>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_RECT_U {}
impl ::core::default::Default for D2D_RECT_U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for D2D_SIZE_F {}
impl ::core::clone::Clone for D2D_SIZE_F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_SIZE_F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_SIZE_F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_SIZE_F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_SIZE_F {}
impl ::core::default::Default for D2D_SIZE_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
impl ::core::marker::Copy for D2D_SIZE_U {}
impl ::core::clone::Clone for D2D_SIZE_U {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_SIZE_U {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_SIZE_U {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_SIZE_U>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_SIZE_U {}
impl ::core::default::Default for D2D_SIZE_U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_VECTOR_2F {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for D2D_VECTOR_2F {}
impl ::core::clone::Clone for D2D_VECTOR_2F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_VECTOR_2F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_VECTOR_2F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_VECTOR_2F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_2F {}
impl ::core::default::Default for D2D_VECTOR_2F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_VECTOR_3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ::core::marker::Copy for D2D_VECTOR_3F {}
impl ::core::clone::Clone for D2D_VECTOR_3F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_VECTOR_3F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_VECTOR_3F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_VECTOR_3F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_3F {}
impl ::core::default::Default for D2D_VECTOR_3F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
pub struct D2D_VECTOR_4F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl ::core::marker::Copy for D2D_VECTOR_4F {}
impl ::core::clone::Clone for D2D_VECTOR_4F {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D_VECTOR_4F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D2D_VECTOR_4F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D2D_VECTOR_4F>()) == 0 }
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_4F {}
impl ::core::default::Default for D2D_VECTOR_4F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
#[repr(transparent)]
pub struct ID2D1SimplifiedGeometrySink(::windows::core::IUnknown);
impl ID2D1SimplifiedGeometrySink {
    #[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
    pub unsafe fn SetFillMode(&self, fillmode: D2D1_FILL_MODE) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(fillmode))
    }
    #[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
    pub unsafe fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(vertexflags))
    }
    #[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
    pub unsafe fn BeginFigure<'a, Param0: ::windows::core::IntoParam<'a, D2D_POINT_2F>>(&self, startpoint: Param0, figurebegin: D2D1_FIGURE_BEGIN) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), startpoint.into_param().abi(), ::core::mem::transmute(figurebegin))
    }
    #[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
    pub unsafe fn AddLines(&self, points: *const D2D_POINT_2F, pointscount: u32) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(points), ::core::mem::transmute(pointscount))
    }
    #[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
    pub unsafe fn AddBeziers(&self, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(beziers), ::core::mem::transmute(bezierscount))
    }
    #[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
    pub unsafe fn EndFigure(&self, figureend: D2D1_FIGURE_END) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(figureend))
    }
    #[doc = "*Required features: 'Win32_Graphics_Direct2D_Common'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ID2D1SimplifiedGeometrySink> for ::windows::core::IUnknown {
    fn from(value: ID2D1SimplifiedGeometrySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID2D1SimplifiedGeometrySink> for ::windows::core::IUnknown {
    fn from(value: &ID2D1SimplifiedGeometrySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID2D1SimplifiedGeometrySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID2D1SimplifiedGeometrySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ID2D1SimplifiedGeometrySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID2D1SimplifiedGeometrySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SimplifiedGeometrySink {}
unsafe impl ::windows::core::Interface for ID2D1SimplifiedGeometrySink {
    type Vtable = ID2D1SimplifiedGeometrySinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd9069e_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SimplifiedGeometrySinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillmode: D2D1_FILL_MODE),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexflags: D2D1_PATH_SEGMENT),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, points: *const D2D_POINT_2F, pointscount: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, figureend: D2D1_FIGURE_END),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
