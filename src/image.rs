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

use libc::int32_t;

// -------------------------------------------------------------------------------------------------
// ENUMS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub enum ImageBayerFormat {
    //defined to be identical to register bits
    IMAGE_BAYER_RAW6             = 0,
    IMAGE_BAYER_RAW7             = 1,
    IMAGE_BAYER_RAW8             = 2,
    IMAGE_BAYER_RAW10            = 3,
    IMAGE_BAYER_RAW12            = 4,
    IMAGE_BAYER_RAW14            = 5,
    IMAGE_BAYER_RAW16            = 6,
    IMAGE_BAYER_RAW10_8          = 7,
    IMAGE_BAYER_RAW12_8          = 8,
    IMAGE_BAYER_RAW14_8          = 9,
    IMAGE_BAYER_RAW10L           = 11,
    IMAGE_BAYER_RAW12L           = 12,
    IMAGE_BAYER_RAW14L           = 13,
    IMAGE_BAYER_RAW16_BIG_ENDIAN = 14,
    IMAGE_BAYER_RAW4             = 15,
}

#[repr(C)]
pub enum ImageBayerOrder {
    //defined to be identical to register bits
    IMAGE_BAYER_RGGB = 0,
    IMAGE_BAYER_GBRG = 1,
    IMAGE_BAYER_BGGR = 2,
    IMAGE_BAYER_GRBG = 3
}

#[repr(C)]
pub enum ImageTransform {
    IMAGE_ROT0           = 0,
    IMAGE_MIRROR_ROT0    = (1<<0),
    IMAGE_MIRROR_ROT180  = (1<<1),
    IMAGE_ROT180         = (1<<0)|(1<<1),
    IMAGE_MIRROR_ROT90   = (1<<2),
    IMAGE_ROT270         = (1<<2)|(1<<0),
    IMAGE_ROT90          = (1<<2)|(1<<1),
    IMAGE_MIRROR_ROT270  = (1<<2)|(1<<0)|(1<<1)
}

#[repr(C)]
pub enum ImageType {
    IMAGE_MIN    = 0,    //bounds for error checking
    IMAGE_RGB565 = 1,
    IMAGE_1BPP,
    IMAGE_YUV420,
    IMAGE_48BPP,
    IMAGE_RGB888,
    IMAGE_8BPP,
    IMAGE_4BPP,          // 4bpp palettised image
    IMAGE_3D32,          // A separated format of 16 colour/light shorts followed by 16 z values
    IMAGE_3D32B,         // 16 colours followed by 16 z values
    IMAGE_3D32MAT,       // A separated format of 16 material/colour/light shorts followed by 16 z values
    IMAGE_RGB2X9,        // 32 bit format containing 18 bits of 6.6.6 RGB, 9 bits per short
    IMAGE_RGB666,        // 32-bit format holding 18 bits of 6.6.6 RGB
    IMAGE_PAL4_OBSOLETE, // 4bpp palettised image with embedded palette
    IMAGE_PAL8_OBSOLETE, // 8bpp palettised image with embedded palette
    IMAGE_RGBA32,        // RGB888 with an alpha byte after each pixel  // xxx: isn't it BEFORE each pixel?
    IMAGE_YUV422,        // a line of Y (32-byte padded), a line of U (16-byte padded), and a line of V (16-byte padded)
    IMAGE_RGBA565,       // RGB565 with a transparent patch
    IMAGE_RGBA16,        // Compressed (4444) version of RGBA32
    IMAGE_YUV_UV,        // VCIII codec format
    IMAGE_TF_RGBA32,     // VCIII T-format RGBA8888
    IMAGE_TF_RGBX32,     // VCIII T-format RGBx8888
    IMAGE_TF_FLOAT,      // VCIII T-format float
    IMAGE_TF_RGBA16,     // VCIII T-format RGBA4444
    IMAGE_TF_RGBA5551,   // VCIII T-format RGB5551
    IMAGE_TF_RGB565,     // VCIII T-format RGB565
    IMAGE_TF_YA88,       // VCIII T-format 8-bit luma and 8-bit alpha
    IMAGE_TF_BYTE,       // VCIII T-format 8 bit generic sample
    IMAGE_TF_PAL8,       // VCIII T-format 8-bit palette
    IMAGE_TF_PAL4,       // VCIII T-format 4-bit palette
    IMAGE_TF_ETC1,       // VCIII T-format Ericsson Texture Compressed
    IMAGE_BGR888,        // RGB888 with R & B swapped
    IMAGE_BGR888_NP,     // RGB888 with R & B swapped, but with no pitch, i.e. no padding after each row of pixels
    IMAGE_BAYER,         // Bayer image, extra defines which variant is being used
    IMAGE_CODEC,         // General wrapper for codec images e.g. JPEG from camera
    IMAGE_YUV_UV32,      // VCIII codec format
    IMAGE_TF_Y8,         // VCIII T-format 8-bit luma
    IMAGE_TF_A8,         // VCIII T-format 8-bit alpha
    IMAGE_TF_SHORT,      // VCIII T-format 16-bit generic sample
    IMAGE_TF_1BPP,       // VCIII T-format 1bpp black/white
    IMAGE_OPENGL,
    IMAGE_YUV444I,       // VCIII-B0 HVS YUV 4:4:4 interleaved samples
    IMAGE_YUV422PLANAR,  // Y, U, & V planes separately (IMAGE_YUV422 has them interleaved on a per line basis)
    IMAGE_ARGB8888,      // 32bpp with 8bit alpha at MS byte, with R, G, B (LS byte)
    IMAGE_XRGB8888,      // 32bpp with 8bit unused at MS byte, with R, G, B (LS byte)
    IMAGE_YUV422YUYV,    // interleaved 8 bit samples of Y, U, Y, V
    IMAGE_YUV422YVYU,    // interleaved 8 bit samples of Y, V, Y, U
    IMAGE_YUV422UYVY,    // interleaved 8 bit samples of U, Y, V, Y
    IMAGE_YUV422VYUY,    // interleaved 8 bit samples of V, Y, U, Y
    IMAGE_RGBX32,        // 32bpp like RGBA32 but with unused alpha
    IMAGE_RGBX8888,      // 32bpp, corresponding to RGBA with unused alpha
    IMAGE_BGRX8888,      // 32bpp, corresponding to BGRA with unused alpha
    IMAGE_YUV420SP,      // Y as a plane, then UV byte interleaved in plane with with same pitch, half height
    IMAGE_YUV444PLANAR,  // Y, U, & V planes separately 4:4:4
    IMAGE_TF_U8,         // T-format 8-bit U - same as TF_Y8 buf from U plane
    IMAGE_TF_V8,         // T-format 8-bit U - same as TF_Y8 buf from V plane
    IMAGE_MAX,           // bounds for error checking
    IMAGE_FORCE_ENUM_16BIT = 0xffff,
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