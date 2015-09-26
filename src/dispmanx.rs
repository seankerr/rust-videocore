// +-----------------------------------------------------------------------------------------------+
// | Copyright 2015 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0 (the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License Author                                                   |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@code-box.org>                                                         |
// +-----------------------------------------------------------------------------------------------+

#![allow(dead_code)]

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

// system
use libc::{ c_int,
            c_void,
            int32_t,
            uint8_t,
            uint32_t };

// local
use display::{ _3dFormat,
               Info,
               InputFormat };

use image::{ Image,
             ImageType,
             Rect };

use vchi::MemHandle;

// -------------------------------------------------------------------------------------------------
// TYPES
// -------------------------------------------------------------------------------------------------

pub type DisplayHandle  = uint32_t;
pub type ElementHandle  = uint32_t;
pub type ResourceHandle = uint32_t;
pub type UpdateHandle   = uint32_t;

pub type CallbackFunc = extern "C" fn(handle: UpdateHandle, arg: *mut c_void);
pub type Protection   = uint32_t;

// -------------------------------------------------------------------------------------------------
// ENUMS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub enum FlagsAlpha {
    // bottom 2 bits sets the alpha mode
    FROM_SOURCE       = 0,
    FIXED_ALL_PIXELS  = 1,
    FIXED_NON_ZERO    = 2,
    FIXED_EXCEED_0X07 = 3,

    PREMULT = 1 << 16,
    MIX     = 1 << 17
}

#[repr(C)]
pub enum FlagsClamp {
    NONE             = 0,
    LUMA_TRANSPARENT = 1,
    TRANSPARENT      = 2,
    REPLACE          = 3
}

#[repr(C)]
pub enum FlagsKeymask {
    OVERRIDE = 1,
    SMOOTH   = 1 << 1,
    CR_INV   = 1 << 2,
    CB_INV   = 1 << 3,
    YY_INV   = 1 << 4
}

#[repr(C)]
pub enum Status {
    SUCCESS = 0,
    INVALID = -1
}

#[repr(C)]
pub enum Transform {
    NO_ROTATE  = 0,
    ROTATE_90  = 1,
    ROTATE_180 = 2,
    ROTATE_270 = 3,

    FLIP_HRIZ = 1 << 16,
    FLIP_VERT = 1 << 17,

    // extra flags for controlling snapshot behaviour
    SNAPSHOT_NO_YUV        = 1 << 24,
    SNAPSHOT_NO_RGB        = 1 << 25,
    SNAPSHOT_FILL          = 1 << 26,
    SNAPSHOT_SWAP_RED_BLUE = 1 << 27,
    SNAPSHOT_PACK          = 1 << 28
}

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub struct Alpha {
    pub flags:   FlagsAlpha,
    pub opacity: uint32_t,
    pub mask:    Image
}

#[repr(C)]
pub struct Clamp {
    pub mode:          FlagsClamp,
    pub key_mask:      FlagsKeymask,
    pub key_value:     *mut c_void,
    pub replace_value: uint32_t
}

#[repr(C)]
pub struct ClampKeysRGB {
    pub red_upper:   uint8_t,
    pub red_lower:   uint8_t,
    pub blue_upper:  uint8_t,
    pub blue_lower:  uint8_t,
    pub green_upper: uint8_t,
    pub green_lower: uint8_t
}

#[repr(C)]
pub struct ClampKeysYUV {
    pub yy_upper: uint8_t,
    pub yy_lower: uint8_t,
    pub cr_upper: uint8_t,
    pub cr_lower: uint8_t,
    pub cb_upper: uint8_t,
    pub cb_lower: uint8_t
}

#[repr(C)]
pub struct DisplayFuncs {
    pub get_hvs_config: extern "C" fn(instance: *mut c_void, pchan: *mut uint32_t,
                                      poptions: *mut uint32_t, info: *mut Info,
                                      bg_color: *mut uint32_t, test_mode: *mut uint32_t),

    pub get_gamma_params: extern "C" fn(instance: *mut c_void,
                                        gain: [int32_t; 3], offset: [int32_t; 3],
                                        gamma: [int32_t; 3]),

    pub get_oled_params: extern "C" fn(instance: *mut c_void, poffsets: *mut uint32_t,
                                       coeffs: [uint32_t; 3]) -> int32_t,

    pub get_dither: extern "C" fn(instance: *mut c_void, dither_depth: *mut uint32_t,
                                  dither_type: *mut uint32_t) -> int32_t,

