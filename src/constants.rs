pub const UNSUPPORTED_EXTENSIONS: &'static [&'static str] = &[
    "eot", "tiff", "tff", "woff", "woff2", "otf", // Fonts
    "jpg", "png", "gif", "jfif", "webp", "bmp", "ico", "svg", // Images
    "mp4", "mov", "avi", "flv", // Videos
    "mp3", "wmv", "wav", "aac", "flac", "ogg", "wma", "zip", // Audio
    "pyc", "pyd", "tar", "gz", "rar", "7z", "iso", "bin", "exe", "dll", "msi", "dmg", "pkg", "deb",
    "rpm", "apk", "jar", "war", "ear", "npz", "npy", "lib", "dat", // Archives and executables
    "xz", "bz2", "lzma", "vdi", "vmdk", "qcow2", // Archives/disk images
    "class", "o", "pyo", "egg", "pex", // Build artifacts
    "so", "a", "dylib", "deb", "rpm", "snap", // Platform executables
    "mo", "pdf", // Misc
    "lock", "sum", //  Lock files (May not be human-readable)
];
pub const DEFAULT_EXCLUSIONS: &'static [&'static str] = &[
    "*LICENSE*", ".gitignore", ".git/", // Version control
    "build/", "dist/", "out/", "target/", "__pycache__/",  // Build/dependency
    ".vscode/", ".idea/", "*.sublime-project", "*.code-workspace",  // IDE
    "venv/", ".venv/", "env/", "conda-env/", "node_modules/",  // Environments
    "*.env", "*.secret", "*.key", "secrets.yml", "*_rsa", "*.pem"  // Secrets
];