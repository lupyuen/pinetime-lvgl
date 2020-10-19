#!/usr/bin/env bash
#  Generate Rust bindings for LVGL C API. Install "bindgen" before running:
#  cargo install bindgen
#  Also install rustfmt when prompted
#  Ignore any "unused option" errors
#    unused option: --whitelist-function (?i)...
#    unused option: --whitelist-var (?i)...
#    unused option: --whitelist-type (?i)...
#  TODO: Remove derive[Debug]

set -e  #  Exit when any command fails.
set -x  #  Echo all commands.
export RUST_BACKTRACE=1  #  Show Rust errors.

#  Define the library name
libname=lvgl
headerprefix=pinetime_lvgl_mynewt

#  TODO: Sync gcc options with https://github.com/AppKaki/lvgl-wasm/blob/mynewt/mynewt/Makefile
CCFLAGS=" -g -I $headerprefix/src/lv_core -D LV_USE_DEMO_WIDGETS "

#  gcc options for bindgen only. We disable "static" and "inline" so that wrappers will be generated for static inline functions like "lv_style_set_text_font"
CCFLAGS_BINDGEN=" -D static= -D inline= "

function generate_bindings() {
    #  Generate bindings for the module.
    local libname=$1     # Library name e.g. lvgl
    local modname=$2     # Module name e.g. core
    local submodname=$3  # Submodule name e.g. obj
    local headerfile=$4  # Header file e.g. pinetime_lvgl_mynewt/src/lv_core/lv_obj.h
    shift 4
    local whitelist="$@" # Whitelist Options: --raw-line, --blacklist-item, --whitelist-function, --whitelist-type, --whitelist-var
    echo "whitelist=$whitelist"

    local expandpath=src/$modname/$submodname.rs
    local tmpexpandpath=src/$modname/$submodname.tmp

    #  Generate Rust bindings
    #  TODO: Ensure that output folder has been created
    bindgen \
        --verbose \
        --use-core \
        --ctypes-prefix "::cty" \
        --with-derive-default \
        --no-derive-copy \
        --no-derive-debug \
        --no-layout-tests \
        $whitelist \
        -o $tmpexpandpath \
        $headerfile \
        -- \
        -Ibaselibc/include/ \
        $CCFLAGS \
        $CCFLAGS_BINDGEN

    # Change extern "C"
    # to     #[lvgl_macros::safe_wrap(attr)] extern "C"
    # Change #[doc = " @param dev The device to open"]
    # to     #[doc = " - __`dev`__: The device to open"]
    # Change @return to Return
    # Change @code{.c} to ```c
    # Change @code{...} to ```
    # Change @endcode to ```
    # Change @note to __Note:__
    # Change pub const LV_ALIGN_CENTER: _bindgen_ty_3 = 0;
    # to     pub const LV_ALIGN_CENTER: lv_align_t = 0;
    # Change pub const LV_LABEL_ALIGN_CENTER: _bindgen_ty_33 = 1;
    # to     pub const LV_LABEL_ALIGN_CENTER: lv_label_align_t = 1;
    # Change pub const LV_LABEL_LONG_BREAK: _bindgen_ty_32 = 1;
    # to     pub const LV_LABEL_LONG_BREAK: lv_label_long_mode_t = 1;
    cat $tmpexpandpath \
        | sed 's/^extern "C" /#[lvgl_macros::safe_wrap(attr)] extern "C" /' \
        | sed 's/@param \([^ ][^ ]*\) /- __`\1`__: /' \
        | sed 's/@return /Return: /' \
        | sed 's/@code{.c}/```c/' \
        | sed 's/@code/```/' \
        | sed 's/@endcode/```/' \
        | sed 's/@note/__Note:__/' \
        | sed 's/\(pub const LV_ALIGN_[^ ][^ ]*\): _[^ ]* /\1: lv_align_t /' \
        | sed 's/\(pub const LV_DESIGN_[^ ][^ ]*\): _[^ ]* /\1: lv_design_res_t /' \
        | sed 's/\(pub const LV_EVENT_[^ ][^ ]*\): _[^ ]* /\1: lv_event_t /' \
        | sed 's/\(pub const _LV_OBJ_PART_[^ ][^ ]*\): _[^ ]* /\1: lv_obj_part_t /' \
        | sed 's/\(pub const LV_OBJ_PART_[^ ][^ ]*\): _[^ ]* /\1: lv_obj_part_t /' \
        | sed 's/\(pub const LV_PROTECT_[^ ][^ ]*\): _[^ ]* /\1: lv_protect_t /' \
        | sed 's/\(pub const LV_SIGNAL_[^ ][^ ]*\): _[^ ]* /\1: lv_signal_t /' \
        | sed 's/\(pub const LV_STATE_[^ ][^ ]*\): _[^ ]* /\1: lv_state_t /' \
        | sed 's/\(pub const LV_LABEL_ALIGN_[^ ][^ ]*\): _[^ ]* /\1: lv_label_align_t /' \
        | sed 's/\(pub const LV_LABEL_LONG_[^ ][^ ]*\): _[^ ]* /\1: lv_label_long_mode_t /' \
        | sed 's/\(pub const LV_LABEL_PART_[^ ][^ ]*\): _[^ ]* /\1: lv_label_part_t /' \
        >$expandpath
    rm $tmpexpandpath
}

