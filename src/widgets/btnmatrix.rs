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
pub const LV_BTNMATRIX_WIDTH_MASK: u32 = 7;
pub const LV_BTNMATRIX_BTN_NONE: u32 = 65535;
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
pub type lv_label_align_t = u8;
#[doc = "< Button hidden"]
pub const LV_BTNMATRIX_CTRL_HIDDEN: _bindgen_ty_41 = 8;
#[doc = "< Do not repeat press this button."]
pub const LV_BTNMATRIX_CTRL_NO_REPEAT: _bindgen_ty_41 = 16;
#[doc = "< Disable this button."]
pub const LV_BTNMATRIX_CTRL_DISABLED: _bindgen_ty_41 = 32;
#[doc = "< Button *can* be toggled."]
pub const LV_BTNMATRIX_CTRL_CHECKABLE: _bindgen_ty_41 = 64;
#[doc = "< Button is currently toggled (e.g. checked)."]
pub const LV_BTNMATRIX_CTRL_CHECK_STATE: _bindgen_ty_41 = 128;
#[doc = "< 1: Send LV_EVENT_SELECTED on CLICK, 0: Send LV_EVENT_SELECTED on PRESS"]
pub const LV_BTNMATRIX_CTRL_CLICK_TRIG: _bindgen_ty_41 = 256;
#[doc = " Type to store button control bits (disabled, hidden etc.)"]
#[doc = " The first 3 bits are used to store the width"]
pub type _bindgen_ty_41 = u32;
pub type lv_btnmatrix_ctrl_t = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv_btnmatrix_ext_t {
    pub map_p: *mut *const ::cty::c_char,
    pub button_areas: *mut lv_area_t,
    pub ctrl_bits: *mut lv_btnmatrix_ctrl_t,
    pub style_btn: lv_style_list_t,
    pub btn_cnt: u16,
    pub btn_id_pr: u16,
    pub btn_id_focused: u16,
    pub btn_id_act: u16,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub __bindgen_padding_0: [u8; 7usize],
}
impl Default for lv_btnmatrix_ext_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl lv_btnmatrix_ext_t {
    #[inline]
    pub fn recolor(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_recolor(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn one_check(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_one_check(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn align(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_align(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        recolor: u8,
        one_check: u8,
        align: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let recolor: u8 = unsafe { ::core::mem::transmute(recolor) };
            recolor as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let one_check: u8 = unsafe { ::core::mem::transmute(one_check) };
            one_check as u64
        });
        __bindgen_bitfield_unit.set(2usize, 2u8, {
            let align: u8 = unsafe { ::core::mem::transmute(align) };
            align as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const LV_BTNMATRIX_PART_BG: _bindgen_ty_42 = 0;
pub const LV_BTNMATRIX_PART_BTN: _bindgen_ty_42 = 1;
pub type _bindgen_ty_42 = u32;
pub type lv_btnmatrix_part_t = u8;
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Create a button matrix objects"]
    #[doc = " - __`par`__: pointer to an object, it will be the parent of the new button matrix"]
    #[doc = " - __`copy`__: pointer to a button matrix object, if not NULL then the new object will be copied"]
    #[doc = " from it"]
    #[doc = " Return: pointer to the created button matrix"]
    pub fn lv_btnmatrix_create(par: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set a new map. Buttons will be created/deleted according to the map. The"]
    #[doc = " button matrix keeps a reference to the map and so the string array must not"]
    #[doc = " be deallocated during the life of the matrix."]
    #[doc = " - __`btnm`__: pointer to a button matrix object"]
    #[doc = " - __`map`__: pointer a string array. The last string has to be: \"\". Use \"\\n\" to make a line break."]
    pub fn lv_btnmatrix_set_map(btnm: *mut lv_obj_t, map: *mut *const ::cty::c_char);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the button control map (hidden, disabled etc.) for a button matrix. The"]
    #[doc = " control map array will be copied and so may be deallocated after this"]
    #[doc = " function returns."]
    #[doc = " - __`btnm`__: pointer to a button matrix object"]
    #[doc = " - __`ctrl_map`__: pointer to an array of `lv_btn_ctrl_t` control bytes. The"]
    #[doc = "                 length of the array and position of the elements must match"]
    #[doc = "                 the number and order of the individual buttons (i.e. excludes"]
    #[doc = "                 newline entries)."]
    #[doc = "                 An element of the map should look like e.g.:"]
    #[doc = "                 `ctrl_map[0] = width | LV_BTNMATRIX_CTRL_NO_REPEAT |  LV_BTNMATRIX_CTRL_TGL_ENABLE`"]
    pub fn lv_btnmatrix_set_ctrl_map(btnm: *mut lv_obj_t, ctrl_map: *const lv_btnmatrix_ctrl_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the focused button i.e. visually highlight it."]
    #[doc = " - __`btnm`__: pointer to button matrix object"]
    #[doc = " - __`id`__: index of the button to focus(`LV_BTNMATRIX_BTN_NONE` to remove focus)"]
    pub fn lv_btnmatrix_set_focused_btn(btnm: *mut lv_obj_t, id: u16);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Enable recoloring of button's texts"]
    #[doc = " - __`btnm`__: pointer to button matrix object"]
    #[doc = " - __`en`__: true: enable recoloring; false: disable"]
    pub fn lv_btnmatrix_set_recolor(btnm: *const lv_obj_t, en: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the attributes of a button of the button matrix"]
    #[doc = " - __`btnm`__: pointer to button matrix object"]
    #[doc = " - __`btn_id`__: 0 based index of the button to modify. (Not counting new lines)"]
    pub fn lv_btnmatrix_set_btn_ctrl(btnm: *const lv_obj_t, btn_id: u16, ctrl: lv_btnmatrix_ctrl_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Clear the attributes of a button of the button matrix"]
    #[doc = " - __`btnm`__: pointer to button matrix object"]
    #[doc = " - __`btn_id`__: 0 based index of the button to modify. (Not counting new lines)"]
    pub fn lv_btnmatrix_clear_btn_ctrl(
        btnm: *const lv_obj_t,
        btn_id: u16,
        ctrl: lv_btnmatrix_ctrl_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the attributes of all buttons of a button matrix"]
    #[doc = " - __`btnm`__: pointer to a button matrix object"]
    #[doc = " - __`ctrl`__: attribute(s) to set from `lv_btnmatrix_ctrl_t`. Values can be ORed."]
    pub fn lv_btnmatrix_set_btn_ctrl_all(btnm: *mut lv_obj_t, ctrl: lv_btnmatrix_ctrl_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Clear the attributes of all buttons of a button matrix"]
    #[doc = " - __`btnm`__: pointer to a button matrix object"]
    #[doc = " - __`ctrl`__: attribute(s) to set from `lv_btnmatrix_ctrl_t`. Values can be ORed."]
    #[doc = " - __`en`__: true: set the attributes; false: clear the attributes"]
    pub fn lv_btnmatrix_clear_btn_ctrl_all(btnm: *mut lv_obj_t, ctrl: lv_btnmatrix_ctrl_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set a single buttons relative width."]
    #[doc = " This method will cause the matrix be regenerated and is a relatively"]
    #[doc = " expensive operation. It is recommended that initial width be specified using"]
    #[doc = " `lv_btnmatrix_set_ctrl_map` and this method only be used for dynamic changes."]
    #[doc = " - __`btnm`__: pointer to button matrix object"]
    #[doc = " - __`btn_id`__: 0 based index of the button to modify."]
    #[doc = " - __`width`__: Relative width compared to the buttons in the same row. [1..7]"]
    pub fn lv_btnmatrix_set_btn_width(btnm: *mut lv_obj_t, btn_id: u16, width: u8);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Make the button matrix like a selector widget (only one button may be toggled at a time)."]
    #[doc = " `Checkable` must be enabled on the buttons you want to be selected with `lv_btnmatrix_set_ctrl` or"]
    #[doc = " `lv_btnmatrix_set_btn_ctrl_all`."]
    #[doc = " - __`btnm`__: Button matrix object"]
    #[doc = " - __`one_chk`__: Whether \"one check\" mode is enabled"]
    pub fn lv_btnmatrix_set_one_check(btnm: *mut lv_obj_t, one_chk: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the align of the map text (left, right or center)"]
    #[doc = " - __`btnm`__: pointer to a btnmatrix object"]
    #[doc = " - __`align`__: LV_LABEL_ALIGN_LEFT, LV_LABEL_ALIGN_RIGHT or LV_LABEL_ALIGN_CENTER"]
    pub fn lv_btnmatrix_set_align(btnm: *mut lv_obj_t, align: lv_label_align_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the current map of a button matrix"]
    #[doc = " - __`btnm`__: pointer to a button matrix object"]
    #[doc = " Return: the current map"]
    pub fn lv_btnmatrix_get_map_array(btnm: *const lv_obj_t) -> *mut *const ::cty::c_char;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Check whether the button's text can use recolor or not"]
    #[doc = " - __`btnm`__: pointer to button matrix object"]
    #[doc = " Return: true: text recolor enable; false: disabled"]
    pub fn lv_btnmatrix_get_recolor(btnm: *const lv_obj_t) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the index of the lastly \"activated\" button by the user (pressed, released etc)"]
    #[doc = " Useful in the the `event_cb` to get the text of the button, check if hidden etc."]
    #[doc = " - __`btnm`__: pointer to button matrix object"]
    #[doc = " Return:  index of the last released button (LV_BTNMATRIX_BTN_NONE: if unset)"]
    pub fn lv_btnmatrix_get_active_btn(btnm: *const lv_obj_t) -> u16;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the focused button's index."]
    #[doc = " - __`btnm`__: pointer to button matrix object"]
    #[doc = " Return:  index of the focused button (LV_BTNMATRIX_BTN_NONE: if unset)"]
    pub fn lv_btnmatrix_get_focused_btn(btnm: *const lv_obj_t) -> u16;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the whether a control value is enabled or disabled for button of a button matrix"]
    #[doc = " - __`btnm`__: pointer to a button matrix object"]
    #[doc = " - __`btn_id`__: the index a button not counting new line characters. (E.g. the return value of"]
    #[doc = " lv_btnmatrix_get_pressed/released)"]
    #[doc = " - __`ctrl`__: control values to check (ORed value can be used)"]
    #[doc = " Return: true: long press repeat is disabled; false: long press repeat enabled"]
    pub fn lv_btnmatrix_get_btn_ctrl(
        btnm: *mut lv_obj_t,
        btn_id: u16,
        ctrl: lv_btnmatrix_ctrl_t,
    ) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Find whether \"one toggle\" mode is enabled."]
    #[doc = " - __`btnm`__: Button matrix object"]
    #[doc = " Return: whether \"one toggle\" mode is enabled"]
    pub fn lv_btnmatrix_get_one_check(btnm: *const lv_obj_t) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the align attribute"]
    #[doc = " @param  btnm pointer to a btnmatrix object"]
    #[doc = " Return: LV_LABEL_ALIGN_LEFT, LV_LABEL_ALIGN_RIGHT or LV_LABEL_ALIGN_CENTER"]
    pub fn lv_btnmatrix_get_align(btnm: *const lv_obj_t) -> lv_label_align_t;
}
