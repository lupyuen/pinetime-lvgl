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
pub type lv_coord_t = i16;
pub type lv_obj_user_data_t = *mut ::cty::c_void;
pub type lv_res_t = u8;
#[doc = " Represents a point on the screen."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct lv_point_t {
    pub x: lv_coord_t,
    pub y: lv_coord_t,
}
#[doc = " Represents an area of the screen."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct lv_area_t {
    pub x1: lv_coord_t,
    pub y1: lv_coord_t,
    pub x2: lv_coord_t,
    pub y2: lv_coord_t,
}
pub type lv_align_t = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub union lv_color16_t {
    pub ch: lv_color16_t__bindgen_ty_1,
    pub full: u16,
    _bindgen_union_align: u16,
}
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
impl Default for lv_color16_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type lv_color_t = lv_color16_t;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct lv_color_hsv_t {
    pub h: u16,
    pub s: u8,
    pub v: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[doc = " Dummy type to make handling easier"]
pub type lv_ll_node_t = u8;
#[doc = " Description of a linked list"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv_ll_t {
    pub n_size: u32,
    pub head: *mut lv_ll_node_t,
    pub tail: *mut lv_ll_node_t,
}
impl Default for lv_ll_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type lv_drag_dir_t = u8;
pub type lv_bidi_dir_t = u8;
pub type lv_design_mode_t = u8;
pub type lv_design_res_t = u8;
#[doc = " The design callback is used to draw the object on the screen."]
#[doc = " It accepts the object, a mask area, and the mode in which to draw the object."]
pub type lv_design_cb_t = ::core::option::Option<
    unsafe extern "C" fn(
        obj: *mut _lv_obj_t,
        clip_area: *const lv_area_t,
        mode: lv_design_mode_t,
    ) -> lv_design_res_t,
>;
pub type lv_event_t = u8;
#[doc = " @brief Event callback."]
#[doc = " Events are used to notify the user of some action being taken on the object."]
#[doc = " For details, see ::lv_event_t."]
pub type lv_event_cb_t =
    ::core::option::Option<unsafe extern "C" fn(obj: *mut _lv_obj_t, event: lv_event_t)>;
