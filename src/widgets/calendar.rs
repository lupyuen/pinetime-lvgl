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
pub const LV_CALENDAR_WEEK_STARTS_MONDAY: u32 = 0;
pub type lv_coord_t = i16;
pub type lv_obj_user_data_t = *mut ::cty::c_void;
pub type lv_res_t = u8;
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
#[doc = " Represents a date on the calendar object (platform-agnostic)."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct lv_calendar_date_t {
    pub year: u16,
    pub month: i8,
    pub day: i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv_calendar_ext_t {
    pub today: lv_calendar_date_t,
    pub showed_date: lv_calendar_date_t,
    pub highlighted_dates: *mut lv_calendar_date_t,
    pub btn_pressing: i8,
    pub highlighted_dates_num: u16,
    pub pressed_date: lv_calendar_date_t,
    pub day_names: *mut *const ::cty::c_char,
    pub month_names: *mut *const ::cty::c_char,
    pub style_header: lv_style_list_t,
    pub style_day_names: lv_style_list_t,
    pub style_date_nums: lv_style_list_t,
}
impl Default for lv_calendar_ext_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "< Background and \"normal\" date numbers style"]
pub const LV_CALENDAR_PART_BG: _bindgen_ty_32 = 0;
pub const LV_CALENDAR_PART_HEADER: _bindgen_ty_32 = 1;
#[doc = " Calendar header style"]
pub const LV_CALENDAR_PART_DAY_NAMES: _bindgen_ty_32 = 2;
#[doc = " Day name style"]
pub const LV_CALENDAR_PART_DATE: _bindgen_ty_32 = 3;
#[doc = " Calendar parts"]
pub type _bindgen_ty_32 = u32;
pub type lv_calendar_part_t = u8;
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Create a calendar objects"]
    #[doc = " - __`par`__: pointer to an object, it will be the parent of the new calendar"]
    #[doc = " - __`copy`__: pointer to a calendar object, if not NULL then the new object will be copied from it"]
    #[doc = " Return: pointer to the created calendar"]
    pub fn lv_calendar_create(par: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the today's date"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " - __`today`__: pointer to an `lv_calendar_date_t` variable containing the date of today. The value"]
    #[doc = " will be saved it can be local variable too."]
    pub fn lv_calendar_set_today_date(calendar: *mut lv_obj_t, today: *mut lv_calendar_date_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the currently showed"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " - __`showed`__: pointer to an `lv_calendar_date_t` variable containing the date to show. The value"]
    #[doc = " will be saved it can be local variable too."]
    pub fn lv_calendar_set_showed_date(calendar: *mut lv_obj_t, showed: *mut lv_calendar_date_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the the highlighted dates"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " - __`highlighted`__: pointer to an `lv_calendar_date_t` array containing the dates. ONLY A POINTER"]
    #[doc = " WILL BE SAVED! CAN'T BE LOCAL ARRAY."]
    #[doc = " - __`date_num`__: number of dates in the array"]
    pub fn lv_calendar_set_highlighted_dates(
        calendar: *mut lv_obj_t,
        highlighted: *mut lv_calendar_date_t,
        date_num: u16,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the name of the days"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " - __`day_names`__: pointer to an array with the names. E.g. `const char * days[7] = {\"Sun\", \"Mon\","]
    #[doc = " ...}` Only the pointer will be saved so this variable can't be local which will be destroyed"]
    #[doc = " later."]
    pub fn lv_calendar_set_day_names(calendar: *mut lv_obj_t, day_names: *mut *const ::cty::c_char);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the name of the month"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " - __`month_names`__: pointer to an array with the names. E.g. `const char * days[12] = {\"Jan\", \"Feb\","]
    #[doc = " ...}` Only the pointer will be saved so this variable can't be local which will be destroyed"]
    #[doc = " later."]
    pub fn lv_calendar_set_month_names(
        calendar: *mut lv_obj_t,
        month_names: *mut *const ::cty::c_char,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the today's date"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " Return: return pointer to an `lv_calendar_date_t` variable containing the date of today."]
    pub fn lv_calendar_get_today_date(calendar: *const lv_obj_t) -> *mut lv_calendar_date_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the currently showed"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " Return: pointer to an `lv_calendar_date_t` variable containing the date is being shown."]
    pub fn lv_calendar_get_showed_date(calendar: *const lv_obj_t) -> *mut lv_calendar_date_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the the pressed date."]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " Return: pointer to an `lv_calendar_date_t` variable containing the pressed date."]
    #[doc = "         `NULL` if not date pressed (e.g. the header)"]
    pub fn lv_calendar_get_pressed_date(calendar: *const lv_obj_t) -> *mut lv_calendar_date_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the the highlighted dates"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " Return: pointer to an `lv_calendar_date_t` array containing the dates."]
    pub fn lv_calendar_get_highlighted_dates(calendar: *const lv_obj_t) -> *mut lv_calendar_date_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the number of the highlighted dates"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " Return: number of highlighted days"]
    pub fn lv_calendar_get_highlighted_dates_num(calendar: *const lv_obj_t) -> u16;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the name of the days"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " Return: pointer to the array of day names"]
    pub fn lv_calendar_get_day_names(calendar: *const lv_obj_t) -> *mut *const ::cty::c_char;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the name of the month"]
    #[doc = " - __`calendar`__: pointer to a calendar object"]
    #[doc = " Return: pointer to the array of month names"]
    pub fn lv_calendar_get_month_names(calendar: *const lv_obj_t) -> *mut *const ::cty::c_char;
}
