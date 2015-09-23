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
use libc::uint32_t;

// -------------------------------------------------------------------------------------------------
// TYPES
// -------------------------------------------------------------------------------------------------

pub type InputFormat = VCOSInputFormat;

// -------------------------------------------------------------------------------------------------
// ENUMS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub enum _3dFormat {
    DISPLAY_3D_UNSUPPORTED = 0, // default
    DISPLAY_3D_INTERLEAVED,     // for autosteroscopic displays
    DISPLAY_3D_SBS_FULL_AUTO,   // side-by-side, full width (also used by some autostereoscopic displays)
    DISPLAY_3D_SBS_HALF_HORIZ,  // side-by-side, half width, horizontal subsampling (see HDMI spec)
    DISPLAY_3D_TB_HALF,         // top-bottom 3D
    DISPLAY_3D_FORMAT_MAX
}

#[repr(C)]
pub enum Dither {
    DISPLAY_DITHER_NONE   = 0, // default if not set
    DISPLAY_DITHER_RGB666 = 1,
    DISPLAY_DITHER_RGB565 = 2,
    DISPLAY_DITHER_RGB555 = 3,
    DISPLAY_DITHER_MAX
}

#[repr(C)]
pub enum Interface {
    DISPLAY_INTERFACE_MIN,
    DISPLAY_INTERFACE_SMI,
    DISPLAY_INTERFACE_DPI,
    DISPLAY_INTERFACE_DSI,
    DISPLAY_INTERFACE_LVDS,
    DISPLAY_INTERFACE_MAX
}

#[repr(C)]
pub enum VCOSInputFormat {
    VCOS_DISPLAY_INPUT_FORMAT_INVALID = 0,
    VCOS_DISPLAY_INPUT_FORMAT_RGB888,
    VCOS_DISPLAY_INPUT_FORMAT_RGB565
}

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

pub struct Info {
   pub type_:            Interface,
   pub width:            uint32_t,
   pub height:           uint32_t,
   pub input_format:     InputFormat,
   pub interlaced:       uint32_t,
   pub output_dither:    Dither,
   pub pixel_freq:       uint32_t,
   pub line_rate:        uint32_t,
   pub format_3d:        _3dFormat,
   pub use_pixelvalve_1: uint32_t,
   pub dsi_video_mode:   uint32_t,
   pub hvs_channel:      uint32_t
}