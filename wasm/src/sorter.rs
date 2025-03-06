use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::sync::OnceLock;

static REMOVE_DUPLICATES: OnceLock<Mutex<bool>> = OnceLock::new();
static DEBUG_MODE: OnceLock<Mutex<bool>> = OnceLock::new();

pub fn set_remove_duplicates(remove: bool) {
    let mutex = REMOVE_DUPLICATES.get_or_init(|| Mutex::new(true));
    if let Ok(mut value) = mutex.lock() {
        *value = remove;
    }
}

pub fn set_debug_mode(debug: bool) {
    let mutex = DEBUG_MODE.get_or_init(|| Mutex::new(false));
    if let Ok(mut value) = mutex.lock() {
        *value = debug;
    }
}

pub fn is_debug_enabled() -> bool {
    let mutex = DEBUG_MODE.get_or_init(|| Mutex::new(false));
    mutex.lock().map(|guard| *guard).unwrap_or(false)
}

static PROPERTY_ORDER: OnceLock<HashMap<&'static str, usize>> = OnceLock::new();
static TAILWIND_PROPERTY_MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
static STANDALONE_UTILITIES: OnceLock<HashSet<&'static str>> = OnceLock::new();
static VALID_PREFIXES: OnceLock<HashSet<&'static str>> = OnceLock::new();

fn get_property_order_map() -> &'static HashMap<&'static str, usize> {
    PROPERTY_ORDER.get_or_init(|| {
        let mut order = HashMap::new();
        let properties = [
            "container-type",              // 0
            "pointer-events",              // 1
            "visibility",                  // 2
            "position",                    // 3
            "inset",                       // 4
            "inset-inline",                // 5
            "inset-block",                 // 6
            "inset-inline-start",          // 7
            "inset-inline-end",            // 8
            "top",                         // 9
            "right",                       // 10
            "bottom",                      // 11
            "left",                        // 12
            "isolation",                   // 13
            "z-index",                     // 14
            "order",                       // 15
            "grid-column",                 // 16
            "grid-column-start",           // 17
            "grid-column-end",             // 18
            "grid-row",                    // 19
            "grid-row-start",              // 20
            "grid-row-end",                // 21
            "float",                       // 22
            "clear",                       // 23
            "--tw-container-component",    // 24
            "margin",                      // 25
            "margin-inline",               // 26
            "margin-block",                // 27
            "margin-inline-start",         // 28
            "margin-inline-end",           // 29
            "margin-top",                  // 30
            "margin-right",                // 31
            "margin-bottom",               // 32
            "margin-left",                 // 33
            "box-sizing",                  // 34
            "display",                     // 35
            "field-sizing",                // 36
            "aspect-ratio",                // 37
            "height",                      // 38
            "max-height",                  // 39
            "min-height",                  // 40
            "width",                       // 41
            "max-width",                   // 42
            "min-width",                   // 43
            "flex",                        // 44
            "flex-shrink",                 // 45
            "flex-grow",                   // 46
            "flex-basis",                  // 47
            "table-layout",                // 48
            "caption-side",                // 49
            "border-collapse",             // 50
            "border-spacing",              // 51
            "transform-origin",            // 52
            "translate",                   // 53
            "--tw-translate-x",            // 54
            "--tw-translate-y",            // 55
            "--tw-translate-z",            // 56
            "scale",                       // 57
            "--tw-scale-x",                // 58
            "--tw-scale-y",                // 59
            "--tw-scale-z",                // 60
            "rotate",                      // 61
            "--tw-rotate-x",               // 62
            "--tw-rotate-y",               // 63
            "--tw-rotate-z",               // 64
            "--tw-skew-x",                 // 65
            "--tw-skew-y",                 // 66
            "transform",                   // 67
            "animation",                   // 68
            "cursor",                      // 69
            "touch-action",                // 70
            "--tw-pan-x",                  // 71
            "--tw-pan-y",                  // 72
            "--tw-pinch-zoom",             // 73
            "resize",                      // 74
            "scroll-snap-type",            // 75
            "--tw-scroll-snap-strictness", // 76
            "scroll-snap-align",           // 77
            "scroll-snap-stop",            // 78
            "scroll-margin",               // 79
            "scroll-margin-inline",        // 80
            "scroll-margin-block",         // 81
            "scroll-margin-inline-start",  // 82
            "scroll-margin-inline-end",    // 83
            "scroll-margin-top",           // 84
            "scroll-margin-right",         // 85
            "scroll-margin-bottom",        // 86
            "scroll-margin-left",          // 87
            "scroll-padding",              // 88
            "scroll-padding-inline",       // 89
            "scroll-padding-block",        // 90
            "scroll-padding-inline-start", // 91
            "scroll-padding-inline-end",   // 92
            "scroll-padding-top",          // 93
            "scroll-padding-right",        // 94
            "scroll-padding-bottom",       // 95
            "scroll-padding-left",         // 96
            "list-style-position",         // 97
            "list-style-type",             // 98
            "list-style-image",            // 99
            "appearance",                  // 100
            "columns",                     // 101
            "break-before",                // 102
            "break-inside",                // 103
            "break-after",                 // 104
            "grid-auto-columns",           // 105
            "grid-auto-flow",              // 106
            "grid-auto-rows",              // 107
            "grid-template-columns",       // 108
            "grid-template-rows",          // 109
            "flex-direction",              // 110
            "flex-wrap",                   // 111
            "place-content",               // 112
            "place-items",                 // 113
            "align-content",               // 114
            "align-items",                 // 115
            "justify-content",             // 116
            "justify-items",               // 117
            "gap",                         // 118
            "column-gap",                  // 119
            "row-gap",                     // 120
            "--tw-space-x-reverse",        // 121
            "--tw-space-y-reverse",        // 122
            "divide-x-width",              // 123
            "divide-y-width",              // 124
            "--tw-divide-y-reverse",       // 125
            "divide-style",                // 126
            "divide-color",                // 127
            "place-self",                  // 128
            "align-self",                  // 129
            "justify-self",                // 130
            "overflow",                    // 131
            "overflow-x",                  // 132
            "overflow-y",                  // 133
            "overscroll-behavior",         // 134
            "overscroll-behavior-x",       // 135
            "overscroll-behavior-y",       // 136
            "scroll-behavior",             // 137
            "border-radius",               // 138
            "border-start-radius",         // 139
            "border-end-radius",           // 140
            "border-top-radius",           // 141
            "border-right-radius",         // 142
            "border-bottom-radius",        // 143
            "border-left-radius",          // 144
            "border-start-start-radius",   // 145
            "border-start-end-radius",     // 146
            "border-end-end-radius",       // 147
            "border-end-start-radius",     // 148
            "border-top-left-radius",      // 149
            "border-top-right-radius",     // 150
            "border-bottom-right-radius",  // 151
            "border-bottom-left-radius",   // 152
            "border-width",                // 153
            "border-inline-width",         // 154
            "border-block-width",          // 155
            "border-inline-start-width",   // 156
            "border-inline-end-width",     // 157
            "border-top-width",            // 158
            "border-right-width",          // 159
            "border-bottom-width",         // 160
            "border-left-width",           // 161
            "border-style",                // 162
            "border-inline-style",         // 163
            "border-block-style",          // 164
            "border-inline-start-style",   // 165
            "border-inline-end-style",     // 166
            "border-top-style",            // 167
            "border-right-style",          // 168
            "border-bottom-style",         // 169
            "border-left-style",           // 170
            "border-color",                // 171
            "border-inline-color",         // 172
            "border-block-color",          // 173
            "border-inline-start-color",   // 174
            "border-inline-end-color",     // 175
            "border-top-color",            // 176
            "border-right-color",          // 177
            "border-bottom-color",         // 178
            "border-left-color",           // 179
            "background-color",            // 180
            "background-image",            // 181
            "--tw-gradient-position",      // 182
            "--tw-gradient-stops",         // 183
            "--tw-gradient-via-stops",     // 184
            "--tw-gradient-from",          // 185
            "--tw-gradient-from-position", // 186
            "--tw-gradient-via",           // 187
            "--tw-gradient-via-position",  // 188
            "--tw-gradient-to",            // 189
            "--tw-gradient-to-position",   // 190
            "box-decoration-break",        // 191
            "background-size",             // 192
            "background-attachment",       // 193
            "background-clip",             // 194
            "background-position",         // 195
            "background-repeat",           // 196
            "background-origin",           // 197
            "fill",                        // 198
            "stroke",                      // 199
            "stroke-width",                // 200
            "object-fit",                  // 201
            "object-position",             // 202
            "padding",                     // 203
            "padding-inline",              // 204
            "padding-block",               // 205
            "padding-inline-start",        // 206
            "padding-inline-end",          // 207
            "padding-top",                 // 208
            "padding-right",               // 209
            "padding-bottom",              // 210
            "padding-left",                // 211
            "text-align",                  // 212
            "text-indent",                 // 213
            "vertical-align",              // 214
            "font-family",                 // 215
            "font-size",                   // 216
            "line-height",                 // 217
            "font-weight",                 // 218
            "letter-spacing",              // 219
            "text-wrap",                   // 220
            "overflow-wrap",               // 221
            "word-break",                  // 222
            "text-overflow",               // 223
            "hyphens",                     // 224
            "white-space",                 // 225
            "color",                       // 226
            "text-transform",              // 227
            "font-style",                  // 228
            "font-stretch",                // 229
            "font-variant-numeric",        // 230
            "text-decoration-line",        // 231
            "text-decoration-color",       // 232
            "text-decoration-style",       // 233
            "text-decoration-thickness",   // 234
            "text-underline-offset",       // 235
            "-webkit-font-smoothing",      // 236
            "placeholder-color",           // 237
            "caret-color",                 // 238
            "accent-color",                // 239
            "color-scheme",                // 240
            "opacity",                     // 241
            "background-blend-mode",       // 242
            "mix-blend-mode",              // 243
            "box-shadow",                  // 244
            "--tw-shadow",                 // 245
            "--tw-shadow-color",           // 246
            "--tw-ring-shadow",            // 247
            "--tw-ring-color",             // 248
            "--tw-inset-shadow",           // 249
            "--tw-inset-shadow-color",     // 250
            "--tw-inset-ring-shadow",      // 251
            "--tw-inset-ring-color",       // 252
            "--tw-ring-offset-width",      // 253
            "--tw-ring-offset-color",      // 254
            "outline",                     // 255
            "outline-width",               // 256
            "outline-offset",              // 257
            "outline-color",               // 258
            "--tw-blur",                   // 259
            "--tw-brightness",             // 260
            "--tw-contrast",               // 261
            "--tw-drop-shadow",            // 262
            "--tw-grayscale",              // 263
            "--tw-hue-rotate",             // 264
            "--tw-invert",                 // 265
            "--tw-saturate",               // 266
            "--tw-sepia",                  // 267
            "filter",                      // 268
            "--tw-backdrop-blur",          // 269
            "--tw-backdrop-brightness",    // 270
            "--tw-backdrop-contrast",      // 271
            "--tw-backdrop-grayscale",     // 272
            "--tw-backdrop-hue-rotate",    // 273
            "--tw-backdrop-invert",        // 274
            "--tw-backdrop-opacity",       // 275
            "--tw-backdrop-saturate",      // 276
            "--tw-backdrop-sepia",         // 277
            "backdrop-filter",             // 278
            "transition-property",         // 279
            "transition-behavior",         // 280
            "transition-delay",            // 281
            "transition-duration",         // 282
            "transition-timing-function",  // 283
            "will-change",                 // 284
            "contain",                     // 285
            "content",                     // 286
            "forced-color-adjust",         // 287
        ];

        for (index, &prop) in properties.iter().enumerate() {
            order.insert(prop, index);
        }
        order
    })
}

