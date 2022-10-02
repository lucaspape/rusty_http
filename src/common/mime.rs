use std::path::Path;

#[derive(Clone)]
pub enum MimeType {
    OctetStream,
    Aac,
    AbiWord,
    FreeArc,
    Avif,
    MsVideo,
    AmazonEbook,
    Bitmap,
    Bzip,
    Bzip2,
    Cdf,
    Csh,
    Css,
    Csv,
    MSWord,
    MSWordOpenXML,
    MSFontObject,
    Epub,
    Gzip,
    Gif,
    Html,
    Icon,
    Calendar,
    JavaArchive,
    Jpeg,
    JavaScript,
    Json,
    LDJson,
    Midi,
    Mpeg,
    MP4,
    MpegVideo,
    AppleInstaller,
    OpenDocumentPresentation,
    OpenDocumentSpreadsheet,
    OpenDocumentText,
    OggAudio,
    OggVideo,
    OggApplication,
    Opus,
    Png,
    Otf,
    Pdf,
    Php,
    MSPowerPoint,
    MSPowerPointOpenXML,
    Rar,
    Rtf,
    Sh,
    Svg,
    Tar,
    Tiff,
    Mp2t,
    Ttf,
    Plain,
    MSVisio,
    Wav,
    WebmAudio,
    WebmVideo,
    Webp,
    Woff,
    Woff2,
    Xhtml,
    MSExcel,
    MSExcelOpenXML,
    Xml,
    Xul,
    Zip,
    ThreeGp,
    ThreeGp2,
    SevenZ,
    WWWFormURLEncoded
}

impl MimeType {
    pub fn get(self) -> String {
        return match self {
            MimeType::OctetStream => String::from(APPLICATION_OCTET_STREAM),
            MimeType::Aac => String::from(AUDIO_AAC),
            MimeType::AbiWord => String::from(APPLICATION_X_ABIWORD),
            MimeType::FreeArc => String::from(APPLICATION_X_FREEARC),
            MimeType::Avif => String::from(IMAGE_AVIF),
            MimeType::MsVideo => String::from(VIDEO_X_MSVIDEO),
            MimeType::AmazonEbook => String::from(APPLICATION_VND_AMAZON_EBOOK),
            MimeType::Bitmap => String::from(IMAGE_BMP),
            MimeType::Bzip => String::from(APPLICATION_X_BZIP),
            MimeType::Bzip2 => String::from(APPLICATION_X_BZIP2),
            MimeType::Cdf => String::from(APPLICATION_X_CDF),
            MimeType::Csh => String::from(APPLICATION_X_CSH),
            MimeType::Css => String::from(TEXT_CSS),
            MimeType::Csv => String::from(TEXT_CSV),
            MimeType::MSWord => String::from(APPLICATION_MSWORD),
            MimeType::MSWordOpenXML => String::from(APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT),
            MimeType::MSFontObject => String::from(APPLICATION_VND_MS_FONTOBJECT),
            MimeType::Epub => String::from(APPLICATION_EPUB_ZIP),
            MimeType::Gzip => String::from(APPLICATION_GZIP),
            MimeType::Gif => String::from(IMAGE_GIF),
            MimeType::Html => String::from(TEXT_HTML),
            MimeType::Icon => String::from(IMAGE_VNC_MICROSOFT_ICON),
            MimeType::Calendar => String::from(TEXT_CALENDAR),
            MimeType::JavaArchive => String::from(APPLICATION_JAVA_ARCHIVE),
            MimeType::Jpeg => String::from(IMAGE_JPEG),
            MimeType::JavaScript => String::from(TEXT_JAVASCRIPT),
            MimeType::Json => String::from(APPLICATION_JSON),
            MimeType::LDJson => String::from(APPLICATION_LD_JSON),
            MimeType::Midi => String::from(AUDIO_MIDI),
            MimeType::Mpeg => String::from(AUDIO_MPEG),
            MimeType::MP4 => String::from(VIDEO_MP4),
            MimeType::MpegVideo => String::from(VIDEO_MPEG),
            MimeType::AppleInstaller => String::from(APPLICATION_VND_APPLE_INSTALLER_XML),
            MimeType::OpenDocumentPresentation => String::from(APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION),
            MimeType::OpenDocumentSpreadsheet => String::from(APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET),
            MimeType::OpenDocumentText => String::from(APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT),
            MimeType::OggAudio => String::from(AUDIO_OGG),
            MimeType::OggVideo => String::from(VIDEO_OGG),
            MimeType::OggApplication => String::from(APPLICATION_OGG),
            MimeType::Opus => String::from(AUDIO_OPUS),
            MimeType::Otf => String::from(FONT_OTF),
            MimeType::Png => String::from(IMAGE_PNG),
            MimeType::Pdf => String::from(APPLICATION_PDF),
            MimeType::Php => String::from(APPLICATION_X_HTTPD_PHP),
            MimeType::MSPowerPoint => String::from(APPLICATION_VND_MS_POWERPOINT),
            MimeType::MSPowerPointOpenXML => String::from(APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION),
            MimeType::Rar => String::from(APPLICATION_VND_RAR),
            MimeType::Rtf => String::from(APPLICATION_RTF),
            MimeType::Sh => String::from(APPLICATION_X_SH),
            MimeType::Svg => String::from(IMAGE_SVG_XML),
            MimeType::Tar => String::from(APPLICATION_X_TAR),
            MimeType::Tiff => String::from(IMAGE_TIFF),
            MimeType::Mp2t => String::from(VIDEO_MP2T),
            MimeType::Ttf => String::from(FONT_TTF),
            MimeType::Plain => String::from(TEXT_PLAIN),
            MimeType::MSVisio => String::from(APPLICATION_VND_VISIO),
            MimeType::Wav => String::from(AUDIO_WAV),
            MimeType::WebmAudio => String::from(AUDIO_WEBM),
            MimeType::WebmVideo => String::from(VIDEO_WEBM),
            MimeType::Webp => String::from(IMAGE_WEBP),
            MimeType::Woff => String::from(FONT_WOFF),
            MimeType::Woff2 => String::from(FONT_WOFF2),
            MimeType::Xhtml => String::from(APPLICATION_XHTML_XML),
            MimeType::MSExcel => String::from(APPLICATION_VND_MS_EXCEL),
            MimeType::MSExcelOpenXML => String::from(APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEET_SHEET),
            MimeType::Xml => String::from(APPLICATION_XML),
            MimeType::Xul => String::from(APPLICATION_VND_MOZILLA_XUL_XML),
            MimeType::Zip => String::from(APPLICATION_ZIP),
            MimeType::ThreeGp => String::from(VIDEO_3GPP),
            MimeType::ThreeGp2 => String::from(VIDEO_3GPP2),
            MimeType::SevenZ => String::from(APPLICATION_X_7Z_COMPRESSED),
            MimeType::WWWFormURLEncoded => String::from(APPLICATION_X_WWW_FORM_URLENCODED)
        }
    }