function generate_bindings_core() {
    #  Add whitelist and blacklist for for lv_core/lv_*
    local modname=core
    local submodname=$1  # Submodule name e.g. obj
    if [ "$submodname" == 'style' ]; then
        #  Combine lv_style.h, lv_obj.h and lv_obj_style_dec.h for processing, because lv_obj_style_dec.h contains macros that define functions like "lv_style_set_text_font"
        local headerfile=$headerprefix/src/lv_$modname/combined.h
        cat \
            $headerprefix/src/lv_$modname/lv_style.h \
            $headerprefix/src/lv_$modname/lv_obj.h \
            $headerprefix/src/lv_$modname/lv_obj_style_dec.h \
            >$headerfile
    else
        local headerfile=$headerprefix/src/lv_$modname/lv_$submodname.h
    fi
    local whitelistname=lv_$submodname
    if [ "$submodname" == 'obj' ]; then
        # For lv_obj.c, include the core constants and the core types
        local whitelisttypes=`cat << EOF
        --whitelist-var      LV_ALIGN_.* \
        --whitelist-var      LV_DESIGN_.* \
        --whitelist-var      LV_EVENT_.* \
        --whitelist-var      LV_PROTECT_.* \
        --whitelist-var      LV_SIGNAL_.* \
        --whitelist-var      LV_STATE_.* \
        --whitelist-type     lv_.*
EOF
`
        local blacklist=
    else
        # For files other than lv_obj.c, exclude the core types.
        # lv_indev_drv_* functions should be defined under lv_hal. 
        local whitelisttypes=
        local blacklist=`cat << EOF
            --blacklist-item     _lv_.* \
            --blacklist-item     lv_.*_t \
            --blacklist-item     lv_indev_drv_init \
            --blacklist-item     lv_indev_drv_register \
            --blacklist-item     lv_indev_drv_update \
            --blacklist-item     lv_indev_get_next
EOF
`
    fi
    #  TODO: Fix returned string lifetime for lv_obj_get_style_value_str.
    #  This function is probably not essential because our Rust app should already have the string.
    local whitelist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --whitelist-function (?i)${whitelistname}.* \
        --whitelist-type     (?i)${whitelistname}.* \
        --whitelist-var      (?i)${whitelistname}.* \
        ${whitelisttypes} \
        --blacklist-item     lv_obj_get_style_value_str \
        ${blacklist}
EOF
`    
    #  Generate the bindings for lv_core/lv_obj: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist

    #  Delete the combined lv_style.h and lv_obj_style_dec.h
    if [ "$submodname" == 'style' ]; then
        rm $headerfile
    fi
}

function generate_bindings_draw() {
    #  Add whitelist and blacklist for for lv_draw/lv_draw
    local modname=draw
    local submodname=draw
    local headerfile=$headerprefix/src/lv_$modname/lv_$submodname.h
    local whitelistname=lv_draw
    local whitelist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --whitelist-function (?i)${whitelistname}.* \
        --whitelist-type     (?i)${whitelistname}.* \
        --whitelist-var      (?i)${whitelistname}.* \
        --blacklist-item     _lv_obj_t \
        --blacklist-item     lv_style_t
EOF
`
    #  Generate the bindings for lv_themes/lv_theme: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist
}

