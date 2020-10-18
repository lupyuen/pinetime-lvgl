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
#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::core::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::core::mem::transmute(self)
    }
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
pub const LV_TABLE_COL_MAX: u32 = 12;
pub const LV_TABLE_CELL_STYLE_CNT: u32 = 4;
pub type lv_coord_t = i16;
pub type lv_obj_user_data_t = *mut ::cty::c_void;
pub type lv_res_t = u8;
#[doc = " Represents an area of the screen."]
#[repr(C)]
#[derive(Default)]
pub struct lv_area_t {
    pub x1: lv_coord_t,
    pub y1: lv_coord_t,
    pub x2: lv_coord_t,
    pub y2: lv_coord_t,
}
pub type lv_align_t = u8;
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
#[doc = " Dummy type to make handling easier"]
pub type lv_ll_node_t = u8;
#[doc = " Description of a linked list"]
#[repr(C)]
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
#[doc = " Internal table cell format structure."]
#[doc = ""]
#[doc = " Use the `lv_table` APIs instead."]
#[repr(C)]
pub struct lv_table_cell_format_t {
    pub s: __BindgenUnionField<lv_table_cell_format_t__bindgen_ty_1>,
    pub format_byte: __BindgenUnionField<u8>,
    pub bindgen_union_field: u8,
}
#[repr(C, packed)]
#[derive(Default)]
pub struct lv_table_cell_format_t__bindgen_ty_1 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
}
impl lv_table_cell_format_t__bindgen_ty_1 {
    #[inline]
    pub fn align(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_align(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn right_merge(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_right_merge(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn type_(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_type(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn crop(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_crop(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        align: u8,
        right_merge: u8,
        type_: u8,
        crop: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let align: u8 = unsafe { ::core::mem::transmute(align) };
            align as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let right_merge: u8 = unsafe { ::core::mem::transmute(right_merge) };
            right_merge as u64
        });
        __bindgen_bitfield_unit.set(3usize, 2u8, {
            let type_: u8 = unsafe { ::core::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let crop: u8 = unsafe { ::core::mem::transmute(crop) };
            crop as u64
        });
        __bindgen_bitfield_unit
    }
}
impl Default for lv_table_cell_format_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct lv_table_ext_t {
    pub col_cnt: u16,
    pub row_cnt: u16,
    pub cell_data: *mut *mut ::cty::c_char,
    pub row_h: *mut lv_coord_t,
    pub cell_style: [lv_style_list_t; 4usize],
    pub col_w: [lv_coord_t; 12usize],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub __bindgen_padding_0: [u8; 7usize],
}
impl Default for lv_table_ext_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl lv_table_ext_t {
    #[inline]
    pub fn cell_types(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_cell_types(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(cell_types: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let cell_types: u8 = unsafe { ::core::mem::transmute(cell_types) };
            cell_types as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const LV_TABLE_PART_BG: _bindgen_ty_35 = 0;
pub const LV_TABLE_PART_CELL1: _bindgen_ty_35 = 1;
pub const LV_TABLE_PART_CELL2: _bindgen_ty_35 = 2;
pub const LV_TABLE_PART_CELL3: _bindgen_ty_35 = 3;
pub const LV_TABLE_PART_CELL4: _bindgen_ty_35 = 4;
pub type _bindgen_ty_35 = u32;
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Create a table object"]
    #[doc = " - __`par`__: pointer to an object, it will be the parent of the new table"]
    #[doc = " - __`copy`__: pointer to a table object, if not NULL then the new object will be copied from it"]
    #[doc = " Return: pointer to the created table"]
    pub fn lv_table_create(par: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the value of a cell."]
    #[doc = " - __`table`__: pointer to a Table object"]
    #[doc = " - __`row`__: id of the row [0 .. row_cnt -1]"]
    #[doc = " - __`col`__: id of the column [0 .. col_cnt -1]"]
    #[doc = " - __`txt`__: text to display in the cell. It will be copied and saved so this variable is not"]
    #[doc = " required after this function call."]
    pub fn lv_table_set_cell_value(
        table: *mut lv_obj_t,
        row: u16,
        col: u16,
        txt: *const ::cty::c_char,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the number of rows"]
    #[doc = " - __`table`__: table pointer to a Table object"]
    #[doc = " - __`row_cnt`__: number of rows"]
    pub fn lv_table_set_row_cnt(table: *mut lv_obj_t, row_cnt: u16);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the number of columns"]
    #[doc = " - __`table`__: table pointer to a Table object"]
    #[doc = " - __`col_cnt`__: number of columns. Must be < LV_TABLE_COL_MAX"]
    pub fn lv_table_set_col_cnt(table: *mut lv_obj_t, col_cnt: u16);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the width of a column"]
    #[doc = " - __`table`__: table pointer to a Table object"]
    #[doc = " - __`col_id`__: id of the column [0 .. LV_TABLE_COL_MAX -1]"]
    #[doc = " - __`w`__: width of the column"]
    pub fn lv_table_set_col_width(table: *mut lv_obj_t, col_id: u16, w: lv_coord_t);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the text align in a cell"]
    #[doc = " - __`table`__: pointer to a Table object"]
    #[doc = " - __`row`__: id of the row [0 .. row_cnt -1]"]
    #[doc = " - __`col`__: id of the column [0 .. col_cnt -1]"]
    #[doc = " - __`align`__: LV_LABEL_ALIGN_LEFT or LV_LABEL_ALIGN_CENTER or LV_LABEL_ALIGN_RIGHT"]
    pub fn lv_table_set_cell_align(
        table: *mut lv_obj_t,
        row: u16,
        col: u16,
        align: lv_label_align_t,
    );
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the type of a cell."]
    #[doc = " - __`table`__: pointer to a Table object"]
    #[doc = " - __`row`__: id of the row [0 .. row_cnt -1]"]
    #[doc = " - __`col`__: id of the column [0 .. col_cnt -1]"]
    #[doc = " - __`type`__: 1,2,3 or 4. The cell style will be chosen accordingly."]
    pub fn lv_table_set_cell_type(table: *mut lv_obj_t, row: u16, col: u16, type_: u8);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Set the cell crop. (Don't adjust the height of the cell according to its content)"]
    #[doc = " - __`table`__: pointer to a Table object"]
    #[doc = " - __`row`__: id of the row [0 .. row_cnt -1]"]
    #[doc = " - __`col`__: id of the column [0 .. col_cnt -1]"]
    #[doc = " - __`crop`__: true: crop the cell content; false: set the cell height to the content."]
    pub fn lv_table_set_cell_crop(table: *mut lv_obj_t, row: u16, col: u16, crop: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Merge a cell with the right neighbor. The value of the cell to the right won't be displayed."]
    #[doc = " - __`table`__: table pointer to a Table object"]
    #[doc = " - __`row`__: id of the row [0 .. row_cnt -1]"]
    #[doc = " - __`col`__: id of the column [0 .. col_cnt -1]"]
    #[doc = " - __`en`__: true: merge right; false: don't merge right"]
    pub fn lv_table_set_cell_merge_right(table: *mut lv_obj_t, row: u16, col: u16, en: bool);
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the number of rows."]
    #[doc = " - __`table`__: table pointer to a Table object"]
    #[doc = " Return: number of rows."]
    pub fn lv_table_get_row_cnt(table: *mut lv_obj_t) -> u16;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the number of columns."]
    #[doc = " - __`table`__: table pointer to a Table object"]
    #[doc = " Return: number of columns."]
    pub fn lv_table_get_col_cnt(table: *mut lv_obj_t) -> u16;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the width of a column"]
    #[doc = " - __`table`__: table pointer to a Table object"]
    #[doc = " - __`col_id`__: id of the column [0 .. LV_TABLE_COL_MAX -1]"]
    #[doc = " Return: width of the column"]
    pub fn lv_table_get_col_width(table: *mut lv_obj_t, col_id: u16) -> lv_coord_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the text align of a cell"]
    #[doc = " - __`table`__: pointer to a Table object"]
    #[doc = " - __`row`__: id of the row [0 .. row_cnt -1]"]
    #[doc = " - __`col`__: id of the column [0 .. col_cnt -1]"]
    #[doc = " Return: LV_LABEL_ALIGN_LEFT (default in case of error) or LV_LABEL_ALIGN_CENTER or"]
    #[doc = " LV_LABEL_ALIGN_RIGHT"]
    pub fn lv_table_get_cell_align(table: *mut lv_obj_t, row: u16, col: u16) -> lv_label_align_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the type of a cell"]
    #[doc = " - __`table`__: pointer to a Table object"]
    #[doc = " - __`row`__: id of the row [0 .. row_cnt -1]"]
    #[doc = " - __`col`__: id of the column [0 .. col_cnt -1]"]
    #[doc = " Return: 1,2,3 or 4"]
    pub fn lv_table_get_cell_type(table: *mut lv_obj_t, row: u16, col: u16) -> lv_label_align_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the crop property of a cell"]
    #[doc = " - __`table`__: pointer to a Table object"]
    #[doc = " - __`row`__: id of the row [0 .. row_cnt -1]"]
    #[doc = " - __`col`__: id of the column [0 .. col_cnt -1]"]
    #[doc = " Return: true: text crop enabled; false: disabled"]
    pub fn lv_table_get_cell_crop(table: *mut lv_obj_t, row: u16, col: u16) -> lv_label_align_t;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the cell merge attribute."]
    #[doc = " - __`table`__: table pointer to a Table object"]
    #[doc = " - __`row`__: id of the row [0 .. row_cnt -1]"]
    #[doc = " - __`col`__: id of the column [0 .. col_cnt -1]"]
    #[doc = " Return: true: merge right; false: don't merge right"]
    pub fn lv_table_get_cell_merge_right(table: *mut lv_obj_t, row: u16, col: u16) -> bool;
}
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Get the last pressed or being pressed cell"]
    #[doc = " - __`table`__: pointer to a table object"]
    #[doc = " - __`row`__: pointer to variable to store the pressed row"]
    #[doc = " - __`col`__: pointer to variable to store the pressed column"]
    #[doc = " Return: LV_RES_OK: a valid pressed cell was found, LV_RES_INV: no valid cell is pressed"]
    pub fn lv_table_get_pressed_cell(
        table: *mut lv_obj_t,
        row: *mut u16,
        col: *mut u16,
    ) -> lv_res_t;
}