    pub fn parse(s: &str) -> Option<MimeType> {
        let s = s.to_lowercase();

        return if s.contains(APPLICATION_OCTET_STREAM) {
            Some(MimeType::OctetStream)
        } else if s.contains(AUDIO_AAC) {
            Some(MimeType::Aac)
        } else if s.contains(APPLICATION_X_ABIWORD) {
            Some(MimeType::AbiWord)
        } else if s.contains(APPLICATION_X_FREEARC) {
            Some(MimeType::FreeArc)
        } else if s.contains(IMAGE_AVIF) {
            Some(MimeType::Avif)
        } else if s.contains(VIDEO_X_MSVIDEO) {
            Some(MimeType::MsVideo)
        } else if s.contains(APPLICATION_VND_AMAZON_EBOOK) {
            Some(MimeType::AmazonEbook)
        } else if s.contains(IMAGE_BMP) {
            Some(MimeType::Bitmap)
        } else if s.contains(APPLICATION_X_BZIP) {
            Some(MimeType::Bzip)
        } else if s.contains(APPLICATION_X_BZIP2) {
            Some(MimeType::Bzip2)
        } else if s.contains(APPLICATION_X_CDF) {
            Some(MimeType::Cdf)
        } else if s.contains(APPLICATION_X_CSH) {
            Some(MimeType::Csh)
        } else if s.contains(TEXT_CSS) {
            Some(MimeType::Css)
        } else if s.contains(TEXT_CSV) {
            Some(MimeType::Csv)
        } else if s.contains(APPLICATION_MSWORD) {
            Some(MimeType::MSWord)
        } else if s.contains(APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT) {
            Some(MimeType::MSExcelOpenXML)
        } else if s.contains(APPLICATION_VND_MS_FONTOBJECT) {
            Some(MimeType::MSFontObject)
        } else if s.contains(APPLICATION_EPUB_ZIP) {
            Some(MimeType::Epub)
        } else if s.contains(APPLICATION_GZIP) {
            Some(MimeType::Gzip)
        } else if s.contains(IMAGE_GIF) {
            Some(MimeType::Gif)
        } else if s.contains(TEXT_HTML) {
            Some(MimeType::Html)
        } else if s.contains(IMAGE_VNC_MICROSOFT_ICON) {
            Some(MimeType::Icon)
        } else if s.contains(TEXT_CALENDAR) {
            Some(MimeType::Calendar)
        } else if s.contains(APPLICATION_JAVA_ARCHIVE) {
            Some(MimeType::JavaArchive)
        } else if s.contains(IMAGE_JPEG) {
            Some(MimeType::Jpeg)
        } else if s.contains(TEXT_JAVASCRIPT) {
            Some(MimeType::JavaScript)
        } else if s.contains(APPLICATION_JSON) {
            Some(MimeType::Json)
        } else if s.contains(APPLICATION_LD_JSON) {
            Some(MimeType::LDJson)
        } else if s.contains(AUDIO_MIDI) {
            Some(MimeType::Midi)
        } else if s.contains(AUDIO_MPEG) {
            Some(MimeType::Mpeg)
        } else if s.contains(VIDEO_MP4) {
            Some(MimeType::MP4)
        } else if s.contains(VIDEO_MPEG) {
            Some(MimeType::MpegVideo)
        } else if s.contains(APPLICATION_VND_APPLE_INSTALLER_XML) {
            Some(MimeType::AppleInstaller)
        } else if s.contains(APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION) {
            Some(MimeType::OpenDocumentPresentation)
        } else if s.contains(APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET) {
            Some(MimeType::OpenDocumentSpreadsheet)
        } else if s.contains(APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT) {
            Some(MimeType::OpenDocumentText)
        } else if s.contains(AUDIO_OGG) {
            Some(MimeType::OggAudio)
        } else if s.contains(VIDEO_OGG) {
            Some(MimeType::OggVideo)
        } else if s.contains(APPLICATION_OGG) {
            Some(MimeType::OggApplication)
        } else if s.contains(AUDIO_OPUS) {
            Some(MimeType::Opus)
        } else if s.contains(FONT_OTF) {
            Some(MimeType::Otf)
        } else if s.contains(IMAGE_PNG) {
            Some(MimeType::Png)
        } else if s.contains(APPLICATION_PDF) {
            Some(MimeType::Pdf)
        } else if s.contains(APPLICATION_X_HTTPD_PHP) {
            Some(MimeType::Php)
        } else if s.contains(APPLICATION_VND_MS_POWERPOINT) {
            Some(MimeType::MSPowerPoint)
        } else if s.contains(APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION) {
            Some(MimeType::MSPowerPointOpenXML)
        } else if s.contains(APPLICATION_VND_RAR) {
            Some(MimeType::Rar)
        } else if s.contains(APPLICATION_RTF) {
            Some(MimeType::Rtf)
        } else if s.contains(APPLICATION_X_SH) {
            Some(MimeType::Sh)
        } else if s.contains(IMAGE_SVG_XML) {
            Some(MimeType::Svg)
        } else if s.contains(APPLICATION_X_TAR) {
            Some(MimeType::Tar)
        } else if s.contains(IMAGE_TIFF) {
            Some(MimeType::Tiff)
        } else if s.contains(VIDEO_MP2T) {
            Some(MimeType::Mp2t)
        } else if s.contains(FONT_TTF) {
            Some(MimeType::Ttf)
        } else if s.contains(TEXT_PLAIN) {
            Some(MimeType::Plain)
        } else if s.contains(APPLICATION_VND_VISIO) {
            Some(MimeType::MSVisio)
        } else if s.contains(AUDIO_WAV) {
            Some(MimeType::Wav)
        } else if s.contains(AUDIO_WEBM) {
            Some(MimeType::WebmAudio)
        } else if s.contains(VIDEO_WEBM) {
            Some(MimeType::WebmVideo)
        } else if s.contains(IMAGE_WEBP) {
            Some(MimeType::Webp)
        } else if s.contains(FONT_WOFF) {
            Some(MimeType::Woff)
        } else if s.contains(FONT_WOFF2) {
            Some(MimeType::Woff2)
        } else if s.contains(APPLICATION_XHTML_XML) {
            Some(MimeType::Xhtml)
        } else if s.contains(APPLICATION_VND_MS_EXCEL) {
            Some(MimeType::MSExcel)
        } else if s.contains(APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEET_SHEET) {
            Some(MimeType::MSExcelOpenXML)
        } else if s.contains(APPLICATION_XML) {
            Some(MimeType::Xml)
        } else if s.contains(APPLICATION_VND_MOZILLA_XUL_XML) {
            Some(MimeType::Xul)
        } else if s.contains(APPLICATION_ZIP) {
            Some(MimeType::Zip)
        } else if s.contains(VIDEO_3GPP) {
            Some(MimeType::ThreeGp)
        } else if s.contains(VIDEO_3GPP2) {
            Some(MimeType::ThreeGp2)
        } else if s.contains(APPLICATION_X_7Z_COMPRESSED) {
            Some(MimeType::SevenZ)
        } else if s.contains(APPLICATION_X_WWW_FORM_URLENCODED) {
            Some(MimeType::WWWFormURLEncoded)
        } else {
            None
        };
    }