    pub get_info: extern "C" fn(instance: *mut c_void, info: *mut Modeinfo) -> int32_t,

    pub open: extern "C" fn(instance: *mut c_void) -> int32_t,

    pub close: extern "C" fn(instance: *mut c_void) -> int32_t,

    // todo: fifo_reg should be volatile
    pub dlist_updated: extern "C" fn(instance: *mut c_void, fifo_reg: *mut uint32_t),

    pub eof_callback: extern "C" fn(instance: *mut c_void),

    pub get_input_format: extern "C" fn(instance: *mut c_void) -> InputFormat,

    pub suspend_resume: extern "C" fn(instance: *mut c_void, up: int32_t) -> int32_t,

    pub get_3d_format: extern "C" fn(instance: *mut c_void) -> _3dFormat
}

#[repr(C)]
pub struct Modeinfo {
    pub width:        int32_t,
    pub height:       int32_t,
    pub transform:    Transform,
    pub input_format: InputFormat
}

#[repr(C)]
pub struct VCAlpha {
    pub flags:   FlagsAlpha,
    pub opacity: uint32_t,
    pub mask:    ResourceHandle
}

#[repr(C)]
pub struct Window {
    pub element: ElementHandle,
    pub width:   c_int,
    pub height:  c_int
}

// -------------------------------------------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------------------------------------------

pub const DISPMANX_NO_HANDLE:       uint32_t = 0;
pub const DISPMANX_PROTECTION_MAX:  uint32_t = 0x0f;
pub const DISPMANX_PROTECTION_NONE: uint32_t = 0;
pub const DISPMANX_PROTECTION_HDCP: uint32_t = 11; // derived from the WM DRM levels, 101-300

pub const DISPMANX_ID_MAIN_LCD:    uint32_t = 0;
pub const DISPMANX_ID_AUX_LCD:     uint32_t = 1;
pub const DISPMANX_ID_HDMI:        uint32_t = 2;
pub const DISPMANX_ID_SDTV:        uint32_t = 3;
pub const DISPMANX_ID_FORCE_LCD:   uint32_t = 4;
pub const DISPMANX_ID_FORCE_TV:    uint32_t = 5;
pub const DISPMANX_ID_FORCE_OTHER: uint32_t = 6; // non-default display

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------

pub fn display_close(display: DisplayHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_close(display) > 0
    }
}

pub fn display_get_info(display: DisplayHandle, modeinfo: *mut Modeinfo) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_get_info(display, modeinfo) > 0
    }
}

pub fn display_open(device: uint32_t) -> DisplayHandle {
    unsafe {
        ffi::vc_dispmanx_display_open(device)
    }
}

pub fn display_open_mode(device: uint32_t, mode: uint32_t) -> DisplayHandle {
    unsafe {
        ffi::vc_dispmanx_display_open_mode(device, mode)
    }
}

pub fn display_open_offscreen(dest: ResourceHandle, orientation: Transform) -> DisplayHandle {
    unsafe {
        ffi::vc_dispmanx_display_open_offscreen(dest, orientation)
    }
}

pub fn display_reconfigure(display: DisplayHandle, mode: uint32_t) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_reconfigure(display, mode) > 0
    }
}

pub fn display_set_background(update: UpdateHandle, display: DisplayHandle,
                              red: uint8_t, green: uint8_t, blue: uint8_t) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_set_background(update, display, red, green, blue) > 0
    }
}

pub fn display_set_destination(display: DisplayHandle, dest: ResourceHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_set_destination(display, dest) > 0
    }
}

pub fn element_add(update: UpdateHandle, display: DisplayHandle, layer: int32_t,
                   dest_rect: *mut Rect, src: ResourceHandle, src_rect: *mut Rect,
                   protection: Protection, alpha: *mut VCAlpha, clamp: *mut Clamp,
                   transform: Transform) -> ElementHandle {
    unsafe {
        ffi::vc_dispmanx_element_add(update, display, layer, dest_rect, src, src_rect,
                                     protection, alpha, clamp, transform)
    }
}

pub fn element_change_attributes(update: UpdateHandle, element: ElementHandle,
                                 change_flags: uint32_t, layer: int32_t, opacity: uint8_t,
                                 dest_rect: *const Rect, src_rect: *const Rect,
                                 mask: ResourceHandle, transform: Transform) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_change_attributes(update, element, change_flags, layer,
                                                   opacity, dest_rect, src_rect,
                                                   mask, transform) > 0
    }
}