fn get_tailwind_property_map() -> &'static HashMap<&'static str, &'static str> {
    TAILWIND_PROPERTY_MAP.get_or_init(|| {
        let mut map = HashMap::new();

        // Pointer events
        map.insert("pointer-events-", "pointer-events");

        // Position and layout
        map.insert("static", "position");
        map.insert("fixed", "position");
        map.insert("absolute", "position");
        map.insert("relative", "position");
        map.insert("sticky", "position");

        map.insert("inset-", "inset");
        map.insert("top-", "top");
        map.insert("right-", "right");
        map.insert("bottom-", "bottom");
        map.insert("left-", "left");

        map.insert("z-", "z-index");
        map.insert("order-", "order");

        // Float/Clear
        map.insert("float-", "float");
        map.insert("clear-", "clear");

        // Box model
        map.insert("m-", "margin");
        map.insert("mx-", "margin-inline");
        map.insert("my-", "margin-block");
        map.insert("mt-", "margin-top");
        map.insert("mr-", "margin-right");
        map.insert("mb-", "margin-bottom");
        map.insert("ml-", "margin-left");

        map.insert("p-", "padding");
        map.insert("px-", "padding-inline");
        map.insert("py-", "padding-block");
        map.insert("pt-", "padding-top");
        map.insert("pr-", "padding-right");
        map.insert("pb-", "padding-bottom");
        map.insert("pl-", "padding-left");

        map.insert("w-", "width");
        map.insert("min-w-", "min-width");
        map.insert("max-w-", "max-width");
        map.insert("h-", "height");
        map.insert("min-h-", "min-height");
        map.insert("max-h-", "max-height");

        // Display
        map.insert("block", "display");
        map.insert("inline-block", "display");
        map.insert("inline", "display");
        map.insert("flex", "display");
        map.insert("inline-flex", "display");
        map.insert("table", "display");
        map.insert("grid", "display");
        map.insert("inline-grid", "display");
        map.insert("contents", "display");
        map.insert("hidden", "display");

        // Visibility
        map.insert("visible", "visibility");
        map.insert("invisible", "visibility");

        // Aspect ratio
        map.insert("aspect-", "aspect-ratio");

        // Flexbox
        map.insert("flex-", "flex");
        map.insert("flex-grow", "flex-grow");
        map.insert("flex-grow-", "flex-grow");
        map.insert("flex-shrink", "flex-shrink");
        map.insert("flex-shrink-", "flex-shrink");
        map.insert("flex-basis-", "flex-basis");

        // Grid
        map.insert("grid-cols-", "grid-template-columns");
        map.insert("grid-rows-", "grid-template-rows");
        map.insert("grid-col-", "grid-column");
        map.insert("col-", "grid-column");
        map.insert("row-", "grid-row");
        map.insert("grid-flow-", "grid-auto-flow");

        // Gap
        map.insert("gap-", "gap");
        map.insert("gap-x-", "column-gap");
        map.insert("gap-y-", "row-gap");

        // Alignment
        map.insert("justify-", "justify-content");
        map.insert("content-", "align-content");
        map.insert("items-", "align-items");
        map.insert("self-", "align-self");
        map.insert("place-", "place-content");

        // Spacing
        map.insert("space-x-", "--tw-space-x-reverse");
        map.insert("space-y-", "--tw-space-y-reverse");
        map.insert("divide-x-", "divide-x-width");
        map.insert("divide-y-", "divide-y-width");
        map.insert("divide-", "divide-style");

        // Accessibility
        map.insert("sr-only", "position");
        map.insert("not-sr-only", "position");

        // User select
        map.insert("select-", "user-select");

        // Isolation
        map.insert("isolate", "isolation");
        map.insert("isolation-", "isolation");

        // Overflow/Scroll
        map.insert("overflow-", "overflow");
        map.insert("overscroll-", "overscroll-behavior");
        map.insert("scroll-", "scroll-behavior");

        // Resize
        map.insert("resize", "resize");
        map.insert("resize-", "resize");

        // Border radius - handle all variants
        map.insert("rounded", "border-radius");
        map.insert("rounded-", "border-radius");

        // Top
        map.insert("rounded-t", "border-top-radius");
        map.insert("rounded-t-", "border-top-radius");

        // Right
        map.insert("rounded-r", "border-right-radius");
        map.insert("rounded-r-", "border-right-radius");

        // Bottom
        map.insert("rounded-b", "border-bottom-radius");
        map.insert("rounded-b-", "border-bottom-radius");

        // Left
        map.insert("rounded-l", "border-left-radius");
        map.insert("rounded-l-", "border-left-radius");

        // Corners
        map.insert("rounded-tl", "border-top-left-radius");
        map.insert("rounded-tl-", "border-top-left-radius");
        map.insert("rounded-tr", "border-top-right-radius");
        map.insert("rounded-tr-", "border-top-right-radius");
        map.insert("rounded-br", "border-bottom-right-radius");
        map.insert("rounded-br-", "border-bottom-right-radius");
        map.insert("rounded-bl", "border-bottom-left-radius");
        map.insert("rounded-bl-", "border-bottom-left-radius");

        // Logical properties (Tailwind v3+)
        map.insert("rounded-s", "border-start-radius");
        map.insert("rounded-s-", "border-start-radius");
        map.insert("rounded-e", "border-end-radius");
        map.insert("rounded-e-", "border-end-radius");
        map.insert("rounded-ss", "border-start-start-radius");
        map.insert("rounded-ss-", "border-start-start-radius");
        map.insert("rounded-se", "border-start-end-radius");
        map.insert("rounded-se-", "border-start-end-radius");
        map.insert("rounded-ee", "border-end-end-radius");
        map.insert("rounded-ee-", "border-end-end-radius");
        map.insert("rounded-es", "border-end-start-radius");
        map.insert("rounded-es-", "border-end-start-radius");

        // Border
        map.insert("border", "border-width");
        map.insert("border-", "border-width");
        map.insert("border-t", "border-top-width");
        map.insert("border-t-", "border-top-width");
        map.insert("border-r", "border-right-width");
        map.insert("border-r-", "border-right-width");
        map.insert("border-b", "border-bottom-width");
        map.insert("border-b-", "border-bottom-width");
        map.insert("border-l", "border-left-width");
        map.insert("border-l-", "border-left-width");

        // Border style
        map.insert("border-solid", "border-style");
        map.insert("border-dashed", "border-style");
        map.insert("border-dotted", "border-style");
        map.insert("border-double", "border-style");
        map.insert("border-hidden", "border-style");
        map.insert("border-none", "border-style");

        // Border collapse
        map.insert("border-collapse", "border-collapse");
        map.insert("border-separate", "border-collapse");

        // Background
        map.insert("bg-", "background-color");
        map.insert("bg-gradient-", "background-image");
        map.insert("bg-clip-", "background-clip");
        map.insert("bg-origin-", "background-origin");
        map.insert("bg-repeat", "background-repeat");
        map.insert("bg-repeat-", "background-repeat");
        map.insert("bg-fixed", "background-attachment");
        map.insert("bg-local", "background-attachment");
        map.insert("bg-scroll", "background-attachment");

        // Object
        map.insert("object-", "object-fit");
        map.insert("object-position-", "object-position");

        // Typography
        map.insert("font-", "font-family");
        map.insert("text-", "color");
        map.insert("tracking-", "letter-spacing");
        map.insert("leading-", "line-height");
        map.insert("whitespace-", "white-space");
        map.insert("break-", "word-break");
        map.insert("truncate", "text-overflow");
        map.insert("hyphens-", "hyphens");

        // Font style/weight
        map.insert("italic", "font-style");
        map.insert("not-italic", "font-style");
        map.insert("font-thin", "font-weight");
        map.insert("font-extralight", "font-weight");
        map.insert("font-light", "font-weight");
        map.insert("font-normal", "font-weight");
        map.insert("font-medium", "font-weight");
        map.insert("font-semibold", "font-weight");
        map.insert("font-bold", "font-weight");
        map.insert("font-extrabold", "font-weight");
        map.insert("font-black", "font-weight");

        // Text transform
        map.insert("uppercase", "text-transform");
        map.insert("lowercase", "text-transform");
        map.insert("capitalize", "text-transform");
        map.insert("normal-case", "text-transform");

        // Text decoration
        map.insert("underline", "text-decoration-line");
        map.insert("overline", "text-decoration-line");
        map.insert("line-through", "text-decoration-line");
        map.insert("no-underline", "text-decoration-line");
        map.insert("decoration-", "text-decoration");

        // Text alignment
        map.insert("text-left", "text-align");
        map.insert("text-center", "text-align");
        map.insert("text-right", "text-align");
        map.insert("text-justify", "text-align");
        map.insert("text-start", "text-align");
        map.insert("text-end", "text-align");

        // Placeholder color
        map.insert("placeholder-", "placeholder-color");

        // Caret/Accent colors
        map.insert("caret-", "caret-color");
        map.insert("accent-", "accent-color");

        // Effects
        map.insert("shadow", "box-shadow");
        map.insert("shadow-", "box-shadow");
        map.insert("opacity-", "opacity");
        map.insert("mix-blend-", "mix-blend-mode");
        map.insert("bg-blend-", "background-blend-mode");

        // Transitions
        map.insert("transition", "transition-property");
        map.insert("transition-", "transition-property");
        map.insert("duration-", "transition-duration");
        map.insert("ease-", "transition-timing-function");
        map.insert("delay-", "transition-delay");

        // Transforms
        map.insert("origin-", "transform-origin");
        map.insert("scale-", "scale");
        map.insert("rotate-", "rotate");
        map.insert("translate-", "translate");
        map.insert("skew-", "skew");
        map.insert("transform", "transform");
        map.insert("transform-", "transform");

        // Filters
        map.insert("blur", "--tw-blur");
        map.insert("blur-", "--tw-blur");
        map.insert("brightness", "--tw-brightness");
        map.insert("brightness-", "--tw-brightness");
        map.insert("contrast", "--tw-contrast");
        map.insert("contrast-", "--tw-contrast");
        map.insert("grayscale", "--tw-grayscale");
        map.insert("grayscale-", "--tw-grayscale");
        map.insert("hue-rotate", "--tw-hue-rotate");
        map.insert("hue-rotate-", "--tw-hue-rotate");
        map.insert("invert", "--tw-invert");
        map.insert("invert-", "--tw-invert");
        map.insert("saturate", "--tw-saturate");
        map.insert("saturate-", "--tw-saturate");
        map.insert("sepia", "--tw-sepia");
        map.insert("sepia-", "--tw-sepia");
        map.insert("drop-shadow", "--tw-drop-shadow");
        map.insert("drop-shadow-", "--tw-drop-shadow");
        map.insert("filter", "filter");
        map.insert("filter-", "filter");

        // Backdrop filters
        map.insert("backdrop-blur", "--tw-backdrop-blur");
        map.insert("backdrop-blur-", "--tw-backdrop-blur");
        map.insert("backdrop-brightness", "--tw-backdrop-brightness");
        map.insert("backdrop-brightness-", "--tw-backdrop-brightness");
        map.insert("backdrop-contrast", "--tw-backdrop-contrast");
        map.insert("backdrop-contrast-", "--tw-backdrop-contrast");
        map.insert("backdrop-grayscale", "--tw-backdrop-grayscale");
        map.insert("backdrop-grayscale-", "--tw-backdrop-grayscale");
        map.insert("backdrop-hue-rotate", "--tw-backdrop-hue-rotate");
        map.insert("backdrop-hue-rotate-", "--tw-backdrop-hue-rotate");
        map.insert("backdrop-invert", "--tw-backdrop-invert");
        map.insert("backdrop-invert-", "--tw-backdrop-invert");
        map.insert("backdrop-opacity", "--tw-backdrop-opacity");
        map.insert("backdrop-opacity-", "--tw-backdrop-opacity");
        map.insert("backdrop-saturate", "--tw-backdrop-saturate");
        map.insert("backdrop-saturate-", "--tw-backdrop-saturate");
        map.insert("backdrop-sepia", "--tw-backdrop-sepia");
        map.insert("backdrop-sepia-", "--tw-backdrop-sepia");
        map.insert("backdrop-filter", "backdrop-filter");
        map.insert("backdrop-filter-", "backdrop-filter");

        // Ring
        map.insert("ring", "--tw-ring-shadow");
        map.insert("ring-", "--tw-ring-color");
        map.insert("ring-offset", "--tw-ring-offset-width");
        map.insert("ring-offset-", "--tw-ring-offset-color");
        map.insert("ring-inset", "--tw-ring-inset");

        // Outline
        map.insert("outline", "outline");
        map.insert("outline-", "outline");
        map.insert("outline-offset-", "outline-offset");
        map.insert("outline-none", "outline");

        // Special
        map.insert("container", "--tw-container-component");
        map.insert("animate", "animation");
        map.insert("animate-", "animation");

        // Group and Peer (parasite utilities)
        map.insert("group", "display"); // Not exactly display, but close enough for ordering
        map.insert("peer", "display"); // Not exactly display, but close enough for ordering

        // SVG
        map.insert("fill-", "fill");
        map.insert("stroke-", "stroke");
        map.insert("stroke-width-", "stroke-width");

        // Tables
        map.insert("table-", "table-layout");

        // Will-change
        map.insert("will-change-", "will-change");

        // Screen readers
        map.insert("sr-", "position");

        map
    })
}

