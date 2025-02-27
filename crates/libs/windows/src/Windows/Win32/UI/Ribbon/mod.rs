#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUIApplication(::windows::core::IUnknown);
impl IUIApplication {
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn OnViewChanged<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, viewid: u32, typeid: UI_VIEWTYPE, view: Param2, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewid), ::core::mem::transmute(typeid), view.into_param().abi(), ::core::mem::transmute(verb), ::core::mem::transmute(ureasoncode)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn OnCreateUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE) -> ::windows::core::Result<IUICommandHandler> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(typeid), ::core::mem::transmute(&mut result__)).from_abi::<IUICommandHandler>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn OnDestroyUICommand<'a, Param2: ::windows::core::IntoParam<'a, IUICommandHandler>>(&self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(typeid), commandhandler.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIApplication> for ::windows::core::IUnknown {
    fn from(value: IUIApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIApplication> for ::windows::core::IUnknown {
    fn from(value: &IUIApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUIApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIApplication {}
unsafe impl ::windows::core::Interface for IUIApplication {
    type Vtable = IUIApplicationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd428903c_729a_491d_910d_682a08ff2522);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIApplicationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut ::core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUICollection(::windows::core::IUnknown);
impl IUICollection {
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn GetItem(&self, index: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, item: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn Insert<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, item: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), item.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn Replace<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, indexreplaced: u32, itemreplacewith: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexreplaced), itemreplacewith.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IUICollection> for ::windows::core::IUnknown {
    fn from(value: IUICollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICollection> for ::windows::core::IUnknown {
    fn from(value: &IUICollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUICollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUICollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUICollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUICollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUICollection {}
unsafe impl ::windows::core::Interface for IUICollection {
    type Vtable = IUICollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf4f45bf_6f9d_4dd7_9d68_d8f9cd18c4db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUICollectionChangedEvent(::windows::core::IUnknown);
impl IUICollectionChangedEvent {
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn OnChanged<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: Param2, newindex: u32, newitem: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(action), ::core::mem::transmute(oldindex), olditem.into_param().abi(), ::core::mem::transmute(newindex), newitem.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUICollectionChangedEvent> for ::windows::core::IUnknown {
    fn from(value: IUICollectionChangedEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICollectionChangedEvent> for ::windows::core::IUnknown {
    fn from(value: &IUICollectionChangedEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUICollectionChangedEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUICollectionChangedEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUICollectionChangedEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUICollectionChangedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUICollectionChangedEvent {}
unsafe impl ::windows::core::Interface for IUICollectionChangedEvent {
    type Vtable = IUICollectionChangedEventVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6502ae91_a14d_44b5_bbd0_62aacc581d52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICollectionChangedEventVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut ::core::ffi::c_void, newindex: u32, newitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUICommandHandler(::windows::core::IUnknown);
impl IUICommandHandler {
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Execute<'a, Param4: ::windows::core::IntoParam<'a, IUISimplePropertySet>>(&self, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(verb), ::core::mem::transmute(key), ::core::mem::transmute(currentvalue), commandexecutionproperties.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn UpdateProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(key), ::core::mem::transmute(currentvalue), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
impl ::core::convert::From<IUICommandHandler> for ::windows::core::IUnknown {
    fn from(value: IUICommandHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICommandHandler> for ::windows::core::IUnknown {
    fn from(value: &IUICommandHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUICommandHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUICommandHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUICommandHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUICommandHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUICommandHandler {}
unsafe impl ::windows::core::Interface for IUICommandHandler {
    type Vtable = IUICommandHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75ae0a2d_dc03_4c9f_8883_069660d0beb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommandHandlerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, newvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUIContextualUI(::windows::core::IUnknown);
impl IUIContextualUI {
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn ShowAtLocation(&self, x: i32, y: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
}
impl ::core::convert::From<IUIContextualUI> for ::windows::core::IUnknown {
    fn from(value: IUIContextualUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIContextualUI> for ::windows::core::IUnknown {
    fn from(value: &IUIContextualUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIContextualUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUIContextualUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIContextualUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIContextualUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIContextualUI {}
unsafe impl ::windows::core::Interface for IUIContextualUI {
    type Vtable = IUIContextualUIVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea11f37_7c46_437c_8e55_b52122b29293);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContextualUIVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUIEventLogger(::windows::core::IUnknown);
impl IUIEventLogger {
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUIEvent(&self, peventparams: *const UI_EVENTPARAMS) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(peventparams))
    }
}
impl ::core::convert::From<IUIEventLogger> for ::windows::core::IUnknown {
    fn from(value: IUIEventLogger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIEventLogger> for ::windows::core::IUnknown {
    fn from(value: &IUIEventLogger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIEventLogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUIEventLogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIEventLogger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIEventLogger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIEventLogger {}
unsafe impl ::windows::core::Interface for IUIEventLogger {
    type Vtable = IUIEventLoggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec3e1034_dbf4_41a1_95d5_03e0f1026e05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIEventLoggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventparams: *const UI_EVENTPARAMS),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUIEventingManager(::windows::core::IUnknown);
impl IUIEventingManager {
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn SetEventLogger<'a, Param0: ::windows::core::IntoParam<'a, IUIEventLogger>>(&self, eventlogger: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), eventlogger.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIEventingManager> for ::windows::core::IUnknown {
    fn from(value: IUIEventingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIEventingManager> for ::windows::core::IUnknown {
    fn from(value: &IUIEventingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIEventingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUIEventingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIEventingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIEventingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIEventingManager {}
unsafe impl ::windows::core::Interface for IUIEventingManager {
    type Vtable = IUIEventingManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3be6ea7f_9a9b_4198_9368_9b0f923bd534);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIEventingManagerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventlogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUIFramework(::windows::core::IUnknown);
impl IUIFramework {
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, IUIApplication>>(&self, framewnd: Param0, application: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), framewnd.into_param().abi(), application.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadUI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, instance: Param0, resourcename: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), instance.into_param().abi(), resourcename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn GetView(&self, viewid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewid), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn InvalidateUICommand(&self, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(flags), ::core::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn FlushPendingInvalidations(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn SetModes(&self, imodes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(imodes)).ok()
    }
}
impl ::core::convert::From<IUIFramework> for ::windows::core::IUnknown {
    fn from(value: IUIFramework) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIFramework> for ::windows::core::IUnknown {
    fn from(value: &IUIFramework) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIFramework {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUIFramework {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIFramework {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIFramework {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIFramework {}
unsafe impl ::windows::core::Interface for IUIFramework {
    type Vtable = IUIFrameworkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4f0385d_6872_43a8_ad09_4c339cb3f5c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIFrameworkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imodes: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUIImage(::windows::core::IUnknown);
impl IUIImage {
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Graphics_Gdi'*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetBitmap(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__: super::super::Graphics::Gdi::HBITMAP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Graphics::Gdi::HBITMAP>(result__)
    }
}
impl ::core::convert::From<IUIImage> for ::windows::core::IUnknown {
    fn from(value: IUIImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIImage> for ::windows::core::IUnknown {
    fn from(value: &IUIImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUIImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIImage {}
unsafe impl ::windows::core::Interface for IUIImage {
    type Vtable = IUIImageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23c8c838_4de6_436b_ab01_5554bb7c30dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIImageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUIImageFromBitmap(::windows::core::IUnknown);
impl IUIImageFromBitmap {
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Graphics_Gdi'*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateImage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, bitmap: Param0, options: UI_OWNERSHIP) -> ::windows::core::Result<IUIImage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bitmap.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IUIImage>(result__)
    }
}
impl ::core::convert::From<IUIImageFromBitmap> for ::windows::core::IUnknown {
    fn from(value: IUIImageFromBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIImageFromBitmap> for ::windows::core::IUnknown {
    fn from(value: &IUIImageFromBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIImageFromBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUIImageFromBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIImageFromBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIImageFromBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIImageFromBitmap {}
unsafe impl ::windows::core::Interface for IUIImageFromBitmap {
    type Vtable = IUIImageFromBitmapVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18aba7f3_4c1c_4ba2_bf6c_f5c3326fa816);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIImageFromBitmapVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUIRibbon(::windows::core::IUnknown);
impl IUIRibbon {
    #[doc = "*Required features: 'Win32_UI_Ribbon'*"]
    pub unsafe fn GetHeight(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadSettingsFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveSettingsToStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIRibbon> for ::windows::core::IUnknown {
    fn from(value: IUIRibbon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIRibbon> for ::windows::core::IUnknown {
    fn from(value: &IUIRibbon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIRibbon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUIRibbon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIRibbon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIRibbon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIRibbon {}
unsafe impl ::windows::core::Interface for IUIRibbon {
    type Vtable = IUIRibbonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x803982ab_370a_4f7e_a9e7_8784036a6e26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIRibbonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cy: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
#[repr(transparent)]
pub struct IUISimplePropertySet(::windows::core::IUnknown);
impl IUISimplePropertySet {
    #[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
impl ::core::convert::From<IUISimplePropertySet> for ::windows::core::IUnknown {
    fn from(value: IUISimplePropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUISimplePropertySet> for ::windows::core::IUnknown {
    fn from(value: &IUISimplePropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUISimplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUISimplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUISimplePropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUISimplePropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUISimplePropertySet {}
unsafe impl ::windows::core::Interface for IUISimplePropertySet {
    type Vtable = IUISimplePropertySetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc205bb48_5b1c_4219_a106_15bd0a5f24e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISimplePropertySetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
);
pub const LIBID_UIRibbon: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x942f35c2_e83b_45ef_b085_ac295dd63d5b);
pub const UIRibbonFramework: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x926749fa_2615_4987_8845_c33e65f2b957);
pub const UIRibbonImageFromBitmapFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f7434b6_59b6_4250_999e_d168d6ae4293);
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_ALL_COMMANDS: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_COLLECTIONCHANGE = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COLLECTIONCHANGE_INSERT: UI_COLLECTIONCHANGE = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COLLECTIONCHANGE_REMOVE: UI_COLLECTIONCHANGE = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COLLECTIONCHANGE_REPLACE: UI_COLLECTIONCHANGE = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COLLECTIONCHANGE_RESET: UI_COLLECTIONCHANGE = 3i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_COMMANDTYPE = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_UNKNOWN: UI_COMMANDTYPE = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_GROUP: UI_COMMANDTYPE = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_ACTION: UI_COMMANDTYPE = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_ANCHOR: UI_COMMANDTYPE = 3i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_CONTEXT: UI_COMMANDTYPE = 4i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_COLLECTION: UI_COMMANDTYPE = 5i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_COMMANDCOLLECTION: UI_COMMANDTYPE = 6i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_DECIMAL: UI_COMMANDTYPE = 7i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_BOOLEAN: UI_COMMANDTYPE = 8i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_FONT: UI_COMMANDTYPE = 9i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_RECENTITEMS: UI_COMMANDTYPE = 10i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_COLORANCHOR: UI_COMMANDTYPE = 11i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_COMMANDTYPE_COLORCOLLECTION: UI_COMMANDTYPE = 12i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_CONTEXTAVAILABILITY = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_CONTEXTAVAILABILITY_NOTAVAILABLE: UI_CONTEXTAVAILABILITY = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_CONTEXTAVAILABILITY_AVAILABLE: UI_CONTEXTAVAILABILITY = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_CONTEXTAVAILABILITY_ACTIVE: UI_CONTEXTAVAILABILITY = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_CONTROLDOCK = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_CONTROLDOCK_TOP: UI_CONTROLDOCK = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_CONTROLDOCK_BOTTOM: UI_CONTROLDOCK = 3i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_EVENTLOCATION = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTLOCATION_Ribbon: UI_EVENTLOCATION = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTLOCATION_QAT: UI_EVENTLOCATION = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTLOCATION_ApplicationMenu: UI_EVENTLOCATION = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTLOCATION_ContextPopup: UI_EVENTLOCATION = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UI_EVENTPARAMS {
    pub EventType: UI_EVENTTYPE,
    pub Anonymous: UI_EVENTPARAMS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UI_EVENTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UI_EVENTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for UI_EVENTPARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UI_EVENTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UI_EVENTPARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UI_EVENTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UI_EVENTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union UI_EVENTPARAMS_0 {
    pub Modes: i32,
    pub Params: UI_EVENTPARAMS_COMMAND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UI_EVENTPARAMS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UI_EVENTPARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for UI_EVENTPARAMS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UI_EVENTPARAMS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UI_EVENTPARAMS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UI_EVENTPARAMS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UI_EVENTPARAMS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Ribbon', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UI_EVENTPARAMS_COMMAND {
    pub CommandID: u32,
    pub CommandName: super::super::Foundation::PWSTR,
    pub ParentCommandID: u32,
    pub ParentCommandName: super::super::Foundation::PWSTR,
    pub SelectionIndex: u32,
    pub Location: UI_EVENTLOCATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UI_EVENTPARAMS_COMMAND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UI_EVENTPARAMS_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for UI_EVENTPARAMS_COMMAND {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UI_EVENTPARAMS_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UI_EVENTPARAMS_COMMAND>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UI_EVENTPARAMS_COMMAND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UI_EVENTPARAMS_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_EVENTTYPE = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTTYPE_ApplicationMenuOpened: UI_EVENTTYPE = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTTYPE_RibbonMinimized: UI_EVENTTYPE = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTTYPE_RibbonExpanded: UI_EVENTTYPE = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTTYPE_ApplicationModeSwitched: UI_EVENTTYPE = 3i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTTYPE_TabActivated: UI_EVENTTYPE = 4i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTTYPE_MenuOpened: UI_EVENTTYPE = 5i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTTYPE_CommandExecuted: UI_EVENTTYPE = 6i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EVENTTYPE_TooltipShown: UI_EVENTTYPE = 7i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_EXECUTIONVERB = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EXECUTIONVERB_EXECUTE: UI_EXECUTIONVERB = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EXECUTIONVERB_PREVIEW: UI_EXECUTIONVERB = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_EXECUTIONVERB_CANCELPREVIEW: UI_EXECUTIONVERB = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_FONTDELTASIZE = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTDELTASIZE_GROW: UI_FONTDELTASIZE = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTDELTASIZE_SHRINK: UI_FONTDELTASIZE = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_FONTPROPERTIES = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTPROPERTIES_NOTAVAILABLE: UI_FONTPROPERTIES = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTPROPERTIES_NOTSET: UI_FONTPROPERTIES = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTPROPERTIES_SET: UI_FONTPROPERTIES = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_FONTUNDERLINE = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTUNDERLINE_NOTAVAILABLE: UI_FONTUNDERLINE = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTUNDERLINE_NOTSET: UI_FONTUNDERLINE = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTUNDERLINE_SET: UI_FONTUNDERLINE = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_FONTVERTICALPOSITION = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTVERTICALPOSITION_NOTAVAILABLE: UI_FONTVERTICALPOSITION = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTVERTICALPOSITION_NOTSET: UI_FONTVERTICALPOSITION = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTVERTICALPOSITION_SUPERSCRIPT: UI_FONTVERTICALPOSITION = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_FONTVERTICALPOSITION_SUBSCRIPT: UI_FONTVERTICALPOSITION = 3i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_INVALIDATIONS = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_INVALIDATIONS_STATE: UI_INVALIDATIONS = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_INVALIDATIONS_VALUE: UI_INVALIDATIONS = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_INVALIDATIONS_PROPERTY: UI_INVALIDATIONS = 4i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_INVALIDATIONS_ALLPROPERTIES: UI_INVALIDATIONS = 8i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_OWNERSHIP = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_OWNERSHIP_TRANSFER: UI_OWNERSHIP = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_OWNERSHIP_COPY: UI_OWNERSHIP = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_SWATCHCOLORMODE = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_SWATCHCOLORMODE_NORMAL: UI_SWATCHCOLORMODE = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_SWATCHCOLORMODE_MONOCHROME: UI_SWATCHCOLORMODE = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_SWATCHCOLORTYPE = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_SWATCHCOLORTYPE_NOCOLOR: UI_SWATCHCOLORTYPE = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_SWATCHCOLORTYPE_AUTOMATIC: UI_SWATCHCOLORTYPE = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_SWATCHCOLORTYPE_RGB: UI_SWATCHCOLORTYPE = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_VIEWTYPE = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_VIEWTYPE_RIBBON: UI_VIEWTYPE = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub type UI_VIEWVERB = i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_VIEWVERB_CREATE: UI_VIEWVERB = 0i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_VIEWVERB_DESTROY: UI_VIEWVERB = 1i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_VIEWVERB_SIZE: UI_VIEWVERB = 2i32;
#[doc = "*Required features: 'Win32_UI_Ribbon'*"]
pub const UI_VIEWVERB_ERROR: UI_VIEWVERB = 3i32;
