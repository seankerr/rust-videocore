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
// STRUCTS
// -------------------------------------------------------------------------------------------------

pub struct GraphicsDisplaySize {
    pub height: uint32_t,
    pub width:  uint32_t
}

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------

pub fn deinit() {
    unsafe {
        ffi::bcm_host_deinit();
    }
}

pub fn get_peripheral_address() -> uint32_t {
    unsafe {
        return ffi::bcm_host_get_peripheral_address();
    }
}

pub fn get_peripheral_size() -> uint32_t {
    unsafe {
        return ffi::bcm_host_get_peripheral_size();
    }
}

pub fn get_sdram_address() -> uint32_t {
    unsafe {
        return ffi::bcm_host_get_sdram_address();
    }
}

pub fn graphics_get_display_size(display_number: u16) -> Option<GraphicsDisplaySize> {
    unsafe {
        let mut width:  uint32_t = 0;
        let mut height: uint32_t = 0;

        if ffi::graphics_get_display_size(display_number, &mut width, &mut height) == 0 {
            Some(GraphicsDisplaySize {
                     height: height,
                     width:  width
                 })
        } else {
            None
        }
    }
}

pub fn init() {
    unsafe {
        ffi::bcm_host_init();
    }
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------

pub mod ffi {
    use libc::{ int32_t,
                uint16_t,
                uint32_t };

    extern {
        pub fn bcm_host_deinit();

        pub fn bcm_host_get_peripheral_address() -> uint32_t;

        pub fn bcm_host_get_peripheral_size() -> uint32_t;

        pub fn bcm_host_get_sdram_address() -> uint32_t;

        pub fn bcm_host_init();

        pub fn graphics_get_display_size(display_number: uint16_t,
                                         width: *mut uint32_t,
                                         height: *mut uint32_t) -> int32_t;
    }
}

// -------------------------------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    #[test]
    pub fn get_peripheral_address_pass() {
        super::init();
        println!("Peripheral address: {}", super::get_peripheral_address());
        super::deinit();
    }

    #[test]
    pub fn get_peripheral_size_pass() {
        super::init();
        println!("Peripheral size: {}", super::get_peripheral_size());
        super::deinit();
    }

    #[test]
    pub fn get_sdram_address_pass() {
        super::init();
        println!("SDRAM address: {}", super::get_sdram_address());
        super::deinit();
    }

    #[test]
    pub fn graphics_test_pass() {
        super::init();
        let x = super::graphics_get_display_size(0);
        println!("Display size = {}x{}", x.width, x.height);
        super::deinit();
    }
}
