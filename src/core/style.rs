/* automatically generated by rust-bindgen */

use
super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const LV_STYLE_ID_MASK: u32 = 255;
pub const LV_STYLE_ATTR_NONE: u32 = 0;
pub const LV_STYLE_ATTR_INHERIT: u32 = 128;
pub const LV_STYLE_TRANS_NUM_MAX: u32 = 6;
pub const LV_STYLE_PROP_ALL: u32 = 255;
pub const LV_STYLE_ID_VALUE: u32 = 0;
pub const LV_STYLE_ID_COLOR: u32 = 9;
pub const LV_STYLE_ID_OPA: u32 = 12;
pub const LV_STYLE_ID_PTR: u32 = 14;
pub const LV_STYLE_STATE_POS: u32 = 8;
pub const LV_STYLE_STATE_MASK: u32 = 32512;
pub const LV_STYLE_INHERIT_MASK: u32 = 32768;
#[repr(C)]
#[repr(align(2))]
#[derive(Default, Copy, Clone)]
pub struct lv_color16_t__bindgen_ty_1 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
}
impl lv_color16_t__bindgen_ty_1 {
    #[inline]
    pub fn green_h(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u16) }
    }
    #[inline]
    pub fn set_green_h(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn red(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 5u8) as u16) }
    }
    #[inline]
    pub fn set_red(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn blue(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 5u8) as u16) }
    }
    #[inline]
    pub fn set_blue(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn green_l(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(13usize, 3u8) as u16) }
    }
    #[inline]
    pub fn set_green_l(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(13usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        green_h: u16,
        red: u16,
        blue: u16,
        green_l: u16,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let green_h: u16 = unsafe { ::core::mem::transmute(green_h) };
            green_h as u64
        });
        __bindgen_bitfield_unit.set(3usize, 5u8, {
            let red: u16 = unsafe { ::core::mem::transmute(red) };
            red as u64
        });
        __bindgen_bitfield_unit.set(8usize, 5u8, {
            let blue: u16 = unsafe { ::core::mem::transmute(blue) };
            blue as u64
        });
        __bindgen_bitfield_unit.set(13usize, 3u8, {
            let green_l: u16 = unsafe { ::core::mem::transmute(green_l) };
            green_l as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const LV_STYLE_RADIUS: _bindgen_ty_14 = 1;
pub const LV_STYLE_CLIP_CORNER: _bindgen_ty_14 = 2;
pub const LV_STYLE_SIZE: _bindgen_ty_14 = 3;
pub const LV_STYLE_TRANSFORM_WIDTH: _bindgen_ty_14 = 4;
pub const LV_STYLE_TRANSFORM_HEIGHT: _bindgen_ty_14 = 5;
pub const LV_STYLE_TRANSFORM_ANGLE: _bindgen_ty_14 = 6;
pub const LV_STYLE_TRANSFORM_ZOOM: _bindgen_ty_14 = 7;
pub const LV_STYLE_OPA_SCALE: _bindgen_ty_14 = 32780;
pub const LV_STYLE_PAD_TOP: _bindgen_ty_14 = 16;
pub const LV_STYLE_PAD_BOTTOM: _bindgen_ty_14 = 17;
pub const LV_STYLE_PAD_LEFT: _bindgen_ty_14 = 18;
pub const LV_STYLE_PAD_RIGHT: _bindgen_ty_14 = 19;
pub const LV_STYLE_PAD_INNER: _bindgen_ty_14 = 20;
pub const LV_STYLE_MARGIN_TOP: _bindgen_ty_14 = 21;
pub const LV_STYLE_MARGIN_BOTTOM: _bindgen_ty_14 = 22;
pub const LV_STYLE_MARGIN_LEFT: _bindgen_ty_14 = 23;
pub const LV_STYLE_MARGIN_RIGHT: _bindgen_ty_14 = 24;
pub const LV_STYLE_BG_BLEND_MODE: _bindgen_ty_14 = 32;
pub const LV_STYLE_BG_MAIN_STOP: _bindgen_ty_14 = 33;
pub const LV_STYLE_BG_GRAD_STOP: _bindgen_ty_14 = 34;
pub const LV_STYLE_BG_GRAD_DIR: _bindgen_ty_14 = 35;
pub const LV_STYLE_BG_COLOR: _bindgen_ty_14 = 41;
pub const LV_STYLE_BG_GRAD_COLOR: _bindgen_ty_14 = 42;
pub const LV_STYLE_BG_OPA: _bindgen_ty_14 = 44;
pub const LV_STYLE_BORDER_WIDTH: _bindgen_ty_14 = 48;
pub const LV_STYLE_BORDER_SIDE: _bindgen_ty_14 = 49;
pub const LV_STYLE_BORDER_BLEND_MODE: _bindgen_ty_14 = 50;
pub const LV_STYLE_BORDER_POST: _bindgen_ty_14 = 51;
pub const LV_STYLE_BORDER_COLOR: _bindgen_ty_14 = 57;
pub const LV_STYLE_BORDER_OPA: _bindgen_ty_14 = 60;
pub const LV_STYLE_OUTLINE_WIDTH: _bindgen_ty_14 = 64;
pub const LV_STYLE_OUTLINE_PAD: _bindgen_ty_14 = 65;
pub const LV_STYLE_OUTLINE_BLEND_MODE: _bindgen_ty_14 = 66;
pub const LV_STYLE_OUTLINE_COLOR: _bindgen_ty_14 = 73;
pub const LV_STYLE_OUTLINE_OPA: _bindgen_ty_14 = 76;
pub const LV_STYLE_SHADOW_WIDTH: _bindgen_ty_14 = 80;
pub const LV_STYLE_SHADOW_OFS_X: _bindgen_ty_14 = 81;
pub const LV_STYLE_SHADOW_OFS_Y: _bindgen_ty_14 = 82;
pub const LV_STYLE_SHADOW_SPREAD: _bindgen_ty_14 = 83;
pub const LV_STYLE_SHADOW_BLEND_MODE: _bindgen_ty_14 = 84;
pub const LV_STYLE_SHADOW_COLOR: _bindgen_ty_14 = 89;
pub const LV_STYLE_SHADOW_OPA: _bindgen_ty_14 = 92;
pub const LV_STYLE_PATTERN_BLEND_MODE: _bindgen_ty_14 = 96;
pub const LV_STYLE_PATTERN_REPEAT: _bindgen_ty_14 = 97;
pub const LV_STYLE_PATTERN_RECOLOR: _bindgen_ty_14 = 105;
pub const LV_STYLE_PATTERN_OPA: _bindgen_ty_14 = 108;
pub const LV_STYLE_PATTERN_RECOLOR_OPA: _bindgen_ty_14 = 109;
pub const LV_STYLE_PATTERN_IMAGE: _bindgen_ty_14 = 110;
pub const LV_STYLE_VALUE_LETTER_SPACE: _bindgen_ty_14 = 112;
pub const LV_STYLE_VALUE_LINE_SPACE: _bindgen_ty_14 = 113;
pub const LV_STYLE_VALUE_BLEND_MODE: _bindgen_ty_14 = 114;
pub const LV_STYLE_VALUE_OFS_X: _bindgen_ty_14 = 115;
pub const LV_STYLE_VALUE_OFS_Y: _bindgen_ty_14 = 116;
pub const LV_STYLE_VALUE_ALIGN: _bindgen_ty_14 = 117;
pub const LV_STYLE_VALUE_COLOR: _bindgen_ty_14 = 121;
pub const LV_STYLE_VALUE_OPA: _bindgen_ty_14 = 124;
pub const LV_STYLE_VALUE_FONT: _bindgen_ty_14 = 126;
pub const LV_STYLE_VALUE_STR: _bindgen_ty_14 = 127;
pub const LV_STYLE_TEXT_LETTER_SPACE: _bindgen_ty_14 = 32896;
pub const LV_STYLE_TEXT_LINE_SPACE: _bindgen_ty_14 = 32897;
pub const LV_STYLE_TEXT_DECOR: _bindgen_ty_14 = 32898;
pub const LV_STYLE_TEXT_BLEND_MODE: _bindgen_ty_14 = 32899;
pub const LV_STYLE_TEXT_COLOR: _bindgen_ty_14 = 32905;
pub const LV_STYLE_TEXT_SEL_COLOR: _bindgen_ty_14 = 32906;
pub const LV_STYLE_TEXT_OPA: _bindgen_ty_14 = 32908;
pub const LV_STYLE_TEXT_FONT: _bindgen_ty_14 = 32910;
pub const LV_STYLE_LINE_WIDTH: _bindgen_ty_14 = 144;
pub const LV_STYLE_LINE_BLEND_MODE: _bindgen_ty_14 = 145;
pub const LV_STYLE_LINE_DASH_WIDTH: _bindgen_ty_14 = 146;
pub const LV_STYLE_LINE_DASH_GAP: _bindgen_ty_14 = 147;
pub const LV_STYLE_LINE_ROUNDED: _bindgen_ty_14 = 148;
pub const LV_STYLE_LINE_COLOR: _bindgen_ty_14 = 153;
pub const LV_STYLE_LINE_OPA: _bindgen_ty_14 = 156;
pub const LV_STYLE_IMAGE_BLEND_MODE: _bindgen_ty_14 = 32928;
pub const LV_STYLE_IMAGE_RECOLOR: _bindgen_ty_14 = 32937;
pub const LV_STYLE_IMAGE_OPA: _bindgen_ty_14 = 32940;
pub const LV_STYLE_IMAGE_RECOLOR_OPA: _bindgen_ty_14 = 32941;
pub const LV_STYLE_TRANSITION_TIME: _bindgen_ty_14 = 176;
pub const LV_STYLE_TRANSITION_DELAY: _bindgen_ty_14 = 177;
pub const LV_STYLE_TRANSITION_PROP_1: _bindgen_ty_14 = 178;
pub const LV_STYLE_TRANSITION_PROP_2: _bindgen_ty_14 = 179;
pub const LV_STYLE_TRANSITION_PROP_3: _bindgen_ty_14 = 180;
pub const LV_STYLE_TRANSITION_PROP_4: _bindgen_ty_14 = 181;
pub const LV_STYLE_TRANSITION_PROP_5: _bindgen_ty_14 = 182;
pub const LV_STYLE_TRANSITION_PROP_6: _bindgen_ty_14 = 183;
pub const LV_STYLE_TRANSITION_PATH: _bindgen_ty_14 = 190;
pub const LV_STYLE_SCALE_WIDTH: _bindgen_ty_14 = 192;
pub const LV_STYLE_SCALE_BORDER_WIDTH: _bindgen_ty_14 = 193;
pub const LV_STYLE_SCALE_END_BORDER_WIDTH: _bindgen_ty_14 = 194;
pub const LV_STYLE_SCALE_END_LINE_WIDTH: _bindgen_ty_14 = 195;
pub const LV_STYLE_SCALE_GRAD_COLOR: _bindgen_ty_14 = 201;
pub const LV_STYLE_SCALE_END_COLOR: _bindgen_ty_14 = 202;
pub type _bindgen_ty_14 = u32;
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Initialize a style"]
    #[doc = " - __`style`__: pointer to a style to initialize"]
    pub fn lv_style_init(style: *mut lv_style_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Copy a style with all its properties"]
    #[doc = " - __`style_dest`__: pointer to the destination style. (Should be initialized with `lv_style_init()`)"]
    #[doc = " - __`style_src`__: pointer to the source (to copy )style"]
    pub fn lv_style_copy(style_dest: *mut lv_style_t, style_src: *const lv_style_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Initialize a style list"]
    #[doc = " - __`list`__: a style list to initialize"]
    pub fn lv_style_list_init(list: *mut lv_style_list_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Copy a style list with all its styles and local style properties"]
    #[doc = " - __`list_dest`__: pointer to the destination style list. (should be initialized with `lv_style_list_init()`)"]
    #[doc = " - __`list_src`__: pointer to the source (to copy) style list."]
    pub fn lv_style_list_copy(list_dest: *mut lv_style_list_t, list_src: *const lv_style_list_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_list_get_style(list: *mut lv_style_list_t, id: u8) -> *mut lv_style_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Clear all properties from a style and all allocated memories."]
    #[doc = " - __`style`__: pointer to a style"]
    pub fn lv_style_reset(style: *mut lv_style_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Remove a property from a style"]
    #[doc = " - __`style`__: pointer to a style"]
    #[doc = " - __`prop`__:  a style property ORed with a state."]
    #[doc = " E.g. `LV_STYLE_BORDER_WIDTH | (LV_STATE_PRESSED << LV_STYLE_STATE_POS)`"]
    #[doc = " Return: true: the property was found and removed; false: the property wasn't found"]
    pub fn lv_style_remove_prop(style: *mut lv_style_t, prop: lv_style_property_t) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the local style of a style list"]
    #[doc = " - __`list`__: pointer to a style list where the local property should be set"]
    #[doc = " Return: pointer to the local style if exists else `NULL`."]
    pub fn lv_style_list_get_local_style(list: *mut lv_style_list_t) -> *mut lv_style_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_radius(style: *mut lv_style_t, state: lv_state_t, value: lv_style_int_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_clip_corner(style: *mut lv_style_t, state: lv_state_t, value: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_size(style: *mut lv_style_t, state: lv_state_t, value: lv_style_int_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transform_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transform_height(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transform_angle(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transform_zoom(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_opa_scale(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pad_top(style: *mut lv_style_t, state: lv_state_t, value: lv_style_int_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pad_bottom(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pad_left(style: *mut lv_style_t, state: lv_state_t, value: lv_style_int_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pad_right(style: *mut lv_style_t, state: lv_state_t, value: lv_style_int_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pad_inner(style: *mut lv_style_t, state: lv_state_t, value: lv_style_int_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_margin_top(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_margin_bottom(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_margin_left(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_margin_right(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_bg_blend_mode(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_blend_mode_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_bg_main_stop(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_bg_grad_stop(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_bg_grad_dir(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_grad_dir_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_bg_color(style: *mut lv_style_t, state: lv_state_t, value: lv_color_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_bg_grad_color(style: *mut lv_style_t, state: lv_state_t, value: lv_color_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_bg_opa(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_border_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_border_side(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_border_side_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_border_blend_mode(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_blend_mode_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_border_post(style: *mut lv_style_t, state: lv_state_t, value: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_border_color(style: *mut lv_style_t, state: lv_state_t, value: lv_color_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_border_opa(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_outline_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_outline_pad(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_outline_blend_mode(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_blend_mode_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_outline_color(style: *mut lv_style_t, state: lv_state_t, value: lv_color_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_outline_opa(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_shadow_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_shadow_ofs_x(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_shadow_ofs_y(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_shadow_spread(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_shadow_blend_mode(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_blend_mode_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_shadow_color(style: *mut lv_style_t, state: lv_state_t, value: lv_color_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_shadow_opa(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pattern_repeat(style: *mut lv_style_t, state: lv_state_t, value: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pattern_blend_mode(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_blend_mode_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pattern_recolor(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_color_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pattern_opa(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pattern_recolor_opa(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_opa_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pattern_image(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: *const ::cty::c_void,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_letter_space(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_line_space(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_blend_mode(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_blend_mode_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_ofs_x(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_ofs_y(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_align(style: *mut lv_style_t, state: lv_state_t, value: lv_align_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_color(style: *mut lv_style_t, state: lv_state_t, value: lv_color_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_opa(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_font(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: *const lv_font_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_value_str(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: *const ::cty::c_char,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_text_letter_space(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_text_line_space(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_text_decor(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_text_decor_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_text_blend_mode(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_blend_mode_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_text_color(style: *mut lv_style_t, state: lv_state_t, value: lv_color_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_text_sel_color(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_color_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_text_opa(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_text_font(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: *const lv_font_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_line_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_line_blend_mode(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_blend_mode_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_line_dash_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_line_dash_gap(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_line_rounded(style: *mut lv_style_t, state: lv_state_t, value: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_line_color(style: *mut lv_style_t, state: lv_state_t, value: lv_color_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_line_opa(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_image_blend_mode(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_blend_mode_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_image_recolor(style: *mut lv_style_t, state: lv_state_t, value: lv_color_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_image_opa(style: *mut lv_style_t, state: lv_state_t, value: lv_opa_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_image_recolor_opa(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_opa_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transition_time(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transition_delay(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transition_prop_1(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transition_prop_2(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transition_prop_3(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transition_prop_4(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transition_prop_5(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transition_prop_6(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_transition_path(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: *mut lv_anim_path_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_scale_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_scale_border_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_scale_end_border_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_scale_end_line_width(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_scale_grad_color(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_color_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_scale_end_color(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_color_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pad_all(style: *mut lv_style_t, state: lv_state_t, value: lv_style_int_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pad_hor(style: *mut lv_style_t, state: lv_state_t, value: lv_style_int_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_pad_ver(style: *mut lv_style_t, state: lv_state_t, value: lv_style_int_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_margin_all(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_margin_hor(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    pub fn lv_style_set_margin_ver(
        style: *mut lv_style_t,
        state: lv_state_t,
        value: lv_style_int_t,
    );
}
