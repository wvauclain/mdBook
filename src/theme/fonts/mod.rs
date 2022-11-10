pub static CSS: &[u8] = include_bytes!("fonts.css");
// An array of (file_name, file_contents) pairs
pub static LICENSES: [(&str, &[u8]); 2] = [
    (
        "fonts/LIBERTINUS-LICENSE.txt",
        include_bytes!("LIBERTINUS-LICENSE.txt"),
    ),
    (
        "fonts/IOSEVKA-LICENSE.txt",
        include_bytes!("IOSEVKA-LICENSE.txt"),
    ),
];
// An array of (file_name, file_contents) pairs
pub static LIBERTINUS: [(&str, &[u8]); 14] = [
    (
        "fonts/LibertinusKeyboard-Regular.woff2",
        include_bytes!("LibertinusKeyboard-Regular.woff2"),
    ),
    (
        "fonts/LibertinusMath-Regular.woff2",
        include_bytes!("LibertinusMath-Regular.woff2"),
    ),
    (
        "fonts/LibertinusMono-Regular.woff2",
        include_bytes!("LibertinusMono-Regular.woff2"),
    ),
    (
        "fonts/LibertinusSans-Bold.woff2",
        include_bytes!("LibertinusSans-Bold.woff2"),
    ),
    (
        "fonts/LibertinusSans-Italic.woff2",
        include_bytes!("LibertinusSans-Italic.woff2"),
    ),
    (
        "fonts/LibertinusSans-Regular.woff2",
        include_bytes!("LibertinusSans-Regular.woff2"),
    ),
    (
        "fonts/LibertinusSerif-Bold.woff2",
        include_bytes!("LibertinusSerif-Bold.woff2"),
    ),
    (
        "fonts/LibertinusSerif-BoldItalic.woff2",
        include_bytes!("LibertinusSerif-BoldItalic.woff2"),
    ),
    (
        "fonts/LibertinusSerif-Italic.woff2",
        include_bytes!("LibertinusSerif-Italic.woff2"),
    ),
    (
        "fonts/LibertinusSerif-Regular.woff2",
        include_bytes!("LibertinusSerif-Regular.woff2"),
    ),
    (
        "fonts/LibertinusSerif-Semibold.woff2",
        include_bytes!("LibertinusSerif-Semibold.woff2"),
    ),
    (
        "fonts/LibertinusSerif-SemiboldItalic.woff2",
        include_bytes!("LibertinusSerif-SemiboldItalic.woff2"),
    ),
    (
        "fonts/LibertinusSerifDisplay-Regular.woff2",
        include_bytes!("LibertinusSerifDisplay-Regular.woff2"),
    ),
    (
        "fonts/LibertinusSerifInitials-Regular.woff2",
        include_bytes!("LibertinusSerifInitials-Regular.woff2"),
    ),
];


// An array of (file_name, file_contents) pairs
pub static IOSEVKA: [(&str, &[u8]); 4] = [
    (
        "fonts/iosevka-fixed-bold.woff2",
        include_bytes!("iosevka-fixed-bold.woff2"),
    ),
    (
        "fonts/iosevka-fixed-bolditalic.woff2",
        include_bytes!("iosevka-fixed-bolditalic.woff2"),
    ),
    (
        "fonts/iosevka-fixed-italic.woff2",
        include_bytes!("iosevka-fixed-italic.woff2"),
    ),
    (
        "fonts/iosevka-fixed-regular.woff2",
        include_bytes!("iosevka-fixed-regular.woff2"),
    ),
];