/// Main function to sort Tailwind classes
pub fn sort_classes(class_string: &str) -> String {
    // Special case for empty string
    if class_string.trim().is_empty() {
        return class_string.to_string();
    }

    // Don't sort class attributes containing `{{`, to match Prettier behavior
    if class_string.contains("{{") {
        if is_debug_enabled() {
            crate::debug_log!("Not sorting classes containing '{{': {}", class_string);
        }
        return class_string.to_string();
    }

    let classes: Vec<&str> = class_string.split_whitespace().collect();

    // Handle ellipsis special case
    let has_ellipsis = classes.iter().any(|&c| c == "..." || c == "…");
    let classes_without_ellipsis: Vec<&str> = classes
        .iter()
        .filter(|&&c| c != "..." && c != "…")
        .copied()
        .collect();

    // If duplicates should be removed, do it
    let mutex = REMOVE_DUPLICATES.get_or_init(|| Mutex::new(true));
    let remove_duplicates_flag = mutex.lock().map(|guard| *guard).unwrap_or(true);

    let filtered_classes = if remove_duplicates_flag {
        remove_duplicates(&classes_without_ellipsis)
    } else {
        classes_without_ellipsis
    };

    // Process the Tailwind classes according to their categories
    let ordered_classes = sort_tailwind_classes(&filtered_classes);

    // Build final result - add ellipsis at the end if it existed
    let mut result = ordered_classes;
    if has_ellipsis {
        result.push("...");
    }

    if is_debug_enabled() {
        crate::debug_log!("Original: {}", class_string);
        crate::debug_log!("Sorted:   {}", result.join(" "));
    }

    result.join(" ")
}

