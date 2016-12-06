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
// | Author: Sean Kerr <sean@metatomic.io>                                                         |
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
    UNSUPPORTED = 0, // default
    INTERLEAVED,     // for autosteroscopic displays
    SBS_FULL_AUTO,   // side-by-side, full width (also used by some autostereoscopic displays)
    SBS_HALF_HORIZ,  // side-by-side, half width, horizontal subsampling (see HDMI spec)
    TB_HALF,         // top-bottom 3D
    MAX
}

#[repr(C)]
pub enum Dither {
    NONE   = 0, // default if not set
    RGB666 = 1,
    RGB565 = 2,
    RGB555 = 3,
    MAX
}

#[repr(C)]
pub enum Interface {
    MIN,
    SMI,
    DPI,
    DSI,
    LVDS,
    MAX
}

#[repr(C)]
pub enum VCOSInputFormat {
    INVALID = 0,
    RGB888,
    RGB565
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