function generate_bindings_font() {
    #  Add whitelist and blacklist for for lv_font/lv_font
    local modname=font
    local submodname=font
    local headerfile=$headerprefix/src/lv_$modname/lv_$submodname.h
    local whitelistname=lv_font
    local whitelist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --whitelist-function (?i)${whitelistname}.* \
        --whitelist-type     (?i)${whitelistname}.* \
        --whitelist-var      (?i)${whitelistname}.* \
        --blacklist-item     _lv_obj_t \
        --blacklist-item     lv_style_t \
        --blacklist-item     _lv_font_struct \
        --blacklist-item     lv_font_glyph_dsc_t
EOF
`
    #  Generate the bindings for lv_themes/lv_theme: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist
}

function generate_bindings_hal() {
    #  Add whitelist and blacklist for for lv_hal/lv_hal_*
    local modname=hal
    local submodname=$1  # Submodule name e.g. disp
    local headerfile=${headerprefix}/src/lv_${modname}/lv_${modname}_${submodname}.h
    local whitelistname=lv_$submodname
    local whitelist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --whitelist-function (?i)${whitelistname}.* \
        --whitelist-type     (?i)${whitelistname}.* \
        --whitelist-var      (?i)${whitelistname}.* \
        --blacklist-item     _lv_obj_t \
        --blacklist-item     lv_style_t
EOF
`
    #  Generate the bindings for lv_themes/lv_theme: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist
}

function generate_bindings_misc() {
    #  Add whitelist and blacklist for for lv_misc/lv_*
    local modname=misc
    local submodname=$1  # Submodule name e.g. anim
    local headerfile=$headerprefix/src/lv_$modname/lv_$submodname.h
    local whitelistname=lv_$submodname
    local whitelist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --whitelist-function (?i)${whitelistname}.* \
        --whitelist-type     (?i)${whitelistname}.* \
        --whitelist-var      (?i)${whitelistname}.* \
        --blacklist-item     _lv_obj_t \
        --blacklist-item     lv_style_t
EOF
`
    #  Generate the bindings for lv_widgets/lv_*: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist
}

function generate_bindings_themes() {
    #  Add whitelist and blacklist for for lv_themes/lv_theme
    local modname=themes
    local submodname=theme
    local headerfile=$headerprefix/src/lv_$modname/lv_$submodname.h
    local whitelistname=lv_theme
    local whitelist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --whitelist-function (?i)${whitelistname}.* \
        --whitelist-type     (?i)${whitelistname}.* \
        --blacklist-item     _lv_obj_t \
        --blacklist-item     lv_style_t \
        --blacklist-item     _lv_font_struct \
        --blacklist-item     lv_font_glyph_dsc_t
EOF
`
    #  Generate the bindings for lv_themes/lv_theme: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist
}

