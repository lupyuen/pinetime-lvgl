//! LVGL Misc API for Rust

#[cfg(feature = "mynewt_os")]     //  If building for Mynewt OS...
use mynewt::{                     //  Use Mynewt types
    result::*,
    Out, Ptr, Strn,
};

#[cfg(feature = "riot_os")]       //  If building for RIOT OS...
use crate::{                      //  Use RIOT types
    result::*,
    Out, Ptr, Strn,
};

use crate::{
    core::{
        obj::{
            _lv_obj_t,
            lv_obj_t,
            lv_style_t,
        },
    },
};

/// Contains Rust bindings for LVGL Misc Animation API `lv_anim`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod anim;

/// Contains Rust bindings for LVGL Misc Area API `lv_area`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod area;

/* TODO: Rename async
/// Contains Rust bindings for LVGL Misc Async API `lv_async`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod async;
*/

/// Contains Rust bindings for LVGL Misc Bidi API `lv_bidi`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod bidi;

/// Contains Rust bindings for LVGL Misc Color API `lv_color`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod color;

/// Contains Rust bindings for LVGL Misc Debug API `lv_debug`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod debug;

/// Contains Rust bindings for LVGL Misc FS API `lv_fs`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod fs;

/// Contains Rust bindings for LVGL Misc GC API `lv_gc`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod gc;

/// Contains Rust bindings for LVGL Misc LL API `lv_ll`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod ll;

/// Contains Rust bindings for LVGL Misc Log API `lv_log`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod log;

/// Contains Rust bindings for LVGL Misc Math API `lv_math`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod math;

/// Contains Rust bindings for LVGL Misc Memory API `lv_mem`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod mem;

/// Contains Rust bindings for LVGL Misc Printf API `lv_printf`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod printf;

/// Contains Rust bindings for LVGL Misc Task API `lv_task`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod task;

/// Contains Rust bindings for LVGL Misc Template API `lv_templ`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod templ;

/// Contains Rust bindings for LVGL Misc Text API `lv_txt`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod txt;

/// Contains Rust bindings for LVGL Misc Text AP API `lv_txt_ap`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod txt_ap;

/// Contains Rust bindings for LVGL Misc Types API `lv_types`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod types;

/// Contains Rust bindings for LVGL Misc Utils API `lv_utils`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod utils;