pub type lv_signal_t = u8;
pub type lv_signal_cb_t = ::core::option::Option<
    unsafe extern "C" fn(
        obj: *mut _lv_obj_t,
        sign: lv_signal_t,
        param: *mut ::cty::c_void,
    ) -> lv_res_t,
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv_realign_t {
    pub base: *const _lv_obj_t,
    pub xofs: lv_coord_t,
    pub yofs: lv_coord_t,
    pub align: lv_align_t,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub __bindgen_padding_0: u16,
}
impl Default for lv_realign_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl lv_realign_t {
    #[inline]
    pub fn auto_realign(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_auto_realign(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn mid_align(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mid_align(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        auto_realign: u8,
        mid_align: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let auto_realign: u8 = unsafe { ::core::mem::transmute(auto_realign) };
            auto_realign as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let mid_align: u8 = unsafe { ::core::mem::transmute(mid_align) };
            mid_align as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type lv_state_t = u8;
pub type lv_obj_t = _lv_obj_t;
pub const LV_CPICKER_TYPE_RECT: _bindgen_ty_32 = 0;
pub const LV_CPICKER_TYPE_DISC: _bindgen_ty_32 = 1;
#[doc = "      TYPEDEFS"]
pub type _bindgen_ty_32 = u32;
pub type lv_cpicker_type_t = u8;
pub const LV_CPICKER_COLOR_MODE_HUE: _bindgen_ty_33 = 0;
pub const LV_CPICKER_COLOR_MODE_SATURATION: _bindgen_ty_33 = 1;
pub const LV_CPICKER_COLOR_MODE_VALUE: _bindgen_ty_33 = 2;
pub type _bindgen_ty_33 = u32;
pub type lv_cpicker_color_mode_t = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv_cpicker_ext_t {
    pub hsv: lv_color_hsv_t,
    pub knob: lv_cpicker_ext_t__bindgen_ty_1,
    pub last_click_time: u32,
    pub last_change_time: u32,
    pub last_press_point: lv_point_t,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv_cpicker_ext_t__bindgen_ty_1 {
    pub style_list: lv_style_list_t,
    pub pos: lv_point_t,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub __bindgen_padding_0: [u8; 3usize],
}
impl Default for lv_cpicker_ext_t__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl lv_cpicker_ext_t__bindgen_ty_1 {
    #[inline]
    pub fn colored(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_colored(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(colored: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let colored: u8 = unsafe { ::core::mem::transmute(colored) };
            colored as u64
        });
        __bindgen_bitfield_unit
    }
}
impl Default for lv_cpicker_ext_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl lv_cpicker_ext_t {
    #[inline]
    pub fn color_mode(&self) -> lv_cpicker_color_mode_t {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_color_mode(&mut self, val: lv_cpicker_color_mode_t) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn color_mode_fixed(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_color_mode_fixed(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn type_(&self) -> lv_cpicker_type_t {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_type(&mut self, val: lv_cpicker_type_t) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        color_mode: lv_cpicker_color_mode_t,
        color_mode_fixed: u8,
        type_: lv_cpicker_type_t,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let color_mode: u8 = unsafe { ::core::mem::transmute(color_mode) };
            color_mode as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let color_mode_fixed: u8 = unsafe { ::core::mem::transmute(color_mode_fixed) };
            color_mode_fixed as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let type_: u8 = unsafe { ::core::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const LV_CPICKER_PART_MAIN: _bindgen_ty_34 = 0;
pub const LV_CPICKER_PART_KNOB: _bindgen_ty_34 = 1;
pub const _LV_CPICKER_PART_VIRTUAL_LAST: _bindgen_ty_34 = 2;
pub const _LV_CPICKER_PART_REAL_LAST: _bindgen_ty_34 = 64;
pub type _bindgen_ty_34 = u32;
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Create a colorpicker objects"]
    #[doc = " - __`par`__: pointer to an object, it will be the parent of the new colorpicker"]
    #[doc = " - __`copy`__: pointer to a colorpicker object, if not NULL then the new object will be copied from it"]
    #[doc = " Return: pointer to the created colorpicker"]
    pub fn lv_cpicker_create(par: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set a new type for a colorpicker"]
    #[doc = " - __`cpicker`__: pointer to a colorpicker object"]
    #[doc = " - __`type`__: new type of the colorpicker (from 'lv_cpicker_type_t' enum)"]
    pub fn lv_cpicker_set_type(cpicker: *mut lv_obj_t, type_: lv_cpicker_type_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the current hue of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " - __`hue`__: current selected hue [0..360]"]
    #[doc = " Return: true if changed, otherwise false"]
    pub fn lv_cpicker_set_hue(cpicker: *mut lv_obj_t, hue: u16) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the current saturation of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " - __`saturation`__: current selected saturation [0..100]"]
    #[doc = " Return: true if changed, otherwise false"]
    pub fn lv_cpicker_set_saturation(cpicker: *mut lv_obj_t, saturation: u8) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the current value of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " - __`val`__: current selected value [0..100]"]
    #[doc = " Return: true if changed, otherwise false"]
    pub fn lv_cpicker_set_value(cpicker: *mut lv_obj_t, val: u8) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the current hsv of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " - __`hsv`__: current selected hsv"]
    #[doc = " Return: true if changed, otherwise false"]
    pub fn lv_cpicker_set_hsv(cpicker: *mut lv_obj_t, hsv: lv_color_hsv_t) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the current color of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " - __`color`__: current selected color"]
    #[doc = " Return: true if changed, otherwise false"]
    pub fn lv_cpicker_set_color(cpicker: *mut lv_obj_t, color: lv_color_t) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the current color mode."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " - __`mode`__: color mode (hue/sat/val)"]
    pub fn lv_cpicker_set_color_mode(cpicker: *mut lv_obj_t, mode: lv_cpicker_color_mode_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set if the color mode is changed on long press on center"]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " - __`fixed`__: color mode cannot be changed on long press"]
    pub fn lv_cpicker_set_color_mode_fixed(cpicker: *mut lv_obj_t, fixed: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Make the knob to be colored to the current color"]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " - __`en`__: true: color the knob; false: not color the knob"]
    pub fn lv_cpicker_set_knob_colored(cpicker: *mut lv_obj_t, en: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the current color mode."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " Return: color mode (hue/sat/val)"]
    pub fn lv_cpicker_get_color_mode(cpicker: *mut lv_obj_t) -> lv_cpicker_color_mode_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get if the color mode is changed on long press on center"]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " Return: mode cannot be changed on long press"]
    pub fn lv_cpicker_get_color_mode_fixed(cpicker: *mut lv_obj_t) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the current hue of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " Return: current selected hue"]
    pub fn lv_cpicker_get_hue(cpicker: *mut lv_obj_t) -> u16;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the current saturation of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " Return: current selected saturation"]
    pub fn lv_cpicker_get_saturation(cpicker: *mut lv_obj_t) -> u8;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the current hue of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " Return: current selected value"]
    pub fn lv_cpicker_get_value(cpicker: *mut lv_obj_t) -> u8;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the current selected hsv of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " Return: current selected hsv"]
    pub fn lv_cpicker_get_hsv(cpicker: *mut lv_obj_t) -> lv_color_hsv_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the current selected color of a colorpicker."]
    #[doc = " - __`cpicker`__: pointer to colorpicker object"]
    #[doc = " Return: current selected color"]
    pub fn lv_cpicker_get_color(cpicker: *mut lv_obj_t) -> lv_color_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Whether the knob is colored to the current color or not"]
    #[doc = " - __`cpicker`__: pointer to color picker object"]
    #[doc = " Return: true: color the knob; false: not color the knob"]
    pub fn lv_cpicker_get_knob_colored(cpicker: *mut lv_obj_t) -> bool;
}