/// Sort Tailwind classes according to their position in CSS output
fn sort_tailwind_classes<'a>(classes: &[&'a str]) -> Vec<&'a str> {
    // Separate classes by type
    let (custom_classes, tailwind_classes): (Vec<&str>, Vec<&str>) = classes
        .iter()
        .copied()
        .partition(|&c| !is_tailwind_class(c));

    // Extract container class
    let (container_class, remaining_tailwind): (Vec<&str>, Vec<&str>) = tailwind_classes
        .iter()
        .copied()
        .partition(|&c| c == "container");

    // Group utilities by parasite, base, state variants, responsive variants
    let (parasite_utilities, non_parasite): (Vec<&str>, Vec<&str>) = remaining_tailwind
        .iter()
        .copied()
        .partition(|&c| is_parasite_utility(c));

    let (variants, base_utilities): (Vec<&str>, Vec<&str>) =
        non_parasite.iter().copied().partition(|&c| has_variant(c));

    let (responsive_variants, state_variants): (Vec<&str>, Vec<&str>) =
        variants.iter().copied().partition(|&c| {
            let (variant, _) = split_variant(c);
            is_responsive_variant(variant)
        });

    // Sort each category
    let sorted_base = sort_base_utilities(&base_utilities);
    let sorted_state = sort_state_variants(&state_variants);
    let sorted_responsive = sort_responsive_variants(&responsive_variants);

    // Combine all sorted categories in the correct order
    let mut result = Vec::new();
    result.extend(custom_classes);
    result.extend(container_class);
    result.extend(parasite_utilities);
    result.extend(sorted_base);
    result.extend(sorted_state);
    result.extend(sorted_responsive);

    result
}

/// Sort base utilities by their position in Tailwind's CSS output
fn sort_base_utilities<'a>(utilities: &[&'a str]) -> Vec<&'a str> {
    let mut utilities_with_order: Vec<(&str, usize)> = utilities
        .iter()
        .map(|&class| (class, get_property_order(class)))
        .collect();

    // Sort by the determined CSS property order
    utilities_with_order.sort_by_key(|&(_, order)| order);

    // Extract just the class names now that they're sorted
    utilities_with_order
        .into_iter()
        .map(|(class, _)| class)
        .collect()
}

/// Sort state variants
fn sort_state_variants<'a>(variants: &[&'a str]) -> Vec<&'a str> {
    // Group variants by their prefix (hover:, focus:, etc.)
    let mut grouped: HashMap<&str, Vec<&str>> = HashMap::new();

    for &class in variants {
        let (variant, _) = split_variant(class);
        grouped.entry(variant).or_default().push(class);
    }

    // Sort variant groups by priority
    let mut variant_prefixes: Vec<(&str, usize)> = grouped
        .keys()
        .map(|&prefix| (prefix, get_state_variant_order(prefix)))
        .collect();
    variant_prefixes.sort_by_key(|&(_, order)| order);

    // Build final sorted list
    let mut result = Vec::new();
    for (prefix, _) in variant_prefixes {
        if let Some(classes) = grouped.get(prefix) {
            // Sort classes within each variant group
            let mut variant_classes = classes.to_vec();
            variant_classes.sort_by_key(|&class| {
                let (_, base) = split_variant(class);
                get_property_order(base)
            });

            result.extend(variant_classes);
        }
    }

    result
}

/// Sort responsive variants
fn sort_responsive_variants<'a>(variants: &[&'a str]) -> Vec<&'a str> {
    // Group variants by their prefix (sm:, md:, etc.)
    let mut grouped: HashMap<&str, Vec<&str>> = HashMap::new();

    for &class in variants {
        let (variant, _) = split_variant(class);
        grouped.entry(variant).or_default().push(class);
    }

    // Sort variant groups by screen size
    let mut variant_prefixes: Vec<(&str, usize)> = grouped
        .keys()
        .map(|&prefix| (prefix, get_responsive_variant_order(prefix)))
        .collect();
    variant_prefixes.sort_by_key(|&(_, order)| order);

    // Build final sorted list
    let mut result = Vec::new();
    for (prefix, _) in variant_prefixes {
        if let Some(classes) = grouped.get(prefix) {
            // Sort classes within each variant group
            let mut variant_classes = classes.to_vec();
            variant_classes.sort_by_key(|&class| {
                let (_, base) = split_variant(class);
                get_property_order(base)
            });

            result.extend(variant_classes);
        }
    }

    result
}

/// Get the ordering value for a utility class based on PROPERTY_ORDER
fn get_property_order(class: &str) -> usize {
    // Special cases first
    if class == "..." || class == "…" {
        return usize::MAX; // Always at the end
    }

    if class == "container" {
        return *get_property_order_map()
            .get("--tw-container-component")
            .unwrap_or(&1000);
    }

    // First check if the exact class is in the property map
    if let Some(&property) = get_tailwind_property_map().get(class) {
        if let Some(&order) = get_property_order_map().get(property) {
            return order;
        }
    }

    // Get the base class name without arbitrary values
    let base_class = if class.contains('[') {
        class.split('[').next().unwrap_or(class)
    } else if class.contains('(') {
        class.split('(').next().unwrap_or(class)
    } else {
        class
    };

    // Find all matching prefixes, then choose the most specific one
    let mut matches: Vec<(&str, &str)> = get_tailwind_property_map()
        .iter()
        .filter(|&(prefix, _)| base_class == *prefix || base_class.starts_with(prefix))
        .map(|(&prefix, &property)| (prefix, property))
        .collect();

    // Sort by prefix length (descending) to find most specific match
    matches.sort_by_key(|(prefix, _)| -(prefix.len() as isize));

    if let Some((_, property)) = matches.first() {
        if let Some(&order) = get_property_order_map().get(property) {
            return order;
        }
    }

    // For unknown utilities, try to match by general category
    if base_class.starts_with("bg-") {
        return *get_property_order_map()
            .get("background-color")
            .unwrap_or(&1000);
    } else if base_class.starts_with("text-") {
        return *get_property_order_map().get("color").unwrap_or(&1000);
    } else if base_class.starts_with("border") {
        return *get_property_order_map()
            .get("border-width")
            .unwrap_or(&1000);
    } else if base_class.starts_with("pointer-events-") {
        return *get_property_order_map().get("pointer-events").unwrap_or(&1);
    } else if base_class.starts_with("opacity-") {
        return *get_property_order_map().get("opacity").unwrap_or(&241);
    }

    // Default order for unknown classes
    1000
}

/// Get ordering value for state variants
fn get_state_variant_order(variant: &str) -> usize {
    match variant {
        "hover:" => 100,
        "focus:" => 200,
        "focus-visible:" => 210,
        "focus-within:" => 220,
        "active:" => 300,
        "visited:" => 400,
        "checked:" => 500,
        "disabled:" => 600,
        "group-hover:" => 700,
        "group-focus:" => 710,
        "peer-hover:" => 800,
        "peer-focus:" => 810,
        "dark:" => 900,
        "first:" => 1000,
        "last:" => 1100,
        "only:" => 1200,
        "odd:" => 1300,
        "even:" => 1400,
        _ => 1500,
    }
}

/// Get ordering value for responsive variants
fn get_responsive_variant_order(variant: &str) -> usize {
    match variant {
        "sm:" => 100,
        "md:" => 200,
        "lg:" => 300,
        "xl:" => 400,
        "2xl:" => 500,
        _ => 600,
    }
}

/// Check if a class is a "parasite utility"
fn is_parasite_utility(class: &str) -> bool {
    class == "group" || class == "peer"
}

/// Check if a class has a variant prefix
fn has_variant(class: &str) -> bool {
    class.contains(':')
}

/// Split a class into its variant and base parts
fn split_variant(class: &str) -> (&str, &str) {
    if let Some(pos) = class.find(':') {
        (&class[0..=pos], &class[pos + 1..])
    } else {
        ("", class)
    }
}

/// Check if a variant is a responsive variant
fn is_responsive_variant(variant: &str) -> bool {
    matches!(variant, "sm:" | "md:" | "lg:" | "xl:" | "2xl:")
}

/// Check if a class is likely a Tailwind utility class
pub fn is_tailwind_class(class: &str) -> bool {
    // Apply rejection rules first for early exit
    if should_reject_candidate(class) {
        return false;
    }

    // Special cases first
    if class == "..." || class == "…" || class == "container" {
        return true;
    }

    // Common Tailwind classes without hyphens
    if is_standalone_utility(class) {
        return true;
    }

    // Classes with variants
    if class.contains(':') {
        return true;
    }

    // Arbitrary values or arbitrary properties
    if class.starts_with('[') && class.ends_with(']') {
        return true;
    }

    // Arbitrary values with square brackets ([])
    if class.contains('[') && class.contains(']') {
        return true;
    }

    // Custom properties with parentheses (Tailwind v4 syntax)
    if class.contains('(') && class.contains(')') {
        return true;
    }

    // Negative values (like -mt-2, -z-10)
    if class.starts_with('-') && class.len() > 1 {
        // Skip the '-' prefix and check if the rest is a valid pattern
        if let Some(dash_pos) = class[1..].find('-') {
            let prefix = format!("{}-", &class[1..=1 + dash_pos]);
            let value = &class[1 + dash_pos + 1..];
            return is_valid_prefix(&prefix) && is_valid_value_for_prefix(&prefix, value);
        }
    }

    is_tailwind_pattern(class)
}