function generate_bindings_widgets() {
    #  Add whitelist and blacklist for for lv_widgets/lv_*
    local modname=widgets
    local submodname=$1  # Submodule name e.g. label
    local headerfile=$headerprefix/src/lv_$modname/lv_$submodname.h
    local whitelistname=lv_$submodname
    #  TODO: Fix returned string lifetime for lv_btnmatrix_get_active_btn_text and the other functions below that return strings.
    #  These functions (except text input) are probably not essential because our Rust app should already have these strings.
    local whitelist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --whitelist-function (?i)${whitelistname}.* \
        --whitelist-type     (?i)${whitelistname}.* \
        --whitelist-var      (?i)${whitelistname}.* \
        --blacklist-item     _lv_obj_t \
        --blacklist-item     lv_style_t \
        --blacklist-item     lv_btnmatrix_get_active_btn_text \
        --blacklist-item     lv_btnmatrix_get_btn_text \
        --blacklist-item     lv_checkbox_get_text \
        --blacklist-item     lv_dropdown_get_text \
        --blacklist-item     lv_dropdown_get_options \
        --blacklist-item     lv_dropdown_get_symbol \
        --blacklist-item     lv_img_get_file_name \
        --blacklist-item     lv_list_get_btn_text \
        --blacklist-item     lv_msgbox_get_text \
        --blacklist-item     lv_msgbox_get_active_btn_text \
        --blacklist-item     lv_roller_get_options \
        --blacklist-item     lv_table_get_cell_value \
        --blacklist-item     lv_textarea_get_text \
        --blacklist-item     lv_textarea_get_placeholder_text \
        --blacklist-item     lv_textarea_get_accepted_chars \
        --blacklist-item     lv_win_get_title
EOF
`
    #  Generate the bindings for lv_widgets/lv_*: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist
}

#  Generate bindings for lv_core
generate_bindings_core disp
generate_bindings_core group
generate_bindings_core indev
generate_bindings_core obj
generate_bindings_core refr
generate_bindings_core style

#  Generate bindings for lv_draw
generate_bindings_draw

#  Generate bindings for lv_font
generate_bindings_font

#  Generate bindings for lv_hal
generate_bindings_hal disp
generate_bindings_hal indev
generate_bindings_hal tick

#  Generate bindings for lv_misc
generate_bindings_misc anim
generate_bindings_misc area
generate_bindings_misc async
generate_bindings_misc bidi
generate_bindings_misc color
generate_bindings_misc debug
generate_bindings_misc fs
generate_bindings_misc gc
generate_bindings_misc ll
generate_bindings_misc log
generate_bindings_misc math
generate_bindings_misc mem
generate_bindings_misc printf
generate_bindings_misc task
generate_bindings_misc templ
generate_bindings_misc txt
generate_bindings_misc txt_ap
generate_bindings_misc types
generate_bindings_misc utils

#  Generate bindings for lv_themes
generate_bindings_themes

#  Generate bindings for lv_widgets
generate_bindings_widgets arc
generate_bindings_widgets bar
generate_bindings_widgets btn
generate_bindings_widgets btnmatrix
generate_bindings_widgets calendar
generate_bindings_widgets canvas
generate_bindings_widgets chart
generate_bindings_widgets checkbox
generate_bindings_widgets cont
generate_bindings_widgets cpicker
generate_bindings_widgets dropdown
generate_bindings_widgets gauge
generate_bindings_widgets img
generate_bindings_widgets imgbtn
generate_bindings_widgets keyboard
generate_bindings_widgets label
generate_bindings_widgets led
generate_bindings_widgets line
generate_bindings_widgets linemeter
generate_bindings_widgets list
generate_bindings_widgets msgbox
generate_bindings_widgets objmask
generate_bindings_widgets objx_templ
generate_bindings_widgets page
generate_bindings_widgets roller
generate_bindings_widgets slider
generate_bindings_widgets spinbox
generate_bindings_widgets spinner
generate_bindings_widgets switch
generate_bindings_widgets table
generate_bindings_widgets tabview
generate_bindings_widgets textarea
generate_bindings_widgets tileview
generate_bindings_widgets win

#  Expand the safe wrapper macros
cargo rustc -- -Z unstable-options --pretty expanded >logs/expanded.rs

#  Build the bindings
cargo build

#  Generate the doc for inspection
cargo doc

exit

→ bindgen --help
bindgen 0.49.2
Generates Rust bindings from C/C++ headers.

USAGE:
    bindgen [FLAGS] [OPTIONS] <header> -- <clang-args>...

FLAGS:
        --block-extern-crate                     Use extern crate instead of use for block.
        --builtins                               Output bindings for builtin definitions, e.g. __builtin_va_list.
        --conservative-inline-namespaces         Conservatively generate inline namespaces to avoid name conflicts.
        --disable-name-namespacing
            Disable namespacing via mangling, causing bindgen to generate names like "Baz" instead of "foo_bar_Baz" for
            an input name "foo::bar::Baz".
        --distrust-clang-mangling                Do not trust the libclang-provided mangling
        --dump-preprocessed-input
            Preprocess and dump the input header files to disk. Useful when debugging bindgen, using C-Reduce, or when
            filing issues. The resulting file will be named something like `__bindgen.i` or `__bindgen.ii`.
        --emit-clang-ast                         Output the Clang AST for debugging purposes.
        --emit-ir                                Output our internal IR for debugging purposes.
        --enable-cxx-namespaces                  Enable support for C++ namespaces.
        --enable-function-attribute-detection
            Enables detecting unexposed attributes in functions (slow).
                                   Used to generate #[must_use] annotations.
        --generate-block                         Generate block signatures instead of void pointers.
        --generate-inline-functions              Generate inline functions.
    -h, --help                                   Prints help information
        --ignore-functions
            Do not generate bindings for functions or methods. This is useful when you only care about struct layouts.

        --ignore-methods                         Do not generate bindings for methods.
        --impl-debug                             Create Debug implementation, if it can not be derived automatically.
        --impl-partialeq
            Create PartialEq implementation, if it can not be derived automatically.

        --no-convert-floats                      Do not automatically convert floats to f32/f64.
        --no-derive-copy                         Avoid deriving Copy on any type.
        --no-derive-debug                        Avoid deriving Debug on any type.
        --no-doc-comments
            Avoid including doc comments in the output, see: https://github.com/rust-lang/rust-bindgen/issues/426

        --no-include-path-detection              Do not try to detect default include paths
        --no-layout-tests                        Avoid generating layout tests for any type.
        --no-prepend-enum-name                   Do not prepend the enum name to bitfield or constant variants.
        --no-record-matches
            Do not record matching items in the regex sets. This disables reporting of unused items.

        --no-recursive-whitelist
            Disable whitelisting types recursively. This will cause bindgen to emit Rust code that won't compile! See
            the `bindgen::Builder::whitelist_recursively` method's documentation for details.
        --no-rustfmt-bindings                    Do not format the generated bindings with rustfmt.
        --objc-extern-crate                      Use extern crate instead of use for objc.
        --rustfmt-bindings
            Format the generated bindings with rustfmt. DEPRECATED: --rustfmt-bindings is now enabled by default.
            Disable with --no-rustfmt-bindings.
        --time-phases                            Time the different bindgen phases and print to stderr
        --unstable-rust                          Generate unstable Rust code (deprecated; use --rust-target instead).
        --use-array-pointers-in-arguments        Use `*const [T; size]` instead of `*const T` for C arrays
        --use-core                               Use types from Rust core instead of std.
        --use-msvc-mangling                      MSVC C++ ABI mangling. DEPRECATED: Has no effect.
    -V, --version                                Prints version information
        --verbose                                Print verbose error messages.
        --with-derive-default                    Derive Default on any type.
        --with-derive-eq
            Derive eq on any type. Enable this option also enables --with-derive-partialeq

        --with-derive-hash                       Derive hash on any type.
        --with-derive-ord
            Derive ord on any type. Enable this option also enables --with-derive-partialord

        --with-derive-partialeq                  Derive partialeq on any type.
        --with-derive-partialord                 Derive partialord on any type.

OPTIONS:
        --bitfield-enum <regex>...             Mark any enum whose name matches <regex> as a set of bitfield flags.
        --blacklist-function <function>...     Mark <function> as hidden.
        --blacklist-item <item>...             Mark <item> as hidden.
        --blacklist-type <type>...             Mark <type> as hidden.
        --constified-enum <regex>...           Mark any enum whose name matches <regex> as a series of constants.
        --constified-enum-module <regex>...    Mark any enum whose name matches <regex> as a module of constants.
        --ctypes-prefix <prefix>               Use the given prefix before raw types instead of ::std::os::raw.
        --default-enum-style <variant>         The default style of code used to generate enums. [default: consts]
                                               [possible values: consts, moduleconsts, bitfield, rust]
        --emit-ir-graphviz <path>              Dump graphviz dot file.
        --generate <generate>                  Generate only given items, split by commas. Valid values are
                                               "functions","types", "vars", "methods", "constructors" and "destructors".
        --no-copy <regex>...                   Avoid deriving Copy for types matching <regex>.
        --no-hash <regex>...                   Avoid deriving Hash for types matching <regex>.
        --no-partialeq <regex>...              Avoid deriving PartialEq for types matching <regex>.
        --opaque-type <type>...                Mark <type> as opaque.
    -o, --output <output>                      Write Rust bindings to <output>.
        --raw-line <raw-line>...               Add a raw line of Rust code at the beginning of output.
        --rust-target <rust-target>            Version of the Rust compiler to target. Valid options are: ["1.0",
                                               "1.19", "1.20", "1.21", "1.25", "1.26", "1.27", "1.28", "1.33",
                                               "nightly"]. Defaults to "1.33".
        --rustfmt-configuration-file <path>    The absolute path to the rustfmt configuration file. The configuration
                                               file will be used for formatting the bindings. This parameter is
                                               incompatible with --no-rustfmt-bindings.
        --rustified-enum <regex>...            Mark any enum whose name matches <regex> as a Rust enum.
        --whitelist-function <regex>...        Whitelist all the free-standing functions matching <regex>. Other non-
                                               whitelisted functions will not be generated.
        --whitelist-type <regex>...            Only generate types matching <regex>. Other non-whitelisted types will
                                               not be generated.
        --whitelist-var <regex>...             Whitelist all the free-standing variables matching <regex>. Other non-
                                               whitelisted variables will not be generated.

ARGS:
    <header>           C or C++ header file
    <clang-args>...
