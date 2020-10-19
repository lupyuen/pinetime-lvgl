//! LVGL Widgets API for Rust

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

/// Contains Rust bindings for LVGL Arc Widget API `lv_arc`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod arc;

/// Contains Rust bindings for LVGL Bar Widget API `lv_bar`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod bar;

/// Contains Rust bindings for LVGL Button Widget API `lv_btn`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod btn;

/// Contains Rust bindings for LVGL Button Matrix Widget API `lv_btnmatrix`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod btnmatrix;

/// Contains Rust bindings for LVGL Calendar Widget API `lv_calendar`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod calendar;

/// Contains Rust bindings for LVGL Canvas Widget API `lv_canvas`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod canvas;

/// Contains Rust bindings for LVGL Chart Widget API `lv_chart`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod chart;

/// Contains Rust bindings for LVGL Checkbox Widget API `lv_checkbox`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod checkbox;

/// Contains Rust bindings for LVGL Cont Widget API `lv_cont`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod cont;

/// Contains Rust bindings for LVGL CPicker Widget API `lv_cpicker`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod cpicker;

/// Contains Rust bindings for LVGL Dropdown Widget API `lv_dropdown`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod dropdown;

/// Contains Rust bindings for LVGL Gauge Widget API `lv_gauge`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod gauge;

/// Contains Rust bindings for LVGL Image Widget API `lv_img`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod img;

/// Contains Rust bindings for LVGL Image Button Widget API `lv_imgbtn`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod imgbtn;

/// Contains Rust bindings for LVGL Keyboard Widget API `lv_keyboard`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod keyboard;

/// Contains Rust bindings for LVGL Label Widget API `lv_label`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod label;

/// Contains Rust bindings for LVGL LED Widget API `lv_led`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod led;

/// Contains Rust bindings for LVGL Line Widget API `lv_line`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod line;

/// Contains Rust bindings for LVGL Line Meter Widget API `lv_linemeter`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod linemeter;

/// Contains Rust bindings for LVGL List Widget API `lv_list`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod list;

/// Contains Rust bindings for LVGL Message Box Widget API `lv_msgbox`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod msgbox;

/// Contains Rust bindings for LVGL Object Mask Widget API `lv_objmask`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod objmask;

/// Contains Rust bindings for LVGL Template Widget API `lv_objx_templ`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod objx_templ;

/// Contains Rust bindings for LVGL Page Widget API `lv_page`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod page;

/// Contains Rust bindings for LVGL Roller Widget API `lv_roller`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod roller;

/// Contains Rust bindings for LVGL Slider Widget API `lv_slider`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod slider;

/// Contains Rust bindings for LVGL Spinbox Widget API `lv_spinbox`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod spinbox;

/// Contains Rust bindings for LVGL Spinner Widget API `lv_spinner`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod spinner;

/// Contains Rust bindings for LVGL Switch Widget API `lv_switch`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod switch;

/// Contains Rust bindings for LVGL Table Widget API `lv_table`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod table;

/// Contains Rust bindings for LVGL Tab View Widget API `lv_tabview`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod tabview;

/// Contains Rust bindings for LVGL Text Area Widget API `lv_textarea`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod textarea;

/// Contains Rust bindings for LVGL Tile View Widget API `lv_tileview`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod tileview;

/// Contains Rust bindings for LVGL Window Widget API `lv_win`
#[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
#[allow(non_upper_case_globals)]  //  Allow globals to have lowercase letters
#[allow(unused_imports)]          //  Allow unused imports
pub mod win;