/// Quick rejection rules based on Tailwind's Oxide engine
fn should_reject_candidate(class: &str) -> bool {
    // Empty class is invalid
    if class.is_empty() {
        return true;
    }

    // Reject candidates that start with a capital letter
    if class.starts_with(|c: char| c.is_ascii_uppercase()) {
        return true;
    }

    // Reject candidates that end with "-" or "_"
    if class.ends_with('-') || class.ends_with('_') {
        return true;
    }

    // Reject candidates that are single camelCase words, e.g.: `useEffect`
    if class.chars().all(|c| c.is_ascii_alphanumeric())
        && class.chars().any(|c| c.is_ascii_uppercase())
    {
        return true;
    }

    // Reject candidates that look like SVG path data, e.g.: `m32.368 m7.5`
    if !class.contains('-') && !class.contains(':') && class.contains('.') {
        return true;
    }

    // Reject candidates that look like version constraints or email addresses, e.g.: `next@latest`, `bob@example.com`
    if class
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '@')
        && class.chars().skip(1).any(|c| c == '@')
    {
        return true;
    }

    // Reject candidates that look like URLs
    if class.starts_with("http://") || class.starts_with("https://") {
        return true;
    }

    // Reject candidates that look like short markdown links, e.g.: `[https://example.com]`
    if class.starts_with("[http://") || class.starts_with("[https://") {
        return true;
    }

    // Reject candidates that look like imports with path aliases, e.g.: `@/components/button`
    if class.len() > 1 && class.starts_with('@') && class.chars().nth(1) == Some('/') {
        return true;
    }

    // Reject candidates that look like paths, e.g.: `app/assets/stylesheets`
    if !class.contains(':') && !class.contains('[') {
        let slash_count = class.chars().filter(|&c| c == '/').count();
        if slash_count > 1 {
            return true;
        }
    }

    false
}

/// Check if a class is a standalone Tailwind utility (no values needed)
fn is_standalone_utility(class: &str) -> bool {
    let utilities = STANDALONE_UTILITIES.get_or_init(|| {
        let utilities = [
            // Layout
            "block",
            "inline-block",
            "inline",
            "flex",
            "inline-flex",
            "table",
            "grid",
            "contents",
            "hidden",
            "flow-root",
            "inline-grid",
            "table-caption",
            "table-cell",
            "table-column",
            "table-column-group",
            "table-footer-group",
            "table-header-group",
            "table-row-group",
            "table-row",
            "list-item",
            // Display
            "sr-only",
            "not-sr-only",
            // Positioning
            "static",
            "fixed",
            "absolute",
            "relative",
            "sticky",
            // Float
            "float-left",
            "float-right",
            "float-none",
            "float-start",
            "float-end",
            // Clear
            "clear-left",
            "clear-right",
            "clear-both",
            "clear-none",
            "clear-start",
            "clear-end",
            // Visibility
            "visible",
            "invisible",
            "collapse",
            // Layout & Box Model
            "box-border",
            "box-content",
            "backface-visible",
            "backface-hidden",
            // Flexbox
            "flex-row",
            "flex-row-reverse",
            "flex-col",
            "flex-col-reverse",
            "flex-wrap",
            "flex-wrap-reverse",
            "flex-nowrap",
            "flex-auto",
            "flex-initial",
            "flex-none",
            "grow",
            "shrink",
            // Alignment
            "items-start",
            "items-end",
            "items-center",
            "items-baseline",
            "items-stretch",
            "justify-start",
            "justify-end",
            "justify-center",
            "justify-between",
            "justify-around",
            "justify-evenly",
            "justify-stretch",
            "justify-baseline",
            "justify-normal",
            "justify-items-start",
            "justify-items-end",
            "justify-items-center",
            "justify-items-stretch",
            "justify-items-normal",
            "justify-self-auto",
            "justify-self-start",
            "justify-self-end",
            "justify-self-center",
            "justify-self-stretch",
            "content-normal",
            "content-center",
            "content-start",
            "content-end",
            "content-between",
            "content-around",
            "content-evenly",
            "content-baseline",
            "content-stretch",
            "self-auto",
            "self-start",
            "self-end",
            "self-center",
            "self-stretch",
            "self-baseline",
            "place-content-center",
            "place-content-start",
            "place-content-end",
            "place-content-between",
            "place-content-around",
            "place-content-evenly",
            "place-content-baseline",
            "place-content-stretch",
            "place-items-start",
            "place-items-end",
            "place-items-center",
            "place-items-baseline",
            "place-items-stretch",
            "place-self-auto",
            "place-self-start",
            "place-self-end",
            "place-self-center",
            "place-self-stretch",
            // Grid
            "grid-flow-row",
            "grid-flow-col",
            "grid-flow-dense",
            "grid-flow-row-dense",
            "grid-flow-col-dense",
            "grid-cols-none",
            "grid-cols-subgrid",
            "grid-rows-none",
            "grid-rows-subgrid",
            "col-auto",
            "row-auto",
            "col-span-full",
            "row-span-full",
            // Spacing
            "space-x-reverse",
            "space-y-reverse",
            // Borders
            "border",
            "border-x",
            "border-y",
            "border-s",
            "border-e",
            "border-t",
            "border-r",
            "border-b",
            "border-l",
            "border-solid",
            "border-dashed",
            "border-dotted",
            "border-double",
            "border-hidden",
            "border-none",
            "divide-solid",
            "divide-dashed",
            "divide-dotted",
            "divide-double",
            "divide-hidden",
            "divide-none",
            "divide-x-reverse",
            "divide-y-reverse",
            "border-collapse",
            "border-separate",
            "rounded",
            "rounded-none",
            "rounded-full",
            // Outline
            "outline-none",
            "outline-hidden",
            "outline-solid",
            "outline-dashed",
            "outline-dotted",
            "outline-double",
            // Effects
            "shadow",
            "shadow-none",
            // Transitions
            "transition",
            "transition-all",
            "transition-colors",
            "transition-opacity",
            "transition-shadow",
            "transition-transform",
            "transition-none",
            "transition-normal",
            "transition-discrete",
            // Typography
            "italic",
            "not-italic",
            "font-thin",
            "font-extralight",
            "font-light",
            "font-normal",
            "font-medium",
            "font-semibold",
            "font-bold",
            "font-extrabold",
            "font-black",
            "font-sans",
            "font-serif",
            "font-mono",
            "normal-nums",
            "ordinal",
            "slashed-zero",
            "lining-nums",
            "oldstyle-nums",
            "proportional-nums",
            "tabular-nums",
            "diagonal-fractions",
            "stacked-fractions",
            "underline",
            "overline",
            "line-through",
            "no-underline",
            "decoration-solid",
            "decoration-double",
            "decoration-dotted",
            "decoration-dashed",
            "decoration-wavy",
            "decoration-from-font",
            "decoration-auto",
            "uppercase",
            "lowercase",
            "capitalize",
            "normal-case",
            "truncate",
            "text-ellipsis",
            "text-clip",
            "text-left",
            "text-center",
            "text-right",
            "text-justify",
            "text-start",
            "text-end",
            "antialiased",
            "subpixel-antialiased",
            "text-wrap",
            "text-nowrap",
            "text-balance",
            "text-pretty",
            "break-normal",
            "break-words",
            "break-all",
            "break-keep",
            "hyphens-none",
            "hyphens-manual",
            "hyphens-auto",
            "whitespace-normal",
            "whitespace-nowrap",
            "whitespace-pre",
            "whitespace-pre-line",
            "whitespace-pre-wrap",
            "whitespace-break-spaces",
            // Inputs and Forms
            "appearance-none",
            "appearance-auto",
            "cursor-auto",
            "cursor-default",
            "cursor-pointer",
            "cursor-wait",
            "cursor-text",
            "cursor-move",
            "cursor-help",
            "cursor-not-allowed",
            "cursor-none",
            "cursor-context-menu",
            "cursor-progress",
            "cursor-cell",
            "cursor-crosshair",
            "cursor-vertical-text",
            "cursor-alias",
            "cursor-copy",
            "cursor-no-drop",
            "cursor-grab",
            "cursor-grabbing",
            "cursor-all-scroll",
            "cursor-col-resize",
            "cursor-row-resize",
            "cursor-n-resize",
            "cursor-e-resize",
            "cursor-s-resize",
            "cursor-w-resize",
            "cursor-ne-resize",
            "cursor-nw-resize",
            "cursor-se-resize",
            "cursor-sw-resize",
            "cursor-ew-resize",
            "cursor-ns-resize",
            "cursor-nesw-resize",
            "cursor-nwse-resize",
            "cursor-zoom-in",
            "cursor-zoom-out",
            "select-none",
            "select-text",
            "select-all",
            "select-auto",
            "resize-none",
            "resize",
            "resize-y",
            "resize-x",
            // Object fit & position
            "object-contain",
            "object-cover",
            "object-fill",
            "object-none",
            "object-scale-down",
            "object-bottom",
            "object-center",
            "object-left",
            "object-left-bottom",
            "object-left-top",
            "object-right",
            "object-right-bottom",
            "object-right-top",
            "object-top",
            // SVG and Graphics
            "fill-current",
            "stroke-current",
            // Touch Actions
            "touch-auto",
            "touch-none",
            "touch-pan-x",
            "touch-pan-left",
            "touch-pan-right",
            "touch-pan-y",
            "touch-pan-up",
            "touch-pan-down",
            "touch-pinch-zoom",
            "touch-manipulation",
            // Tables
            "table-auto",
            "table-fixed",
            "caption-top",
            "caption-bottom",
            // Scroll
            "scroll-auto",
            "scroll-smooth",
            "snap-start",
            "snap-end",
            "snap-center",
            "snap-align-none",
            "snap-normal",
            "snap-always",
            "snap-none",
            "snap-x",
            "snap-y",
            "snap-both",
            "snap-mandatory",
            "snap-proximity",
            // 3D Transforms
            "transform-3d",
            "transform-flat",
            "transform-none",
            "transform-gpu",
            "transform-cpu",
            // Force Color Adjust
            "forced-color-adjust-auto",
            "forced-color-adjust-none",
            // Color Scheme
            "scheme-normal",
            "scheme-dark",
            "scheme-light",
            "scheme-light-dark",
            "scheme-only-dark",
            "scheme-only-light",
            // Isolation
            "isolate",
            "isolation-auto",
            // Will Change
            "will-change-auto",
            "will-change-scroll",
            "will-change-contents",
            "will-change-transform",
            // Fieldset Sizing
            "field-sizing-fixed",
            "field-sizing-content",
            // Box Decoration
            "box-decoration-slice",
            "box-decoration-clone",
            // Other useful ones
            "group",
            "peer",
            "container",
            "animate-none",
            "filter-none",
            "backdrop-filter-none",
        ];
        HashSet::from_iter(utilities.iter().copied())
    });

    utilities.contains(class)
}

