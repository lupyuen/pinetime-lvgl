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
pub type lv_style_attr_t = u8;
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
pub type lv_style_property_t = u16;
pub type lv_style_state_t = u16;
#[repr(C)]
pub struct lv_style_t {
    pub map: *mut u8,
}
impl Default for lv_style_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type lv_style_int_t = i16;
#[repr(C)]
pub struct lv_style_list_t {
    pub style_list: *mut *mut lv_style_t,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u8>,
    pub __bindgen_padding_0: u32,
}
impl Default for lv_style_list_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl lv_style_list_t {
    #[inline]
    pub fn style_cnt(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 6u8) as u32) }
    }
    #[inline]
    pub fn set_style_cnt(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub fn has_local(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_has_local(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn has_trans(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_has_trans(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn skip_trans(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_skip_trans(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ignore_trans(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ignore_trans(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn valid_cache(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_valid_cache(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ignore_cache(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ignore_cache(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn radius_zero(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_radius_zero(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn opa_scale_cover(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_opa_scale_cover(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn clip_corner_off(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_clip_corner_off(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn transform_all_zero(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_transform_all_zero(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn pad_all_zero(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_pad_all_zero(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn blend_mode_all_normal(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_blend_mode_all_normal(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(17usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bg_opa_transp(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(18usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bg_opa_transp(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(18usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bg_opa_cover(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(19usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bg_opa_cover(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(19usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bg_grad_dir_none(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(20usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bg_grad_dir_none(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(20usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn border_width_zero(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(21usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_border_width_zero(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(21usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn border_side_full(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(22usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_border_side_full(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(22usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn border_post_off(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(23usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_border_post_off(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(23usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn outline_width_zero(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(24usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_outline_width_zero(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(24usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn pattern_img_null(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(25usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_pattern_img_null(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(25usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn shadow_width_zero(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(26usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_shadow_width_zero(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(26usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn value_txt_str(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(27usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_value_txt_str(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(27usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn img_recolor_opa_transp(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(28usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_img_recolor_opa_transp(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(28usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn text_space_zero(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(29usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_text_space_zero(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(29usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn text_decor_none(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_text_decor_none(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn text_font_normal(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_text_font_normal(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        style_cnt: u32,
        has_local: u32,
        has_trans: u32,
        skip_trans: u32,
        ignore_trans: u32,
        valid_cache: u32,
        ignore_cache: u32,
        radius_zero: u32,
        opa_scale_cover: u32,
        clip_corner_off: u32,
        transform_all_zero: u32,
        pad_all_zero: u32,
        blend_mode_all_normal: u32,
        bg_opa_transp: u32,
        bg_opa_cover: u32,
        bg_grad_dir_none: u32,
        border_width_zero: u32,
        border_side_full: u32,
        border_post_off: u32,
        outline_width_zero: u32,
        pattern_img_null: u32,
        shadow_width_zero: u32,
        value_txt_str: u32,
        img_recolor_opa_transp: u32,
        text_space_zero: u32,
        text_decor_none: u32,
        text_font_normal: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 6u8, {
            let style_cnt: u32 = unsafe { ::core::mem::transmute(style_cnt) };
            style_cnt as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let has_local: u32 = unsafe { ::core::mem::transmute(has_local) };
            has_local as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let has_trans: u32 = unsafe { ::core::mem::transmute(has_trans) };
            has_trans as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let skip_trans: u32 = unsafe { ::core::mem::transmute(skip_trans) };
            skip_trans as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let ignore_trans: u32 = unsafe { ::core::mem::transmute(ignore_trans) };
            ignore_trans as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let valid_cache: u32 = unsafe { ::core::mem::transmute(valid_cache) };
            valid_cache as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let ignore_cache: u32 = unsafe { ::core::mem::transmute(ignore_cache) };
            ignore_cache as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let radius_zero: u32 = unsafe { ::core::mem::transmute(radius_zero) };
            radius_zero as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let opa_scale_cover: u32 = unsafe { ::core::mem::transmute(opa_scale_cover) };
            opa_scale_cover as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let clip_corner_off: u32 = unsafe { ::core::mem::transmute(clip_corner_off) };
            clip_corner_off as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let transform_all_zero: u32 = unsafe { ::core::mem::transmute(transform_all_zero) };
            transform_all_zero as u64
        });
        __bindgen_bitfield_unit.set(16usize, 1u8, {
            let pad_all_zero: u32 = unsafe { ::core::mem::transmute(pad_all_zero) };
            pad_all_zero as u64
        });
        __bindgen_bitfield_unit.set(17usize, 1u8, {
            let blend_mode_all_normal: u32 =
                unsafe { ::core::mem::transmute(blend_mode_all_normal) };
            blend_mode_all_normal as u64
        });
        __bindgen_bitfield_unit.set(18usize, 1u8, {
            let bg_opa_transp: u32 = unsafe { ::core::mem::transmute(bg_opa_transp) };
            bg_opa_transp as u64
        });
        __bindgen_bitfield_unit.set(19usize, 1u8, {
            let bg_opa_cover: u32 = unsafe { ::core::mem::transmute(bg_opa_cover) };
            bg_opa_cover as u64
        });
        __bindgen_bitfield_unit.set(20usize, 1u8, {
            let bg_grad_dir_none: u32 = unsafe { ::core::mem::transmute(bg_grad_dir_none) };
            bg_grad_dir_none as u64
        });
        __bindgen_bitfield_unit.set(21usize, 1u8, {
            let border_width_zero: u32 = unsafe { ::core::mem::transmute(border_width_zero) };
            border_width_zero as u64
        });
        __bindgen_bitfield_unit.set(22usize, 1u8, {
            let border_side_full: u32 = unsafe { ::core::mem::transmute(border_side_full) };
            border_side_full as u64
        });
        __bindgen_bitfield_unit.set(23usize, 1u8, {
            let border_post_off: u32 = unsafe { ::core::mem::transmute(border_post_off) };
            border_post_off as u64
        });
        __bindgen_bitfield_unit.set(24usize, 1u8, {
            let outline_width_zero: u32 = unsafe { ::core::mem::transmute(outline_width_zero) };
            outline_width_zero as u64
        });
        __bindgen_bitfield_unit.set(25usize, 1u8, {
            let pattern_img_null: u32 = unsafe { ::core::mem::transmute(pattern_img_null) };
            pattern_img_null as u64
        });
        __bindgen_bitfield_unit.set(26usize, 1u8, {
            let shadow_width_zero: u32 = unsafe { ::core::mem::transmute(shadow_width_zero) };
            shadow_width_zero as u64
        });
        __bindgen_bitfield_unit.set(27usize, 1u8, {
            let value_txt_str: u32 = unsafe { ::core::mem::transmute(value_txt_str) };
            value_txt_str as u64
        });
        __bindgen_bitfield_unit.set(28usize, 1u8, {
            let img_recolor_opa_transp: u32 =
                unsafe { ::core::mem::transmute(img_recolor_opa_transp) };
            img_recolor_opa_transp as u64
        });
        __bindgen_bitfield_unit.set(29usize, 1u8, {
            let text_space_zero: u32 = unsafe { ::core::mem::transmute(text_space_zero) };
            text_space_zero as u64
        });
        __bindgen_bitfield_unit.set(30usize, 1u8, {
            let text_decor_none: u32 = unsafe { ::core::mem::transmute(text_decor_none) };
            text_decor_none as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let text_font_normal: u32 = unsafe { ::core::mem::transmute(text_font_normal) };
            text_font_normal as u64
        });
        __bindgen_bitfield_unit
    }
}
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