    pub fn from_file_path(f: &str) -> MimeType {
        let extension = Path::new(f).extension();

        if extension == None {
            return MimeType::OctetStream
        }

        let t = extension.unwrap().to_str().unwrap();

        return match t {
            "aac" => MimeType::Aac,
            "abw" => MimeType::AbiWord,
            "arc" => MimeType::FreeArc,
            "avif" => MimeType::Avif,
            "avi" => MimeType::MsVideo,
            "azw" => MimeType::AmazonEbook,
            "bin" => MimeType::OctetStream,
            "bmp" => MimeType::Bitmap,
            "bz" => MimeType::Bzip,
            "bz2" => MimeType::Bzip2,
            "cda" => MimeType::Cdf,
            "csh" => MimeType::Csh,
            "css" => MimeType::Css,
            "csv" => MimeType::Csv,
            "doc" => MimeType::MSWord,
            "docx" => MimeType::MSWordOpenXML,
            "eot" => MimeType::MSFontObject,
            "epub" => MimeType::Epub,
            "gz" => MimeType::Gzip,
            "gif" => MimeType::Gif,
            "htm" => MimeType::Html,
            "html" => MimeType::Html,
            "ico" => MimeType::Icon,
            "ics" => MimeType::Calendar,
            "jar" => MimeType::JavaArchive,
            "jpeg" => MimeType::Jpeg,
            "jpg" => MimeType::Jpeg,
            "js" => MimeType::JavaScript,
            "json" => MimeType::Json,
            "jsonld" => MimeType::LDJson,
            "mid" => MimeType::Midi,
            "midi" => MimeType::Midi,
            "mjs" => MimeType::JavaScript,
            "mp3" => MimeType::Mpeg,
            "mp4" => MimeType::MP4,
            "mpeg" => MimeType::MpegVideo,
            "mpkg" => MimeType::AppleInstaller,
            "odp" => MimeType::OpenDocumentPresentation,
            "ods" => MimeType::OpenDocumentSpreadsheet,
            "odt" => MimeType::OpenDocumentText,
            "oga" => MimeType::OggAudio,
            "ogv" => MimeType::OggVideo,
            "ogx" => MimeType::OggApplication,
            "opus" => MimeType::Opus,
            "otf" => MimeType::Otf,
            "png" => MimeType::Png,
            "pdf" => MimeType::Pdf,
            "php" => MimeType::Php,
            "ppt" => MimeType::MSPowerPoint,
            "pptx" => MimeType::MSPowerPointOpenXML,
            "rar" => MimeType::Rar,
            "rtf" => MimeType::Rtf,
            "sh" => MimeType::Sh,
            "svg" => MimeType::Svg,
            "tar" => MimeType::Tar,
            "tif" => MimeType::Tiff,
            "tiff" => MimeType::Tiff,
            "ts" => MimeType::Mp2t,
            "ttf" => MimeType::Ttf,
            "txt" => MimeType::Plain,
            "vsd" => MimeType::MSVisio,
            "wav" => MimeType::Wav,
            "weba" => MimeType::WebmAudio,
            "webm" => MimeType::WebmVideo,
            "webp" => MimeType::Webp,
            "woff" => MimeType::Woff,
            "woff2" => MimeType::Woff2,
            "xhtml" => MimeType::Xhtml,
            "xls" => MimeType::MSExcel,
            "xlsx" => MimeType::MSExcelOpenXML,
            "xml" => MimeType::Xml,
            "xul" => MimeType::Xul,
            "zip" => MimeType::Zip,
            "3gp" => MimeType::ThreeGp,
            "3g2" => MimeType::ThreeGp2,
            "7z" => MimeType::SevenZ,
            _ => MimeType::OctetStream
        };
    }
}