/// Check if a string follows a valid Tailwind utility pattern
fn is_tailwind_pattern(class: &str) -> bool {
    // Extract the prefix and value parts (if any)
    if let Some(idx) = class.find('-') {
        let (prefix, value) = class.split_at(idx + 1); // Include the '-' with prefix

        // Check if the prefix is valid
        if !is_valid_prefix(prefix) {
            return false;
        }

        // Check if the value is valid for this prefix
        return is_valid_value_for_prefix(prefix, value);
    }

    false
}

fn is_valid_prefix(prefix: &str) -> bool {
    let prefixes = VALID_PREFIXES.get_or_init(|| {
        let prefixes = [
            // Position and layout
            "inset-",
            "-inset-",
            "inset-x-",
            "-inset-x-",
            "inset-y-",
            "-inset-y-",
            "inset-s-",
            "-inset-s-",
            "inset-e-",
            "-inset-e-",
            "top-",
            "-top-",
            "right-",
            "-right-",
            "bottom-",
            "-bottom-",
            "left-",
            "-left-",
            "start-",
            "-start-",
            "end-",
            "-end-",
            "z-",
            "-z-",
            "order-",
            "-order-",
            "col-",
            "-col-",
            "row-",
            "-row-",
            "col-span-",
            "row-span-",
            "col-start-",
            "-col-start-",
            "col-end-",
            "-col-end-",
            "row-start-",
            "-row-start-",
            "row-end-",
            "-row-end-",
            "float-",
            "clear-",
            // Spacing
            "m-",
            "-m-",
            "mx-",
            "-mx-",
            "my-",
            "-my-",
            "mt-",
            "-mt-",
            "mr-",
            "-mr-",
            "mb-",
            "-mb-",
            "ml-",
            "-ml-",
            "ms-",
            "-ms-",
            "me-",
            "-me-",
            "p-",
            "-p-",
            "px-",
            "-px-",
            "py-",
            "-py-",
            "pt-",
            "-pt-",
            "pr-",
            "-pr-",
            "pb-",
            "-pb-",
            "pl-",
            "-pl-",
            "ps-",
            "-ps-",
            "pe-",
            "-pe-",
            "space-x-",
            "-space-x-",
            "space-y-",
            "-space-y-",
            "indent-",
            "-indent-",
            // Sizing
            "w-",
            "min-w-",
            "max-w-",
            "h-",
            "min-h-",
            "max-h-",
            "size-",
            // Typography
            "font-",
            "text-",
            "tracking-",
            "leading-",
            "list-",
            "list-image-",
            "align-",
            "line-clamp-",
            "decoration-",
            "underline-offset-",
            "-underline-offset-",
            // Backgrounds
            "bg-",
            "from-",
            "via-",
            "to-",
            // Border
            "border-",
            "border-x-",
            "border-y-",
            "border-t-",
            "border-r-",
            "border-b-",
            "border-l-",
            "border-s-",
            "border-e-",
            "divide-",
            "divide-x-",
            "divide-y-",
            "outline-",
            "outline-offset-",
            "-outline-offset-",
            "ring-",
            "ring-offset-",
            // Border radius
            "rounded-",
            "rounded-t-",
            "rounded-r-",
            "rounded-b-",
            "rounded-l-",
            "rounded-tl-",
            "rounded-tr-",
            "rounded-br-",
            "rounded-bl-",
            "rounded-ss-",
            "rounded-se-",
            "rounded-ee-",
            "rounded-es-",
            "rounded-s-",
            "rounded-e-",
            // Flexbox and Grid
            "flex-",
            "basis-",
            "grow-",
            "shrink-",
            "grid-",
            "grid-cols-",
            "grid-rows-",
            "grid-flow-",
            "auto-cols-",
            "auto-rows-",
            "gap-",
            "gap-x-",
            "gap-y-",
            "justify-",
            "content-",
            "items-",
            "self-",
            "place-",
            // Effects
            "shadow-",
            "inset-shadow-",
            "opacity-",
            "mix-blend-",
            "bg-blend-",
            // Filters
            "blur-",
            "brightness-",
            "contrast-",
            "drop-shadow-",
            "grayscale-",
            "hue-rotate-",
            "-hue-rotate-",
            "invert-",
            "saturate-",
            "sepia-",
            "filter-",
            "backdrop-blur-",
            "backdrop-brightness-",
            "backdrop-contrast-",
            "backdrop-grayscale-",
            "backdrop-hue-rotate-",
            "-backdrop-hue-rotate-",
            "backdrop-invert-",
            "backdrop-opacity-",
            "backdrop-saturate-",
            "backdrop-sepia-",
            "backdrop-filter-",
            // SVG
            "fill-",
            "stroke-",
            "stroke-width-",
            // Transitions and animations
            "transition-",
            "duration-",
            "ease-",
            "delay-",
            "animate-",
            "transition-behavior-",
            // Transform
            "transform-",
            "origin-",
            "scale-",
            "-scale-",
            "scale-x-",
            "-scale-x-",
            "scale-y-",
            "-scale-y-",
            "scale-z-",
            "-scale-z-",
            "rotate-",
            "-rotate-",
            "rotate-x-",
            "-rotate-x-",
            "rotate-y-",
            "-rotate-y-",
            "rotate-z-",
            "-rotate-z-",
            "translate-",
            "-translate-",
            "translate-x-",
            "-translate-x-",
            "translate-y-",
            "-translate-y-",
            "translate-z-",
            "-translate-z-",
            "skew-",
            "-skew-",
            "skew-x-",
            "-skew-x-",
            "skew-y-",
            "-skew-y-",
            "perspective-",
            "perspective-origin-",
            // Scroll utilities
            "scroll-m-",
            "scroll-mx-",
            "scroll-my-",
            "scroll-mt-",
            "scroll-mr-",
            "scroll-mb-",
            "scroll-ml-",
            "scroll-ms-",
            "scroll-me-",
            "-scroll-m-",
            "-scroll-mx-",
            "-scroll-my-",
            "-scroll-mt-",
            "-scroll-mr-",
            "-scroll-mb-",
            "-scroll-ml-",
            "-scroll-ms-",
            "-scroll-me-",
            "scroll-p-",
            "scroll-px-",
            "scroll-py-",
            "scroll-pt-",
            "scroll-pr-",
            "scroll-pb-",
            "scroll-pl-",
            "scroll-ps-",
            "scroll-pe-",
            "-scroll-p-",
            "-scroll-px-",
            "-scroll-py-",
            "-scroll-pt-",
            "-scroll-pr-",
            "-scroll-pb-",
            "-scroll-pl-",
            "-scroll-ps-",
            "-scroll-pe-",
            "scroll-snap-",
            "snap-",
            // Table utilities
            "table-",
            "caption-",
            "border-collapse-",
            "border-spacing-",
            "border-spacing-x-",
            "border-spacing-y-",
            // Misc utilities
            "accent-",
            "caret-",
            "cursor-",
            "object-",
            "object-position-",
            "will-change-",
            "aspect-",
            "columns-",
            "container-type-",
            "field-sizing-",
            "box-decoration-",
            "box-sizing-",
            "forced-color-adjust-",
            // Color scheme
            "scheme-",
            "color-scheme-",
            // Other style utilities
            "font-variant-",
            "font-stretch-",
            "font-smoothing-",
            "backface-",
            // Break utilities
            "break-before-",
            "break-inside-",
            "break-after-",
            // New in Tailwind v4 or recently added
            "text-wrap-",
            "text-decoration-",
            "text-underline-",
            "-webkit-font-smoothing-",
            // Prefixes ending without hyphen (standalone utilities)
            "pointer-events-",
            "visibility-",
            "touch-",
            "appearance-",
            "overflow-",
            "overscroll-",
            "whitespace-",
            "break-",
            "hyphens-",
            "resize-",
            "user-select-",
        ];
        HashSet::from_iter(prefixes.iter().copied())
    });

    prefixes.contains(prefix)
}