pub fn element_change_layer(update: UpdateHandle, element: ElementHandle,
                            layer: int32_t) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_change_layer(update, element, layer) > 0
    }
}

pub fn element_change_source(update: UpdateHandle, element: ElementHandle,
                             src: ResourceHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_change_source(update, element, src) > 0
    }
}

pub fn element_modified(update: UpdateHandle, element: ElementHandle, rect: *mut Rect) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_modified(update, element, rect) > 0
    }
}

pub fn element_remove(update: UpdateHandle, element: ElementHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_remove(update, element) > 0
    }
}

pub fn query_image_formats(supported_formats: *mut uint32_t) -> bool {
    unsafe {
        ffi::vc_dispmanx_query_image_formats(supported_formats) > 0
    }
}

pub fn rect_set(rect: *mut Rect, x_offset: uint32_t, y_offset: uint32_t, width: uint32_t,
                height: uint32_t) -> bool {
    unsafe {
        ffi::vc_dispmanx_rect_set(rect, x_offset, y_offset, width, height) > 0
    }
}

pub fn resource_create(type_: ImageType, width: uint32_t, height: uint32_t,
                       native_image_handle: *mut uint32_t) -> ResourceHandle {
    unsafe {
        ffi::vc_dispmanx_resource_create(type_, width, height, native_image_handle)
    }
}

pub fn resource_delete(res: ResourceHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_delete(res) > 0
    }
}

pub fn resource_read_data(res: ResourceHandle, rect: *const Rect, dst_address: *mut c_void,
                          dst_pitch: uint32_t) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_read_data(res, rect, dst_address, dst_pitch) > 0
    }
}

pub fn resource_set_palette(res: ResourceHandle, src_address: *mut c_void, offset: int32_t,
                            size: int32_t) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_set_palette(res, src_address, offset, size) > 0
    }
}

pub fn resource_write_data(res: ResourceHandle, src_type: ImageType, src_pitch: int32_t,
                           src_address: *mut c_void, rect: *const Rect) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_write_data(res, src_type, src_pitch, src_address, rect) > 0
    }
}

pub fn resource_write_data_handle(res: ResourceHandle, src_type: ImageType, src_pitch: int32_t,
                                  handle: MemHandle, offset: uint32_t,
                                  rect: *const Rect) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_write_data_handle(res, src_type, src_pitch, handle, offset,
                                                    rect) > 0
    }
}

pub fn snapshot(display: DisplayHandle, snapshot_resource: ResourceHandle,
                transform: Transform) -> bool {
    unsafe {
        ffi::vc_dispmanx_snapshot(display, snapshot_resource, transform) > 0
    }
}

pub fn stop() {
    unsafe {
        ffi::vc_dispmanx_stop()
    }
}

pub fn update_start(priority: int32_t) -> UpdateHandle {
    unsafe {
        ffi::vc_dispmanx_update_start(priority)
    }
}

pub fn update_submit(update: UpdateHandle, callback_func: CallbackFunc,
                     callback_arg: *mut c_void) -> bool {
    unsafe {
        ffi::vc_dispmanx_update_submit(update, callback_func, callback_arg) > 0
    }
}

pub fn update_submit_sync(update: UpdateHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_update_submit_sync(update) > 0
    }
}