const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";
const AUDIO_AAC: &str = "audio/aac";
const APPLICATION_X_ABIWORD: &str = "application/x-abiword";
const APPLICATION_X_FREEARC: &str = "application/x-freearc";
const IMAGE_AVIF: &str = "image/avif";
const VIDEO_X_MSVIDEO: &str = "video/x-msvideo";
const APPLICATION_VND_AMAZON_EBOOK: &str = "application/vnd.amazon.ebook";
const IMAGE_BMP: &str = "image/bmp";
const APPLICATION_X_BZIP: &str = "application/x-bzip";
const APPLICATION_X_BZIP2: &str = "application/x-bzip2";
const APPLICATION_X_CDF: &str = "application/x-cdf";
const APPLICATION_X_CSH: &str = "application/x-csh";
const TEXT_CSS: &str = "text/css";
const TEXT_CSV: &str = "text/csv";
const APPLICATION_MSWORD: &str = "application/msword";
const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const APPLICATION_VND_MS_FONTOBJECT: &str = "application/vnd.ms-fontobject";
const APPLICATION_EPUB_ZIP: &str = "application/epub+zip";
const APPLICATION_GZIP: &str = "application/gzip";
const IMAGE_GIF: &str = "image/gif";
const TEXT_HTML: &str = "text/html";
const IMAGE_VNC_MICROSOFT_ICON: &str = "image/vnd.microsoft.icon";
const TEXT_CALENDAR: &str = "text/calendar";
const APPLICATION_JAVA_ARCHIVE: &str = "application/java-archive";
const IMAGE_JPEG: &str = "image/jpeg";
const TEXT_JAVASCRIPT: &str = "text/javascript";
const APPLICATION_JSON: &str = "application/json";
const APPLICATION_LD_JSON: &str = "application/ld+json";
const AUDIO_MIDI: &str = "audio/midi";
const AUDIO_MPEG: &str = "audio/mpeg";
const VIDEO_MP4: &str = "video/mp4";
const VIDEO_MPEG: &str = "video/mpeg";
const APPLICATION_VND_APPLE_INSTALLER_XML: &str = "application/vnd.apple.installer+xml";
const APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION: &str = "application/vnd.oasis.opendocument.presentation";
const APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET: &str = "application/vnd.oasis.opendocument.spreadsheet";
const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT: &str = "application/vnd.oasis.opendocument.text";
const AUDIO_OGG: &str = "audio/ogg";
const VIDEO_OGG: &str = "video/ogg";
const APPLICATION_OGG: &str = "application/ogg";
const AUDIO_OPUS: &str = "audio/opus";
const FONT_OTF: &str = "font/otf";
const IMAGE_PNG: &str = "image/png";
const APPLICATION_PDF: &str = "application/pdf";
const APPLICATION_X_HTTPD_PHP: &str = "application/x-httpd-php";
const APPLICATION_VND_MS_POWERPOINT: &str = "application/vnd.ms-powerpoint";
const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation";
const APPLICATION_VND_RAR: &str = "application/vnd.rar";
const APPLICATION_RTF: &str = "application/rtf";
const APPLICATION_X_SH: &str = "application/x-sh";
const IMAGE_SVG_XML: &str = "image/svg+xml";
const APPLICATION_X_TAR: &str = "image/svg+xml";
const IMAGE_TIFF: &str = "image/tiff";
const VIDEO_MP2T: &str = "video/mp2t";
const FONT_TTF: &str = "font/ttf";
const TEXT_PLAIN: &str = "text/plain";
const APPLICATION_VND_VISIO: &str = "application/vnd.visio";
const AUDIO_WAV: &str = "audio/wav";
const AUDIO_WEBM: &str = "audio/webm";
const VIDEO_WEBM: &str = "video/webm";
const IMAGE_WEBP: &str = "image/webp";
const FONT_WOFF: &str = "font/woff";
const FONT_WOFF2: &str = "font/woff2";
const APPLICATION_XHTML_XML: &str = "application/xhtml+xml";
const APPLICATION_VND_MS_EXCEL: &str = "application/vnd.ms-excel";
const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEET_SHEET: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
const APPLICATION_XML: &str = "application/xml";
const APPLICATION_VND_MOZILLA_XUL_XML: &str = "application/vnd.mozilla.xul+xml";
const APPLICATION_ZIP: &str = "application/zip";
const VIDEO_3GPP: &str = "video/3gpp";
const VIDEO_3GPP2: &str = "video/3gpp2";
const APPLICATION_X_7Z_COMPRESSED: &str = "application/x-7z-compressed";
const APPLICATION_X_WWW_FORM_URLENCODED: &str = "application/x-www-form-urlencoded";