fn is_valid_value_for_prefix(prefix: &str, value: &str) -> bool {
    // Special case for color utilities
    if [
        "bg-",
        "text-",
        "border-",
        "border-t-",
        "border-r-",
        "border-b-",
        "border-l-",
        "border-x-",
        "border-y-",
        "border-s-",
        "border-e-",
        "ring-",
        "divide-",
        "fill-",
        "stroke-",
        "outline-",
        "accent-",
        "caret-",
        "shadow-",
        "inset-shadow-",
        "from-",
        "to-",
        "via-",
        "decoration-",
    ]
    .contains(&prefix)
        && is_valid_color_value(value)
    {
        return true;
    }

    // Check for numeric values (including those with decimals)
    if value.parse::<f64>().is_ok() {
        // Position and layout utilities that take numeric values
        if [
            "inset-",
            "top-",
            "right-",
            "bottom-",
            "left-",
            "inset-x-",
            "inset-y-",
            "inset-s-",
            "inset-e-",
            "start-",
            "end-",
            "z-",
            "order-",
            "-inset-",
            "-top-",
            "-right-",
            "-bottom-",
            "-left-",
            "-inset-x-",
            "-inset-y-",
            "-inset-s-",
            "-inset-e-",
            "-start-",
            "-end-",
            "-z-",
            "-order-",
        ]
        .contains(&prefix)
        {
            return true;
        }

        // Spacing utilities that take numeric values
        if [
            "m-",
            "mx-",
            "my-",
            "mt-",
            "mr-",
            "mb-",
            "ml-",
            "ms-",
            "me-",
            "-m-",
            "-mx-",
            "-my-",
            "-mt-",
            "-mr-",
            "-mb-",
            "-ml-",
            "-ms-",
            "-me-",
            "p-",
            "px-",
            "py-",
            "pt-",
            "pr-",
            "pb-",
            "pl-",
            "ps-",
            "pe-",
            "gap-",
            "gap-x-",
            "gap-y-",
            "space-x-",
            "space-y-",
            "-space-x-",
            "-space-y-",
            "indent-",
            "-indent-",
        ]
        .contains(&prefix)
        {
            return true;
        }

        // Scroll utilities that take numeric values
        if [
            "scroll-m-",
            "scroll-mx-",
            "scroll-my-",
            "scroll-mt-",
            "scroll-mr-",
            "scroll-mb-",
            "scroll-ml-",
            "scroll-ms-",
            "scroll-me-",
            "-scroll-m-",
            "-scroll-mx-",
            "-scroll-my-",
            "-scroll-mt-",
            "-scroll-mr-",
            "-scroll-mb-",
            "-scroll-ml-",
            "-scroll-ms-",
            "-scroll-me-",
            "scroll-p-",
            "scroll-px-",
            "scroll-py-",
            "scroll-pt-",
            "scroll-pr-",
            "scroll-pb-",
            "scroll-pl-",
            "scroll-ps-",
            "scroll-pe-",
            "-scroll-p-",
            "-scroll-px-",
            "-scroll-py-",
            "-scroll-pt-",
            "-scroll-pr-",
            "-scroll-pb-",
            "-scroll-pl-",
            "-scroll-ps-",
            "-scroll-pe-",
        ]
        .contains(&prefix)
        {
            return true;
        }

        // Sizing utilities that take numeric values
        if ["w-", "h-", "min-w-", "min-h-", "max-w-", "max-h-", "size-"].contains(&prefix) {
            return true;
        }

        // Border utilities that take numeric values
        if [
            "border-",
            "border-x-",
            "border-y-",
            "border-s-",
            "border-e-",
            "border-t-",
            "border-r-",
            "border-b-",
            "border-l-",
            "rounded-",
            "rounded-t-",
            "rounded-r-",
            "rounded-b-",
            "rounded-l-",
            "rounded-tl-",
            "rounded-tr-",
            "rounded-br-",
            "rounded-bl-",
            "rounded-ss-",
            "rounded-se-",
            "rounded-ee-",
            "rounded-es-",
            "outline-",
            "outline-offset-",
            "-outline-offset-",
            "ring-",
            "ring-offset-",
            "divide-x-",
            "divide-y-",
        ]
        .contains(&prefix)
        {
            return true;
        }

        // Transformation utilities
        if [
            "scale-",
            "-scale-",
            "scale-x-",
            "-scale-x-",
            "scale-y-",
            "-scale-y-",
            "scale-z-",
            "-scale-z-",
            "rotate-",
            "-rotate-",
            "rotate-x-",
            "-rotate-x-",
            "rotate-y-",
            "-rotate-y-",
            "rotate-z-",
            "-rotate-z-",
            "translate-",
            "-translate-",
            "translate-x-",
            "-translate-x-",
            "translate-y-",
            "-translate-y-",
            "translate-z-",
            "-translate-z-",
            "skew-",
            "-skew-",
            "skew-x-",
            "-skew-x-",
            "skew-y-",
            "-skew-y-",
        ]
        .contains(&prefix)
        {
            return true;
        }

        // Other utilities that take numeric values
        if [
            "opacity-",
            "backdrop-opacity-",
            "brightness-",
            "backdrop-brightness-",
            "contrast-",
            "backdrop-contrast-",
            "saturate-",
            "backdrop-saturate-",
            "grayscale-",
            "backdrop-grayscale-",
            "invert-",
            "backdrop-invert-",
            "sepia-",
            "backdrop-sepia-",
            "hue-rotate-",
            "-hue-rotate-",
            "backdrop-hue-rotate-",
            "-backdrop-hue-rotate-",
            "blur-",
            "backdrop-blur-",
            "drop-shadow-",
            "duration-",
            "delay-",
            "flex-",
            "grow-",
            "shrink-",
            "basis-",
            "cols-",
            "grid-cols-",
            "rows-",
            "grid-rows-",
            "stroke-",
            "underline-offset-",
            "-underline-offset-",
            "line-clamp-",
            "text-",
            "columns-",
            "border-spacing-",
            "border-spacing-x-",
            "border-spacing-y-",
        ]
        .contains(&prefix)
        {
            return true;
        }

        return true;
    }

    // Check for fraction values (like 1/2, 2/3)
    if value.contains('/') {
        let parts: Vec<&str> = value.split('/').collect();
        if parts.len() == 2 && parts[0].parse::<f64>().is_ok() && parts[1].parse::<f64>().is_ok() {
            return true;
        }
    }

    // Arbitrary values and custom properties
    if (value.starts_with('[') && value.ends_with(']'))
        || (value.starts_with('(') && value.ends_with(')'))
    {
        return true;
    }

    // Common values for many utilities
    let common_values = [
        "auto", "full", "screen", "px", "none", "dvh", "dvw", "lvh", "lvw", "svh", "svw", "min",
        "max", "fit",
    ];
    if common_values.contains(&value) {
        return true;
    }

    // Container breakpoints and sizing
    if ["w-", "max-w-", "min-w-", "columns-", "basis-"].contains(&prefix)
        && [
            "3xs", "2xs", "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl",
            "prose",
        ]
        .contains(&value)
    {
        return true;
    }

    // Prefix-specific non-numeric values
    match prefix {
        "rounded-" | "rounded-t-" | "rounded-r-" | "rounded-b-" | "rounded-l-" | "rounded-tl-"
        | "rounded-tr-" | "rounded-br-" | "rounded-bl-" | "rounded-ss-" | "rounded-se-"
        | "rounded-ee-" | "rounded-es-" => {
            ["sm", "md", "lg", "xl", "2xl", "3xl", "full"].contains(&value)
        }
        "shadow-" | "inset-shadow-" => ["sm", "md", "lg", "xl", "2xl", "inner"].contains(&value),
        "drop-shadow-" => ["xs", "sm", "md", "lg", "xl", "2xl"].contains(&value),
        "blur-" | "backdrop-blur-" => ["xs", "sm", "md", "lg", "xl", "2xl", "3xl"].contains(&value),
        "font-" => [
            "thin",
            "extralight",
            "light",
            "normal",
            "medium",
            "semibold",
            "bold",
            "extrabold",
            "black",
            "sans",
            "serif",
            "mono",
        ]
        .contains(&value),
        "text-" => {
            [
                "xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl",
                "9xl",
            ]
            .contains(&value)
                || is_valid_color_value(value)
        }
        "tracking-" => ["tighter", "tight", "normal", "wide", "wider", "widest"].contains(&value),
        "leading-" => ["none", "tight", "snug", "normal", "relaxed", "loose"].contains(&value),
        "ease-" => ["linear", "in", "out", "in-out", "initial"].contains(&value),
        "object-" => [
            "contain",
            "cover",
            "fill",
            "none",
            "scale-down",
            "top",
            "bottom",
            "center",
            "left",
            "left-bottom",
            "left-top",
            "right",
            "right-bottom",
            "right-top",
        ]
        .contains(&value),
        "aspect-" => ["auto", "square", "video"].contains(&value),
        "line-clamp-" => ["none"].contains(&value),
        "z-" | "-z-" => ["auto"].contains(&value),
        "perspective-" => {
            ["dramatic", "near", "normal", "midrange", "distant", "none"].contains(&value)
        }
        "perspective-origin-" => [
            "center",
            "top",
            "top-right",
            "right",
            "bottom-right",
            "bottom",
            "bottom-left",
            "left",
            "top-left",
        ]
        .contains(&value),
        "transition-" => {
            ["none", "all", "colors", "opacity", "shadow", "transform"].contains(&value)
        }
        "transition-behavior-" => ["normal", "discrete"].contains(&value),
        "scheme-" => [
            "normal",
            "dark",
            "light",
            "light-dark",
            "only-dark",
            "only-light",
        ]
        .contains(&value),
        "field-sizing-" => ["fixed", "content"].contains(&value),
        "box-decoration-" => ["slice", "clone"].contains(&value),
        // Wildcard matching for common patterns
        _ if prefix.starts_with("outline-")
            || prefix.starts_with("shadow-")
            || prefix.starts_with("ring-")
            || prefix.starts_with("stroke-") =>
        {
            true
        }
        _ if prefix.starts_with("hue-rotate-") || prefix.starts_with("backdrop-hue-rotate-") => {
            value.parse::<f64>().is_ok() || value.ends_with("deg")
        }
        _ if prefix.starts_with("col-") || prefix.starts_with("row-") => {
            value.parse::<f64>().is_ok() || ["span", "auto", "start", "end"].contains(&value)
        }
        _ => false,
    }
}