pub fn vsync_callback(display: DisplayHandle, callback_func: CallbackFunc,
                      callback_arg: *mut c_void) -> bool {
    unsafe {
        ffi::vc_dispmanx_vsync_callback(display, callback_func, callback_arg) > 0
    }
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------

mod ffi {
    use libc::{ c_void,
                int32_t,
                uint8_t,
                uint32_t };

    use image::{ ImageType,
                 Rect };

    use vchi::MemHandle;

    use super::*;

    extern {
        // deprecated
        pub fn vc_dispman_init() -> int32_t;

        pub fn vc_dispmanx_display_close(display: DisplayHandle) -> int32_t;

        pub fn vc_dispmanx_display_get_info(display: DisplayHandle,
                                            pinfo: *mut Modeinfo) -> int32_t;

        pub fn vc_dispmanx_display_open(device: uint32_t) -> DisplayHandle;

        pub fn vc_dispmanx_display_open_mode(device: uint32_t, mode: uint32_t) -> DisplayHandle;

        pub fn vc_dispmanx_display_open_offscreen(dest: ResourceHandle,
                                                  orientation: Transform) -> DisplayHandle;

        pub fn vc_dispmanx_display_reconfigure(display: DisplayHandle, mode: uint32_t) -> int32_t;

        pub fn vc_dispmanx_display_set_background(update: UpdateHandle,
                                                  display: DisplayHandle,
                                                  red: uint8_t, green: uint8_t,
                                                  blue: uint8_t) -> int32_t;

        pub fn vc_dispmanx_display_set_destination(display: DisplayHandle,
                                                   dest: ResourceHandle) -> int32_t;

        pub fn vc_dispmanx_element_add(update: UpdateHandle, display: DisplayHandle,
                                       layer: int32_t, dest_rect: *mut Rect,
                                       src: ResourceHandle, src_rect: *mut Rect,
                                       protection: Protection, alpha: *mut VCAlpha,
                                       clamp: *mut Clamp, transform: Transform) -> ElementHandle;

        pub fn vc_dispmanx_element_change_attributes(update: UpdateHandle,
                                                     element: ElementHandle,
                                                     change_flags: uint32_t, layer: int32_t,
                                                     opacity: uint8_t, dest_rect: *const Rect,
                                                     src_rect: *const Rect, mask: ResourceHandle,
                                                     transform: Transform) -> int32_t;

        pub fn vc_dispmanx_element_change_layer(update: UpdateHandle, element: ElementHandle,
                                                layer: int32_t) -> int32_t;

        pub fn vc_dispmanx_element_change_source(update: UpdateHandle, element: ElementHandle,
                                                 src: ResourceHandle) -> int32_t;

        pub fn vc_dispmanx_element_modified(update: UpdateHandle, element: ElementHandle,
                                            rect: *mut Rect) -> int32_t;

        pub fn vc_dispmanx_element_remove(update: UpdateHandle, element: ElementHandle) -> int32_t;

        pub fn vc_dispmanx_query_image_formats(supported_formats: *mut uint32_t) -> int32_t;

        pub fn vc_dispmanx_rect_set(rect: *mut Rect, x_offset: uint32_t, y_offset: uint32_t,
                                    width: uint32_t, height: uint32_t) -> int32_t;

        pub fn vc_dispmanx_resource_create(type_: ImageType, width: uint32_t, height: uint32_t,
                                           native_image_handle: *mut uint32_t) -> ResourceHandle;

        pub fn vc_dispmanx_resource_delete(res: ResourceHandle) -> int32_t;

        // deprecated
        pub fn vc_dispmanx_resource_get_image_handle(res: ResourceHandle) -> uint32_t;

        pub fn vc_dispmanx_resource_read_data(handle: ResourceHandle, p_rect: *const Rect,
                                              dst_address: *mut c_void,
                                              dst_pitch: uint32_t) -> int32_t;

        pub fn vc_dispmanx_resource_set_palette(handle: ResourceHandle,
                                                src_address: *mut c_void, offset: int32_t,
                                                size: int32_t) -> int32_t;

        pub fn vc_dispmanx_resource_write_data(res: ResourceHandle, src_type: ImageType,
                                               src_pitch: int32_t, src_address: *mut c_void,
                                               rect: *const Rect) -> int32_t;

        pub fn vc_dispmanx_resource_write_data_handle(res: ResourceHandle, src_type: ImageType,
                                                      src_pitch: int32_t, handle: MemHandle,
                                                      offset: uint32_t,
                                                      rect: *const Rect) -> int32_t;

        pub fn vc_dispmanx_snapshot(display: DisplayHandle,
                                    snapshot_resource: ResourceHandle,
                                    transform: Transform) -> int32_t;

        pub fn vc_dispmanx_stop();

        pub fn vc_dispmanx_update_start(priority: int32_t) -> UpdateHandle;

        pub fn vc_dispmanx_update_submit(update: UpdateHandle, cb_func: CallbackFunc,
                                         cb_arg: *mut c_void) -> int32_t;

        pub fn vc_dispmanx_update_submit_sync(update: UpdateHandle) -> int32_t;

        pub fn vc_dispmanx_vsync_callback(display: DisplayHandle, cb_func: CallbackFunc,
                                          cb_arg: *mut c_void) -> int32_t;

        // call this instead of vc_dispman_init()
        //pub fn vc_vchi_dispmanx_init(VCHI_INSTANCE_T initialise_instance, VCHI_CONNECTION_T **connections, uint32_t num_connections );
    }
}