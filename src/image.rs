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

use libc::int32_t;

// -------------------------------------------------------------------------------------------------
// ENUMS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub enum ImageBayerFormat {
    //defined to be identical to register bits
    RAW6             = 0,
    RAW7             = 1,
    RAW8             = 2,
    RAW10            = 3,
    RAW12            = 4,
    RAW14            = 5,
    RAW16            = 6,
    RAW10_8          = 7,
    RAW12_8          = 8,
    RAW14_8          = 9,
    RAW10L           = 11,
    RAW12L           = 12,
    RAW14L           = 13,
    RAW16_BIG_ENDIAN = 14,
    RAW4             = 15,
}

#[repr(C)]
pub enum ImageBayerOrder {
    //defined to be identical to register bits
    RGGB = 0,
    GBRG = 1,
    BGGR = 2,
    GRBG = 3
}

#[repr(C)]
pub enum ImageTransform {
    ROT0           = 0,
    MIRROR_ROT0    = (1<<0),
    MIRROR_ROT180  = (1<<1),
    ROT180         = (1<<0)|(1<<1),
    MIRROR_ROT90   = (1<<2),
    ROT270         = (1<<2)|(1<<0),
    ROT90          = (1<<2)|(1<<1),
    MIRROR_ROT270  = (1<<2)|(1<<0)|(1<<1)
}

#[repr(C)]
pub enum ImageType {
    MIN    = 0,    //bounds for error checking
    RGB565 = 1,
    _1BPP,
    YUV420,
    _48BPP,
    RGB888,
    _8BPP,
    _4BPP,          // 4bpp palettised image
    _3D32,          // A separated format of 16 colour/light shorts followed by 16 z values
    _3D32B,         // 16 colours followed by 16 z values
    _3D32MAT,       // A separated format of 16 material/colour/light shorts followed by 16 z values
    RGB2X9,        // 32 bit format containing 18 bits of 6.6.6 RGB, 9 bits per short
    RGB666,        // 32-bit format holding 18 bits of 6.6.6 RGB
    PAL4_OBSOLETE, // 4bpp palettised image with embedded palette
    PAL8_OBSOLETE, // 8bpp palettised image with embedded palette
    RGBA32,        // RGB888 with an alpha byte after each pixel  // xxx: isn't it BEFORE each pixel?
    YUV422,        // a line of Y (32-byte padded), a line of U (16-byte padded), and a line of V (16-byte padded)
    RGBA565,       // RGB565 with a transparent patch
    RGBA16,        // Compressed (4444) version of RGBA32
    YUV_UV,        // VCIII codec format
    TF_RGBA32,     // VCIII T-format RGBA8888
    TF_RGBX32,     // VCIII T-format RGBx8888
    TF_FLOAT,      // VCIII T-format float
    TF_RGBA16,     // VCIII T-format RGBA4444
    TF_RGBA5551,   // VCIII T-format RGB5551
    TF_RGB565,     // VCIII T-format RGB565
    TF_YA88,       // VCIII T-format 8-bit luma and 8-bit alpha
    TF_BYTE,       // VCIII T-format 8 bit generic sample
    TF_PAL8,       // VCIII T-format 8-bit palette
    TF_PAL4,       // VCIII T-format 4-bit palette
    TF_ETC1,       // VCIII T-format Ericsson Texture Compressed
    BGR888,        // RGB888 with R & B swapped
    BGR888_NP,     // RGB888 with R & B swapped, but with no pitch, i.e. no padding after each row of pixels
    BAYER,         // Bayer image, extra defines which variant is being used
    CODEC,         // General wrapper for codec images e.g. JPEG from camera
    YUV_UV32,      // VCIII codec format
    TF_Y8,         // VCIII T-format 8-bit luma
    TF_A8,         // VCIII T-format 8-bit alpha
    TF_SHORT,      // VCIII T-format 16-bit generic sample
    TF_1BPP,       // VCIII T-format 1bpp black/white
    OPENGL,
    YUV444I,       // VCIII-B0 HVS YUV 4:4:4 interleaved samples
    YUV422PLANAR,  // Y, U, & V planes separately (YUV422 has them interleaved on a per line basis)
    ARGB8888,      // 32bpp with 8bit alpha at MS byte, with R, G, B (LS byte)
    XRGB8888,      // 32bpp with 8bit unused at MS byte, with R, G, B (LS byte)
    YUV422YUYV,    // interleaved 8 bit samples of Y, U, Y, V
    YUV422YVYU,    // interleaved 8 bit samples of Y, V, Y, U
    YUV422UYVY,    // interleaved 8 bit samples of U, Y, V, Y
    YUV422VYUY,    // interleaved 8 bit samples of V, Y, U, Y
    RGBX32,        // 32bpp like RGBA32 but with unused alpha
    RGBX8888,      // 32bpp, corresponding to RGBA with unused alpha
    BGRX8888,      // 32bpp, corresponding to BGRA with unused alpha
    YUV420SP,      // Y as a plane, then UV byte interleaved in plane with with same pitch, half height
    YUV444PLANAR,  // Y, U, & V planes separately 4:4:4
    TF_U8,         // T-format 8-bit U - same as TF_Y8 buf from U plane
    TF_V8,         // T-format 8-bit U - same as TF_Y8 buf from V plane
    MAX,           // bounds for error checking
    FORCE_ENUM_16BIT = 0xffff,
}

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub struct Image;

#[repr(C)]
pub struct Rect {
    pub x:      int32_t,
    pub y:      int32_t,
    pub width:  int32_t,
    pub height: int32_t
}