/// Check if a value is a valid Tailwind color value
fn is_valid_color_value(value: &str) -> bool {
    // Basic color names
    if ["white", "black", "transparent", "current", "inherit"].contains(&value) {
        return true;
    }

    // Handle arbitrary color values like `bg-[#f00]` or `text-[rgb(255,0,0)]`
    if (value.starts_with('[') && value.ends_with(']'))
        || (value.starts_with('(') && value.ends_with(')'))
    {
        return true;
    }

    // Check for color-number pattern (red-500, blue-200, etc.)
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() == 2 {
        let color = parts[0];
        let number = parts[1];

        // Check if the color name is a valid Tailwind color
        if [
            "slate", "gray", "zinc", "neutral", "stone", "red", "orange", "amber", "yellow",
            "lime", "green", "emerald", "teal", "cyan", "sky", "blue", "indigo", "violet",
            "purple", "fuchsia", "pink", "rose",
        ]
        .contains(&color)
        {
            // Check if "number" is a valid number
            return number.parse::<usize>().is_ok();
        }
    }

    false
}

/// Remove duplicate classes, keeping only the last occurrence of each class
/// Only removes duplicates of Tailwind classes, custom classes are preserved
fn remove_duplicates<'a>(classes: &[&'a str]) -> Vec<&'a str> {
    let mut seen_tailwind = HashSet::new();
    let mut result = Vec::new();

    // Process from end to beginning to find last occurrence first
    for &class in classes.iter().rev() {
        if is_tailwind_class(class) {
            // For Tailwind classes, only add if not seen before
            if seen_tailwind.insert(class) {
                // Insert at beginning to maintain original order
                result.insert(0, class);
            }
        } else {
            // For custom classes, always add
            result.insert(0, class);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blog_post_example() {
        let input = "text-white px-4 sm:px-8 py-2 sm:py-3 bg-sky-700 hover:bg-sky-800";
        let expected = "bg-sky-700 px-4 py-2 text-white hover:bg-sky-800 sm:px-8 sm:py-3";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_position_utilities() {
        let input = "py-2 px-4 my-2 mx-4 top-0 right-0 bottom-0 left-0 inset-0";
        let expected = "inset-0 top-0 right-0 bottom-0 left-0 mx-4 my-2 px-4 py-2";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_preserve_duplicates() {
        // Override the global setting
        set_remove_duplicates(false);

        let input = "p-4 m-2 p-4 bg-blue-500 m-2";
        let expected = "m-2 m-2 bg-blue-500 p-4 p-4";
        assert_eq!(sort_classes(input), expected);

        set_remove_duplicates(true);
    }

    #[test]
    fn test_container_with_utilities() {
        let input = "p-4 container mx-auto md:w-1/2 bg-white";
        let expected = "container mx-auto bg-white p-4 md:w-1/2";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_padding_margin_order() {
        let input = "pl-4 p-2 m-4 mb-8 mx-2 py-3";
        let expected = "m-4 mx-2 mb-8 p-2 py-3 pl-4";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_disabled() {
        let input = "disabled:opacity-50 disabled:pointer-events-none";
        let expected = "disabled:pointer-events-none disabled:opacity-50";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_without_disabled() {
        let input = "opacity-50 pointer-events-none";
        let expected = "pointer-events-none opacity-50";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_ring_utilities() {
        let input = "inline-flex items-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-1 pt-2 pb-4 px-4";
        let expected = "inline-flex items-center rounded-md px-4 pt-2 pb-4 font-medium ring-offset-1 transition-colors focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-2 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_tailwind_custom_properties() {
        let input = "bg-(--main-color) text-(--text-color) border-(--border-color) p-4 m-2";
        let expected = "m-2 border-(--border-color) bg-(--main-color) p-4 text-(--text-color)";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_custom_classes() {
        let input = "custom-class p-4 another-custom bg-blue-500 text-white my-class";
        let expected = "custom-class another-custom my-class bg-blue-500 p-4 text-white";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_ellipsis() {
        let input = "p-4 ... bg-blue-500";
        let expected = "bg-blue-500 p-4 ...";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_multiple_responsive_variants() {
        let input = "flex md:grid lg:flex xl:grid 2xl:flex sm:block";
        let expected = "flex sm:block md:grid lg:flex xl:grid 2xl:flex";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_parasite_utilities() {
        let input = "p-4 group peer flex";
        let expected = "group peer flex p-4";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_complex_variants() {
        let input = "hover:opacity-75 focus:outline-none opacity-50 hover:scale-150 scale-125 sm:flex md:block lg:hidden sm:p-4 p-2 md:m-6";
        let expected = "scale-125 p-2 opacity-50 hover:scale-150 hover:opacity-75 focus:outline-none sm:flex sm:p-4 md:m-6 md:block lg:hidden";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_arbitrary_values() {
        let input = "bg-[#ff0000] text-[16px] p-[10px] m-[5px]";
        let expected = "m-[5px] bg-[#ff0000] p-[10px] text-[16px]";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_remove_duplicates() {
        set_remove_duplicates(true);
        let input = "p-4 m-2 p-4 bg-blue-500 m-2";
        let expected = "m-2 bg-blue-500 p-4";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_disabled_variant_sorting() {
        let input = "disabled:opacity-50 disabled:pointer-events-none";
        let expected = "disabled:pointer-events-none disabled:opacity-50";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_variant_consistent_with_base() {
        let input1 = "opacity-50 pointer-events-none";
        let expected1 = "pointer-events-none opacity-50";

        let input2 = "disabled:opacity-50 disabled:pointer-events-none";
        let expected2 = "disabled:pointer-events-none disabled:opacity-50";

        assert_eq!(sort_classes(input1), expected1);
        assert_eq!(sort_classes(input2), expected2);
    }

    #[test]
    fn test_remove_duplicates_tailwind_only() {
        set_remove_duplicates(true);

        // Custom classes "my-class" and "custom" should be preserved, even if duplicated
        let input = "p-4 my-class p-4 bg-blue-500 custom my-class custom";

        // Expected: duplicated p-4 removed, but duplicated custom classes remain
        let expected = "my-class custom my-class custom bg-blue-500 p-4";

        assert_eq!(sort_classes(input), expected);
    }
}
