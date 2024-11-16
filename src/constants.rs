pub const UNSUPPORTED_EXTENSIONS: &'static [&'static str] = &[
    "eot", "tiff", "tff", "woff", "woff2", "otf", // Fonts
    "jpg", "png", "gif", "jfif", "webp", "bmp", "ico", "svg", // Images
    "mp4", "mov", "avi", "flv", // Videos
    "mp3", "wmv", "wav", "aac", "flac", "ogg", "wma", "zip", // Audio
    "pyc", "pyd", "tar", "gz", "rar", "7z", "iso", "bin", "exe", "dll", "msi", "dmg", "pkg", "deb",
    "rpm", "apk", "jar", "war", "ear", "npz", "npy", "lib", "dat", // Archives and executables
    "mo", "pdf", // Misc
    "lock", //  Lock files (May not be human-readable)
];
pub const DEFAULT_EXCLUSIONS: &'static [&'static str] = &[
    "*LICENSE*",
    ".gitignore",
    "node_modules/",
    ".git/",
];
