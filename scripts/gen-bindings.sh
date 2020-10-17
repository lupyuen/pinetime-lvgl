#!/usr/bin/env bash
#  Generate Rust bindings for LVGL C API. Install "bindgen" before running:
#  cargo install bindgen
#  Also install rustfmt when prompted
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
        | sed 's/\(pub const LV_LABEL_ALIGN_[^ ][^ ]*\): _[^ ]* /\1: lv_label_align_t /' \
        | sed 's/\(pub const LV_LABEL_LONG_[^ ][^ ]*\): _[^ ]* /\1: lv_label_long_mode_t /' \
        >$expandpath
    rm $tmpexpandpath
}

function generate_bindings_core() {
    #  Add whitelist and blacklist for for lv_core/lv_obj
    local modname=core
    local submodname=obj
    local headerfile=$headerprefix/src/lv_$modname/lv_$submodname.h
    local whitelistname=lv_
    #  TODO: Fix lifetime parameter for lv_obj_get_style_value_str
    local whitelist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --whitelist-function (?i)${whitelistname}.* \
        --whitelist-type     (?i)${whitelistname}.* \
        --whitelist-var      (?i)${whitelistname}.* \
        --blacklist-item     lv_obj_get_style_value_str
EOF
`
    #  Generate the bindings for lv_core/lv_obj: libname, modname, submodname, headerfile, whitelist
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
        --blacklist-item     lv_style_t
EOF
`
    #  Generate the bindings for lv_themes/lv_theme: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist
}

function generate_bindings_widgets() {
    #  Add whitelist and blacklist for for lv_widgets/lv_label
    #  TODO: Handle other widgets
    local modname=widgets
    local submodname=label
    local headerfile=$headerprefix/src/lv_$modname/lv_$submodname.h
    local whitelistname=lv_label
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
    #  Generate the bindings for lv_widgets/lv_label: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist
}

#  Generate bindings for lv_core
generate_bindings_core

#  TODO: Generate bindings for lv_draw
#  generate_bindings_draw

#  TODO: Generate bindings for lv_font
#  generate_bindings_font

#  TODO: Generate bindings for lv_hal
#  generate_bindings_hal

#  TODO: Generate bindings for lv_misc
#  generate_bindings_misc

#  Generate bindings for lv_themes
generate_bindings_themes

#  Generate bindings for lv_widgets
generate_bindings_widgets

#  Expand the safe wrapper macros
cargo rustc -- -Z unstable-options --pretty expanded >logs/expanded.rs

#  Build the bindings
cargo build

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
