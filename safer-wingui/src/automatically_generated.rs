pub struct Aashellmenufilename {
    pub cbTotal: i16,
    pub rgbReserved: [u8; 12],
    pub szFileName: [u8; 1],
}
pub struct Aashellmenuitem<'a> {
    pub lpReserved1: &'a mut todo_void,
    pub iReserved: i32,
    pub uiReserved: u32,
    pub lpName: &'a mut AASHELLMENUFILENAME,
    pub psz: &'a mut Cow<'a, OsStr>,
}
pub struct Abc {
    pub abcA: i32,
    pub abcB: u32,
    pub abcC: i32,
}
pub struct Abcfloat {
    pub abcfA: f32,
    pub abcfB: f32,
    pub abcfC: f32,
}
pub struct Accel {
    pub fVirt: u8,
    pub key: u16,
    pub cmd: u16,
}
pub const ACEO_NONE: i32 = 0x0;
pub const ACEO_MOSTRECENTFIRST: i32 = 0x1;
pub const ACEO_FIRSTUNUSED: i32 = 0x10000;
pub const AO_NONE: i32 = 0x0;
pub const AO_DESIGNMODE: i32 = 0x1;
pub const AO_NOERRORUI: i32 = 0x2;
pub const AO_NOSPLASHSCREEN: i32 = 0x4;
pub const AO_PRELAUNCH: i32 = 0x2000000;
pub const ADE_NONE: i32 = 0x0;
pub const ADE_LEFT: i32 = 0x1;
pub const ADE_RIGHT: i32 = 0x2;
pub const AHE_DESKTOP: i32 = 0x0;
pub const AHE_IMMERSIVE: i32 = 0x1;
pub const AHTYPE_UNDEFINED: i32 = 0x0;
pub const AHTYPE_USER_APPLICATION: i32 = 0x8;
pub const AHTYPE_ANY_APPLICATION: i32 = 0x10;
pub const AHTYPE_MACHINEDEFAULT: i32 = 0x20;
pub const AHTYPE_PROGID: i32 = 0x40;
pub const AHTYPE_APPLICATION: i32 = 0x80;
pub const AHTYPE_CLASS_APPLICATION: i32 = 0x100;
pub const AHTYPE_ANY_PROGID: i32 = 0x200;
pub struct Alttabinfo {
    pub cbSize: u32,
    pub cItems: i32,
    pub cColumns: i32,
    pub cRows: i32,
    pub iColFocus: i32,
    pub iRowFocus: i32,
    pub cxItem: i32,
    pub cyItem: i32,
    pub ptStart: POINT,
}
pub const AW_ACTIVATE: u32 = 0x20000;
pub const AW_BLEND: u32 = 0x80000;
pub const AW_CENTER: u32 = 0x10;
pub const AW_HIDE: u32 = 0x10000;
pub const AW_HOR_POSITIVE: u32 = 0x1;
pub const AW_HOR_NEGATIVE: u32 = 0x2;
pub const AW_SLIDE: u32 = 0x40000;
pub const AW_VER_POSITIVE: u32 = 0x4;
pub const AW_VER_NEGATIVE: u32 = 0x8;
pub struct Animationinfo {
    pub cbSize: u32,
    pub iMinAnimate: i32,
}
pub const APPACTION_INSTALL: i32 = 0x1;
pub const APPACTION_UNINSTALL: i32 = 0x2;
pub const APPACTION_MODIFY: i32 = 0x4;
pub const APPACTION_REPAIR: i32 = 0x8;
pub const APPACTION_UPGRADE: i32 = 0x10;
pub const APPACTION_CANGETSIZE: i32 = 0x20;
pub const APPACTION_MODIFYREMOVE: i32 = 0x80;
pub const APPACTION_ADDLATER: i32 = 0x100;
pub const APPACTION_UNSCHEDULE: i32 = 0x200;
pub struct Appbardata {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uCallbackMessage: u32,
    pub uEdge: u32,
    pub rc: RECT,
    pub lParam: LPARAM,
}
pub struct Appcategoryinfo<'a> {
    pub Locale: u32,
    pub pszDescription: &'a mut Cow<'a, OsStr>,
    pub AppCategoryId: GUID,
}
pub struct Appcategoryinfolist<'a> {
    pub cCategory: u32,
    pub pCategoryInfo: &'a mut APPCATEGORYINFO<'a>,
}
pub const ADLT_RECENT: i32 = 0x0;
pub const ADLT_FREQUENT: i32 = 0x1;
pub struct Appinfodata<'a> {
    pub cbSize: u32,
    pub dwMask: u32,
    pub pszDisplayName: &'a mut Cow<'a, OsStr>,
    pub pszVersion: &'a mut Cow<'a, OsStr>,
    pub pszPublisher: &'a mut Cow<'a, OsStr>,
    pub pszProductID: &'a mut Cow<'a, OsStr>,
    pub pszRegisteredOwner: &'a mut Cow<'a, OsStr>,
    pub pszRegisteredCompany: &'a mut Cow<'a, OsStr>,
    pub pszLanguage: &'a mut Cow<'a, OsStr>,
    pub pszSupportUrl: &'a mut Cow<'a, OsStr>,
    pub pszSupportTelephone: &'a mut Cow<'a, OsStr>,
    pub pszHelpLink: &'a mut Cow<'a, OsStr>,
    pub pszInstallLocation: &'a mut Cow<'a, OsStr>,
    pub pszInstallSource: &'a mut Cow<'a, OsStr>,
    pub pszInstallDate: &'a mut Cow<'a, OsStr>,
    pub pszContact: &'a mut Cow<'a, OsStr>,
    pub pszComments: &'a mut Cow<'a, OsStr>,
    pub pszImage: &'a mut Cow<'a, OsStr>,
    pub pszReadmeUrl: &'a mut Cow<'a, OsStr>,
    pub pszUpdateInfoUrl: &'a mut Cow<'a, OsStr>,
}
pub const AIM_DISPLAYNAME: i32 = 0x1;
pub const AIM_VERSION: i32 = 0x2;
pub const AIM_PUBLISHER: i32 = 0x4;
pub const AIM_PRODUCTID: i32 = 0x8;
pub const AIM_REGISTEREDOWNER: i32 = 0x10;
pub const AIM_REGISTEREDCOMPANY: i32 = 0x20;
pub const AIM_LANGUAGE: i32 = 0x40;
pub const AIM_SUPPORTURL: i32 = 0x80;
pub const AIM_SUPPORTTELEPHONE: i32 = 0x100;
pub const AIM_HELPLINK: i32 = 0x200;
pub const AIM_INSTALLLOCATION: i32 = 0x400;
pub const AIM_INSTALLSOURCE: i32 = 0x800;
pub const AIM_INSTALLDATE: i32 = 0x1000;
pub const AIM_CONTACT: i32 = 0x4000;
pub const AIM_COMMENTS: i32 = 0x8000;
pub const AIM_IMAGE: i32 = 0x20000;
pub const AIM_READMEURL: i32 = 0x40000;
pub const AIM_UPDATEINFOURL: i32 = 0x80000;
pub const AVMW_DEFAULT: i32 = 0x0;
pub const AVMW_320: i32 = 0x1;
pub const AVMW_500: i32 = 0x2;
pub const AVO_LANDSCAPE: i32 = 0x0;
pub const AVO_PORTRAIT: i32 = 0x1;
pub const AVSP_DEFAULT: i32 = 0x0;
pub const AVSP_USE_LESS: i32 = 0x1;
pub const AVSP_USE_HALF: i32 = 0x2;
pub const AVSP_USE_MORE: i32 = 0x3;
pub const AVSP_USE_MINIMUM: i32 = 0x4;
pub const AVSP_USE_NONE: i32 = 0x5;
pub const AVSP_CUSTOM: i32 = 0x6;
pub const AVS_FULLSCREEN_LANDSCAPE: i32 = 0x0;
pub const AVS_FILLED: i32 = 0x1;
pub const AVS_SNAPPED: i32 = 0x2;
pub const AVS_FULLSCREEN_PORTRAIT: i32 = 0x3;
pub struct AppLocalDeviceId {
    pub value: [u8; 32],
}
pub const AD_COUNTERCLOCKWISE: u32 = 0x1;
pub const AD_CLOCKWISE: u32 = 0x2;
pub const ASSOCCLASS_SHELL_KEY: i32 = 0x0;
pub const ASSOCCLASS_PROGID_KEY: i32 = 0x1;
pub const ASSOCCLASS_PROGID_STR: i32 = 0x2;
pub const ASSOCCLASS_CLSID_KEY: i32 = 0x3;
pub const ASSOCCLASS_CLSID_STR: i32 = 0x4;
pub const ASSOCCLASS_APP_KEY: i32 = 0x5;
pub const ASSOCCLASS_APP_STR: i32 = 0x6;
pub const ASSOCCLASS_SYSTEM_STR: i32 = 0x7;
pub const ASSOCCLASS_FOLDER: i32 = 0x8;
pub const ASSOCCLASS_STAR: i32 = 0x9;
pub const ASSOCCLASS_FIXED_PROGID_STR: i32 = 0xA;
pub const ASSOCCLASS_PROTOCOL_STR: i32 = 0xB;
pub const ASSOCDATA_MSIDESCRIPTOR: i32 = 0x1;
pub const ASSOCDATA_NOACTIVATEHANDLER: i32 = 0x2;
pub const ASSOCDATA_UNUSED1: i32 = 0x3;
pub const ASSOCDATA_HASPERUSERASSOC: i32 = 0x4;
pub const ASSOCDATA_EDITFLAGS: i32 = 0x5;
pub const ASSOCDATA_VALUE: i32 = 0x6;
pub const ASSOCDATA_MAX: i32 = 0x7;
pub const ASSOCENUM_NONE: i32 = 0x0;
pub struct Associationelement<'a> {
    pub ac: i32,
    pub hkClass: HKEY,
    pub pszClass: &'a Cow<'a, OsStr>,
}
pub const AL_MACHINE: i32 = 0x0;
pub const AL_EFFECTIVE: i32 = 0x1;
pub const AL_USER: i32 = 0x2;
pub const AT_FILEEXTENSION: i32 = 0x0;
pub const AT_URLPROTOCOL: i32 = 0x1;
pub const AT_STARTMENUCLIENT: i32 = 0x2;
pub const AT_MIMETYPE: i32 = 0x3;
pub const ASSOCKEY_SHELLEXECCLASS: i32 = 0x1;
pub const ASSOCKEY_APP: i32 = 0x2;
pub const ASSOCKEY_CLASS: i32 = 0x3;
pub const ASSOCKEY_BASECLASS: i32 = 0x4;
pub const ASSOCKEY_MAX: i32 = 0x5;
pub const ASSOCSTR_COMMAND: i32 = 0x1;
pub const ASSOCSTR_EXECUTABLE: i32 = 0x2;
pub const ASSOCSTR_FRIENDLYDOCNAME: i32 = 0x3;
pub const ASSOCSTR_FRIENDLYAPPNAME: i32 = 0x4;
pub const ASSOCSTR_NOOPEN: i32 = 0x5;
pub const ASSOCSTR_SHELLNEWVALUE: i32 = 0x6;
pub const ASSOCSTR_DDECOMMAND: i32 = 0x7;
pub const ASSOCSTR_DDEIFEXEC: i32 = 0x8;
pub const ASSOCSTR_DDEAPPLICATION: i32 = 0x9;
pub const ASSOCSTR_DDETOPIC: i32 = 0xA;
pub const ASSOCSTR_INFOTIP: i32 = 0xB;
pub const ASSOCSTR_QUICKTIP: i32 = 0xC;
pub const ASSOCSTR_TILEINFO: i32 = 0xD;
pub const ASSOCSTR_CONTENTTYPE: i32 = 0xE;
pub const ASSOCSTR_DEFAULTICON: i32 = 0xF;
pub const ASSOCSTR_SHELLEXTENSION: i32 = 0x10;
pub const ASSOCSTR_DROPTARGET: i32 = 0x11;
pub const ASSOCSTR_DELEGATEEXECUTE: i32 = 0x12;
pub const ASSOCSTR_SUPPORTED_URI_PROTOCOLS: i32 = 0x13;
pub const ASSOCSTR_PROGID: i32 = 0x14;
pub const ASSOCSTR_APPID: i32 = 0x15;
pub const ASSOCSTR_APPPUBLISHER: i32 = 0x16;
pub const ASSOCSTR_APPICONREFERENCE: i32 = 0x17;
pub const ASSOCSTR_MAX: i32 = 0x18;
pub const ASSOC_FILTER_NONE: i32 = 0x0;
pub const ASSOC_FILTER_RECOMMENDED: i32 = 0x1;
pub const ATTACHMENT_ACTION_CANCEL: i32 = 0x0;
pub const ATTACHMENT_ACTION_SAVE: i32 = 0x1;
pub const ATTACHMENT_ACTION_EXEC: i32 = 0x2;
pub const ATTACHMENT_PROMPT_NONE: i32 = 0x0;
pub const ATTACHMENT_PROMPT_SAVE: i32 = 0x1;
pub const ATTACHMENT_PROMPT_EXEC: i32 = 0x2;
pub const ATTACHMENT_PROMPT_EXEC_OR_SAVE: i32 = 0x3;
pub struct Audiodescription {
    pub cbSize: u32,
    pub Enabled: BOOL,
    pub Locale: u32,
}
pub const ACLO_NONE: i32 = 0x0;
pub const ACLO_CURRENTDIR: i32 = 0x1;
pub const ACLO_MYCOMPUTER: i32 = 0x2;
pub const ACLO_DESKTOP: i32 = 0x4;
pub const ACLO_FAVORITES: i32 = 0x8;
pub const ACLO_FILESYSONLY: i32 = 0x10;
pub const ACLO_FILESYSDIRS: i32 = 0x20;
pub const ACLO_VIRTUALNAMESPACE: i32 = 0x40;
pub const ACO_NONE: i32 = 0x0;
pub const ACO_AUTOSUGGEST: i32 = 0x1;
pub const ACO_AUTOAPPEND: i32 = 0x2;
pub const ACO_SEARCH: i32 = 0x4;
pub const ACO_FILTERPREFIXES: i32 = 0x8;
pub const ACO_USETAB: i32 = 0x10;
pub const ACO_UPDOWNKEYDROPSLIST: i32 = 0x20;
pub const ACO_RTLREADING: i32 = 0x40;
pub const ACO_WORD_FILTER: i32 = 0x80;
pub const ACO_NOPREFIXFILTERING: i32 = 0x100;
pub struct AutoScrollData {
    pub iNextSample: i32,
    pub dwLastScroll: u32,
    pub bFull: BOOL,
    pub pts: [POINT; 3],
    pub dwTimes: [u32; 3],
}
pub struct Axeslista {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOA; 16],
}
pub struct Axeslistw {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOW; 16],
}
pub struct Axisinfoa {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u8; 16],
}
pub struct Axisinfow {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u8; 16],
}
pub struct AccessibilityDockingService {
}
pub struct AlphabeticalCategorizer {
}
pub struct AppShellVerbHandler {
}
pub struct AppStartupLink {
}
pub struct AppVisibility {
}
pub struct ApplicationActivationManager {
}
pub struct ApplicationAssociationRegistration {
}
pub struct ApplicationAssociationRegistrationUi {
}
pub struct ApplicationDesignModeSettings {
}
pub struct ApplicationDestinations {
}
pub struct ApplicationDocumentLists {
}
pub struct AttachmentServices {
}
pub const OPAQUE: u32 = 0x2;
pub const TRANSPARENT: u32 = 0x1;
pub const BSID_BANDADDED: i32 = 0x0;
pub const BSID_BANDREMOVED: i32 = 0x1;
pub struct Bandsiteinfo {
    pub dwMask: u32,
    pub dwState: u32,
    pub dwStyle: u32,
}
pub struct BannerNotification<'a> {
    pub event: i32,
    pub providerIdentity: &'a Cow<'a, OsStr>,
    pub contentId: &'a Cow<'a, OsStr>,
}
pub const BNE_RENDERED: i32 = 0x0;
pub const BNE_HOVERED: i32 = 0x1;
pub const BNE_CLOSED: i32 = 0x2;
pub const BNE_DISMISSED: i32 = 0x3;
pub const BNE_BUTTON1_CLICKED: i32 = 0x4;
pub const BNE_BUTTON2_CLICKED: i32 = 0x5;
pub const BT_IMAGEFILE: i32 = 0x0;
pub const BT_BORDERFILL: i32 = 0x1;
pub const BT_NONE: i32 = 0x2;
pub struct Bitmap<'a> {
    pub bmType: i32,
    pub bmWidth: i32,
    pub bmHeight: i32,
    pub bmWidthBytes: i32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub bmBits: &'a mut todo_void,
}
pub struct Bitmapcoreheader {
    pub bcSize: u32,
    pub bcWidth: u16,
    pub bcHeight: u16,
    pub bcPlanes: u16,
    pub bcBitCount: u16,
}
pub struct Bitmapcoreinfo {
    pub bmciHeader: BITMAPCOREHEADER,
    pub bmciColors: [RGBTRIPLE; 1],
}
pub struct Bitmapfileheader {
    pub bfType: u16,
    pub bfSize: u32,
    pub bfReserved1: u16,
    pub bfReserved2: u16,
    pub bfOffBits: u32,
}
pub struct Bitmapinfo {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}
pub struct Bitmapinfoheader {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
pub struct Bitmapv4header {
    pub bV4Size: u32,
    pub bV4Width: i32,
    pub bV4Height: i32,
    pub bV4Planes: u16,
    pub bV4BitCount: u16,
    pub bV4V4Compression: u32,
    pub bV4SizeImage: u32,
    pub bV4XPelsPerMeter: i32,
    pub bV4YPelsPerMeter: i32,
    pub bV4ClrUsed: u32,
    pub bV4ClrImportant: u32,
    pub bV4RedMask: u32,
    pub bV4GreenMask: u32,
    pub bV4BlueMask: u32,
    pub bV4AlphaMask: u32,
    pub bV4CSType: u32,
    pub bV4Endpoints: CIEXYZTRIPLE,
    pub bV4GammaRed: u32,
    pub bV4GammaGreen: u32,
    pub bV4GammaBlue: u32,
}
pub struct Bitmapv5header {
    pub bV5Size: u32,
    pub bV5Width: i32,
    pub bV5Height: i32,
    pub bV5Planes: u16,
    pub bV5BitCount: u16,
    pub bV5Compression: u32,
    pub bV5SizeImage: u32,
    pub bV5XPelsPerMeter: i32,
    pub bV5YPelsPerMeter: i32,
    pub bV5ClrUsed: u32,
    pub bV5ClrImportant: u32,
    pub bV5RedMask: u32,
    pub bV5GreenMask: u32,
    pub bV5BlueMask: u32,
    pub bV5AlphaMask: u32,
    pub bV5CSType: u32,
    pub bV5Endpoints: CIEXYZTRIPLE,
    pub bV5GammaRed: u32,
    pub bV5GammaGreen: u32,
    pub bV5GammaBlue: u32,
    pub bV5Intent: u32,
    pub bV5ProfileData: u32,
    pub bV5ProfileSize: u32,
    pub bV5Reserved: u32,
}
pub struct Blendfunction {
    pub BlendOp: u8,
    pub BlendFlags: u8,
    pub SourceConstantAlpha: u8,
    pub AlphaFormat: u8,
}
pub const BNS_NORMAL: i32 = 0x0;
pub const BNS_BEGIN_NAVIGATE: i32 = 0x1;
pub const BNS_NAVIGATE: i32 = 0x2;
pub const BT_RECT: i32 = 0x0;
pub const BT_ROUNDRECT: i32 = 0x1;
pub const BT_ELLIPSE: i32 = 0x2;
pub struct BpAnimationparams {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub style: i32,
    pub dwDuration: u32,
}
pub const BPAS_NONE: i32 = 0x0;
pub const BPAS_LINEAR: i32 = 0x1;
pub const BPAS_CUBIC: i32 = 0x2;
pub const BPAS_SINE: i32 = 0x3;
pub const BPBF_COMPATIBLEBITMAP: i32 = 0x0;
pub const BPBF_DIB: i32 = 0x1;
pub const BPBF_TOPDOWNDIB: i32 = 0x2;
pub const BPBF_TOPDOWNMONODIB: i32 = 0x3;
pub struct BpPaintparams<'a> {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub prcExclude: &'a RECT,
    pub pBlendFunction: &'a BLENDFUNCTION,
}
pub const BPPF_ERASE: u32 = 0x1;
pub const BPPF_NOCLIP: u32 = 0x2;
pub const BPPF_NONCLIENT: u32 = 0x4;
pub struct ButtonImagelist {
    pub himl: HIMAGELIST,
    pub margin: RECT,
    pub uAlign: u32,
}
pub const BUTTON_IMAGELIST_ALIGN_LEFT: u32 = 0x0;
pub const BUTTON_IMAGELIST_ALIGN_RIGHT: u32 = 0x1;
pub const BUTTON_IMAGELIST_ALIGN_TOP: u32 = 0x2;
pub const BUTTON_IMAGELIST_ALIGN_BOTTOM: u32 = 0x3;
pub const BUTTON_IMAGELIST_ALIGN_CENTER: u32 = 0x4;
pub struct ButtonSplitinfo {
    pub mask: u32,
    pub himlGlyph: HIMAGELIST,
    pub uSplitStyle: u32,
    pub size: SIZE,
}
pub const NAV_OPEN_IN_NEW_WINDOW: i32 = 0x1;
pub const NAV_NO_HISTORY: i32 = 0x2;
pub const NAV_NO_READ_FROM_CACHE: i32 = 0x4;
pub const NAV_NO_WRITE_TO_CACHE: i32 = 0x8;
pub const NAV_ALLOW_AUTOSEARCH: i32 = 0x10;
pub const NAV_BROWSER_BAR: i32 = 0x20;
pub const NAV_HYPERLINK: i32 = 0x40;
pub const NAV_ENFORCE_RESTRICTED: i32 = 0x80;
pub const NAV_NEW_WINDOWS_MANAGED: i32 = 0x100;
pub const NAV_UNTRUSTED_FOR_DOWNLOAD: i32 = 0x200;
pub const NAV_TRUSTED_FOR_ACTIVE_X: i32 = 0x400;
pub const NAV_OPEN_IN_NEW_TAB: i32 = 0x800;
pub const NAV_OPEN_IN_BACKGROUND_TAB: i32 = 0x1000;
pub const NAV_KEEP_WORD_WHEEL_TEXT: i32 = 0x2000;
pub const NAV_VIRTUAL_TAB: i32 = 0x4000;
pub const NAV_BLOCK_REDIRECTS_X_DOMAIN: i32 = 0x8000;
pub const NAV_OPEN_NEW_FOREGROUND_TAB: i32 = 0x10000;
pub const NAV_TRAVEL_LOG_SCREENSHOT: i32 = 0x20000;
pub const NAV_DEFER_UNLOAD: i32 = 0x40000;
pub const NAV_SPECULATIVE: i32 = 0x80000;
pub const NAV_SUGGEST_NEW_WINDOW: i32 = 0x100000;
pub const NAV_SUGGEST_NEW_TAB: i32 = 0x200000;
pub const NAV_RESERVED1: i32 = 0x400000;
pub const NAV_HOMEPAGE_NAVIGATE: i32 = 0x800000;
pub const NAV_REFRESH: i32 = 0x1000000;
pub const NAV_HOST_NAVIGATION: i32 = 0x2000000;
pub const NAV_RESERVED2: i32 = 0x4000000;
pub const NAV_RESERVED3: i32 = 0x8000000;
pub const NAV_RESERVED4: i32 = 0x10000000;
pub const NAV_RESERVED5: i32 = 0x20000000;
pub const NAV_RESERVED6: i32 = 0x40000000;
pub const NAV_RESERVED7: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000;
pub struct Cabinetstate {
    pub cLength: u16,
    pub nVersion: u16,
    pub _bitfield: i32,
    pub fMenuEnumFilter: u32,
}
pub const MDITILE_SKIPDISABLED: u32 = 0x2;
pub const MDITILE_ZORDER: u32 = 0x4;
pub const CATINFO_NORMAL: i32 = 0x0;
pub const CATINFO_COLLAPSED: i32 = 0x1;
pub const CATINFO_HIDDEN: i32 = 0x2;
pub const CATINFO_EXPANDED: i32 = 0x4;
pub const CATINFO_NOHEADER: i32 = 0x8;
pub const CATINFO_NOTCOLLAPSIBLE: i32 = 0x10;
pub const CATINFO_NOHEADERCOUNT: i32 = 0x20;
pub const CATINFO_SUBSETTED: i32 = 0x40;
pub const CATINFO_SEPARATE_IMAGES: i32 = 0x80;
pub const CATINFO_SHOWEMPTY: i32 = 0x100;
pub struct CategoryInfo {
    pub cif: i32,
    pub wszName: [u8; 260],
}
pub const CATSORT_DEFAULT: i32 = 0x0;
pub const CATSORT_NAME: i32 = 0x1;
pub struct Cbtactivatestruct {
    pub fMouse: BOOL,
    pub hWndActive: HWND,
}
pub struct CbtCreatewnda<'a> {
    pub lpcs: &'a mut CREATESTRUCTA<'a>,
    pub hwndInsertAfter: HWND,
}
pub struct CbtCreatewndw<'a> {
    pub lpcs: &'a mut CREATESTRUCTW<'a>,
    pub hwndInsertAfter: HWND,
}
pub struct Ccinfoa<'a> {
    pub szClass: [CHAR; 32],
    pub flOptions: u32,
    pub szDesc: [CHAR; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub szTextDefault: [CHAR; 256],
    pub cStyleFlags: i32,
    pub aStyleFlags: &'a mut CCSTYLEFLAGA<'a>,
    pub lpfnStyle: todo_fn,
    pub lpfnSizeToText: todo_fn,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
pub struct Ccinfow<'a> {
    pub szClass: [u8; 32],
    pub flOptions: u32,
    pub szDesc: [u8; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub cStyleFlags: i32,
    pub aStyleFlags: &'a mut CCSTYLEFLAGW<'a>,
    pub szTextDefault: [u8; 256],
    pub lpfnStyle: todo_fn,
    pub lpfnSizeToText: todo_fn,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
pub struct Ccstylea {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [CHAR; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
pub struct Ccstyleflaga<'a> {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: &'a mut Cow<'a, str>,
}
pub struct Ccstyleflagw<'a> {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: &'a mut Cow<'a, OsStr>,
}
pub struct Ccstylew {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [u8; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
pub const CDBE_RET_DEFAULT: i32 = 0x0;
pub const CDBE_RET_DONTRUNOTHEREXTS: i32 = 0x1;
pub const CDBE_RET_STOPWIZARD: i32 = 0x2;
pub struct CdBurn {
}
pub const CDCS_INACTIVE: i32 = 0x0;
pub const CDCS_ENABLED: i32 = 0x1;
pub const CDCS_VISIBLE: i32 = 0x2;
pub const CDCS_ENABLEDVISIBLE: i32 = 0x3;
pub const CDS_FULLSCREEN: u32 = 0x4;
pub const CDS_GLOBAL: u32 = 0x8;
pub const CDS_NORESET: u32 = 0x10000000;
pub const CDS_RESET: u32 = 0x40000000;
pub const CDS_SET_PRIMARY: u32 = 0x10;
pub const CDS_TEST: u32 = 0x2;
pub const CDS_UPDATEREGISTRY: u32 = 0x1;
pub const CDS_VIDEOPARAMETERS: u32 = 0x20;
pub const CDS_ENABLE_UNSAFE_MODES: u32 = 0x100;
pub const CDS_DISABLE_UNSAFE_MODES: u32 = 0x200;
pub const CDS_RESET_EX: u32 = 0x20000000;
pub struct Changefilterstruct {
    pub cbSize: u32,
    pub ExtStatus: u32,
}
pub const MSGFLT_ADD: u32 = 0x1;
pub const MSGFLT_REMOVE: u32 = 0x2;
pub struct Cida {
    pub cidl: u32,
    pub aoffset: [u32; 1],
}
pub struct Cie4ConnectionPoint {
}
pub struct Ciexyz {
    pub ciexyzX: i32,
    pub ciexyzY: i32,
    pub ciexyzZ: i32,
}
pub struct Ciexyztriple {
    pub ciexyzRed: CIEXYZ,
    pub ciexyzGreen: CIEXYZ,
    pub ciexyzBlue: CIEXYZ,
}
pub struct Clientcreatestruct {
    pub hWindowMenu: HANDLE,
    pub idFirstChild: u32,
}
pub const CLP_TIME: i32 = 0x1;
pub const CLS_NORMAL: i32 = 0x1;
pub const CLS_HOT: i32 = 0x2;
pub const CLS_PRESSED: i32 = 0x3;
pub struct Cminvokecommandinfo<'a> {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: HWND,
    pub lpVerb: &'a Cow<'a, str>,
    pub lpParameters: &'a Cow<'a, str>,
    pub lpDirectory: &'a Cow<'a, str>,
    pub nShow: i32,
    pub dwHotKey: u32,
    pub hIcon: HANDLE,
}
pub struct Cminvokecommandinfoex<'a> {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: HWND,
    pub lpVerb: &'a Cow<'a, str>,
    pub lpParameters: &'a Cow<'a, str>,
    pub lpDirectory: &'a Cow<'a, str>,
    pub nShow: i32,
    pub dwHotKey: u32,
    pub hIcon: HANDLE,
    pub lpTitle: &'a Cow<'a, str>,
    pub lpVerbW: &'a Cow<'a, OsStr>,
    pub lpParametersW: &'a Cow<'a, OsStr>,
    pub lpDirectoryW: &'a Cow<'a, OsStr>,
    pub lpTitleW: &'a Cow<'a, OsStr>,
    pub ptInvoke: POINT,
}
pub struct CminvokecommandinfoexRemote<'a> {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: HWND,
    pub lpVerbString: &'a Cow<'a, str>,
    pub lpParameters: &'a Cow<'a, str>,
    pub lpDirectory: &'a Cow<'a, str>,
    pub nShow: i32,
    pub dwHotKey: u32,
    pub lpTitle: &'a Cow<'a, str>,
    pub lpVerbWString: &'a Cow<'a, OsStr>,
    pub lpParametersW: &'a Cow<'a, OsStr>,
    pub lpDirectoryW: &'a Cow<'a, OsStr>,
    pub lpTitleW: &'a Cow<'a, OsStr>,
    pub ptInvoke: POINT,
    pub lpVerbInt: u32,
    pub lpVerbWInt: u32,
}
pub struct CmColumninfo {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwState: u32,
    pub uWidth: u32,
    pub uDefaultWidth: u32,
    pub uIdealWidth: u32,
    pub wszName: [u8; 80],
}
pub const CM_ENUM_ALL: i32 = 0x1;
pub const CM_ENUM_VISIBLE: i32 = 0x2;
pub const CM_MASK_WIDTH: i32 = 0x1;
pub const CM_MASK_DEFAULTWIDTH: i32 = 0x2;
pub const CM_MASK_IDEALWIDTH: i32 = 0x4;
pub const CM_MASK_NAME: i32 = 0x8;
pub const CM_MASK_STATE: i32 = 0x10;
pub const CM_WIDTH_USEDEFAULT: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const CM_WIDTH_AUTOSIZE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE;
pub const CM_STATE_NONE: i32 = 0x0;
pub const CM_STATE_VISIBLE: i32 = 0x1;
pub const CM_STATE_FIXEDWIDTH: i32 = 0x2;
pub const CM_STATE_NOSORTBYFOLDERNESS: i32 = 0x4;
pub const CM_STATE_ALWAYSVISIBLE: i32 = 0x8;
pub struct Coloradjustment {
    pub caSize: u16,
    pub caFlags: u16,
    pub caIlluminantIndex: u16,
    pub caRedGamma: u16,
    pub caGreenGamma: u16,
    pub caBlueGamma: u16,
    pub caReferenceBlack: u16,
    pub caReferenceWhite: u16,
    pub caContrast: i16,
    pub caBrightness: i16,
    pub caColorfulness: i16,
    pub caRedGreenTint: i16,
}
pub struct Colorcorrectpalette {
    pub emr: EMR,
    pub ihPalette: u32,
    pub nFirstEntry: u32,
    pub nPalEntries: u32,
    pub nReserved: u32,
}
pub struct Colormap {
    pub from: u32,
    pub to: u32,
}
pub struct Colormatchtotarget {
    pub emr: EMR,
    pub dwAction: u32,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
pub struct Colorscheme {
    pub dwSize: u32,
    pub clrBtnHighlight: u32,
    pub clrBtnShadow: u32,
}
pub struct Comboboxexitema<'a> {
    pub mask: u32,
    pub iItem: isize,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: LPARAM,
}
pub struct Comboboxexitemw<'a> {
    pub mask: u32,
    pub iItem: isize,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: LPARAM,
}
pub struct Comboboxinfo {
    pub cbSize: u32,
    pub rcItem: RECT,
    pub rcButton: RECT,
    pub stateButton: u32,
    pub hwndCombo: HWND,
    pub hwndItem: HWND,
    pub hwndList: HWND,
}
pub const STATE_SYSTEM_INVISIBLE: u32 = 0x8000;
pub const STATE_SYSTEM_PRESSED: u32 = 0x8;
pub const STATE_SYSTEM_FOCUSABLE: u32 = 0x100000;
pub const STATE_SYSTEM_OFFSCREEN: u32 = 0x10000;
pub const STATE_SYSTEM_UNAVAILABLE: u32 = 0x1;
pub const CBEIF_DI_SETITEM: u32 = 0x10000000;
pub const CBEIF_IMAGE: u32 = 0x2;
pub const CBEIF_INDENT: u32 = 0x10;
pub const CBEIF_LPARAM: u32 = 0x20;
pub const CBEIF_OVERLAY: u32 = 0x8;
pub const CBEIF_SELECTEDIMAGE: u32 = 0x4;
pub const CBEIF_TEXT: u32 = 0x1;
pub struct Compareitemstruct {
    pub CtlType: u32,
    pub CtlID: u32,
    pub hwndItem: HWND,
    pub itemID1: u32,
    pub itemData1: usize,
    pub itemID2: u32,
    pub itemData2: usize,
    pub dwLocaleId: u32,
}
pub struct ConfirmConflictResultInfo<'a> {
    pub pszNewName: &'a mut Cow<'a, OsStr>,
    pub iItemIndex: u32,
}
pub const CA_LEFT: i32 = 0x0;
pub const CA_CENTER: i32 = 0x1;
pub const CA_RIGHT: i32 = 0x2;
pub struct Cplinfo {
    pub idIcon: i32,
    pub idName: i32,
    pub idInfo: i32,
    pub lData: isize,
}
pub const CPVIEW_CLASSIC: i32 = 0x0;
pub const CPVIEW_ALLITEMS: i32 = 0x0;
pub const CPVIEW_CATEGORY: i32 = 0x1;
pub const CPVIEW_HOME: i32 = 0x1;
pub struct Createstructa<'a> {
    pub lpCreateParams: &'a mut todo_void,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: &'a Cow<'a, str>,
    pub lpszClass: &'a Cow<'a, str>,
    pub dwExStyle: u32,
}
pub struct Createstructw<'a> {
    pub lpCreateParams: &'a mut todo_void,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: &'a Cow<'a, OsStr>,
    pub lpszClass: &'a Cow<'a, OsStr>,
    pub dwExStyle: u32,
}
pub const TTFCFP_STD_MAC_CHAR_SET: u32 = 0x0;
pub const TTFCFP_SYMBOL_CHAR_SET: u32 = 0x0;
pub const TTFCFP_UNICODE_CHAR_SET: u32 = 0x1;
pub const TTFCFP_UNICODE_PLATFORMID: u32 = 0x0;
pub const TTFCFP_ISO_PLATFORMID: u32 = 0x2;
pub const ALTERNATE: u32 = 0x1;
pub const WINDING: u32 = 0x2;
pub const CPAO_NONE: i32 = 0x0;
pub const CPAO_EMPTY_LOCAL: i32 = 0x1;
pub const CPAO_EMPTY_CONNECTED: i32 = 0x2;
pub const CPCFO_NONE: i32 = 0x0;
pub const CPCFO_ENABLE_PASSWORD_REVEAL: i32 = 0x1;
pub const CPCFO_IS_EMAIL_ADDRESS: i32 = 0x2;
pub const CPCFO_ENABLE_TOUCH_KEYBOARD_AUTO_INVOKE: i32 = 0x4;
pub const CPCFO_NUMBERS_ONLY: i32 = 0x8;
pub const CPCFO_SHOW_ENGLISH_KEYBOARD: i32 = 0x10;
pub struct CredentialProviderCredentialSerialization<'a> {
    pub ulAuthenticationPackage: u32,
    pub clsidCredentialProvider: GUID,
    pub cbSerialization: u32,
    pub rgbSerialization: &'a mut u8,
}
pub struct CredentialProviderFieldDescriptor<'a> {
    pub dwFieldID: u32,
    pub cpft: i32,
    pub pszLabel: &'a mut Cow<'a, OsStr>,
    pub guidFieldType: GUID,
}
pub const CPFIS_NONE: i32 = 0x0;
pub const CPFIS_READONLY: i32 = 0x1;
pub const CPFIS_DISABLED: i32 = 0x2;
pub const CPFIS_FOCUSED: i32 = 0x3;
pub const CPFS_HIDDEN: i32 = 0x0;
pub const CPFS_DISPLAY_IN_SELECTED_TILE: i32 = 0x1;
pub const CPFS_DISPLAY_IN_DESELECTED_TILE: i32 = 0x2;
pub const CPFS_DISPLAY_IN_BOTH: i32 = 0x3;
pub const CPFT_INVALID: i32 = 0x0;
pub const CPFT_LARGE_TEXT: i32 = 0x1;
pub const CPFT_SMALL_TEXT: i32 = 0x2;
pub const CPFT_COMMAND_LINK: i32 = 0x3;
pub const CPFT_EDIT_TEXT: i32 = 0x4;
pub const CPFT_PASSWORD_TEXT: i32 = 0x5;
pub const CPFT_TILE_IMAGE: i32 = 0x6;
pub const CPFT_CHECKBOX: i32 = 0x7;
pub const CPFT_COMBOBOX: i32 = 0x8;
pub const CPFT_SUBMIT_BUTTON: i32 = 0x9;
pub const CPGSR_NO_CREDENTIAL_NOT_FINISHED: i32 = 0x0;
pub const CPGSR_NO_CREDENTIAL_FINISHED: i32 = 0x1;
pub const CPGSR_RETURN_CREDENTIAL_FINISHED: i32 = 0x2;
pub const CPGSR_RETURN_NO_CREDENTIAL_FINISHED: i32 = 0x3;
pub const CPSI_NONE: i32 = 0x0;
pub const CPSI_ERROR: i32 = 0x1;
pub const CPSI_WARNING: i32 = 0x2;
pub const CPSI_SUCCESS: i32 = 0x3;
pub const CPUS_INVALID: i32 = 0x0;
pub const CPUS_LOGON: i32 = 0x1;
pub const CPUS_UNLOCK_WORKSTATION: i32 = 0x2;
pub const CPUS_CHANGE_PASSWORD: i32 = 0x3;
pub const CPUS_CREDUI: i32 = 0x4;
pub const CPUS_PLAP: i32 = 0x5;
pub struct CScriptErrorList {
}
pub struct Cursorinfo {
    pub cbSize: u32,
    pub flags: u32,
    pub hCursor: HCURSOR,
    pub ptScreenPos: POINT,
}
pub const CURSOR_SHOWING: u32 = 0x1;
pub const CURSOR_SUPPRESSED: u32 = 0x2;
pub struct Cursorshape {
    pub xHotSpot: i32,
    pub yHotSpot: i32,
    pub cx: i32,
    pub cy: i32,
    pub cbWidth: i32,
    pub Planes: u8,
    pub BitsPixel: u8,
}
pub struct Cwpretstruct {
    pub lResult: LRESULT,
    pub lParam: LPARAM,
    pub wParam: WPARAM,
    pub message: u32,
    pub hwnd: HWND,
}
pub struct Cwpstruct {
    pub lParam: LPARAM,
    pub wParam: WPARAM,
    pub message: u32,
    pub hwnd: HWND,
}
pub const CWP_ALL: u32 = 0x0;
pub const CWP_SKIPINVISIBLE: u32 = 0x1;
pub const CWP_SKIPDISABLED: u32 = 0x2;
pub const CWP_SKIPTRANSPARENT: u32 = 0x4;
pub const CSC_UPDATECOMMANDS: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const CSC_NAVIGATEFORWARD: i32 = 0x1;
pub const CSC_NAVIGATEBACK: i32 = 0x2;
pub struct ConflictFolder {
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatedHdc {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl CreatedHdc {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for CreatedHdc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub struct DatablockHeader {
    pub cbSize: u32,
    pub dwSignature: u32,
}
pub const DOGIF_DEFAULT: i32 = 0x0;
pub const DOGIF_TRAVERSE_LINK: i32 = 0x1;
pub const DOGIF_NO_HDROP: i32 = 0x2;
pub const DOGIF_NO_URL: i32 = 0x4;
pub const DOGIF_ONLY_IF_ONE: i32 = 0x8;
pub struct Datetimepickerinfo {
    pub cbSize: u32,
    pub rcCheck: RECT,
    pub stateCheck: u32,
    pub rcButton: RECT,
    pub stateButton: u32,
    pub hwndEdit: HWND,
    pub hwndUD: HWND,
    pub hwndDropDown: HWND,
}
pub const LAYOUT_BITMAPORIENTATIONPRESERVED: u32 = 0x8;
pub const LAYOUT_RTL: u32 = 0x1;
pub struct Debughookinfo {
    pub idThread: u32,
    pub idThreadInstaller: u32,
    pub lParam: LPARAM,
    pub wParam: WPARAM,
    pub code: i32,
}
pub const DSFT_DETECT: i32 = 0x1;
pub const DSFT_PRIVATE: i32 = 0x2;
pub const DSFT_PUBLIC: i32 = 0x3;
pub const DFMR_DEFAULT: i32 = 0x0;
pub const DFMR_NO_STATIC_VERBS: i32 = 0x8;
pub const DFMR_STATIC_VERBS_ONLY: i32 = 0x10;
pub const DFMR_NO_RESOURCE_VERBS: i32 = 0x20;
pub const DFMR_OPTIN_HANDLERS_ONLY: i32 = 0x40;
pub const DFMR_RESOURCE_AND_FOLDER_VERBS_ONLY: i32 = 0x80;
pub const DFMR_USE_SPECIFIED_HANDLERS: i32 = 0x100;
pub const DFMR_USE_SPECIFIED_VERBS: i32 = 0x200;
pub const DFMR_NO_ASYNC_VERBS: i32 = 0x400;
pub const DFMR_NO_NATIVECPU_VERBS: i32 = 0x800;
pub const DFMR_NO_NONWOW_VERBS: i32 = 0x1000;
pub const DEFSHAREID_USERS: i32 = 0x1;
pub const DEFSHAREID_PUBLIC: i32 = 0x2;
pub struct Delegateitemid {
    pub cbSize: u16,
    pub wOuter: u16,
    pub cbInner: u16,
    pub rgb: [u8; 1],
}
pub struct Deleteitemstruct {
    pub CtlType: u32,
    pub CtlID: u32,
    pub itemID: u32,
    pub hwndItem: HWND,
    pub itemData: usize,
}
pub struct Designvector {
    pub dvReserved: u32,
    pub dvNumAxes: u32,
    pub dvValues: [i32; 16],
}
pub const DBID_BANDINFOCHANGED: i32 = 0x0;
pub const DBID_SHOWONLY: i32 = 0x1;
pub const DBID_MAXIMIZEBAND: i32 = 0x2;
pub const DBID_PUSHCHEVRON: i32 = 0x3;
pub const DBID_DELAYINIT: i32 = 0x4;
pub const DBID_FINISHINIT: i32 = 0x5;
pub const DBID_SETWINDOWTHEME: i32 = 0x6;
pub const DBID_PERMITAUTOHIDE: i32 = 0x7;
pub struct Deskbandinfo {
    pub dwMask: u32,
    pub ptMinSize: POINTL,
    pub ptMaxSize: POINTL,
    pub ptIntegral: POINTL,
    pub ptActual: POINTL,
    pub wszTitle: [u8; 256],
    pub dwModeFlags: u32,
    pub crBkgnd: u32,
}
pub const DSD_FORWARD: i32 = 0x0;
pub const DSD_BACKWARD: i32 = 0x1;
pub const DSO_SHUFFLEIMAGES: i32 = 0x1;
pub const DSS_ENABLED: i32 = 0x1;
pub const DSS_SLIDESHOW: i32 = 0x2;
pub const DSS_DISABLED_BY_REMOTE_SESSION: i32 = 0x4;
pub const DWPOS_CENTER: i32 = 0x0;
pub const DWPOS_TILE: i32 = 0x1;
pub const DWPOS_STRETCH: i32 = 0x2;
pub const DWPOS_FIT: i32 = 0x3;
pub const DWPOS_FILL: i32 = 0x4;
pub const DWPOS_SPAN: i32 = 0x5;
pub struct Devmodea {
    pub dmDeviceName: [u8; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: u32,
    pub Anonymous1: DEVMODEA_0,
    pub dmColor: i16,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: i16,
    pub dmCollate: i16,
    pub dmFormName: [u8; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: _Anonymous2_e__Struct_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Devmodea0 {
    pub field0: _Anonymous1_e__Struct,
    pub field1: _Anonymous2_e__Struct,
}
pub struct Anonymous1EStruct {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
pub struct Anonymous2EStruct {
    pub dmPosition: POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
pub struct Devmodew {
    pub dmDeviceName: [u8; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: u32,
    pub Anonymous1: DEVMODEW_2,
    pub dmColor: i16,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: i16,
    pub dmCollate: i16,
    pub dmFormName: [u8; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: _Anonymous2_e__Struct_3,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Devmodew2 {
    pub field0: _Anonymous1_e__Struct,
    pub field1: _Anonymous2_e__Struct,
}
pub struct Anonymous1EStruct {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
pub struct Anonymous2EStruct {
    pub dmPosition: POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
pub const DFCS_CAPTIONCLOSE: u32 = 0x0;
pub const DFCS_CAPTIONMIN: u32 = 0x1;
pub const DFCS_CAPTIONMAX: u32 = 0x2;
pub const DFCS_CAPTIONRESTORE: u32 = 0x3;
pub const DFCS_CAPTIONHELP: u32 = 0x4;
pub const DFCS_MENUARROW: u32 = 0x0;
pub const DFCS_MENUCHECK: u32 = 0x1;
pub const DFCS_MENUBULLET: u32 = 0x2;
pub const DFCS_MENUARROWRIGHT: u32 = 0x4;
pub const DFCS_SCROLLUP: u32 = 0x0;
pub const DFCS_SCROLLDOWN: u32 = 0x1;
pub const DFCS_SCROLLLEFT: u32 = 0x2;
pub const DFCS_SCROLLRIGHT: u32 = 0x3;
pub const DFCS_SCROLLCOMBOBOX: u32 = 0x5;
pub const DFCS_SCROLLSIZEGRIP: u32 = 0x8;
pub const DFCS_SCROLLSIZEGRIPRIGHT: u32 = 0x10;
pub const DFCS_BUTTONCHECK: u32 = 0x0;
pub const DFCS_BUTTONRADIOIMAGE: u32 = 0x1;
pub const DFCS_BUTTONRADIOMASK: u32 = 0x2;
pub const DFCS_BUTTONRADIO: u32 = 0x4;
pub const DFCS_BUTTON3STATE: u32 = 0x8;
pub const DFCS_BUTTONPUSH: u32 = 0x10;
pub const DFCS_INACTIVE: u32 = 0x100;
pub const DFCS_PUSHED: u32 = 0x200;
pub const DFCS_CHECKED: u32 = 0x400;
pub const DFCS_TRANSPARENT: u32 = 0x800;
pub const DFCS_HOT: u32 = 0x1000;
pub const DFCS_ADJUSTRECT: u32 = 0x2000;
pub const DFCS_FLAT: u32 = 0x4000;
pub const DFCS_MONO: u32 = 0x8000;
pub const DFC_CAPTION: u32 = 0x1;
pub const DFC_MENU: u32 = 0x2;
pub const DFC_SCROLL: u32 = 0x3;
pub const DFC_BUTTON: u32 = 0x4;
pub const DFC_POPUPMENU: u32 = 0x5;
pub struct DfConstraint {
}
pub const DFM_CMD_DELETE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const DFM_CMD_MOVE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE;
pub const DFM_CMD_COPY: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD;
pub const DFM_CMD_LINK: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC;
pub const DFM_CMD_PROPERTIES: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFB;
pub const DFM_CMD_NEWFOLDER: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFA;
pub const DFM_CMD_PASTE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF9;
pub const DFM_CMD_VIEWLIST: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8;
pub const DFM_CMD_VIEWDETAILS: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7;
pub const DFM_CMD_PASTELINK: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF6;
pub const DFM_CMD_PASTESPECIAL: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5;
pub const DFM_CMD_MODALPROP: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4;
pub const DFM_CMD_RENAME: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF3;
pub const DFM_MERGECONTEXTMENU: i32 = 0x1;
pub const DFM_INVOKECOMMAND: i32 = 0x2;
pub const DFM_GETHELPTEXT: i32 = 0x5;
pub const DFM_WM_MEASUREITEM: i32 = 0x6;
pub const DFM_WM_DRAWITEM: i32 = 0x7;
pub const DFM_WM_INITMENUPOPUP: i32 = 0x8;
pub const DFM_VALIDATECMD: i32 = 0x9;
pub const DFM_MERGECONTEXTMENU_TOP: i32 = 0xA;
pub const DFM_GETHELPTEXTW: i32 = 0xB;
pub const DFM_INVOKECOMMANDEX: i32 = 0xC;
pub const DFM_MAPCOMMANDNAME: i32 = 0xD;
pub const DFM_GETDEFSTATICID: i32 = 0xE;
pub const DFM_GETVERBW: i32 = 0xF;
pub const DFM_GETVERBA: i32 = 0x10;
pub const DFM_MERGECONTEXTMENU_BOTTOM: i32 = 0x11;
pub const DFM_MODIFYQCMFLAGS: i32 = 0x12;
pub struct Dibsection<'a> {
    pub dsBm: BITMAP<'a>,
    pub dsBmih: BITMAPINFOHEADER,
    pub dsBitfields: [u32; 3],
    pub dshSection: HANDLE,
    pub dsOffset: u32,
}
pub const DIB_RGB_COLORS: u32 = 0x0;
pub const DIB_PAL_COLORS: u32 = 0x1;
pub const DISPLAYCONFIG_COLOR_ENCODING_RGB: i32 = 0x0;
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR444: i32 = 0x1;
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR422: i32 = 0x2;
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR420: i32 = 0x3;
pub const DISPLAYCONFIG_COLOR_ENCODING_INTENSITY: i32 = 0x4;
pub const DISPLAYCONFIG_COLOR_ENCODING_FORCE_UINT32: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub struct DisplayDevicea {
    pub cb: u32,
    pub DeviceName: [CHAR; 32],
    pub DeviceString: [CHAR; 128],
    pub StateFlags: u32,
    pub DeviceID: [CHAR; 128],
    pub DeviceKey: [CHAR; 128],
}
pub struct DisplayDevicew {
    pub cb: u32,
    pub DeviceName: [u8; 32],
    pub DeviceString: [u8; 128],
    pub StateFlags: u32,
    pub DeviceID: [u8; 128],
    pub DeviceKey: [u8; 128],
}
pub const DEVICE_PRIMARY: i32 = 0x0;
pub const DEVICE_IMMERSIVE: i32 = 0x1;
pub const DISP_CHANGE_SUCCESSFUL: i32 = 0x0;
pub const DISP_CHANGE_RESTART: i32 = 0x1;
pub const DISP_CHANGE_FAILED: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const DISP_CHANGE_BADMODE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE;
pub const DISP_CHANGE_NOTUPDATED: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD;
pub const DISP_CHANGE_BADFLAGS: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC;
pub const DISP_CHANGE_BADPARAM: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFB;
pub const DISP_CHANGE_BADDUALVIEW: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFA;
pub const DI_MASK: u32 = 0x1;
pub const DI_IMAGE: u32 = 0x2;
pub const DI_NORMAL: u32 = 0x3;
pub const DI_COMPAT: u32 = 0x4;
pub const DI_DEFAULTSIZE: u32 = 0x8;
pub const DI_NOMIRROR: u32 = 0x10;
pub struct Dlgitemtemplate {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
    pub id: u16,
}
pub struct Dlgtemplate {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub cdit: u16,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
}
pub const BST_CHECKED: u32 = 0x1;
pub const BST_INDETERMINATE: u32 = 0x2;
pub const BST_UNCHECKED: u32 = 0x0;
pub const DDL_ARCHIVE: u32 = 0x20;
pub const DDL_DIRECTORY: u32 = 0x10;
pub const DDL_DRIVES: u32 = 0x4000;
pub const DDL_EXCLUSIVE: u32 = 0x8000;
pub const DDL_HIDDEN: u32 = 0x2;
pub const DDL_READONLY: u32 = 0x1;
pub const DDL_READWRITE: u32 = 0x0;
pub const DDL_SYSTEM: u32 = 0x4;
pub const DDL_POSTMSGS: u32 = 0x2000;
pub struct Dllversioninfo {
    pub cbSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformID: u32,
}
pub struct Dllversioninfo2 {
    pub info1: DLLVERSIONINFO,
    pub dwFlags: u32,
    pub ullVersion: u64,
}
pub const DPAMM_MERGE: u32 = 0x1;
pub const DPAMM_DELETE: u32 = 0x2;
pub const DPAMM_INSERT: u32 = 0x3;
pub struct Dpastreaminfo<'a> {
    pub iPos: i32,
    pub pvItem: &'a mut todo_void,
}
pub struct Draginfoa<'a> {
    pub uSize: u32,
    pub pt: POINT,
    pub fNC: BOOL,
    pub lpFileList: &'a mut Cow<'a, str>,
    pub grfKeyState: u32,
}
pub struct Draginfow<'a> {
    pub uSize: u32,
    pub pt: POINT,
    pub fNC: BOOL,
    pub lpFileList: &'a mut Cow<'a, OsStr>,
    pub grfKeyState: u32,
}
pub struct Draglistinfo {
    pub uNotification: u32,
    pub hWnd: HWND,
    pub ptCursor: POINT,
}
pub const DL_BEGINDRAG: u32 = 0x485;
pub const DL_CANCELDRAG: u32 = 0x488;
pub const DL_DRAGGING: u32 = 0x486;
pub const DL_DROPPED: u32 = 0x487;
pub const BDR_RAISEDOUTER: u32 = 0x1;
pub const BDR_SUNKENOUTER: u32 = 0x2;
pub const BDR_RAISEDINNER: u32 = 0x4;
pub const BDR_SUNKENINNER: u32 = 0x8;
pub const BDR_OUTER: u32 = 0x3;
pub const BDR_INNER: u32 = 0xC;
pub const BDR_RAISED: u32 = 0x5;
pub const BDR_SUNKEN: u32 = 0xA;
pub const EDGE_RAISED: u32 = 0x5;
pub const EDGE_SUNKEN: u32 = 0xA;
pub const EDGE_ETCHED: u32 = 0x6;
pub const EDGE_BUMP: u32 = 0x9;
pub struct Drawitemstruct {
    pub CtlType: u32,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemAction: u32,
    pub itemState: u32,
    pub hwndItem: HWND,
    pub hDC: HDC,
    pub rcItem: RECT,
    pub itemData: usize,
}
pub const ODT_BUTTON: u32 = 0x4;
pub const ODT_COMBOBOX: u32 = 0x3;
pub const ODT_LISTBOX: u32 = 0x2;
pub const ODT_LISTVIEW: u32 = 0x66;
pub const ODT_MENU: u32 = 0x1;
pub const ODT_STATIC: u32 = 0x5;
pub const ODT_TAB: u32 = 0x65;
pub const DST_COMPLEX: u32 = 0x0;
pub const DST_TEXT: u32 = 0x1;
pub const DST_PREFIXTEXT: u32 = 0x2;
pub const DST_ICON: u32 = 0x3;
pub const DST_BITMAP: u32 = 0x4;
pub const DSS_NORMAL: u32 = 0x0;
pub const DSS_UNION: u32 = 0x10;
pub const DSS_DISABLED: u32 = 0x20;
pub const DSS_MONO: u32 = 0x80;
pub const DSS_HIDEPREFIX: u32 = 0x200;
pub const DSS_PREFIXONLY: u32 = 0x400;
pub const DSS_RIGHT: u32 = 0x8000;
pub struct Drawtextparams {
    pub cbSize: u32,
    pub iTabLength: i32,
    pub iLeftMargin: i32,
    pub iRightMargin: i32,
    pub uiLengthDrawn: u32,
}
pub const DC_ACTIVE: u32 = 0x1;
pub const DC_BUTTONS: u32 = 0x1000;
pub const DC_GRADIENT: u32 = 0x20;
pub const DC_ICON: u32 = 0x4;
pub const DC_INBUTTON: u32 = 0x10;
pub const DC_SMALLCAP: u32 = 0x2;
pub const DC_TEXT: u32 = 0x8;
pub const BF_ADJUST: u32 = 0x2000;
pub const BF_BOTTOM: u32 = 0x8;
pub const BF_BOTTOMLEFT: u32 = 0x9;
pub const BF_BOTTOMRIGHT: u32 = 0xC;
pub const BF_DIAGONAL: u32 = 0x10;
pub const BF_DIAGONAL_ENDBOTTOMLEFT: u32 = 0x19;
pub const BF_DIAGONAL_ENDBOTTOMRIGHT: u32 = 0x1C;
pub const BF_DIAGONAL_ENDTOPLEFT: u32 = 0x13;
pub const BF_DIAGONAL_ENDTOPRIGHT: u32 = 0x16;
pub const BF_FLAT: u32 = 0x4000;
pub const BF_LEFT: u32 = 0x1;
pub const BF_MIDDLE: u32 = 0x800;
pub const BF_MONO: u32 = 0x8000;
pub const BF_RECT: u32 = 0xF;
pub const BF_RIGHT: u32 = 0x4;
pub const BF_SOFT: u32 = 0x1000;
pub const BF_TOP: u32 = 0x2;
pub const BF_TOPLEFT: u32 = 0x3;
pub const BF_TOPRIGHT: u32 = 0x6;
pub const DT_BOTTOM: u32 = 0x8;
pub const DT_CALCRECT: u32 = 0x400;
pub const DT_CENTER: u32 = 0x1;
pub const DT_EDITCONTROL: u32 = 0x2000;
pub const DT_END_ELLIPSIS: u32 = 0x8000;
pub const DT_EXPANDTABS: u32 = 0x40;
pub const DT_EXTERNALLEADING: u32 = 0x200;
pub const DT_HIDEPREFIX: u32 = 0x100000;
pub const DT_INTERNAL: u32 = 0x1000;
pub const DT_LEFT: u32 = 0x0;
pub const DT_MODIFYSTRING: u32 = 0x10000;
pub const DT_NOCLIP: u32 = 0x100;
pub const DT_NOFULLWIDTHCHARBREAK: u32 = 0x80000;
pub const DT_NOPREFIX: u32 = 0x800;
pub const DT_PATH_ELLIPSIS: u32 = 0x4000;
pub const DT_PREFIXONLY: u32 = 0x200000;
pub const DT_RIGHT: u32 = 0x2;
pub const DT_RTLREADING: u32 = 0x20000;
pub const DT_SINGLELINE: u32 = 0x20;
pub const DT_TABSTOP: u32 = 0x80;
pub const DT_TOP: u32 = 0x0;
pub const DT_VCENTER: u32 = 0x4;
pub const DT_WORDBREAK: u32 = 0x10;
pub const DT_WORD_ELLIPSIS: u32 = 0x40000;
pub const DTPB_WINDOWDC: u32 = 0x1;
pub const DTPB_USECTLCOLORSTATIC: u32 = 0x2;
pub const DTPB_USEERASEBKGND: u32 = 0x4;
pub struct Dropdescription {
    pub type: i32,
    pub szMessage: [u8; 260],
    pub szInsert: [u8; 260],
}
pub struct Dropfiles {
    pub pFiles: u32,
    pub pt: POINT,
    pub fNC: BOOL,
    pub fWide: BOOL,
}
pub const DROPIMAGE_INVALID: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const DROPIMAGE_NONE: i32 = 0x0;
pub const DROPIMAGE_COPY: i32 = 0x1;
pub const DROPIMAGE_MOVE: i32 = 0x2;
pub const DROPIMAGE_LINK: i32 = 0x4;
pub const DROPIMAGE_LABEL: i32 = 0x6;
pub const DROPIMAGE_WARNING: i32 = 0x7;
pub const DROPIMAGE_NOIMAGE: i32 = 0x8;
pub struct Dropstruct {
    pub hwndSource: HWND,
    pub hwndSink: HWND,
    pub wFmt: u32,
    pub dwData: usize,
    pub ptDrop: POINT,
    pub dwControlData: u32,
}
pub const DSH_ALLOWDROPDESCRIPTIONTEXT: i32 = 0x1;
pub struct Dsktlsystemtime {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
    pub wResult: u16,
}
pub struct DShellFolderViewEvents {
}
pub struct DShellNameSpaceEvents {
}
pub struct DShellWindowsEvents {
}
pub struct Dtbgopts {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub rcClip: RECT,
}
pub struct Dttopts {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub crText: u32,
    pub crBorder: u32,
    pub crShadow: u32,
    pub iTextShadowType: i32,
    pub ptShadowOffset: POINT,
    pub iBorderSize: i32,
    pub iFontPropId: i32,
    pub iColorPropId: i32,
    pub iStateId: i32,
    pub fApplyOverlay: BOOL,
    pub iGlowSize: i32,
    pub pfnDrawTextCallback: todo_fn,
    pub lParam: LPARAM,
}
pub const DUPLICATE_CLOSE_SOURCE: u32 = 0x1;
pub const DUPLICATE_SAME_ACCESS: u32 = 0x2;
pub struct DWebBrowserEvents {
}
pub struct DWebBrowserEvents2 {
}
pub struct DefFolderMenu {
}
pub struct DesktopGadget {
}
pub struct DesktopWallpaper {
}
pub struct DestinationList {
}
pub struct DocPropShellExtension {
}
pub struct DriveSizeCategorizer {
}
pub struct DriveTypeCategorizer {
}
pub const EC_ENDOFLINE_DETECTFROMCONTENT: i32 = 0x0;
pub const EC_ENDOFLINE_CRLF: i32 = 0x1;
pub const EC_ENDOFLINE_CR: i32 = 0x2;
pub const EC_ENDOFLINE_LF: i32 = 0x3;
pub const ECHUIM_DESKTOP: i32 = 0x0;
pub const ECHUIM_IMMERSIVE: i32 = 0x1;
pub const ECHUIM_SYSTEM_LAUNCHER: i32 = 0x2;
pub const EC_SEARCHWEB_ENTRYPOINT_EXTERNAL: i32 = 0x0;
pub const EC_SEARCHWEB_ENTRYPOINT_CONTEXTMENU: i32 = 0x1;
pub const EGK_TOUCH: i32 = 0x0;
pub const EGK_KEYBOARD: i32 = 0x1;
pub const EGK_MOUSE: i32 = 0x2;
pub struct Editballoontip<'a> {
    pub cbStruct: u32,
    pub pszTitle: &'a Cow<'a, OsStr>,
    pub pszText: &'a Cow<'a, OsStr>,
    pub ttiIcon: u32,
}
pub const TTI_ERROR: u32 = 0x3;
pub const TTI_INFO: u32 = 0x1;
pub const TTI_NONE: u32 = 0x0;
pub const TTI_WARNING: u32 = 0x2;
pub const TTI_INFO_LARGE: u32 = 0x4;
pub const TTI_WARNING_LARGE: u32 = 0x5;
pub const TTI_ERROR_LARGE: u32 = 0x6;
pub const EDIT_CONTROL_FEATURE_ENTERPRISE_DATA_PROTECTION_PASTE_SUPPORT: i32 = 0x0;
pub const EDIT_CONTROL_FEATURE_PASTE_NOTIFICATIONS: i32 = 0x1;
pub const EMBED_PREVIEWPRINT: u32 = 0x1;
pub const EMBED_EDITABLE: u32 = 0x2;
pub const EMBED_INSTALLABLE: u32 = 0x3;
pub const EMBED_NOEMBEDDING: u32 = 0x4;
pub const CHARSET_UNICODE: u32 = 0x1;
pub const CHARSET_SYMBOL: u32 = 0x2;
pub const EMP_MARKUPTEXT: i32 = 0x1;
pub struct Emr {
    pub iType: u32,
    pub nSize: u32,
}
pub struct Emralphablend {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
pub struct Emranglearc {
    pub emr: EMR,
    pub ptlCenter: POINTL,
    pub nRadius: u32,
    pub eStartAngle: f32,
    pub eSweepAngle: f32,
}
pub struct Emrarc {
    pub emr: EMR,
    pub rclBox: RECTL,
    pub ptlStart: POINTL,
    pub ptlEnd: POINTL,
}
pub struct Emrbitblt {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
}
pub struct Emrcreatebrushindirect {
    pub emr: EMR,
    pub ihBrush: u32,
    pub lb: LOGBRUSH32,
}
pub struct Emrcreatedibpatternbrushpt {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
pub struct Emrcreatemonobrush {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
pub struct Emrcreatepalette {
    pub emr: EMR,
    pub ihPal: u32,
    pub lgpl: LOGPALETTE,
}
pub struct Emrcreatepen {
    pub emr: EMR,
    pub ihPen: u32,
    pub lopn: LOGPEN,
}
pub struct Emrellipse {
    pub emr: EMR,
    pub rclBox: RECTL,
}
pub struct Emreof {
    pub emr: EMR,
    pub nPalEntries: u32,
    pub offPalEntries: u32,
    pub nSizeLast: u32,
}
pub struct Emrexcludecliprect {
    pub emr: EMR,
    pub rclClip: RECTL,
}
pub struct Emrextcreatefontindirectw {
    pub emr: EMR,
    pub ihFont: u32,
    pub elfw: EXTLOGFONTW,
}
pub struct Emrextcreatepen {
    pub emr: EMR,
    pub ihPen: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
    pub elp: EXTLOGPEN32,
}
pub struct Emrextescape {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
pub struct Emrextfloodfill {
    pub emr: EMR,
    pub ptlStart: POINTL,
    pub crColor: u32,
    pub iMode: u32,
}
pub struct Emrextselectcliprgn {
    pub emr: EMR,
    pub cbRgnData: u32,
    pub iMode: u32,
    pub RgnData: [u8; 1],
}
pub struct Emrexttextouta {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub emrtext: EMRTEXT,
}
pub struct Emrfillpath {
    pub emr: EMR,
    pub rclBounds: RECTL,
}
pub struct Emrfillrgn {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub RgnData: [u8; 1],
}
pub struct Emrformat {
    pub dSignature: u32,
    pub nVersion: u32,
    pub cbData: u32,
    pub offData: u32,
}
pub struct Emrframergn {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub szlStroke: SIZE,
    pub RgnData: [u8; 1],
}
pub struct Emrgdicomment {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
pub struct Emrglsboundedrecord {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub cbData: u32,
    pub Data: [u8; 1],
}
pub struct Emrglsrecord {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
pub struct Emrgradientfill {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub nVer: u32,
    pub nTri: u32,
    pub ulMode: u32,
    pub Ver: [TRIVERTEX; 1],
}
pub struct Emrinvertrgn {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub cbRgnData: u32,
    pub RgnData: [u8; 1],
}
pub struct Emrlineto {
    pub emr: EMR,
    pub ptl: POINTL,
}
pub struct Emrmaskblt {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub xMask: i32,
    pub yMask: i32,
    pub iUsageMask: u32,
    pub offBmiMask: u32,
    pub cbBmiMask: u32,
    pub offBitsMask: u32,
    pub cbBitsMask: u32,
}
pub struct Emrmodifyworldtransform {
    pub emr: EMR,
    pub xform: XFORM,
    pub iMode: u32,
}
pub struct Emrnamedescape {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbDriver: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
pub struct Emroffsetcliprgn {
    pub emr: EMR,
    pub ptlOffset: POINTL,
}
pub struct Emrplgblt {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub aptlDest: [POINTL; 3],
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub xMask: i32,
    pub yMask: i32,
    pub iUsageMask: u32,
    pub offBmiMask: u32,
    pub cbBmiMask: u32,
    pub offBitsMask: u32,
    pub cbBitsMask: u32,
}
pub struct Emrpolydraw {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub cptl: u32,
    pub aptl: [POINTL; 1],
    pub abTypes: [u8; 1],
}
pub struct Emrpolydraw16 {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub cpts: u32,
    pub apts: [POINTS; 1],
    pub abTypes: [u8; 1],
}
pub struct Emrpolyline {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub cptl: u32,
    pub aptl: [POINTL; 1],
}
pub struct Emrpolyline16 {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub cpts: u32,
    pub apts: [POINTS; 1],
}
pub struct Emrpolypolyline {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub nPolys: u32,
    pub cptl: u32,
    pub aPolyCounts: [u32; 1],
    pub aptl: [POINTL; 1],
}
pub struct Emrpolypolyline16 {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub nPolys: u32,
    pub cpts: u32,
    pub aPolyCounts: [u32; 1],
    pub apts: [POINTS; 1],
}
pub struct Emrpolytextouta {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub cStrings: i32,
    pub aemrtext: [EMRTEXT; 1],
}
pub struct Emrresizepalette {
    pub emr: EMR,
    pub ihPal: u32,
    pub cEntries: u32,
}
pub struct Emrrestoredc {
    pub emr: EMR,
    pub iRelative: i32,
}
pub struct Emrroundrect {
    pub emr: EMR,
    pub rclBox: RECTL,
    pub szlCorner: SIZE,
}
pub struct Emrscaleviewportextex {
    pub emr: EMR,
    pub xNum: i32,
    pub xDenom: i32,
    pub yNum: i32,
    pub yDenom: i32,
}
pub struct Emrselectclippath {
    pub emr: EMR,
    pub iMode: u32,
}
pub struct Emrselectobject {
    pub emr: EMR,
    pub ihObject: u32,
}
pub struct Emrselectpalette {
    pub emr: EMR,
    pub ihPal: u32,
}
pub struct Emrsetarcdirection {
    pub emr: EMR,
    pub iArcDirection: u32,
}
pub struct Emrsetcoloradjustment {
    pub emr: EMR,
    pub ColorAdjustment: COLORADJUSTMENT,
}
pub struct Emrsetcolorspace {
    pub emr: EMR,
    pub ihCS: u32,
}
pub struct Emrsetdibitstodevice {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub iStartScan: u32,
    pub cScans: u32,
}
pub struct Emrseticmprofile {
    pub emr: EMR,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
pub struct Emrsetmapperflags {
    pub emr: EMR,
    pub dwFlags: u32,
}
pub struct Emrsetmiterlimit {
    pub emr: EMR,
    pub eMiterLimit: f32,
}
pub struct Emrsetpaletteentries {
    pub emr: EMR,
    pub ihPal: u32,
    pub iStart: u32,
    pub cEntries: u32,
    pub aPalEntries: [PALETTEENTRY; 1],
}
pub struct Emrsetpixelv {
    pub emr: EMR,
    pub ptlPixel: POINTL,
    pub crColor: u32,
}
pub struct Emrsettextcolor {
    pub emr: EMR,
    pub crColor: u32,
}
pub struct Emrsetviewportextex {
    pub emr: EMR,
    pub szlExtent: SIZE,
}
pub struct Emrsetviewportorgex {
    pub emr: EMR,
    pub ptlOrigin: POINTL,
}
pub struct Emrsetworldtransform {
    pub emr: EMR,
    pub xform: XFORM,
}
pub struct Emrstretchblt {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
pub struct Emrstretchdibits {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub dwRop: u32,
    pub cxDest: i32,
    pub cyDest: i32,
}
pub struct Emrtext {
    pub ptlReference: POINTL,
    pub nChars: u32,
    pub offString: u32,
    pub fOptions: u32,
    pub rcl: RECTL,
    pub offDx: u32,
}
pub struct Emrtransparentblt {
    pub emr: EMR,
    pub rclBounds: RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
pub const ESB_DISABLE_BOTH: u32 = 0x3;
pub const ESB_DISABLE_DOWN: u32 = 0x2;
pub const ESB_DISABLE_LEFT: u32 = 0x1;
pub const ESB_DISABLE_LTUP: u32 = 0x1;
pub const ESB_DISABLE_RIGHT: u32 = 0x2;
pub const ESB_DISABLE_RTDN: u32 = 0x2;
pub const ESB_DISABLE_UP: u32 = 0x1;
pub const ESB_ENABLE_BOTH: u32 = 0x0;
pub struct Enhmetaheader {
    pub iType: u32,
    pub nSize: u32,
    pub rclBounds: RECTL,
    pub rclFrame: RECTL,
    pub dSignature: u32,
    pub nVersion: u32,
    pub nBytes: u32,
    pub nRecords: u32,
    pub nHandles: u16,
    pub sReserved: u16,
    pub nDescription: u32,
    pub offDescription: u32,
    pub nPalEntries: u32,
    pub szlDevice: SIZE,
    pub szlMillimeters: SIZE,
    pub cbPixelFormat: u32,
    pub offPixelFormat: u32,
    pub bOpenGL: u32,
    pub szlMicrometers: SIZE,
}
pub struct Enhmetarecord {
    pub iType: u32,
    pub nSize: u32,
    pub dParm: [u32; 1],
}
pub struct Enumlogfonta {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
}
pub struct Enumlogfontexa {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfScript: [u8; 32],
}
pub struct Enumlogfontexdva {
    pub elfEnumLogfontEx: ENUMLOGFONTEXA,
    pub elfDesignVector: DESIGNVECTOR,
}
pub struct Enumlogfontexdvw {
    pub elfEnumLogfontEx: ENUMLOGFONTEXW,
    pub elfDesignVector: DESIGNVECTOR,
}
pub struct Enumlogfontexw {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfScript: [u8; 32],
}
pub struct Enumlogfontw {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
}
pub struct Enumuilang<'a> {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: &'a mut u16,
}
pub const ENUM_CURRENT_SETTINGS: u32 = 0xFFFFFFFF;
pub const ENUM_REGISTRY_SETTINGS: u32 = 0xFFFFFFFE;
pub const ETO_OPAQUE: u32 = 0x2;
pub const ETO_CLIPPED: u32 = 0x4;
pub const ETO_GLYPH_INDEX: u32 = 0x10;
pub const ETO_RTLREADING: u32 = 0x80;
pub const ETO_NUMERICSLOCAL: u32 = 0x400;
pub const ETO_NUMERICSLATIN: u32 = 0x800;
pub const ETO_IGNORELANGUAGE: u32 = 0x1000;
pub const ETO_PDY: u32 = 0x2000;
pub const ETO_REVERSE_INDEX_MAP: u32 = 0x10000;
pub struct Eventmsg {
    pub message: u32,
    pub paramL: u32,
    pub paramH: u32,
    pub time: u32,
    pub hwnd: HWND,
}
pub struct ExpDarwinLink {
    pub dbh: DATABLOCK_HEADER,
    pub szDarwinID: [CHAR; 260],
    pub szwDarwinID: [u8; 260],
}
pub struct ExpPropertystorage {
    pub cbSize: u32,
    pub dwSignature: u32,
    pub abPropertyStorage: [u8; 1],
}
pub struct ExpSpecialFolder {
    pub cbSize: u32,
    pub dwSignature: u32,
    pub idSpecialFolder: u32,
    pub cbOffset: u32,
}
pub struct ExpSzLink {
    pub cbSize: u32,
    pub dwSignature: u32,
    pub szTarget: [CHAR; 260],
    pub swzTarget: [u8; 260],
}
pub struct Extlogfonta {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
pub struct Extlogfontw {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
pub struct Extlogpen {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: u32,
    pub elpHatch: usize,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
pub struct Extlogpen32 {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: u32,
    pub elpHatch: u32,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
pub struct Extrasearch {
    pub guidSearch: GUID,
    pub wszFriendlyName: [u8; 80],
    pub wszUrl: [u8; 2084],
}
pub const FLOODFILLBORDER: u32 = 0x0;
pub const FLOODFILLSURFACE: u32 = 0x1;
pub struct EnumerableObjectCollection {
}
pub struct ExecuteFolder {
}
pub struct ExecuteUnknown {
}
pub struct ExplorerBrowser {
}
pub const FDAP_BOTTOM: i32 = 0x0;
pub const FDAP_TOP: i32 = 0x1;
pub const FDEOR_DEFAULT: i32 = 0x0;
pub const FDEOR_ACCEPT: i32 = 0x1;
pub const FDEOR_REFUSE: i32 = 0x2;
pub const FDESVR_DEFAULT: i32 = 0x0;
pub const FDESVR_ACCEPT: i32 = 0x1;
pub const FDESVR_REFUSE: i32 = 0x2;
pub const FD_CLSID: i32 = 0x1;
pub const FD_SIZEPOINT: i32 = 0x2;
pub const FD_ATTRIBUTES: i32 = 0x4;
pub const FD_CREATETIME: i32 = 0x8;
pub const FD_ACCESSTIME: i32 = 0x10;
pub const FD_WRITESTIME: i32 = 0x20;
pub const FD_FILESIZE: i32 = 0x40;
pub const FD_PROGRESSUI: i32 = 0x4000;
pub const FD_LINKUI: i32 = 0x8000;
pub const FD_UNICODE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000;
pub const FEEDBACK_TOUCH_CONTACTVISUALIZATION: i32 = 0x1;
pub const FEEDBACK_PEN_BARRELVISUALIZATION: i32 = 0x2;
pub const FEEDBACK_PEN_TAP: i32 = 0x3;
pub const FEEDBACK_PEN_DOUBLETAP: i32 = 0x4;
pub const FEEDBACK_PEN_PRESSANDHOLD: i32 = 0x5;
pub const FEEDBACK_PEN_RIGHTTAP: i32 = 0x6;
pub const FEEDBACK_TOUCH_TAP: i32 = 0x7;
pub const FEEDBACK_TOUCH_DOUBLETAP: i32 = 0x8;
pub const FEEDBACK_TOUCH_PRESSANDHOLD: i32 = 0x9;
pub const FEEDBACK_TOUCH_RIGHTTAP: i32 = 0xA;
pub const FEEDBACK_GESTURE_PRESSANDTAP: i32 = 0xB;
pub const FEEDBACK_MAX: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const FFFP_EXACTMATCH: i32 = 0x0;
pub const FFFP_NEARESTPARENTMATCH: i32 = 0x1;
pub struct Filedescriptora {
    pub dwFlags: u32,
    pub clsid: GUID,
    pub sizel: SIZE,
    pub pointl: POINTL,
    pub dwFileAttributes: u32,
    pub ftCreationTime: FILETIME,
    pub ftLastAccessTime: FILETIME,
    pub ftLastWriteTime: FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub cFileName: [CHAR; 260],
}
pub struct Filedescriptorw {
    pub dwFlags: u32,
    pub clsid: GUID,
    pub sizel: SIZE,
    pub pointl: POINTL,
    pub dwFileAttributes: u32,
    pub ftCreationTime: FILETIME,
    pub ftLastAccessTime: FILETIME,
    pub ftLastWriteTime: FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub cFileName: [u8; 260],
}
pub struct Filegroupdescriptora {
    pub cItems: u32,
    pub fgd: [FILEDESCRIPTORA; 1],
}
pub struct Filegroupdescriptorw {
    pub cItems: u32,
    pub fgd: [FILEDESCRIPTORW; 1],
}
pub const FOS_OVERWRITEPROMPT: u32 = 0x2;
pub const FOS_STRICTFILETYPES: u32 = 0x4;
pub const FOS_NOCHANGEDIR: u32 = 0x8;
pub const FOS_PICKFOLDERS: u32 = 0x20;
pub const FOS_FORCEFILESYSTEM: u32 = 0x40;
pub const FOS_ALLNONSTORAGEITEMS: u32 = 0x80;
pub const FOS_NOVALIDATE: u32 = 0x100;
pub const FOS_ALLOWMULTISELECT: u32 = 0x200;
pub const FOS_PATHMUSTEXIST: u32 = 0x800;
pub const FOS_FILEMUSTEXIST: u32 = 0x1000;
pub const FOS_CREATEPROMPT: u32 = 0x2000;
pub const FOS_SHAREAWARE: u32 = 0x4000;
pub const FOS_NOREADONLYRETURN: u32 = 0x8000;
pub const FOS_NOTESTFILECREATE: u32 = 0x10000;
pub const FOS_HIDEMRUPLACES: u32 = 0x20000;
pub const FOS_HIDEPINNEDPLACES: u32 = 0x40000;
pub const FOS_NODEREFERENCELINKS: u32 = 0x100000;
pub const FOS_OKBUTTONNEEDSINTERACTION: u32 = 0x200000;
pub const FOS_DONTADDTORECENT: u32 = 0x2000000;
pub const FOS_FORCESHOWHIDDEN: u32 = 0x10000000;
pub const FOS_DEFAULTNOMINIMODE: u32 = 0x20000000;
pub const FOS_FORCEPREVIEWPANEON: u32 = 0x40000000;
pub const FOS_SUPPORTSTREAMABLEITEMS: u32 = 0x80000000;
pub struct Filetime {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
pub const FTA_NONE: i32 = 0x0;
pub const FTA_EXCLUDE: i32 = 0x1;
pub const FTA_SHOW: i32 = 0x2;
pub const FTA_HAS_EXTENSION: i32 = 0x4;
pub const FTA_NO_EDIT: i32 = 0x8;
pub const FTA_NO_REMOVE: i32 = 0x10;
pub const FTA_NO_NEW_VERB: i32 = 0x20;
pub const FTA_NO_EDIT_VERB: i32 = 0x40;
pub const FTA_NO_REMOVE_VERB: i32 = 0x80;
pub const FTA_NO_EDIT_DESC: i32 = 0x100;
pub const FTA_NO_EDIT_ICON: i32 = 0x200;
pub const FTA_NO_EDIT_DFLT: i32 = 0x400;
pub const FTA_NO_EDIT_VERB_CMD: i32 = 0x800;
pub const FTA_NO_EDIT_VERB_EXE: i32 = 0x1000;
pub const FTA_NO_DDE: i32 = 0x2000;
pub const FTA_NO_EDIT_MIME: i32 = 0x8000;
pub const FTA_OPEN_IS_SAFE: i32 = 0x10000;
pub const FTA_ALWAYS_UNSAFE: i32 = 0x20000;
pub const FTA_NO_RECENT_DOCS: i32 = 0x100000;
pub const FTA_SAFE_FOR_ELEVATION: i32 = 0x200000;
pub const FTA_ALWAYS_USE_DIRECT_INVOKE: i32 = 0x400000;
pub struct FileAttributesArray {
    pub cItems: u32,
    pub dwSumFileAttributes: u32,
    pub dwProductFileAttributes: u32,
    pub rgdwFileAttributes: [u32; 1],
}
pub const FOF2_NONE: i32 = 0x0;
pub const FOF2_MERGEFOLDERSONCOLLISION: i32 = 0x1;
pub const FUT_PLAYING: i32 = 0x0;
pub const FUT_EDITING: i32 = 0x1;
pub const FUT_GENERIC: i32 = 0x2;
pub const FT_SOLID: i32 = 0x0;
pub const FT_VERTGRADIENT: i32 = 0x1;
pub const FT_HORZGRADIENT: i32 = 0x2;
pub const FT_RADIALGRADIENT: i32 = 0x3;
pub const FT_TILEIMAGE: i32 = 0x4;
pub struct Fixed {
    pub fract: u16,
    pub value: i16,
}
pub struct Flashwinfo {
    pub cbSize: u32,
    pub hwnd: HWND,
    pub dwFlags: u32,
    pub uCount: u32,
    pub dwTimeout: u32,
}
pub const FLASHW_ALL: u32 = 0x3;
pub const FLASHW_CAPTION: u32 = 0x1;
pub const FLASHW_STOP: u32 = 0x0;
pub const FLASHW_TIMER: u32 = 0x4;
pub const FLASHW_TIMERNOFG: u32 = 0xC;
pub const FLASHW_TRAY: u32 = 0x2;
pub struct Float128 {
    pub LowPart: i64,
    pub HighPart: i64,
}
pub const FP_DEFAULT: i32 = 0x0;
pub const FP_ABOVE: i32 = 0x1;
pub const FP_BELOW: i32 = 0x2;
pub const FP_LEFT: i32 = 0x3;
pub const FP_RIGHT: i32 = 0x4;
pub const FWF_NONE: i32 = 0x0;
pub const FWF_AUTOARRANGE: i32 = 0x1;
pub const FWF_ABBREVIATEDNAMES: i32 = 0x2;
pub const FWF_SNAPTOGRID: i32 = 0x4;
pub const FWF_OWNERDATA: i32 = 0x8;
pub const FWF_BESTFITWINDOW: i32 = 0x10;
pub const FWF_DESKTOP: i32 = 0x20;
pub const FWF_SINGLESEL: i32 = 0x40;
pub const FWF_NOSUBFOLDERS: i32 = 0x80;
pub const FWF_TRANSPARENT: i32 = 0x100;
pub const FWF_NOCLIENTEDGE: i32 = 0x200;
pub const FWF_NOSCROLL: i32 = 0x400;
pub const FWF_ALIGNLEFT: i32 = 0x800;
pub const FWF_NOICONS: i32 = 0x1000;
pub const FWF_SHOWSELALWAYS: i32 = 0x2000;
pub const FWF_NOVISIBLE: i32 = 0x4000;
pub const FWF_SINGLECLICKACTIVATE: i32 = 0x8000;
pub const FWF_NOWEBVIEW: i32 = 0x10000;
pub const FWF_HIDEFILENAMES: i32 = 0x20000;
pub const FWF_CHECKSELECT: i32 = 0x40000;
pub const FWF_NOENUMREFRESH: i32 = 0x80000;
pub const FWF_NOGROUPING: i32 = 0x100000;
pub const FWF_FULLROWSELECT: i32 = 0x200000;
pub const FWF_NOFILTERS: i32 = 0x400000;
pub const FWF_NOCOLUMNHEADER: i32 = 0x800000;
pub const FWF_NOHEADERINALLVIEWS: i32 = 0x1000000;
pub const FWF_EXTENDEDTILES: i32 = 0x2000000;
pub const FWF_TRICHECKSELECT: i32 = 0x4000000;
pub const FWF_AUTOCHECKSELECT: i32 = 0x8000000;
pub const FWF_NOBROWSERVIEWSTATE: i32 = 0x10000000;
pub const FWF_SUBSETGROUPS: i32 = 0x20000000;
pub const FWF_USESEARCHFOLDER: i32 = 0x40000000;
pub const FWF_ALLOWRTLREADING: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000;
pub const FLVM_UNSPECIFIED: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const FLVM_FIRST: i32 = 0x1;
pub const FLVM_DETAILS: i32 = 0x1;
pub const FLVM_TILES: i32 = 0x2;
pub const FLVM_ICONS: i32 = 0x3;
pub const FLVM_LIST: i32 = 0x4;
pub const FLVM_CONTENT: i32 = 0x5;
pub const FLVM_LAST: i32 = 0x5;
pub struct Foldersetdata {
    pub _fs: FOLDERSETTINGS,
    pub _vidRestore: GUID,
    pub _dwViewPriority: u32,
}
pub struct Foldersettings {
    pub ViewMode: u32,
    pub fFlags: u32,
}
pub const FVM_AUTO: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const FVM_FIRST: i32 = 0x1;
pub const FVM_ICON: i32 = 0x1;
pub const FVM_SMALLICON: i32 = 0x2;
pub const FVM_LIST: i32 = 0x3;
pub const FVM_DETAILS: i32 = 0x4;
pub const FVM_THUMBNAIL: i32 = 0x5;
pub const FVM_TILE: i32 = 0x6;
pub const FVM_THUMBSTRIP: i32 = 0x7;
pub const FVM_CONTENT: i32 = 0x8;
pub const FVM_LAST: i32 = 0x8;
pub const FVO_DEFAULT: i32 = 0x0;
pub const FVO_VISTALAYOUT: i32 = 0x1;
pub const FVO_CUSTOMPOSITION: i32 = 0x2;
pub const FVO_CUSTOMORDERING: i32 = 0x4;
pub const FVO_SUPPORTHYPERLINKS: i32 = 0x8;
pub const FVO_NOANIMATIONS: i32 = 0x10;
pub const FVO_NOSCROLLTIPS: i32 = 0x20;
pub const FEM_VIEWRESULT: i32 = 0x0;
pub const FEM_NAVIGATION: i32 = 0x1;
pub const CLIP_CHARACTER_PRECIS: u32 = 0x1;
pub const CLIP_DEFAULT_PRECIS: u32 = 0x0;
pub const CLIP_DFA_DISABLE: u32 = 0x40;
pub const CLIP_EMBEDDED: u32 = 0x80;
pub const CLIP_LH_ANGLES: u32 = 0x10;
pub const CLIP_MASK: u32 = 0xF;
pub const CLIP_STROKE_PRECIS: u32 = 0x2;
pub const CLIP_TT_ALWAYS: u32 = 0x20;
pub const LICENSE_PREVIEWPRINT: u32 = 0x4;
pub const LICENSE_EDITABLE: u32 = 0x8;
pub const LICENSE_INSTALLABLE: u32 = 0x0;
pub const LICENSE_NOEMBEDDING: u32 = 0x2;
pub const LICENSE_DEFAULT: u32 = 0x0;
pub const OUT_CHARACTER_PRECIS: u32 = 0x2;
pub const OUT_DEFAULT_PRECIS: u32 = 0x0;
pub const OUT_DEVICE_PRECIS: u32 = 0x5;
pub const OUT_OUTLINE_PRECIS: u32 = 0x8;
pub const OUT_PS_ONLY_PRECIS: u32 = 0xA;
pub const OUT_RASTER_PRECIS: u32 = 0x6;
pub const OUT_STRING_PRECIS: u32 = 0x1;
pub const OUT_STROKE_PRECIS: u32 = 0x3;
pub const OUT_TT_ONLY_PRECIS: u32 = 0x7;
pub const OUT_TT_PRECIS: u32 = 0x4;
pub const FF_DECORATIVE: u32 = 0x50;
pub const FF_DONTCARE: u32 = 0x0;
pub const FF_MODERN: u32 = 0x30;
pub const FF_ROMAN: u32 = 0x10;
pub const FF_SCRIPT: u32 = 0x40;
pub const FF_SWISS: u32 = 0x20;
pub const ANTIALIASED_QUALITY: u32 = 0x4;
pub const CLEARTYPE_QUALITY: u32 = 0x5;
pub const DEFAULT_QUALITY: u32 = 0x0;
pub const DRAFT_QUALITY: u32 = 0x1;
pub const NONANTIALIASED_QUALITY: u32 = 0x3;
pub const PROOF_QUALITY: u32 = 0x2;
pub const FR_PRIVATE: u32 = 0x10;
pub const FR_NOT_ENUM: u32 = 0x20;
pub const LSFW_LOCK: u32 = 0x1;
pub const LSFW_UNLOCK: u32 = 0x2;
pub struct FsCopyHandler {
}
pub const FVST_EMPTYTEXT: i32 = 0x0;
pub struct FileOpenDialog {
}
pub struct FileOperation {
}
pub struct FileSaveDialog {
}
pub struct FileSearchBand {
}
pub struct Folder {
}
pub struct Folder2 {
}
pub struct Folder3 {
}
pub struct FolderItem {
}
pub struct FolderItem2 {
}
pub struct FolderItemVerb {
}
pub struct FolderItemVerbs {
}
pub struct FolderItems {
}
pub struct FolderItems2 {
}
pub struct FolderItems3 {
}
pub struct FolderViewHost {
}
pub struct FrameworkInputPane {
}
pub struct FreeSpaceCategorizer {
}
pub struct GcpResultsa<'a> {
    pub lStructSize: u32,
    pub lpOutString: &'a mut Cow<'a, str>,
    pub lpOrder: &'a mut u32,
    pub lpDx: &'a mut i32,
    pub lpCaretPos: &'a mut i32,
    pub lpClass: &'a mut Cow<'a, str>,
    pub lpGlyphs: &'a mut Cow<'a, OsStr>,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
pub struct GcpResultsw<'a> {
    pub lStructSize: u32,
    pub lpOutString: &'a mut Cow<'a, OsStr>,
    pub lpOrder: &'a mut u32,
    pub lpDx: &'a mut i32,
    pub lpCaretPos: &'a mut i32,
    pub lpClass: &'a mut Cow<'a, str>,
    pub lpGlyphs: &'a mut Cow<'a, OsStr>,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
pub const IMAGE_BITMAP: u32 = 0x0;
pub const IMAGE_CURSOR: u32 = 0x2;
pub const IMAGE_ICON: u32 = 0x1;
pub const GA_PARENT: u32 = 0x1;
pub const GA_ROOT: u32 = 0x2;
pub const GA_ROOTOWNER: u32 = 0x3;
pub const GCP_CLASSIN: u32 = 0x80000;
pub const GCP_DIACRITIC: u32 = 0x100;
pub const GCP_DISPLAYZWG: u32 = 0x400000;
pub const GCP_GLYPHSHAPE: u32 = 0x10;
pub const GCP_JUSTIFY: u32 = 0x10000;
pub const GCP_KASHIDA: u32 = 0x400;
pub const GCP_LIGATE: u32 = 0x20;
pub const GCP_MAXEXTENT: u32 = 0x100000;
pub const GCP_NEUTRALOVERRIDE: u32 = 0x2000000;
pub const GCP_NUMERICOVERRIDE: u32 = 0x1000000;
pub const GCP_NUMERICSLATIN: u32 = 0x4000000;
pub const GCP_NUMERICSLOCAL: u32 = 0x8000000;
pub const GCP_REORDER: u32 = 0x2;
pub const GCP_SYMSWAPOFF: u32 = 0x800000;
pub const GCP_USEKERNING: u32 = 0x8;
pub const GCW_ATOM: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE0;
pub const GCL_CBCLSEXTRA: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC;
pub const GCL_CBWNDEXTRA: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEE;
pub const GCL_HBRBACKGROUND: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF6;
pub const GCL_HCURSOR: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4;
pub const GCL_HICON: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF2;
pub const GCL_HICONSM: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFDE;
pub const GCL_HMODULE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0;
pub const GCL_MENUNAME: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8;
pub const GCL_STYLE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE6;
pub const GCL_WNDPROC: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE8;
pub const GCLP_HBRBACKGROUND: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF6;
pub const GCLP_HCURSOR: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4;
pub const GCLP_HICON: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF2;
pub const GCLP_HICONSM: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFDE;
pub const GCLP_HMODULE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0;
pub const GCLP_MENUNAME: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8;
pub const GCLP_WNDPROC: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE8;
pub const DCX_WINDOW: u32 = 0x1;
pub const DCX_CACHE: u32 = 0x2;
pub const DCX_PARENTCLIP: u32 = 0x20;
pub const DCX_CLIPSIBLINGS: u32 = 0x10;
pub const DCX_CLIPCHILDREN: u32 = 0x8;
pub const DCX_NORESETATTRS: u32 = 0x4;
pub const DCX_LOCKWINDOWUPDATE: u32 = 0x400;
pub const DCX_EXCLUDERGN: u32 = 0x40;
pub const DCX_INTERSECTRGN: u32 = 0x80;
pub const DCX_INTERSECTUPDATE: u32 = 0x200;
pub const DCX_VALIDATE: u32 = 0x200000;
pub const DRIVERVERSION: u32 = 0x0;
pub const TECHNOLOGY: u32 = 0x2;
pub const HORZSIZE: u32 = 0x4;
pub const VERTSIZE: u32 = 0x6;
pub const HORZRES: u32 = 0x8;
pub const VERTRES: u32 = 0xA;
pub const BITSPIXEL: u32 = 0xC;
pub const PLANES: u32 = 0xE;
pub const NUMBRUSHES: u32 = 0x10;
pub const NUMPENS: u32 = 0x12;
pub const NUMMARKERS: u32 = 0x14;
pub const NUMFONTS: u32 = 0x16;
pub const NUMCOLORS: u32 = 0x18;
pub const PDEVICESIZE: u32 = 0x1A;
pub const CURVECAPS: u32 = 0x1C;
pub const LINECAPS: u32 = 0x1E;
pub const POLYGONALCAPS: u32 = 0x20;
pub const TEXTCAPS: u32 = 0x22;
pub const CLIPCAPS: u32 = 0x24;
pub const RASTERCAPS: u32 = 0x26;
pub const ASPECTX: u32 = 0x28;
pub const ASPECTY: u32 = 0x2A;
pub const ASPECTXY: u32 = 0x2C;
pub const LOGPIXELSX: u32 = 0x58;
pub const LOGPIXELSY: u32 = 0x5A;
pub const SIZEPALETTE: u32 = 0x68;
pub const NUMRESERVED: u32 = 0x6A;
pub const COLORRES: u32 = 0x6C;
pub const PHYSICALWIDTH: u32 = 0x6E;
pub const PHYSICALHEIGHT: u32 = 0x6F;
pub const PHYSICALOFFSETX: u32 = 0x70;
pub const PHYSICALOFFSETY: u32 = 0x71;
pub const SCALINGFACTORX: u32 = 0x72;
pub const SCALINGFACTORY: u32 = 0x73;
pub const VREFRESH: u32 = 0x74;
pub const DESKTOPVERTRES: u32 = 0x75;
pub const DESKTOPHORZRES: u32 = 0x76;
pub const BLTALIGNMENT: u32 = 0x77;
pub const SHADEBLENDCAPS: u32 = 0x78;
pub const COLORMGMTCAPS: u32 = 0x79;
pub const GGO_BEZIER: u32 = 0x3;
pub const GGO_BITMAP: u32 = 0x1;
pub const GGO_GLYPH_INDEX: u32 = 0x80;
pub const GGO_GRAY2_BITMAP: u32 = 0x4;
pub const GGO_GRAY4_BITMAP: u32 = 0x5;
pub const GGO_GRAY8_BITMAP: u32 = 0x6;
pub const GGO_METRICS: u32 = 0x0;
pub const GGO_NATIVE: u32 = 0x2;
pub const GGO_UNHINTED: u32 = 0x100;
pub const GMDI_GOINTOPOPUPS: u32 = 0x2;
pub const GMDI_USEDISABLED: u32 = 0x1;
pub const BLACK_BRUSH: u32 = 0x4;
pub const DKGRAY_BRUSH: u32 = 0x3;
pub const DC_BRUSH: u32 = 0x12;
pub const GRAY_BRUSH: u32 = 0x2;
pub const HOLLOW_BRUSH: u32 = 0x5;
pub const LTGRAY_BRUSH: u32 = 0x1;
pub const NULL_BRUSH: u32 = 0x5;
pub const WHITE_BRUSH: u32 = 0x0;
pub const BLACK_PEN: u32 = 0x7;
pub const DC_PEN: u32 = 0x13;
pub const NULL_PEN: u32 = 0x8;
pub const WHITE_PEN: u32 = 0x6;
pub const ANSI_FIXED_FONT: u32 = 0xB;
pub const ANSI_VAR_FONT: u32 = 0xC;
pub const DEVICE_DEFAULT_FONT: u32 = 0xE;
pub const DEFAULT_GUI_FONT: u32 = 0x11;
pub const OEM_FIXED_FONT: u32 = 0xA;
pub const SYSTEM_FONT: u32 = 0xD;
pub const SYSTEM_FIXED_FONT: u32 = 0x10;
pub const DEFAULT_PALETTE: u32 = 0xF;
pub const GBF_DIRECT: u32 = 0x1;
pub const GBF_COPY: u32 = 0x2;
pub const GBF_VALIDBITS: u32 = 0x3;
pub const GW_CHILD: u32 = 0x5;
pub const GW_ENABLEDPOPUP: u32 = 0x6;
pub const GW_HWNDFIRST: u32 = 0x0;
pub const GW_HWNDLAST: u32 = 0x1;
pub const GW_HWNDNEXT: u32 = 0x2;
pub const GW_HWNDPREV: u32 = 0x3;
pub const GW_OWNER: u32 = 0x4;
pub const GFST_NONE: i32 = 0x0;
pub const GFST_SIZE: i32 = 0x1;
pub const GFST_DPI: i32 = 0x2;
pub struct Glyphmetrics {
    pub gmBlackBoxX: u32,
    pub gmBlackBoxY: u32,
    pub gmptGlyphOrigin: POINT,
    pub gmCellIncX: i16,
    pub gmCellIncY: i16,
}
pub struct Glyphset {
    pub cbThis: u32,
    pub flAccel: u32,
    pub cGlyphsSupported: u32,
    pub cRanges: u32,
    pub ranges: [WCRANGE; 1],
}
pub const GT_NONE: i32 = 0x0;
pub const GT_IMAGEGLYPH: i32 = 0x1;
pub const GT_FONTGLYPH: i32 = 0x2;
pub const GRADIENT_FILL_RECT_H: u32 = 0x0;
pub const GRADIENT_FILL_RECT_V: u32 = 0x1;
pub const GRADIENT_FILL_TRIANGLE: u32 = 0x2;
pub struct GradientRect {
    pub UpperLeft: u32,
    pub LowerRight: u32,
}
pub struct GradientTriangle {
    pub Vertex1: u32,
    pub Vertex2: u32,
    pub Vertex3: u32,
}
pub const GM_COMPATIBLE: u32 = 0x1;
pub const GM_ADVANCED: u32 = 0x2;
pub const MCGCB_SELECTED: i32 = 0x1;
pub const MCGCB_HOT: i32 = 0x2;
pub const MCGCB_SELECTEDHOT: i32 = 0x3;
pub const MCGCB_SELECTEDNOTFOCUSED: i32 = 0x4;
pub const MCGCB_TODAY: i32 = 0x5;
pub const MCGCB_TODAYSELECTED: i32 = 0x6;
pub const MCGC_HOT: i32 = 0x1;
pub const MCGC_HASSTATE: i32 = 0x2;
pub const MCGC_HASSTATEHOT: i32 = 0x3;
pub const MCGC_TODAY: i32 = 0x4;
pub const MCGC_TODAYSELECTED: i32 = 0x5;
pub const MCGC_SELECTED: i32 = 0x6;
pub const MCGC_SELECTEDHOT: i32 = 0x7;
pub const MCGCU_HOT: i32 = 0x1;
pub const MCGCU_HASSTATE: i32 = 0x2;
pub const MCGCU_HASSTATEHOT: i32 = 0x3;
pub const MCGCU_SELECTED: i32 = 0x4;
pub const MCGCU_SELECTEDHOT: i32 = 0x5;
pub struct Guithreadinfo {
    pub cbSize: u32,
    pub flags: u32,
    pub hwndActive: HWND,
    pub hwndFocus: HWND,
    pub hwndCapture: HWND,
    pub hwndMenuOwner: HWND,
    pub hwndMoveSize: HWND,
    pub hwndCaret: HWND,
    pub rcCaret: RECT,
}
pub const GUI_CARETBLINKING: u32 = 0x1;
pub const GUI_INMENUMODE: u32 = 0x4;
pub const GUI_INMOVESIZE: u32 = 0x2;
pub const GUI_POPUPMENUMODE: u32 = 0x10;
pub const GUI_SYSTEMMENUMODE: u32 = 0x8;
pub struct GenericCredentialProvider {
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Haccel {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Haccel {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Haccel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub const HA_LEFT: i32 = 0x0;
pub const HA_CENTER: i32 = 0x1;
pub const HA_RIGHT: i32 = 0x2;
pub const HANDEDNESS_LEFT: i32 = 0x0;
pub const HANDEDNESS_RIGHT: i32 = 0x1;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Handle {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub struct Handletable {
    pub objectHandle: [HGDIOBJ; 1],
}
pub const HANDLE_FLAG_INHERIT: u32 = 0x1;
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: u32 = 0x2;
pub struct Hardwarehookstruct {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
}
pub const HS_BDIAGONAL: u32 = 0x3;
pub const HS_CROSS: u32 = 0x4;
pub const HS_DIAGCROSS: u32 = 0x5;
pub const HS_FDIAGONAL: u32 = 0x2;
pub const HS_HORIZONTAL: u32 = 0x0;
pub const HS_VERTICAL: u32 = 0x1;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hbitmap {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hbitmap {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hbitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hbrush {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hbrush {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hbrush {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hcursor {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hcursor {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hcursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hdc {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hdc {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hdc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub const MM_ANISOTROPIC: u32 = 0x8;
pub const MM_HIENGLISH: u32 = 0x5;
pub const MM_HIMETRIC: u32 = 0x3;
pub const MM_ISOTROPIC: u32 = 0x7;
pub const MM_LOENGLISH: u32 = 0x4;
pub const MM_LOMETRIC: u32 = 0x2;
pub const MM_TEXT: u32 = 0x1;
pub const MM_TWIPS: u32 = 0x6;
pub struct Hdhittestinfo {
    pub pt: POINT,
    pub flags: u32,
    pub iItem: i32,
}
pub struct Hditema<'a> {
    pub mask: u32,
    pub cxy: i32,
    pub pszText: &'a mut Cow<'a, str>,
    pub hbm: HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub type: u32,
    pub pvFilter: &'a mut todo_void,
    pub state: u32,
}
pub struct Hditemw<'a> {
    pub mask: u32,
    pub cxy: i32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub hbm: HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub type: u32,
    pub pvFilter: &'a mut todo_void,
    pub state: u32,
}
pub const HDI_WIDTH: u32 = 0x1;
pub const HDI_HEIGHT: u32 = 0x1;
pub const HDI_TEXT: u32 = 0x2;
pub const HDI_FORMAT: u32 = 0x4;
pub const HDI_LPARAM: u32 = 0x8;
pub const HDI_BITMAP: u32 = 0x10;
pub const HDI_IMAGE: u32 = 0x20;
pub const HDI_DI_SETITEM: u32 = 0x40;
pub const HDI_ORDER: u32 = 0x80;
pub const HDI_FILTER: u32 = 0x100;
pub const HDI_STATE: u32 = 0x200;
pub struct Hdlayout<'a> {
    pub prc: &'a mut RECT,
    pub pwpos: &'a mut WINDOWPOS,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hdpa {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hdpa {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hdpa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hdrop {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hdrop {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hdrop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hdsa {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hdsa {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hdsa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub struct HdTextfiltera<'a> {
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
}
pub struct HdTextfilterw<'a> {
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
}
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_LEFT: u32 = 0x0;
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_RIGHT: u32 = 0x1;
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_MIDDLE: u32 = 0x2;
pub struct Helpinfo {
    pub cbSize: u32,
    pub iContextType: i32,
    pub iCtrlId: i32,
    pub hItemHandle: HANDLE,
    pub dwContextId: usize,
    pub MousePos: POINT,
}
pub struct Helpwininfoa {
    pub wStructSize: i32,
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub wMax: i32,
    pub rgchMember: [CHAR; 2],
}
pub struct Helpwininfow {
    pub wStructSize: i32,
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub wMax: i32,
    pub rgchMember: [u8; 2],
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Henhmetafile {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Henhmetafile {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Henhmetafile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hfont {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hfont {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hfont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hgdiobj {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hgdiobj {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hgdiobj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hhook {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hhook {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hhook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hicon {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hicon {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hicon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon10<'a> {
    pub field0: u32,
    pub field1: &'a Cow<'a, str>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon12<'a> {
    pub field0: HBITMAP,
    pub field1: &'a Cow<'a, str>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon13<'a> {
    pub field0: HBITMAP,
    pub field1: &'a Cow<'a, str>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon15<'a> {
    pub field0: u32,
    pub field1: &'a Cow<'a, OsStr>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon18<'a> {
    pub field0: u32,
    pub field1: &'a Cow<'a, OsStr>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon20<'a> {
    pub field0: HBITMAP,
    pub field1: &'a Cow<'a, OsStr>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon21<'a> {
    pub field0: HBITMAP,
    pub field1: &'a Cow<'a, OsStr>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon24<'a> {
    pub field0: HBITMAP,
    pub field1: &'a Cow<'a, str>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon33<'a> {
    pub field0: HBITMAP,
    pub field1: &'a Cow<'a, OsStr>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon41<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, OsStr>,
}
pub struct Hicon {
    pub Value: isize,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Hicon7<'a> {
    pub field0: u32,
    pub field1: &'a Cow<'a, str>,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Himagelist {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Himagelist {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Himagelist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hinstance {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hinstance {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hinstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hkey {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hkey {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub const HLBWIF_HASFRAMEWNDINFO: u32 = 0x1;
pub const HLBWIF_HASDOCWNDINFO: u32 = 0x2;
pub const HLBWIF_FRAMEWNDMAXIMIZED: u32 = 0x4;
pub const HLBWIF_DOCWNDMAXIMIZED: u32 = 0x8;
pub const HLBWIF_HASWEBTOOLBARINFO: u32 = 0x10;
pub const HLBWIF_WEBTOOLBARHIDDEN: u32 = 0x20;
pub struct Hlbwinfo {
    pub cbSize: u32,
    pub grfHLBWIF: u32,
    pub rcFramePos: RECT,
    pub rcDocPos: RECT,
    pub hltbinfo: HLTBINFO,
}
pub const HLFNAMEF_DEFAULT: u32 = 0x0;
pub const HLFNAMEF_TRYCACHE: u32 = 0x1;
pub const HLFNAMEF_TRYPRETTYTARGET: u32 = 0x2;
pub const HLFNAMEF_TRYFULLTARGET: u32 = 0x4;
pub const HLFNAMEF_TRYWIN95SHORTCUT: u32 = 0x8;
pub const HLID_INVALID: u32 = 0x0;
pub const HLID_PREVIOUS: u32 = 0xFFFFFFFF;
pub const HLID_NEXT: u32 = 0xFFFFFFFE;
pub const HLID_CURRENT: u32 = 0xFFFFFFFD;
pub const HLID_STACKBOTTOM: u32 = 0xFFFFFFFC;
pub const HLID_STACKTOP: u32 = 0xFFFFFFFB;
pub const HLINKGETREF_DEFAULT: i32 = 0x0;
pub const HLINKGETREF_ABSOLUTE: i32 = 0x1;
pub const HLINKGETREF_RELATIVE: i32 = 0x2;
pub const HLINKMISC_RELATIVE: i32 = 0x1;
pub const HLINKSETF_TARGET: i32 = 0x1;
pub const HLINKSETF_LOCATION: i32 = 0x2;
pub const HLINKWHICHMK_CONTAINER: i32 = 0x1;
pub const HLINKWHICHMK_BASE: i32 = 0x2;
pub struct Hlitem<'a> {
    pub uHLID: u32,
    pub pwzFriendlyName: &'a mut Cow<'a, OsStr>,
}
pub const HLNF_INTERNALJUMP: u32 = 0x1;
pub const HLNF_OPENINNEWWINDOW: u32 = 0x2;
pub const HLNF_NAVIGATINGBACK: u32 = 0x4;
pub const HLNF_NAVIGATINGFORWARD: u32 = 0x8;
pub const HLNF_NAVIGATINGTOSTACKITEM: u32 = 0x10;
pub const HLNF_CREATENOHISTORY: u32 = 0x20;
pub const HLQF_ISVALID: i32 = 0x1;
pub const HLQF_ISCURRENT: i32 = 0x2;
pub struct Hlsurf {
    pub unused: i32,
}
pub struct Hltbinfo {
    pub uDockType: u32,
    pub rcTbPos: RECT,
}
pub const HLTB_DOCKEDLEFT: i32 = 0x0;
pub const HLTB_DOCKEDTOP: i32 = 0x1;
pub const HLTB_DOCKEDRIGHT: i32 = 0x2;
pub const HLTB_DOCKEDBOTTOM: i32 = 0x3;
pub const HLTB_FLOATING: i32 = 0x4;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hmenu {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hmenu {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hmenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hmetafile {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hmetafile {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hmetafile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hmonitor {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hmonitor {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hmonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub const HGSC_NONE: i32 = 0x0;
pub const HGSC_MUSICLIBRARY: i32 = 0x1;
pub const HGSC_PICTURESLIBRARY: i32 = 0x2;
pub const HGSC_VIDEOSLIBRARY: i32 = 0x4;
pub const HGSC_DOCUMENTSLIBRARY: i32 = 0x8;
pub const HGSC_PRINTERS: i32 = 0x10;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hpalette {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hpalette {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hpalette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hpen {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hpen {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hpen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hpropsheetpage {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hpropsheetpage {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hpropsheetpage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hpsxa {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hpsxa {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hpsxa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hrgn {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hrgn {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hrgn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hrsrc {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hrsrc {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hrsrc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub struct Hsprite {
    pub unused: i32,
}
pub struct Hstr {
    pub unused: i32,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hsyntheticpointerdevice {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hsyntheticpointerdevice {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hsyntheticpointerdevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Htreeitem {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Htreeitem {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Htreeitem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub struct Humpd {
    pub unused: i32,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hwnd {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Hwnd {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Hwnd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub const HLS_NORMALTEXT: i32 = 0x1;
pub const HLS_LINKTEXT: i32 = 0x2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HdcMetdataEnhFileHandle {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl HdcMetdataEnhFileHandle {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for HdcMetdataEnhFileHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HdcMetdataFileHandle {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl HdcMetdataFileHandle {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for HdcMetdataFileHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub struct HideInputPaneAnimationCoordinator {
}
pub struct HomeGroup {
}
pub const ICE_NONE: i32 = 0x0;
pub const ICE_GLOW: i32 = 0x1;
pub const ICE_SHADOW: i32 = 0x2;
pub const ICE_PULSE: i32 = 0x3;
pub const ICE_ALPHA: i32 = 0x4;
pub struct Iconinfo {
    pub fIcon: BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: HBITMAP,
    pub hbmColor: HBITMAP,
}
pub struct Iconinfoexa {
    pub cbSize: u32,
    pub fIcon: BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: HBITMAP,
    pub hbmColor: HBITMAP,
    pub wResID: u16,
    pub szModName: [CHAR; 260],
    pub szResName: [CHAR; 260],
}
pub struct Iconinfoexw {
    pub cbSize: u32,
    pub fIcon: BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: HBITMAP,
    pub hbmColor: HBITMAP,
    pub wResID: u16,
    pub szModName: [u8; 260],
    pub szResName: [u8; 260],
}
pub struct Iconmetricsa {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: LOGFONTA,
}
pub struct Iconmetricsw {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: LOGFONTW,
}
pub const IEPDN_BINDINGUI: i32 = 0x1;
pub const IESHORTCUT_NEWBROWSER: i32 = 0x1;
pub const IESHORTCUT_OPENNEWTAB: i32 = 0x2;
pub const IESHORTCUT_FORCENAVIGATE: i32 = 0x4;
pub const IESHORTCUT_BACKGROUNDTAB: i32 = 0x8;
pub struct Imageinfo {
    pub hbmImage: HBITMAP,
    pub hbmMask: HBITMAP,
    pub Unused1: i32,
    pub Unused2: i32,
    pub rcImage: RECT,
}
pub const IL_VERTICAL: i32 = 0x0;
pub const IL_HORIZONTAL: i32 = 0x1;
pub struct Imagelistdrawparams {
    pub cbSize: u32,
    pub himl: HIMAGELIST,
    pub i: i32,
    pub hdcDst: HDC,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub xBitmap: i32,
    pub yBitmap: i32,
    pub rgbBk: u32,
    pub rgbFg: u32,
    pub fStyle: u32,
    pub dwRop: u32,
    pub fState: u32,
    pub Frame: u32,
    pub crEffect: u32,
}
pub struct Imageliststats {
    pub cbSize: u32,
    pub cAlloc: i32,
    pub cUsed: i32,
    pub cStandby: i32,
}
pub const ILC_MASK: u32 = 0x1;
pub const ILC_COLOR: u32 = 0x0;
pub const ILC_COLORDDB: u32 = 0xFE;
pub const ILC_COLOR4: u32 = 0x4;
pub const ILC_COLOR8: u32 = 0x8;
pub const ILC_COLOR16: u32 = 0x10;
pub const ILC_COLOR24: u32 = 0x18;
pub const ILC_COLOR32: u32 = 0x20;
pub const ILC_PALETTE: u32 = 0x800;
pub const ILC_MIRROR: u32 = 0x2000;
pub const ILC_PERITEMMIRROR: u32 = 0x8000;
pub const ILC_ORIGINALSIZE: u32 = 0x10000;
pub const ILC_HIGHQUALITYSCALE: u32 = 0x20000;
pub const IST_NONE: i32 = 0x0;
pub const IST_SIZE: i32 = 0x1;
pub const IST_DPI: i32 = 0x2;
pub const LR_CREATEDIBSECTION: u32 = 0x2000;
pub const LR_DEFAULTCOLOR: u32 = 0x0;
pub const LR_DEFAULTSIZE: u32 = 0x40;
pub const LR_LOADFROMFILE: u32 = 0x10;
pub const LR_LOADMAP3DCOLORS: u32 = 0x1000;
pub const LR_LOADTRANSPARENT: u32 = 0x20;
pub const LR_MONOCHROME: u32 = 0x1;
pub const LR_SHARED: u32 = 0x8000;
pub const LR_VGACOLOR: u32 = 0x80;
pub const LR_COPYDELETEORG: u32 = 0x8;
pub const LR_COPYFROMRESOURCE: u32 = 0x4000;
pub const LR_COPYRETURNORG: u32 = 0x4;
pub const ILCF_MOVE: u32 = 0x0;
pub const ILCF_SWAP: u32 = 0x1;
pub const ILD_BLEND: u32 = 0x4;
pub const ILD_BLEND50: u32 = 0x4;
pub const ILD_FOCUS: u32 = 0x2;
pub const ILD_MASK: u32 = 0x10;
pub const ILD_NORMAL: u32 = 0x0;
pub const ILD_SELECTED: u32 = 0x4;
pub const ILIF_ALPHA: u32 = 0x1;
pub const ILIF_LOWQUALITY: u32 = 0x2;
pub struct Initcommoncontrolsex {
    pub dwSize: u32,
    pub dwICC: u32,
}
pub const ICC_ANIMATE_CLASS: u32 = 0x80;
pub const ICC_BAR_CLASSES: u32 = 0x4;
pub const ICC_COOL_CLASSES: u32 = 0x400;
pub const ICC_DATE_CLASSES: u32 = 0x100;
pub const ICC_HOTKEY_CLASS: u32 = 0x40;
pub const ICC_INTERNET_CLASSES: u32 = 0x800;
pub const ICC_LINK_CLASS: u32 = 0x8000;
pub const ICC_LISTVIEW_CLASSES: u32 = 0x1;
pub const ICC_NATIVEFNTCTL_CLASS: u32 = 0x2000;
pub const ICC_PAGESCROLLER_CLASS: u32 = 0x1000;
pub const ICC_PROGRESS_CLASS: u32 = 0x20;
pub const ICC_STANDARD_CLASSES: u32 = 0x4000;
pub const ICC_TAB_CLASSES: u32 = 0x8;
pub const ICC_TREEVIEW_CLASSES: u32 = 0x2;
pub const ICC_UPDOWN_CLASS: u32 = 0x10;
pub const ICC_USEREX_CLASSES: u32 = 0x200;
pub const ICC_WIN95_CLASSES: u32 = 0xFF;
pub struct InputInjectionValue {
    pub page: u16,
    pub usage: u16,
    pub value: i32,
    pub index: u16,
}
pub struct InputTransform {
    pub Anonymous: INPUT_TRANSFORM_4,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union InputTransform4 {
    pub field0: _Anonymous_e__Struct,
    pub field1: [f32; 16],
}
pub struct AnonymousEStruct {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
pub struct Intlist {
    pub iValueCount: i32,
    pub iValues: [i32; 402],
}
pub struct Itemspacing {
    pub cxSmall: i32,
    pub cySmall: i32,
    pub cxLarge: i32,
    pub cyLarge: i32,
}
pub struct Kbdllhookstruct {
    pub vkCode: u32,
    pub scanCode: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub const LLKHF_EXTENDED: u32 = 0x1;
pub const LLKHF_ALTDOWN: u32 = 0x20;
pub const LLKHF_UP: u32 = 0x80;
pub const LLKHF_INJECTED: u32 = 0x10;
pub const LLKHF_LOWER_IL_INJECTED: u32 = 0x2;
pub struct Kerningpair {
    pub wFirst: u16,
    pub wSecond: u16,
    pub iKernAmount: i32,
}
pub const KF_CATEGORY_VIRTUAL: i32 = 0x1;
pub const KF_CATEGORY_FIXED: i32 = 0x2;
pub const KF_CATEGORY_COMMON: i32 = 0x3;
pub const KF_CATEGORY_PERUSER: i32 = 0x4;
pub const KDC_FREQUENT: i32 = 0x1;
pub const KDC_RECENT: i32 = 0x2;
pub struct KnownfolderDefinition<'a> {
    pub category: i32,
    pub pszName: &'a mut Cow<'a, OsStr>,
    pub pszDescription: &'a mut Cow<'a, OsStr>,
    pub fidParent: GUID,
    pub pszRelativePath: &'a mut Cow<'a, OsStr>,
    pub pszParsingName: &'a mut Cow<'a, OsStr>,
    pub pszTooltip: &'a mut Cow<'a, OsStr>,
    pub pszLocalizedName: &'a mut Cow<'a, OsStr>,
    pub pszIcon: &'a mut Cow<'a, OsStr>,
    pub pszSecurity: &'a mut Cow<'a, OsStr>,
    pub dwAttributes: u32,
    pub kfdFlags: u32,
    pub ftidType: GUID,
}
pub const KF_FLAG_DEFAULT: i32 = 0x0;
pub const KF_FLAG_FORCE_APP_DATA_REDIRECTION: i32 = 0x80000;
pub const KF_FLAG_RETURN_FILTER_REDIRECTION_TARGET: i32 = 0x40000;
pub const KF_FLAG_FORCE_PACKAGE_REDIRECTION: i32 = 0x20000;
pub const KF_FLAG_NO_PACKAGE_REDIRECTION: i32 = 0x10000;
pub const KF_FLAG_FORCE_APPCONTAINER_REDIRECTION: i32 = 0x20000;
pub const KF_FLAG_NO_APPCONTAINER_REDIRECTION: i32 = 0x10000;
pub const KF_FLAG_CREATE: i32 = 0x8000;
pub const KF_FLAG_DONT_VERIFY: i32 = 0x4000;
pub const KF_FLAG_DONT_UNEXPAND: i32 = 0x2000;
pub const KF_FLAG_NO_ALIAS: i32 = 0x1000;
pub const KF_FLAG_INIT: i32 = 0x800;
pub const KF_FLAG_DEFAULT_PATH: i32 = 0x400;
pub const KF_FLAG_NOT_PARENT_RELATIVE: i32 = 0x200;
pub const KF_FLAG_SIMPLE_IDLIST: i32 = 0x100;
pub const KF_FLAG_ALIAS_ONLY: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000;
pub struct KnownFolderManager {
}
pub const LWA_ALPHA: u32 = 0x2;
pub const LWA_COLORKEY: u32 = 0x1;
pub struct Lhittestinfo {
    pub pt: POINT,
    pub item: LITEM,
}
pub const LFF_FORCEFILESYSTEM: i32 = 0x1;
pub const LFF_STORAGEITEMS: i32 = 0x2;
pub const LFF_ALLITEMS: i32 = 0x3;
pub const LMD_DEFAULT: i32 = 0x0;
pub const LMD_ALLOWUNINDEXABLENETWORKLOCATIONS: i32 = 0x1;
pub const LOF_DEFAULT: i32 = 0x0;
pub const LOF_PINNEDTONAVPANE: i32 = 0x1;
pub const LOF_MASK_ALL: i32 = 0x1;
pub const LSF_FAILIFTHERE: i32 = 0x0;
pub const LSF_OVERRIDEEXISTING: i32 = 0x1;
pub const LSF_MAKEUNIQUENAME: i32 = 0x2;
pub const LP_HYPERLINK: i32 = 0x1;
pub struct Litem {
    pub mask: u32,
    pub iLink: i32,
    pub state: u32,
    pub stateMask: u32,
    pub szID: [u8; 48],
    pub szUrl: [u8; 2084],
}
pub const DONT_RESOLVE_DLL_REFERENCES: u32 = 0x1;
pub const LOAD_LIBRARY_AS_DATAFILE: u32 = 0x2;
pub const LOAD_WITH_ALTERED_SEARCH_PATH: u32 = 0x8;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: u32 = 0x10;
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: u32 = 0x20;
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: u32 = 0x40;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: u32 = 0x80;
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: u32 = 0x100;
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: u32 = 0x200;
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: u32 = 0x400;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x800;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: u32 = 0x1000;
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: u32 = 0x2000;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: u32 = 0x4000;
pub struct Logbrush {
    pub lbStyle: u32,
    pub lbColor: u32,
    pub lbHatch: usize,
}
pub struct Logbrush32 {
    pub lbStyle: u32,
    pub lbColor: u32,
    pub lbHatch: u32,
}
pub struct Logfonta {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: u8,
    pub lfClipPrecision: u8,
    pub lfQuality: u8,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [CHAR; 32],
}
pub struct Logfontw {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: u8,
    pub lfClipPrecision: u8,
    pub lfQuality: u8,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [u8; 32],
}
pub const SPLS_NORMAL: i32 = 0x1;
pub const SPLS_HOT: i32 = 0x2;
pub const SPLS_PRESSED: i32 = 0x3;
pub struct Logpalette {
    pub palVersion: u16,
    pub palNumEntries: u16,
    pub palPalEntry: [PALETTEENTRY; 1],
}
pub struct Logpen {
    pub lopnStyle: u32,
    pub lopnWidth: POINT,
    pub lopnColor: u32,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lparam {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Lparam {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Lparam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lresult {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl Lresult {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for Lresult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub struct Luid {
    pub LowPart: u32,
    pub HighPart: i32,
}
pub struct Lvbkimagea<'a> {
    pub ulFlags: u32,
    pub hbm: HBITMAP,
    pub pszImage: &'a mut Cow<'a, str>,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
pub struct Lvbkimagew<'a> {
    pub ulFlags: u32,
    pub hbm: HBITMAP,
    pub pszImage: &'a mut Cow<'a, OsStr>,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
pub struct Lvcolumna<'a> {
    pub mask: u32,
    pub fmt: u32,
    pub cx: i32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
pub struct Lvcolumnw<'a> {
    pub mask: u32,
    pub fmt: u32,
    pub cx: i32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
pub const LVCFMT_LEFT: u32 = 0x0;
pub const LVCFMT_RIGHT: u32 = 0x1;
pub const LVCFMT_CENTER: u32 = 0x2;
pub const LVCFMT_JUSTIFYMASK: u32 = 0x3;
pub const LVCFMT_IMAGE: u32 = 0x800;
pub const LVCFMT_BITMAP_ON_RIGHT: u32 = 0x1000;
pub const LVCFMT_COL_HAS_IMAGES: u32 = 0x8000;
pub const LVCFMT_FIXED_WIDTH: u32 = 0x100;
pub const LVCFMT_NO_DPI_SCALE: u32 = 0x40000;
pub const LVCFMT_FIXED_RATIO: u32 = 0x80000;
pub const LVCFMT_SPLITBUTTON: u32 = 0x1000000;
pub const LVCF_FMT: u32 = 0x1;
pub const LVCF_WIDTH: u32 = 0x2;
pub const LVCF_TEXT: u32 = 0x4;
pub const LVCF_SUBITEM: u32 = 0x8;
pub const LVCF_IMAGE: u32 = 0x10;
pub const LVCF_ORDER: u32 = 0x20;
pub const LVCF_MINWIDTH: u32 = 0x40;
pub const LVCF_DEFAULTWIDTH: u32 = 0x80;
pub const LVCF_IDEALWIDTH: u32 = 0x100;
pub struct Lvfindinfoa<'a> {
    pub flags: u32,
    pub psz: &'a Cow<'a, str>,
    pub lParam: LPARAM,
    pub pt: POINT,
    pub vkDirection: u32,
}
pub struct Lvfindinfow<'a> {
    pub flags: u32,
    pub psz: &'a Cow<'a, OsStr>,
    pub lParam: LPARAM,
    pub pt: POINT,
    pub vkDirection: u32,
}
pub const LVFI_PARAM: u32 = 0x1;
pub const LVFI_PARTIAL: u32 = 0x8;
pub const LVFI_STRING: u32 = 0x2;
pub const LVFI_SUBSTRING: u32 = 0x4;
pub const LVFI_WRAP: u32 = 0x20;
pub const LVFI_NEARESTXY: u32 = 0x40;
pub struct Lvfooterinfo<'a> {
    pub mask: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub cItems: u32,
}
pub struct Lvfooteritem<'a> {
    pub mask: u32,
    pub iItem: i32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub state: u32,
    pub stateMask: u32,
}
pub const LVFIF_TEXT: u32 = 0x1;
pub const LVFIF_STATE: u32 = 0x2;
pub struct Lvgroup<'a> {
    pub cbSize: u32,
    pub mask: u32,
    pub pszHeader: &'a mut Cow<'a, OsStr>,
    pub cchHeader: i32,
    pub pszFooter: &'a mut Cow<'a, OsStr>,
    pub cchFooter: i32,
    pub iGroupId: i32,
    pub stateMask: u32,
    pub state: u32,
    pub uAlign: u32,
    pub pszSubtitle: &'a mut Cow<'a, OsStr>,
    pub cchSubtitle: u32,
    pub pszTask: &'a mut Cow<'a, OsStr>,
    pub cchTask: u32,
    pub pszDescriptionTop: &'a mut Cow<'a, OsStr>,
    pub cchDescriptionTop: u32,
    pub pszDescriptionBottom: &'a mut Cow<'a, OsStr>,
    pub cchDescriptionBottom: u32,
    pub iTitleImage: i32,
    pub iExtendedImage: i32,
    pub iFirstItem: i32,
    pub cItems: u32,
    pub pszSubsetTitle: &'a mut Cow<'a, OsStr>,
    pub cchSubsetTitle: u32,
}
pub struct Lvgroupmetrics {
    pub cbSize: u32,
    pub mask: u32,
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub crLeft: u32,
    pub crTop: u32,
    pub crRight: u32,
    pub crBottom: u32,
    pub crHeader: u32,
    pub crFooter: u32,
}
pub const LVGF_NONE: u32 = 0x0;
pub const LVGF_HEADER: u32 = 0x1;
pub const LVGF_FOOTER: u32 = 0x2;
pub const LVGF_STATE: u32 = 0x4;
pub struct Lvhittestinfo {
    pub pt: POINT,
    pub flags: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub iGroup: i32,
}
pub const LVHT_ABOVE: u32 = 0x8;
pub const LVHT_BELOW: u32 = 0x10;
pub const LVHT_NOWHERE: u32 = 0x1;
pub const LVHT_ONITEMICON: u32 = 0x2;
pub const LVHT_ONITEMLABEL: u32 = 0x4;
pub const LVHT_ONITEMSTATEICON: u32 = 0x8;
pub const LVHT_TOLEFT: u32 = 0x40;
pub const LVHT_TORIGHT: u32 = 0x20;
pub const LVHT_EX_GROUP_HEADER: u32 = 0x10000000;
pub const LVHT_EX_GROUP_FOOTER: u32 = 0x20000000;
pub const LVHT_EX_GROUP_COLLAPSE: u32 = 0x40000000;
pub const LVHT_EX_GROUP_BACKGROUND: u32 = 0x80000000;
pub const LVHT_EX_GROUP_STATEICON: u32 = 0x1000000;
pub const LVHT_EX_GROUP_SUBSETLINK: u32 = 0x2000000;
pub const LVHT_EX_GROUP: u32 = 0xF3000000;
pub const LVHT_EX_ONCONTENTS: u32 = 0x4000000;
pub const LVHT_EX_FOOTER: u32 = 0x8000000;
pub struct Lvinsertgroupsorted<'a> {
    pub pfnGroupCompare: todo_fn,
    pub pvData: &'a mut todo_void,
    pub lvGroup: LVGROUP<'a>,
}
pub struct Lvinsertmark {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iItem: i32,
    pub dwReserved: u32,
}
pub struct Lvitema<'a> {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: LPARAM,
    pub iIndent: i32,
    pub iGroupId: i32,
    pub cColumns: u32,
    pub puColumns: &'a mut u32,
    pub piColFmt: &'a mut i32,
    pub iGroup: i32,
}
pub const I_GROUPIDCALLBACK: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const I_GROUPIDNONE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE;
pub struct Lvitemindex {
    pub iItem: i32,
    pub iGroup: i32,
}
pub struct Lvitemw<'a> {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: LPARAM,
    pub iIndent: i32,
    pub iGroupId: i32,
    pub cColumns: u32,
    pub puColumns: &'a mut u32,
    pub piColFmt: &'a mut i32,
    pub iGroup: i32,
}
pub struct Lvsetinfotip<'a> {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub iItem: i32,
    pub iSubItem: i32,
}
pub struct Lvtileinfo<'a> {
    pub cbSize: u32,
    pub iItem: i32,
    pub cColumns: u32,
    pub puColumns: &'a mut u32,
    pub piColFmt: &'a mut i32,
}
pub struct Lvtileviewinfo {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwFlags: u32,
    pub sizeTile: SIZE,
    pub cLines: i32,
    pub rcLabelMargin: RECT,
}
pub const LVTVIF_EXTENDED: u32 = 0x4;
pub struct LocalThumbnailCache {
}
pub struct Margins {
    pub cxLeftWidth: i32,
    pub cxRightWidth: i32,
    pub cyTopHeight: i32,
    pub cyBottomHeight: i32,
}
pub const EMT_NORMALTEXT: i32 = 0x1;
pub const EMT_LINKTEXT: i32 = 0x2;
pub struct Mat2 {
    pub eM11: FIXED,
    pub eM12: FIXED,
    pub eM21: FIXED,
    pub eM22: FIXED,
}
pub struct Mcgridinfo<'a> {
    pub cbSize: u32,
    pub dwPart: u32,
    pub dwFlags: u32,
    pub iCalendar: i32,
    pub iRow: i32,
    pub iCol: i32,
    pub bSelected: BOOL,
    pub stStart: SYSTEMTIME,
    pub stEnd: SYSTEMTIME,
    pub rc: RECT,
    pub pszName: &'a mut Cow<'a, OsStr>,
    pub cchName: usize,
}
pub const MCGIF_DATE: u32 = 0x1;
pub const MCGIF_RECT: u32 = 0x2;
pub const MCGIF_NAME: u32 = 0x4;
pub const MCGIP_CALENDARCONTROL: u32 = 0x0;
pub const MCGIP_NEXT: u32 = 0x1;
pub const MCGIP_PREV: u32 = 0x2;
pub const MCGIP_FOOTER: u32 = 0x3;
pub const MCGIP_CALENDAR: u32 = 0x4;
pub const MCGIP_CALENDARHEADER: u32 = 0x5;
pub const MCGIP_CALENDARBODY: u32 = 0x6;
pub const MCGIP_CALENDARROW: u32 = 0x7;
pub const MCGIP_CALENDARCELL: u32 = 0x8;
pub struct Mchittestinfo {
    pub cbSize: u32,
    pub pt: POINT,
    pub uHit: u32,
    pub st: SYSTEMTIME,
    pub rc: RECT,
    pub iOffset: i32,
    pub iRow: i32,
    pub iCol: i32,
}
pub struct Mdicreatestructa<'a> {
    pub szClass: &'a Cow<'a, str>,
    pub szTitle: &'a Cow<'a, str>,
    pub hOwner: HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: u32,
    pub lParam: LPARAM,
}
pub struct Mdicreatestructw<'a> {
    pub szClass: &'a Cow<'a, OsStr>,
    pub szTitle: &'a Cow<'a, OsStr>,
    pub hOwner: HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: u32,
    pub lParam: LPARAM,
}
pub struct Mdinextmenu {
    pub hmenuIn: HMENU,
    pub hmenuNext: HMENU,
    pub hwndNext: HWND,
}
pub struct Measureitemstruct {
    pub CtlType: u32,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemWidth: u32,
    pub itemHeight: u32,
    pub itemData: usize,
}
pub const MBHANDCID_PIDLSELECT: i32 = 0x0;
pub const MDP_NEWAPPBUTTON: i32 = 0x1;
pub const MDP_SEPERATOR: i32 = 0x2;
pub const MDS_NORMAL: i32 = 0x1;
pub const MDS_HOT: i32 = 0x2;
pub const MDS_PRESSED: i32 = 0x3;
pub const MDS_DISABLED: i32 = 0x4;
pub const MDS_CHECKED: i32 = 0x5;
pub const MDS_HOTCHECKED: i32 = 0x6;
pub struct Menubarinfo {
    pub cbSize: u32,
    pub rcBar: RECT,
    pub hMenu: HMENU,
    pub hwndMenu: HWND,
    pub _bitfield: i32,
}
pub struct Menugetobjectinfo<'a> {
    pub dwFlags: u32,
    pub uPos: u32,
    pub hmenu: HMENU,
    pub riid: &'a mut todo_void,
    pub pvObj: &'a mut todo_void,
}
pub const MNGOF_BOTTOMGAP: u32 = 0x2;
pub const MNGOF_TOPGAP: u32 = 0x1;
pub struct Menuinfo {
    pub cbSize: u32,
    pub fMask: u32,
    pub dwStyle: u32,
    pub cyMax: u32,
    pub hbrBack: HBRUSH,
    pub dwContextHelpID: u32,
    pub dwMenuData: usize,
}
pub const MIM_APPLYTOSUBMENUS: u32 = 0x80000000;
pub const MIM_BACKGROUND: u32 = 0x2;
pub const MIM_HELPID: u32 = 0x4;
pub const MIM_MAXHEIGHT: u32 = 0x1;
pub const MIM_MENUDATA: u32 = 0x8;
pub const MIM_STYLE: u32 = 0x10;
pub const MNS_AUTODISMISS: u32 = 0x10000000;
pub const MNS_CHECKORBMP: u32 = 0x4000000;
pub const MNS_DRAGDROP: u32 = 0x20000000;
pub const MNS_MODELESS: u32 = 0x40000000;
pub const MNS_NOCHECK: u32 = 0x80000000;
pub const MNS_NOTIFYBYPOS: u32 = 0x8000000;
pub struct Menuiteminfoa<'a> {
    pub cbSize: u32,
    pub fMask: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hSubMenu: HMENU,
    pub hbmpChecked: HBITMAP,
    pub hbmpUnchecked: HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: &'a mut Cow<'a, str>,
    pub cch: u32,
    pub hbmpItem: HBITMAP,
}
pub struct Menuiteminfow<'a> {
    pub cbSize: u32,
    pub fMask: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hSubMenu: HMENU,
    pub hbmpChecked: HBITMAP,
    pub hbmpUnchecked: HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: &'a mut Cow<'a, OsStr>,
    pub cch: u32,
    pub hbmpItem: HBITMAP,
}
pub struct Menuitemtemplate {
    pub mtOption: u16,
    pub mtID: u16,
    pub mtString: [u8; 1],
}
pub struct Menuitemtemplateheader {
    pub versionNumber: u16,
    pub offset: u16,
}
pub const MPPF_SETFOCUS: i32 = 0x1;
pub const MPPF_INITIALSELECT: i32 = 0x2;
pub const MPPF_NOANIMATE: i32 = 0x4;
pub const MPPF_KEYBOARD: i32 = 0x10;
pub const MPPF_REPOSITION: i32 = 0x20;
pub const MPPF_FORCEZORDER: i32 = 0x40;
pub const MPPF_FINALSELECT: i32 = 0x80;
pub const MPPF_TOP: i32 = 0x20000000;
pub const MPPF_LEFT: i32 = 0x40000000;
pub const MPPF_RIGHT: i32 = 0x60000000;
pub const MPPF_BOTTOM: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000;
pub const MPPF_POS_MASK: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFE0000000;
pub const MPPF_ALIGN_LEFT: i32 = 0x2000000;
pub const MPPF_ALIGN_RIGHT: i32 = 0x4000000;
pub const MPOS_EXECUTE: i32 = 0x0;
pub const MPOS_FULLCANCEL: i32 = 0x1;
pub const MPOS_CANCELLEVEL: i32 = 0x2;
pub const MPOS_SELECTLEFT: i32 = 0x3;
pub const MPOS_SELECTRIGHT: i32 = 0x4;
pub const MPOS_CHILDTRACKING: i32 = 0x5;
pub const MF_BYCOMMAND: u32 = 0x0;
pub const MF_BYPOSITION: u32 = 0x400;
pub const MF_BITMAP: u32 = 0x4;
pub const MF_CHECKED: u32 = 0x8;
pub const MF_DISABLED: u32 = 0x2;
pub const MF_ENABLED: u32 = 0x0;
pub const MF_GRAYED: u32 = 0x1;
pub const MF_MENUBARBREAK: u32 = 0x20;
pub const MF_MENUBREAK: u32 = 0x40;
pub const MF_OWNERDRAW: u32 = 0x100;
pub const MF_POPUP: u32 = 0x10;
pub const MF_SEPARATOR: u32 = 0x800;
pub const MF_STRING: u32 = 0x0;
pub const MF_UNCHECKED: u32 = 0x0;
pub const MF_INSERT: u32 = 0x0;
pub const MF_CHANGE: u32 = 0x80;
pub const MF_APPEND: u32 = 0x100;
pub const MF_DELETE: u32 = 0x200;
pub const MF_REMOVE: u32 = 0x1000;
pub const MF_USECHECKBITMAPS: u32 = 0x200;
pub const MF_UNHILITE: u32 = 0x0;
pub const MF_HILITE: u32 = 0x80;
pub const MF_DEFAULT: u32 = 0x1000;
pub const MF_SYSMENU: u32 = 0x2000;
pub const MF_HELP: u32 = 0x4000;
pub const MF_RIGHTJUSTIFY: u32 = 0x4000;
pub const MF_MOUSESELECT: u32 = 0x8000;
pub const MF_END: u32 = 0x80;
pub const MIIM_BITMAP: u32 = 0x80;
pub const MIIM_CHECKMARKS: u32 = 0x8;
pub const MIIM_DATA: u32 = 0x20;
pub const MIIM_FTYPE: u32 = 0x100;
pub const MIIM_ID: u32 = 0x2;
pub const MIIM_STATE: u32 = 0x1;
pub const MIIM_STRING: u32 = 0x40;
pub const MIIM_SUBMENU: u32 = 0x4;
pub const MIIM_TYPE: u32 = 0x10;
pub const MFS_GRAYED: u32 = 0x3;
pub const MFS_DISABLED: u32 = 0x3;
pub const MFS_CHECKED: u32 = 0x8;
pub const MFS_HILITE: u32 = 0x80;
pub const MFS_ENABLED: u32 = 0x0;
pub const MFS_UNCHECKED: u32 = 0x0;
pub const MFS_UNHILITE: u32 = 0x0;
pub const MFS_DEFAULT: u32 = 0x1000;
pub const MFT_BITMAP: u32 = 0x4;
pub const MFT_MENUBARBREAK: u32 = 0x20;
pub const MFT_MENUBREAK: u32 = 0x40;
pub const MFT_OWNERDRAW: u32 = 0x100;
pub const MFT_RADIOCHECK: u32 = 0x200;
pub const MFT_RIGHTJUSTIFY: u32 = 0x4000;
pub const MFT_RIGHTORDER: u32 = 0x2000;
pub const MFT_SEPARATOR: u32 = 0x800;
pub const MFT_STRING: u32 = 0x0;
pub const MUS_COMPLETE: i32 = 0x0;
pub const MUS_USERINPUTNEEDED: i32 = 0x1;
pub const MUS_FAILED: i32 = 0x2;
pub const IDOK: i32 = 0x1;
pub const IDCANCEL: i32 = 0x2;
pub const IDABORT: i32 = 0x3;
pub const IDRETRY: i32 = 0x4;
pub const IDIGNORE: i32 = 0x5;
pub const IDYES: i32 = 0x6;
pub const IDNO: i32 = 0x7;
pub const IDCLOSE: i32 = 0x8;
pub const IDHELP: i32 = 0x9;
pub const IDTRYAGAIN: i32 = 0xA;
pub const IDCONTINUE: i32 = 0xB;
pub const IDASYNC: i32 = 0x7D01;
pub const IDTIMEOUT: i32 = 0x7D00;
pub const MB_ABORTRETRYIGNORE: u32 = 0x2;
pub const MB_CANCELTRYCONTINUE: u32 = 0x6;
pub const MB_HELP: u32 = 0x4000;
pub const MB_OK: u32 = 0x0;
pub const MB_OKCANCEL: u32 = 0x1;
pub const MB_RETRYCANCEL: u32 = 0x5;
pub const MB_YESNO: u32 = 0x4;
pub const MB_YESNOCANCEL: u32 = 0x3;
pub const MB_ICONHAND: u32 = 0x10;
pub const MB_ICONQUESTION: u32 = 0x20;
pub const MB_ICONEXCLAMATION: u32 = 0x30;
pub const MB_ICONASTERISK: u32 = 0x40;
pub const MB_USERICON: u32 = 0x80;
pub const MB_ICONWARNING: u32 = 0x30;
pub const MB_ICONERROR: u32 = 0x10;
pub const MB_ICONINFORMATION: u32 = 0x40;
pub const MB_ICONSTOP: u32 = 0x10;
pub const MB_DEFBUTTON1: u32 = 0x0;
pub const MB_DEFBUTTON2: u32 = 0x100;
pub const MB_DEFBUTTON3: u32 = 0x200;
pub const MB_DEFBUTTON4: u32 = 0x300;
pub const MB_APPLMODAL: u32 = 0x0;
pub const MB_SYSTEMMODAL: u32 = 0x1000;
pub const MB_TASKMODAL: u32 = 0x2000;
pub const MB_NOFOCUS: u32 = 0x8000;
pub const MB_SETFOREGROUND: u32 = 0x10000;
pub const MB_DEFAULT_DESKTOP_ONLY: u32 = 0x20000;
pub const MB_TOPMOST: u32 = 0x40000;
pub const MB_RIGHT: u32 = 0x80000;
pub const MB_RTLREADING: u32 = 0x100000;
pub const MB_SERVICE_NOTIFICATION: u32 = 0x200000;
pub const MB_SERVICE_NOTIFICATION_NT3X: u32 = 0x40000;
pub const MB_TYPEMASK: u32 = 0xF;
pub const MB_ICONMASK: u32 = 0xF0;
pub const MB_DEFMASK: u32 = 0xF00;
pub const MB_MODEMASK: u32 = 0x3000;
pub const MB_MISCMASK: u32 = 0xC000;
pub struct MessageResourceBlock {
    pub LowId: u32,
    pub HighId: u32,
    pub OffsetToEntries: u32,
}
pub struct MessageResourceData {
    pub NumberOfBlocks: u32,
    pub Blocks: [MESSAGE_RESOURCE_BLOCK; 1],
}
pub struct MessageResourceEntry {
    pub Length: u16,
    pub Flags: u16,
    pub Text: [u8; 1],
}
pub struct Metaheader {
    pub mtType: u16,
    pub mtHeaderSize: u16,
    pub mtVersion: u16,
    pub mtSize: u32,
    pub mtNoObjects: u16,
    pub mtMaxRecord: u32,
    pub mtNoParameters: u16,
}
pub struct Metarecord {
    pub rdSize: u32,
    pub rdFunction: u16,
    pub rdParm: [u16; 1],
}
pub struct Minimizedmetrics {
    pub cbSize: u32,
    pub iWidth: i32,
    pub iHorzGap: i32,
    pub iVertGap: i32,
    pub iArrange: i32,
}
pub const ARW_BOTTOMLEFT: i32 = 0x0;
pub const ARW_BOTTOMRIGHT: i32 = 0x1;
pub const ARW_TOPLEFT: i32 = 0x2;
pub const ARW_TOPRIGHT: i32 = 0x3;
pub struct Minmaxinfo {
    pub ptReserved: POINT,
    pub ptMaxSize: POINT,
    pub ptMaxPosition: POINT,
    pub ptMinTrackSize: POINT,
    pub ptMaxTrackSize: POINT,
}
pub const MM_ADDSEPARATOR: u32 = 0x1;
pub const MM_SUBMENUSHAVEIDS: u32 = 0x2;
pub const MM_DONTREMOVESEPS: u32 = 0x4;
pub const MWT_IDENTITY: u32 = 0x1;
pub const MWT_LEFTMULTIPLY: u32 = 0x2;
pub const MWT_RIGHTMULTIPLY: u32 = 0x3;
pub struct Monitorinfo {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}
pub struct Monitorinfoexa {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [CHAR; 32],
}
pub struct Monitorinfoexw {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [u8; 32],
}
pub const MAV_UNKNOWN: i32 = 0x0;
pub const MAV_NO_APP_VISIBLE: i32 = 0x1;
pub const MAV_APP_VISIBLE: i32 = 0x2;
pub const MONITOR_DEFAULTTONEAREST: u32 = 0x2;
pub const MONITOR_DEFAULTTONULL: u32 = 0x0;
pub const MONITOR_DEFAULTTOPRIMARY: u32 = 0x1;
pub const MC_BACKGROUND: i32 = 0x1;
pub const MC_BORDERS: i32 = 0x2;
pub const MC_GRIDBACKGROUND: i32 = 0x3;
pub const MC_COLHEADERSPLITTER: i32 = 0x4;
pub const MC_GRIDCELLBACKGROUND: i32 = 0x5;
pub const MC_GRIDCELL: i32 = 0x6;
pub const MC_GRIDCELLUPPER: i32 = 0x7;
pub const MC_TRAILINGGRIDCELL: i32 = 0x8;
pub const MC_TRAILINGGRIDCELLUPPER: i32 = 0x9;
pub const MC_NAVNEXT: i32 = 0xA;
pub const MC_NAVPREV: i32 = 0xB;
pub const SPSB_NORMAL: i32 = 0x1;
pub const SPSB_HOT: i32 = 0x2;
pub const SPSB_PRESSED: i32 = 0x3;
pub const SPS_NORMAL: i32 = 0x1;
pub const SPS_HOT: i32 = 0x2;
pub const SPS_PRESSED: i32 = 0x3;
pub const SPMPT_NORMAL: i32 = 0x1;
pub const SPMPT_HOT: i32 = 0x2;
pub const SPMPT_SELECTED: i32 = 0x3;
pub const SPMPT_DISABLED: i32 = 0x4;
pub const SPMPT_FOCUSED: i32 = 0x5;
pub struct Mousehookstruct {
    pub pt: POINT,
    pub hwnd: HWND,
    pub wHitTestCode: u32,
    pub dwExtraInfo: usize,
}
pub struct Mousehookstructex {
    pub __AnonymousBase_winuser_L1166_C46: MOUSEHOOKSTRUCT,
    pub mouseData: u32,
}
pub const XBUTTON1: u32 = 0x1;
pub const XBUTTON2: u32 = 0x2;
pub struct Msg {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}
pub struct Msgboxparamsa<'a> {
    pub cbSize: u32,
    pub hwndOwner: HWND,
    pub hInstance: HINSTANCE,
    pub lpszText: &'a Cow<'a, str>,
    pub lpszCaption: &'a Cow<'a, str>,
    pub dwStyle: u32,
    pub lpszIcon: &'a Cow<'a, str>,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: todo_fn,
    pub dwLanguageId: u32,
}
pub struct Msgboxparamsw<'a> {
    pub cbSize: u32,
    pub hwndOwner: HWND,
    pub hInstance: HINSTANCE,
    pub lpszText: &'a Cow<'a, OsStr>,
    pub lpszCaption: &'a Cow<'a, OsStr>,
    pub dwStyle: u32,
    pub lpszIcon: &'a Cow<'a, OsStr>,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: todo_fn,
    pub dwLanguageId: u32,
}
pub const MSGFLTINFO_NONE: u32 = 0x0;
pub const MSGFLTINFO_ALLOWED_HIGHER: u32 = 0x3;
pub const MSGFLTINFO_ALREADYALLOWED_FORWND: u32 = 0x1;
pub const MSGFLTINFO_ALREADYDISALLOWED_FORWND: u32 = 0x2;
pub const MWMO_NONE: u32 = 0x0;
pub const MWMO_ALERTABLE: u32 = 0x2;
pub const MWMO_INPUTAVAILABLE: u32 = 0x4;
pub const MWMO_WAITALL: u32 = 0x1;
pub struct Msllhookstruct {
    pub pt: POINT,
    pub mouseData: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub struct Multikeyhelpa {
    pub mkSize: u32,
    pub mkKeylist: CHAR,
    pub szKeyphrase: [CHAR; 1],
}
pub struct Multikeyhelpw {
    pub mkSize: u32,
    pub mkKeylist: u8,
    pub szKeyphrase: [u8; 1],
}
pub struct MailRecipient {
}
pub struct MergedCategorizer {
}
pub const MRM_DUMP_TYPE_BASIC: i32 = 0x0;
pub const MRM_DUMP_TYPE_DETAILED: i32 = 0x1;
pub const MRM_DUMP_TYPE_SCHEMA: i32 = 0x2;
pub const MRM_INDEXER_FLAGS_NONE: i32 = 0x0;
pub const MRM_INDEXER_FLAGS_AUTO_MERGE: i32 = 0x1;
pub const MRM_INDEXER_FLAGS_CREATE_CONTENT_CHECKSUM: i32 = 0x2;
pub const MRM_PACKAGING_MODE_STANDALONE_FILE: i32 = 0x0;
pub const MRM_PACKAGING_MODE_AUTO_SPLIT: i32 = 0x1;
pub const MRM_PACKAGING_MODE_RESOURCE_PACK: i32 = 0x2;
pub const MRM_PACKAGING_OPTIONS_NONE: i32 = 0x0;
pub const MRM_PACKAGING_OPTIONS_OMIT_SCHEMA_FROM_RESOURCE_PACKS: i32 = 0x1;
pub const MRM_PACKAGING_OPTIONS_SPLIT_LANGUAGE_VARIANTS: i32 = 0x2;
pub const MRM_PLATFORM_VERSION_DEFAULT: i32 = 0x0;
pub const MRM_PLATFORM_VERSION_WINDOWS10_0_0_0: i32 = 0x10A0000;
pub const MRM_PLATFORM_VERSION_WINDOWS10_0_0_5: i32 = 0x10A0005;
pub struct MrmResourceIndexerHandle<'a> {
    pub handle: &'a mut todo_void,
}
pub struct MrmResourceIndexerMessage<'a> {
    pub severity: i32,
    pub id: u32,
    pub text: &'a Cow<'a, OsStr>,
}
pub const MRM_RESOURCE_INDEXER_MESSAGE_SEVERITY_VERBOSE: i32 = 0x0;
pub const MRM_RESOURCE_INDEXER_MESSAGE_SEVERITY_INFO: i32 = 0x1;
pub const MRM_RESOURCE_INDEXER_MESSAGE_SEVERITY_WARNING: i32 = 0x2;
pub const MRM_RESOURCE_INDEXER_MESSAGE_SEVERITY_ERROR: i32 = 0x3;
pub const NSWF_DEFAULT: i32 = 0x0;
pub const NSWF_NONE_IMPLIES_ALL: i32 = 0x1;
pub const NSWF_ONE_IMPLIES_ALL: i32 = 0x2;
pub const NSWF_DONT_TRAVERSE_LINKS: i32 = 0x4;
pub const NSWF_DONT_ACCUMULATE_RESULT: i32 = 0x8;
pub const NSWF_TRAVERSE_STREAM_JUNCTIONS: i32 = 0x10;
pub const NSWF_FILESYSTEM_ONLY: i32 = 0x20;
pub const NSWF_SHOW_PROGRESS: i32 = 0x40;
pub const NSWF_FLAG_VIEWORDER: i32 = 0x80;
pub const NSWF_IGNORE_AUTOPLAY_HIDA: i32 = 0x100;
pub const NSWF_ASYNC: i32 = 0x200;
pub const NSWF_DONT_RESOLVE_LINKS: i32 = 0x400;
pub const NSWF_ACCUMULATE_FOLDERS: i32 = 0x800;
pub const NSWF_DONT_SORT: i32 = 0x1000;
pub const NSWF_USE_TRANSFER_MEDIUM: i32 = 0x2000;
pub const NSWF_DONT_TRAVERSE_STREAM_JUNCTIONS: i32 = 0x4000;
pub const NSWF_ANY_IMPLIES_ALL: i32 = 0x8000;
pub const NDO_LANDSCAPE: i32 = 0x0;
pub const NDO_PORTRAIT: i32 = 0x1;
pub const MCNN_NORMAL: i32 = 0x1;
pub const MCNN_HOT: i32 = 0x2;
pub const MCNN_PRESSED: i32 = 0x3;
pub const MCNN_DISABLED: i32 = 0x4;
pub const MCNP_NORMAL: i32 = 0x1;
pub const MCNP_HOT: i32 = 0x2;
pub const MCNP_PRESSED: i32 = 0x3;
pub const MCNP_DISABLED: i32 = 0x4;
pub struct NccalcsizeParams<'a> {
    pub rgrc: [RECT; 3],
    pub lppos: &'a mut WINDOWPOS,
}
pub struct Newcplinfoa {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHelpContext: u32,
    pub lData: isize,
    pub hIcon: HICON,
    pub szName: [CHAR; 32],
    pub szInfo: [CHAR; 64],
    pub szHelpFile: [CHAR; 128],
}
pub struct Newcplinfow {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHelpContext: u32,
    pub lData: isize,
    pub hIcon: HICON,
    pub szName: [u8; 32],
    pub szInfo: [u8; 64],
    pub szHelpFile: [u8; 128],
}
pub struct Newtextmetrica {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
pub struct Newtextmetricw {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
pub struct Nmbcdropdown {
    pub hdr: NMHDR,
    pub rcButton: RECT,
}
pub struct Nmbchotitem {
    pub hdr: NMHDR,
    pub dwFlags: u32,
}
pub struct Nmcbedragbegina {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [CHAR; 260],
}
pub struct Nmcbedragbeginw {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [u8; 260],
}
pub struct Nmcbeendedita {
    pub hdr: NMHDR,
    pub fChanged: BOOL,
    pub iNewSelection: i32,
    pub szText: [CHAR; 260],
    pub iWhy: i32,
}
pub struct Nmcbeendeditw {
    pub hdr: NMHDR,
    pub fChanged: BOOL,
    pub iNewSelection: i32,
    pub szText: [u8; 260],
    pub iWhy: i32,
}
pub struct Nmchar {
    pub hdr: NMHDR,
    pub ch: u32,
    pub dwItemPrev: u32,
    pub dwItemNext: u32,
}
pub struct Nmcomboboxexa<'a> {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMA<'a>,
}
pub struct Nmcomboboxexw<'a> {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMW<'a>,
}
pub struct Nmcustomdraw {
    pub hdr: NMHDR,
    pub dwDrawStage: u32,
    pub hdc: HDC,
    pub rc: RECT,
    pub dwItemSpec: usize,
    pub uItemState: u32,
    pub lItemlParam: LPARAM,
}
pub const CDDS_POSTPAINT: u32 = 0x2;
pub const CDDS_PREERASE: u32 = 0x3;
pub const CDDS_PREPAINT: u32 = 0x1;
pub const CDDS_ITEMPOSTERASE: u32 = 0x10004;
pub const CDDS_ITEMPOSTPAINT: u32 = 0x10002;
pub const CDDS_ITEMPREERASE: u32 = 0x10003;
pub const CDDS_ITEMPREPAINT: u32 = 0x10001;
pub const CDDS_SUBITEM: u32 = 0x20000;
pub struct Nmcustomsplitrectinfo {
    pub hdr: NMHDR,
    pub rcClient: RECT,
    pub rcButton: RECT,
    pub rcSplit: RECT,
}
pub struct Nmcustomtext<'a> {
    pub hdr: NMHDR,
    pub hDC: HDC,
    pub lpString: &'a Cow<'a, OsStr>,
    pub nCount: i32,
    pub lpRect: &'a mut RECT,
    pub uFormat: u32,
    pub fLink: BOOL,
}
pub struct Nmdatetimechange {
    pub nmhdr: NMHDR,
    pub dwFlags: u32,
    pub st: SYSTEMTIME,
}
pub struct Nmdatetimeformata<'a> {
    pub nmhdr: NMHDR,
    pub pszFormat: &'a Cow<'a, str>,
    pub st: SYSTEMTIME,
    pub pszDisplay: &'a Cow<'a, str>,
    pub szDisplay: [CHAR; 64],
}
pub struct Nmdatetimeformatquerya<'a> {
    pub nmhdr: NMHDR,
    pub pszFormat: &'a Cow<'a, str>,
    pub szMax: SIZE,
}
pub struct Nmdatetimeformatqueryw<'a> {
    pub nmhdr: NMHDR,
    pub pszFormat: &'a Cow<'a, OsStr>,
    pub szMax: SIZE,
}
pub struct Nmdatetimeformatw<'a> {
    pub nmhdr: NMHDR,
    pub pszFormat: &'a Cow<'a, OsStr>,
    pub st: SYSTEMTIME,
    pub pszDisplay: &'a Cow<'a, OsStr>,
    pub szDisplay: [u8; 64],
}
pub struct Nmdatetimestringa<'a> {
    pub nmhdr: NMHDR,
    pub pszUserString: &'a Cow<'a, str>,
    pub st: SYSTEMTIME,
    pub dwFlags: u32,
}
pub struct Nmdatetimestringw<'a> {
    pub nmhdr: NMHDR,
    pub pszUserString: &'a Cow<'a, OsStr>,
    pub st: SYSTEMTIME,
    pub dwFlags: u32,
}
pub struct Nmdatetimewmkeydowna<'a> {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: &'a Cow<'a, str>,
    pub st: SYSTEMTIME,
}
pub struct Nmdatetimewmkeydownw<'a> {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: &'a Cow<'a, OsStr>,
    pub st: SYSTEMTIME,
}
pub struct Nmdaystate<'a> {
    pub nmhdr: NMHDR,
    pub stStart: SYSTEMTIME,
    pub cDayState: i32,
    pub prgDayState: &'a mut u32,
}
pub struct Nmhddispinfoa<'a> {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: u32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: LPARAM,
}
pub struct Nmhddispinfow<'a> {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: LPARAM,
}
pub struct Nmhdfilterbtnclick {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub rc: RECT,
}
pub struct Nmhdr {
    pub hwndFrom: HWND,
    pub idFrom: usize,
    pub code: u32,
}
pub struct Nmheadera<'a> {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: u32,
    pub pitem: &'a mut HDITEMA<'a>,
}
pub struct Nmheaderw<'a> {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: u32,
    pub pitem: &'a mut HDITEMW<'a>,
}
pub struct Nmipaddress {
    pub hdr: NMHDR,
    pub iField: i32,
    pub iValue: i32,
}
pub struct Nmitemactivate {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: POINT,
    pub lParam: LPARAM,
    pub uKeyFlags: u32,
}
pub struct Nmkey {
    pub hdr: NMHDR,
    pub nVKey: u32,
    pub uFlags: u32,
}
pub struct Nmlink {
    pub hdr: NMHDR,
    pub item: LITEM,
}
pub struct Nmlistview {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: POINT,
    pub lParam: LPARAM,
}
pub struct Nmlvcachehint {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
}
pub struct Nmlvcustomdraw {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: u32,
    pub clrTextBk: u32,
    pub iSubItem: i32,
    pub dwItemType: u32,
    pub clrFace: u32,
    pub iIconEffect: i32,
    pub iIconPhase: i32,
    pub iPartId: i32,
    pub iStateId: i32,
    pub rcText: RECT,
    pub uAlign: u32,
}
pub const LVGA_HEADER_CENTER: u32 = 0x2;
pub const LVGA_HEADER_LEFT: u32 = 0x1;
pub const LVGA_HEADER_RIGHT: u32 = 0x4;
pub const LVCDI_ITEM: u32 = 0x0;
pub const LVCDI_GROUP: u32 = 0x1;
pub const LVCDI_ITEMSLIST: u32 = 0x2;
pub struct Nmlvdispinfoa<'a> {
    pub hdr: NMHDR,
    pub item: LVITEMA<'a>,
}
pub struct Nmlvdispinfow<'a> {
    pub hdr: NMHDR,
    pub item: LVITEMW<'a>,
}
pub struct Nmlvemptymarkup {
    pub hdr: NMHDR,
    pub dwFlags: u32,
    pub szMarkup: [u8; 2084],
}
pub const EMF_CENTERED: u32 = 0x1;
pub struct Nmlvfinditema<'a> {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOA<'a>,
}
pub struct Nmlvfinditemw<'a> {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOW<'a>,
}
pub struct Nmlvgetinfotipa<'a> {
    pub hdr: NMHDR,
    pub dwFlags: u32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: LPARAM,
}
pub struct Nmlvgetinfotipw<'a> {
    pub hdr: NMHDR,
    pub dwFlags: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: LPARAM,
}
pub struct Nmlvkeydown {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
pub struct Nmlvlink {
    pub hdr: NMHDR,
    pub link: LITEM,
    pub iItem: i32,
    pub iSubItem: i32,
}
pub struct Nmlvodstatechange {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
    pub uNewState: u32,
    pub uOldState: u32,
}
pub struct Nmlvscroll {
    pub hdr: NMHDR,
    pub dx: i32,
    pub dy: i32,
}
pub struct Nmmouse {
    pub hdr: NMHDR,
    pub dwItemSpec: usize,
    pub dwItemData: usize,
    pub pt: POINT,
    pub dwHitInfo: LPARAM,
}
pub struct Nmobjectnotify<'a> {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub piid: &'a GUID,
    pub pObject: &'a mut todo_void,
    pub hResult: HRESULT,
    pub dwFlags: u32,
}
pub struct Nmpgcalcsize {
    pub hdr: NMHDR,
    pub dwFlag: u32,
    pub iWidth: i32,
    pub iHeight: i32,
}
pub const PGF_CALCHEIGHT: u32 = 0x2;
pub const PGF_CALCWIDTH: u32 = 0x1;
pub struct Nmpghotitem {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
pub struct Nmpgscroll {
    pub hdr: NMHDR,
    pub fwKeys: u16,
    pub rcParent: RECT,
    pub iDir: u32,
    pub iXpos: i32,
    pub iYpos: i32,
    pub iScroll: i32,
}
pub const PGF_SCROLLDOWN: u32 = 0x2;
pub const PGF_SCROLLLEFT: u32 = 0x4;
pub const PGF_SCROLLRIGHT: u32 = 0x8;
pub const PGF_SCROLLUP: u32 = 0x1;
pub const PGK_NONE: u16 = 0x0;
pub const PGK_SHIFT: u16 = 0x1;
pub const PGK_CONTROL: u16 = 0x2;
pub const PGK_MENU: u16 = 0x4;
pub struct Nmrbautosize {
    pub hdr: NMHDR,
    pub fChanged: BOOL,
    pub rcTarget: RECT,
    pub rcActual: RECT,
}
pub struct Nmrebar {
    pub hdr: NMHDR,
    pub dwMask: u32,
    pub uBand: u32,
    pub fStyle: u32,
    pub wID: u32,
    pub lParam: LPARAM,
}
pub struct Nmrebarautobreak {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: LPARAM,
    pub uMsg: u32,
    pub fStyleCurrent: u32,
    pub fAutoBreak: BOOL,
}
pub struct Nmrebarchevron {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: LPARAM,
    pub rc: RECT,
    pub lParamNM: LPARAM,
}
pub struct Nmrebarchildsize {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub rcChild: RECT,
    pub rcBand: RECT,
}
pub struct Nmrebarsplitter {
    pub hdr: NMHDR,
    pub rcSizing: RECT,
}
pub const RBNM_ID: u32 = 0x1;
pub const RBNM_LPARAM: u32 = 0x4;
pub const RBNM_STYLE: u32 = 0x2;
pub struct Nmsearchweb {
    pub hdr: NMHDR,
    pub entrypoint: i32,
    pub hasQueryText: BOOL,
    pub invokeSucceeded: BOOL,
}
pub struct Nmselchange {
    pub nmhdr: NMHDR,
    pub stSelStart: SYSTEMTIME,
    pub stSelEnd: SYSTEMTIME,
}
pub struct Nmtbcustomdraw {
    pub nmcd: NMCUSTOMDRAW,
    pub hbrMonoDither: HBRUSH,
    pub hbrLines: HBRUSH,
    pub hpenLines: HPEN,
    pub clrText: u32,
    pub clrMark: u32,
    pub clrTextHighlight: u32,
    pub clrBtnFace: u32,
    pub clrBtnHighlight: u32,
    pub clrHighlightHotTrack: u32,
    pub rcText: RECT,
    pub nStringBkMode: i32,
    pub nHLStringBkMode: i32,
    pub iListGap: i32,
}
pub struct Nmtbdispinfoa<'a> {
    pub hdr: NMHDR,
    pub dwMask: u32,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchText: i32,
}
pub struct Nmtbdispinfow<'a> {
    pub hdr: NMHDR,
    pub dwMask: u32,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchText: i32,
}
pub const TBNF_IMAGE: u32 = 0x1;
pub const TBNF_TEXT: u32 = 0x2;
pub const TBNF_DI_SETITEM: u32 = 0x10000000;
pub struct Nmtbgetinfotipa<'a> {
    pub hdr: NMHDR,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: LPARAM,
}
pub struct Nmtbgetinfotipw<'a> {
    pub hdr: NMHDR,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: LPARAM,
}
pub struct Nmtbhotitem {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
pub const HICF_ACCELERATOR: u32 = 0x4;
pub const HICF_ARROWKEYS: u32 = 0x2;
pub const HICF_DUPACCEL: u32 = 0x8;
pub const HICF_ENTERING: u32 = 0x10;
pub const HICF_LEAVING: u32 = 0x20;
pub const HICF_LMOUSE: u32 = 0x80;
pub const HICF_MOUSE: u32 = 0x1;
pub const HICF_OTHER: u32 = 0x0;
pub const HICF_RESELECT: u32 = 0x40;
pub const HICF_TOGGLEDROPDOWN: u32 = 0x100;
pub struct Nmtbrestore<'a> {
    pub hdr: NMHDR,
    pub pData: &'a mut u32,
    pub pCurrent: &'a mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub cbBytesPerRecord: i32,
    pub tbButton: TBBUTTON,
}
pub struct Nmtbsave<'a> {
    pub hdr: NMHDR,
    pub pData: &'a mut u32,
    pub pCurrent: &'a mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub tbButton: TBBUTTON,
}
pub struct Nmtckeydown {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
pub struct Nmtoolbara<'a> {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: &'a mut Cow<'a, str>,
    pub rcButton: RECT,
}
pub struct Nmtoolbarw<'a> {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub rcButton: RECT,
}
pub struct Nmtooltipscreated {
    pub hdr: NMHDR,
    pub hwndToolTips: HWND,
}
pub struct Nmtrbthumbposchanging {
    pub hdr: NMHDR,
    pub dwPos: u32,
    pub nReason: i32,
}
pub struct Nmtreeviewa<'a> {
    pub hdr: NMHDR,
    pub action: u32,
    pub itemOld: TVITEMA<'a>,
    pub itemNew: TVITEMA<'a>,
    pub ptDrag: POINT,
}
pub struct Nmtreevieww<'a> {
    pub hdr: NMHDR,
    pub action: u32,
    pub itemOld: TVITEMW<'a>,
    pub itemNew: TVITEMW<'a>,
    pub ptDrag: POINT,
}
pub struct Nmttcustomdraw {
    pub nmcd: NMCUSTOMDRAW,
    pub uDrawFlags: u32,
}
pub struct Nmttdispinfoa<'a> {
    pub hdr: NMHDR,
    pub lpszText: &'a mut Cow<'a, str>,
    pub szText: [CHAR; 80],
    pub hinst: HINSTANCE,
    pub uFlags: u32,
    pub lParam: LPARAM,
}
pub struct Nmttdispinfow<'a> {
    pub hdr: NMHDR,
    pub lpszText: &'a mut Cow<'a, OsStr>,
    pub szText: [u8; 80],
    pub hinst: HINSTANCE,
    pub uFlags: u32,
    pub lParam: LPARAM,
}
pub struct Nmtvasyncdraw<'a> {
    pub hdr: NMHDR,
    pub pimldp: &'a mut IMAGELISTDRAWPARAMS,
    pub hr: HRESULT,
    pub hItem: HTREEITEM,
    pub lParam: LPARAM,
    pub dwRetFlags: u32,
    pub iRetImageIndex: i32,
}
pub struct Nmtvcustomdraw {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: u32,
    pub clrTextBk: u32,
    pub iLevel: i32,
}
pub struct Nmtvdispinfoa<'a> {
    pub hdr: NMHDR,
    pub item: TVITEMA<'a>,
}
pub struct Nmtvdispinfoexa<'a> {
    pub hdr: NMHDR,
    pub item: TVITEMEXA<'a>,
}
pub struct Nmtvdispinfoexw<'a> {
    pub hdr: NMHDR,
    pub item: TVITEMEXW<'a>,
}
pub struct Nmtvdispinfow<'a> {
    pub hdr: NMHDR,
    pub item: TVITEMW<'a>,
}
pub struct Nmtvgetinfotipa<'a> {
    pub hdr: NMHDR,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: LPARAM,
}
pub struct Nmtvgetinfotipw<'a> {
    pub hdr: NMHDR,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: LPARAM,
}
pub struct Nmtvitemchange {
    pub hdr: NMHDR,
    pub uChanged: u32,
    pub hItem: HTREEITEM,
    pub uStateNew: u32,
    pub uStateOld: u32,
    pub lParam: LPARAM,
}
pub struct Nmtvkeydown {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
pub struct Nmtvstateimagechanging {
    pub hdr: NMHDR,
    pub hti: HTREEITEM,
    pub iOldStateImageIndex: i32,
    pub iNewStateImageIndex: i32,
}
pub struct Nmupdown {
    pub hdr: NMHDR,
    pub iPos: i32,
    pub iDelta: i32,
}
pub struct Nmviewchange {
    pub nmhdr: NMHDR,
    pub dwOldView: u32,
    pub dwNewView: u32,
}
pub struct Nonclientmetricsa {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: LOGFONTA,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: LOGFONTA,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: LOGFONTA,
    pub lfStatusFont: LOGFONTA,
    pub lfMessageFont: LOGFONTA,
    pub iPaddedBorderWidth: i32,
}
pub struct Nonclientmetricsw {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: LOGFONTW,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: LOGFONTW,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: LOGFONTW,
    pub lfStatusFont: LOGFONTW,
    pub lfMessageFont: LOGFONTW,
    pub iPaddedBorderWidth: i32,
}
pub struct Notifyicondataa {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [CHAR; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [CHAR; 256],
    pub Anonymous: NOTIFYICONDATAA_45,
    pub szInfoTitle: [CHAR; 64],
    pub dwInfoFlags: u32,
    pub guidItem: GUID,
    pub hBalloonIcon: HICON,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Notifyicondataa44 {
    pub field0: u32,
    pub field1: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Notifyicondataa45 {
    pub field0: u32,
    pub field1: u32,
}
pub struct Notifyicondataw {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u8; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u8; 256],
    pub Anonymous: NOTIFYICONDATAW_47,
    pub szInfoTitle: [u8; 64],
    pub dwInfoFlags: u32,
    pub guidItem: GUID,
    pub hBalloonIcon: HICON,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Notifyicondataw46 {
    pub field0: u32,
    pub field1: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Notifyicondataw47 {
    pub field0: u32,
    pub field1: u32,
}
pub struct Notifyiconidentifier {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub guidItem: GUID,
}
pub const NIF_MESSAGE: u32 = 0x1;
pub const NIF_ICON: u32 = 0x2;
pub const NIF_TIP: u32 = 0x4;
pub const NIF_STATE: u32 = 0x8;
pub const NIF_INFO: u32 = 0x10;
pub const NIF_GUID: u32 = 0x20;
pub const NIF_REALTIME: u32 = 0x40;
pub const NIF_SHOWTIP: u32 = 0x80;
pub const NIM_ADD: u32 = 0x0;
pub const NIM_MODIFY: u32 = 0x1;
pub const NIM_DELETE: u32 = 0x2;
pub const NIM_SETFOCUS: u32 = 0x3;
pub const NIM_SETVERSION: u32 = 0x4;
pub struct NpCredentialProvider {
}
pub const NSTCFC_NONE: i32 = 0x0;
pub const NSTCFC_PINNEDITEMFILTERING: i32 = 0x1;
pub const NSTCFC_DELAY_REGISTER_NOTIFY: i32 = 0x2;
pub const NSTCGNI_NEXT: i32 = 0x0;
pub const NSTCGNI_NEXTVISIBLE: i32 = 0x1;
pub const NSTCGNI_PREV: i32 = 0x2;
pub const NSTCGNI_PREVVISIBLE: i32 = 0x3;
pub const NSTCGNI_PARENT: i32 = 0x4;
pub const NSTCGNI_CHILD: i32 = 0x5;
pub const NSTCGNI_FIRSTVISIBLE: i32 = 0x6;
pub const NSTCGNI_LASTVISIBLE: i32 = 0x7;
pub const NSTCS2_DEFAULT: i32 = 0x0;
pub const NSTCS2_INTERRUPTNOTIFICATIONS: i32 = 0x1;
pub const NSTCS2_SHOWNULLSPACEMENU: i32 = 0x2;
pub const NSTCS2_DISPLAYPADDING: i32 = 0x4;
pub const NSTCS2_DISPLAYPINNEDONLY: i32 = 0x8;
pub const NTSCS2_NOSINGLETONAUTOEXPAND: i32 = 0x10;
pub const NTSCS2_NEVERINSERTNONENUMERATED: i32 = 0x20;
pub const FACILITY_DEBUGGER: u32 = 0x1;
pub const FACILITY_RPC_RUNTIME: u32 = 0x2;
pub const FACILITY_RPC_STUBS: u32 = 0x3;
pub const FACILITY_IO_ERROR_CODE: u32 = 0x4;
pub const FACILITY_CODCLASS_ERROR_CODE: u32 = 0x6;
pub const FACILITY_NTWIN32: u32 = 0x7;
pub const FACILITY_NTCERT: u32 = 0x8;
pub const FACILITY_NTSSPI: u32 = 0x9;
pub const FACILITY_TERMINAL_SERVER: u32 = 0xA;
pub const FACILITY_USB_ERROR_CODE: u32 = 0x10;
pub const FACILITY_HID_ERROR_CODE: u32 = 0x11;
pub const FACILITY_FIREWIRE_ERROR_CODE: u32 = 0x12;
pub const FACILITY_CLUSTER_ERROR_CODE: u32 = 0x13;
pub const FACILITY_ACPI_ERROR_CODE: u32 = 0x14;
pub const FACILITY_SXS_ERROR_CODE: u32 = 0x15;
pub const FACILITY_TRANSACTION: u32 = 0x19;
pub const FACILITY_COMMONLOG: u32 = 0x1A;
pub const FACILITY_VIDEO: u32 = 0x1B;
pub const FACILITY_FILTER_MANAGER: u32 = 0x1C;
pub const FACILITY_MONITOR: u32 = 0x1D;
pub const FACILITY_GRAPHICS_KERNEL: u32 = 0x1E;
pub const FACILITY_DRIVER_FRAMEWORK: u32 = 0x20;
pub const FACILITY_FVE_ERROR_CODE: u32 = 0x21;
pub const FACILITY_FWP_ERROR_CODE: u32 = 0x22;
pub const FACILITY_NDIS_ERROR_CODE: u32 = 0x23;
pub const FACILITY_QUIC_ERROR_CODE: u32 = 0x24;
pub const FACILITY_TPM: u32 = 0x29;
pub const FACILITY_RTPM: u32 = 0x2A;
pub const FACILITY_HYPERVISOR: u32 = 0x35;
pub const FACILITY_IPSEC: u32 = 0x36;
pub const FACILITY_VIRTUALIZATION: u32 = 0x37;
pub const FACILITY_VOLMGR: u32 = 0x38;
pub const FACILITY_BCD_ERROR_CODE: u32 = 0x39;
pub const FACILITY_WIN32K_NTUSER: u32 = 0x3E;
pub const FACILITY_WIN32K_NTGDI: u32 = 0x3F;
pub const FACILITY_RESUME_KEY_FILTER: u32 = 0x40;
pub const FACILITY_RDBSS: u32 = 0x41;
pub const FACILITY_BTH_ATT: u32 = 0x42;
pub const FACILITY_SECUREBOOT: u32 = 0x43;
pub const FACILITY_AUDIO_KERNEL: u32 = 0x44;
pub const FACILITY_VSM: u32 = 0x45;
pub const FACILITY_NT_IORING: u32 = 0x46;
pub const FACILITY_VOLSNAP: u32 = 0x50;
pub const FACILITY_SDBUS: u32 = 0x51;
pub const FACILITY_SHARED_VHDX: u32 = 0x5C;
pub const FACILITY_SMB: u32 = 0x5D;
pub const FACILITY_XVS: u32 = 0x5E;
pub const FACILITY_INTERIX: u32 = 0x99;
pub const FACILITY_SPACES: u32 = 0xE7;
pub const FACILITY_SECURITY_CORE: u32 = 0xE8;
pub const FACILITY_SYSTEM_INTEGRITY: u32 = 0xE9;
pub const FACILITY_LICENSING: u32 = 0xEA;
pub const FACILITY_PLATFORM_MANIFEST: u32 = 0xEB;
pub const FACILITY_APP_EXEC: u32 = 0xEC;
pub const FACILITY_MAXIMUM_VALUE: u32 = 0xED;
pub struct NtFeConsoleProps {
    pub dbh: DATABLOCK_HEADER,
    pub uCodePage: u32,
}
pub const NWMF_UNLOADING: i32 = 0x1;
pub const NWMF_USERINITED: i32 = 0x2;
pub const NWMF_FIRST: i32 = 0x4;
pub const NWMF_OVERRIDEKEY: i32 = 0x8;
pub const NWMF_SHOWHELP: i32 = 0x10;
pub const NWMF_HTMLDIALOG: i32 = 0x20;
pub const NWMF_FROMDIALOGCHILD: i32 = 0x40;
pub const NWMF_USERREQUESTED: i32 = 0x80;
pub const NWMF_USERALLOWED: i32 = 0x100;
pub const NWMF_FORCEWINDOW: i32 = 0x10000;
pub const NWMF_FORCETAB: i32 = 0x20000;
pub const NWMF_SUGGESTWINDOW: i32 = 0x40000;
pub const NWMF_SUGGESTTAB: i32 = 0x80000;
pub const NWMF_INACTIVETAB: i32 = 0x100000;
pub struct NamespaceTreeControl {
}
pub struct NamespaceWalker {
}
pub struct NetworkConnections {
}
pub struct NetworkExplorerFolder {
}
pub struct NetworkPlaces {
}
pub const PROTECTED_MODE_REDIRECT: i32 = 0x1;
pub const OBJID_WINDOW: i32 = 0x0;
pub const OBJID_SYSMENU: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const OBJID_TITLEBAR: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE;
pub const OBJID_MENU: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD;
pub const OBJID_CLIENT: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC;
pub const OBJID_VSCROLL: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFB;
pub const OBJID_HSCROLL: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFA;
pub const OBJID_SIZEGRIP: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF9;
pub const OBJID_CARET: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8;
pub const OBJID_CURSOR: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7;
pub const OBJID_ALERT: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF6;
pub const OBJID_SOUND: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5;
pub const OBJID_QUERYCLASSNAMEIDX: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4;
pub const OBJID_NATIVEOM: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0;
pub const OBJ_PEN: i32 = 0x1;
pub const OBJ_BRUSH: i32 = 0x2;
pub const OBJ_DC: i32 = 0x3;
pub const OBJ_METADC: i32 = 0x4;
pub const OBJ_PAL: i32 = 0x5;
pub const OBJ_FONT: i32 = 0x6;
pub const OBJ_BITMAP: i32 = 0x7;
pub const OBJ_REGION: i32 = 0x8;
pub const OBJ_METAFILE: i32 = 0x9;
pub const OBJ_MEMDC: i32 = 0xA;
pub const OBJ_EXTPEN: i32 = 0xB;
pub const OBJ_ENHMETADC: i32 = 0xC;
pub const OBJ_ENHMETAFILE: i32 = 0xD;
pub const OBJ_COLORSPACE: i32 = 0xE;
pub const OT_TOPLEFT: i32 = 0x0;
pub const OT_TOPRIGHT: i32 = 0x1;
pub const OT_TOPMIDDLE: i32 = 0x2;
pub const OT_BOTTOMLEFT: i32 = 0x3;
pub const OT_BOTTOMRIGHT: i32 = 0x4;
pub const OT_BOTTOMMIDDLE: i32 = 0x5;
pub const OT_MIDDLELEFT: i32 = 0x6;
pub const OT_MIDDLERIGHT: i32 = 0x7;
pub const OT_LEFTOFCAPTION: i32 = 0x8;
pub const OT_RIGHTOFCAPTION: i32 = 0x9;
pub const OT_LEFTOFLASTBUTTON: i32 = 0xA;
pub const OT_RIGHTOFLASTBUTTON: i32 = 0xB;
pub const OT_ABOVELASTBUTTON: i32 = 0xC;
pub const OT_BELOWLASTBUTTON: i32 = 0xD;
pub struct Openasinfo<'a> {
    pub pcszFile: &'a Cow<'a, OsStr>,
    pub pcszClass: &'a Cow<'a, OsStr>,
    pub oaifInFlags: u32,
}
pub const SPOB_NORMAL: i32 = 0x1;
pub const SPOB_HOT: i32 = 0x2;
pub const SPOB_SELECTED: i32 = 0x3;
pub const SPOB_DISABLED: i32 = 0x4;
pub const SPOB_FOCUSED: i32 = 0x5;
pub const OAIF_ALLOW_REGISTRATION: u32 = 0x1;
pub const OAIF_REGISTER_EXT: u32 = 0x2;
pub const OAIF_EXEC: u32 = 0x4;
pub const OAIF_FORCE_REGISTRATION: u32 = 0x8;
pub const OAIF_HIDE_REGISTRATION: u32 = 0x20;
pub const OAIF_URL_PROTOCOL: u32 = 0x40;
pub const OAIF_FILE_IS_URI: u32 = 0x80;
pub struct OpenPrinterPropsInfoa<'a> {
    pub dwSize: u32,
    pub pszSheetName: &'a mut Cow<'a, str>,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: BOOL,
}
pub struct OpenPrinterPropsInfow<'a> {
    pub dwSize: u32,
    pub pszSheetName: &'a mut Cow<'a, OsStr>,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: BOOL,
}
pub const OTD_FORCE_RECT_SIZING: u32 = 0x1;
pub const OTD_NONCLIENT: u32 = 0x2;
pub const OS_WINDOWS: u32 = 0x0;
pub const OS_NT: u32 = 0x1;
pub const OS_WIN95ORGREATER: u32 = 0x2;
pub const OS_NT4ORGREATER: u32 = 0x3;
pub const OS_WIN98ORGREATER: u32 = 0x5;
pub const OS_WIN98_GOLD: u32 = 0x6;
pub const OS_WIN2000ORGREATER: u32 = 0x7;
pub const OS_WIN2000PRO: u32 = 0x8;
pub const OS_WIN2000SERVER: u32 = 0x9;
pub const OS_WIN2000ADVSERVER: u32 = 0xA;
pub const OS_WIN2000DATACENTER: u32 = 0xB;
pub const OS_WIN2000TERMINAL: u32 = 0xC;
pub const OS_EMBEDDED: u32 = 0xD;
pub const OS_TERMINALCLIENT: u32 = 0xE;
pub const OS_TERMINALREMOTEADMIN: u32 = 0xF;
pub const OS_WIN95_GOLD: u32 = 0x10;
pub const OS_MEORGREATER: u32 = 0x11;
pub const OS_XPORGREATER: u32 = 0x12;
pub const OS_HOME: u32 = 0x13;
pub const OS_PROFESSIONAL: u32 = 0x14;
pub const OS_DATACENTER: u32 = 0x15;
pub const OS_ADVSERVER: u32 = 0x16;
pub const OS_SERVER: u32 = 0x17;
pub const OS_TERMINALSERVER: u32 = 0x18;
pub const OS_PERSONALTERMINALSERVER: u32 = 0x19;
pub const OS_FASTUSERSWITCHING: u32 = 0x1A;
pub const OS_WELCOMELOGONUI: u32 = 0x1B;
pub const OS_DOMAINMEMBER: u32 = 0x1C;
pub const OS_ANYSERVER: u32 = 0x1D;
pub const OS_WOW6432: u32 = 0x1E;
pub const OS_WEBSERVER: u32 = 0x1F;
pub const OS_SMALLBUSINESSSERVER: u32 = 0x20;
pub const OS_TABLETPC: u32 = 0x21;
pub const OS_SERVERADMINUI: u32 = 0x22;
pub const OS_MEDIACENTER: u32 = 0x23;
pub const OS_APPLIANCE: u32 = 0x24;
pub struct Outlinetextmetrica<'a> {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICA,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: POINT,
    pub otmptSubscriptOffset: POINT,
    pub otmptSuperscriptSize: POINT,
    pub otmptSuperscriptOffset: POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: &'a mut Cow<'a, str>,
    pub otmpFaceName: &'a mut Cow<'a, str>,
    pub otmpStyleName: &'a mut Cow<'a, str>,
    pub otmpFullName: &'a mut Cow<'a, str>,
}
pub struct Outlinetextmetricw<'a> {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICW,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: POINT,
    pub otmptSubscriptOffset: POINT,
    pub otmptSuperscriptSize: POINT,
    pub otmptSuperscriptOffset: POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: &'a mut Cow<'a, str>,
    pub otmpFaceName: &'a mut Cow<'a, str>,
    pub otmpStyleName: &'a mut Cow<'a, str>,
    pub otmpFullName: &'a mut Cow<'a, str>,
}
pub const OFS_INACTIVE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const OFS_ONLINE: i32 = 0x0;
pub const OFS_OFFLINE: i32 = 0x1;
pub const OFS_SERVERBACK: i32 = 0x2;
pub const OFS_DIRTYCACHE: i32 = 0x3;
pub struct OnexCredentialProvider {
}
pub struct OnexPlapSmartcardCredentialProvider {
}
pub struct OpenControlPanel {
}
pub const PES_UNKNOWN: i32 = 0x0;
pub const PES_RUNNING: i32 = 0x1;
pub const PES_SUSPENDING: i32 = 0x2;
pub const PES_SUSPENDED: i32 = 0x3;
pub const PES_TERMINATED: i32 = 0x4;
pub const PGRP_UP: i32 = 0x1;
pub const PGRP_DOWN: i32 = 0x2;
pub const PGRP_UPHORZ: i32 = 0x3;
pub const PGRP_DOWNHORZ: i32 = 0x4;
pub struct Paintstruct {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [u8; 32],
}
pub struct Paletteentry {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
pub struct Panose {
    pub bFamilyType: u8,
    pub bSerifStyle: u8,
    pub bWeight: u8,
    pub bProportion: u8,
    pub bContrast: u8,
    pub bStrokeVariation: u8,
    pub bArmStyle: u8,
    pub bLetterform: u8,
    pub bMidline: u8,
    pub bXHeight: u8,
}
pub struct Parsedurla<'a> {
    pub cbSize: u32,
    pub pszProtocol: &'a Cow<'a, str>,
    pub cchProtocol: u32,
    pub pszSuffix: &'a Cow<'a, str>,
    pub cchSuffix: u32,
    pub nScheme: u32,
}
pub struct Parsedurlw<'a> {
    pub cbSize: u32,
    pub pszProtocol: &'a Cow<'a, OsStr>,
    pub cchProtocol: u32,
    pub pszSuffix: &'a Cow<'a, OsStr>,
    pub cchSuffix: u32,
    pub nScheme: u32,
}
pub const PATHCCH_NONE: i32 = 0x0;
pub const PATHCCH_ALLOW_LONG_PATHS: i32 = 0x1;
pub const PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS: i32 = 0x2;
pub const PATHCCH_FORCE_DISABLE_LONG_NAME_PROCESS: i32 = 0x4;
pub const PATHCCH_DO_NOT_NORMALIZE_SEGMENTS: i32 = 0x8;
pub const PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH: i32 = 0x10;
pub const PATHCCH_ENSURE_TRAILING_SLASH: i32 = 0x20;
pub const PATHCCH_CANONICALIZE_SLASHES: i32 = 0x40;
pub struct Pbrange {
    pub iLow: i32,
    pub iHigh: i32,
}
pub const PCS_FATAL: u32 = 0x80000000;
pub const PCS_REPLACEDCHAR: u32 = 0x1;
pub const PCS_REMOVEDCHAR: u32 = 0x2;
pub const PCS_TRUNCATED: u32 = 0x4;
pub const PCS_PATHTOOLONG: u32 = 0x8;
pub const PM_NOREMOVE: u32 = 0x0;
pub const PM_REMOVE: u32 = 0x1;
pub const PM_NOYIELD: u32 = 0x2;
pub const PM_QS_INPUT: u32 = 0x4070000;
pub const PM_QS_POSTMESSAGE: u32 = 0x980000;
pub const PM_QS_PAINT: u32 = 0x200000;
pub const PM_QS_SENDMESSAGE: u32 = 0x400000;
pub struct Pelarray {
    pub paXCount: i32,
    pub paYCount: i32,
    pub paXExt: i32,
    pub paYExt: i32,
    pub paRGBs: u8,
}
pub const PS_GEOMETRIC: u32 = 0x10000;
pub const PS_COSMETIC: u32 = 0x0;
pub const PS_SOLID: u32 = 0x0;
pub const PS_DASH: u32 = 0x1;
pub const PS_DOT: u32 = 0x2;
pub const PS_DASHDOT: u32 = 0x3;
pub const PS_DASHDOTDOT: u32 = 0x4;
pub const PS_NULL: u32 = 0x5;
pub const PS_INSIDEFRAME: u32 = 0x6;
pub const PS_USERSTYLE: u32 = 0x7;
pub const PS_ALTERNATE: u32 = 0x8;
pub const PS_STYLE_MASK: u32 = 0xF;
pub const PS_ENDCAP_ROUND: u32 = 0x0;
pub const PS_ENDCAP_SQUARE: u32 = 0x100;
pub const PS_ENDCAP_FLAT: u32 = 0x200;
pub const PS_ENDCAP_MASK: u32 = 0xF00;
pub const PS_JOIN_ROUND: u32 = 0x0;
pub const PS_JOIN_BEVEL: u32 = 0x1000;
pub const PS_JOIN_MITER: u32 = 0x2000;
pub const PS_JOIN_MASK: u32 = 0xF000;
pub const PS_TYPE_MASK: u32 = 0xF0000;
pub const PIDISF_RECENTLYCHANGED: i32 = 0x1;
pub const PIDISF_CACHEDSTICKY: i32 = 0x2;
pub const PIDISF_CACHEIMAGES: i32 = 0x10;
pub const PIDISF_FOLLOWALLLINKS: i32 = 0x20;
pub const PIDISM_GLOBAL: i32 = 0x0;
pub const PIDISM_WATCH: i32 = 0x1;
pub const PIDISM_DONTWATCH: i32 = 0x2;
pub const PIDISR_UP_TO_DATE: i32 = 0x0;
pub const PIDISR_NEEDS_ADD: i32 = 0x1;
pub const PIDISR_NEEDS_UPDATE: i32 = 0x2;
pub const PIDISR_NEEDS_DELETE: i32 = 0x3;
pub const PID_INTSITE_WHATSNEW: i32 = 0x2;
pub const PID_INTSITE_AUTHOR: i32 = 0x3;
pub const PID_INTSITE_LASTVISIT: i32 = 0x4;
pub const PID_INTSITE_LASTMOD: i32 = 0x5;
pub const PID_INTSITE_VISITCOUNT: i32 = 0x6;
pub const PID_INTSITE_DESCRIPTION: i32 = 0x7;
pub const PID_INTSITE_COMMENT: i32 = 0x8;
pub const PID_INTSITE_FLAGS: i32 = 0x9;
pub const PID_INTSITE_CONTENTLEN: i32 = 0xA;
pub const PID_INTSITE_CONTENTCODE: i32 = 0xB;
pub const PID_INTSITE_RECURSE: i32 = 0xC;
pub const PID_INTSITE_WATCH: i32 = 0xD;
pub const PID_INTSITE_SUBSCRIPTION: i32 = 0xE;
pub const PID_INTSITE_URL: i32 = 0xF;
pub const PID_INTSITE_TITLE: i32 = 0x10;
pub const PID_INTSITE_CODEPAGE: i32 = 0x12;
pub const PID_INTSITE_TRACKING: i32 = 0x13;
pub const PID_INTSITE_ICONINDEX: i32 = 0x14;
pub const PID_INTSITE_ICONFILE: i32 = 0x15;
pub const PID_INTSITE_ROAMED: i32 = 0x22;
pub const PID_IS_URL: i32 = 0x2;
pub const PID_IS_NAME: i32 = 0x4;
pub const PID_IS_WORKINGDIR: i32 = 0x5;
pub const PID_IS_HOTKEY: i32 = 0x6;
pub const PID_IS_SHOWCMD: i32 = 0x7;
pub const PID_IS_ICONINDEX: i32 = 0x8;
pub const PID_IS_ICONFILE: i32 = 0x9;
pub const PID_IS_WHATSNEW: i32 = 0xA;
pub const PID_IS_AUTHOR: i32 = 0xB;
pub const PID_IS_DESCRIPTION: i32 = 0xC;
pub const PID_IS_COMMENT: i32 = 0xD;
pub const PID_IS_ROAMED: i32 = 0xF;
pub struct PinLogonCredentialProvider {
}
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub const POINTER_CHANGE_NONE: i32 = 0x0;
pub const POINTER_CHANGE_FIRSTBUTTON_DOWN: i32 = 0x1;
pub const POINTER_CHANGE_FIRSTBUTTON_UP: i32 = 0x2;
pub const POINTER_CHANGE_SECONDBUTTON_DOWN: i32 = 0x3;
pub const POINTER_CHANGE_SECONDBUTTON_UP: i32 = 0x4;
pub const POINTER_CHANGE_THIRDBUTTON_DOWN: i32 = 0x5;
pub const POINTER_CHANGE_THIRDBUTTON_UP: i32 = 0x6;
pub const POINTER_CHANGE_FOURTHBUTTON_DOWN: i32 = 0x7;
pub const POINTER_CHANGE_FOURTHBUTTON_UP: i32 = 0x8;
pub const POINTER_CHANGE_FIFTHBUTTON_DOWN: i32 = 0x9;
pub const POINTER_CHANGE_FIFTHBUTTON_UP: i32 = 0xA;
pub struct PointerDeviceCursorInfo {
    pub cursorId: u32,
    pub cursor: i32,
}
pub const POINTER_DEVICE_CURSOR_TYPE_UNKNOWN: i32 = 0x0;
pub const POINTER_DEVICE_CURSOR_TYPE_TIP: i32 = 0x1;
pub const POINTER_DEVICE_CURSOR_TYPE_ERASER: i32 = 0x2;
pub const POINTER_DEVICE_CURSOR_TYPE_MAX: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub struct PointerDeviceInfo {
    pub displayOrientation: u32,
    pub device: HANDLE,
    pub pointerDeviceType: i32,
    pub monitor: HMONITOR,
    pub startingCursorId: u32,
    pub maxActiveContacts: u16,
    pub productString: [u8; 520],
}
pub struct PointerDeviceProperty {
    pub logicalMin: i32,
    pub logicalMax: i32,
    pub physicalMin: i32,
    pub physicalMax: i32,
    pub unit: u32,
    pub unitExponent: u32,
    pub usagePageId: u16,
    pub usageId: u16,
}
pub const POINTER_DEVICE_TYPE_INTEGRATED_PEN: i32 = 0x1;
pub const POINTER_DEVICE_TYPE_EXTERNAL_PEN: i32 = 0x2;
pub const POINTER_DEVICE_TYPE_TOUCH: i32 = 0x3;
pub const POINTER_DEVICE_TYPE_TOUCH_PAD: i32 = 0x4;
pub const POINTER_DEVICE_TYPE_MAX: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const POINTER_FEEDBACK_DEFAULT: i32 = 0x1;
pub const POINTER_FEEDBACK_INDIRECT: i32 = 0x2;
pub const POINTER_FEEDBACK_NONE: i32 = 0x3;
pub const POINTER_FLAG_NONE: u32 = 0x0;
pub const POINTER_FLAG_NEW: u32 = 0x1;
pub const POINTER_FLAG_INRANGE: u32 = 0x2;
pub const POINTER_FLAG_INCONTACT: u32 = 0x4;
pub const POINTER_FLAG_FIRSTBUTTON: u32 = 0x10;
pub const POINTER_FLAG_SECONDBUTTON: u32 = 0x20;
pub const POINTER_FLAG_THIRDBUTTON: u32 = 0x40;
pub const POINTER_FLAG_FOURTHBUTTON: u32 = 0x80;
pub const POINTER_FLAG_FIFTHBUTTON: u32 = 0x100;
pub const POINTER_FLAG_PRIMARY: u32 = 0x2000;
pub const POINTER_FLAG_CONFIDENCE: u32 = 0x4000;
pub const POINTER_FLAG_CANCELED: u32 = 0x8000;
pub const POINTER_FLAG_DOWN: u32 = 0x10000;
pub const POINTER_FLAG_UPDATE: u32 = 0x20000;
pub const POINTER_FLAG_UP: u32 = 0x40000;
pub const POINTER_FLAG_WHEEL: u32 = 0x80000;
pub const POINTER_FLAG_HWHEEL: u32 = 0x100000;
pub const POINTER_FLAG_CAPTURECHANGED: u32 = 0x200000;
pub const POINTER_FLAG_HASTRANSFORM: u32 = 0x400000;
pub struct PointerInfo {
    pub pointerType: i32,
    pub pointerId: u32,
    pub frameId: u32,
    pub pointerFlags: u32,
    pub sourceDevice: HANDLE,
    pub hwndTarget: HWND,
    pub ptPixelLocation: POINT,
    pub ptHimetricLocation: POINT,
    pub ptPixelLocationRaw: POINT,
    pub ptHimetricLocationRaw: POINT,
    pub dwTime: u32,
    pub historyCount: u32,
    pub InputData: i32,
    pub dwKeyStates: u32,
    pub PerformanceCount: u64,
    pub ButtonChangeType: i32,
}
pub const PT_POINTER: i32 = 0x1;
pub const PT_TOUCH: i32 = 0x2;
pub const PT_PEN: i32 = 0x3;
pub const PT_MOUSE: i32 = 0x4;
pub const PT_TOUCHPAD: i32 = 0x5;
pub struct PointerPenInfo {
    pub pointerInfo: POINTER_INFO,
    pub penFlags: u32,
    pub penMask: u32,
    pub pressure: u32,
    pub rotation: u32,
    pub tiltX: i32,
    pub tiltY: i32,
}
pub struct PointerTouchInfo {
    pub pointerInfo: POINTER_INFO,
    pub touchFlags: u32,
    pub touchMask: u32,
    pub rcContact: RECT,
    pub rcContactRaw: RECT,
    pub orientation: u32,
    pub pressure: u32,
}
pub struct PointerTypeInfo {
    pub type: i32,
    pub Anonymous: POINTER_TYPE_INFO_5,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PointerTypeInfo5 {
    pub field0: POINTER_TOUCH_INFO,
    pub field1: POINTER_PEN_INFO,
}
pub struct Pointfx {
    pub x: FIXED,
    pub y: FIXED,
}
pub struct Pointl {
    pub x: i32,
    pub y: i32,
}
pub struct Points {
    pub x: i16,
    pub y: i16,
}
pub struct Polytexta<'a> {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: &'a Cow<'a, str>,
    pub uiFlags: u32,
    pub rcl: RECT,
    pub pdx: &'a mut i32,
}
pub struct Polytextw<'a> {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: &'a Cow<'a, OsStr>,
    pub uiFlags: u32,
    pub rcl: RECT,
    pub pdx: &'a mut i32,
}
pub struct Previewhandlerframeinfo {
    pub haccel: HACCEL,
    pub cAccelEntries: u32,
}
pub const PRF_VERIFYEXISTS: i32 = 0x1;
pub const PRF_TRYPROGRAMEXTENSIONS: i32 = 0x3;
pub const PRF_FIRSTDIRDEF: i32 = 0x4;
pub const PRF_DONTFINDLNK: i32 = 0x8;
pub const PRF_REQUIREABSOLUTE: i32 = 0x10;
pub struct Profileinfoa<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lpUserName: &'a mut Cow<'a, str>,
    pub lpProfilePath: &'a mut Cow<'a, str>,
    pub lpDefaultPath: &'a mut Cow<'a, str>,
    pub lpServerName: &'a mut Cow<'a, str>,
    pub lpPolicyPath: &'a mut Cow<'a, str>,
    pub hProfile: HANDLE,
}
pub struct Profileinfow<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lpUserName: &'a mut Cow<'a, OsStr>,
    pub lpProfilePath: &'a mut Cow<'a, OsStr>,
    pub lpDefaultPath: &'a mut Cow<'a, OsStr>,
    pub lpServerName: &'a mut Cow<'a, OsStr>,
    pub lpPolicyPath: &'a mut Cow<'a, OsStr>,
    pub hProfile: HANDLE,
}
pub const PO_STATE: i32 = 0x0;
pub const PO_PART: i32 = 0x1;
pub const PO_CLASS: i32 = 0x2;
pub const PO_GLOBAL: i32 = 0x3;
pub const PO_NOTFOUND: i32 = 0x4;
pub struct PropsheetheaderaV1<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: HWND,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V1_6<'a>,
    pub pszCaption: &'a Cow<'a, str>,
    pub nPages: u32,
    pub Anonymous2: HICON_7<'a>,
    pub Anonymous3: PROPSHEETHEADERA_V1_52<'a>,
    pub pfnCallback: todo_fn,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetheaderaV16<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, str>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct PropsheetheaderaV2<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: HWND,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V2_9<'a>,
    pub pszCaption: &'a Cow<'a, str>,
    pub nPages: u32,
    pub Anonymous2: HICON_10<'a>,
    pub Anonymous3: PROPSHEETHEADERA_V2_53<'a>,
    pub pfnCallback: todo_fn,
    pub Anonymous4: HICON_12<'a>,
    pub hplWatermark: HPALETTE,
    pub Anonymous5: HICON_13<'a>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetheaderaV29<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, str>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct PropsheetheaderwV1<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: HWND,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V1_14<'a>,
    pub pszCaption: &'a Cow<'a, OsStr>,
    pub nPages: u32,
    pub Anonymous2: HICON_15<'a>,
    pub Anonymous3: PROPSHEETHEADERW_V1_54<'a>,
    pub pfnCallback: todo_fn,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetheaderwV114<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, OsStr>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct PropsheetheaderwV2<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: HWND,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V2_17<'a>,
    pub pszCaption: &'a Cow<'a, OsStr>,
    pub nPages: u32,
    pub Anonymous2: HICON_18<'a>,
    pub Anonymous3: PROPSHEETHEADERW_V2_55<'a>,
    pub pfnCallback: todo_fn,
    pub Anonymous4: HICON_20<'a>,
    pub hplWatermark: HPALETTE,
    pub Anonymous5: HICON_21<'a>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetheaderwV217<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, OsStr>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct Propsheetpagea<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_56<'a>,
    pub Anonymous2: PROPSHEETPAGEA_23<'a>,
    pub pszTitle: &'a Cow<'a, str>,
    pub pfnDlgProc: todo_fn,
    pub lParam: LPARAM,
    pub pfnCallback: todo_fn,
    pub pcRefParent: &'a mut u32,
    pub pszHeaderTitle: &'a Cow<'a, str>,
    pub pszHeaderSubTitle: &'a Cow<'a, str>,
    pub hActCtx: HANDLE,
    pub Anonymous3: HICON_24<'a>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Propsheetpagea23<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, str>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct PropsheetpageaV1<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V1_57<'a>,
    pub Anonymous2: PROPSHEETPAGEA_V1_26<'a>,
    pub pszTitle: &'a Cow<'a, str>,
    pub pfnDlgProc: todo_fn,
    pub lParam: LPARAM,
    pub pfnCallback: todo_fn,
    pub pcRefParent: &'a mut u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetpageaV126<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, str>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct PropsheetpageaV2<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V2_58<'a>,
    pub Anonymous2: PROPSHEETPAGEA_V2_28<'a>,
    pub pszTitle: &'a Cow<'a, str>,
    pub pfnDlgProc: todo_fn,
    pub lParam: LPARAM,
    pub pfnCallback: todo_fn,
    pub pcRefParent: &'a mut u32,
    pub pszHeaderTitle: &'a Cow<'a, str>,
    pub pszHeaderSubTitle: &'a Cow<'a, str>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetpageaV228<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, str>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct PropsheetpageaV3<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V3_59<'a>,
    pub Anonymous2: PROPSHEETPAGEA_V3_30<'a>,
    pub pszTitle: &'a Cow<'a, str>,
    pub pfnDlgProc: todo_fn,
    pub lParam: LPARAM,
    pub pfnCallback: todo_fn,
    pub pcRefParent: &'a mut u32,
    pub pszHeaderTitle: &'a Cow<'a, str>,
    pub pszHeaderSubTitle: &'a Cow<'a, str>,
    pub hActCtx: HANDLE,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetpageaV330<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, str>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct Propsheetpagew<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_60<'a>,
    pub Anonymous2: PROPSHEETPAGEW_32<'a>,
    pub pszTitle: &'a Cow<'a, OsStr>,
    pub pfnDlgProc: todo_fn,
    pub lParam: LPARAM,
    pub pfnCallback: todo_fn,
    pub pcRefParent: &'a mut u32,
    pub pszHeaderTitle: &'a Cow<'a, OsStr>,
    pub pszHeaderSubTitle: &'a Cow<'a, OsStr>,
    pub hActCtx: HANDLE,
    pub Anonymous3: HICON_33<'a>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Propsheetpagew32<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, OsStr>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct PropsheetpagewV1<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V1_61<'a>,
    pub Anonymous2: PROPSHEETPAGEW_V1_35<'a>,
    pub pszTitle: &'a Cow<'a, OsStr>,
    pub pfnDlgProc: todo_fn,
    pub lParam: LPARAM,
    pub pfnCallback: todo_fn,
    pub pcRefParent: &'a mut u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetpagewV135<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, OsStr>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct PropsheetpagewV2<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V2_62<'a>,
    pub Anonymous2: PROPSHEETPAGEW_V2_37<'a>,
    pub pszTitle: &'a Cow<'a, OsStr>,
    pub pfnDlgProc: todo_fn,
    pub lParam: LPARAM,
    pub pfnCallback: todo_fn,
    pub pcRefParent: &'a mut u32,
    pub pszHeaderTitle: &'a Cow<'a, OsStr>,
    pub pszHeaderSubTitle: &'a Cow<'a, OsStr>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetpagewV237<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, OsStr>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct PropsheetpagewV3<'a> {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V3_63<'a>,
    pub Anonymous2: PROPSHEETPAGEW_V3_39<'a>,
    pub pszTitle: &'a Cow<'a, OsStr>,
    pub pfnDlgProc: todo_fn,
    pub lParam: LPARAM,
    pub pfnCallback: todo_fn,
    pub pcRefParent: &'a mut u32,
    pub pszHeaderTitle: &'a Cow<'a, OsStr>,
    pub pszHeaderSubTitle: &'a Cow<'a, OsStr>,
    pub hActCtx: HANDLE,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union PropsheetpagewV339<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, OsStr>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct Pshnotify {
    pub hdr: NMHDR,
    pub lParam: LPARAM,
}
pub const PSPCB_ADDREF: u32 = 0x0;
pub const PSPCB_CREATE: u32 = 0x2;
pub const PSPCB_RELEASE: u32 = 0x1;
pub const PSPCB_SI_INITDIALOG: u32 = 0x401;
pub struct Pubappinfo<'a> {
    pub cbSize: u32,
    pub dwMask: u32,
    pub pszSource: &'a mut Cow<'a, OsStr>,
    pub stAssigned: SYSTEMTIME,
    pub stPublished: SYSTEMTIME,
    pub stScheduled: SYSTEMTIME,
    pub stExpire: SYSTEMTIME,
}
pub const PAI_SOURCE: i32 = 0x1;
pub const PAI_ASSIGNEDTIME: i32 = 0x2;
pub const PAI_PUBLISHEDTIME: i32 = 0x4;
pub const PAI_SCHEDULEDTIME: i32 = 0x8;
pub const PAI_EXPIRETIME: i32 = 0x10;
pub struct PackageDebugSettings {
}
pub struct PasswordCredentialProvider {
}
pub struct PreviousVersions {
}
pub struct PropertiesUi {
}
pub struct PublishDropTarget {
}
pub struct PublishingWizard {
}
pub struct Qcminfo<'a> {
    pub hmenu: HMENU,
    pub indexMenu: u32,
    pub idCmdFirst: u32,
    pub idCmdLast: u32,
    pub pIdMap: &'a QCMINFO_IDMAP,
}
pub struct QcminfoIdmap {
    pub nMaxIds: u32,
    pub pIdList: [QCMINFO_IDMAP_PLACEMENT; 1],
}
pub struct QcminfoIdmapPlacement {
    pub id: u32,
    pub fFlags: u32,
}
pub struct Qitab<'a> {
    pub piid: &'a GUID,
    pub dwOffset: u32,
}
pub const QITIPF_DEFAULT: i32 = 0x0;
pub const QITIPF_USENAME: i32 = 0x1;
pub const QITIPF_LINKNOTARGET: i32 = 0x2;
pub const QITIPF_LINKUSETARGET: i32 = 0x4;
pub const QITIPF_USESLOWTIP: i32 = 0x8;
pub const QITIPF_SINGLELINE: i32 = 0x10;
pub const QIF_CACHED: i32 = 0x1;
pub const QIF_DONTEXPANDFOLDER: i32 = 0x2;
pub const QUNS_NOT_PRESENT: i32 = 0x1;
pub const QUNS_BUSY: i32 = 0x2;
pub const QUNS_RUNNING_D3D_FULL_SCREEN: i32 = 0x3;
pub const QUNS_PRESENTATION_MODE: i32 = 0x4;
pub const QUNS_ACCEPTS_NOTIFICATIONS: i32 = 0x5;
pub const QUNS_QUIET_TIME: i32 = 0x6;
pub const QUNS_APP: i32 = 0x7;
pub const QS_ALLEVENTS: u32 = 0x4BF;
pub const QS_ALLINPUT: u32 = 0x4FF;
pub const QS_ALLPOSTMESSAGE: u32 = 0x100;
pub const QS_HOTKEY: u32 = 0x80;
pub const QS_INPUT: u32 = 0x407;
pub const QS_KEY: u32 = 0x1;
pub const QS_MOUSE: u32 = 0x6;
pub const QS_MOUSEBUTTON: u32 = 0x4;
pub const QS_MOUSEMOVE: u32 = 0x2;
pub const QS_PAINT: u32 = 0x20;
pub const QS_POSTMESSAGE: u32 = 0x8;
pub const QS_RAWINPUT: u32 = 0x400;
pub const QS_SENDMESSAGE: u32 = 0x40;
pub const QS_TIMER: u32 = 0x10;
pub struct QueryCancelAutoPlay {
}
pub const R2_BLACK: i32 = 0x1;
pub const R2_NOTMERGEPEN: i32 = 0x2;
pub const R2_MASKNOTPEN: i32 = 0x3;
pub const R2_NOTCOPYPEN: i32 = 0x4;
pub const R2_MASKPENNOT: i32 = 0x5;
pub const R2_NOT: i32 = 0x6;
pub const R2_XORPEN: i32 = 0x7;
pub const R2_NOTMASKPEN: i32 = 0x8;
pub const R2_MASKPEN: i32 = 0x9;
pub const R2_NOTXORPEN: i32 = 0xA;
pub const R2_NOP: i32 = 0xB;
pub const R2_MERGENOTPEN: i32 = 0xC;
pub const R2_COPYPEN: i32 = 0xD;
pub const R2_MERGEPENNOT: i32 = 0xE;
pub const R2_MERGEPEN: i32 = 0xF;
pub const R2_WHITE: i32 = 0x10;
pub const R2_LAST: i32 = 0x10;
pub struct RasProvider {
}
pub struct RasterizerStatus {
    pub nSize: i16,
    pub wFlags: i16,
    pub nLanguageID: i16,
}
pub struct Rbhittestinfo {
    pub pt: POINT,
    pub flags: u32,
    pub iBand: i32,
}
pub struct Rebarbandinfoa<'a> {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: u32,
    pub clrBack: u32,
    pub lpText: &'a mut Cow<'a, str>,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: RECT,
    pub uChevronState: u32,
}
pub struct Rebarbandinfow<'a> {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: u32,
    pub clrBack: u32,
    pub lpText: &'a mut Cow<'a, OsStr>,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: RECT,
    pub uChevronState: u32,
}
pub struct Rebarinfo {
    pub cbSize: u32,
    pub fMask: u32,
    pub himl: HIMAGELIST,
}
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub struct Rectl {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub struct RedirectionDescriptor<'a> {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: &'a mut REDIRECTION_FUNCTION_DESCRIPTOR<'a>,
}
pub struct RedirectionFunctionDescriptor<'a> {
    pub DllName: &'a Cow<'a, str>,
    pub FunctionName: &'a Cow<'a, str>,
    pub RedirectionTarget: &'a mut todo_void,
}
pub const RDW_INVALIDATE: u32 = 0x1;
pub const RDW_INTERNALPAINT: u32 = 0x2;
pub const RDW_ERASE: u32 = 0x4;
pub const RDW_VALIDATE: u32 = 0x8;
pub const RDW_NOINTERNALPAINT: u32 = 0x10;
pub const RDW_NOERASE: u32 = 0x20;
pub const RDW_NOCHILDREN: u32 = 0x40;
pub const RDW_ALLCHILDREN: u32 = 0x80;
pub const RDW_UPDATENOW: u32 = 0x100;
pub const RDW_ERASENOW: u32 = 0x200;
pub const RDW_FRAME: u32 = 0x400;
pub const RDW_NOFRAME: u32 = 0x800;
pub const REG_CREATED_NEW_KEY: u32 = 0x1;
pub const REG_OPENED_EXISTING_KEY: u32 = 0x2;
pub const REG_NOTIFY_CHANGE_NAME: u32 = 0x1;
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: u32 = 0x2;
pub const REG_NOTIFY_CHANGE_LAST_SET: u32 = 0x4;
pub const REG_NOTIFY_CHANGE_SECURITY: u32 = 0x8;
pub const REG_NOTIFY_THREAD_AGNOSTIC: u32 = 0x10000000;
pub const REG_OPTION_RESERVED: u32 = 0x0;
pub const REG_OPTION_NON_VOLATILE: u32 = 0x0;
pub const REG_OPTION_VOLATILE: u32 = 0x1;
pub const REG_OPTION_CREATE_LINK: u32 = 0x2;
pub const REG_OPTION_BACKUP_RESTORE: u32 = 0x4;
pub const REG_OPTION_OPEN_LINK: u32 = 0x8;
pub const REG_OPTION_DONT_VIRTUALIZE: u32 = 0x10;
pub const REG_FORCE_RESTORE: i32 = 0x8;
pub const REG_WHOLE_HIVE_VOLATILE: i32 = 0x1;
pub const KEY_QUERY_VALUE: u32 = 0x1;
pub const KEY_SET_VALUE: u32 = 0x2;
pub const KEY_CREATE_SUB_KEY: u32 = 0x4;
pub const KEY_ENUMERATE_SUB_KEYS: u32 = 0x8;
pub const KEY_NOTIFY: u32 = 0x10;
pub const KEY_CREATE_LINK: u32 = 0x20;
pub const KEY_WOW64_32KEY: u32 = 0x200;
pub const KEY_WOW64_64KEY: u32 = 0x100;
pub const KEY_WOW64_RES: u32 = 0x300;
pub const KEY_READ: u32 = 0x20019;
pub const KEY_WRITE: u32 = 0x20006;
pub const KEY_EXECUTE: u32 = 0x20019;
pub const KEY_ALL_ACCESS: u32 = 0xF003F;
pub const REG_STANDARD_FORMAT: u32 = 0x1;
pub const REG_LATEST_FORMAT: u32 = 0x2;
pub const REG_NO_COMPRESSION: u32 = 0x4;
pub const REG_NONE: u32 = 0x0;
pub const REG_SZ: u32 = 0x1;
pub const REG_EXPAND_SZ: u32 = 0x2;
pub const REG_BINARY: u32 = 0x3;
pub const REG_DWORD: u32 = 0x4;
pub const REG_DWORD_LITTLE_ENDIAN: u32 = 0x4;
pub const REG_DWORD_BIG_ENDIAN: u32 = 0x5;
pub const REG_LINK: u32 = 0x6;
pub const REG_MULTI_SZ: u32 = 0x7;
pub const REG_RESOURCE_LIST: u32 = 0x8;
pub const REG_FULL_RESOURCE_DESCRIPTOR: u32 = 0x9;
pub const REG_RESOURCE_REQUIREMENTS_LIST: u32 = 0xA;
pub const REG_QWORD: u32 = 0xB;
pub const REG_QWORD_LITTLE_ENDIAN: u32 = 0xB;
pub const REST_NONE: i32 = 0x0;
pub const REST_NORUN: i32 = 0x1;
pub const REST_NOCLOSE: i32 = 0x2;
pub const REST_NOSAVESET: i32 = 0x4;
pub const REST_NOFILEMENU: i32 = 0x8;
pub const REST_NOSETFOLDERS: i32 = 0x10;
pub const REST_NOSETTASKBAR: i32 = 0x20;
pub const REST_NODESKTOP: i32 = 0x40;
pub const REST_NOFIND: i32 = 0x80;
pub const REST_NODRIVES: i32 = 0x100;
pub const REST_NODRIVEAUTORUN: i32 = 0x200;
pub const REST_NODRIVETYPEAUTORUN: i32 = 0x400;
pub const REST_NONETHOOD: i32 = 0x800;
pub const REST_STARTBANNER: i32 = 0x1000;
pub const REST_RESTRICTRUN: i32 = 0x2000;
pub const REST_NOPRINTERTABS: i32 = 0x4000;
pub const REST_NOPRINTERDELETE: i32 = 0x8000;
pub const REST_NOPRINTERADD: i32 = 0x10000;
pub const REST_NOSTARTMENUSUBFOLDERS: i32 = 0x20000;
pub const REST_MYDOCSONNET: i32 = 0x40000;
pub const REST_NOEXITTODOS: i32 = 0x80000;
pub const REST_ENFORCESHELLEXTSECURITY: i32 = 0x100000;
pub const REST_LINKRESOLVEIGNORELINKINFO: i32 = 0x200000;
pub const REST_NOCOMMONGROUPS: i32 = 0x400000;
pub const REST_SEPARATEDESKTOPPROCESS: i32 = 0x800000;
pub const REST_NOWEB: i32 = 0x1000000;
pub const REST_NOTRAYCONTEXTMENU: i32 = 0x2000000;
pub const REST_NOVIEWCONTEXTMENU: i32 = 0x4000000;
pub const REST_NONETCONNECTDISCONNECT: i32 = 0x8000000;
pub const REST_STARTMENULOGOFF: i32 = 0x10000000;
pub const REST_NOSETTINGSASSIST: i32 = 0x20000000;
pub const REST_NOINTERNETICON: i32 = 0x40000001;
pub const REST_NORECENTDOCSHISTORY: i32 = 0x40000002;
pub const REST_NORECENTDOCSMENU: i32 = 0x40000003;
pub const REST_NOACTIVEDESKTOP: i32 = 0x40000004;
pub const REST_NOACTIVEDESKTOPCHANGES: i32 = 0x40000005;
pub const REST_NOFAVORITESMENU: i32 = 0x40000006;
pub const REST_CLEARRECENTDOCSONEXIT: i32 = 0x40000007;
pub const REST_CLASSICSHELL: i32 = 0x40000008;
pub const REST_NOCUSTOMIZEWEBVIEW: i32 = 0x40000009;
pub const REST_NOHTMLWALLPAPER: i32 = 0x40000010;
pub const REST_NOCHANGINGWALLPAPER: i32 = 0x40000011;
pub const REST_NODESKCOMP: i32 = 0x40000012;
pub const REST_NOADDDESKCOMP: i32 = 0x40000013;
pub const REST_NODELDESKCOMP: i32 = 0x40000014;
pub const REST_NOCLOSEDESKCOMP: i32 = 0x40000015;
pub const REST_NOCLOSE_DRAGDROPBAND: i32 = 0x40000016;
pub const REST_NOMOVINGBAND: i32 = 0x40000017;
pub const REST_NOEDITDESKCOMP: i32 = 0x40000018;
pub const REST_NORESOLVESEARCH: i32 = 0x40000019;
pub const REST_NORESOLVETRACK: i32 = 0x4000001A;
pub const REST_FORCECOPYACLWITHFILE: i32 = 0x4000001B;
pub const REST_NOFORGETSOFTWAREUPDATE: i32 = 0x4000001D;
pub const REST_NOSETACTIVEDESKTOP: i32 = 0x4000001E;
pub const REST_NOUPDATEWINDOWS: i32 = 0x4000001F;
pub const REST_NOCHANGESTARMENU: i32 = 0x40000020;
pub const REST_NOFOLDEROPTIONS: i32 = 0x40000021;
pub const REST_HASFINDCOMPUTERS: i32 = 0x40000022;
pub const REST_INTELLIMENUS: i32 = 0x40000023;
pub const REST_RUNDLGMEMCHECKBOX: i32 = 0x40000024;
pub const REST_ARP_SHOW_POST_SETUP: i32 = 0x40000025;
pub const REST_NOCSC: i32 = 0x40000026;
pub const REST_NOCONTROLPANEL: i32 = 0x40000027;
pub const REST_ENUMWORKGROUP: i32 = 0x40000028;
pub const REST_ARP_NOARP: i32 = 0x40000029;
pub const REST_ARP_NOREMOVEPAGE: i32 = 0x4000002A;
pub const REST_ARP_NOADDPAGE: i32 = 0x4000002B;
pub const REST_ARP_NOWINSETUPPAGE: i32 = 0x4000002C;
pub const REST_GREYMSIADS: i32 = 0x4000002D;
pub const REST_NOCHANGEMAPPEDDRIVELABEL: i32 = 0x4000002E;
pub const REST_NOCHANGEMAPPEDDRIVECOMMENT: i32 = 0x4000002F;
pub const REST_MAX_RECENT_DOCS: i32 = 0x40000030;
pub const REST_NONETWORKCONNECTIONS: i32 = 0x40000031;
pub const REST_FORCESTARTMENULOGOFF: i32 = 0x40000032;
pub const REST_NOWEBVIEW: i32 = 0x40000033;
pub const REST_NOCUSTOMIZETHISFOLDER: i32 = 0x40000034;
pub const REST_NOENCRYPTION: i32 = 0x40000035;
pub const REST_DONTSHOWSUPERHIDDEN: i32 = 0x40000037;
pub const REST_NOSHELLSEARCHBUTTON: i32 = 0x40000038;
pub const REST_NOHARDWARETAB: i32 = 0x40000039;
pub const REST_NORUNASINSTALLPROMPT: i32 = 0x4000003A;
pub const REST_PROMPTRUNASINSTALLNETPATH: i32 = 0x4000003B;
pub const REST_NOMANAGEMYCOMPUTERVERB: i32 = 0x4000003C;
pub const REST_DISALLOWRUN: i32 = 0x4000003E;
pub const REST_NOWELCOMESCREEN: i32 = 0x4000003F;
pub const REST_RESTRICTCPL: i32 = 0x40000040;
pub const REST_DISALLOWCPL: i32 = 0x40000041;
pub const REST_NOSMBALLOONTIP: i32 = 0x40000042;
pub const REST_NOSMHELP: i32 = 0x40000043;
pub const REST_NOWINKEYS: i32 = 0x40000044;
pub const REST_NOENCRYPTONMOVE: i32 = 0x40000045;
pub const REST_NOLOCALMACHINERUN: i32 = 0x40000046;
pub const REST_NOCURRENTUSERRUN: i32 = 0x40000047;
pub const REST_NOLOCALMACHINERUNONCE: i32 = 0x40000048;
pub const REST_NOCURRENTUSERRUNONCE: i32 = 0x40000049;
pub const REST_FORCEACTIVEDESKTOPON: i32 = 0x4000004A;
pub const REST_NOVIEWONDRIVE: i32 = 0x4000004C;
pub const REST_NONETCRAWL: i32 = 0x4000004D;
pub const REST_NOSHAREDDOCUMENTS: i32 = 0x4000004E;
pub const REST_NOSMMYDOCS: i32 = 0x4000004F;
pub const REST_NOSMMYPICS: i32 = 0x40000050;
pub const REST_ALLOWBITBUCKDRIVES: i32 = 0x40000051;
pub const REST_NONLEGACYSHELLMODE: i32 = 0x40000052;
pub const REST_NOCONTROLPANELBARRICADE: i32 = 0x40000053;
pub const REST_NOSTARTPAGE: i32 = 0x40000054;
pub const REST_NOAUTOTRAYNOTIFY: i32 = 0x40000055;
pub const REST_NOTASKGROUPING: i32 = 0x40000056;
pub const REST_NOCDBURNING: i32 = 0x40000057;
pub const REST_MYCOMPNOPROP: i32 = 0x40000058;
pub const REST_MYDOCSNOPROP: i32 = 0x40000059;
pub const REST_NOSTARTPANEL: i32 = 0x4000005A;
pub const REST_NODISPLAYAPPEARANCEPAGE: i32 = 0x4000005B;
pub const REST_NOTHEMESTAB: i32 = 0x4000005C;
pub const REST_NOVISUALSTYLECHOICE: i32 = 0x4000005D;
pub const REST_NOSIZECHOICE: i32 = 0x4000005E;
pub const REST_NOCOLORCHOICE: i32 = 0x4000005F;
pub const REST_SETVISUALSTYLE: i32 = 0x40000060;
pub const REST_STARTRUNNOHOMEPATH: i32 = 0x40000061;
pub const REST_NOUSERNAMEINSTARTPANEL: i32 = 0x40000062;
pub const REST_NOMYCOMPUTERICON: i32 = 0x40000063;
pub const REST_NOSMNETWORKPLACES: i32 = 0x40000064;
pub const REST_NOSMPINNEDLIST: i32 = 0x40000065;
pub const REST_NOSMMYMUSIC: i32 = 0x40000066;
pub const REST_NOSMEJECTPC: i32 = 0x40000067;
pub const REST_NOSMMOREPROGRAMS: i32 = 0x40000068;
pub const REST_NOSMMFUPROGRAMS: i32 = 0x40000069;
pub const REST_NOTRAYITEMSDISPLAY: i32 = 0x4000006A;
pub const REST_NOTOOLBARSONTASKBAR: i32 = 0x4000006B;
pub const REST_NOSMCONFIGUREPROGRAMS: i32 = 0x4000006F;
pub const REST_HIDECLOCK: i32 = 0x40000070;
pub const REST_NOLOWDISKSPACECHECKS: i32 = 0x40000071;
pub const REST_NOENTIRENETWORK: i32 = 0x40000072;
pub const REST_NODESKTOPCLEANUP: i32 = 0x40000073;
pub const REST_BITBUCKNUKEONDELETE: i32 = 0x40000074;
pub const REST_BITBUCKCONFIRMDELETE: i32 = 0x40000075;
pub const REST_BITBUCKNOPROP: i32 = 0x40000076;
pub const REST_NODISPBACKGROUND: i32 = 0x40000077;
pub const REST_NODISPSCREENSAVEPG: i32 = 0x40000078;
pub const REST_NODISPSETTINGSPG: i32 = 0x40000079;
pub const REST_NODISPSCREENSAVEPREVIEW: i32 = 0x4000007A;
pub const REST_NODISPLAYCPL: i32 = 0x4000007B;
pub const REST_HIDERUNASVERB: i32 = 0x4000007C;
pub const REST_NOTHUMBNAILCACHE: i32 = 0x4000007D;
pub const REST_NOSTRCMPLOGICAL: i32 = 0x4000007E;
pub const REST_NOPUBLISHWIZARD: i32 = 0x4000007F;
pub const REST_NOONLINEPRINTSWIZARD: i32 = 0x40000080;
pub const REST_NOWEBSERVICES: i32 = 0x40000081;
pub const REST_ALLOWUNHASHEDWEBVIEW: i32 = 0x40000082;
pub const REST_ALLOWLEGACYWEBVIEW: i32 = 0x40000083;
pub const REST_REVERTWEBVIEWSECURITY: i32 = 0x40000084;
pub const REST_INHERITCONSOLEHANDLES: i32 = 0x40000086;
pub const REST_NOREMOTERECURSIVEEVENTS: i32 = 0x40000089;
pub const REST_NOREMOTECHANGENOTIFY: i32 = 0x40000091;
pub const REST_NOENUMENTIRENETWORK: i32 = 0x40000093;
pub const REST_NOINTERNETOPENWITH: i32 = 0x40000095;
pub const REST_DONTRETRYBADNETNAME: i32 = 0x4000009B;
pub const REST_ALLOWFILECLSIDJUNCTIONS: i32 = 0x4000009C;
pub const REST_NOUPNPINSTALL: i32 = 0x4000009D;
pub const REST_ARP_DONTGROUPPATCHES: i32 = 0x400000AC;
pub const REST_ARP_NOCHOOSEPROGRAMSPAGE: i32 = 0x400000AD;
pub const REST_NODISCONNECT: i32 = 0x41000001;
pub const REST_NOSECURITY: i32 = 0x41000002;
pub const REST_NOFILEASSOCIATE: i32 = 0x41000003;
pub const REST_ALLOWCOMMENTTOGGLE: i32 = 0x41000004;
pub struct Rgbquad {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
pub struct Rgbtriple {
    pub rgbtBlue: u8,
    pub rgbtGreen: u8,
    pub rgbtRed: u8,
}
pub struct Rgndata {
    pub rdh: RGNDATAHEADER,
    pub Buffer: [CHAR; 1],
}
pub struct Rgndataheader {
    pub dwSize: u32,
    pub iType: u32,
    pub nCount: u32,
    pub nRgnSize: u32,
    pub rcBound: RECT,
}
pub const RGN_AND: i32 = 0x1;
pub const RGN_OR: i32 = 0x2;
pub const RGN_XOR: i32 = 0x3;
pub const RGN_DIFF: i32 = 0x4;
pub const RGN_COPY: i32 = 0x5;
pub const RGN_MIN: i32 = 0x1;
pub const RGN_MAX: i32 = 0x5;
pub const SRCCOPY: u32 = 0xCC0020;
pub const SRCPAINT: u32 = 0xEE0086;
pub const SRCAND: u32 = 0x8800C6;
pub const SRCINVERT: u32 = 0x660046;
pub const SRCERASE: u32 = 0x440328;
pub const NOTSRCCOPY: u32 = 0x330008;
pub const NOTSRCERASE: u32 = 0x1100A6;
pub const MERGECOPY: u32 = 0xC000CA;
pub const MERGEPAINT: u32 = 0xBB0226;
pub const PATCOPY: u32 = 0xF00021;
pub const PATPAINT: u32 = 0xFB0A09;
pub const PATINVERT: u32 = 0x5A0049;
pub const DSTINVERT: u32 = 0x550009;
pub const BLACKNESS: u32 = 0x42;
pub const WHITENESS: u32 = 0xFF0062;
pub const NOMIRRORBITMAP: u32 = 0x80000000;
pub const CAPTUREBLT: u32 = 0x40000000;
pub const RRF_RT_ANY: u32 = 0xFFFF;
pub const RRF_RT_DWORD: u32 = 0x18;
pub const RRF_RT_QWORD: u32 = 0x48;
pub const RRF_RT_REG_BINARY: u32 = 0x8;
pub const RRF_RT_REG_DWORD: u32 = 0x10;
pub const RRF_RT_REG_EXPAND_SZ: u32 = 0x4;
pub const RRF_RT_REG_MULTI_SZ: u32 = 0x20;
pub const RRF_RT_REG_NONE: u32 = 0x1;
pub const RRF_RT_REG_QWORD: u32 = 0x40;
pub const RRF_RT_REG_SZ: u32 = 0x2;
pub const REFRESH_NORMAL: i32 = 0x0;
pub const REFRESH_IFEXPIRED: i32 = 0x1;
pub const REFRESH_COMPLETELY: i32 = 0x3;
pub const SCF_VALUE_NONE: u32 = 0x0;
pub const SCF_SCALE: u32 = 0x1;
pub const SCF_PHYSICAL: u32 = 0x2;
pub const SCNRT_ENABLE: i32 = 0x0;
pub const SCNRT_DISABLE: i32 = 0x1;
pub struct Scrollbarinfo {
    pub cbSize: u32,
    pub rcScrollBar: RECT,
    pub dxyLineButton: i32,
    pub xyThumbTop: i32,
    pub xyThumbBottom: i32,
    pub reserved: i32,
    pub rgstate: [u32; 6],
}
pub const SB_CTL: u32 = 0x2;
pub const SB_HORZ: u32 = 0x0;
pub const SB_VERT: u32 = 0x1;
pub const SB_BOTH: u32 = 0x3;
pub struct Scrollinfo {
    pub cbSize: u32,
    pub fMask: u32,
    pub nMin: i32,
    pub nMax: i32,
    pub nPage: u32,
    pub nPos: i32,
    pub nTrackPos: i32,
}
pub const SIF_ALL: u32 = 0x17;
pub const SIF_DISABLENOSCROLL: u32 = 0x8;
pub const SIF_PAGE: u32 = 0x2;
pub const SIF_POS: u32 = 0x4;
pub const SIF_RANGE: u32 = 0x1;
pub const SIF_TRACKPOS: u32 = 0x10;
pub const SECURELOCK_NOCHANGE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const SECURELOCK_SET_UNSECURE: i32 = 0x0;
pub const SECURELOCK_SET_MIXED: i32 = 0x1;
pub const SECURELOCK_SET_SECUREUNKNOWNBIT: i32 = 0x2;
pub const SECURELOCK_SET_SECURE40BIT: i32 = 0x3;
pub const SECURELOCK_SET_SECURE56BIT: i32 = 0x4;
pub const SECURELOCK_SET_FORTEZZA: i32 = 0x5;
pub const SECURELOCK_SET_SECURE128BIT: i32 = 0x6;
pub const SECURELOCK_FIRSTSUGGEST: i32 = 0x7;
pub const SECURELOCK_SUGGEST_UNSECURE: i32 = 0x7;
pub const SECURELOCK_SUGGEST_MIXED: i32 = 0x8;
pub const SECURELOCK_SUGGEST_SECUREUNKNOWNBIT: i32 = 0x9;
pub const SECURELOCK_SUGGEST_SECURE40BIT: i32 = 0xA;
pub const SECURELOCK_SUGGEST_SECURE56BIT: i32 = 0xB;
pub const SECURELOCK_SUGGEST_FORTEZZA: i32 = 0xC;
pub const SECURELOCK_SUGGEST_SECURE128BIT: i32 = 0xD;
pub const SMTO_ABORTIFHUNG: u32 = 0x2;
pub const SMTO_BLOCK: u32 = 0x1;
pub const SMTO_NORMAL: u32 = 0x0;
pub const SMTO_NOTIMEOUTIFNOTHUNG: u32 = 0x8;
pub const SMTO_ERRORONEXIT: u32 = 0x20;
pub const DCB_ACCUMULATE: u32 = 0x2;
pub const DCB_DISABLE: u32 = 0x8;
pub const DCB_ENABLE: u32 = 0x4;
pub const DCB_RESET: u32 = 0x1;
pub const SWP_ASYNCWINDOWPOS: u32 = 0x4000;
pub const SWP_DEFERERASE: u32 = 0x2000;
pub const SWP_DRAWFRAME: u32 = 0x20;
pub const SWP_FRAMECHANGED: u32 = 0x20;
pub const SWP_HIDEWINDOW: u32 = 0x80;
pub const SWP_NOACTIVATE: u32 = 0x10;
pub const SWP_NOCOPYBITS: u32 = 0x100;
pub const SWP_NOMOVE: u32 = 0x2;
pub const SWP_NOOWNERZORDER: u32 = 0x200;
pub const SWP_NOREDRAW: u32 = 0x8;
pub const SWP_NOREPOSITION: u32 = 0x200;
pub const SWP_NOSENDCHANGING: u32 = 0x400;
pub const SWP_NOSIZE: u32 = 0x1;
pub const SWP_NOZORDER: u32 = 0x4;
pub const SWP_SHOWWINDOW: u32 = 0x40;
pub const SWP_NOOWNERZORDER: u32 = 0x200;
pub const SFBS_FLAGS_ROUND_TO_NEAREST_DISPLAYED_DIGIT: i32 = 0x1;
pub const SFBS_FLAGS_TRUNCATE_UNDISPLAYED_DECIMAL_DIGITS: i32 = 0x2;
pub struct SfvmHelptopicData {
    pub wszHelpFile: [u8; 260],
    pub wszHelpTopic: [u8; 260],
}
pub const SFVM_MERGEMENU: i32 = 0x1;
pub const SFVM_INVOKECOMMAND: i32 = 0x2;
pub const SFVM_GETHELPTEXT: i32 = 0x3;
pub const SFVM_GETTOOLTIPTEXT: i32 = 0x4;
pub const SFVM_GETBUTTONINFO: i32 = 0x5;
pub const SFVM_GETBUTTONS: i32 = 0x6;
pub const SFVM_INITMENUPOPUP: i32 = 0x7;
pub const SFVM_FSNOTIFY: i32 = 0xE;
pub const SFVM_WINDOWCREATED: i32 = 0xF;
pub const SFVM_GETDETAILSOF: i32 = 0x17;
pub const SFVM_COLUMNCLICK: i32 = 0x18;
pub const SFVM_QUERYFSNOTIFY: i32 = 0x19;
pub const SFVM_DEFITEMCOUNT: i32 = 0x1A;
pub const SFVM_DEFVIEWMODE: i32 = 0x1B;
pub const SFVM_UNMERGEMENU: i32 = 0x1C;
pub const SFVM_UPDATESTATUSBAR: i32 = 0x1F;
pub const SFVM_BACKGROUNDENUM: i32 = 0x20;
pub const SFVM_DIDDRAGDROP: i32 = 0x24;
pub const SFVM_SETISFV: i32 = 0x27;
pub const SFVM_THISIDLIST: i32 = 0x29;
pub const SFVM_ADDPROPERTYPAGES: i32 = 0x2F;
pub const SFVM_BACKGROUNDENUMDONE: i32 = 0x30;
pub const SFVM_GETNOTIFY: i32 = 0x31;
pub const SFVM_GETSORTDEFAULTS: i32 = 0x35;
pub const SFVM_SIZE: i32 = 0x39;
pub const SFVM_GETZONE: i32 = 0x3A;
pub const SFVM_GETPANE: i32 = 0x3B;
pub const SFVM_GETHELPTOPIC: i32 = 0x3F;
pub const SFVM_GETANIMATION: i32 = 0x44;
pub struct SfvmProppageData {
    pub dwReserved: u32,
    pub pfn: todo_fn,
    pub lParam: LPARAM,
}
pub const SFVS_SELECT_NONE: i32 = 0x0;
pub const SFVS_SELECT_ALLITEMS: i32 = 0x1;
pub const SFVS_SELECT_INVERT: i32 = 0x2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShandlePtr {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl ShandlePtr {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for ShandlePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub const SHARE_ROLE_INVALID: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const SHARE_ROLE_READER: i32 = 0x0;
pub const SHARE_ROLE_CONTRIBUTOR: i32 = 0x1;
pub const SHARE_ROLE_CO_OWNER: i32 = 0x2;
pub const SHARE_ROLE_OWNER: i32 = 0x3;
pub const SHARE_ROLE_CUSTOM: i32 = 0x4;
pub const SHARE_ROLE_MIXED: i32 = 0x5;
pub const SHCNE_RENAMEITEM: u32 = 0x1;
pub const SHCNE_CREATE: u32 = 0x2;
pub const SHCNE_DELETE: u32 = 0x4;
pub const SHCNE_MKDIR: u32 = 0x8;
pub const SHCNE_RMDIR: u32 = 0x10;
pub const SHCNE_MEDIAINSERTED: u32 = 0x20;
pub const SHCNE_MEDIAREMOVED: u32 = 0x40;
pub const SHCNE_DRIVEREMOVED: u32 = 0x80;
pub const SHCNE_DRIVEADD: u32 = 0x100;
pub const SHCNE_NETSHARE: u32 = 0x200;
pub const SHCNE_NETUNSHARE: u32 = 0x400;
pub const SHCNE_ATTRIBUTES: u32 = 0x800;
pub const SHCNE_UPDATEDIR: u32 = 0x1000;
pub const SHCNE_UPDATEITEM: u32 = 0x2000;
pub const SHCNE_SERVERDISCONNECT: u32 = 0x4000;
pub const SHCNE_UPDATEIMAGE: u32 = 0x8000;
pub const SHCNE_DRIVEADDGUI: u32 = 0x10000;
pub const SHCNE_RENAMEFOLDER: u32 = 0x20000;
pub const SHCNE_FREESPACE: u32 = 0x40000;
pub const SHCNE_EXTENDED_EVENT: u32 = 0x4000000;
pub const SHCNE_ASSOCCHANGED: u32 = 0x8000000;
pub const SHCNE_DISKEVENTS: u32 = 0x2381F;
pub const SHCNE_GLOBALEVENTS: u32 = 0xC0581E0;
pub const SHCNE_ALLEVENTS: u32 = 0x7FFFFFFF;
pub const SHCNE_INTERRUPT: u32 = 0x80000000;
pub const SHCNF_IDLIST: u32 = 0x0;
pub const SHCNF_PATHA: u32 = 0x1;
pub const SHCNF_PRINTERA: u32 = 0x2;
pub const SHCNF_DWORD: u32 = 0x3;
pub const SHCNF_PATHW: u32 = 0x5;
pub const SHCNF_PRINTERW: u32 = 0x6;
pub const SHCNF_TYPE: u32 = 0xFF;
pub const SHCNF_FLUSH: u32 = 0x1000;
pub const SHCNF_FLUSHNOWAIT: u32 = 0x3000;
pub const SHCNF_NOTIFYRECURSIVE: u32 = 0x10000;
pub const SHCNF_PATH: u32 = 0x5;
pub const SHCNF_PRINTER: u32 = 0x6;
pub const SHCNRF_INTERRUPT_LEVEL: i32 = 0x1;
pub const SHCNRF_SHELL_LEVEL: i32 = 0x2;
pub const SHCNRF_RECURSIVE_INTERRUPT: i32 = 0x1000;
pub const SHCNRF_NEW_DELIVERY: i32 = 0x8000;
pub struct Shdescriptionid {
    pub dwDescriptionId: i32,
    pub clsid: GUID,
}
pub const SHDID_ROOT_REGITEM: i32 = 0x1;
pub const SHDID_FS_FILE: i32 = 0x2;
pub const SHDID_FS_DIRECTORY: i32 = 0x3;
pub const SHDID_FS_OTHER: i32 = 0x4;
pub const SHDID_COMPUTER_DRIVE35: i32 = 0x5;
pub const SHDID_COMPUTER_DRIVE525: i32 = 0x6;
pub const SHDID_COMPUTER_REMOVABLE: i32 = 0x7;
pub const SHDID_COMPUTER_FIXED: i32 = 0x8;
pub const SHDID_COMPUTER_NETDRIVE: i32 = 0x9;
pub const SHDID_COMPUTER_CDROM: i32 = 0xA;
pub const SHDID_COMPUTER_RAMDISK: i32 = 0xB;
pub const SHDID_COMPUTER_OTHER: i32 = 0xC;
pub const SHDID_NET_DOMAIN: i32 = 0xD;
pub const SHDID_NET_SERVER: i32 = 0xE;
pub const SHDID_NET_SHARE: i32 = 0xF;
pub const SHDID_NET_RESTOFNET: i32 = 0x10;
pub const SHDID_NET_OTHER: i32 = 0x11;
pub const SHDID_COMPUTER_IMAGING: i32 = 0x12;
pub const SHDID_COMPUTER_AUDIO: i32 = 0x13;
pub const SHDID_COMPUTER_SHAREDDOCS: i32 = 0x14;
pub const SHDID_MOBILE_DEVICE: i32 = 0x15;
pub const SHDID_REMOTE_DESKTOP_DRIVE: i32 = 0x16;
pub struct Shdragimage {
    pub sizeDragImage: SIZE,
    pub ptOffset: POINT,
    pub hbmpDragImage: HBITMAP,
    pub crColorKey: u32,
}
pub struct Shellexecuteinfoa<'a> {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: HWND,
    pub lpVerb: &'a Cow<'a, str>,
    pub lpFile: &'a Cow<'a, str>,
    pub lpParameters: &'a Cow<'a, str>,
    pub lpDirectory: &'a Cow<'a, str>,
    pub nShow: i32,
    pub hInstApp: HINSTANCE,
    pub lpIDList: &'a mut todo_void,
    pub lpClass: &'a Cow<'a, str>,
    pub hkeyClass: HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOA_49,
    pub hProcess: HANDLE,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Shellexecuteinfoa48 {
    pub field0: HANDLE,
    pub field1: HANDLE,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Shellexecuteinfoa49 {
    pub field0: HANDLE,
    pub field1: HANDLE,
}
pub struct Shellexecuteinfow<'a> {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: HWND,
    pub lpVerb: &'a Cow<'a, OsStr>,
    pub lpFile: &'a Cow<'a, OsStr>,
    pub lpParameters: &'a Cow<'a, OsStr>,
    pub lpDirectory: &'a Cow<'a, OsStr>,
    pub nShow: i32,
    pub hInstApp: HINSTANCE,
    pub lpIDList: &'a mut todo_void,
    pub lpClass: &'a Cow<'a, OsStr>,
    pub hkeyClass: HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOW_51,
    pub hProcess: HANDLE,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Shellexecuteinfow50 {
    pub field0: HANDLE,
    pub field1: HANDLE,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Shellexecuteinfow51 {
    pub field0: HANDLE,
    pub field1: HANDLE,
}
pub struct Shellflagstate {
    pub _bitfield: i32,
}
pub struct Shellhookinfo {
    pub hwnd: HWND,
    pub rc: RECT,
}
pub struct Shellstatea {
    pub _bitfield1: i32,
    pub dwWin95Unused: u32,
    pub uWin95Unused: u32,
    pub lParamSort: i32,
    pub iSortDirection: i32,
    pub version: u32,
    pub uNotUsed: u32,
    pub _bitfield2: i32,
}
pub struct Shellstatew {
    pub _bitfield1: i32,
    pub dwWin95Unused: u32,
    pub uWin95Unused: u32,
    pub lParamSort: i32,
    pub iSortDirection: i32,
    pub version: u32,
    pub uNotUsed: u32,
    pub _bitfield2: i32,
}
pub struct ShellItemResource {
    pub guidType: GUID,
    pub szName: [u8; 260],
}
pub const SLDF_DEFAULT: i32 = 0x0;
pub const SLDF_HAS_ID_LIST: i32 = 0x1;
pub const SLDF_HAS_LINK_INFO: i32 = 0x2;
pub const SLDF_HAS_NAME: i32 = 0x4;
pub const SLDF_HAS_RELPATH: i32 = 0x8;
pub const SLDF_HAS_WORKINGDIR: i32 = 0x10;
pub const SLDF_HAS_ARGS: i32 = 0x20;
pub const SLDF_HAS_ICONLOCATION: i32 = 0x40;
pub const SLDF_UNICODE: i32 = 0x80;
pub const SLDF_FORCE_NO_LINKINFO: i32 = 0x100;
pub const SLDF_HAS_EXP_SZ: i32 = 0x200;
pub const SLDF_RUN_IN_SEPARATE: i32 = 0x400;
pub const SLDF_HAS_DARWINID: i32 = 0x1000;
pub const SLDF_RUNAS_USER: i32 = 0x2000;
pub const SLDF_HAS_EXP_ICON_SZ: i32 = 0x4000;
pub const SLDF_NO_PIDL_ALIAS: i32 = 0x8000;
pub const SLDF_FORCE_UNCNAME: i32 = 0x10000;
pub const SLDF_RUN_WITH_SHIMLAYER: i32 = 0x20000;
pub const SLDF_FORCE_NO_LINKTRACK: i32 = 0x40000;
pub const SLDF_ENABLE_TARGET_METADATA: i32 = 0x80000;
pub const SLDF_DISABLE_LINK_PATH_TRACKING: i32 = 0x100000;
pub const SLDF_DISABLE_KNOWNFOLDER_RELATIVE_TRACKING: i32 = 0x200000;
pub const SLDF_NO_KF_ALIAS: i32 = 0x400000;
pub const SLDF_ALLOW_LINK_TO_LINK: i32 = 0x800000;
pub const SLDF_UNALIAS_ON_SAVE: i32 = 0x1000000;
pub const SLDF_PREFER_ENVIRONMENT_PATH: i32 = 0x2000000;
pub const SLDF_KEEP_LOCAL_IDLIST_FOR_UNC_TARGET: i32 = 0x4000000;
pub const SLDF_PERSIST_VOLUME_ID_RELATIVE: i32 = 0x8000000;
pub const SLDF_VALID: i32 = 0xFFFF7FF;
pub const SLDF_RESERVED: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000;
pub const SHELL_UI_COMPONENT_TASKBARS: i32 = 0x0;
pub const SHELL_UI_COMPONENT_NOTIFICATIONAREA: i32 = 0x1;
pub const SHELL_UI_COMPONENT_DESKBAND: i32 = 0x2;
pub struct Shfileinfoa {
    pub hIcon: HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [CHAR; 260],
    pub szTypeName: [CHAR; 80],
}
pub struct Shfileinfow {
    pub hIcon: HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [u8; 260],
    pub szTypeName: [u8; 80],
}
pub struct Shfileopstructa<'a> {
    pub hwnd: HWND,
    pub wFunc: u32,
    pub pFrom: &'a mut i8,
    pub pTo: &'a mut i8,
    pub fFlags: u16,
    pub fAnyOperationsAborted: BOOL,
    pub hNameMappings: &'a mut todo_void,
    pub lpszProgressTitle: &'a Cow<'a, str>,
}
pub struct Shfileopstructw<'a> {
    pub hwnd: HWND,
    pub wFunc: u32,
    pub pFrom: &'a Cow<'a, OsStr>,
    pub pTo: &'a Cow<'a, OsStr>,
    pub fFlags: u16,
    pub fAnyOperationsAborted: BOOL,
    pub hNameMappings: &'a mut todo_void,
    pub lpszProgressTitle: &'a Cow<'a, OsStr>,
}
pub const SHFMT_ID_DEFAULT: u32 = 0xFFFF;
pub const SHFMT_OPT_NONE: i32 = 0x0;
pub const SHFMT_OPT_FULL: i32 = 0x1;
pub const SHFMT_OPT_SYSONLY: i32 = 0x2;
pub const SHFMT_ERROR: u32 = 0xFFFFFFFF;
pub const SHFMT_CANCEL: u32 = 0xFFFFFFFE;
pub const SHFMT_NOFORMAT: u32 = 0xFFFFFFFD;
pub struct Shfoldercustomsettings<'a> {
    pub dwSize: u32,
    pub dwMask: u32,
    pub pvid: &'a mut GUID,
    pub pszWebViewTemplate: &'a mut Cow<'a, OsStr>,
    pub cchWebViewTemplate: u32,
    pub pszWebViewTemplateVersion: &'a mut Cow<'a, OsStr>,
    pub pszInfoTip: &'a mut Cow<'a, OsStr>,
    pub cchInfoTip: u32,
    pub pclsid: &'a mut GUID,
    pub dwFlags: u32,
    pub pszIconFile: &'a mut Cow<'a, OsStr>,
    pub cchIconFile: u32,
    pub iIconIndex: i32,
    pub pszLogo: &'a mut Cow<'a, OsStr>,
    pub cchLogo: u32,
}
pub const SHGDFIL_FINDDATA: i32 = 0x1;
pub const SHGDFIL_NETRESOURCE: i32 = 0x2;
pub const SHGDFIL_DESCRIPTIONID: i32 = 0x3;
pub const SHGFI_ADDOVERLAYS: i32 = 0x20;
pub const SHGFI_ATTR_SPECIFIED: i32 = 0x20000;
pub const SHGFI_ATTRIBUTES: i32 = 0x800;
pub const SHGFI_DISPLAYNAME: i32 = 0x200;
pub const SHGFI_EXETYPE: i32 = 0x2000;
pub const SHGFI_ICON: i32 = 0x100;
pub const SHGFI_ICONLOCATION: i32 = 0x1000;
pub const SHGFI_LARGEICON: i32 = 0x0;
pub const SHGFI_LINKOVERLAY: i32 = 0x8000;
pub const SHGFI_OPENICON: i32 = 0x2;
pub const SHGFI_OVERLAYINDEX: i32 = 0x40;
pub const SHGFI_PIDL: i32 = 0x8;
pub const SHGFI_SELECTED: i32 = 0x10000;
pub const SHGFI_SHELLICONSIZE: i32 = 0x4;
pub const SHGFI_SMALLICON: i32 = 0x1;
pub const SHGFI_SYSICONINDEX: i32 = 0x4000;
pub const SHGFI_TYPENAME: i32 = 0x400;
pub const SHGFI_USEFILEATTRIBUTES: i32 = 0x10;
pub const SHGFP_TYPE_CURRENT: i32 = 0x0;
pub const SHGFP_TYPE_DEFAULT: i32 = 0x1;
pub const GLOBALCOUNTER_SEARCHMANAGER: i32 = 0x0;
pub const GLOBALCOUNTER_SEARCHOPTIONS: i32 = 0x1;
pub const GLOBALCOUNTER_FOLDERSETTINGSCHANGE: i32 = 0x2;
pub const GLOBALCOUNTER_RATINGS: i32 = 0x3;
pub const GLOBALCOUNTER_APPROVEDSITES: i32 = 0x4;
pub const GLOBALCOUNTER_RESTRICTIONS: i32 = 0x5;
pub const GLOBALCOUNTER_SHELLSETTINGSCHANGED: i32 = 0x6;
pub const GLOBALCOUNTER_SYSTEMPIDLCHANGE: i32 = 0x7;
pub const GLOBALCOUNTER_OVERLAYMANAGER: i32 = 0x8;
pub const GLOBALCOUNTER_QUERYASSOCIATIONS: i32 = 0x9;
pub const GLOBALCOUNTER_IESESSIONS: i32 = 0xA;
pub const GLOBALCOUNTER_IEONLY_SESSIONS: i32 = 0xB;
pub const GLOBALCOUNTER_APPLICATION_DESTINATIONS: i32 = 0xC;
pub const UNUSED_RECYCLE_WAS_GLOBALCOUNTER_CSCSYNCINPROGRESS: i32 = 0xD;
pub const GLOBALCOUNTER_BITBUCKETNUMDELETERS: i32 = 0xE;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SHARES: i32 = 0xF;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_A: i32 = 0x10;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_B: i32 = 0x11;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_C: i32 = 0x12;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_D: i32 = 0x13;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_E: i32 = 0x14;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_F: i32 = 0x15;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_G: i32 = 0x16;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_H: i32 = 0x17;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_I: i32 = 0x18;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_J: i32 = 0x19;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_K: i32 = 0x1A;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_L: i32 = 0x1B;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_M: i32 = 0x1C;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_N: i32 = 0x1D;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_O: i32 = 0x1E;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_P: i32 = 0x1F;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Q: i32 = 0x20;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_R: i32 = 0x21;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_S: i32 = 0x22;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_T: i32 = 0x23;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_U: i32 = 0x24;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_V: i32 = 0x25;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_W: i32 = 0x26;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_X: i32 = 0x27;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Y: i32 = 0x28;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Z: i32 = 0x29;
pub const UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SERVERDRIVE: i32 = 0x2A;
pub const UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEGLOBALDIRTYCOUNT: i32 = 0x2B;
pub const GLOBALCOUNTER_RECYCLEBINENUM: i32 = 0x2C;
pub const GLOBALCOUNTER_RECYCLEBINCORRUPTED: i32 = 0x2D;
pub const GLOBALCOUNTER_RATINGS_STATECOUNTER: i32 = 0x2E;
pub const GLOBALCOUNTER_PRIVATE_PROFILE_CACHE: i32 = 0x2F;
pub const GLOBALCOUNTER_INTERNETTOOLBAR_LAYOUT: i32 = 0x30;
pub const GLOBALCOUNTER_FOLDERDEFINITION_CACHE: i32 = 0x31;
pub const GLOBALCOUNTER_COMMONPLACES_LIST_CACHE: i32 = 0x32;
pub const GLOBALCOUNTER_PRIVATE_PROFILE_CACHE_MACHINEWIDE: i32 = 0x33;
pub const GLOBALCOUNTER_ASSOCCHANGED: i32 = 0x34;
pub const GLOBALCOUNTER_APP_ITEMS_STATE_STORE_CACHE: i32 = 0x35;
pub const GLOBALCOUNTER_SETTINGSYNC_ENABLED: i32 = 0x36;
pub const GLOBALCOUNTER_APPSFOLDER_FILETYPEASSOCIATION_COUNTER: i32 = 0x37;
pub const GLOBALCOUNTER_USERINFOCHANGED: i32 = 0x38;
pub const GLOBALCOUNTER_SYNC_ENGINE_INFORMATION_CACHE_MACHINEWIDE: i32 = 0x39;
pub const GLOBALCOUNTER_BANNERS_DATAMODEL_CACHE_MACHINEWIDE: i32 = 0x3A;
pub const GLOBALCOUNTER_MAXIMUMVALUE: i32 = 0x3B;
pub struct Shnamemappinga<'a> {
    pub pszOldPath: &'a mut Cow<'a, str>,
    pub pszNewPath: &'a mut Cow<'a, str>,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
pub struct Shnamemappingw<'a> {
    pub pszOldPath: &'a mut Cow<'a, OsStr>,
    pub pszNewPath: &'a mut Cow<'a, OsStr>,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
pub const SHOP_PRINTERNAME: i32 = 0x1;
pub const SHOP_FILEPATH: i32 = 0x2;
pub const SHOP_VOLUMEGUID: i32 = 0x4;
pub const SW_FORCEMINIMIZE: u32 = 0xB;
pub const SW_HIDE: u32 = 0x0;
pub const SW_MAXIMIZE: u32 = 0x3;
pub const SW_MINIMIZE: u32 = 0x6;
pub const SW_RESTORE: u32 = 0x9;
pub const SW_SHOW: u32 = 0x5;
pub const SW_SHOWDEFAULT: u32 = 0xA;
pub const SW_SHOWMAXIMIZED: u32 = 0x3;
pub const SW_SHOWMINIMIZED: u32 = 0x2;
pub const SW_SHOWMINNOACTIVE: u32 = 0x7;
pub const SW_SHOWNA: u32 = 0x8;
pub const SW_SHOWNOACTIVATE: u32 = 0x4;
pub const SW_SHOWNORMAL: u32 = 0x1;
pub const SW_NORMAL: u32 = 0x1;
pub const SW_MAX: u32 = 0xB;
pub const SW_PARENTCLOSING: u32 = 0x1;
pub const SW_OTHERZOOM: u32 = 0x2;
pub const SW_PARENTOPENING: u32 = 0x3;
pub const SW_OTHERUNZOOM: u32 = 0x4;
pub const SW_SCROLLCHILDREN: u32 = 0x1;
pub const SW_INVALIDATE: u32 = 0x2;
pub const SW_ERASE: u32 = 0x4;
pub const SW_SMOOTHSCROLL: u32 = 0x10;
pub struct Shqueryrbinfo {
    pub cbSize: u32,
    pub i64Size: i64,
    pub i64NumItems: i64,
}
pub const SHREGDEL_DEFAULT: i32 = 0x0;
pub const SHREGDEL_HKCU: i32 = 0x1;
pub const SHREGDEL_HKLM: i32 = 0x10;
pub const SHREGDEL_BOTH: i32 = 0x11;
pub const SHREGENUM_DEFAULT: i32 = 0x0;
pub const SHREGENUM_HKCU: i32 = 0x1;
pub const SHREGENUM_HKLM: i32 = 0x10;
pub const SHREGENUM_BOTH: i32 = 0x11;
pub const SIID_DOCNOASSOC: i32 = 0x0;
pub const SIID_DOCASSOC: i32 = 0x1;
pub const SIID_APPLICATION: i32 = 0x2;
pub const SIID_FOLDER: i32 = 0x3;
pub const SIID_FOLDEROPEN: i32 = 0x4;
pub const SIID_DRIVE525: i32 = 0x5;
pub const SIID_DRIVE35: i32 = 0x6;
pub const SIID_DRIVEREMOVE: i32 = 0x7;
pub const SIID_DRIVEFIXED: i32 = 0x8;
pub const SIID_DRIVENET: i32 = 0x9;
pub const SIID_DRIVENETDISABLED: i32 = 0xA;
pub const SIID_DRIVECD: i32 = 0xB;
pub const SIID_DRIVERAM: i32 = 0xC;
pub const SIID_WORLD: i32 = 0xD;
pub const SIID_SERVER: i32 = 0xF;
pub const SIID_PRINTER: i32 = 0x10;
pub const SIID_MYNETWORK: i32 = 0x11;
pub const SIID_FIND: i32 = 0x16;
pub const SIID_HELP: i32 = 0x17;
pub const SIID_SHARE: i32 = 0x1C;
pub const SIID_LINK: i32 = 0x1D;
pub const SIID_SLOWFILE: i32 = 0x1E;
pub const SIID_RECYCLER: i32 = 0x1F;
pub const SIID_RECYCLERFULL: i32 = 0x20;
pub const SIID_MEDIACDAUDIO: i32 = 0x28;
pub const SIID_LOCK: i32 = 0x2F;
pub const SIID_AUTOLIST: i32 = 0x31;
pub const SIID_PRINTERNET: i32 = 0x32;
pub const SIID_SERVERSHARE: i32 = 0x33;
pub const SIID_PRINTERFAX: i32 = 0x34;
pub const SIID_PRINTERFAXNET: i32 = 0x35;
pub const SIID_PRINTERFILE: i32 = 0x36;
pub const SIID_STACK: i32 = 0x37;
pub const SIID_MEDIASVCD: i32 = 0x38;
pub const SIID_STUFFEDFOLDER: i32 = 0x39;
pub const SIID_DRIVEUNKNOWN: i32 = 0x3A;
pub const SIID_DRIVEDVD: i32 = 0x3B;
pub const SIID_MEDIADVD: i32 = 0x3C;
pub const SIID_MEDIADVDRAM: i32 = 0x3D;
pub const SIID_MEDIADVDRW: i32 = 0x3E;
pub const SIID_MEDIADVDR: i32 = 0x3F;
pub const SIID_MEDIADVDROM: i32 = 0x40;
pub const SIID_MEDIACDAUDIOPLUS: i32 = 0x41;
pub const SIID_MEDIACDRW: i32 = 0x42;
pub const SIID_MEDIACDR: i32 = 0x43;
pub const SIID_MEDIACDBURN: i32 = 0x44;
pub const SIID_MEDIABLANKCD: i32 = 0x45;
pub const SIID_MEDIACDROM: i32 = 0x46;
pub const SIID_AUDIOFILES: i32 = 0x47;
pub const SIID_IMAGEFILES: i32 = 0x48;
pub const SIID_VIDEOFILES: i32 = 0x49;
pub const SIID_MIXEDFILES: i32 = 0x4A;
pub const SIID_FOLDERBACK: i32 = 0x4B;
pub const SIID_FOLDERFRONT: i32 = 0x4C;
pub const SIID_SHIELD: i32 = 0x4D;
pub const SIID_WARNING: i32 = 0x4E;
pub const SIID_INFO: i32 = 0x4F;
pub const SIID_ERROR: i32 = 0x50;
pub const SIID_KEY: i32 = 0x51;
pub const SIID_SOFTWARE: i32 = 0x52;
pub const SIID_RENAME: i32 = 0x53;
pub const SIID_DELETE: i32 = 0x54;
pub const SIID_MEDIAAUDIODVD: i32 = 0x55;
pub const SIID_MEDIAMOVIEDVD: i32 = 0x56;
pub const SIID_MEDIAENHANCEDCD: i32 = 0x57;
pub const SIID_MEDIAENHANCEDDVD: i32 = 0x58;
pub const SIID_MEDIAHDDVD: i32 = 0x59;
pub const SIID_MEDIABLURAY: i32 = 0x5A;
pub const SIID_MEDIAVCD: i32 = 0x5B;
pub const SIID_MEDIADVDPLUSR: i32 = 0x5C;
pub const SIID_MEDIADVDPLUSRW: i32 = 0x5D;
pub const SIID_DESKTOPPC: i32 = 0x5E;
pub const SIID_MOBILEPC: i32 = 0x5F;
pub const SIID_USERS: i32 = 0x60;
pub const SIID_MEDIASMARTMEDIA: i32 = 0x61;
pub const SIID_MEDIACOMPACTFLASH: i32 = 0x62;
pub const SIID_DEVICECELLPHONE: i32 = 0x63;
pub const SIID_DEVICECAMERA: i32 = 0x64;
pub const SIID_DEVICEVIDEOCAMERA: i32 = 0x65;
pub const SIID_DEVICEAUDIOPLAYER: i32 = 0x66;
pub const SIID_NETWORKCONNECT: i32 = 0x67;
pub const SIID_INTERNET: i32 = 0x68;
pub const SIID_ZIPFILE: i32 = 0x69;
pub const SIID_SETTINGS: i32 = 0x6A;
pub const SIID_DRIVEHDDVD: i32 = 0x84;
pub const SIID_DRIVEBD: i32 = 0x85;
pub const SIID_MEDIAHDDVDROM: i32 = 0x86;
pub const SIID_MEDIAHDDVDR: i32 = 0x87;
pub const SIID_MEDIAHDDVDRAM: i32 = 0x88;
pub const SIID_MEDIABDROM: i32 = 0x89;
pub const SIID_MEDIABDR: i32 = 0x8A;
pub const SIID_MEDIABDRE: i32 = 0x8B;
pub const SIID_CLUSTEREDDRIVE: i32 = 0x8C;
pub const SIID_MAX_ICONS: i32 = 0xB5;
pub struct Shstockiconinfo {
    pub cbSize: u32,
    pub hIcon: HICON,
    pub iSysImageIndex: i32,
    pub iIcon: i32,
    pub szPath: [u8; 260],
}
pub const SIATTRIBFLAGS_AND: i32 = 0x1;
pub const SIATTRIBFLAGS_OR: i32 = 0x2;
pub const SIATTRIBFLAGS_APPCOMPAT: i32 = 0x3;
pub const SIATTRIBFLAGS_MASK: i32 = 0x3;
pub const SIATTRIBFLAGS_ALLITEMS: i32 = 0x4000;
pub const SIGDN_NORMALDISPLAY: i32 = 0x0;
pub const SIGDN_PARENTRELATIVEPARSING: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80018001;
pub const SIGDN_DESKTOPABSOLUTEPARSING: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80028000;
pub const SIGDN_PARENTRELATIVEEDITING: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80031001;
pub const SIGDN_DESKTOPABSOLUTEEDITING: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF8004C000;
pub const SIGDN_FILESYSPATH: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80058000;
pub const SIGDN_URL: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80068000;
pub const SIGDN_PARENTRELATIVEFORADDRESSBAR: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF8007C001;
pub const SIGDN_PARENTRELATIVE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80080001;
pub const SIGDN_PARENTRELATIVEFORUI: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80094001;
pub const SIIGBF_RESIZETOFIT: i32 = 0x0;
pub const SIIGBF_BIGGERSIZEOK: i32 = 0x1;
pub const SIIGBF_MEMORYONLY: i32 = 0x2;
pub const SIIGBF_ICONONLY: i32 = 0x4;
pub const SIIGBF_THUMBNAILONLY: i32 = 0x8;
pub const SIIGBF_INCACHEONLY: i32 = 0x10;
pub const SIIGBF_CROPTOSQUARE: i32 = 0x20;
pub const SIIGBF_WIDETHUMBNAILS: i32 = 0x40;
pub const SIIGBF_ICONBACKGROUND: i32 = 0x80;
pub const SIIGBF_SCALEUP: i32 = 0x100;
pub struct Size {
    pub cx: i32,
    pub cy: i32,
}
pub const ST_TRUESIZE: i32 = 0x0;
pub const ST_STRETCH: i32 = 0x1;
pub const ST_TILE: i32 = 0x2;
pub const SLGP_SHORTPATH: i32 = 0x1;
pub const SLGP_UNCPRIORITY: i32 = 0x2;
pub const SLGP_RAWPATH: i32 = 0x4;
pub const SLGP_RELATIVEPRIORITY: i32 = 0x8;
pub struct Slowappinfo<'a> {
    pub ullSize: u64,
    pub ftLastUsed: FILETIME,
    pub iTimesUsed: i32,
    pub pszImage: &'a mut Cow<'a, OsStr>,
}
pub const SLR_NONE: i32 = 0x0;
pub const SLR_NO_UI: i32 = 0x1;
pub const SLR_ANY_MATCH: i32 = 0x2;
pub const SLR_UPDATE: i32 = 0x4;
pub const SLR_NOUPDATE: i32 = 0x8;
pub const SLR_NOSEARCH: i32 = 0x10;
pub const SLR_NOTRACK: i32 = 0x20;
pub const SLR_NOLINKINFO: i32 = 0x40;
pub const SLR_INVOKE_MSI: i32 = 0x80;
pub const SLR_NO_UI_WITH_MSG_PUMP: i32 = 0x101;
pub const SLR_OFFER_DELETE_WITHOUT_FILE: i32 = 0x200;
pub const SLR_KNOWNFOLDER: i32 = 0x400;
pub const SLR_MACHINE_IN_LOCAL_TARGET: i32 = 0x800;
pub const SLR_UPDATE_MACHINE_AND_SID: i32 = 0x1000;
pub const SLR_NO_OBJECT_ID: i32 = 0x2000;
pub struct Sminfo {
    pub dwMask: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub iIcon: i32,
}
pub const SMIF_ICON: i32 = 0x1;
pub const SMIF_ACCELERATOR: i32 = 0x2;
pub const SMIF_DROPTARGET: i32 = 0x4;
pub const SMIF_SUBMENU: i32 = 0x8;
pub const SMIF_CHECKED: i32 = 0x20;
pub const SMIF_DROPCASCADE: i32 = 0x40;
pub const SMIF_HIDDEN: i32 = 0x80;
pub const SMIF_DISABLED: i32 = 0x100;
pub const SMIF_TRACKPOPUP: i32 = 0x200;
pub const SMIF_DEMOTED: i32 = 0x400;
pub const SMIF_ALTSTATE: i32 = 0x800;
pub const SMIF_DRAGNDROP: i32 = 0x1000;
pub const SMIF_NEW: i32 = 0x2000;
pub const SMIM_TYPE: i32 = 0x1;
pub const SMIM_FLAGS: i32 = 0x2;
pub const SMIM_ICON: i32 = 0x4;
pub const SMIT_SEPARATOR: i32 = 0x1;
pub const SMIT_STRING: i32 = 0x2;
pub const SPSE_NORMAL: i32 = 0x1;
pub const SPSE_HOT: i32 = 0x2;
pub const SPSE_SELECTED: i32 = 0x3;
pub const SPSE_DISABLED: i32 = 0x4;
pub const SPSE_FOCUSED: i32 = 0x5;
pub const SPACTION_NONE: i32 = 0x0;
pub const SPACTION_MOVING: i32 = 0x1;
pub const SPACTION_COPYING: i32 = 0x2;
pub const SPACTION_RECYCLING: i32 = 0x3;
pub const SPACTION_APPLYINGATTRIBS: i32 = 0x4;
pub const SPACTION_DOWNLOADING: i32 = 0x5;
pub const SPACTION_SEARCHING_INTERNET: i32 = 0x6;
pub const SPACTION_CALCULATING: i32 = 0x7;
pub const SPACTION_UPLOADING: i32 = 0x8;
pub const SPACTION_SEARCHING_FILES: i32 = 0x9;
pub const SPACTION_DELETING: i32 = 0xA;
pub const SPACTION_RENAMING: i32 = 0xB;
pub const SPACTION_FORMATTING: i32 = 0xC;
pub const SPACTION_COPY_MOVING: i32 = 0xD;
pub const SPTEXT_ACTIONDESCRIPTION: i32 = 0x1;
pub const SPTEXT_ACTIONDETAIL: i32 = 0x2;
pub const SSF_SHOWALLOBJECTS: u32 = 0x1;
pub const SSF_SHOWEXTENSIONS: u32 = 0x2;
pub const SSF_HIDDENFILEEXTS: u32 = 0x4;
pub const SSF_SERVERADMINUI: u32 = 0x4;
pub const SSF_SHOWCOMPCOLOR: u32 = 0x8;
pub const SSF_SORTCOLUMNS: u32 = 0x10;
pub const SSF_SHOWSYSFILES: u32 = 0x20;
pub const SSF_DOUBLECLICKINWEBVIEW: u32 = 0x80;
pub const SSF_SHOWATTRIBCOL: u32 = 0x100;
pub const SSF_DESKTOPHTML: u32 = 0x200;
pub const SSF_WIN95CLASSIC: u32 = 0x400;
pub const SSF_DONTPRETTYPATH: u32 = 0x800;
pub const SSF_SHOWINFOTIP: u32 = 0x2000;
pub const SSF_MAPNETDRVBUTTON: u32 = 0x1000;
pub const SSF_NOCONFIRMRECYCLE: u32 = 0x8000;
pub const SSF_HIDEICONS: u32 = 0x4000;
pub const SSF_FILTER: u32 = 0x10000;
pub const SSF_WEBVIEW: u32 = 0x20000;
pub const SSF_SHOWSUPERHIDDEN: u32 = 0x40000;
pub const SSF_SEPPROCESS: u32 = 0x80000;
pub const SSF_NONETCRAWLING: u32 = 0x100000;
pub const SSF_STARTPANELON: u32 = 0x200000;
pub const SSF_SHOWSTARTPAGE: u32 = 0x400000;
pub const SSF_AUTOCHECKSELECT: u32 = 0x800000;
pub const SSF_ICONSONLY: u32 = 0x1000000;
pub const SSF_SHOWTYPEOVERLAY: u32 = 0x2000000;
pub const SSF_SHOWSTATUSBAR: u32 = 0x4000000;
pub const SPP_USERPANE: i32 = 0x1;
pub const SPP_MOREPROGRAMS: i32 = 0x2;
pub const SPP_MOREPROGRAMSARROW: i32 = 0x3;
pub const SPP_PROGLIST: i32 = 0x4;
pub const SPP_PROGLISTSEPARATOR: i32 = 0x5;
pub const SPP_PLACESLIST: i32 = 0x6;
pub const SPP_PLACESLISTSEPARATOR: i32 = 0x7;
pub const SPP_LOGOFF: i32 = 0x8;
pub const SPP_LOGOFFBUTTONS: i32 = 0x9;
pub const SPP_USERPICTURE: i32 = 0xA;
pub const SPP_PREVIEW: i32 = 0xB;
pub const SPP_MOREPROGRAMSTAB: i32 = 0xC;
pub const SPP_NSCHOST: i32 = 0xD;
pub const SPP_SOFTWAREEXPLORER: i32 = 0xE;
pub const SPP_OPENBOX: i32 = 0xF;
pub const SPP_SEARCHVIEW: i32 = 0x10;
pub const SPP_MOREPROGRAMSARROWBACK: i32 = 0x11;
pub const SPP_TOPMATCH: i32 = 0x12;
pub const SPP_LOGOFFSPLITBUTTONDROPDOWN: i32 = 0x13;
pub const STAT_TEXT: i32 = 0x1;
pub const STGOP_MOVE: i32 = 0x1;
pub const STGOP_COPY: i32 = 0x2;
pub const STGOP_SYNC: i32 = 0x3;
pub const STGOP_REMOVE: i32 = 0x5;
pub const STGOP_RENAME: i32 = 0x6;
pub const STGOP_APPLYPROPERTIES: i32 = 0x8;
pub const STGOP_NEW: i32 = 0xA;
pub const SPFF_NONE: i32 = 0x0;
pub const SPFF_DOWNLOAD_BY_DEFAULT: i32 = 0x1;
pub const SPFF_CREATED_ON_THIS_DEVICE: i32 = 0x2;
pub const STPF_NONE: i32 = 0x0;
pub const STPF_USEAPPTHUMBNAILALWAYS: i32 = 0x1;
pub const STPF_USEAPPTHUMBNAILWHENACTIVE: i32 = 0x2;
pub const STPF_USEAPPPEEKALWAYS: i32 = 0x4;
pub const STPF_USEAPPPEEKWHENACTIVE: i32 = 0x8;
pub const BLACKONWHITE: u32 = 0x1;
pub const COLORONCOLOR: u32 = 0x3;
pub const HALFTONE: u32 = 0x4;
pub const STRETCH_ANDSCANS: u32 = 0x1;
pub const STRETCH_DELETESCANS: u32 = 0x3;
pub const STRETCH_HALFTONE: u32 = 0x4;
pub const STRETCH_ORSCANS: u32 = 0x2;
pub const WHITEONBLACK: u32 = 0x2;
pub struct Stylestruct {
    pub styleOld: u32,
    pub styleNew: u32,
}
pub const SVUIA_DEACTIVATE: i32 = 0x0;
pub const SVUIA_ACTIVATE_NOFOCUS: i32 = 0x1;
pub const SVUIA_ACTIVATE_FOCUS: i32 = 0x2;
pub const SVUIA_INPLACEACTIVATE: i32 = 0x3;
pub struct Systemtime {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
pub const OCR_APPSTARTING: u32 = 0x7F8A;
pub const OCR_NORMAL: u32 = 0x7F00;
pub const OCR_CROSS: u32 = 0x7F03;
pub const OCR_HAND: u32 = 0x7F89;
pub const OCR_HELP: u32 = 0x7F8B;
pub const OCR_IBEAM: u32 = 0x7F01;
pub const OCR_NO: u32 = 0x7F88;
pub const OCR_SIZEALL: u32 = 0x7F86;
pub const OCR_SIZENESW: u32 = 0x7F83;
pub const OCR_SIZENS: u32 = 0x7F85;
pub const OCR_SIZENWSE: u32 = 0x7F82;
pub const OCR_SIZEWE: u32 = 0x7F84;
pub const OCR_UP: u32 = 0x7F04;
pub const OCR_WAIT: u32 = 0x7F02;
pub const SM_ARRANGE: u32 = 0x38;
pub const SM_CLEANBOOT: u32 = 0x43;
pub const SM_CMONITORS: u32 = 0x50;
pub const SM_CMOUSEBUTTONS: u32 = 0x2B;
pub const SM_CONVERTIBLESLATEMODE: u32 = 0x2003;
pub const SM_CXBORDER: u32 = 0x5;
pub const SM_CXCURSOR: u32 = 0xD;
pub const SM_CXDLGFRAME: u32 = 0x7;
pub const SM_CXDOUBLECLK: u32 = 0x24;
pub const SM_CXDRAG: u32 = 0x44;
pub const SM_CXEDGE: u32 = 0x2D;
pub const SM_CXFIXEDFRAME: u32 = 0x7;
pub const SM_CXFOCUSBORDER: u32 = 0x53;
pub const SM_CXFRAME: u32 = 0x20;
pub const SM_CXFULLSCREEN: u32 = 0x10;
pub const SM_CXHSCROLL: u32 = 0x15;
pub const SM_CXHTHUMB: u32 = 0xA;
pub const SM_CXICON: u32 = 0xB;
pub const SM_CXICONSPACING: u32 = 0x26;
pub const SM_CXMAXIMIZED: u32 = 0x3D;
pub const SM_CXMAXTRACK: u32 = 0x3B;
pub const SM_CXMENUCHECK: u32 = 0x47;
pub const SM_CXMENUSIZE: u32 = 0x36;
pub const SM_CXMIN: u32 = 0x1C;
pub const SM_CXMINIMIZED: u32 = 0x39;
pub const SM_CXMINSPACING: u32 = 0x2F;
pub const SM_CXMINTRACK: u32 = 0x22;
pub const SM_CXPADDEDBORDER: u32 = 0x5C;
pub const SM_CXSCREEN: u32 = 0x0;
pub const SM_CXSIZE: u32 = 0x1E;
pub const SM_CXSIZEFRAME: u32 = 0x20;
pub const SM_CXSMICON: u32 = 0x31;
pub const SM_CXSMSIZE: u32 = 0x34;
pub const SM_CXVIRTUALSCREEN: u32 = 0x4E;
pub const SM_CXVSCROLL: u32 = 0x2;
pub const SM_CYBORDER: u32 = 0x6;
pub const SM_CYCAPTION: u32 = 0x4;
pub const SM_CYCURSOR: u32 = 0xE;
pub const SM_CYDLGFRAME: u32 = 0x8;
pub const SM_CYDOUBLECLK: u32 = 0x25;
pub const SM_CYDRAG: u32 = 0x45;
pub const SM_CYEDGE: u32 = 0x2E;
pub const SM_CYFIXEDFRAME: u32 = 0x8;
pub const SM_CYFOCUSBORDER: u32 = 0x54;
pub const SM_CYFRAME: u32 = 0x21;
pub const SM_CYFULLSCREEN: u32 = 0x11;
pub const SM_CYHSCROLL: u32 = 0x3;
pub const SM_CYICON: u32 = 0xC;
pub const SM_CYICONSPACING: u32 = 0x27;
pub const SM_CYKANJIWINDOW: u32 = 0x12;
pub const SM_CYMAXIMIZED: u32 = 0x3E;
pub const SM_CYMAXTRACK: u32 = 0x3C;
pub const SM_CYMENU: u32 = 0xF;
pub const SM_CYMENUCHECK: u32 = 0x48;
pub const SM_CYMENUSIZE: u32 = 0x37;
pub const SM_CYMIN: u32 = 0x1D;
pub const SM_CYMINIMIZED: u32 = 0x3A;
pub const SM_CYMINSPACING: u32 = 0x30;
pub const SM_CYMINTRACK: u32 = 0x23;
pub const SM_CYSCREEN: u32 = 0x1;
pub const SM_CYSIZE: u32 = 0x1F;
pub const SM_CYSIZEFRAME: u32 = 0x21;
pub const SM_CYSMCAPTION: u32 = 0x33;
pub const SM_CYSMICON: u32 = 0x32;
pub const SM_CYSMSIZE: u32 = 0x35;
pub const SM_CYVIRTUALSCREEN: u32 = 0x4F;
pub const SM_CYVSCROLL: u32 = 0x14;
pub const SM_CYVTHUMB: u32 = 0x9;
pub const SM_DBCSENABLED: u32 = 0x2A;
pub const SM_DEBUG: u32 = 0x16;
pub const SM_DIGITIZER: u32 = 0x5E;
pub const SM_IMMENABLED: u32 = 0x52;
pub const SM_MAXIMUMTOUCHES: u32 = 0x5F;
pub const SM_MEDIACENTER: u32 = 0x57;
pub const SM_MENUDROPALIGNMENT: u32 = 0x28;
pub const SM_MIDEASTENABLED: u32 = 0x4A;
pub const SM_MOUSEPRESENT: u32 = 0x13;
pub const SM_MOUSEHORIZONTALWHEELPRESENT: u32 = 0x5B;
pub const SM_MOUSEWHEELPRESENT: u32 = 0x4B;
pub const SM_NETWORK: u32 = 0x3F;
pub const SM_PENWINDOWS: u32 = 0x29;
pub const SM_REMOTECONTROL: u32 = 0x2001;
pub const SM_REMOTESESSION: u32 = 0x1000;
pub const SM_SAMEDISPLAYFORMAT: u32 = 0x51;
pub const SM_SECURE: u32 = 0x2C;
pub const SM_SERVERR2: u32 = 0x59;
pub const SM_SHOWSOUNDS: u32 = 0x46;
pub const SM_SHUTTINGDOWN: u32 = 0x2000;
pub const SM_SLOWMACHINE: u32 = 0x49;
pub const SM_STARTER: u32 = 0x58;
pub const SM_SWAPBUTTON: u32 = 0x17;
pub const SM_SYSTEMDOCKED: u32 = 0x2004;
pub const SM_TABLETPC: u32 = 0x56;
pub const SM_XVIRTUALSCREEN: u32 = 0x4C;
pub const SM_YVIRTUALSCREEN: u32 = 0x4D;
pub const SYSPAL_NOSTATIC: u32 = 0x2;
pub const SYSPAL_NOSTATIC256: u32 = 0x3;
pub const SYSPAL_STATIC: u32 = 0x1;
pub const SPI_GETBEEP: u32 = 0x1;
pub const SPI_SETBEEP: u32 = 0x2;
pub const SPI_GETMOUSE: u32 = 0x3;
pub const SPI_SETMOUSE: u32 = 0x4;
pub const SPI_GETBORDER: u32 = 0x5;
pub const SPI_SETBORDER: u32 = 0x6;
pub const SPI_GETKEYBOARDSPEED: u32 = 0xA;
pub const SPI_SETKEYBOARDSPEED: u32 = 0xB;
pub const SPI_LANGDRIVER: u32 = 0xC;
pub const SPI_ICONHORIZONTALSPACING: u32 = 0xD;
pub const SPI_GETSCREENSAVETIMEOUT: u32 = 0xE;
pub const SPI_SETSCREENSAVETIMEOUT: u32 = 0xF;
pub const SPI_GETSCREENSAVEACTIVE: u32 = 0x10;
pub const SPI_SETSCREENSAVEACTIVE: u32 = 0x11;
pub const SPI_GETGRIDGRANULARITY: u32 = 0x12;
pub const SPI_SETGRIDGRANULARITY: u32 = 0x13;
pub const SPI_SETDESKWALLPAPER: u32 = 0x14;
pub const SPI_SETDESKPATTERN: u32 = 0x15;
pub const SPI_GETKEYBOARDDELAY: u32 = 0x16;
pub const SPI_SETKEYBOARDDELAY: u32 = 0x17;
pub const SPI_ICONVERTICALSPACING: u32 = 0x18;
pub const SPI_GETICONTITLEWRAP: u32 = 0x19;
pub const SPI_SETICONTITLEWRAP: u32 = 0x1A;
pub const SPI_GETMENUDROPALIGNMENT: u32 = 0x1B;
pub const SPI_SETMENUDROPALIGNMENT: u32 = 0x1C;
pub const SPI_SETDOUBLECLKWIDTH: u32 = 0x1D;
pub const SPI_SETDOUBLECLKHEIGHT: u32 = 0x1E;
pub const SPI_GETICONTITLELOGFONT: u32 = 0x1F;
pub const SPI_SETDOUBLECLICKTIME: u32 = 0x20;
pub const SPI_SETMOUSEBUTTONSWAP: u32 = 0x21;
pub const SPI_SETICONTITLELOGFONT: u32 = 0x22;
pub const SPI_GETFASTTASKSWITCH: u32 = 0x23;
pub const SPI_SETFASTTASKSWITCH: u32 = 0x24;
pub const SPI_SETDRAGFULLWINDOWS: u32 = 0x25;
pub const SPI_GETDRAGFULLWINDOWS: u32 = 0x26;
pub const SPI_GETNONCLIENTMETRICS: u32 = 0x29;
pub const SPI_SETNONCLIENTMETRICS: u32 = 0x2A;
pub const SPI_GETMINIMIZEDMETRICS: u32 = 0x2B;
pub const SPI_SETMINIMIZEDMETRICS: u32 = 0x2C;
pub const SPI_GETICONMETRICS: u32 = 0x2D;
pub const SPI_SETICONMETRICS: u32 = 0x2E;
pub const SPI_SETWORKAREA: u32 = 0x2F;
pub const SPI_GETWORKAREA: u32 = 0x30;
pub const SPI_SETPENWINDOWS: u32 = 0x31;
pub const SPI_GETHIGHCONTRAST: u32 = 0x42;
pub const SPI_SETHIGHCONTRAST: u32 = 0x43;
pub const SPI_GETKEYBOARDPREF: u32 = 0x44;
pub const SPI_SETKEYBOARDPREF: u32 = 0x45;
pub const SPI_GETSCREENREADER: u32 = 0x46;
pub const SPI_SETSCREENREADER: u32 = 0x47;
pub const SPI_GETANIMATION: u32 = 0x48;
pub const SPI_SETANIMATION: u32 = 0x49;
pub const SPI_GETFONTSMOOTHING: u32 = 0x4A;
pub const SPI_SETFONTSMOOTHING: u32 = 0x4B;
pub const SPI_SETDRAGWIDTH: u32 = 0x4C;
pub const SPI_SETDRAGHEIGHT: u32 = 0x4D;
pub const SPI_SETHANDHELD: u32 = 0x4E;
pub const SPI_GETLOWPOWERTIMEOUT: u32 = 0x4F;
pub const SPI_GETPOWEROFFTIMEOUT: u32 = 0x50;
pub const SPI_SETLOWPOWERTIMEOUT: u32 = 0x51;
pub const SPI_SETPOWEROFFTIMEOUT: u32 = 0x52;
pub const SPI_GETLOWPOWERACTIVE: u32 = 0x53;
pub const SPI_GETPOWEROFFACTIVE: u32 = 0x54;
pub const SPI_SETLOWPOWERACTIVE: u32 = 0x55;
pub const SPI_SETPOWEROFFACTIVE: u32 = 0x56;
pub const SPI_SETCURSORS: u32 = 0x57;
pub const SPI_SETICONS: u32 = 0x58;
pub const SPI_GETDEFAULTINPUTLANG: u32 = 0x59;
pub const SPI_SETDEFAULTINPUTLANG: u32 = 0x5A;
pub const SPI_SETLANGTOGGLE: u32 = 0x5B;
pub const SPI_GETWINDOWSEXTENSION: u32 = 0x5C;
pub const SPI_SETMOUSETRAILS: u32 = 0x5D;
pub const SPI_GETMOUSETRAILS: u32 = 0x5E;
pub const SPI_SETSCREENSAVERRUNNING: u32 = 0x61;
pub const SPI_SCREENSAVERRUNNING: u32 = 0x61;
pub const SPI_GETFILTERKEYS: u32 = 0x32;
pub const SPI_SETFILTERKEYS: u32 = 0x33;
pub const SPI_GETTOGGLEKEYS: u32 = 0x34;
pub const SPI_SETTOGGLEKEYS: u32 = 0x35;
pub const SPI_GETMOUSEKEYS: u32 = 0x36;
pub const SPI_SETMOUSEKEYS: u32 = 0x37;
pub const SPI_GETSHOWSOUNDS: u32 = 0x38;
pub const SPI_SETSHOWSOUNDS: u32 = 0x39;
pub const SPI_GETSTICKYKEYS: u32 = 0x3A;
pub const SPI_SETSTICKYKEYS: u32 = 0x3B;
pub const SPI_GETACCESSTIMEOUT: u32 = 0x3C;
pub const SPI_SETACCESSTIMEOUT: u32 = 0x3D;
pub const SPI_GETSERIALKEYS: u32 = 0x3E;
pub const SPI_SETSERIALKEYS: u32 = 0x3F;
pub const SPI_GETSOUNDSENTRY: u32 = 0x40;
pub const SPI_SETSOUNDSENTRY: u32 = 0x41;
pub const SPI_GETSNAPTODEFBUTTON: u32 = 0x5F;
pub const SPI_SETSNAPTODEFBUTTON: u32 = 0x60;
pub const SPI_GETMOUSEHOVERWIDTH: u32 = 0x62;
pub const SPI_SETMOUSEHOVERWIDTH: u32 = 0x63;
pub const SPI_GETMOUSEHOVERHEIGHT: u32 = 0x64;
pub const SPI_SETMOUSEHOVERHEIGHT: u32 = 0x65;
pub const SPI_GETMOUSEHOVERTIME: u32 = 0x66;
pub const SPI_SETMOUSEHOVERTIME: u32 = 0x67;
pub const SPI_GETWHEELSCROLLLINES: u32 = 0x68;
pub const SPI_SETWHEELSCROLLLINES: u32 = 0x69;
pub const SPI_GETMENUSHOWDELAY: u32 = 0x6A;
pub const SPI_SETMENUSHOWDELAY: u32 = 0x6B;
pub const SPI_GETWHEELSCROLLCHARS: u32 = 0x6C;
pub const SPI_SETWHEELSCROLLCHARS: u32 = 0x6D;
pub const SPI_GETSHOWIMEUI: u32 = 0x6E;
pub const SPI_SETSHOWIMEUI: u32 = 0x6F;
pub const SPI_GETMOUSESPEED: u32 = 0x70;
pub const SPI_SETMOUSESPEED: u32 = 0x71;
pub const SPI_GETSCREENSAVERRUNNING: u32 = 0x72;
pub const SPI_GETDESKWALLPAPER: u32 = 0x73;
pub const SPI_GETAUDIODESCRIPTION: u32 = 0x74;
pub const SPI_SETAUDIODESCRIPTION: u32 = 0x75;
pub const SPI_GETSCREENSAVESECURE: u32 = 0x76;
pub const SPI_SETSCREENSAVESECURE: u32 = 0x77;
pub const SPI_GETHUNGAPPTIMEOUT: u32 = 0x78;
pub const SPI_SETHUNGAPPTIMEOUT: u32 = 0x79;
pub const SPI_GETWAITTOKILLTIMEOUT: u32 = 0x7A;
pub const SPI_SETWAITTOKILLTIMEOUT: u32 = 0x7B;
pub const SPI_GETWAITTOKILLSERVICETIMEOUT: u32 = 0x7C;
pub const SPI_SETWAITTOKILLSERVICETIMEOUT: u32 = 0x7D;
pub const SPI_GETMOUSEDOCKTHRESHOLD: u32 = 0x7E;
pub const SPI_SETMOUSEDOCKTHRESHOLD: u32 = 0x7F;
pub const SPI_GETPENDOCKTHRESHOLD: u32 = 0x80;
pub const SPI_SETPENDOCKTHRESHOLD: u32 = 0x81;
pub const SPI_GETWINARRANGING: u32 = 0x82;
pub const SPI_SETWINARRANGING: u32 = 0x83;
pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: u32 = 0x84;
pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: u32 = 0x85;
pub const SPI_GETPENDRAGOUTTHRESHOLD: u32 = 0x86;
pub const SPI_SETPENDRAGOUTTHRESHOLD: u32 = 0x87;
pub const SPI_GETMOUSESIDEMOVETHRESHOLD: u32 = 0x88;
pub const SPI_SETMOUSESIDEMOVETHRESHOLD: u32 = 0x89;
pub const SPI_GETPENSIDEMOVETHRESHOLD: u32 = 0x8A;
pub const SPI_SETPENSIDEMOVETHRESHOLD: u32 = 0x8B;
pub const SPI_GETDRAGFROMMAXIMIZE: u32 = 0x8C;
pub const SPI_SETDRAGFROMMAXIMIZE: u32 = 0x8D;
pub const SPI_GETSNAPSIZING: u32 = 0x8E;
pub const SPI_SETSNAPSIZING: u32 = 0x8F;
pub const SPI_GETDOCKMOVING: u32 = 0x90;
pub const SPI_SETDOCKMOVING: u32 = 0x91;
pub const SPI_GETTOUCHPREDICTIONPARAMETERS: u32 = 0x9C;
pub const SPI_SETTOUCHPREDICTIONPARAMETERS: u32 = 0x9D;
pub const SPI_GETLOGICALDPIOVERRIDE: u32 = 0x9E;
pub const SPI_SETLOGICALDPIOVERRIDE: u32 = 0x9F;
pub const SPI_GETMENURECT: u32 = 0xA2;
pub const SPI_SETMENURECT: u32 = 0xA3;
pub const SPI_GETACTIVEWINDOWTRACKING: u32 = 0x1000;
pub const SPI_SETACTIVEWINDOWTRACKING: u32 = 0x1001;
pub const SPI_GETMENUANIMATION: u32 = 0x1002;
pub const SPI_SETMENUANIMATION: u32 = 0x1003;
pub const SPI_GETCOMBOBOXANIMATION: u32 = 0x1004;
pub const SPI_SETCOMBOBOXANIMATION: u32 = 0x1005;
pub const SPI_GETLISTBOXSMOOTHSCROLLING: u32 = 0x1006;
pub const SPI_SETLISTBOXSMOOTHSCROLLING: u32 = 0x1007;
pub const SPI_GETGRADIENTCAPTIONS: u32 = 0x1008;
pub const SPI_SETGRADIENTCAPTIONS: u32 = 0x1009;
pub const SPI_GETKEYBOARDCUES: u32 = 0x100A;
pub const SPI_SETKEYBOARDCUES: u32 = 0x100B;
pub const SPI_GETMENUUNDERLINES: u32 = 0x100A;
pub const SPI_SETMENUUNDERLINES: u32 = 0x100B;
pub const SPI_GETACTIVEWNDTRKZORDER: u32 = 0x100C;
pub const SPI_SETACTIVEWNDTRKZORDER: u32 = 0x100D;
pub const SPI_GETHOTTRACKING: u32 = 0x100E;
pub const SPI_SETHOTTRACKING: u32 = 0x100F;
pub const SPI_GETMENUFADE: u32 = 0x1012;
pub const SPI_SETMENUFADE: u32 = 0x1013;
pub const SPI_GETSELECTIONFADE: u32 = 0x1014;
pub const SPI_SETSELECTIONFADE: u32 = 0x1015;
pub const SPI_GETTOOLTIPANIMATION: u32 = 0x1016;
pub const SPI_SETTOOLTIPANIMATION: u32 = 0x1017;
pub const SPI_GETTOOLTIPFADE: u32 = 0x1018;
pub const SPI_SETTOOLTIPFADE: u32 = 0x1019;
pub const SPI_GETCURSORSHADOW: u32 = 0x101A;
pub const SPI_SETCURSORSHADOW: u32 = 0x101B;
pub const SPI_GETMOUSESONAR: u32 = 0x101C;
pub const SPI_SETMOUSESONAR: u32 = 0x101D;
pub const SPI_GETMOUSECLICKLOCK: u32 = 0x101E;
pub const SPI_SETMOUSECLICKLOCK: u32 = 0x101F;
pub const SPI_GETMOUSEVANISH: u32 = 0x1020;
pub const SPI_SETMOUSEVANISH: u32 = 0x1021;
pub const SPI_GETFLATMENU: u32 = 0x1022;
pub const SPI_SETFLATMENU: u32 = 0x1023;
pub const SPI_GETDROPSHADOW: u32 = 0x1024;
pub const SPI_SETDROPSHADOW: u32 = 0x1025;
pub const SPI_GETBLOCKSENDINPUTRESETS: u32 = 0x1026;
pub const SPI_SETBLOCKSENDINPUTRESETS: u32 = 0x1027;
pub const SPI_GETUIEFFECTS: u32 = 0x103E;
pub const SPI_SETUIEFFECTS: u32 = 0x103F;
pub const SPI_GETDISABLEOVERLAPPEDCONTENT: u32 = 0x1040;
pub const SPI_SETDISABLEOVERLAPPEDCONTENT: u32 = 0x1041;
pub const SPI_GETCLIENTAREAANIMATION: u32 = 0x1042;
pub const SPI_SETCLIENTAREAANIMATION: u32 = 0x1043;
pub const SPI_GETCLEARTYPE: u32 = 0x1048;
pub const SPI_SETCLEARTYPE: u32 = 0x1049;
pub const SPI_GETSPEECHRECOGNITION: u32 = 0x104A;
pub const SPI_SETSPEECHRECOGNITION: u32 = 0x104B;
pub const SPI_GETCARETBROWSING: u32 = 0x104C;
pub const SPI_SETCARETBROWSING: u32 = 0x104D;
pub const SPI_GETTHREADLOCALINPUTSETTINGS: u32 = 0x104E;
pub const SPI_SETTHREADLOCALINPUTSETTINGS: u32 = 0x104F;
pub const SPI_GETSYSTEMLANGUAGEBAR: u32 = 0x1050;
pub const SPI_SETSYSTEMLANGUAGEBAR: u32 = 0x1051;
pub const SPI_GETFOREGROUNDLOCKTIMEOUT: u32 = 0x2000;
pub const SPI_SETFOREGROUNDLOCKTIMEOUT: u32 = 0x2001;
pub const SPI_GETACTIVEWNDTRKTIMEOUT: u32 = 0x2002;
pub const SPI_SETACTIVEWNDTRKTIMEOUT: u32 = 0x2003;
pub const SPI_GETFOREGROUNDFLASHCOUNT: u32 = 0x2004;
pub const SPI_SETFOREGROUNDFLASHCOUNT: u32 = 0x2005;
pub const SPI_GETCARETWIDTH: u32 = 0x2006;
pub const SPI_SETCARETWIDTH: u32 = 0x2007;
pub const SPI_GETMOUSECLICKLOCKTIME: u32 = 0x2008;
pub const SPI_SETMOUSECLICKLOCKTIME: u32 = 0x2009;
pub const SPI_GETFONTSMOOTHINGTYPE: u32 = 0x200A;
pub const SPI_SETFONTSMOOTHINGTYPE: u32 = 0x200B;
pub const SPI_GETFONTSMOOTHINGCONTRAST: u32 = 0x200C;
pub const SPI_SETFONTSMOOTHINGCONTRAST: u32 = 0x200D;
pub const SPI_GETFOCUSBORDERWIDTH: u32 = 0x200E;
pub const SPI_SETFOCUSBORDERWIDTH: u32 = 0x200F;
pub const SPI_GETFOCUSBORDERHEIGHT: u32 = 0x2010;
pub const SPI_SETFOCUSBORDERHEIGHT: u32 = 0x2011;
pub const SPI_GETFONTSMOOTHINGORIENTATION: u32 = 0x2012;
pub const SPI_SETFONTSMOOTHINGORIENTATION: u32 = 0x2013;
pub const SPI_GETMINIMUMHITRADIUS: u32 = 0x2014;
pub const SPI_SETMINIMUMHITRADIUS: u32 = 0x2015;
pub const SPI_GETMESSAGEDURATION: u32 = 0x2016;
pub const SPI_SETMESSAGEDURATION: u32 = 0x2017;
pub const SPI_GETCONTACTVISUALIZATION: u32 = 0x2018;
pub const SPI_SETCONTACTVISUALIZATION: u32 = 0x2019;
pub const SPI_GETGESTUREVISUALIZATION: u32 = 0x201A;
pub const SPI_SETGESTUREVISUALIZATION: u32 = 0x201B;
pub const SPI_GETMOUSEWHEELROUTING: u32 = 0x201C;
pub const SPI_SETMOUSEWHEELROUTING: u32 = 0x201D;
pub const SPI_GETPENVISUALIZATION: u32 = 0x201E;
pub const SPI_SETPENVISUALIZATION: u32 = 0x201F;
pub const SPI_GETPENARBITRATIONTYPE: u32 = 0x2020;
pub const SPI_SETPENARBITRATIONTYPE: u32 = 0x2021;
pub const SPI_GETCARETTIMEOUT: u32 = 0x2022;
pub const SPI_SETCARETTIMEOUT: u32 = 0x2023;
pub const SPI_GETHANDEDNESS: u32 = 0x2024;
pub const SPI_SETHANDEDNESS: u32 = 0x2025;
pub const SPIF_UPDATEINIFILE: u32 = 0x1;
pub const SPIF_SENDCHANGE: u32 = 0x2;
pub const SPIF_SENDWININICHANGE: u32 = 0x2;
pub const COLOR_3DDKSHADOW: u32 = 0x15;
pub const COLOR_3DFACE: u32 = 0xF;
pub const COLOR_3DHIGHLIGHT: u32 = 0x14;
pub const COLOR_3DHILIGHT: u32 = 0x14;
pub const COLOR_3DLIGHT: u32 = 0x16;
pub const COLOR_3DSHADOW: u32 = 0x10;
pub const COLOR_ACTIVEBORDER: u32 = 0xA;
pub const COLOR_ACTIVECAPTION: u32 = 0x2;
pub const COLOR_APPWORKSPACE: u32 = 0xC;
pub const COLOR_BACKGROUND: u32 = 0x1;
pub const COLOR_BTNFACE: u32 = 0xF;
pub const COLOR_BTNHIGHLIGHT: u32 = 0x14;
pub const COLOR_BTNHILIGHT: u32 = 0x14;
pub const COLOR_BTNSHADOW: u32 = 0x10;
pub const COLOR_BTNTEXT: u32 = 0x12;
pub const COLOR_CAPTIONTEXT: u32 = 0x9;
pub const COLOR_DESKTOP: u32 = 0x1;
pub const COLOR_GRADIENTACTIVECAPTION: u32 = 0x1B;
pub const COLOR_GRADIENTINACTIVECAPTION: u32 = 0x1C;
pub const COLOR_GRAYTEXT: u32 = 0x11;
pub const COLOR_HIGHLIGHT: u32 = 0xD;
pub const COLOR_HIGHLIGHTTEXT: u32 = 0xE;
pub const COLOR_HOTLIGHT: u32 = 0x1A;
pub const COLOR_INACTIVEBORDER: u32 = 0xB;
pub const COLOR_INACTIVECAPTION: u32 = 0x3;
pub const COLOR_INACTIVECAPTIONTEXT: u32 = 0x13;
pub const COLOR_INFOBK: u32 = 0x18;
pub const COLOR_INFOTEXT: u32 = 0x17;
pub const COLOR_MENU: u32 = 0x4;
pub const COLOR_MENUHILIGHT: u32 = 0x1D;
pub const COLOR_MENUBAR: u32 = 0x1E;
pub const COLOR_MENUTEXT: u32 = 0x7;
pub const COLOR_SCROLLBAR: u32 = 0x0;
pub const COLOR_WINDOW: u32 = 0x5;
pub const COLOR_WINDOWFRAME: u32 = 0x6;
pub const COLOR_WINDOWTEXT: u32 = 0x8;
pub struct ScheduledTasks {
}
pub struct SearchFolderItemFactory {
}
pub const SECURE_LOCK_ICON_UNSECURE: i32 = 0x0;
pub const SECURE_LOCK_ICON_MIXED: i32 = 0x1;
pub const SECURE_LOCK_ICON_SECURE_UNKNOWN_BITS: i32 = 0x2;
pub const SECURE_LOCK_ICON_SECURE40_BIT: i32 = 0x3;
pub const SECURE_LOCK_ICON_SECURE56_BIT: i32 = 0x4;
pub const SECURE_LOCK_ICON_SECURE_FORTEZZA: i32 = 0x5;
pub const SECURE_LOCK_ICON_SECURE128_BIT: i32 = 0x6;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShFindChangeNotificationHandle {
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}

impl ShFindChangeNotificationHandle {
    pub const fn null() -> Self {
        // SAFETY: null is always a valid pointer
        unsafe { Self::new(0) }
    }

    pub const unsafe fn new(handle: isize) -> Self {
        Self {
            handle,
            _thread_unsafe: PhantomData,
        }
    }

    pub fn into_raw(self) -> isize {
        self.handle
    }
}

impl fmt::Debug for ShFindChangeNotificationHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.handle, f)
    }
}
                    
pub struct SharedBitmap {
}
pub struct SharingConfigurationManager {
}
pub struct Shell {
}
pub struct ShellBrowserWindow {
}
pub struct ShellDesktop {
}
pub struct ShellDispatchInproc {
}
pub struct ShellFsFolder {
}
pub struct ShellFolderItem {
}
pub struct ShellFolderView {
}
pub struct ShellFolderViewOc {
}
pub const SFVVO_SHOWALLOBJECTS: i32 = 0x1;
pub const SFVVO_SHOWEXTENSIONS: i32 = 0x2;
pub const SFVVO_SHOWCOMPCOLOR: i32 = 0x8;
pub const SFVVO_SHOWSYSFILES: i32 = 0x20;
pub const SFVVO_WIN95CLASSIC: i32 = 0x40;
pub const SFVVO_DOUBLECLICKINWEBVIEW: i32 = 0x80;
pub const SFVVO_DESKTOPHTML: i32 = 0x200;
pub struct ShellImageDataFactory {
}
pub struct ShellItem {
}
pub struct ShellLibrary {
}
pub struct ShellLink {
}
pub struct ShellLinkObject {
}
pub struct ShellNameSpace {
}
pub const SSF_DESKTOP: i32 = 0x0;
pub const SSF_PROGRAMS: i32 = 0x2;
pub const SSF_CONTROLS: i32 = 0x3;
pub const SSF_PRINTERS: i32 = 0x4;
pub const SSF_PERSONAL: i32 = 0x5;
pub const SSF_FAVORITES: i32 = 0x6;
pub const SSF_STARTUP: i32 = 0x7;
pub const SSF_RECENT: i32 = 0x8;
pub const SSF_SENDTO: i32 = 0x9;
pub const SSF_BITBUCKET: i32 = 0xA;
pub const SSF_STARTMENU: i32 = 0xB;
pub const SSF_DESKTOPDIRECTORY: i32 = 0x10;
pub const SSF_DRIVES: i32 = 0x11;
pub const SSF_NETWORK: i32 = 0x12;
pub const SSF_NETHOOD: i32 = 0x13;
pub const SSF_FONTS: i32 = 0x14;
pub const SSF_TEMPLATES: i32 = 0x15;
pub const SSF_COMMONSTARTMENU: i32 = 0x16;
pub const SSF_COMMONPROGRAMS: i32 = 0x17;
pub const SSF_COMMONSTARTUP: i32 = 0x18;
pub const SSF_COMMONDESKTOPDIR: i32 = 0x19;
pub const SSF_APPDATA: i32 = 0x1A;
pub const SSF_PRINTHOOD: i32 = 0x1B;
pub const SSF_LOCALAPPDATA: i32 = 0x1C;
pub const SSF_ALTSTARTUP: i32 = 0x1D;
pub const SSF_COMMONALTSTARTUP: i32 = 0x1E;
pub const SSF_COMMONFAVORITES: i32 = 0x1F;
pub const SSF_INTERNETCACHE: i32 = 0x20;
pub const SSF_COOKIES: i32 = 0x21;
pub const SSF_HISTORY: i32 = 0x22;
pub const SSF_COMMONAPPDATA: i32 = 0x23;
pub const SSF_WINDOWS: i32 = 0x24;
pub const SSF_SYSTEM: i32 = 0x25;
pub const SSF_PROGRAMFILES: i32 = 0x26;
pub const SSF_MYPICTURES: i32 = 0x27;
pub const SSF_PROFILE: i32 = 0x28;
pub const SSF_SYSTE_MX86: i32 = 0x29;
pub const SSF_PROGRAMFILE_SX86: i32 = 0x30;
pub struct ShellUiHelper {
}
pub const SWFO_NEEDDISPATCH: i32 = 0x1;
pub const SWFO_INCLUDEPENDING: i32 = 0x2;
pub const SWFO_COOKIEPASSED: i32 = 0x4;
pub const SWC_EXPLORER: i32 = 0x0;
pub const SWC_BROWSER: i32 = 0x1;
pub const SWC_3RDPARTY: i32 = 0x2;
pub const SWC_CALLBACK: i32 = 0x4;
pub const SWC_DESKTOP: i32 = 0x8;
pub struct ShellWindows {
}
pub struct ShowInputPaneAnimationCoordinator {
}
pub struct SimpleConflictPresenter {
}
pub struct SizeCategorizer {
}
pub struct SmartcardCredentialProvider {
}
pub struct SmartcardPinProvider {
}
pub struct SmartcardReaderSelectionProvider {
}
pub struct SmartcardWinRtProvider {
}
pub struct StartMenuPin {
}
pub struct StorageProviderBanners {
}
pub struct SuspensionDependencyManager {
}
pub struct SyncMgr {
}
pub struct SyncMgrClient {
}
pub struct SyncMgrControl {
}
pub struct SyncMgrFolder {
}
pub struct SyncMgrScheduleWizard {
}
pub struct SyncResultsFolder {
}
pub struct SyncSetupFolder {
}
pub const TDP_GROUPCOUNT: i32 = 0x1;
pub const TDP_FLASHBUTTON: i32 = 0x2;
pub const TDP_FLASHBUTTONGROUPMENU: i32 = 0x3;
pub const TBP_BACKGROUNDBOTTOM: i32 = 0x1;
pub const TBP_BACKGROUNDRIGHT: i32 = 0x2;
pub const TBP_BACKGROUNDTOP: i32 = 0x3;
pub const TBP_BACKGROUNDLEFT: i32 = 0x4;
pub const TBP_SIZINGBARBOTTOM: i32 = 0x5;
pub const TBP_SIZINGBARRIGHT: i32 = 0x6;
pub const TBP_SIZINGBARTOP: i32 = 0x7;
pub const TBP_SIZINGBARLEFT: i32 = 0x8;
pub struct Taskdialogconfig<'a> {
    pub cbSize: u32,
    pub hwndParent: HWND,
    pub hInstance: HINSTANCE,
    pub dwFlags: i32,
    pub dwCommonButtons: i32,
    pub pszWindowTitle: &'a Cow<'a, OsStr>,
    pub Anonymous1: TASKDIALOGCONFIG_40<'a>,
    pub pszMainInstruction: &'a Cow<'a, OsStr>,
    pub pszContent: &'a Cow<'a, OsStr>,
    pub cButtons: u32,
    pub pButtons: &'a TASKDIALOG_BUTTON<'a>,
    pub nDefaultButton: i32,
    pub cRadioButtons: u32,
    pub pRadioButtons: &'a TASKDIALOG_BUTTON<'a>,
    pub nDefaultRadioButton: i32,
    pub pszVerificationText: &'a Cow<'a, OsStr>,
    pub pszExpandedInformation: &'a Cow<'a, OsStr>,
    pub pszExpandedControlText: &'a Cow<'a, OsStr>,
    pub pszCollapsedControlText: &'a Cow<'a, OsStr>,
    pub Anonymous2: HICON_41<'a>,
    pub pszFooter: &'a Cow<'a, OsStr>,
    pub pfCallback: todo_fn,
    pub lpCallbackData: isize,
    pub cxWidth: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Taskdialogconfig40<'a> {
    pub field0: HICON,
    pub field1: &'a Cow<'a, OsStr>,
}
pub struct Hicon {
    pub Value: isize,
}
pub struct TaskdialogButton<'a> {
    pub nButtonID: i32,
    pub pszButtonText: &'a Cow<'a, OsStr>,
}
pub const TDCBF_OK_BUTTON: i32 = 0x1;
pub const TDCBF_YES_BUTTON: i32 = 0x2;
pub const TDCBF_NO_BUTTON: i32 = 0x4;
pub const TDCBF_CANCEL_BUTTON: i32 = 0x8;
pub const TDCBF_RETRY_BUTTON: i32 = 0x10;
pub const TDCBF_CLOSE_BUTTON: i32 = 0x20;
pub const TDE_CONTENT: i32 = 0x0;
pub const TDE_EXPANDED_INFORMATION: i32 = 0x1;
pub const TDE_FOOTER: i32 = 0x2;
pub const TDE_MAIN_INSTRUCTION: i32 = 0x3;
pub const TDF_ENABLE_HYPERLINKS: i32 = 0x1;
pub const TDF_USE_HICON_MAIN: i32 = 0x2;
pub const TDF_USE_HICON_FOOTER: i32 = 0x4;
pub const TDF_ALLOW_DIALOG_CANCELLATION: i32 = 0x8;
pub const TDF_USE_COMMAND_LINKS: i32 = 0x10;
pub const TDF_USE_COMMAND_LINKS_NO_ICON: i32 = 0x20;
pub const TDF_EXPAND_FOOTER_AREA: i32 = 0x40;
pub const TDF_EXPANDED_BY_DEFAULT: i32 = 0x80;
pub const TDF_VERIFICATION_FLAG_CHECKED: i32 = 0x100;
pub const TDF_SHOW_PROGRESS_BAR: i32 = 0x200;
pub const TDF_SHOW_MARQUEE_PROGRESS_BAR: i32 = 0x400;
pub const TDF_CALLBACK_TIMER: i32 = 0x800;
pub const TDF_POSITION_RELATIVE_TO_WINDOW: i32 = 0x1000;
pub const TDF_RTL_LAYOUT: i32 = 0x2000;
pub const TDF_NO_DEFAULT_RADIO_BUTTON: i32 = 0x4000;
pub const TDF_CAN_BE_MINIMIZED: i32 = 0x8000;
pub const TDF_NO_SET_FOREGROUND: i32 = 0x10000;
pub const TDF_SIZE_TO_CONTENT: i32 = 0x1000000;
pub const TDIE_ICON_MAIN: i32 = 0x0;
pub const TDIE_ICON_FOOTER: i32 = 0x1;
pub const TDM_NAVIGATE_PAGE: i32 = 0x465;
pub const TDM_CLICK_BUTTON: i32 = 0x466;
pub const TDM_SET_MARQUEE_PROGRESS_BAR: i32 = 0x467;
pub const TDM_SET_PROGRESS_BAR_STATE: i32 = 0x468;
pub const TDM_SET_PROGRESS_BAR_RANGE: i32 = 0x469;
pub const TDM_SET_PROGRESS_BAR_POS: i32 = 0x46A;
pub const TDM_SET_PROGRESS_BAR_MARQUEE: i32 = 0x46B;
pub const TDM_SET_ELEMENT_TEXT: i32 = 0x46C;
pub const TDM_CLICK_RADIO_BUTTON: i32 = 0x46E;
pub const TDM_ENABLE_BUTTON: i32 = 0x46F;
pub const TDM_ENABLE_RADIO_BUTTON: i32 = 0x470;
pub const TDM_CLICK_VERIFICATION: i32 = 0x471;
pub const TDM_UPDATE_ELEMENT_TEXT: i32 = 0x472;
pub const TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE: i32 = 0x473;
pub const TDM_UPDATE_ICON: i32 = 0x474;
pub const TDN_CREATED: i32 = 0x0;
pub const TDN_NAVIGATED: i32 = 0x1;
pub const TDN_BUTTON_CLICKED: i32 = 0x2;
pub const TDN_HYPERLINK_CLICKED: i32 = 0x3;
pub const TDN_TIMER: i32 = 0x4;
pub const TDN_DESTROYED: i32 = 0x5;
pub const TDN_RADIO_BUTTON_CLICKED: i32 = 0x6;
pub const TDN_DIALOG_CONSTRUCTED: i32 = 0x7;
pub const TDN_VERIFICATION_CLICKED: i32 = 0x8;
pub const TDN_HELP: i32 = 0x9;
pub const TDN_EXPANDO_BUTTON_CLICKED: i32 = 0xA;
pub struct TaCubicBezier {
    pub header: TA_TIMINGFUNCTION,
    pub rX0: f32,
    pub rY0: f32,
    pub rX1: f32,
    pub rY1: f32,
}
pub const TAP_FLAGS: i32 = 0x0;
pub const TAP_TRANSFORMCOUNT: i32 = 0x1;
pub const TAP_STAGGERDELAY: i32 = 0x2;
pub const TAP_STAGGERDELAYCAP: i32 = 0x3;
pub const TAP_STAGGERDELAYFACTOR: i32 = 0x4;
pub const TAP_ZORDER: i32 = 0x5;
pub const TAPF_NONE: u32 = 0x0;
pub const TAPF_HASSTAGGER: u32 = 0x1;
pub const TAPF_ISRTLAWARE: u32 = 0x2;
pub const TAPF_ALLOWCOLLECTION: u32 = 0x4;
pub const TAPF_HASBACKGROUND: u32 = 0x8;
pub const TAPF_HASPERSPECTIVE: u32 = 0x10;
pub struct TaTimingfunction {
    pub eTimingFunctionType: i32,
}
pub const TTFT_UNDEFINED: i32 = 0x0;
pub const TTFT_CUBIC_BEZIER: i32 = 0x1;
pub struct TaTransform {
    pub eTransformType: i32,
    pub dwTimingFunctionId: u32,
    pub dwStartTime: u32,
    pub dwDurationTime: u32,
    pub eFlags: i32,
}
pub struct TaTransform2d {
    pub header: TA_TRANSFORM,
    pub rX: f32,
    pub rY: f32,
    pub rInitialX: f32,
    pub rInitialY: f32,
    pub rOriginX: f32,
    pub rOriginY: f32,
}
pub struct TaTransformClip {
    pub header: TA_TRANSFORM,
    pub rLeft: f32,
    pub rTop: f32,
    pub rRight: f32,
    pub rBottom: f32,
    pub rInitialLeft: f32,
    pub rInitialTop: f32,
    pub rInitialRight: f32,
    pub rInitialBottom: f32,
}
pub const TATF_NONE: i32 = 0x0;
pub const TATF_TARGETVALUES_USER: i32 = 0x1;
pub const TATF_HASINITIALVALUES: i32 = 0x2;
pub const TATF_HASORIGINVALUES: i32 = 0x4;
pub struct TaTransformOpacity {
    pub header: TA_TRANSFORM,
    pub rOpacity: f32,
    pub rInitialOpacity: f32,
}
pub const TATT_TRANSLATE_2D: i32 = 0x0;
pub const TATT_SCALE_2D: i32 = 0x1;
pub const TATT_OPACITY: i32 = 0x2;
pub const TATT_CLIP: i32 = 0x3;
pub struct Tbaddbitmap {
    pub hInst: HINSTANCE,
    pub nID: usize,
}
pub struct Tbbutton {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 2],
    pub dwData: usize,
    pub iString: isize,
}
pub struct Tbbuttoninfoa<'a> {
    pub cbSize: u32,
    pub dwMask: u32,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchText: i32,
}
pub struct Tbbuttoninfow<'a> {
    pub cbSize: u32,
    pub dwMask: u32,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchText: i32,
}
pub const TBIF_BYINDEX: u32 = 0x80000000;
pub const TBIF_COMMAND: u32 = 0x20;
pub const TBIF_IMAGE: u32 = 0x1;
pub const TBIF_LPARAM: u32 = 0x10;
pub const TBIF_SIZE: u32 = 0x40;
pub const TBIF_STATE: u32 = 0x4;
pub const TBIF_STYLE: u32 = 0x8;
pub const TBIF_TEXT: u32 = 0x2;
pub struct Tbinfo {
    pub cbuttons: u32,
    pub uFlags: u32,
}
pub struct Tbinsertmark {
    pub iButton: i32,
    pub dwFlags: u32,
}
pub const TBIMHT_NONE: u32 = 0x0;
pub const TBIMHT_AFTER: u32 = 0x1;
pub const TBIMHT_BACKGROUND: u32 = 0x2;
pub struct Tbmetrics {
    pub cbSize: u32,
    pub dwMask: u32,
    pub cxPad: i32,
    pub cyPad: i32,
    pub cxBarPad: i32,
    pub cyBarPad: i32,
    pub cxButtonSpacing: i32,
    pub cyButtonSpacing: i32,
}
pub const TBPF_NOPROGRESS: i32 = 0x0;
pub const TBPF_INDETERMINATE: i32 = 0x1;
pub const TBPF_NORMAL: i32 = 0x2;
pub const TBPF_ERROR: i32 = 0x4;
pub const TBPF_PAUSED: i32 = 0x8;
pub struct Tbreplacebitmap {
    pub hInstOld: HINSTANCE,
    pub nIDOld: usize,
    pub hInstNew: HINSTANCE,
    pub nIDNew: usize,
    pub nButtons: i32,
}
pub struct Tbsaveparamsa<'a> {
    pub hkr: HKEY,
    pub pszSubKey: &'a Cow<'a, str>,
    pub pszValueName: &'a Cow<'a, str>,
}
pub struct Tbsaveparamsw<'a> {
    pub hkr: HKEY,
    pub pszSubKey: &'a Cow<'a, OsStr>,
    pub pszValueName: &'a Cow<'a, OsStr>,
}
pub struct Tchittestinfo {
    pub pt: POINT,
    pub flags: u32,
}
pub const TCHT_NOWHERE: u32 = 0x1;
pub const TCHT_ONITEM: u32 = 0x6;
pub const TCHT_ONITEMICON: u32 = 0x2;
pub const TCHT_ONITEMLABEL: u32 = 0x4;
pub struct Tcitema<'a> {
    pub mask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: LPARAM,
}
pub struct Tcitemheadera<'a> {
    pub mask: u32,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iImage: i32,
}
pub const TCIF_IMAGE: u32 = 0x2;
pub const TCIF_RTLREADING: u32 = 0x4;
pub const TCIF_TEXT: u32 = 0x1;
pub const TCIF_PARAM: u32 = 0x8;
pub const TCIF_STATE: u32 = 0x10;
pub struct Tcitemheaderw<'a> {
    pub mask: u32,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iImage: i32,
}
pub struct Tcitemw<'a> {
    pub mask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: LPARAM,
}
pub struct Textmetrica {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
}
pub struct Textmetricw {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
}
pub const TST_NONE: i32 = 0x0;
pub const TST_SINGLE: i32 = 0x1;
pub const TST_CONTINUOUS: i32 = 0x2;
pub const TA_NOUPDATECP: u32 = 0x0;
pub const TA_UPDATECP: u32 = 0x1;
pub const TA_LEFT: u32 = 0x0;
pub const TA_RIGHT: u32 = 0x2;
pub const TA_CENTER: u32 = 0x6;
pub const TA_TOP: u32 = 0x0;
pub const TA_BOTTOM: u32 = 0x8;
pub const TA_BASELINE: u32 = 0x18;
pub const TA_RTLREADING: u32 = 0x100;
pub const TA_MASK: u32 = 0x11F;
pub const VTA_BASELINE: u32 = 0x18;
pub const VTA_LEFT: u32 = 0x8;
pub const VTA_RIGHT: u32 = 0x0;
pub const VTA_CENTER: u32 = 0x6;
pub const VTA_BOTTOM: u32 = 0x2;
pub const VTA_TOP: u32 = 0x0;
pub const TS_MIN: i32 = 0x0;
pub const TS_TRUE: i32 = 0x1;
pub const TS_DRAW: i32 = 0x2;
pub const TMT_RESERVEDLOW: u32 = 0x0;
pub const TMT_RESERVEDHIGH: u32 = 0x1F3F;
pub const TMT_DIBDATA: u32 = 0x2;
pub const TMT_GLYPHDIBDATA: u32 = 0x8;
pub const TMT_ENUM: u32 = 0xC8;
pub const TMT_STRING: u32 = 0xC9;
pub const TMT_INT: u32 = 0xCA;
pub const TMT_BOOL: u32 = 0xCB;
pub const TMT_COLOR: u32 = 0xCC;
pub const TMT_MARGINS: u32 = 0xCD;
pub const TMT_FILENAME: u32 = 0xCE;
pub const TMT_SIZE: u32 = 0xCF;
pub const TMT_POSITION: u32 = 0xD0;
pub const TMT_RECT: u32 = 0xD1;
pub const TMT_FONT: u32 = 0xD2;
pub const TMT_INTLIST: u32 = 0xD3;
pub const TMT_HBITMAP: u32 = 0xD4;
pub const TMT_DISKSTREAM: u32 = 0xD5;
pub const TMT_STREAM: u32 = 0xD6;
pub const TMT_BITMAPREF: u32 = 0xD7;
pub const TMT_FLOAT: u32 = 0xD8;
pub const TMT_FLOATLIST: u32 = 0xD9;
pub const TMT_COLORSCHEMES: u32 = 0x191;
pub const TMT_SIZES: u32 = 0x192;
pub const TMT_CHARSET: u32 = 0x193;
pub const TMT_NAME: u32 = 0x258;
pub const TMT_DISPLAYNAME: u32 = 0x259;
pub const TMT_TOOLTIP: u32 = 0x25A;
pub const TMT_COMPANY: u32 = 0x25B;
pub const TMT_AUTHOR: u32 = 0x25C;
pub const TMT_COPYRIGHT: u32 = 0x25D;
pub const TMT_URL: u32 = 0x25E;
pub const TMT_VERSION: u32 = 0x25F;
pub const TMT_DESCRIPTION: u32 = 0x260;
pub const TMT_FIRST_RCSTRING_NAME: u32 = 0x259;
pub const TMT_LAST_RCSTRING_NAME: u32 = 0x260;
pub const TMT_CAPTIONFONT: u32 = 0x321;
pub const TMT_SMALLCAPTIONFONT: u32 = 0x322;
pub const TMT_MENUFONT: u32 = 0x323;
pub const TMT_STATUSFONT: u32 = 0x324;
pub const TMT_MSGBOXFONT: u32 = 0x325;
pub const TMT_ICONTITLEFONT: u32 = 0x326;
pub const TMT_HEADING1FONT: u32 = 0x327;
pub const TMT_HEADING2FONT: u32 = 0x328;
pub const TMT_BODYFONT: u32 = 0x329;
pub const TMT_FIRSTFONT: u32 = 0x321;
pub const TMT_LASTFONT: u32 = 0x329;
pub const TMT_FLATMENUS: u32 = 0x3E9;
pub const TMT_FIRSTBOOL: u32 = 0x3E9;
pub const TMT_LASTBOOL: u32 = 0x3E9;
pub const TMT_SIZINGBORDERWIDTH: u32 = 0x4B1;
pub const TMT_SCROLLBARWIDTH: u32 = 0x4B2;
pub const TMT_SCROLLBARHEIGHT: u32 = 0x4B3;
pub const TMT_CAPTIONBARWIDTH: u32 = 0x4B4;
pub const TMT_CAPTIONBARHEIGHT: u32 = 0x4B5;
pub const TMT_SMCAPTIONBARWIDTH: u32 = 0x4B6;
pub const TMT_SMCAPTIONBARHEIGHT: u32 = 0x4B7;
pub const TMT_MENUBARWIDTH: u32 = 0x4B8;
pub const TMT_MENUBARHEIGHT: u32 = 0x4B9;
pub const TMT_PADDEDBORDERWIDTH: u32 = 0x4BA;
pub const TMT_FIRSTSIZE: u32 = 0x4B1;
pub const TMT_LASTSIZE: u32 = 0x4BA;
pub const TMT_MINCOLORDEPTH: u32 = 0x515;
pub const TMT_FIRSTINT: u32 = 0x515;
pub const TMT_LASTINT: u32 = 0x515;
pub const TMT_CSSNAME: u32 = 0x579;
pub const TMT_XMLNAME: u32 = 0x57A;
pub const TMT_LASTUPDATED: u32 = 0x57B;
pub const TMT_ALIAS: u32 = 0x57C;
pub const TMT_FIRSTSTRING: u32 = 0x579;
pub const TMT_LASTSTRING: u32 = 0x57C;
pub const TMT_SCROLLBAR: u32 = 0x641;
pub const TMT_BACKGROUND: u32 = 0x642;
pub const TMT_ACTIVECAPTION: u32 = 0x643;
pub const TMT_INACTIVECAPTION: u32 = 0x644;
pub const TMT_MENU: u32 = 0x645;
pub const TMT_WINDOW: u32 = 0x646;
pub const TMT_WINDOWFRAME: u32 = 0x647;
pub const TMT_MENUTEXT: u32 = 0x648;
pub const TMT_WINDOWTEXT: u32 = 0x649;
pub const TMT_CAPTIONTEXT: u32 = 0x64A;
pub const TMT_ACTIVEBORDER: u32 = 0x64B;
pub const TMT_INACTIVEBORDER: u32 = 0x64C;
pub const TMT_APPWORKSPACE: u32 = 0x64D;
pub const TMT_HIGHLIGHT: u32 = 0x64E;
pub const TMT_HIGHLIGHTTEXT: u32 = 0x64F;
pub const TMT_BTNFACE: u32 = 0x650;
pub const TMT_BTNSHADOW: u32 = 0x651;
pub const TMT_GRAYTEXT: u32 = 0x652;
pub const TMT_BTNTEXT: u32 = 0x653;
pub const TMT_INACTIVECAPTIONTEXT: u32 = 0x654;
pub const TMT_BTNHIGHLIGHT: u32 = 0x655;
pub const TMT_DKSHADOW3D: u32 = 0x656;
pub const TMT_LIGHT3D: u32 = 0x657;
pub const TMT_INFOTEXT: u32 = 0x658;
pub const TMT_INFOBK: u32 = 0x659;
pub const TMT_BUTTONALTERNATEFACE: u32 = 0x65A;
pub const TMT_HOTTRACKING: u32 = 0x65B;
pub const TMT_GRADIENTACTIVECAPTION: u32 = 0x65C;
pub const TMT_GRADIENTINACTIVECAPTION: u32 = 0x65D;
pub const TMT_MENUHILIGHT: u32 = 0x65E;
pub const TMT_MENUBAR: u32 = 0x65F;
pub const TMT_FIRSTCOLOR: u32 = 0x641;
pub const TMT_LASTCOLOR: u32 = 0x65F;
pub const TMT_FROMHUE1: u32 = 0x709;
pub const TMT_FROMHUE2: u32 = 0x70A;
pub const TMT_FROMHUE3: u32 = 0x70B;
pub const TMT_FROMHUE4: u32 = 0x70C;
pub const TMT_FROMHUE5: u32 = 0x70D;
pub const TMT_TOHUE1: u32 = 0x70E;
pub const TMT_TOHUE2: u32 = 0x70F;
pub const TMT_TOHUE3: u32 = 0x710;
pub const TMT_TOHUE4: u32 = 0x711;
pub const TMT_TOHUE5: u32 = 0x712;
pub const TMT_FROMCOLOR1: u32 = 0x7D1;
pub const TMT_FROMCOLOR2: u32 = 0x7D2;
pub const TMT_FROMCOLOR3: u32 = 0x7D3;
pub const TMT_FROMCOLOR4: u32 = 0x7D4;
pub const TMT_FROMCOLOR5: u32 = 0x7D5;
pub const TMT_TOCOLOR1: u32 = 0x7D6;
pub const TMT_TOCOLOR2: u32 = 0x7D7;
pub const TMT_TOCOLOR3: u32 = 0x7D8;
pub const TMT_TOCOLOR4: u32 = 0x7D9;
pub const TMT_TOCOLOR5: u32 = 0x7DA;
pub const TMT_TRANSPARENT: u32 = 0x899;
pub const TMT_AUTOSIZE: u32 = 0x89A;
pub const TMT_BORDERONLY: u32 = 0x89B;
pub const TMT_COMPOSITED: u32 = 0x89C;
pub const TMT_BGFILL: u32 = 0x89D;
pub const TMT_GLYPHTRANSPARENT: u32 = 0x89E;
pub const TMT_GLYPHONLY: u32 = 0x89F;
pub const TMT_ALWAYSSHOWSIZINGBAR: u32 = 0x8A0;
pub const TMT_MIRRORIMAGE: u32 = 0x8A1;
pub const TMT_UNIFORMSIZING: u32 = 0x8A2;
pub const TMT_INTEGRALSIZING: u32 = 0x8A3;
pub const TMT_SOURCEGROW: u32 = 0x8A4;
pub const TMT_SOURCESHRINK: u32 = 0x8A5;
pub const TMT_DRAWBORDERS: u32 = 0x8A6;
pub const TMT_NOETCHEDEFFECT: u32 = 0x8A7;
pub const TMT_TEXTAPPLYOVERLAY: u32 = 0x8A8;
pub const TMT_TEXTGLOW: u32 = 0x8A9;
pub const TMT_TEXTITALIC: u32 = 0x8AA;
pub const TMT_COMPOSITEDOPAQUE: u32 = 0x8AB;
pub const TMT_LOCALIZEDMIRRORIMAGE: u32 = 0x8AC;
pub const TMT_IMAGECOUNT: u32 = 0x961;
pub const TMT_ALPHALEVEL: u32 = 0x962;
pub const TMT_BORDERSIZE: u32 = 0x963;
pub const TMT_ROUNDCORNERWIDTH: u32 = 0x964;
pub const TMT_ROUNDCORNERHEIGHT: u32 = 0x965;
pub const TMT_GRADIENTRATIO1: u32 = 0x966;
pub const TMT_GRADIENTRATIO2: u32 = 0x967;
pub const TMT_GRADIENTRATIO3: u32 = 0x968;
pub const TMT_GRADIENTRATIO4: u32 = 0x969;
pub const TMT_GRADIENTRATIO5: u32 = 0x96A;
pub const TMT_PROGRESSCHUNKSIZE: u32 = 0x96B;
pub const TMT_PROGRESSSPACESIZE: u32 = 0x96C;
pub const TMT_SATURATION: u32 = 0x96D;
pub const TMT_TEXTBORDERSIZE: u32 = 0x96E;
pub const TMT_ALPHATHRESHOLD: u32 = 0x96F;
pub const TMT_WIDTH: u32 = 0x970;
pub const TMT_HEIGHT: u32 = 0x971;
pub const TMT_GLYPHINDEX: u32 = 0x972;
pub const TMT_TRUESIZESTRETCHMARK: u32 = 0x973;
pub const TMT_MINDPI1: u32 = 0x974;
pub const TMT_MINDPI2: u32 = 0x975;
pub const TMT_MINDPI3: u32 = 0x976;
pub const TMT_MINDPI4: u32 = 0x977;
pub const TMT_MINDPI5: u32 = 0x978;
pub const TMT_TEXTGLOWSIZE: u32 = 0x979;
pub const TMT_FRAMESPERSECOND: u32 = 0x97A;
pub const TMT_PIXELSPERFRAME: u32 = 0x97B;
pub const TMT_ANIMATIONDELAY: u32 = 0x97C;
pub const TMT_GLOWINTENSITY: u32 = 0x97D;
pub const TMT_OPACITY: u32 = 0x97E;
pub const TMT_COLORIZATIONCOLOR: u32 = 0x97F;
pub const TMT_COLORIZATIONOPACITY: u32 = 0x980;
pub const TMT_MINDPI6: u32 = 0x981;
pub const TMT_MINDPI7: u32 = 0x982;
pub const TMT_GLYPHFONT: u32 = 0xA29;
pub const TMT_IMAGEFILE: u32 = 0xBB9;
pub const TMT_IMAGEFILE1: u32 = 0xBBA;
pub const TMT_IMAGEFILE2: u32 = 0xBBB;
pub const TMT_IMAGEFILE3: u32 = 0xBBC;
pub const TMT_IMAGEFILE4: u32 = 0xBBD;
pub const TMT_IMAGEFILE5: u32 = 0xBBE;
pub const TMT_GLYPHIMAGEFILE: u32 = 0xBC0;
pub const TMT_IMAGEFILE6: u32 = 0xBC1;
pub const TMT_IMAGEFILE7: u32 = 0xBC2;
pub const TMT_TEXT: u32 = 0xC81;
pub const TMT_CLASSICVALUE: u32 = 0xC82;
pub const TMT_OFFSET: u32 = 0xD49;
pub const TMT_TEXTSHADOWOFFSET: u32 = 0xD4A;
pub const TMT_MINSIZE: u32 = 0xD4B;
pub const TMT_MINSIZE1: u32 = 0xD4C;
pub const TMT_MINSIZE2: u32 = 0xD4D;
pub const TMT_MINSIZE3: u32 = 0xD4E;
pub const TMT_MINSIZE4: u32 = 0xD4F;
pub const TMT_MINSIZE5: u32 = 0xD50;
pub const TMT_NORMALSIZE: u32 = 0xD51;
pub const TMT_MINSIZE6: u32 = 0xD52;
pub const TMT_MINSIZE7: u32 = 0xD53;
pub const TMT_SIZINGMARGINS: u32 = 0xE11;
pub const TMT_CONTENTMARGINS: u32 = 0xE12;
pub const TMT_CAPTIONMARGINS: u32 = 0xE13;
pub const TMT_BORDERCOLOR: u32 = 0xED9;
pub const TMT_FILLCOLOR: u32 = 0xEDA;
pub const TMT_TEXTCOLOR: u32 = 0xEDB;
pub const TMT_EDGELIGHTCOLOR: u32 = 0xEDC;
pub const TMT_EDGEHIGHLIGHTCOLOR: u32 = 0xEDD;
pub const TMT_EDGESHADOWCOLOR: u32 = 0xEDE;
pub const TMT_EDGEDKSHADOWCOLOR: u32 = 0xEDF;
pub const TMT_EDGEFILLCOLOR: u32 = 0xEE0;
pub const TMT_TRANSPARENTCOLOR: u32 = 0xEE1;
pub const TMT_GRADIENTCOLOR1: u32 = 0xEE2;
pub const TMT_GRADIENTCOLOR2: u32 = 0xEE3;
pub const TMT_GRADIENTCOLOR3: u32 = 0xEE4;
pub const TMT_GRADIENTCOLOR4: u32 = 0xEE5;
pub const TMT_GRADIENTCOLOR5: u32 = 0xEE6;
pub const TMT_SHADOWCOLOR: u32 = 0xEE7;
pub const TMT_GLOWCOLOR: u32 = 0xEE8;
pub const TMT_TEXTBORDERCOLOR: u32 = 0xEE9;
pub const TMT_TEXTSHADOWCOLOR: u32 = 0xEEA;
pub const TMT_GLYPHTEXTCOLOR: u32 = 0xEEB;
pub const TMT_GLYPHTRANSPARENTCOLOR: u32 = 0xEEC;
pub const TMT_FILLCOLORHINT: u32 = 0xEED;
pub const TMT_BORDERCOLORHINT: u32 = 0xEEE;
pub const TMT_ACCENTCOLORHINT: u32 = 0xEEF;
pub const TMT_TEXTCOLORHINT: u32 = 0xEF0;
pub const TMT_HEADING1TEXTCOLOR: u32 = 0xEF1;
pub const TMT_HEADING2TEXTCOLOR: u32 = 0xEF2;
pub const TMT_BODYTEXTCOLOR: u32 = 0xEF3;
pub const TMT_BGTYPE: u32 = 0xFA1;
pub const TMT_BORDERTYPE: u32 = 0xFA2;
pub const TMT_FILLTYPE: u32 = 0xFA3;
pub const TMT_SIZINGTYPE: u32 = 0xFA4;
pub const TMT_HALIGN: u32 = 0xFA5;
pub const TMT_CONTENTALIGNMENT: u32 = 0xFA6;
pub const TMT_VALIGN: u32 = 0xFA7;
pub const TMT_OFFSETTYPE: u32 = 0xFA8;
pub const TMT_ICONEFFECT: u32 = 0xFA9;
pub const TMT_TEXTSHADOWTYPE: u32 = 0xFAA;
pub const TMT_IMAGELAYOUT: u32 = 0xFAB;
pub const TMT_GLYPHTYPE: u32 = 0xFAC;
pub const TMT_IMAGESELECTTYPE: u32 = 0xFAD;
pub const TMT_GLYPHFONTSIZINGTYPE: u32 = 0xFAE;
pub const TMT_TRUESIZESCALINGTYPE: u32 = 0xFAF;
pub const TMT_USERPICTURE: u32 = 0x1389;
pub const TMT_DEFAULTPANESIZE: u32 = 0x138A;
pub const TMT_BLENDCOLOR: u32 = 0x138B;
pub const TMT_CUSTOMSPLITRECT: u32 = 0x138C;
pub const TMT_ANIMATIONBUTTONRECT: u32 = 0x138D;
pub const TMT_ANIMATIONDURATION: u32 = 0x138E;
pub const TMT_TRANSITIONDURATIONS: u32 = 0x1770;
pub const TMT_SCALEDBACKGROUND: u32 = 0x1B59;
pub const TMT_ATLASIMAGE: u32 = 0x1F40;
pub const TMT_ATLASINPUTIMAGE: u32 = 0x1F41;
pub const TMT_ATLASRECT: u32 = 0x1F42;
pub struct Thumbbutton {
    pub dwMask: i32,
    pub iId: u32,
    pub iBitmap: u32,
    pub hIcon: HICON,
    pub szTip: [u8; 260],
    pub dwFlags: i32,
}
pub const THBF_ENABLED: i32 = 0x0;
pub const THBF_DISABLED: i32 = 0x1;
pub const THBF_DISMISSONCLICK: i32 = 0x2;
pub const THBF_NOBACKGROUND: i32 = 0x4;
pub const THBF_HIDDEN: i32 = 0x8;
pub const THBF_NONINTERACTIVE: i32 = 0x10;
pub const THB_BITMAP: i32 = 0x1;
pub const THB_ICON: i32 = 0x2;
pub const THB_TOOLTIP: i32 = 0x4;
pub const THB_FLAGS: i32 = 0x8;
pub const MDITILE_HORIZONTAL: u32 = 0x1;
pub const MDITILE_VERTICAL: u32 = 0x0;
pub struct Titlebarinfo {
    pub cbSize: u32,
    pub rcTitleBar: RECT,
    pub rgstate: [u32; 6],
}
pub struct Titlebarinfoex {
    pub cbSize: u32,
    pub rcTitleBar: RECT,
    pub rgstate: [u32; 6],
    pub rgrect: [RECT; 6],
}
pub const TI_BITMAP: i32 = 0x1;
pub const TI_JPEG: i32 = 0x2;
pub const TLEF_RELATIVE_INCLUDE_CURRENT: i32 = 0x1;
pub const TLEF_RELATIVE_BACK: i32 = 0x10;
pub const TLEF_RELATIVE_FORE: i32 = 0x20;
pub const TLEF_INCLUDE_UNINVOKEABLE: i32 = 0x40;
pub const TLEF_ABSOLUTE: i32 = 0x31;
pub const TLEF_EXCLUDE_SUBFRAME_ENTRIES: i32 = 0x80;
pub const TLEF_EXCLUDE_ABOUT_PAGES: i32 = 0x100;
pub const TOUCH_FEEDBACK_DEFAULT: u32 = 0x1;
pub const TOUCH_FEEDBACK_INDIRECT: u32 = 0x2;
pub const TOUCH_FEEDBACK_NONE: u32 = 0x3;
pub struct TouchHitTestingInput {
    pub pointerId: u32,
    pub point: POINT,
    pub boundingBox: RECT,
    pub nonOccludedBoundingBox: RECT,
    pub orientation: u32,
}
pub struct TouchHitTestingProximityEvaluation {
    pub score: u16,
    pub adjustedPoint: POINT,
}
pub struct Tpmparams {
    pub cbSize: u32,
    pub rcExclude: RECT,
}
pub const TPM_LEFTBUTTON: u32 = 0x0;
pub const TPM_RIGHTBUTTON: u32 = 0x2;
pub const TPM_LEFTALIGN: u32 = 0x0;
pub const TPM_CENTERALIGN: u32 = 0x4;
pub const TPM_RIGHTALIGN: u32 = 0x8;
pub const TPM_TOPALIGN: u32 = 0x0;
pub const TPM_VCENTERALIGN: u32 = 0x10;
pub const TPM_BOTTOMALIGN: u32 = 0x20;
pub const TPM_HORIZONTAL: u32 = 0x0;
pub const TPM_VERTICAL: u32 = 0x40;
pub const TPM_NONOTIFY: u32 = 0x80;
pub const TPM_RETURNCMD: u32 = 0x100;
pub const TPM_RECURSE: u32 = 0x1;
pub const TPM_HORPOSANIMATION: u32 = 0x400;
pub const TPM_HORNEGANIMATION: u32 = 0x800;
pub const TPM_VERPOSANIMATION: u32 = 0x1000;
pub const TPM_VERNEGANIMATION: u32 = 0x2000;
pub const TPM_NOANIMATION: u32 = 0x4000;
pub const TPM_LAYOUTRTL: u32 = 0x8000;
pub const TPM_WORKAREA: u32 = 0x10000;
pub const MCTGC_HOT: i32 = 0x1;
pub const MCTGC_HASSTATE: i32 = 0x2;
pub const MCTGC_HASSTATEHOT: i32 = 0x3;
pub const MCTGC_TODAY: i32 = 0x4;
pub const MCTGC_TODAYSELECTED: i32 = 0x5;
pub const MCTGC_SELECTED: i32 = 0x6;
pub const MCTGC_SELECTEDHOT: i32 = 0x7;
pub const MCTGCU_HOT: i32 = 0x1;
pub const MCTGCU_HASSTATE: i32 = 0x2;
pub const MCTGCU_HASSTATEHOT: i32 = 0x3;
pub const MCTGCU_SELECTED: i32 = 0x4;
pub const MCTGCU_SELECTEDHOT: i32 = 0x5;
pub const TNP_BACKGROUND: i32 = 0x1;
pub const TNP_ANIMBACKGROUND: i32 = 0x2;
pub struct Trivertex {
    pub x: i32,
    pub y: i32,
    pub Red: u16,
    pub Green: u16,
    pub Blue: u16,
    pub Alpha: u16,
}
pub const TSST_NONE: i32 = 0x0;
pub const TSST_SIZE: i32 = 0x1;
pub const TSST_DPI: i32 = 0x2;
pub struct Ttembedinfo<'a> {
    pub usStructSize: u16,
    pub usRootStrSize: u16,
    pub pusRootStr: &'a mut u16,
}
pub const TTEMBED_EMBEDEUDC: u32 = 0x20;
pub const TTEMBED_RAW: u32 = 0x0;
pub const TTEMBED_SUBSET: u32 = 0x1;
pub const TTEMBED_TTCOMPRESSED: u32 = 0x4;
pub struct Ttgettitle<'a> {
    pub dwSize: u32,
    pub uTitleBitmap: u32,
    pub cch: u32,
    pub pszTitle: &'a mut Cow<'a, OsStr>,
}
pub struct Tthittestinfoa<'a> {
    pub hwnd: HWND,
    pub pt: POINT,
    pub ti: TTTOOLINFOA<'a>,
}
pub struct Tthittestinfow<'a> {
    pub hwnd: HWND,
    pub pt: POINT,
    pub ti: TTTOOLINFOW<'a>,
}
pub struct Ttloadinfo<'a> {
    pub usStructSize: u16,
    pub usRefStrSize: u16,
    pub pusRefStr: &'a mut u16,
}
pub const TTLOAD_FONT_SUBSETTED: u32 = 0x1;
pub const TTLOAD_FONT_IN_SYSSTARTUP: u32 = 0x2;
pub struct Ttpolycurve {
    pub wType: u16,
    pub cpfx: u16,
    pub apfx: [POINTFX; 1],
}
pub struct Ttpolygonheader {
    pub cb: u32,
    pub dwType: u32,
    pub pfxStart: POINTFX,
}
pub struct Tttoolinfoa<'a> {
    pub cbSize: u32,
    pub uFlags: u32,
    pub hwnd: HWND,
    pub uId: usize,
    pub rect: RECT,
    pub hinst: HINSTANCE,
    pub lpszText: &'a mut Cow<'a, str>,
    pub lParam: LPARAM,
    pub lpReserved: &'a mut todo_void,
}
pub struct Tttoolinfow<'a> {
    pub cbSize: u32,
    pub uFlags: u32,
    pub hwnd: HWND,
    pub uId: usize,
    pub rect: RECT,
    pub hinst: HINSTANCE,
    pub lpszText: &'a mut Cow<'a, OsStr>,
    pub lParam: LPARAM,
    pub lpReserved: &'a mut todo_void,
}
pub const TTF_ABSOLUTE: u32 = 0x80;
pub const TTF_CENTERTIP: u32 = 0x2;
pub const TTF_IDISHWND: u32 = 0x1;
pub const TTF_PARSELINKS: u32 = 0x1000;
pub const TTF_RTLREADING: u32 = 0x4;
pub const TTF_SUBCLASS: u32 = 0x10;
pub const TTF_TRACK: u32 = 0x20;
pub const TTF_TRANSPARENT: u32 = 0x100;
pub struct Ttvalidationtestsparams<'a> {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pusCharCodeSet: &'a mut u16,
}
pub struct Ttvalidationtestsparamsex<'a> {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pulCharCodeSet: &'a mut u32,
}
pub struct Tvgetitempartrectinfo<'a> {
    pub hti: HTREEITEM,
    pub prc: &'a mut RECT,
    pub partID: i32,
}
pub struct Tvhittestinfo {
    pub pt: POINT,
    pub flags: u32,
    pub hItem: HTREEITEM,
}
pub const TVHT_ABOVE: u32 = 0x100;
pub const TVHT_BELOW: u32 = 0x200;
pub const TVHT_NOWHERE: u32 = 0x1;
pub const TVHT_ONITEM: u32 = 0x46;
pub const TVHT_ONITEMBUTTON: u32 = 0x10;
pub const TVHT_ONITEMICON: u32 = 0x2;
pub const TVHT_ONITEMINDENT: u32 = 0x8;
pub const TVHT_ONITEMLABEL: u32 = 0x4;
pub const TVHT_ONITEMRIGHT: u32 = 0x20;
pub const TVHT_ONITEMSTATEICON: u32 = 0x40;
pub const TVHT_TOLEFT: u32 = 0x800;
pub const TVHT_TORIGHT: u32 = 0x400;
pub struct Tvinsertstructa<'a> {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTA_42<'a>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Tvinsertstructa42<'a> {
    pub field0: TVITEMEXA<'a>,
    pub field1: TVITEMA<'a>,
}
pub struct Tvitemexa<'a> {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
pub struct Tvitema<'a> {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: LPARAM,
}
pub struct Tvinsertstructw<'a> {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTW_43<'a>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Tvinsertstructw43<'a> {
    pub field0: TVITEMEXW<'a>,
    pub field1: TVITEMW<'a>,
}
pub struct Tvitemexw<'a> {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
pub struct Tvitemw<'a> {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: LPARAM,
}
pub struct Tvitema<'a> {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: LPARAM,
}
pub struct Tvitemexa<'a> {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, str>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
pub struct Tvitemexw<'a> {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
pub const I_ZERO: i32 = 0x0;
pub const I_ONE_OR_MORE: i32 = 0x1;
pub const I_CHILDRENCALLBACK: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const I_CHILDRENAUTO: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE;
pub const TVGIPR_BUTTON: i32 = 0x1;
pub struct Tvitemw<'a> {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: &'a mut Cow<'a, OsStr>,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: LPARAM,
}
pub const TVIF_CHILDREN: u32 = 0x40;
pub const TVIF_DI_SETITEM: u32 = 0x1000;
pub const TVIF_HANDLE: u32 = 0x10;
pub const TVIF_IMAGE: u32 = 0x2;
pub const TVIF_PARAM: u32 = 0x4;
pub const TVIF_SELECTEDIMAGE: u32 = 0x20;
pub const TVIF_STATE: u32 = 0x8;
pub const TVIF_TEXT: u32 = 0x1;
pub const TVIF_EXPANDEDIMAGE: u32 = 0x200;
pub const TVIF_INTEGRAL: u32 = 0x80;
pub const TVIF_STATEEX: u32 = 0x100;
pub struct Tvsortcb {
    pub hParent: HTREEITEM,
    pub lpfnCompare: todo_fn,
    pub lParam: LPARAM,
}
pub struct TaskbarList {
}
pub struct ThumbnailStreamCache {
}
pub const EXTRACT_IF_NOT_CACHED: i32 = 0x0;
pub const RETURN_ONLY_IF_CACHED: i32 = 0x1;
pub const RESIZE_THUMBNAIL: i32 = 0x2;
pub const ALLOW_SMALLER_SIZE: i32 = 0x4;
pub struct TimeCategorizer {
}
pub struct TouchPredictionParameters {
    pub cbSize: u32,
    pub dwLatency: u32,
    pub dwSampleTime: u32,
    pub bUseHWTimeStamp: u32,
}
pub struct TrackShellMenu {
}
pub struct TrayBandSiteService {
}
pub struct TrayDeskBand {
}
pub struct Udaccel {
    pub nSec: u32,
    pub nInc: u32,
}
pub const UR_RESOLUTION_CHANGE: i32 = 0x0;
pub const UR_MONITOR_DISCONNECT: i32 = 0x1;
pub struct UnicodeString<'a> {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: &'a mut Cow<'a, OsStr>,
}
pub struct Updatelayeredwindowinfo<'a> {
    pub cbSize: u32,
    pub hdcDst: HDC,
    pub pptDst: &'a POINT,
    pub psize: &'a SIZE,
    pub hdcSrc: HDC,
    pub pptSrc: &'a POINT,
    pub crKey: u32,
    pub pblend: &'a BLENDFUNCTION,
    pub dwFlags: u32,
    pub prcDirty: &'a RECT,
}
pub const ULW_ALPHA: u32 = 0x2;
pub const ULW_COLORKEY: u32 = 0x1;
pub const ULW_OPAQUE: u32 = 0x4;
pub const ULW_EX_NORESIZE: u32 = 0x8;
pub const URLIS_URL: i32 = 0x0;
pub const URLIS_OPAQUE: i32 = 0x1;
pub const URLIS_NOHISTORY: i32 = 0x2;
pub const URLIS_FILEURL: i32 = 0x3;
pub const URLIS_APPLIABLE: i32 = 0x4;
pub const URLIS_DIRECTORY: i32 = 0x5;
pub const URLIS_HASQUERY: i32 = 0x6;
pub const URL_PART_NONE: i32 = 0x0;
pub const URL_PART_SCHEME: i32 = 0x1;
pub const URL_PART_HOSTNAME: i32 = 0x2;
pub const URL_PART_USERNAME: i32 = 0x3;
pub const URL_PART_PASSWORD: i32 = 0x4;
pub const URL_PART_PORT: i32 = 0x5;
pub const URL_PART_QUERY: i32 = 0x6;
pub const URL_SCHEME_INVALID: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const URL_SCHEME_UNKNOWN: i32 = 0x0;
pub const URL_SCHEME_FTP: i32 = 0x1;
pub const URL_SCHEME_HTTP: i32 = 0x2;
pub const URL_SCHEME_GOPHER: i32 = 0x3;
pub const URL_SCHEME_MAILTO: i32 = 0x4;
pub const URL_SCHEME_NEWS: i32 = 0x5;
pub const URL_SCHEME_NNTP: i32 = 0x6;
pub const URL_SCHEME_TELNET: i32 = 0x7;
pub const URL_SCHEME_WAIS: i32 = 0x8;
pub const URL_SCHEME_FILE: i32 = 0x9;
pub const URL_SCHEME_MK: i32 = 0xA;
pub const URL_SCHEME_HTTPS: i32 = 0xB;
pub const URL_SCHEME_SHELL: i32 = 0xC;
pub const URL_SCHEME_SNEWS: i32 = 0xD;
pub const URL_SCHEME_LOCAL: i32 = 0xE;
pub const URL_SCHEME_JAVASCRIPT: i32 = 0xF;
pub const URL_SCHEME_VBSCRIPT: i32 = 0x10;
pub const URL_SCHEME_ABOUT: i32 = 0x11;
pub const URL_SCHEME_RES: i32 = 0x12;
pub const URL_SCHEME_MSSHELLROOTED: i32 = 0x13;
pub const URL_SCHEME_MSSHELLIDLIST: i32 = 0x14;
pub const URL_SCHEME_MSHELP: i32 = 0x15;
pub const URL_SCHEME_MSSHELLDEVICE: i32 = 0x16;
pub const URL_SCHEME_WILDCARD: i32 = 0x17;
pub const URL_SCHEME_SEARCH_MS: i32 = 0x18;
pub const URL_SCHEME_SEARCH: i32 = 0x19;
pub const URL_SCHEME_KNOWNFOLDER: i32 = 0x1A;
pub const URL_SCHEME_MAXVALUE: i32 = 0x1B;
pub struct UsageProperties {
    pub level: u16,
    pub page: u16,
    pub usage: u16,
    pub logicalMinimum: i32,
    pub logicalMaximum: i32,
    pub unit: u16,
    pub exponent: u16,
    pub count: u8,
    pub physicalMinimum: i32,
    pub physicalMaximum: i32,
}
pub struct UserNotification {
}
pub struct V1PasswordCredentialProvider {
}
pub struct V1SmartcardCredentialProvider {
}
pub struct V1WinBioCredentialProvider {
}
pub struct Valenta<'a> {
    pub ve_valuename: &'a mut Cow<'a, str>,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
}
pub struct Valentw<'a> {
    pub ve_valuename: &'a mut Cow<'a, OsStr>,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
}
pub const VALIDATEUNC_CONNECT: i32 = 0x1;
pub const VALIDATEUNC_NOUI: i32 = 0x2;
pub const VALIDATEUNC_PRINT: i32 = 0x4;
pub const VALIDATEUNC_PERSIST: i32 = 0x8;
pub const VALIDATEUNC_VALID: i32 = 0xF;
pub const VA_TOP: i32 = 0x0;
pub const VA_CENTER: i32 = 0x1;
pub const VA_BOTTOM: i32 = 0x2;
pub const VPCF_TEXT: i32 = 0x1;
pub const VPCF_BACKGROUND: i32 = 0x2;
pub const VPCF_SORTCOLUMN: i32 = 0x3;
pub const VPCF_SUBTEXT: i32 = 0x4;
pub const VPCF_TEXTBACKGROUND: i32 = 0x5;
pub const VPWF_DEFAULT: i32 = 0x0;
pub const VPWF_ALPHABLEND: i32 = 0x1;
pub struct VaultProvider {
}
pub struct VirtualDesktopManager {
}
pub struct Wcrange {
    pub wcLow: u8,
    pub cGlyphs: u16,
}
pub struct Wglswap {
    pub hdc: HDC,
    pub uiFlags: u32,
}
pub const NO_ERROR: u32 = 0x0;
pub const WAIT_TIMEOUT: u32 = 0x102;
pub const WAIT_FAILED: u32 = 0xFFFFFFFF;
pub const ERROR_SUCCESS: u32 = 0x0;
pub const ERROR_INVALID_FUNCTION: u32 = 0x1;
pub const ERROR_FILE_NOT_FOUND: u32 = 0x2;
pub const ERROR_PATH_NOT_FOUND: u32 = 0x3;
pub const ERROR_TOO_MANY_OPEN_FILES: u32 = 0x4;
pub const ERROR_ACCESS_DENIED: u32 = 0x5;
pub const ERROR_INVALID_HANDLE: u32 = 0x6;
pub const ERROR_ARENA_TRASHED: u32 = 0x7;
pub const ERROR_NOT_ENOUGH_MEMORY: u32 = 0x8;
pub const ERROR_INVALID_BLOCK: u32 = 0x9;
pub const ERROR_BAD_ENVIRONMENT: u32 = 0xA;
pub const ERROR_BAD_FORMAT: u32 = 0xB;
pub const ERROR_INVALID_ACCESS: u32 = 0xC;
pub const ERROR_INVALID_DATA: u32 = 0xD;
pub const ERROR_OUTOFMEMORY: u32 = 0xE;
pub const ERROR_INVALID_DRIVE: u32 = 0xF;
pub const ERROR_CURRENT_DIRECTORY: u32 = 0x10;
pub const ERROR_NOT_SAME_DEVICE: u32 = 0x11;
pub const ERROR_NO_MORE_FILES: u32 = 0x12;
pub const ERROR_WRITE_PROTECT: u32 = 0x13;
pub const ERROR_BAD_UNIT: u32 = 0x14;
pub const ERROR_NOT_READY: u32 = 0x15;
pub const ERROR_BAD_COMMAND: u32 = 0x16;
pub const ERROR_CRC: u32 = 0x17;
pub const ERROR_BAD_LENGTH: u32 = 0x18;
pub const ERROR_SEEK: u32 = 0x19;
pub const ERROR_NOT_DOS_DISK: u32 = 0x1A;
pub const ERROR_SECTOR_NOT_FOUND: u32 = 0x1B;
pub const ERROR_OUT_OF_PAPER: u32 = 0x1C;
pub const ERROR_WRITE_FAULT: u32 = 0x1D;
pub const ERROR_READ_FAULT: u32 = 0x1E;
pub const ERROR_GEN_FAILURE: u32 = 0x1F;
pub const ERROR_SHARING_VIOLATION: u32 = 0x20;
pub const ERROR_LOCK_VIOLATION: u32 = 0x21;
pub const ERROR_WRONG_DISK: u32 = 0x22;
pub const ERROR_SHARING_BUFFER_EXCEEDED: u32 = 0x24;
pub const ERROR_HANDLE_EOF: u32 = 0x26;
pub const ERROR_HANDLE_DISK_FULL: u32 = 0x27;
pub const ERROR_NOT_SUPPORTED: u32 = 0x32;
pub const ERROR_REM_NOT_LIST: u32 = 0x33;
pub const ERROR_DUP_NAME: u32 = 0x34;
pub const ERROR_BAD_NETPATH: u32 = 0x35;
pub const ERROR_NETWORK_BUSY: u32 = 0x36;
pub const ERROR_DEV_NOT_EXIST: u32 = 0x37;
pub const ERROR_TOO_MANY_CMDS: u32 = 0x38;
pub const ERROR_ADAP_HDW_ERR: u32 = 0x39;
pub const ERROR_BAD_NET_RESP: u32 = 0x3A;
pub const ERROR_UNEXP_NET_ERR: u32 = 0x3B;
pub const ERROR_BAD_REM_ADAP: u32 = 0x3C;
pub const ERROR_PRINTQ_FULL: u32 = 0x3D;
pub const ERROR_NO_SPOOL_SPACE: u32 = 0x3E;
pub const ERROR_PRINT_CANCELLED: u32 = 0x3F;
pub const ERROR_NETNAME_DELETED: u32 = 0x40;
pub const ERROR_NETWORK_ACCESS_DENIED: u32 = 0x41;
pub const ERROR_BAD_DEV_TYPE: u32 = 0x42;
pub const ERROR_BAD_NET_NAME: u32 = 0x43;
pub const ERROR_TOO_MANY_NAMES: u32 = 0x44;
pub const ERROR_TOO_MANY_SESS: u32 = 0x45;
pub const ERROR_SHARING_PAUSED: u32 = 0x46;
pub const ERROR_REQ_NOT_ACCEP: u32 = 0x47;
pub const ERROR_REDIR_PAUSED: u32 = 0x48;
pub const ERROR_FILE_EXISTS: u32 = 0x50;
pub const ERROR_CANNOT_MAKE: u32 = 0x52;
pub const ERROR_FAIL_I24: u32 = 0x53;
pub const ERROR_OUT_OF_STRUCTURES: u32 = 0x54;
pub const ERROR_ALREADY_ASSIGNED: u32 = 0x55;
pub const ERROR_INVALID_PASSWORD: u32 = 0x56;
pub const ERROR_INVALID_PARAMETER: u32 = 0x57;
pub const ERROR_NET_WRITE_FAULT: u32 = 0x58;
pub const ERROR_NO_PROC_SLOTS: u32 = 0x59;
pub const ERROR_TOO_MANY_SEMAPHORES: u32 = 0x64;
pub const ERROR_EXCL_SEM_ALREADY_OWNED: u32 = 0x65;
pub const ERROR_SEM_IS_SET: u32 = 0x66;
pub const ERROR_TOO_MANY_SEM_REQUESTS: u32 = 0x67;
pub const ERROR_INVALID_AT_INTERRUPT_TIME: u32 = 0x68;
pub const ERROR_SEM_OWNER_DIED: u32 = 0x69;
pub const ERROR_SEM_USER_LIMIT: u32 = 0x6A;
pub const ERROR_DISK_CHANGE: u32 = 0x6B;
pub const ERROR_DRIVE_LOCKED: u32 = 0x6C;
pub const ERROR_BROKEN_PIPE: u32 = 0x6D;
pub const ERROR_OPEN_FAILED: u32 = 0x6E;
pub const ERROR_BUFFER_OVERFLOW: u32 = 0x6F;
pub const ERROR_DISK_FULL: u32 = 0x70;
pub const ERROR_NO_MORE_SEARCH_HANDLES: u32 = 0x71;
pub const ERROR_INVALID_TARGET_HANDLE: u32 = 0x72;
pub const ERROR_INVALID_CATEGORY: u32 = 0x75;
pub const ERROR_INVALID_VERIFY_SWITCH: u32 = 0x76;
pub const ERROR_BAD_DRIVER_LEVEL: u32 = 0x77;
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 0x78;
pub const ERROR_SEM_TIMEOUT: u32 = 0x79;
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 0x7A;
pub const ERROR_INVALID_NAME: u32 = 0x7B;
pub const ERROR_INVALID_LEVEL: u32 = 0x7C;
pub const ERROR_NO_VOLUME_LABEL: u32 = 0x7D;
pub const ERROR_MOD_NOT_FOUND: u32 = 0x7E;
pub const ERROR_PROC_NOT_FOUND: u32 = 0x7F;
pub const ERROR_WAIT_NO_CHILDREN: u32 = 0x80;
pub const ERROR_CHILD_NOT_COMPLETE: u32 = 0x81;
pub const ERROR_DIRECT_ACCESS_HANDLE: u32 = 0x82;
pub const ERROR_NEGATIVE_SEEK: u32 = 0x83;
pub const ERROR_SEEK_ON_DEVICE: u32 = 0x84;
pub const ERROR_IS_JOIN_TARGET: u32 = 0x85;
pub const ERROR_IS_JOINED: u32 = 0x86;
pub const ERROR_IS_SUBSTED: u32 = 0x87;
pub const ERROR_NOT_JOINED: u32 = 0x88;
pub const ERROR_NOT_SUBSTED: u32 = 0x89;
pub const ERROR_JOIN_TO_JOIN: u32 = 0x8A;
pub const ERROR_SUBST_TO_SUBST: u32 = 0x8B;
pub const ERROR_JOIN_TO_SUBST: u32 = 0x8C;
pub const ERROR_SUBST_TO_JOIN: u32 = 0x8D;
pub const ERROR_BUSY_DRIVE: u32 = 0x8E;
pub const ERROR_SAME_DRIVE: u32 = 0x8F;
pub const ERROR_DIR_NOT_ROOT: u32 = 0x90;
pub const ERROR_DIR_NOT_EMPTY: u32 = 0x91;
pub const ERROR_IS_SUBST_PATH: u32 = 0x92;
pub const ERROR_IS_JOIN_PATH: u32 = 0x93;
pub const ERROR_PATH_BUSY: u32 = 0x94;
pub const ERROR_IS_SUBST_TARGET: u32 = 0x95;
pub const ERROR_SYSTEM_TRACE: u32 = 0x96;
pub const ERROR_INVALID_EVENT_COUNT: u32 = 0x97;
pub const ERROR_TOO_MANY_MUXWAITERS: u32 = 0x98;
pub const ERROR_INVALID_LIST_FORMAT: u32 = 0x99;
pub const ERROR_LABEL_TOO_LONG: u32 = 0x9A;
pub const ERROR_TOO_MANY_TCBS: u32 = 0x9B;
pub const ERROR_SIGNAL_REFUSED: u32 = 0x9C;
pub const ERROR_DISCARDED: u32 = 0x9D;
pub const ERROR_NOT_LOCKED: u32 = 0x9E;
pub const ERROR_BAD_THREADID_ADDR: u32 = 0x9F;
pub const ERROR_BAD_ARGUMENTS: u32 = 0xA0;
pub const ERROR_BAD_PATHNAME: u32 = 0xA1;
pub const ERROR_SIGNAL_PENDING: u32 = 0xA2;
pub const ERROR_MAX_THRDS_REACHED: u32 = 0xA4;
pub const ERROR_LOCK_FAILED: u32 = 0xA7;
pub const ERROR_BUSY: u32 = 0xAA;
pub const ERROR_DEVICE_SUPPORT_IN_PROGRESS: u32 = 0xAB;
pub const ERROR_CANCEL_VIOLATION: u32 = 0xAD;
pub const ERROR_ATOMIC_LOCKS_NOT_SUPPORTED: u32 = 0xAE;
pub const ERROR_INVALID_SEGMENT_NUMBER: u32 = 0xB4;
pub const ERROR_INVALID_ORDINAL: u32 = 0xB6;
pub const ERROR_ALREADY_EXISTS: u32 = 0xB7;
pub const ERROR_INVALID_FLAG_NUMBER: u32 = 0xBA;
pub const ERROR_SEM_NOT_FOUND: u32 = 0xBB;
pub const ERROR_INVALID_STARTING_CODESEG: u32 = 0xBC;
pub const ERROR_INVALID_STACKSEG: u32 = 0xBD;
pub const ERROR_INVALID_MODULETYPE: u32 = 0xBE;
pub const ERROR_INVALID_EXE_SIGNATURE: u32 = 0xBF;
pub const ERROR_EXE_MARKED_INVALID: u32 = 0xC0;
pub const ERROR_BAD_EXE_FORMAT: u32 = 0xC1;
pub const ERROR_ITERATED_DATA_EXCEEDS_64K: u32 = 0xC2;
pub const ERROR_INVALID_MINALLOCSIZE: u32 = 0xC3;
pub const ERROR_DYNLINK_FROM_INVALID_RING: u32 = 0xC4;
pub const ERROR_IOPL_NOT_ENABLED: u32 = 0xC5;
pub const ERROR_INVALID_SEGDPL: u32 = 0xC6;
pub const ERROR_AUTODATASEG_EXCEEDS_64K: u32 = 0xC7;
pub const ERROR_RING2SEG_MUST_BE_MOVABLE: u32 = 0xC8;
pub const ERROR_RELOC_CHAIN_XEEDS_SEGLIM: u32 = 0xC9;
pub const ERROR_INFLOOP_IN_RELOC_CHAIN: u32 = 0xCA;
pub const ERROR_ENVVAR_NOT_FOUND: u32 = 0xCB;
pub const ERROR_NO_SIGNAL_SENT: u32 = 0xCD;
pub const ERROR_FILENAME_EXCED_RANGE: u32 = 0xCE;
pub const ERROR_RING2_STACK_IN_USE: u32 = 0xCF;
pub const ERROR_META_EXPANSION_TOO_LONG: u32 = 0xD0;
pub const ERROR_INVALID_SIGNAL_NUMBER: u32 = 0xD1;
pub const ERROR_THREAD_1_INACTIVE: u32 = 0xD2;
pub const ERROR_LOCKED: u32 = 0xD4;
pub const ERROR_TOO_MANY_MODULES: u32 = 0xD6;
pub const ERROR_NESTING_NOT_ALLOWED: u32 = 0xD7;
pub const ERROR_EXE_MACHINE_TYPE_MISMATCH: u32 = 0xD8;
pub const ERROR_EXE_CANNOT_MODIFY_SIGNED_BINARY: u32 = 0xD9;
pub const ERROR_EXE_CANNOT_MODIFY_STRONG_SIGNED_BINARY: u32 = 0xDA;
pub const ERROR_FILE_CHECKED_OUT: u32 = 0xDC;
pub const ERROR_CHECKOUT_REQUIRED: u32 = 0xDD;
pub const ERROR_BAD_FILE_TYPE: u32 = 0xDE;
pub const ERROR_FILE_TOO_LARGE: u32 = 0xDF;
pub const ERROR_FORMS_AUTH_REQUIRED: u32 = 0xE0;
pub const ERROR_VIRUS_INFECTED: u32 = 0xE1;
pub const ERROR_VIRUS_DELETED: u32 = 0xE2;
pub const ERROR_PIPE_LOCAL: u32 = 0xE5;
pub const ERROR_BAD_PIPE: u32 = 0xE6;
pub const ERROR_PIPE_BUSY: u32 = 0xE7;
pub const ERROR_NO_DATA: u32 = 0xE8;
pub const ERROR_PIPE_NOT_CONNECTED: u32 = 0xE9;
pub const ERROR_MORE_DATA: u32 = 0xEA;
pub const ERROR_NO_WORK_DONE: u32 = 0xEB;
pub const ERROR_VC_DISCONNECTED: u32 = 0xF0;
pub const ERROR_INVALID_EA_NAME: u32 = 0xFE;
pub const ERROR_EA_LIST_INCONSISTENT: u32 = 0xFF;
pub const ERROR_NO_MORE_ITEMS: u32 = 0x103;
pub const ERROR_CANNOT_COPY: u32 = 0x10A;
pub const ERROR_DIRECTORY: u32 = 0x10B;
pub const ERROR_EAS_DIDNT_FIT: u32 = 0x113;
pub const ERROR_EA_FILE_CORRUPT: u32 = 0x114;
pub const ERROR_EA_TABLE_FULL: u32 = 0x115;
pub const ERROR_INVALID_EA_HANDLE: u32 = 0x116;
pub const ERROR_EAS_NOT_SUPPORTED: u32 = 0x11A;
pub const ERROR_NOT_OWNER: u32 = 0x120;
pub const ERROR_TOO_MANY_POSTS: u32 = 0x12A;
pub const ERROR_PARTIAL_COPY: u32 = 0x12B;
pub const ERROR_OPLOCK_NOT_GRANTED: u32 = 0x12C;
pub const ERROR_INVALID_OPLOCK_PROTOCOL: u32 = 0x12D;
pub const ERROR_DISK_TOO_FRAGMENTED: u32 = 0x12E;
pub const ERROR_DELETE_PENDING: u32 = 0x12F;
pub const ERROR_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING: u32 = 0x130;
pub const ERROR_SHORT_NAMES_NOT_ENABLED_ON_VOLUME: u32 = 0x131;
pub const ERROR_SECURITY_STREAM_IS_INCONSISTENT: u32 = 0x132;
pub const ERROR_INVALID_LOCK_RANGE: u32 = 0x133;
pub const ERROR_IMAGE_SUBSYSTEM_NOT_PRESENT: u32 = 0x134;
pub const ERROR_NOTIFICATION_GUID_ALREADY_DEFINED: u32 = 0x135;
pub const ERROR_INVALID_EXCEPTION_HANDLER: u32 = 0x136;
pub const ERROR_DUPLICATE_PRIVILEGES: u32 = 0x137;
pub const ERROR_NO_RANGES_PROCESSED: u32 = 0x138;
pub const ERROR_NOT_ALLOWED_ON_SYSTEM_FILE: u32 = 0x139;
pub const ERROR_DISK_RESOURCES_EXHAUSTED: u32 = 0x13A;
pub const ERROR_INVALID_TOKEN: u32 = 0x13B;
pub const ERROR_DEVICE_FEATURE_NOT_SUPPORTED: u32 = 0x13C;
pub const ERROR_MR_MID_NOT_FOUND: u32 = 0x13D;
pub const ERROR_SCOPE_NOT_FOUND: u32 = 0x13E;
pub const ERROR_UNDEFINED_SCOPE: u32 = 0x13F;
pub const ERROR_INVALID_CAP: u32 = 0x140;
pub const ERROR_DEVICE_UNREACHABLE: u32 = 0x141;
pub const ERROR_DEVICE_NO_RESOURCES: u32 = 0x142;
pub const ERROR_DATA_CHECKSUM_ERROR: u32 = 0x143;
pub const ERROR_INTERMIXED_KERNEL_EA_OPERATION: u32 = 0x144;
pub const ERROR_FILE_LEVEL_TRIM_NOT_SUPPORTED: u32 = 0x146;
pub const ERROR_OFFSET_ALIGNMENT_VIOLATION: u32 = 0x147;
pub const ERROR_INVALID_FIELD_IN_PARAMETER_LIST: u32 = 0x148;
pub const ERROR_OPERATION_IN_PROGRESS: u32 = 0x149;
pub const ERROR_BAD_DEVICE_PATH: u32 = 0x14A;
pub const ERROR_TOO_MANY_DESCRIPTORS: u32 = 0x14B;
pub const ERROR_SCRUB_DATA_DISABLED: u32 = 0x14C;
pub const ERROR_NOT_REDUNDANT_STORAGE: u32 = 0x14D;
pub const ERROR_RESIDENT_FILE_NOT_SUPPORTED: u32 = 0x14E;
pub const ERROR_COMPRESSED_FILE_NOT_SUPPORTED: u32 = 0x14F;
pub const ERROR_DIRECTORY_NOT_SUPPORTED: u32 = 0x150;
pub const ERROR_NOT_READ_FROM_COPY: u32 = 0x151;
pub const ERROR_FT_WRITE_FAILURE: u32 = 0x152;
pub const ERROR_FT_DI_SCAN_REQUIRED: u32 = 0x153;
pub const ERROR_INVALID_KERNEL_INFO_VERSION: u32 = 0x154;
pub const ERROR_INVALID_PEP_INFO_VERSION: u32 = 0x155;
pub const ERROR_OBJECT_NOT_EXTERNALLY_BACKED: u32 = 0x156;
pub const ERROR_EXTERNAL_BACKING_PROVIDER_UNKNOWN: u32 = 0x157;
pub const ERROR_COMPRESSION_NOT_BENEFICIAL: u32 = 0x158;
pub const ERROR_STORAGE_TOPOLOGY_ID_MISMATCH: u32 = 0x159;
pub const ERROR_BLOCKED_BY_PARENTAL_CONTROLS: u32 = 0x15A;
pub const ERROR_BLOCK_TOO_MANY_REFERENCES: u32 = 0x15B;
pub const ERROR_MARKED_TO_DISALLOW_WRITES: u32 = 0x15C;
pub const ERROR_ENCLAVE_FAILURE: u32 = 0x15D;
pub const ERROR_FAIL_NOACTION_REBOOT: u32 = 0x15E;
pub const ERROR_FAIL_SHUTDOWN: u32 = 0x15F;
pub const ERROR_FAIL_RESTART: u32 = 0x160;
pub const ERROR_MAX_SESSIONS_REACHED: u32 = 0x161;
pub const ERROR_NETWORK_ACCESS_DENIED_EDP: u32 = 0x162;
pub const ERROR_DEVICE_HINT_NAME_BUFFER_TOO_SMALL: u32 = 0x163;
pub const ERROR_EDP_POLICY_DENIES_OPERATION: u32 = 0x164;
pub const ERROR_EDP_DPL_POLICY_CANT_BE_SATISFIED: u32 = 0x165;
pub const ERROR_CLOUD_FILE_SYNC_ROOT_METADATA_CORRUPT: u32 = 0x166;
pub const ERROR_DEVICE_IN_MAINTENANCE: u32 = 0x167;
pub const ERROR_NOT_SUPPORTED_ON_DAX: u32 = 0x168;
pub const ERROR_DAX_MAPPING_EXISTS: u32 = 0x169;
pub const ERROR_CLOUD_FILE_PROVIDER_NOT_RUNNING: u32 = 0x16A;
pub const ERROR_CLOUD_FILE_METADATA_CORRUPT: u32 = 0x16B;
pub const ERROR_CLOUD_FILE_METADATA_TOO_LARGE: u32 = 0x16C;
pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_TOO_LARGE: u32 = 0x16D;
pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_CHECKSUM_MISMATCH: u32 = 0x16E;
pub const ERROR_CHILD_PROCESS_BLOCKED: u32 = 0x16F;
pub const ERROR_STORAGE_LOST_DATA_PERSISTENCE: u32 = 0x170;
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_UNAVAILABLE: u32 = 0x171;
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_METADATA_CORRUPT: u32 = 0x172;
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_BUSY: u32 = 0x173;
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_PROVIDER_UNKNOWN: u32 = 0x174;
pub const ERROR_GDI_HANDLE_LEAK: u32 = 0x175;
pub const ERROR_CLOUD_FILE_TOO_MANY_PROPERTY_BLOBS: u32 = 0x176;
pub const ERROR_CLOUD_FILE_PROPERTY_VERSION_NOT_SUPPORTED: u32 = 0x177;
pub const ERROR_NOT_A_CLOUD_FILE: u32 = 0x178;
pub const ERROR_CLOUD_FILE_NOT_IN_SYNC: u32 = 0x179;
pub const ERROR_CLOUD_FILE_ALREADY_CONNECTED: u32 = 0x17A;
pub const ERROR_CLOUD_FILE_NOT_SUPPORTED: u32 = 0x17B;
pub const ERROR_CLOUD_FILE_INVALID_REQUEST: u32 = 0x17C;
pub const ERROR_CLOUD_FILE_READ_ONLY_VOLUME: u32 = 0x17D;
pub const ERROR_CLOUD_FILE_CONNECTED_PROVIDER_ONLY: u32 = 0x17E;
pub const ERROR_CLOUD_FILE_VALIDATION_FAILED: u32 = 0x17F;
pub const ERROR_SMB1_NOT_AVAILABLE: u32 = 0x180;
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_INVALID_OPERATION: u32 = 0x181;
pub const ERROR_CLOUD_FILE_AUTHENTICATION_FAILED: u32 = 0x182;
pub const ERROR_CLOUD_FILE_INSUFFICIENT_RESOURCES: u32 = 0x183;
pub const ERROR_CLOUD_FILE_NETWORK_UNAVAILABLE: u32 = 0x184;
pub const ERROR_CLOUD_FILE_UNSUCCESSFUL: u32 = 0x185;
pub const ERROR_CLOUD_FILE_NOT_UNDER_SYNC_ROOT: u32 = 0x186;
pub const ERROR_CLOUD_FILE_IN_USE: u32 = 0x187;
pub const ERROR_CLOUD_FILE_PINNED: u32 = 0x188;
pub const ERROR_CLOUD_FILE_REQUEST_ABORTED: u32 = 0x189;
pub const ERROR_CLOUD_FILE_PROPERTY_CORRUPT: u32 = 0x18A;
pub const ERROR_CLOUD_FILE_ACCESS_DENIED: u32 = 0x18B;
pub const ERROR_CLOUD_FILE_INCOMPATIBLE_HARDLINKS: u32 = 0x18C;
pub const ERROR_CLOUD_FILE_PROPERTY_LOCK_CONFLICT: u32 = 0x18D;
pub const ERROR_CLOUD_FILE_REQUEST_CANCELED: u32 = 0x18E;
pub const ERROR_EXTERNAL_SYSKEY_NOT_SUPPORTED: u32 = 0x18F;
pub const ERROR_THREAD_MODE_ALREADY_BACKGROUND: u32 = 0x190;
pub const ERROR_THREAD_MODE_NOT_BACKGROUND: u32 = 0x191;
pub const ERROR_PROCESS_MODE_ALREADY_BACKGROUND: u32 = 0x192;
pub const ERROR_PROCESS_MODE_NOT_BACKGROUND: u32 = 0x193;
pub const ERROR_CLOUD_FILE_PROVIDER_TERMINATED: u32 = 0x194;
pub const ERROR_NOT_A_CLOUD_SYNC_ROOT: u32 = 0x195;
pub const ERROR_FILE_PROTECTED_UNDER_DPL: u32 = 0x196;
pub const ERROR_VOLUME_NOT_CLUSTER_ALIGNED: u32 = 0x197;
pub const ERROR_NO_PHYSICALLY_ALIGNED_FREE_SPACE_FOUND: u32 = 0x198;
pub const ERROR_APPX_FILE_NOT_ENCRYPTED: u32 = 0x199;
pub const ERROR_RWRAW_ENCRYPTED_FILE_NOT_ENCRYPTED: u32 = 0x19A;
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILEOFFSET: u32 = 0x19B;
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILERANGE: u32 = 0x19C;
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_PARAMETER: u32 = 0x19D;
pub const ERROR_LINUX_SUBSYSTEM_NOT_PRESENT: u32 = 0x19E;
pub const ERROR_FT_READ_FAILURE: u32 = 0x19F;
pub const ERROR_STORAGE_RESERVE_ID_INVALID: u32 = 0x1A0;
pub const ERROR_STORAGE_RESERVE_DOES_NOT_EXIST: u32 = 0x1A1;
pub const ERROR_STORAGE_RESERVE_ALREADY_EXISTS: u32 = 0x1A2;
pub const ERROR_STORAGE_RESERVE_NOT_EMPTY: u32 = 0x1A3;
pub const ERROR_NOT_A_DAX_VOLUME: u32 = 0x1A4;
pub const ERROR_NOT_DAX_MAPPABLE: u32 = 0x1A5;
pub const ERROR_TIME_SENSITIVE_THREAD: u32 = 0x1A6;
pub const ERROR_DPL_NOT_SUPPORTED_FOR_USER: u32 = 0x1A7;
pub const ERROR_CASE_DIFFERING_NAMES_IN_DIR: u32 = 0x1A8;
pub const ERROR_FILE_NOT_SUPPORTED: u32 = 0x1A9;
pub const ERROR_CLOUD_FILE_REQUEST_TIMEOUT: u32 = 0x1AA;
pub const ERROR_NO_TASK_QUEUE: u32 = 0x1AB;
pub const ERROR_SRC_SRV_DLL_LOAD_FAILED: u32 = 0x1AC;
pub const ERROR_NOT_SUPPORTED_WITH_BTT: u32 = 0x1AD;
pub const ERROR_ENCRYPTION_DISABLED: u32 = 0x1AE;
pub const ERROR_ENCRYPTING_METADATA_DISALLOWED: u32 = 0x1AF;
pub const ERROR_CANT_CLEAR_ENCRYPTION_FLAG: u32 = 0x1B0;
pub const ERROR_NO_SUCH_DEVICE: u32 = 0x1B1;
pub const ERROR_CLOUD_FILE_DEHYDRATION_DISALLOWED: u32 = 0x1B2;
pub const ERROR_FILE_SNAP_IN_PROGRESS: u32 = 0x1B3;
pub const ERROR_FILE_SNAP_USER_SECTION_NOT_SUPPORTED: u32 = 0x1B4;
pub const ERROR_FILE_SNAP_MODIFY_NOT_SUPPORTED: u32 = 0x1B5;
pub const ERROR_FILE_SNAP_IO_NOT_COORDINATED: u32 = 0x1B6;
pub const ERROR_FILE_SNAP_UNEXPECTED_ERROR: u32 = 0x1B7;
pub const ERROR_FILE_SNAP_INVALID_PARAMETER: u32 = 0x1B8;
pub const ERROR_UNSATISFIED_DEPENDENCIES: u32 = 0x1B9;
pub const ERROR_CASE_SENSITIVE_PATH: u32 = 0x1BA;
pub const ERROR_UNEXPECTED_NTCACHEMANAGER_ERROR: u32 = 0x1BB;
pub const ERROR_LINUX_SUBSYSTEM_UPDATE_REQUIRED: u32 = 0x1BC;
pub const ERROR_DLP_POLICY_WARNS_AGAINST_OPERATION: u32 = 0x1BD;
pub const ERROR_DLP_POLICY_DENIES_OPERATION: u32 = 0x1BE;
pub const ERROR_SECURITY_DENIES_OPERATION: u32 = 0x1BF;
pub const ERROR_UNTRUSTED_MOUNT_POINT: u32 = 0x1C0;
pub const ERROR_DLP_POLICY_SILENTLY_FAIL: u32 = 0x1C1;
pub const ERROR_CAPAUTHZ_NOT_DEVUNLOCKED: u32 = 0x1C2;
pub const ERROR_CAPAUTHZ_CHANGE_TYPE: u32 = 0x1C3;
pub const ERROR_CAPAUTHZ_NOT_PROVISIONED: u32 = 0x1C4;
pub const ERROR_CAPAUTHZ_NOT_AUTHORIZED: u32 = 0x1C5;
pub const ERROR_CAPAUTHZ_NO_POLICY: u32 = 0x1C6;
pub const ERROR_CAPAUTHZ_DB_CORRUPTED: u32 = 0x1C7;
pub const ERROR_CAPAUTHZ_SCCD_INVALID_CATALOG: u32 = 0x1C8;
pub const ERROR_CAPAUTHZ_SCCD_NO_AUTH_ENTITY: u32 = 0x1C9;
pub const ERROR_CAPAUTHZ_SCCD_PARSE_ERROR: u32 = 0x1CA;
pub const ERROR_CAPAUTHZ_SCCD_DEV_MODE_REQUIRED: u32 = 0x1CB;
pub const ERROR_CAPAUTHZ_SCCD_NO_CAPABILITY_MATCH: u32 = 0x1CC;
pub const ERROR_CIMFS_IMAGE_CORRUPT: u32 = 0x1D6;
pub const ERROR_CIMFS_IMAGE_VERSION_NOT_SUPPORTED: u32 = 0x1D7;
pub const ERROR_STORAGE_STACK_ACCESS_DENIED: u32 = 0x1D8;
pub const ERROR_INSUFFICIENT_VIRTUAL_ADDR_RESOURCES: u32 = 0x1D9;
pub const ERROR_INDEX_OUT_OF_BOUNDS: u32 = 0x1DA;
pub const ERROR_PNP_QUERY_REMOVE_DEVICE_TIMEOUT: u32 = 0x1E0;
pub const ERROR_PNP_QUERY_REMOVE_RELATED_DEVICE_TIMEOUT: u32 = 0x1E1;
pub const ERROR_PNP_QUERY_REMOVE_UNRELATED_DEVICE_TIMEOUT: u32 = 0x1E2;
pub const ERROR_DEVICE_HARDWARE_ERROR: u32 = 0x1E3;
pub const ERROR_INVALID_ADDRESS: u32 = 0x1E7;
pub const ERROR_HAS_SYSTEM_CRITICAL_FILES: u32 = 0x1E8;
pub const ERROR_ENCRYPTED_FILE_NOT_SUPPORTED: u32 = 0x1E9;
pub const ERROR_SPARSE_FILE_NOT_SUPPORTED: u32 = 0x1EA;
pub const ERROR_PAGEFILE_NOT_SUPPORTED: u32 = 0x1EB;
pub const ERROR_VOLUME_NOT_SUPPORTED: u32 = 0x1EC;
pub const ERROR_NOT_SUPPORTED_WITH_BYPASSIO: u32 = 0x1ED;
pub const ERROR_NO_BYPASSIO_DRIVER_SUPPORT: u32 = 0x1EE;
pub const ERROR_NOT_SUPPORTED_WITH_ENCRYPTION: u32 = 0x1EF;
pub const ERROR_NOT_SUPPORTED_WITH_COMPRESSION: u32 = 0x1F0;
pub const ERROR_NOT_SUPPORTED_WITH_REPLICATION: u32 = 0x1F1;
pub const ERROR_NOT_SUPPORTED_WITH_DEDUPLICATION: u32 = 0x1F2;
pub const ERROR_NOT_SUPPORTED_WITH_AUDITING: u32 = 0x1F3;
pub const ERROR_USER_PROFILE_LOAD: u32 = 0x1F4;
pub const ERROR_SESSION_KEY_TOO_SHORT: u32 = 0x1F5;
pub const ERROR_ACCESS_DENIED_APPDATA: u32 = 0x1F6;
pub const ERROR_NOT_SUPPORTED_WITH_MONITORING: u32 = 0x1F7;
pub const ERROR_NOT_SUPPORTED_WITH_SNAPSHOT: u32 = 0x1F8;
pub const ERROR_NOT_SUPPORTED_WITH_VIRTUALIZATION: u32 = 0x1F9;
pub const ERROR_BYPASSIO_FLT_NOT_SUPPORTED: u32 = 0x1FA;
pub const ERROR_DEVICE_RESET_REQUIRED: u32 = 0x1FB;
pub const ERROR_VOLUME_WRITE_ACCESS_DENIED: u32 = 0x1FC;
pub const ERROR_ARITHMETIC_OVERFLOW: u32 = 0x216;
pub const ERROR_PIPE_CONNECTED: u32 = 0x217;
pub const ERROR_PIPE_LISTENING: u32 = 0x218;
pub const ERROR_VERIFIER_STOP: u32 = 0x219;
pub const ERROR_ABIOS_ERROR: u32 = 0x21A;
pub const ERROR_WX86_WARNING: u32 = 0x21B;
pub const ERROR_WX86_ERROR: u32 = 0x21C;
pub const ERROR_TIMER_NOT_CANCELED: u32 = 0x21D;
pub const ERROR_UNWIND: u32 = 0x21E;
pub const ERROR_BAD_STACK: u32 = 0x21F;
pub const ERROR_INVALID_UNWIND_TARGET: u32 = 0x220;
pub const ERROR_INVALID_PORT_ATTRIBUTES: u32 = 0x221;
pub const ERROR_PORT_MESSAGE_TOO_LONG: u32 = 0x222;
pub const ERROR_INVALID_QUOTA_LOWER: u32 = 0x223;
pub const ERROR_DEVICE_ALREADY_ATTACHED: u32 = 0x224;
pub const ERROR_INSTRUCTION_MISALIGNMENT: u32 = 0x225;
pub const ERROR_PROFILING_NOT_STARTED: u32 = 0x226;
pub const ERROR_PROFILING_NOT_STOPPED: u32 = 0x227;
pub const ERROR_COULD_NOT_INTERPRET: u32 = 0x228;
pub const ERROR_PROFILING_AT_LIMIT: u32 = 0x229;
pub const ERROR_CANT_WAIT: u32 = 0x22A;
pub const ERROR_CANT_TERMINATE_SELF: u32 = 0x22B;
pub const ERROR_UNEXPECTED_MM_CREATE_ERR: u32 = 0x22C;
pub const ERROR_UNEXPECTED_MM_MAP_ERROR: u32 = 0x22D;
pub const ERROR_UNEXPECTED_MM_EXTEND_ERR: u32 = 0x22E;
pub const ERROR_BAD_FUNCTION_TABLE: u32 = 0x22F;
pub const ERROR_NO_GUID_TRANSLATION: u32 = 0x230;
pub const ERROR_INVALID_LDT_SIZE: u32 = 0x231;
pub const ERROR_INVALID_LDT_OFFSET: u32 = 0x233;
pub const ERROR_INVALID_LDT_DESCRIPTOR: u32 = 0x234;
pub const ERROR_TOO_MANY_THREADS: u32 = 0x235;
pub const ERROR_THREAD_NOT_IN_PROCESS: u32 = 0x236;
pub const ERROR_PAGEFILE_QUOTA_EXCEEDED: u32 = 0x237;
pub const ERROR_LOGON_SERVER_CONFLICT: u32 = 0x238;
pub const ERROR_SYNCHRONIZATION_REQUIRED: u32 = 0x239;
pub const ERROR_NET_OPEN_FAILED: u32 = 0x23A;
pub const ERROR_IO_PRIVILEGE_FAILED: u32 = 0x23B;
pub const ERROR_CONTROL_C_EXIT: u32 = 0x23C;
pub const ERROR_MISSING_SYSTEMFILE: u32 = 0x23D;
pub const ERROR_UNHANDLED_EXCEPTION: u32 = 0x23E;
pub const ERROR_APP_INIT_FAILURE: u32 = 0x23F;
pub const ERROR_PAGEFILE_CREATE_FAILED: u32 = 0x240;
pub const ERROR_INVALID_IMAGE_HASH: u32 = 0x241;
pub const ERROR_NO_PAGEFILE: u32 = 0x242;
pub const ERROR_ILLEGAL_FLOAT_CONTEXT: u32 = 0x243;
pub const ERROR_NO_EVENT_PAIR: u32 = 0x244;
pub const ERROR_DOMAIN_CTRLR_CONFIG_ERROR: u32 = 0x245;
pub const ERROR_ILLEGAL_CHARACTER: u32 = 0x246;
pub const ERROR_UNDEFINED_CHARACTER: u32 = 0x247;
pub const ERROR_FLOPPY_VOLUME: u32 = 0x248;
pub const ERROR_BIOS_FAILED_TO_CONNECT_INTERRUPT: u32 = 0x249;
pub const ERROR_BACKUP_CONTROLLER: u32 = 0x24A;
pub const ERROR_MUTANT_LIMIT_EXCEEDED: u32 = 0x24B;
pub const ERROR_FS_DRIVER_REQUIRED: u32 = 0x24C;
pub const ERROR_CANNOT_LOAD_REGISTRY_FILE: u32 = 0x24D;
pub const ERROR_DEBUG_ATTACH_FAILED: u32 = 0x24E;
pub const ERROR_SYSTEM_PROCESS_TERMINATED: u32 = 0x24F;
pub const ERROR_DATA_NOT_ACCEPTED: u32 = 0x250;
pub const ERROR_VDM_HARD_ERROR: u32 = 0x251;
pub const ERROR_DRIVER_CANCEL_TIMEOUT: u32 = 0x252;
pub const ERROR_REPLY_MESSAGE_MISMATCH: u32 = 0x253;
pub const ERROR_LOST_WRITEBEHIND_DATA: u32 = 0x254;
pub const ERROR_CLIENT_SERVER_PARAMETERS_INVALID: u32 = 0x255;
pub const ERROR_NOT_TINY_STREAM: u32 = 0x256;
pub const ERROR_STACK_OVERFLOW_READ: u32 = 0x257;
pub const ERROR_CONVERT_TO_LARGE: u32 = 0x258;
pub const ERROR_FOUND_OUT_OF_SCOPE: u32 = 0x259;
pub const ERROR_ALLOCATE_BUCKET: u32 = 0x25A;
pub const ERROR_MARSHALL_OVERFLOW: u32 = 0x25B;
pub const ERROR_INVALID_VARIANT: u32 = 0x25C;
pub const ERROR_BAD_COMPRESSION_BUFFER: u32 = 0x25D;
pub const ERROR_AUDIT_FAILED: u32 = 0x25E;
pub const ERROR_TIMER_RESOLUTION_NOT_SET: u32 = 0x25F;
pub const ERROR_INSUFFICIENT_LOGON_INFO: u32 = 0x260;
pub const ERROR_BAD_DLL_ENTRYPOINT: u32 = 0x261;
pub const ERROR_BAD_SERVICE_ENTRYPOINT: u32 = 0x262;
pub const ERROR_IP_ADDRESS_CONFLICT1: u32 = 0x263;
pub const ERROR_IP_ADDRESS_CONFLICT2: u32 = 0x264;
pub const ERROR_REGISTRY_QUOTA_LIMIT: u32 = 0x265;
pub const ERROR_NO_CALLBACK_ACTIVE: u32 = 0x266;
pub const ERROR_PWD_TOO_SHORT: u32 = 0x267;
pub const ERROR_PWD_TOO_RECENT: u32 = 0x268;
pub const ERROR_PWD_HISTORY_CONFLICT: u32 = 0x269;
pub const ERROR_UNSUPPORTED_COMPRESSION: u32 = 0x26A;
pub const ERROR_INVALID_HW_PROFILE: u32 = 0x26B;
pub const ERROR_INVALID_PLUGPLAY_DEVICE_PATH: u32 = 0x26C;
pub const ERROR_QUOTA_LIST_INCONSISTENT: u32 = 0x26D;
pub const ERROR_EVALUATION_EXPIRATION: u32 = 0x26E;
pub const ERROR_ILLEGAL_DLL_RELOCATION: u32 = 0x26F;
pub const ERROR_DLL_INIT_FAILED_LOGOFF: u32 = 0x270;
pub const ERROR_VALIDATE_CONTINUE: u32 = 0x271;
pub const ERROR_NO_MORE_MATCHES: u32 = 0x272;
pub const ERROR_RANGE_LIST_CONFLICT: u32 = 0x273;
pub const ERROR_SERVER_SID_MISMATCH: u32 = 0x274;
pub const ERROR_CANT_ENABLE_DENY_ONLY: u32 = 0x275;
pub const ERROR_FLOAT_MULTIPLE_FAULTS: u32 = 0x276;
pub const ERROR_FLOAT_MULTIPLE_TRAPS: u32 = 0x277;
pub const ERROR_NOINTERFACE: u32 = 0x278;
pub const ERROR_DRIVER_FAILED_SLEEP: u32 = 0x279;
pub const ERROR_CORRUPT_SYSTEM_FILE: u32 = 0x27A;
pub const ERROR_COMMITMENT_MINIMUM: u32 = 0x27B;
pub const ERROR_PNP_RESTART_ENUMERATION: u32 = 0x27C;
pub const ERROR_SYSTEM_IMAGE_BAD_SIGNATURE: u32 = 0x27D;
pub const ERROR_PNP_REBOOT_REQUIRED: u32 = 0x27E;
pub const ERROR_INSUFFICIENT_POWER: u32 = 0x27F;
pub const ERROR_MULTIPLE_FAULT_VIOLATION: u32 = 0x280;
pub const ERROR_SYSTEM_SHUTDOWN: u32 = 0x281;
pub const ERROR_PORT_NOT_SET: u32 = 0x282;
pub const ERROR_DS_VERSION_CHECK_FAILURE: u32 = 0x283;
pub const ERROR_RANGE_NOT_FOUND: u32 = 0x284;
pub const ERROR_NOT_SAFE_MODE_DRIVER: u32 = 0x286;
pub const ERROR_FAILED_DRIVER_ENTRY: u32 = 0x287;
pub const ERROR_DEVICE_ENUMERATION_ERROR: u32 = 0x288;
pub const ERROR_MOUNT_POINT_NOT_RESOLVED: u32 = 0x289;
pub const ERROR_INVALID_DEVICE_OBJECT_PARAMETER: u32 = 0x28A;
pub const ERROR_MCA_OCCURED: u32 = 0x28B;
pub const ERROR_DRIVER_DATABASE_ERROR: u32 = 0x28C;
pub const ERROR_SYSTEM_HIVE_TOO_LARGE: u32 = 0x28D;
pub const ERROR_DRIVER_FAILED_PRIOR_UNLOAD: u32 = 0x28E;
pub const ERROR_VOLSNAP_PREPARE_HIBERNATE: u32 = 0x28F;
pub const ERROR_HIBERNATION_FAILURE: u32 = 0x290;
pub const ERROR_PWD_TOO_LONG: u32 = 0x291;
pub const ERROR_FILE_SYSTEM_LIMITATION: u32 = 0x299;
pub const ERROR_ASSERTION_FAILURE: u32 = 0x29C;
pub const ERROR_ACPI_ERROR: u32 = 0x29D;
pub const ERROR_WOW_ASSERTION: u32 = 0x29E;
pub const ERROR_PNP_BAD_MPS_TABLE: u32 = 0x29F;
pub const ERROR_PNP_TRANSLATION_FAILED: u32 = 0x2A0;
pub const ERROR_PNP_IRQ_TRANSLATION_FAILED: u32 = 0x2A1;
pub const ERROR_PNP_INVALID_ID: u32 = 0x2A2;
pub const ERROR_WAKE_SYSTEM_DEBUGGER: u32 = 0x2A3;
pub const ERROR_HANDLES_CLOSED: u32 = 0x2A4;
pub const ERROR_EXTRANEOUS_INFORMATION: u32 = 0x2A5;
pub const ERROR_RXACT_COMMIT_NECESSARY: u32 = 0x2A6;
pub const ERROR_MEDIA_CHECK: u32 = 0x2A7;
pub const ERROR_GUID_SUBSTITUTION_MADE: u32 = 0x2A8;
pub const ERROR_STOPPED_ON_SYMLINK: u32 = 0x2A9;
pub const ERROR_LONGJUMP: u32 = 0x2AA;
pub const ERROR_PLUGPLAY_QUERY_VETOED: u32 = 0x2AB;
pub const ERROR_UNWIND_CONSOLIDATE: u32 = 0x2AC;
pub const ERROR_REGISTRY_HIVE_RECOVERED: u32 = 0x2AD;
pub const ERROR_DLL_MIGHT_BE_INSECURE: u32 = 0x2AE;
pub const ERROR_DLL_MIGHT_BE_INCOMPATIBLE: u32 = 0x2AF;
pub const ERROR_DBG_EXCEPTION_NOT_HANDLED: u32 = 0x2B0;
pub const ERROR_DBG_REPLY_LATER: u32 = 0x2B1;
pub const ERROR_DBG_UNABLE_TO_PROVIDE_HANDLE: u32 = 0x2B2;
pub const ERROR_DBG_TERMINATE_THREAD: u32 = 0x2B3;
pub const ERROR_DBG_TERMINATE_PROCESS: u32 = 0x2B4;
pub const ERROR_DBG_CONTROL_C: u32 = 0x2B5;
pub const ERROR_DBG_PRINTEXCEPTION_C: u32 = 0x2B6;
pub const ERROR_DBG_RIPEXCEPTION: u32 = 0x2B7;
pub const ERROR_DBG_CONTROL_BREAK: u32 = 0x2B8;
pub const ERROR_DBG_COMMAND_EXCEPTION: u32 = 0x2B9;
pub const ERROR_OBJECT_NAME_EXISTS: u32 = 0x2BA;
pub const ERROR_THREAD_WAS_SUSPENDED: u32 = 0x2BB;
pub const ERROR_IMAGE_NOT_AT_BASE: u32 = 0x2BC;
pub const ERROR_RXACT_STATE_CREATED: u32 = 0x2BD;
pub const ERROR_SEGMENT_NOTIFICATION: u32 = 0x2BE;
pub const ERROR_BAD_CURRENT_DIRECTORY: u32 = 0x2BF;
pub const ERROR_FT_READ_RECOVERY_FROM_BACKUP: u32 = 0x2C0;
pub const ERROR_FT_WRITE_RECOVERY: u32 = 0x2C1;
pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH: u32 = 0x2C2;
pub const ERROR_RECEIVE_PARTIAL: u32 = 0x2C3;
pub const ERROR_RECEIVE_EXPEDITED: u32 = 0x2C4;
pub const ERROR_RECEIVE_PARTIAL_EXPEDITED: u32 = 0x2C5;
pub const ERROR_EVENT_DONE: u32 = 0x2C6;
pub const ERROR_EVENT_PENDING: u32 = 0x2C7;
pub const ERROR_CHECKING_FILE_SYSTEM: u32 = 0x2C8;
pub const ERROR_FATAL_APP_EXIT: u32 = 0x2C9;
pub const ERROR_PREDEFINED_HANDLE: u32 = 0x2CA;
pub const ERROR_WAS_UNLOCKED: u32 = 0x2CB;
pub const ERROR_SERVICE_NOTIFICATION: u32 = 0x2CC;
pub const ERROR_WAS_LOCKED: u32 = 0x2CD;
pub const ERROR_LOG_HARD_ERROR: u32 = 0x2CE;
pub const ERROR_ALREADY_WIN32: u32 = 0x2CF;
pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH_EXE: u32 = 0x2D0;
pub const ERROR_NO_YIELD_PERFORMED: u32 = 0x2D1;
pub const ERROR_TIMER_RESUME_IGNORED: u32 = 0x2D2;
pub const ERROR_ARBITRATION_UNHANDLED: u32 = 0x2D3;
pub const ERROR_CARDBUS_NOT_SUPPORTED: u32 = 0x2D4;
pub const ERROR_MP_PROCESSOR_MISMATCH: u32 = 0x2D5;
pub const ERROR_HIBERNATED: u32 = 0x2D6;
pub const ERROR_RESUME_HIBERNATION: u32 = 0x2D7;
pub const ERROR_FIRMWARE_UPDATED: u32 = 0x2D8;
pub const ERROR_DRIVERS_LEAKING_LOCKED_PAGES: u32 = 0x2D9;
pub const ERROR_WAKE_SYSTEM: u32 = 0x2DA;
pub const ERROR_WAIT_1: u32 = 0x2DB;
pub const ERROR_WAIT_2: u32 = 0x2DC;
pub const ERROR_WAIT_3: u32 = 0x2DD;
pub const ERROR_WAIT_63: u32 = 0x2DE;
pub const ERROR_ABANDONED_WAIT_0: u32 = 0x2DF;
pub const ERROR_ABANDONED_WAIT_63: u32 = 0x2E0;
pub const ERROR_USER_APC: u32 = 0x2E1;
pub const ERROR_KERNEL_APC: u32 = 0x2E2;
pub const ERROR_ALERTED: u32 = 0x2E3;
pub const ERROR_ELEVATION_REQUIRED: u32 = 0x2E4;
pub const ERROR_REPARSE: u32 = 0x2E5;
pub const ERROR_OPLOCK_BREAK_IN_PROGRESS: u32 = 0x2E6;
pub const ERROR_VOLUME_MOUNTED: u32 = 0x2E7;
pub const ERROR_RXACT_COMMITTED: u32 = 0x2E8;
pub const ERROR_NOTIFY_CLEANUP: u32 = 0x2E9;
pub const ERROR_PRIMARY_TRANSPORT_CONNECT_FAILED: u32 = 0x2EA;
pub const ERROR_PAGE_FAULT_TRANSITION: u32 = 0x2EB;
pub const ERROR_PAGE_FAULT_DEMAND_ZERO: u32 = 0x2EC;
pub const ERROR_PAGE_FAULT_COPY_ON_WRITE: u32 = 0x2ED;
pub const ERROR_PAGE_FAULT_GUARD_PAGE: u32 = 0x2EE;
pub const ERROR_PAGE_FAULT_PAGING_FILE: u32 = 0x2EF;
pub const ERROR_CACHE_PAGE_LOCKED: u32 = 0x2F0;
pub const ERROR_CRASH_DUMP: u32 = 0x2F1;
pub const ERROR_BUFFER_ALL_ZEROS: u32 = 0x2F2;
pub const ERROR_REPARSE_OBJECT: u32 = 0x2F3;
pub const ERROR_RESOURCE_REQUIREMENTS_CHANGED: u32 = 0x2F4;
pub const ERROR_TRANSLATION_COMPLETE: u32 = 0x2F5;
pub const ERROR_NOTHING_TO_TERMINATE: u32 = 0x2F6;
pub const ERROR_PROCESS_NOT_IN_JOB: u32 = 0x2F7;
pub const ERROR_PROCESS_IN_JOB: u32 = 0x2F8;
pub const ERROR_VOLSNAP_HIBERNATE_READY: u32 = 0x2F9;
pub const ERROR_FSFILTER_OP_COMPLETED_SUCCESSFULLY: u32 = 0x2FA;
pub const ERROR_INTERRUPT_VECTOR_ALREADY_CONNECTED: u32 = 0x2FB;
pub const ERROR_INTERRUPT_STILL_CONNECTED: u32 = 0x2FC;
pub const ERROR_WAIT_FOR_OPLOCK: u32 = 0x2FD;
pub const ERROR_DBG_EXCEPTION_HANDLED: u32 = 0x2FE;
pub const ERROR_DBG_CONTINUE: u32 = 0x2FF;
pub const ERROR_CALLBACK_POP_STACK: u32 = 0x300;
pub const ERROR_COMPRESSION_DISABLED: u32 = 0x301;
pub const ERROR_CANTFETCHBACKWARDS: u32 = 0x302;
pub const ERROR_CANTSCROLLBACKWARDS: u32 = 0x303;
pub const ERROR_ROWSNOTRELEASED: u32 = 0x304;
pub const ERROR_BAD_ACCESSOR_FLAGS: u32 = 0x305;
pub const ERROR_ERRORS_ENCOUNTERED: u32 = 0x306;
pub const ERROR_NOT_CAPABLE: u32 = 0x307;
pub const ERROR_REQUEST_OUT_OF_SEQUENCE: u32 = 0x308;
pub const ERROR_VERSION_PARSE_ERROR: u32 = 0x309;
pub const ERROR_BADSTARTPOSITION: u32 = 0x30A;
pub const ERROR_MEMORY_HARDWARE: u32 = 0x30B;
pub const ERROR_DISK_REPAIR_DISABLED: u32 = 0x30C;
pub const ERROR_INSUFFICIENT_RESOURCE_FOR_SPECIFIED_SHARED_SECTION_SIZE: u32 = 0x30D;
pub const ERROR_SYSTEM_POWERSTATE_TRANSITION: u32 = 0x30E;
pub const ERROR_SYSTEM_POWERSTATE_COMPLEX_TRANSITION: u32 = 0x30F;
pub const ERROR_MCA_EXCEPTION: u32 = 0x310;
pub const ERROR_ACCESS_AUDIT_BY_POLICY: u32 = 0x311;
pub const ERROR_ACCESS_DISABLED_NO_SAFER_UI_BY_POLICY: u32 = 0x312;
pub const ERROR_ABANDON_HIBERFILE: u32 = 0x313;
pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_DISCONNECTED: u32 = 0x314;
pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_SERVER_ERROR: u32 = 0x315;
pub const ERROR_LOST_WRITEBEHIND_DATA_LOCAL_DISK_ERROR: u32 = 0x316;
pub const ERROR_BAD_MCFG_TABLE: u32 = 0x317;
pub const ERROR_DISK_REPAIR_REDIRECTED: u32 = 0x318;
pub const ERROR_DISK_REPAIR_UNSUCCESSFUL: u32 = 0x319;
pub const ERROR_CORRUPT_LOG_OVERFULL: u32 = 0x31A;
pub const ERROR_CORRUPT_LOG_CORRUPTED: u32 = 0x31B;
pub const ERROR_CORRUPT_LOG_UNAVAILABLE: u32 = 0x31C;
pub const ERROR_CORRUPT_LOG_DELETED_FULL: u32 = 0x31D;
pub const ERROR_CORRUPT_LOG_CLEARED: u32 = 0x31E;
pub const ERROR_ORPHAN_NAME_EXHAUSTED: u32 = 0x31F;
pub const ERROR_OPLOCK_SWITCHED_TO_NEW_HANDLE: u32 = 0x320;
pub const ERROR_CANNOT_GRANT_REQUESTED_OPLOCK: u32 = 0x321;
pub const ERROR_CANNOT_BREAK_OPLOCK: u32 = 0x322;
pub const ERROR_OPLOCK_HANDLE_CLOSED: u32 = 0x323;
pub const ERROR_NO_ACE_CONDITION: u32 = 0x324;
pub const ERROR_INVALID_ACE_CONDITION: u32 = 0x325;
pub const ERROR_FILE_HANDLE_REVOKED: u32 = 0x326;
pub const ERROR_IMAGE_AT_DIFFERENT_BASE: u32 = 0x327;
pub const ERROR_ENCRYPTED_IO_NOT_POSSIBLE: u32 = 0x328;
pub const ERROR_FILE_METADATA_OPTIMIZATION_IN_PROGRESS: u32 = 0x329;
pub const ERROR_QUOTA_ACTIVITY: u32 = 0x32A;
pub const ERROR_HANDLE_REVOKED: u32 = 0x32B;
pub const ERROR_CALLBACK_INVOKE_INLINE: u32 = 0x32C;
pub const ERROR_CPU_SET_INVALID: u32 = 0x32D;
pub const ERROR_ENCLAVE_NOT_TERMINATED: u32 = 0x32E;
pub const ERROR_ENCLAVE_VIOLATION: u32 = 0x32F;
pub const ERROR_SERVER_TRANSPORT_CONFLICT: u32 = 0x330;
pub const ERROR_CERTIFICATE_VALIDATION_PREFERENCE_CONFLICT: u32 = 0x331;
pub const ERROR_FT_READ_FROM_COPY_FAILURE: u32 = 0x332;
pub const ERROR_SECTION_DIRECT_MAP_ONLY: u32 = 0x333;
pub const ERROR_EA_ACCESS_DENIED: u32 = 0x3E2;
pub const ERROR_OPERATION_ABORTED: u32 = 0x3E3;
pub const ERROR_IO_INCOMPLETE: u32 = 0x3E4;
pub const ERROR_IO_PENDING: u32 = 0x3E5;
pub const ERROR_NOACCESS: u32 = 0x3E6;
pub const ERROR_SWAPERROR: u32 = 0x3E7;
pub const ERROR_STACK_OVERFLOW: u32 = 0x3E9;
pub const ERROR_INVALID_MESSAGE: u32 = 0x3EA;
pub const ERROR_CAN_NOT_COMPLETE: u32 = 0x3EB;
pub const ERROR_INVALID_FLAGS: u32 = 0x3EC;
pub const ERROR_UNRECOGNIZED_VOLUME: u32 = 0x3ED;
pub const ERROR_FILE_INVALID: u32 = 0x3EE;
pub const ERROR_FULLSCREEN_MODE: u32 = 0x3EF;
pub const ERROR_NO_TOKEN: u32 = 0x3F0;
pub const ERROR_BADDB: u32 = 0x3F1;
pub const ERROR_BADKEY: u32 = 0x3F2;
pub const ERROR_CANTOPEN: u32 = 0x3F3;
pub const ERROR_CANTREAD: u32 = 0x3F4;
pub const ERROR_CANTWRITE: u32 = 0x3F5;
pub const ERROR_REGISTRY_RECOVERED: u32 = 0x3F6;
pub const ERROR_REGISTRY_CORRUPT: u32 = 0x3F7;
pub const ERROR_REGISTRY_IO_FAILED: u32 = 0x3F8;
pub const ERROR_NOT_REGISTRY_FILE: u32 = 0x3F9;
pub const ERROR_KEY_DELETED: u32 = 0x3FA;
pub const ERROR_NO_LOG_SPACE: u32 = 0x3FB;
pub const ERROR_KEY_HAS_CHILDREN: u32 = 0x3FC;
pub const ERROR_CHILD_MUST_BE_VOLATILE: u32 = 0x3FD;
pub const ERROR_NOTIFY_ENUM_DIR: u32 = 0x3FE;
pub const ERROR_DEPENDENT_SERVICES_RUNNING: u32 = 0x41B;
pub const ERROR_INVALID_SERVICE_CONTROL: u32 = 0x41C;
pub const ERROR_SERVICE_REQUEST_TIMEOUT: u32 = 0x41D;
pub const ERROR_SERVICE_NO_THREAD: u32 = 0x41E;
pub const ERROR_SERVICE_DATABASE_LOCKED: u32 = 0x41F;
pub const ERROR_SERVICE_ALREADY_RUNNING: u32 = 0x420;
pub const ERROR_INVALID_SERVICE_ACCOUNT: u32 = 0x421;
pub const ERROR_SERVICE_DISABLED: u32 = 0x422;
pub const ERROR_CIRCULAR_DEPENDENCY: u32 = 0x423;
pub const ERROR_SERVICE_DOES_NOT_EXIST: u32 = 0x424;
pub const ERROR_SERVICE_CANNOT_ACCEPT_CTRL: u32 = 0x425;
pub const ERROR_SERVICE_NOT_ACTIVE: u32 = 0x426;
pub const ERROR_FAILED_SERVICE_CONTROLLER_CONNECT: u32 = 0x427;
pub const ERROR_EXCEPTION_IN_SERVICE: u32 = 0x428;
pub const ERROR_DATABASE_DOES_NOT_EXIST: u32 = 0x429;
pub const ERROR_SERVICE_SPECIFIC_ERROR: u32 = 0x42A;
pub const ERROR_PROCESS_ABORTED: u32 = 0x42B;
pub const ERROR_SERVICE_DEPENDENCY_FAIL: u32 = 0x42C;
pub const ERROR_SERVICE_LOGON_FAILED: u32 = 0x42D;
pub const ERROR_SERVICE_START_HANG: u32 = 0x42E;
pub const ERROR_INVALID_SERVICE_LOCK: u32 = 0x42F;
pub const ERROR_SERVICE_MARKED_FOR_DELETE: u32 = 0x430;
pub const ERROR_SERVICE_EXISTS: u32 = 0x431;
pub const ERROR_ALREADY_RUNNING_LKG: u32 = 0x432;
pub const ERROR_SERVICE_DEPENDENCY_DELETED: u32 = 0x433;
pub const ERROR_BOOT_ALREADY_ACCEPTED: u32 = 0x434;
pub const ERROR_SERVICE_NEVER_STARTED: u32 = 0x435;
pub const ERROR_DUPLICATE_SERVICE_NAME: u32 = 0x436;
pub const ERROR_DIFFERENT_SERVICE_ACCOUNT: u32 = 0x437;
pub const ERROR_CANNOT_DETECT_DRIVER_FAILURE: u32 = 0x438;
pub const ERROR_CANNOT_DETECT_PROCESS_ABORT: u32 = 0x439;
pub const ERROR_NO_RECOVERY_PROGRAM: u32 = 0x43A;
pub const ERROR_SERVICE_NOT_IN_EXE: u32 = 0x43B;
pub const ERROR_NOT_SAFEBOOT_SERVICE: u32 = 0x43C;
pub const ERROR_END_OF_MEDIA: u32 = 0x44C;
pub const ERROR_FILEMARK_DETECTED: u32 = 0x44D;
pub const ERROR_BEGINNING_OF_MEDIA: u32 = 0x44E;
pub const ERROR_SETMARK_DETECTED: u32 = 0x44F;
pub const ERROR_NO_DATA_DETECTED: u32 = 0x450;
pub const ERROR_PARTITION_FAILURE: u32 = 0x451;
pub const ERROR_INVALID_BLOCK_LENGTH: u32 = 0x452;
pub const ERROR_DEVICE_NOT_PARTITIONED: u32 = 0x453;
pub const ERROR_UNABLE_TO_LOCK_MEDIA: u32 = 0x454;
pub const ERROR_UNABLE_TO_UNLOAD_MEDIA: u32 = 0x455;
pub const ERROR_MEDIA_CHANGED: u32 = 0x456;
pub const ERROR_BUS_RESET: u32 = 0x457;
pub const ERROR_NO_MEDIA_IN_DRIVE: u32 = 0x458;
pub const ERROR_NO_UNICODE_TRANSLATION: u32 = 0x459;
pub const ERROR_DLL_INIT_FAILED: u32 = 0x45A;
pub const ERROR_SHUTDOWN_IN_PROGRESS: u32 = 0x45B;
pub const ERROR_NO_SHUTDOWN_IN_PROGRESS: u32 = 0x45C;
pub const ERROR_IO_DEVICE: u32 = 0x45D;
pub const ERROR_SERIAL_NO_DEVICE: u32 = 0x45E;
pub const ERROR_IRQ_BUSY: u32 = 0x45F;
pub const ERROR_MORE_WRITES: u32 = 0x460;
pub const ERROR_COUNTER_TIMEOUT: u32 = 0x461;
pub const ERROR_FLOPPY_ID_MARK_NOT_FOUND: u32 = 0x462;
pub const ERROR_FLOPPY_WRONG_CYLINDER: u32 = 0x463;
pub const ERROR_FLOPPY_UNKNOWN_ERROR: u32 = 0x464;
pub const ERROR_FLOPPY_BAD_REGISTERS: u32 = 0x465;
pub const ERROR_DISK_RECALIBRATE_FAILED: u32 = 0x466;
pub const ERROR_DISK_OPERATION_FAILED: u32 = 0x467;
pub const ERROR_DISK_RESET_FAILED: u32 = 0x468;
pub const ERROR_EOM_OVERFLOW: u32 = 0x469;
pub const ERROR_NOT_ENOUGH_SERVER_MEMORY: u32 = 0x46A;
pub const ERROR_POSSIBLE_DEADLOCK: u32 = 0x46B;
pub const ERROR_MAPPED_ALIGNMENT: u32 = 0x46C;
pub const ERROR_SET_POWER_STATE_VETOED: u32 = 0x474;
pub const ERROR_SET_POWER_STATE_FAILED: u32 = 0x475;
pub const ERROR_TOO_MANY_LINKS: u32 = 0x476;
pub const ERROR_OLD_WIN_VERSION: u32 = 0x47E;
pub const ERROR_APP_WRONG_OS: u32 = 0x47F;
pub const ERROR_SINGLE_INSTANCE_APP: u32 = 0x480;
pub const ERROR_RMODE_APP: u32 = 0x481;
pub const ERROR_INVALID_DLL: u32 = 0x482;
pub const ERROR_NO_ASSOCIATION: u32 = 0x483;
pub const ERROR_DDE_FAIL: u32 = 0x484;
pub const ERROR_DLL_NOT_FOUND: u32 = 0x485;
pub const ERROR_NO_MORE_USER_HANDLES: u32 = 0x486;
pub const ERROR_MESSAGE_SYNC_ONLY: u32 = 0x487;
pub const ERROR_SOURCE_ELEMENT_EMPTY: u32 = 0x488;
pub const ERROR_DESTINATION_ELEMENT_FULL: u32 = 0x489;
pub const ERROR_ILLEGAL_ELEMENT_ADDRESS: u32 = 0x48A;
pub const ERROR_MAGAZINE_NOT_PRESENT: u32 = 0x48B;
pub const ERROR_DEVICE_REINITIALIZATION_NEEDED: u32 = 0x48C;
pub const ERROR_DEVICE_REQUIRES_CLEANING: u32 = 0x48D;
pub const ERROR_DEVICE_DOOR_OPEN: u32 = 0x48E;
pub const ERROR_DEVICE_NOT_CONNECTED: u32 = 0x48F;
pub const ERROR_NOT_FOUND: u32 = 0x490;
pub const ERROR_NO_MATCH: u32 = 0x491;
pub const ERROR_SET_NOT_FOUND: u32 = 0x492;
pub const ERROR_POINT_NOT_FOUND: u32 = 0x493;
pub const ERROR_NO_TRACKING_SERVICE: u32 = 0x494;
pub const ERROR_NO_VOLUME_ID: u32 = 0x495;
pub const ERROR_UNABLE_TO_REMOVE_REPLACED: u32 = 0x497;
pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT: u32 = 0x498;
pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT_2: u32 = 0x499;
pub const ERROR_JOURNAL_DELETE_IN_PROGRESS: u32 = 0x49A;
pub const ERROR_JOURNAL_NOT_ACTIVE: u32 = 0x49B;
pub const ERROR_POTENTIAL_FILE_FOUND: u32 = 0x49C;
pub const ERROR_JOURNAL_ENTRY_DELETED: u32 = 0x49D;
pub const ERROR_PARTITION_TERMINATING: u32 = 0x4A0;
pub const ERROR_SHUTDOWN_IS_SCHEDULED: u32 = 0x4A6;
pub const ERROR_SHUTDOWN_USERS_LOGGED_ON: u32 = 0x4A7;
pub const ERROR_SHUTDOWN_DISKS_NOT_IN_MAINTENANCE_MODE: u32 = 0x4A8;
pub const ERROR_BAD_DEVICE: u32 = 0x4B0;
pub const ERROR_CONNECTION_UNAVAIL: u32 = 0x4B1;
pub const ERROR_DEVICE_ALREADY_REMEMBERED: u32 = 0x4B2;
pub const ERROR_NO_NET_OR_BAD_PATH: u32 = 0x4B3;
pub const ERROR_BAD_PROVIDER: u32 = 0x4B4;
pub const ERROR_CANNOT_OPEN_PROFILE: u32 = 0x4B5;
pub const ERROR_BAD_PROFILE: u32 = 0x4B6;
pub const ERROR_NOT_CONTAINER: u32 = 0x4B7;
pub const ERROR_EXTENDED_ERROR: u32 = 0x4B8;
pub const ERROR_INVALID_GROUPNAME: u32 = 0x4B9;
pub const ERROR_INVALID_COMPUTERNAME: u32 = 0x4BA;
pub const ERROR_INVALID_EVENTNAME: u32 = 0x4BB;
pub const ERROR_INVALID_DOMAINNAME: u32 = 0x4BC;
pub const ERROR_INVALID_SERVICENAME: u32 = 0x4BD;
pub const ERROR_INVALID_NETNAME: u32 = 0x4BE;
pub const ERROR_INVALID_SHARENAME: u32 = 0x4BF;
pub const ERROR_INVALID_PASSWORDNAME: u32 = 0x4C0;
pub const ERROR_INVALID_MESSAGENAME: u32 = 0x4C1;
pub const ERROR_INVALID_MESSAGEDEST: u32 = 0x4C2;
pub const ERROR_SESSION_CREDENTIAL_CONFLICT: u32 = 0x4C3;
pub const ERROR_REMOTE_SESSION_LIMIT_EXCEEDED: u32 = 0x4C4;
pub const ERROR_DUP_DOMAINNAME: u32 = 0x4C5;
pub const ERROR_NO_NETWORK: u32 = 0x4C6;
pub const ERROR_CANCELLED: u32 = 0x4C7;
pub const ERROR_USER_MAPPED_FILE: u32 = 0x4C8;
pub const ERROR_CONNECTION_REFUSED: u32 = 0x4C9;
pub const ERROR_GRACEFUL_DISCONNECT: u32 = 0x4CA;
pub const ERROR_ADDRESS_ALREADY_ASSOCIATED: u32 = 0x4CB;
pub const ERROR_ADDRESS_NOT_ASSOCIATED: u32 = 0x4CC;
pub const ERROR_CONNECTION_INVALID: u32 = 0x4CD;
pub const ERROR_CONNECTION_ACTIVE: u32 = 0x4CE;
pub const ERROR_NETWORK_UNREACHABLE: u32 = 0x4CF;
pub const ERROR_HOST_UNREACHABLE: u32 = 0x4D0;
pub const ERROR_PROTOCOL_UNREACHABLE: u32 = 0x4D1;
pub const ERROR_PORT_UNREACHABLE: u32 = 0x4D2;
pub const ERROR_REQUEST_ABORTED: u32 = 0x4D3;
pub const ERROR_CONNECTION_ABORTED: u32 = 0x4D4;
pub const ERROR_RETRY: u32 = 0x4D5;
pub const ERROR_CONNECTION_COUNT_LIMIT: u32 = 0x4D6;
pub const ERROR_LOGIN_TIME_RESTRICTION: u32 = 0x4D7;
pub const ERROR_LOGIN_WKSTA_RESTRICTION: u32 = 0x4D8;
pub const ERROR_INCORRECT_ADDRESS: u32 = 0x4D9;
pub const ERROR_ALREADY_REGISTERED: u32 = 0x4DA;
pub const ERROR_SERVICE_NOT_FOUND: u32 = 0x4DB;
pub const ERROR_NOT_AUTHENTICATED: u32 = 0x4DC;
pub const ERROR_NOT_LOGGED_ON: u32 = 0x4DD;
pub const ERROR_CONTINUE: u32 = 0x4DE;
pub const ERROR_ALREADY_INITIALIZED: u32 = 0x4DF;
pub const ERROR_NO_MORE_DEVICES: u32 = 0x4E0;
pub const ERROR_NO_SUCH_SITE: u32 = 0x4E1;
pub const ERROR_DOMAIN_CONTROLLER_EXISTS: u32 = 0x4E2;
pub const ERROR_ONLY_IF_CONNECTED: u32 = 0x4E3;
pub const ERROR_OVERRIDE_NOCHANGES: u32 = 0x4E4;
pub const ERROR_BAD_USER_PROFILE: u32 = 0x4E5;
pub const ERROR_NOT_SUPPORTED_ON_SBS: u32 = 0x4E6;
pub const ERROR_SERVER_SHUTDOWN_IN_PROGRESS: u32 = 0x4E7;
pub const ERROR_HOST_DOWN: u32 = 0x4E8;
pub const ERROR_NON_ACCOUNT_SID: u32 = 0x4E9;
pub const ERROR_NON_DOMAIN_SID: u32 = 0x4EA;
pub const ERROR_APPHELP_BLOCK: u32 = 0x4EB;
pub const ERROR_ACCESS_DISABLED_BY_POLICY: u32 = 0x4EC;
pub const ERROR_REG_NAT_CONSUMPTION: u32 = 0x4ED;
pub const ERROR_CSCSHARE_OFFLINE: u32 = 0x4EE;
pub const ERROR_PKINIT_FAILURE: u32 = 0x4EF;
pub const ERROR_SMARTCARD_SUBSYSTEM_FAILURE: u32 = 0x4F0;
pub const ERROR_DOWNGRADE_DETECTED: u32 = 0x4F1;
pub const ERROR_MACHINE_LOCKED: u32 = 0x4F7;
pub const ERROR_SMB_GUEST_LOGON_BLOCKED: u32 = 0x4F8;
pub const ERROR_CALLBACK_SUPPLIED_INVALID_DATA: u32 = 0x4F9;
pub const ERROR_SYNC_FOREGROUND_REFRESH_REQUIRED: u32 = 0x4FA;
pub const ERROR_DRIVER_BLOCKED: u32 = 0x4FB;
pub const ERROR_INVALID_IMPORT_OF_NON_DLL: u32 = 0x4FC;
pub const ERROR_ACCESS_DISABLED_WEBBLADE: u32 = 0x4FD;
pub const ERROR_ACCESS_DISABLED_WEBBLADE_TAMPER: u32 = 0x4FE;
pub const ERROR_RECOVERY_FAILURE: u32 = 0x4FF;
pub const ERROR_ALREADY_FIBER: u32 = 0x500;
pub const ERROR_ALREADY_THREAD: u32 = 0x501;
pub const ERROR_STACK_BUFFER_OVERRUN: u32 = 0x502;
pub const ERROR_PARAMETER_QUOTA_EXCEEDED: u32 = 0x503;
pub const ERROR_DEBUGGER_INACTIVE: u32 = 0x504;
pub const ERROR_DELAY_LOAD_FAILED: u32 = 0x505;
pub const ERROR_VDM_DISALLOWED: u32 = 0x506;
pub const ERROR_UNIDENTIFIED_ERROR: u32 = 0x507;
pub const ERROR_INVALID_CRUNTIME_PARAMETER: u32 = 0x508;
pub const ERROR_BEYOND_VDL: u32 = 0x509;
pub const ERROR_INCOMPATIBLE_SERVICE_SID_TYPE: u32 = 0x50A;
pub const ERROR_DRIVER_PROCESS_TERMINATED: u32 = 0x50B;
pub const ERROR_IMPLEMENTATION_LIMIT: u32 = 0x50C;
pub const ERROR_PROCESS_IS_PROTECTED: u32 = 0x50D;
pub const ERROR_SERVICE_NOTIFY_CLIENT_LAGGING: u32 = 0x50E;
pub const ERROR_DISK_QUOTA_EXCEEDED: u32 = 0x50F;
pub const ERROR_CONTENT_BLOCKED: u32 = 0x510;
pub const ERROR_INCOMPATIBLE_SERVICE_PRIVILEGE: u32 = 0x511;
pub const ERROR_APP_HANG: u32 = 0x512;
pub const ERROR_INVALID_LABEL: u32 = 0x513;
pub const ERROR_NOT_ALL_ASSIGNED: u32 = 0x514;
pub const ERROR_SOME_NOT_MAPPED: u32 = 0x515;
pub const ERROR_NO_QUOTAS_FOR_ACCOUNT: u32 = 0x516;
pub const ERROR_LOCAL_USER_SESSION_KEY: u32 = 0x517;
pub const ERROR_NULL_LM_PASSWORD: u32 = 0x518;
pub const ERROR_UNKNOWN_REVISION: u32 = 0x519;
pub const ERROR_REVISION_MISMATCH: u32 = 0x51A;
pub const ERROR_INVALID_OWNER: u32 = 0x51B;
pub const ERROR_INVALID_PRIMARY_GROUP: u32 = 0x51C;
pub const ERROR_NO_IMPERSONATION_TOKEN: u32 = 0x51D;
pub const ERROR_CANT_DISABLE_MANDATORY: u32 = 0x51E;
pub const ERROR_NO_LOGON_SERVERS: u32 = 0x51F;
pub const ERROR_NO_SUCH_LOGON_SESSION: u32 = 0x520;
pub const ERROR_NO_SUCH_PRIVILEGE: u32 = 0x521;
pub const ERROR_PRIVILEGE_NOT_HELD: u32 = 0x522;
pub const ERROR_INVALID_ACCOUNT_NAME: u32 = 0x523;
pub const ERROR_USER_EXISTS: u32 = 0x524;
pub const ERROR_NO_SUCH_USER: u32 = 0x525;
pub const ERROR_GROUP_EXISTS: u32 = 0x526;
pub const ERROR_NO_SUCH_GROUP: u32 = 0x527;
pub const ERROR_MEMBER_IN_GROUP: u32 = 0x528;
pub const ERROR_MEMBER_NOT_IN_GROUP: u32 = 0x529;
pub const ERROR_LAST_ADMIN: u32 = 0x52A;
pub const ERROR_WRONG_PASSWORD: u32 = 0x52B;
pub const ERROR_ILL_FORMED_PASSWORD: u32 = 0x52C;
pub const ERROR_PASSWORD_RESTRICTION: u32 = 0x52D;
pub const ERROR_LOGON_FAILURE: u32 = 0x52E;
pub const ERROR_ACCOUNT_RESTRICTION: u32 = 0x52F;
pub const ERROR_INVALID_LOGON_HOURS: u32 = 0x530;
pub const ERROR_INVALID_WORKSTATION: u32 = 0x531;
pub const ERROR_PASSWORD_EXPIRED: u32 = 0x532;
pub const ERROR_ACCOUNT_DISABLED: u32 = 0x533;
pub const ERROR_NONE_MAPPED: u32 = 0x534;
pub const ERROR_TOO_MANY_LUIDS_REQUESTED: u32 = 0x535;
pub const ERROR_LUIDS_EXHAUSTED: u32 = 0x536;
pub const ERROR_INVALID_SUB_AUTHORITY: u32 = 0x537;
pub const ERROR_INVALID_ACL: u32 = 0x538;
pub const ERROR_INVALID_SID: u32 = 0x539;
pub const ERROR_INVALID_SECURITY_DESCR: u32 = 0x53A;
pub const ERROR_BAD_INHERITANCE_ACL: u32 = 0x53C;
pub const ERROR_SERVER_DISABLED: u32 = 0x53D;
pub const ERROR_SERVER_NOT_DISABLED: u32 = 0x53E;
pub const ERROR_INVALID_ID_AUTHORITY: u32 = 0x53F;
pub const ERROR_ALLOTTED_SPACE_EXCEEDED: u32 = 0x540;
pub const ERROR_INVALID_GROUP_ATTRIBUTES: u32 = 0x541;
pub const ERROR_BAD_IMPERSONATION_LEVEL: u32 = 0x542;
pub const ERROR_CANT_OPEN_ANONYMOUS: u32 = 0x543;
pub const ERROR_BAD_VALIDATION_CLASS: u32 = 0x544;
pub const ERROR_BAD_TOKEN_TYPE: u32 = 0x545;
pub const ERROR_NO_SECURITY_ON_OBJECT: u32 = 0x546;
pub const ERROR_CANT_ACCESS_DOMAIN_INFO: u32 = 0x547;
pub const ERROR_INVALID_SERVER_STATE: u32 = 0x548;
pub const ERROR_INVALID_DOMAIN_STATE: u32 = 0x549;
pub const ERROR_INVALID_DOMAIN_ROLE: u32 = 0x54A;
pub const ERROR_NO_SUCH_DOMAIN: u32 = 0x54B;
pub const ERROR_DOMAIN_EXISTS: u32 = 0x54C;
pub const ERROR_DOMAIN_LIMIT_EXCEEDED: u32 = 0x54D;
pub const ERROR_INTERNAL_DB_CORRUPTION: u32 = 0x54E;
pub const ERROR_INTERNAL_ERROR: u32 = 0x54F;
pub const ERROR_GENERIC_NOT_MAPPED: u32 = 0x550;
pub const ERROR_BAD_DESCRIPTOR_FORMAT: u32 = 0x551;
pub const ERROR_NOT_LOGON_PROCESS: u32 = 0x552;
pub const ERROR_LOGON_SESSION_EXISTS: u32 = 0x553;
pub const ERROR_NO_SUCH_PACKAGE: u32 = 0x554;
pub const ERROR_BAD_LOGON_SESSION_STATE: u32 = 0x555;
pub const ERROR_LOGON_SESSION_COLLISION: u32 = 0x556;
pub const ERROR_INVALID_LOGON_TYPE: u32 = 0x557;
pub const ERROR_CANNOT_IMPERSONATE: u32 = 0x558;
pub const ERROR_RXACT_INVALID_STATE: u32 = 0x559;
pub const ERROR_RXACT_COMMIT_FAILURE: u32 = 0x55A;
pub const ERROR_SPECIAL_ACCOUNT: u32 = 0x55B;
pub const ERROR_SPECIAL_GROUP: u32 = 0x55C;
pub const ERROR_SPECIAL_USER: u32 = 0x55D;
pub const ERROR_MEMBERS_PRIMARY_GROUP: u32 = 0x55E;
pub const ERROR_TOKEN_ALREADY_IN_USE: u32 = 0x55F;
pub const ERROR_NO_SUCH_ALIAS: u32 = 0x560;
pub const ERROR_MEMBER_NOT_IN_ALIAS: u32 = 0x561;
pub const ERROR_MEMBER_IN_ALIAS: u32 = 0x562;
pub const ERROR_ALIAS_EXISTS: u32 = 0x563;
pub const ERROR_LOGON_NOT_GRANTED: u32 = 0x564;
pub const ERROR_TOO_MANY_SECRETS: u32 = 0x565;
pub const ERROR_SECRET_TOO_LONG: u32 = 0x566;
pub const ERROR_INTERNAL_DB_ERROR: u32 = 0x567;
pub const ERROR_TOO_MANY_CONTEXT_IDS: u32 = 0x568;
pub const ERROR_LOGON_TYPE_NOT_GRANTED: u32 = 0x569;
pub const ERROR_NT_CROSS_ENCRYPTION_REQUIRED: u32 = 0x56A;
pub const ERROR_NO_SUCH_MEMBER: u32 = 0x56B;
pub const ERROR_INVALID_MEMBER: u32 = 0x56C;
pub const ERROR_TOO_MANY_SIDS: u32 = 0x56D;
pub const ERROR_LM_CROSS_ENCRYPTION_REQUIRED: u32 = 0x56E;
pub const ERROR_NO_INHERITANCE: u32 = 0x56F;
pub const ERROR_FILE_CORRUPT: u32 = 0x570;
pub const ERROR_DISK_CORRUPT: u32 = 0x571;
pub const ERROR_NO_USER_SESSION_KEY: u32 = 0x572;
pub const ERROR_LICENSE_QUOTA_EXCEEDED: u32 = 0x573;
pub const ERROR_WRONG_TARGET_NAME: u32 = 0x574;
pub const ERROR_MUTUAL_AUTH_FAILED: u32 = 0x575;
pub const ERROR_TIME_SKEW: u32 = 0x576;
pub const ERROR_CURRENT_DOMAIN_NOT_ALLOWED: u32 = 0x577;
pub const ERROR_INVALID_WINDOW_HANDLE: u32 = 0x578;
pub const ERROR_INVALID_MENU_HANDLE: u32 = 0x579;
pub const ERROR_INVALID_CURSOR_HANDLE: u32 = 0x57A;
pub const ERROR_INVALID_ACCEL_HANDLE: u32 = 0x57B;
pub const ERROR_INVALID_HOOK_HANDLE: u32 = 0x57C;
pub const ERROR_INVALID_DWP_HANDLE: u32 = 0x57D;
pub const ERROR_TLW_WITH_WSCHILD: u32 = 0x57E;
pub const ERROR_CANNOT_FIND_WND_CLASS: u32 = 0x57F;
pub const ERROR_WINDOW_OF_OTHER_THREAD: u32 = 0x580;
pub const ERROR_HOTKEY_ALREADY_REGISTERED: u32 = 0x581;
pub const ERROR_CLASS_ALREADY_EXISTS: u32 = 0x582;
pub const ERROR_CLASS_DOES_NOT_EXIST: u32 = 0x583;
pub const ERROR_CLASS_HAS_WINDOWS: u32 = 0x584;
pub const ERROR_INVALID_INDEX: u32 = 0x585;
pub const ERROR_INVALID_ICON_HANDLE: u32 = 0x586;
pub const ERROR_PRIVATE_DIALOG_INDEX: u32 = 0x587;
pub const ERROR_LISTBOX_ID_NOT_FOUND: u32 = 0x588;
pub const ERROR_NO_WILDCARD_CHARACTERS: u32 = 0x589;
pub const ERROR_CLIPBOARD_NOT_OPEN: u32 = 0x58A;
pub const ERROR_HOTKEY_NOT_REGISTERED: u32 = 0x58B;
pub const ERROR_WINDOW_NOT_DIALOG: u32 = 0x58C;
pub const ERROR_CONTROL_ID_NOT_FOUND: u32 = 0x58D;
pub const ERROR_INVALID_COMBOBOX_MESSAGE: u32 = 0x58E;
pub const ERROR_WINDOW_NOT_COMBOBOX: u32 = 0x58F;
pub const ERROR_INVALID_EDIT_HEIGHT: u32 = 0x590;
pub const ERROR_DC_NOT_FOUND: u32 = 0x591;
pub const ERROR_INVALID_HOOK_FILTER: u32 = 0x592;
pub const ERROR_INVALID_FILTER_PROC: u32 = 0x593;
pub const ERROR_HOOK_NEEDS_HMOD: u32 = 0x594;
pub const ERROR_GLOBAL_ONLY_HOOK: u32 = 0x595;
pub const ERROR_JOURNAL_HOOK_SET: u32 = 0x596;
pub const ERROR_HOOK_NOT_INSTALLED: u32 = 0x597;
pub const ERROR_INVALID_LB_MESSAGE: u32 = 0x598;
pub const ERROR_SETCOUNT_ON_BAD_LB: u32 = 0x599;
pub const ERROR_LB_WITHOUT_TABSTOPS: u32 = 0x59A;
pub const ERROR_DESTROY_OBJECT_OF_OTHER_THREAD: u32 = 0x59B;
pub const ERROR_CHILD_WINDOW_MENU: u32 = 0x59C;
pub const ERROR_NO_SYSTEM_MENU: u32 = 0x59D;
pub const ERROR_INVALID_MSGBOX_STYLE: u32 = 0x59E;
pub const ERROR_INVALID_SPI_VALUE: u32 = 0x59F;
pub const ERROR_SCREEN_ALREADY_LOCKED: u32 = 0x5A0;
pub const ERROR_HWNDS_HAVE_DIFF_PARENT: u32 = 0x5A1;
pub const ERROR_NOT_CHILD_WINDOW: u32 = 0x5A2;
pub const ERROR_INVALID_GW_COMMAND: u32 = 0x5A3;
pub const ERROR_INVALID_THREAD_ID: u32 = 0x5A4;
pub const ERROR_NON_MDICHILD_WINDOW: u32 = 0x5A5;
pub const ERROR_POPUP_ALREADY_ACTIVE: u32 = 0x5A6;
pub const ERROR_NO_SCROLLBARS: u32 = 0x5A7;
pub const ERROR_INVALID_SCROLLBAR_RANGE: u32 = 0x5A8;
pub const ERROR_INVALID_SHOWWIN_COMMAND: u32 = 0x5A9;
pub const ERROR_NO_SYSTEM_RESOURCES: u32 = 0x5AA;
pub const ERROR_NONPAGED_SYSTEM_RESOURCES: u32 = 0x5AB;
pub const ERROR_PAGED_SYSTEM_RESOURCES: u32 = 0x5AC;
pub const ERROR_WORKING_SET_QUOTA: u32 = 0x5AD;
pub const ERROR_PAGEFILE_QUOTA: u32 = 0x5AE;
pub const ERROR_COMMITMENT_LIMIT: u32 = 0x5AF;
pub const ERROR_MENU_ITEM_NOT_FOUND: u32 = 0x5B0;
pub const ERROR_INVALID_KEYBOARD_HANDLE: u32 = 0x5B1;
pub const ERROR_HOOK_TYPE_NOT_ALLOWED: u32 = 0x5B2;
pub const ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION: u32 = 0x5B3;
pub const ERROR_TIMEOUT: u32 = 0x5B4;
pub const ERROR_INVALID_MONITOR_HANDLE: u32 = 0x5B5;
pub const ERROR_INCORRECT_SIZE: u32 = 0x5B6;
pub const ERROR_SYMLINK_CLASS_DISABLED: u32 = 0x5B7;
pub const ERROR_SYMLINK_NOT_SUPPORTED: u32 = 0x5B8;
pub const ERROR_XML_PARSE_ERROR: u32 = 0x5B9;
pub const ERROR_XMLDSIG_ERROR: u32 = 0x5BA;
pub const ERROR_RESTART_APPLICATION: u32 = 0x5BB;
pub const ERROR_WRONG_COMPARTMENT: u32 = 0x5BC;
pub const ERROR_AUTHIP_FAILURE: u32 = 0x5BD;
pub const ERROR_NO_NVRAM_RESOURCES: u32 = 0x5BE;
pub const ERROR_NOT_GUI_PROCESS: u32 = 0x5BF;
pub const ERROR_EVENTLOG_FILE_CORRUPT: u32 = 0x5DC;
pub const ERROR_EVENTLOG_CANT_START: u32 = 0x5DD;
pub const ERROR_LOG_FILE_FULL: u32 = 0x5DE;
pub const ERROR_EVENTLOG_FILE_CHANGED: u32 = 0x5DF;
pub const ERROR_CONTAINER_ASSIGNED: u32 = 0x5E0;
pub const ERROR_JOB_NO_CONTAINER: u32 = 0x5E1;
pub const ERROR_INVALID_TASK_NAME: u32 = 0x60E;
pub const ERROR_INVALID_TASK_INDEX: u32 = 0x60F;
pub const ERROR_THREAD_ALREADY_IN_TASK: u32 = 0x610;
pub const ERROR_INSTALL_SERVICE_FAILURE: u32 = 0x641;
pub const ERROR_INSTALL_USEREXIT: u32 = 0x642;
pub const ERROR_INSTALL_FAILURE: u32 = 0x643;
pub const ERROR_INSTALL_SUSPEND: u32 = 0x644;
pub const ERROR_UNKNOWN_PRODUCT: u32 = 0x645;
pub const ERROR_UNKNOWN_FEATURE: u32 = 0x646;
pub const ERROR_UNKNOWN_COMPONENT: u32 = 0x647;
pub const ERROR_UNKNOWN_PROPERTY: u32 = 0x648;
pub const ERROR_INVALID_HANDLE_STATE: u32 = 0x649;
pub const ERROR_BAD_CONFIGURATION: u32 = 0x64A;
pub const ERROR_INDEX_ABSENT: u32 = 0x64B;
pub const ERROR_INSTALL_SOURCE_ABSENT: u32 = 0x64C;
pub const ERROR_INSTALL_PACKAGE_VERSION: u32 = 0x64D;
pub const ERROR_PRODUCT_UNINSTALLED: u32 = 0x64E;
pub const ERROR_BAD_QUERY_SYNTAX: u32 = 0x64F;
pub const ERROR_INVALID_FIELD: u32 = 0x650;
pub const ERROR_DEVICE_REMOVED: u32 = 0x651;
pub const ERROR_INSTALL_ALREADY_RUNNING: u32 = 0x652;
pub const ERROR_INSTALL_PACKAGE_OPEN_FAILED: u32 = 0x653;
pub const ERROR_INSTALL_PACKAGE_INVALID: u32 = 0x654;
pub const ERROR_INSTALL_UI_FAILURE: u32 = 0x655;
pub const ERROR_INSTALL_LOG_FAILURE: u32 = 0x656;
pub const ERROR_INSTALL_LANGUAGE_UNSUPPORTED: u32 = 0x657;
pub const ERROR_INSTALL_TRANSFORM_FAILURE: u32 = 0x658;
pub const ERROR_INSTALL_PACKAGE_REJECTED: u32 = 0x659;
pub const ERROR_FUNCTION_NOT_CALLED: u32 = 0x65A;
pub const ERROR_FUNCTION_FAILED: u32 = 0x65B;
pub const ERROR_INVALID_TABLE: u32 = 0x65C;
pub const ERROR_DATATYPE_MISMATCH: u32 = 0x65D;
pub const ERROR_UNSUPPORTED_TYPE: u32 = 0x65E;
pub const ERROR_CREATE_FAILED: u32 = 0x65F;
pub const ERROR_INSTALL_TEMP_UNWRITABLE: u32 = 0x660;
pub const ERROR_INSTALL_PLATFORM_UNSUPPORTED: u32 = 0x661;
pub const ERROR_INSTALL_NOTUSED: u32 = 0x662;
pub const ERROR_PATCH_PACKAGE_OPEN_FAILED: u32 = 0x663;
pub const ERROR_PATCH_PACKAGE_INVALID: u32 = 0x664;
pub const ERROR_PATCH_PACKAGE_UNSUPPORTED: u32 = 0x665;
pub const ERROR_PRODUCT_VERSION: u32 = 0x666;
pub const ERROR_INVALID_COMMAND_LINE: u32 = 0x667;
pub const ERROR_INSTALL_REMOTE_DISALLOWED: u32 = 0x668;
pub const ERROR_SUCCESS_REBOOT_INITIATED: u32 = 0x669;
pub const ERROR_PATCH_TARGET_NOT_FOUND: u32 = 0x66A;
pub const ERROR_PATCH_PACKAGE_REJECTED: u32 = 0x66B;
pub const ERROR_INSTALL_TRANSFORM_REJECTED: u32 = 0x66C;
pub const ERROR_INSTALL_REMOTE_PROHIBITED: u32 = 0x66D;
pub const ERROR_PATCH_REMOVAL_UNSUPPORTED: u32 = 0x66E;
pub const ERROR_UNKNOWN_PATCH: u32 = 0x66F;
pub const ERROR_PATCH_NO_SEQUENCE: u32 = 0x670;
pub const ERROR_PATCH_REMOVAL_DISALLOWED: u32 = 0x671;
pub const ERROR_INVALID_PATCH_XML: u32 = 0x672;
pub const ERROR_PATCH_MANAGED_ADVERTISED_PRODUCT: u32 = 0x673;
pub const ERROR_INSTALL_SERVICE_SAFEBOOT: u32 = 0x674;
pub const ERROR_FAIL_FAST_EXCEPTION: u32 = 0x675;
pub const ERROR_INSTALL_REJECTED: u32 = 0x676;
pub const ERROR_DYNAMIC_CODE_BLOCKED: u32 = 0x677;
pub const ERROR_NOT_SAME_OBJECT: u32 = 0x678;
pub const ERROR_STRICT_CFG_VIOLATION: u32 = 0x679;
pub const ERROR_SET_CONTEXT_DENIED: u32 = 0x67C;
pub const ERROR_CROSS_PARTITION_VIOLATION: u32 = 0x67D;
pub const ERROR_RETURN_ADDRESS_HIJACK_ATTEMPT: u32 = 0x67E;
pub const ERROR_INVALID_USER_BUFFER: u32 = 0x6F8;
pub const ERROR_UNRECOGNIZED_MEDIA: u32 = 0x6F9;
pub const ERROR_NO_TRUST_LSA_SECRET: u32 = 0x6FA;
pub const ERROR_NO_TRUST_SAM_ACCOUNT: u32 = 0x6FB;
pub const ERROR_TRUSTED_DOMAIN_FAILURE: u32 = 0x6FC;
pub const ERROR_TRUSTED_RELATIONSHIP_FAILURE: u32 = 0x6FD;
pub const ERROR_TRUST_FAILURE: u32 = 0x6FE;
pub const ERROR_NETLOGON_NOT_STARTED: u32 = 0x700;
pub const ERROR_ACCOUNT_EXPIRED: u32 = 0x701;
pub const ERROR_REDIRECTOR_HAS_OPEN_HANDLES: u32 = 0x702;
pub const ERROR_PRINTER_DRIVER_ALREADY_INSTALLED: u32 = 0x703;
pub const ERROR_UNKNOWN_PORT: u32 = 0x704;
pub const ERROR_UNKNOWN_PRINTER_DRIVER: u32 = 0x705;
pub const ERROR_UNKNOWN_PRINTPROCESSOR: u32 = 0x706;
pub const ERROR_INVALID_SEPARATOR_FILE: u32 = 0x707;
pub const ERROR_INVALID_PRIORITY: u32 = 0x708;
pub const ERROR_INVALID_PRINTER_NAME: u32 = 0x709;
pub const ERROR_PRINTER_ALREADY_EXISTS: u32 = 0x70A;
pub const ERROR_INVALID_PRINTER_COMMAND: u32 = 0x70B;
pub const ERROR_INVALID_DATATYPE: u32 = 0x70C;
pub const ERROR_INVALID_ENVIRONMENT: u32 = 0x70D;
pub const ERROR_NOLOGON_INTERDOMAIN_TRUST_ACCOUNT: u32 = 0x70F;
pub const ERROR_NOLOGON_WORKSTATION_TRUST_ACCOUNT: u32 = 0x710;
pub const ERROR_NOLOGON_SERVER_TRUST_ACCOUNT: u32 = 0x711;
pub const ERROR_DOMAIN_TRUST_INCONSISTENT: u32 = 0x712;
pub const ERROR_SERVER_HAS_OPEN_HANDLES: u32 = 0x713;
pub const ERROR_RESOURCE_DATA_NOT_FOUND: u32 = 0x714;
pub const ERROR_RESOURCE_TYPE_NOT_FOUND: u32 = 0x715;
pub const ERROR_RESOURCE_NAME_NOT_FOUND: u32 = 0x716;
pub const ERROR_RESOURCE_LANG_NOT_FOUND: u32 = 0x717;
pub const ERROR_NOT_ENOUGH_QUOTA: u32 = 0x718;
pub const ERROR_INVALID_TIME: u32 = 0x76D;
pub const ERROR_INVALID_FORM_NAME: u32 = 0x76E;
pub const ERROR_INVALID_FORM_SIZE: u32 = 0x76F;
pub const ERROR_ALREADY_WAITING: u32 = 0x770;
pub const ERROR_PRINTER_DELETED: u32 = 0x771;
pub const ERROR_INVALID_PRINTER_STATE: u32 = 0x772;
pub const ERROR_PASSWORD_MUST_CHANGE: u32 = 0x773;
pub const ERROR_DOMAIN_CONTROLLER_NOT_FOUND: u32 = 0x774;
pub const ERROR_ACCOUNT_LOCKED_OUT: u32 = 0x775;
pub const ERROR_NO_SITENAME: u32 = 0x77F;
pub const ERROR_CANT_ACCESS_FILE: u32 = 0x780;
pub const ERROR_CANT_RESOLVE_FILENAME: u32 = 0x781;
pub const ERROR_KM_DRIVER_BLOCKED: u32 = 0x78A;
pub const ERROR_CONTEXT_EXPIRED: u32 = 0x78B;
pub const ERROR_PER_USER_TRUST_QUOTA_EXCEEDED: u32 = 0x78C;
pub const ERROR_ALL_USER_TRUST_QUOTA_EXCEEDED: u32 = 0x78D;
pub const ERROR_USER_DELETE_TRUST_QUOTA_EXCEEDED: u32 = 0x78E;
pub const ERROR_AUTHENTICATION_FIREWALL_FAILED: u32 = 0x78F;
pub const ERROR_REMOTE_PRINT_CONNECTIONS_BLOCKED: u32 = 0x790;
pub const ERROR_NTLM_BLOCKED: u32 = 0x791;
pub const ERROR_PASSWORD_CHANGE_REQUIRED: u32 = 0x792;
pub const ERROR_LOST_MODE_LOGON_RESTRICTION: u32 = 0x793;
pub const ERROR_INVALID_PIXEL_FORMAT: u32 = 0x7D0;
pub const ERROR_BAD_DRIVER: u32 = 0x7D1;
pub const ERROR_INVALID_WINDOW_STYLE: u32 = 0x7D2;
pub const ERROR_METAFILE_NOT_SUPPORTED: u32 = 0x7D3;
pub const ERROR_TRANSFORM_NOT_SUPPORTED: u32 = 0x7D4;
pub const ERROR_CLIPPING_NOT_SUPPORTED: u32 = 0x7D5;
pub const ERROR_INVALID_CMM: u32 = 0x7DA;
pub const ERROR_INVALID_PROFILE: u32 = 0x7DB;
pub const ERROR_TAG_NOT_FOUND: u32 = 0x7DC;
pub const ERROR_TAG_NOT_PRESENT: u32 = 0x7DD;
pub const ERROR_DUPLICATE_TAG: u32 = 0x7DE;
pub const ERROR_PROFILE_NOT_ASSOCIATED_WITH_DEVICE: u32 = 0x7DF;
pub const ERROR_PROFILE_NOT_FOUND: u32 = 0x7E0;
pub const ERROR_INVALID_COLORSPACE: u32 = 0x7E1;
pub const ERROR_ICM_NOT_ENABLED: u32 = 0x7E2;
pub const ERROR_DELETING_ICM_XFORM: u32 = 0x7E3;
pub const ERROR_INVALID_TRANSFORM: u32 = 0x7E4;
pub const ERROR_COLORSPACE_MISMATCH: u32 = 0x7E5;
pub const ERROR_INVALID_COLORINDEX: u32 = 0x7E6;
pub const ERROR_PROFILE_DOES_NOT_MATCH_DEVICE: u32 = 0x7E7;
pub const ERROR_CONNECTED_OTHER_PASSWORD: u32 = 0x83C;
pub const ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT: u32 = 0x83D;
pub const ERROR_BAD_USERNAME: u32 = 0x89A;
pub const ERROR_NOT_CONNECTED: u32 = 0x8CA;
pub const ERROR_OPEN_FILES: u32 = 0x961;
pub const ERROR_ACTIVE_CONNECTIONS: u32 = 0x962;
pub const ERROR_DEVICE_IN_USE: u32 = 0x964;
pub const ERROR_UNKNOWN_PRINT_MONITOR: u32 = 0xBB8;
pub const ERROR_PRINTER_DRIVER_IN_USE: u32 = 0xBB9;
pub const ERROR_SPOOL_FILE_NOT_FOUND: u32 = 0xBBA;
pub const ERROR_SPL_NO_STARTDOC: u32 = 0xBBB;
pub const ERROR_SPL_NO_ADDJOB: u32 = 0xBBC;
pub const ERROR_PRINT_PROCESSOR_ALREADY_INSTALLED: u32 = 0xBBD;
pub const ERROR_PRINT_MONITOR_ALREADY_INSTALLED: u32 = 0xBBE;
pub const ERROR_INVALID_PRINT_MONITOR: u32 = 0xBBF;
pub const ERROR_PRINT_MONITOR_IN_USE: u32 = 0xBC0;
pub const ERROR_PRINTER_HAS_JOBS_QUEUED: u32 = 0xBC1;
pub const ERROR_SUCCESS_REBOOT_REQUIRED: u32 = 0xBC2;
pub const ERROR_SUCCESS_RESTART_REQUIRED: u32 = 0xBC3;
pub const ERROR_PRINTER_NOT_FOUND: u32 = 0xBC4;
pub const ERROR_PRINTER_DRIVER_WARNED: u32 = 0xBC5;
pub const ERROR_PRINTER_DRIVER_BLOCKED: u32 = 0xBC6;
pub const ERROR_PRINTER_DRIVER_PACKAGE_IN_USE: u32 = 0xBC7;
pub const ERROR_CORE_DRIVER_PACKAGE_NOT_FOUND: u32 = 0xBC8;
pub const ERROR_FAIL_REBOOT_REQUIRED: u32 = 0xBC9;
pub const ERROR_FAIL_REBOOT_INITIATED: u32 = 0xBCA;
pub const ERROR_PRINTER_DRIVER_DOWNLOAD_NEEDED: u32 = 0xBCB;
pub const ERROR_PRINT_JOB_RESTART_REQUIRED: u32 = 0xBCC;
pub const ERROR_INVALID_PRINTER_DRIVER_MANIFEST: u32 = 0xBCD;
pub const ERROR_PRINTER_NOT_SHAREABLE: u32 = 0xBCE;
pub const ERROR_REQUEST_PAUSED: u32 = 0xBEA;
pub const ERROR_APPEXEC_CONDITION_NOT_SATISFIED: u32 = 0xBF4;
pub const ERROR_APPEXEC_HANDLE_INVALIDATED: u32 = 0xBF5;
pub const ERROR_APPEXEC_INVALID_HOST_GENERATION: u32 = 0xBF6;
pub const ERROR_APPEXEC_UNEXPECTED_PROCESS_REGISTRATION: u32 = 0xBF7;
pub const ERROR_APPEXEC_INVALID_HOST_STATE: u32 = 0xBF8;
pub const ERROR_APPEXEC_NO_DONOR: u32 = 0xBF9;
pub const ERROR_APPEXEC_HOST_ID_MISMATCH: u32 = 0xBFA;
pub const ERROR_APPEXEC_UNKNOWN_USER: u32 = 0xBFB;
pub const ERROR_APPEXEC_APP_COMPAT_BLOCK: u32 = 0xBFC;
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT: u32 = 0xBFD;
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_TERMINATION: u32 = 0xBFE;
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_LICENSING: u32 = 0xBFF;
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_RESOURCES: u32 = 0xC00;
pub const ERROR_VRF_VOLATILE_CFG_AND_IO_ENABLED: u32 = 0xC08;
pub const ERROR_VRF_VOLATILE_NOT_STOPPABLE: u32 = 0xC09;
pub const ERROR_VRF_VOLATILE_SAFE_MODE: u32 = 0xC0A;
pub const ERROR_VRF_VOLATILE_NOT_RUNNABLE_SYSTEM: u32 = 0xC0B;
pub const ERROR_VRF_VOLATILE_NOT_SUPPORTED_RULECLASS: u32 = 0xC0C;
pub const ERROR_VRF_VOLATILE_PROTECTED_DRIVER: u32 = 0xC0D;
pub const ERROR_VRF_VOLATILE_NMI_REGISTERED: u32 = 0xC0E;
pub const ERROR_VRF_VOLATILE_SETTINGS_CONFLICT: u32 = 0xC0F;
pub const ERROR_DIF_IOCALLBACK_NOT_REPLACED: u32 = 0xC76;
pub const ERROR_DIF_LIVEDUMP_LIMIT_EXCEEDED: u32 = 0xC77;
pub const ERROR_DIF_VOLATILE_SECTION_NOT_LOCKED: u32 = 0xC78;
pub const ERROR_DIF_VOLATILE_DRIVER_HOTPATCHED: u32 = 0xC79;
pub const ERROR_DIF_VOLATILE_INVALID_INFO: u32 = 0xC7A;
pub const ERROR_DIF_VOLATILE_DRIVER_IS_NOT_RUNNING: u32 = 0xC7B;
pub const ERROR_DIF_VOLATILE_PLUGIN_IS_NOT_RUNNING: u32 = 0xC7C;
pub const ERROR_DIF_VOLATILE_PLUGIN_CHANGE_NOT_ALLOWED: u32 = 0xC7D;
pub const ERROR_DIF_VOLATILE_NOT_ALLOWED: u32 = 0xC7E;
pub const ERROR_DIF_BINDING_API_NOT_FOUND: u32 = 0xC7F;
pub const ERROR_IO_REISSUE_AS_CACHED: u32 = 0xF6E;
pub const ERROR_WINS_INTERNAL: u32 = 0xFA0;
pub const ERROR_CAN_NOT_DEL_LOCAL_WINS: u32 = 0xFA1;
pub const ERROR_STATIC_INIT: u32 = 0xFA2;
pub const ERROR_INC_BACKUP: u32 = 0xFA3;
pub const ERROR_FULL_BACKUP: u32 = 0xFA4;
pub const ERROR_REC_NON_EXISTENT: u32 = 0xFA5;
pub const ERROR_RPL_NOT_ALLOWED: u32 = 0xFA6;
pub const ERROR_DHCP_ADDRESS_CONFLICT: u32 = 0x1004;
pub const ERROR_WMI_GUID_NOT_FOUND: u32 = 0x1068;
pub const ERROR_WMI_INSTANCE_NOT_FOUND: u32 = 0x1069;
pub const ERROR_WMI_ITEMID_NOT_FOUND: u32 = 0x106A;
pub const ERROR_WMI_TRY_AGAIN: u32 = 0x106B;
pub const ERROR_WMI_DP_NOT_FOUND: u32 = 0x106C;
pub const ERROR_WMI_UNRESOLVED_INSTANCE_REF: u32 = 0x106D;
pub const ERROR_WMI_ALREADY_ENABLED: u32 = 0x106E;
pub const ERROR_WMI_GUID_DISCONNECTED: u32 = 0x106F;
pub const ERROR_WMI_SERVER_UNAVAILABLE: u32 = 0x1070;
pub const ERROR_WMI_DP_FAILED: u32 = 0x1071;
pub const ERROR_WMI_INVALID_MOF: u32 = 0x1072;
pub const ERROR_WMI_INVALID_REGINFO: u32 = 0x1073;
pub const ERROR_WMI_ALREADY_DISABLED: u32 = 0x1074;
pub const ERROR_WMI_READ_ONLY: u32 = 0x1075;
pub const ERROR_WMI_SET_FAILURE: u32 = 0x1076;
pub const ERROR_NOT_APPCONTAINER: u32 = 0x109A;
pub const ERROR_APPCONTAINER_REQUIRED: u32 = 0x109B;
pub const ERROR_NOT_SUPPORTED_IN_APPCONTAINER: u32 = 0x109C;
pub const ERROR_INVALID_PACKAGE_SID_LENGTH: u32 = 0x109D;
pub const ERROR_INVALID_MEDIA: u32 = 0x10CC;
pub const ERROR_INVALID_LIBRARY: u32 = 0x10CD;
pub const ERROR_INVALID_MEDIA_POOL: u32 = 0x10CE;
pub const ERROR_DRIVE_MEDIA_MISMATCH: u32 = 0x10CF;
pub const ERROR_MEDIA_OFFLINE: u32 = 0x10D0;
pub const ERROR_LIBRARY_OFFLINE: u32 = 0x10D1;
pub const ERROR_EMPTY: u32 = 0x10D2;
pub const ERROR_NOT_EMPTY: u32 = 0x10D3;
pub const ERROR_MEDIA_UNAVAILABLE: u32 = 0x10D4;
pub const ERROR_RESOURCE_DISABLED: u32 = 0x10D5;
pub const ERROR_INVALID_CLEANER: u32 = 0x10D6;
pub const ERROR_UNABLE_TO_CLEAN: u32 = 0x10D7;
pub const ERROR_OBJECT_NOT_FOUND: u32 = 0x10D8;
pub const ERROR_DATABASE_FAILURE: u32 = 0x10D9;
pub const ERROR_DATABASE_FULL: u32 = 0x10DA;
pub const ERROR_MEDIA_INCOMPATIBLE: u32 = 0x10DB;
pub const ERROR_RESOURCE_NOT_PRESENT: u32 = 0x10DC;
pub const ERROR_INVALID_OPERATION: u32 = 0x10DD;
pub const ERROR_MEDIA_NOT_AVAILABLE: u32 = 0x10DE;
pub const ERROR_DEVICE_NOT_AVAILABLE: u32 = 0x10DF;
pub const ERROR_REQUEST_REFUSED: u32 = 0x10E0;
pub const ERROR_INVALID_DRIVE_OBJECT: u32 = 0x10E1;
pub const ERROR_LIBRARY_FULL: u32 = 0x10E2;
pub const ERROR_MEDIUM_NOT_ACCESSIBLE: u32 = 0x10E3;
pub const ERROR_UNABLE_TO_LOAD_MEDIUM: u32 = 0x10E4;
pub const ERROR_UNABLE_TO_INVENTORY_DRIVE: u32 = 0x10E5;
pub const ERROR_UNABLE_TO_INVENTORY_SLOT: u32 = 0x10E6;
pub const ERROR_UNABLE_TO_INVENTORY_TRANSPORT: u32 = 0x10E7;
pub const ERROR_TRANSPORT_FULL: u32 = 0x10E8;
pub const ERROR_CONTROLLING_IEPORT: u32 = 0x10E9;
pub const ERROR_UNABLE_TO_EJECT_MOUNTED_MEDIA: u32 = 0x10EA;
pub const ERROR_CLEANER_SLOT_SET: u32 = 0x10EB;
pub const ERROR_CLEANER_SLOT_NOT_SET: u32 = 0x10EC;
pub const ERROR_CLEANER_CARTRIDGE_SPENT: u32 = 0x10ED;
pub const ERROR_UNEXPECTED_OMID: u32 = 0x10EE;
pub const ERROR_CANT_DELETE_LAST_ITEM: u32 = 0x10EF;
pub const ERROR_MESSAGE_EXCEEDS_MAX_SIZE: u32 = 0x10F0;
pub const ERROR_VOLUME_CONTAINS_SYS_FILES: u32 = 0x10F1;
pub const ERROR_INDIGENOUS_TYPE: u32 = 0x10F2;
pub const ERROR_NO_SUPPORTING_DRIVES: u32 = 0x10F3;
pub const ERROR_CLEANER_CARTRIDGE_INSTALLED: u32 = 0x10F4;
pub const ERROR_IEPORT_FULL: u32 = 0x10F5;
pub const ERROR_FILE_OFFLINE: u32 = 0x10FE;
pub const ERROR_REMOTE_STORAGE_NOT_ACTIVE: u32 = 0x10FF;
pub const ERROR_REMOTE_STORAGE_MEDIA_ERROR: u32 = 0x1100;
pub const ERROR_NOT_A_REPARSE_POINT: u32 = 0x1126;
pub const ERROR_REPARSE_ATTRIBUTE_CONFLICT: u32 = 0x1127;
pub const ERROR_INVALID_REPARSE_DATA: u32 = 0x1128;
pub const ERROR_REPARSE_TAG_INVALID: u32 = 0x1129;
pub const ERROR_REPARSE_TAG_MISMATCH: u32 = 0x112A;
pub const ERROR_REPARSE_POINT_ENCOUNTERED: u32 = 0x112B;
pub const ERROR_APP_DATA_NOT_FOUND: u32 = 0x1130;
pub const ERROR_APP_DATA_EXPIRED: u32 = 0x1131;
pub const ERROR_APP_DATA_CORRUPT: u32 = 0x1132;
pub const ERROR_APP_DATA_LIMIT_EXCEEDED: u32 = 0x1133;
pub const ERROR_APP_DATA_REBOOT_REQUIRED: u32 = 0x1134;
pub const ERROR_SECUREBOOT_ROLLBACK_DETECTED: u32 = 0x1144;
pub const ERROR_SECUREBOOT_POLICY_VIOLATION: u32 = 0x1145;
pub const ERROR_SECUREBOOT_INVALID_POLICY: u32 = 0x1146;
pub const ERROR_SECUREBOOT_POLICY_PUBLISHER_NOT_FOUND: u32 = 0x1147;
pub const ERROR_SECUREBOOT_POLICY_NOT_SIGNED: u32 = 0x1148;
pub const ERROR_SECUREBOOT_NOT_ENABLED: u32 = 0x1149;
pub const ERROR_SECUREBOOT_FILE_REPLACED: u32 = 0x114A;
pub const ERROR_SECUREBOOT_POLICY_NOT_AUTHORIZED: u32 = 0x114B;
pub const ERROR_SECUREBOOT_POLICY_UNKNOWN: u32 = 0x114C;
pub const ERROR_SECUREBOOT_POLICY_MISSING_ANTIROLLBACKVERSION: u32 = 0x114D;
pub const ERROR_SECUREBOOT_PLATFORM_ID_MISMATCH: u32 = 0x114E;
pub const ERROR_SECUREBOOT_POLICY_ROLLBACK_DETECTED: u32 = 0x114F;
pub const ERROR_SECUREBOOT_POLICY_UPGRADE_MISMATCH: u32 = 0x1150;
pub const ERROR_SECUREBOOT_REQUIRED_POLICY_FILE_MISSING: u32 = 0x1151;
pub const ERROR_SECUREBOOT_NOT_BASE_POLICY: u32 = 0x1152;
pub const ERROR_SECUREBOOT_NOT_SUPPLEMENTAL_POLICY: u32 = 0x1153;
pub const ERROR_OFFLOAD_READ_FLT_NOT_SUPPORTED: u32 = 0x1158;
pub const ERROR_OFFLOAD_WRITE_FLT_NOT_SUPPORTED: u32 = 0x1159;
pub const ERROR_OFFLOAD_READ_FILE_NOT_SUPPORTED: u32 = 0x115A;
pub const ERROR_OFFLOAD_WRITE_FILE_NOT_SUPPORTED: u32 = 0x115B;
pub const ERROR_ALREADY_HAS_STREAM_ID: u32 = 0x115C;
pub const ERROR_SMR_GARBAGE_COLLECTION_REQUIRED: u32 = 0x115D;
pub const ERROR_WOF_WIM_HEADER_CORRUPT: u32 = 0x115E;
pub const ERROR_WOF_WIM_RESOURCE_TABLE_CORRUPT: u32 = 0x115F;
pub const ERROR_WOF_FILE_RESOURCE_TABLE_CORRUPT: u32 = 0x1160;
pub const ERROR_OBJECT_IS_IMMUTABLE: u32 = 0x1161;
pub const ERROR_VOLUME_NOT_SIS_ENABLED: u32 = 0x1194;
pub const ERROR_SYSTEM_INTEGRITY_ROLLBACK_DETECTED: u32 = 0x11C6;
pub const ERROR_SYSTEM_INTEGRITY_POLICY_VIOLATION: u32 = 0x11C7;
pub const ERROR_SYSTEM_INTEGRITY_INVALID_POLICY: u32 = 0x11C8;
pub const ERROR_SYSTEM_INTEGRITY_POLICY_NOT_SIGNED: u32 = 0x11C9;
pub const ERROR_SYSTEM_INTEGRITY_TOO_MANY_POLICIES: u32 = 0x11CA;
pub const ERROR_SYSTEM_INTEGRITY_SUPPLEMENTAL_POLICY_NOT_AUTHORIZED: u32 = 0x11CB;
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_MALICIOUS: u32 = 0x11CC;
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_PUA: u32 = 0x11CD;
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_DANGEROUS_EXT: u32 = 0x11CE;
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_OFFLINE: u32 = 0x11CF;
pub const ERROR_VSM_NOT_INITIALIZED: u32 = 0x11D0;
pub const ERROR_VSM_DMA_PROTECTION_NOT_IN_USE: u32 = 0x11D1;
pub const ERROR_PLATFORM_MANIFEST_NOT_AUTHORIZED: u32 = 0x11DA;
pub const ERROR_PLATFORM_MANIFEST_INVALID: u32 = 0x11DB;
pub const ERROR_PLATFORM_MANIFEST_FILE_NOT_AUTHORIZED: u32 = 0x11DC;
pub const ERROR_PLATFORM_MANIFEST_CATALOG_NOT_AUTHORIZED: u32 = 0x11DD;
pub const ERROR_PLATFORM_MANIFEST_BINARY_ID_NOT_FOUND: u32 = 0x11DE;
pub const ERROR_PLATFORM_MANIFEST_NOT_ACTIVE: u32 = 0x11DF;
pub const ERROR_PLATFORM_MANIFEST_NOT_SIGNED: u32 = 0x11E0;
pub const ERROR_DEPENDENT_RESOURCE_EXISTS: u32 = 0x1389;
pub const ERROR_DEPENDENCY_NOT_FOUND: u32 = 0x138A;
pub const ERROR_DEPENDENCY_ALREADY_EXISTS: u32 = 0x138B;
pub const ERROR_RESOURCE_NOT_ONLINE: u32 = 0x138C;
pub const ERROR_HOST_NODE_NOT_AVAILABLE: u32 = 0x138D;
pub const ERROR_RESOURCE_NOT_AVAILABLE: u32 = 0x138E;
pub const ERROR_RESOURCE_NOT_FOUND: u32 = 0x138F;
pub const ERROR_SHUTDOWN_CLUSTER: u32 = 0x1390;
pub const ERROR_CANT_EVICT_ACTIVE_NODE: u32 = 0x1391;
pub const ERROR_OBJECT_ALREADY_EXISTS: u32 = 0x1392;
pub const ERROR_OBJECT_IN_LIST: u32 = 0x1393;
pub const ERROR_GROUP_NOT_AVAILABLE: u32 = 0x1394;
pub const ERROR_GROUP_NOT_FOUND: u32 = 0x1395;
pub const ERROR_GROUP_NOT_ONLINE: u32 = 0x1396;
pub const ERROR_HOST_NODE_NOT_RESOURCE_OWNER: u32 = 0x1397;
pub const ERROR_HOST_NODE_NOT_GROUP_OWNER: u32 = 0x1398;
pub const ERROR_RESMON_CREATE_FAILED: u32 = 0x1399;
pub const ERROR_RESMON_ONLINE_FAILED: u32 = 0x139A;
pub const ERROR_RESOURCE_ONLINE: u32 = 0x139B;
pub const ERROR_QUORUM_RESOURCE: u32 = 0x139C;
pub const ERROR_NOT_QUORUM_CAPABLE: u32 = 0x139D;
pub const ERROR_CLUSTER_SHUTTING_DOWN: u32 = 0x139E;
pub const ERROR_INVALID_STATE: u32 = 0x139F;
pub const ERROR_RESOURCE_PROPERTIES_STORED: u32 = 0x13A0;
pub const ERROR_NOT_QUORUM_CLASS: u32 = 0x13A1;
pub const ERROR_CORE_RESOURCE: u32 = 0x13A2;
pub const ERROR_QUORUM_RESOURCE_ONLINE_FAILED: u32 = 0x13A3;
pub const ERROR_QUORUMLOG_OPEN_FAILED: u32 = 0x13A4;
pub const ERROR_CLUSTERLOG_CORRUPT: u32 = 0x13A5;
pub const ERROR_CLUSTERLOG_RECORD_EXCEEDS_MAXSIZE: u32 = 0x13A6;
pub const ERROR_CLUSTERLOG_EXCEEDS_MAXSIZE: u32 = 0x13A7;
pub const ERROR_CLUSTERLOG_CHKPOINT_NOT_FOUND: u32 = 0x13A8;
pub const ERROR_CLUSTERLOG_NOT_ENOUGH_SPACE: u32 = 0x13A9;
pub const ERROR_QUORUM_OWNER_ALIVE: u32 = 0x13AA;
pub const ERROR_NETWORK_NOT_AVAILABLE: u32 = 0x13AB;
pub const ERROR_NODE_NOT_AVAILABLE: u32 = 0x13AC;
pub const ERROR_ALL_NODES_NOT_AVAILABLE: u32 = 0x13AD;
pub const ERROR_RESOURCE_FAILED: u32 = 0x13AE;
pub const ERROR_CLUSTER_INVALID_NODE: u32 = 0x13AF;
pub const ERROR_CLUSTER_NODE_EXISTS: u32 = 0x13B0;
pub const ERROR_CLUSTER_JOIN_IN_PROGRESS: u32 = 0x13B1;
pub const ERROR_CLUSTER_NODE_NOT_FOUND: u32 = 0x13B2;
pub const ERROR_CLUSTER_LOCAL_NODE_NOT_FOUND: u32 = 0x13B3;
pub const ERROR_CLUSTER_NETWORK_EXISTS: u32 = 0x13B4;
pub const ERROR_CLUSTER_NETWORK_NOT_FOUND: u32 = 0x13B5;
pub const ERROR_CLUSTER_NETINTERFACE_EXISTS: u32 = 0x13B6;
pub const ERROR_CLUSTER_NETINTERFACE_NOT_FOUND: u32 = 0x13B7;
pub const ERROR_CLUSTER_INVALID_REQUEST: u32 = 0x13B8;
pub const ERROR_CLUSTER_INVALID_NETWORK_PROVIDER: u32 = 0x13B9;
pub const ERROR_CLUSTER_NODE_DOWN: u32 = 0x13BA;
pub const ERROR_CLUSTER_NODE_UNREACHABLE: u32 = 0x13BB;
pub const ERROR_CLUSTER_NODE_NOT_MEMBER: u32 = 0x13BC;
pub const ERROR_CLUSTER_JOIN_NOT_IN_PROGRESS: u32 = 0x13BD;
pub const ERROR_CLUSTER_INVALID_NETWORK: u32 = 0x13BE;
pub const ERROR_CLUSTER_NODE_UP: u32 = 0x13C0;
pub const ERROR_CLUSTER_IPADDR_IN_USE: u32 = 0x13C1;
pub const ERROR_CLUSTER_NODE_NOT_PAUSED: u32 = 0x13C2;
pub const ERROR_CLUSTER_NO_SECURITY_CONTEXT: u32 = 0x13C3;
pub const ERROR_CLUSTER_NETWORK_NOT_INTERNAL: u32 = 0x13C4;
pub const ERROR_CLUSTER_NODE_ALREADY_UP: u32 = 0x13C5;
pub const ERROR_CLUSTER_NODE_ALREADY_DOWN: u32 = 0x13C6;
pub const ERROR_CLUSTER_NETWORK_ALREADY_ONLINE: u32 = 0x13C7;
pub const ERROR_CLUSTER_NETWORK_ALREADY_OFFLINE: u32 = 0x13C8;
pub const ERROR_CLUSTER_NODE_ALREADY_MEMBER: u32 = 0x13C9;
pub const ERROR_CLUSTER_LAST_INTERNAL_NETWORK: u32 = 0x13CA;
pub const ERROR_CLUSTER_NETWORK_HAS_DEPENDENTS: u32 = 0x13CB;
pub const ERROR_INVALID_OPERATION_ON_QUORUM: u32 = 0x13CC;
pub const ERROR_DEPENDENCY_NOT_ALLOWED: u32 = 0x13CD;
pub const ERROR_CLUSTER_NODE_PAUSED: u32 = 0x13CE;
pub const ERROR_NODE_CANT_HOST_RESOURCE: u32 = 0x13CF;
pub const ERROR_CLUSTER_NODE_NOT_READY: u32 = 0x13D0;
pub const ERROR_CLUSTER_NODE_SHUTTING_DOWN: u32 = 0x13D1;
pub const ERROR_CLUSTER_JOIN_ABORTED: u32 = 0x13D2;
pub const ERROR_CLUSTER_INCOMPATIBLE_VERSIONS: u32 = 0x13D3;
pub const ERROR_CLUSTER_MAXNUM_OF_RESOURCES_EXCEEDED: u32 = 0x13D4;
pub const ERROR_CLUSTER_SYSTEM_CONFIG_CHANGED: u32 = 0x13D5;
pub const ERROR_CLUSTER_RESOURCE_TYPE_NOT_FOUND: u32 = 0x13D6;
pub const ERROR_CLUSTER_RESTYPE_NOT_SUPPORTED: u32 = 0x13D7;
pub const ERROR_CLUSTER_RESNAME_NOT_FOUND: u32 = 0x13D8;
pub const ERROR_CLUSTER_NO_RPC_PACKAGES_REGISTERED: u32 = 0x13D9;
pub const ERROR_CLUSTER_OWNER_NOT_IN_PREFLIST: u32 = 0x13DA;
pub const ERROR_CLUSTER_DATABASE_SEQMISMATCH: u32 = 0x13DB;
pub const ERROR_RESMON_INVALID_STATE: u32 = 0x13DC;
pub const ERROR_CLUSTER_GUM_NOT_LOCKER: u32 = 0x13DD;
pub const ERROR_QUORUM_DISK_NOT_FOUND: u32 = 0x13DE;
pub const ERROR_DATABASE_BACKUP_CORRUPT: u32 = 0x13DF;
pub const ERROR_CLUSTER_NODE_ALREADY_HAS_DFS_ROOT: u32 = 0x13E0;
pub const ERROR_RESOURCE_PROPERTY_UNCHANGEABLE: u32 = 0x13E1;
pub const ERROR_NO_ADMIN_ACCESS_POINT: u32 = 0x13E2;
pub const ERROR_CLUSTER_MEMBERSHIP_INVALID_STATE: u32 = 0x1702;
pub const ERROR_CLUSTER_QUORUMLOG_NOT_FOUND: u32 = 0x1703;
pub const ERROR_CLUSTER_MEMBERSHIP_HALT: u32 = 0x1704;
pub const ERROR_CLUSTER_INSTANCE_ID_MISMATCH: u32 = 0x1705;
pub const ERROR_CLUSTER_NETWORK_NOT_FOUND_FOR_IP: u32 = 0x1706;
pub const ERROR_CLUSTER_PROPERTY_DATA_TYPE_MISMATCH: u32 = 0x1707;
pub const ERROR_CLUSTER_EVICT_WITHOUT_CLEANUP: u32 = 0x1708;
pub const ERROR_CLUSTER_PARAMETER_MISMATCH: u32 = 0x1709;
pub const ERROR_NODE_CANNOT_BE_CLUSTERED: u32 = 0x170A;
pub const ERROR_CLUSTER_WRONG_OS_VERSION: u32 = 0x170B;
pub const ERROR_CLUSTER_CANT_CREATE_DUP_CLUSTER_NAME: u32 = 0x170C;
pub const ERROR_CLUSCFG_ALREADY_COMMITTED: u32 = 0x170D;
pub const ERROR_CLUSCFG_ROLLBACK_FAILED: u32 = 0x170E;
pub const ERROR_CLUSCFG_SYSTEM_DISK_DRIVE_LETTER_CONFLICT: u32 = 0x170F;
pub const ERROR_CLUSTER_OLD_VERSION: u32 = 0x1710;
pub const ERROR_CLUSTER_MISMATCHED_COMPUTER_ACCT_NAME: u32 = 0x1711;
pub const ERROR_CLUSTER_NO_NET_ADAPTERS: u32 = 0x1712;
pub const ERROR_CLUSTER_POISONED: u32 = 0x1713;
pub const ERROR_CLUSTER_GROUP_MOVING: u32 = 0x1714;
pub const ERROR_CLUSTER_RESOURCE_TYPE_BUSY: u32 = 0x1715;
pub const ERROR_RESOURCE_CALL_TIMED_OUT: u32 = 0x1716;
pub const ERROR_INVALID_CLUSTER_IPV6_ADDRESS: u32 = 0x1717;
pub const ERROR_CLUSTER_INTERNAL_INVALID_FUNCTION: u32 = 0x1718;
pub const ERROR_CLUSTER_PARAMETER_OUT_OF_BOUNDS: u32 = 0x1719;
pub const ERROR_CLUSTER_PARTIAL_SEND: u32 = 0x171A;
pub const ERROR_CLUSTER_REGISTRY_INVALID_FUNCTION: u32 = 0x171B;
pub const ERROR_CLUSTER_INVALID_STRING_TERMINATION: u32 = 0x171C;
pub const ERROR_CLUSTER_INVALID_STRING_FORMAT: u32 = 0x171D;
pub const ERROR_CLUSTER_DATABASE_TRANSACTION_IN_PROGRESS: u32 = 0x171E;
pub const ERROR_CLUSTER_DATABASE_TRANSACTION_NOT_IN_PROGRESS: u32 = 0x171F;
pub const ERROR_CLUSTER_NULL_DATA: u32 = 0x1720;
pub const ERROR_CLUSTER_PARTIAL_READ: u32 = 0x1721;
pub const ERROR_CLUSTER_PARTIAL_WRITE: u32 = 0x1722;
pub const ERROR_CLUSTER_CANT_DESERIALIZE_DATA: u32 = 0x1723;
pub const ERROR_DEPENDENT_RESOURCE_PROPERTY_CONFLICT: u32 = 0x1724;
pub const ERROR_CLUSTER_NO_QUORUM: u32 = 0x1725;
pub const ERROR_CLUSTER_INVALID_IPV6_NETWORK: u32 = 0x1726;
pub const ERROR_CLUSTER_INVALID_IPV6_TUNNEL_NETWORK: u32 = 0x1727;
pub const ERROR_QUORUM_NOT_ALLOWED_IN_THIS_GROUP: u32 = 0x1728;
pub const ERROR_DEPENDENCY_TREE_TOO_COMPLEX: u32 = 0x1729;
pub const ERROR_EXCEPTION_IN_RESOURCE_CALL: u32 = 0x172A;
pub const ERROR_CLUSTER_RHS_FAILED_INITIALIZATION: u32 = 0x172B;
pub const ERROR_CLUSTER_NOT_INSTALLED: u32 = 0x172C;
pub const ERROR_CLUSTER_RESOURCES_MUST_BE_ONLINE_ON_THE_SAME_NODE: u32 = 0x172D;
pub const ERROR_CLUSTER_MAX_NODES_IN_CLUSTER: u32 = 0x172E;
pub const ERROR_CLUSTER_TOO_MANY_NODES: u32 = 0x172F;
pub const ERROR_CLUSTER_OBJECT_ALREADY_USED: u32 = 0x1730;
pub const ERROR_NONCORE_GROUPS_FOUND: u32 = 0x1731;
pub const ERROR_FILE_SHARE_RESOURCE_CONFLICT: u32 = 0x1732;
pub const ERROR_CLUSTER_EVICT_INVALID_REQUEST: u32 = 0x1733;
pub const ERROR_CLUSTER_SINGLETON_RESOURCE: u32 = 0x1734;
pub const ERROR_CLUSTER_GROUP_SINGLETON_RESOURCE: u32 = 0x1735;
pub const ERROR_CLUSTER_RESOURCE_PROVIDER_FAILED: u32 = 0x1736;
pub const ERROR_CLUSTER_RESOURCE_CONFIGURATION_ERROR: u32 = 0x1737;
pub const ERROR_CLUSTER_GROUP_BUSY: u32 = 0x1738;
pub const ERROR_CLUSTER_NOT_SHARED_VOLUME: u32 = 0x1739;
pub const ERROR_CLUSTER_INVALID_SECURITY_DESCRIPTOR: u32 = 0x173A;
pub const ERROR_CLUSTER_SHARED_VOLUMES_IN_USE: u32 = 0x173B;
pub const ERROR_CLUSTER_USE_SHARED_VOLUMES_API: u32 = 0x173C;
pub const ERROR_CLUSTER_BACKUP_IN_PROGRESS: u32 = 0x173D;
pub const ERROR_NON_CSV_PATH: u32 = 0x173E;
pub const ERROR_CSV_VOLUME_NOT_LOCAL: u32 = 0x173F;
pub const ERROR_CLUSTER_WATCHDOG_TERMINATING: u32 = 0x1740;
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_INCOMPATIBLE_NODES: u32 = 0x1741;
pub const ERROR_CLUSTER_INVALID_NODE_WEIGHT: u32 = 0x1742;
pub const ERROR_CLUSTER_RESOURCE_VETOED_CALL: u32 = 0x1743;
pub const ERROR_RESMON_SYSTEM_RESOURCES_LACKING: u32 = 0x1744;
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_DESTINATION: u32 = 0x1745;
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_SOURCE: u32 = 0x1746;
pub const ERROR_CLUSTER_GROUP_QUEUED: u32 = 0x1747;
pub const ERROR_CLUSTER_RESOURCE_LOCKED_STATUS: u32 = 0x1748;
pub const ERROR_CLUSTER_SHARED_VOLUME_FAILOVER_NOT_ALLOWED: u32 = 0x1749;
pub const ERROR_CLUSTER_NODE_DRAIN_IN_PROGRESS: u32 = 0x174A;
pub const ERROR_CLUSTER_DISK_NOT_CONNECTED: u32 = 0x174B;
pub const ERROR_DISK_NOT_CSV_CAPABLE: u32 = 0x174C;
pub const ERROR_RESOURCE_NOT_IN_AVAILABLE_STORAGE: u32 = 0x174D;
pub const ERROR_CLUSTER_SHARED_VOLUME_REDIRECTED: u32 = 0x174E;
pub const ERROR_CLUSTER_SHARED_VOLUME_NOT_REDIRECTED: u32 = 0x174F;
pub const ERROR_CLUSTER_CANNOT_RETURN_PROPERTIES: u32 = 0x1750;
pub const ERROR_CLUSTER_RESOURCE_CONTAINS_UNSUPPORTED_DIFF_AREA_FOR_SHARED_VOLUMES: u32 = 0x1751;
pub const ERROR_CLUSTER_RESOURCE_IS_IN_MAINTENANCE_MODE: u32 = 0x1752;
pub const ERROR_CLUSTER_AFFINITY_CONFLICT: u32 = 0x1753;
pub const ERROR_CLUSTER_RESOURCE_IS_REPLICA_VIRTUAL_MACHINE: u32 = 0x1754;
pub const ERROR_CLUSTER_UPGRADE_INCOMPATIBLE_VERSIONS: u32 = 0x1755;
pub const ERROR_CLUSTER_UPGRADE_FIX_QUORUM_NOT_SUPPORTED: u32 = 0x1756;
pub const ERROR_CLUSTER_UPGRADE_RESTART_REQUIRED: u32 = 0x1757;
pub const ERROR_CLUSTER_UPGRADE_IN_PROGRESS: u32 = 0x1758;
pub const ERROR_CLUSTER_UPGRADE_INCOMPLETE: u32 = 0x1759;
pub const ERROR_CLUSTER_NODE_IN_GRACE_PERIOD: u32 = 0x175A;
pub const ERROR_CLUSTER_CSV_IO_PAUSE_TIMEOUT: u32 = 0x175B;
pub const ERROR_NODE_NOT_ACTIVE_CLUSTER_MEMBER: u32 = 0x175C;
pub const ERROR_CLUSTER_RESOURCE_NOT_MONITORED: u32 = 0x175D;
pub const ERROR_CLUSTER_RESOURCE_DOES_NOT_SUPPORT_UNMONITORED: u32 = 0x175E;
pub const ERROR_CLUSTER_RESOURCE_IS_REPLICATED: u32 = 0x175F;
pub const ERROR_CLUSTER_NODE_ISOLATED: u32 = 0x1760;
pub const ERROR_CLUSTER_NODE_QUARANTINED: u32 = 0x1761;
pub const ERROR_CLUSTER_DATABASE_UPDATE_CONDITION_FAILED: u32 = 0x1762;
pub const ERROR_CLUSTER_SPACE_DEGRADED: u32 = 0x1763;
pub const ERROR_CLUSTER_TOKEN_DELEGATION_NOT_SUPPORTED: u32 = 0x1764;
pub const ERROR_CLUSTER_CSV_INVALID_HANDLE: u32 = 0x1765;
pub const ERROR_CLUSTER_CSV_SUPPORTED_ONLY_ON_COORDINATOR: u32 = 0x1766;
pub const ERROR_GROUPSET_NOT_AVAILABLE: u32 = 0x1767;
pub const ERROR_GROUPSET_NOT_FOUND: u32 = 0x1768;
pub const ERROR_GROUPSET_CANT_PROVIDE: u32 = 0x1769;
pub const ERROR_CLUSTER_FAULT_DOMAIN_PARENT_NOT_FOUND: u32 = 0x176A;
pub const ERROR_CLUSTER_FAULT_DOMAIN_INVALID_HIERARCHY: u32 = 0x176B;
pub const ERROR_CLUSTER_FAULT_DOMAIN_FAILED_S2D_VALIDATION: u32 = 0x176C;
pub const ERROR_CLUSTER_FAULT_DOMAIN_S2D_CONNECTIVITY_LOSS: u32 = 0x176D;
pub const ERROR_CLUSTER_INVALID_INFRASTRUCTURE_FILESERVER_NAME: u32 = 0x176E;
pub const ERROR_CLUSTERSET_MANAGEMENT_CLUSTER_UNREACHABLE: u32 = 0x176F;
pub const ERROR_ENCRYPTION_FAILED: u32 = 0x1770;
pub const ERROR_DECRYPTION_FAILED: u32 = 0x1771;
pub const ERROR_FILE_ENCRYPTED: u32 = 0x1772;
pub const ERROR_NO_RECOVERY_POLICY: u32 = 0x1773;
pub const ERROR_NO_EFS: u32 = 0x1774;
pub const ERROR_WRONG_EFS: u32 = 0x1775;
pub const ERROR_NO_USER_KEYS: u32 = 0x1776;
pub const ERROR_FILE_NOT_ENCRYPTED: u32 = 0x1777;
pub const ERROR_NOT_EXPORT_FORMAT: u32 = 0x1778;
pub const ERROR_FILE_READ_ONLY: u32 = 0x1779;
pub const ERROR_DIR_EFS_DISALLOWED: u32 = 0x177A;
pub const ERROR_EFS_SERVER_NOT_TRUSTED: u32 = 0x177B;
pub const ERROR_BAD_RECOVERY_POLICY: u32 = 0x177C;
pub const ERROR_EFS_ALG_BLOB_TOO_BIG: u32 = 0x177D;
pub const ERROR_VOLUME_NOT_SUPPORT_EFS: u32 = 0x177E;
pub const ERROR_EFS_DISABLED: u32 = 0x177F;
pub const ERROR_EFS_VERSION_NOT_SUPPORT: u32 = 0x1780;
pub const ERROR_CS_ENCRYPTION_INVALID_SERVER_RESPONSE: u32 = 0x1781;
pub const ERROR_CS_ENCRYPTION_UNSUPPORTED_SERVER: u32 = 0x1782;
pub const ERROR_CS_ENCRYPTION_EXISTING_ENCRYPTED_FILE: u32 = 0x1783;
pub const ERROR_CS_ENCRYPTION_NEW_ENCRYPTED_FILE: u32 = 0x1784;
pub const ERROR_CS_ENCRYPTION_FILE_NOT_CSE: u32 = 0x1785;
pub const ERROR_ENCRYPTION_POLICY_DENIES_OPERATION: u32 = 0x1786;
pub const ERROR_WIP_ENCRYPTION_FAILED: u32 = 0x1787;
pub const ERROR_NO_BROWSER_SERVERS_FOUND: u32 = 0x17E6;
pub const ERROR_CLUSTER_OBJECT_IS_CLUSTER_SET_VM: u32 = 0x186A;
pub const ERROR_LOG_SECTOR_INVALID: u32 = 0x19C8;
pub const ERROR_LOG_SECTOR_PARITY_INVALID: u32 = 0x19C9;
pub const ERROR_LOG_SECTOR_REMAPPED: u32 = 0x19CA;
pub const ERROR_LOG_BLOCK_INCOMPLETE: u32 = 0x19CB;
pub const ERROR_LOG_INVALID_RANGE: u32 = 0x19CC;
pub const ERROR_LOG_BLOCKS_EXHAUSTED: u32 = 0x19CD;
pub const ERROR_LOG_READ_CONTEXT_INVALID: u32 = 0x19CE;
pub const ERROR_LOG_RESTART_INVALID: u32 = 0x19CF;
pub const ERROR_LOG_BLOCK_VERSION: u32 = 0x19D0;
pub const ERROR_LOG_BLOCK_INVALID: u32 = 0x19D1;
pub const ERROR_LOG_READ_MODE_INVALID: u32 = 0x19D2;
pub const ERROR_LOG_NO_RESTART: u32 = 0x19D3;
pub const ERROR_LOG_METADATA_CORRUPT: u32 = 0x19D4;
pub const ERROR_LOG_METADATA_INVALID: u32 = 0x19D5;
pub const ERROR_LOG_METADATA_INCONSISTENT: u32 = 0x19D6;
pub const ERROR_LOG_RESERVATION_INVALID: u32 = 0x19D7;
pub const ERROR_LOG_CANT_DELETE: u32 = 0x19D8;
pub const ERROR_LOG_CONTAINER_LIMIT_EXCEEDED: u32 = 0x19D9;
pub const ERROR_LOG_START_OF_LOG: u32 = 0x19DA;
pub const ERROR_LOG_POLICY_ALREADY_INSTALLED: u32 = 0x19DB;
pub const ERROR_LOG_POLICY_NOT_INSTALLED: u32 = 0x19DC;
pub const ERROR_LOG_POLICY_INVALID: u32 = 0x19DD;
pub const ERROR_LOG_POLICY_CONFLICT: u32 = 0x19DE;
pub const ERROR_LOG_PINNED_ARCHIVE_TAIL: u32 = 0x19DF;
pub const ERROR_LOG_RECORD_NONEXISTENT: u32 = 0x19E0;
pub const ERROR_LOG_RECORDS_RESERVED_INVALID: u32 = 0x19E1;
pub const ERROR_LOG_SPACE_RESERVED_INVALID: u32 = 0x19E2;
pub const ERROR_LOG_TAIL_INVALID: u32 = 0x19E3;
pub const ERROR_LOG_FULL: u32 = 0x19E4;
pub const ERROR_COULD_NOT_RESIZE_LOG: u32 = 0x19E5;
pub const ERROR_LOG_MULTIPLEXED: u32 = 0x19E6;
pub const ERROR_LOG_DEDICATED: u32 = 0x19E7;
pub const ERROR_LOG_ARCHIVE_NOT_IN_PROGRESS: u32 = 0x19E8;
pub const ERROR_LOG_ARCHIVE_IN_PROGRESS: u32 = 0x19E9;
pub const ERROR_LOG_EPHEMERAL: u32 = 0x19EA;
pub const ERROR_LOG_NOT_ENOUGH_CONTAINERS: u32 = 0x19EB;
pub const ERROR_LOG_CLIENT_ALREADY_REGISTERED: u32 = 0x19EC;
pub const ERROR_LOG_CLIENT_NOT_REGISTERED: u32 = 0x19ED;
pub const ERROR_LOG_FULL_HANDLER_IN_PROGRESS: u32 = 0x19EE;
pub const ERROR_LOG_CONTAINER_READ_FAILED: u32 = 0x19EF;
pub const ERROR_LOG_CONTAINER_WRITE_FAILED: u32 = 0x19F0;
pub const ERROR_LOG_CONTAINER_OPEN_FAILED: u32 = 0x19F1;
pub const ERROR_LOG_CONTAINER_STATE_INVALID: u32 = 0x19F2;
pub const ERROR_LOG_STATE_INVALID: u32 = 0x19F3;
pub const ERROR_LOG_PINNED: u32 = 0x19F4;
pub const ERROR_LOG_METADATA_FLUSH_FAILED: u32 = 0x19F5;
pub const ERROR_LOG_INCONSISTENT_SECURITY: u32 = 0x19F6;
pub const ERROR_LOG_APPENDED_FLUSH_FAILED: u32 = 0x19F7;
pub const ERROR_LOG_PINNED_RESERVATION: u32 = 0x19F8;
pub const ERROR_INVALID_TRANSACTION: u32 = 0x1A2C;
pub const ERROR_TRANSACTION_NOT_ACTIVE: u32 = 0x1A2D;
pub const ERROR_TRANSACTION_REQUEST_NOT_VALID: u32 = 0x1A2E;
pub const ERROR_TRANSACTION_NOT_REQUESTED: u32 = 0x1A2F;
pub const ERROR_TRANSACTION_ALREADY_ABORTED: u32 = 0x1A30;
pub const ERROR_TRANSACTION_ALREADY_COMMITTED: u32 = 0x1A31;
pub const ERROR_TM_INITIALIZATION_FAILED: u32 = 0x1A32;
pub const ERROR_RESOURCEMANAGER_READ_ONLY: u32 = 0x1A33;
pub const ERROR_TRANSACTION_NOT_JOINED: u32 = 0x1A34;
pub const ERROR_TRANSACTION_SUPERIOR_EXISTS: u32 = 0x1A35;
pub const ERROR_CRM_PROTOCOL_ALREADY_EXISTS: u32 = 0x1A36;
pub const ERROR_TRANSACTION_PROPAGATION_FAILED: u32 = 0x1A37;
pub const ERROR_CRM_PROTOCOL_NOT_FOUND: u32 = 0x1A38;
pub const ERROR_TRANSACTION_INVALID_MARSHALL_BUFFER: u32 = 0x1A39;
pub const ERROR_CURRENT_TRANSACTION_NOT_VALID: u32 = 0x1A3A;
pub const ERROR_TRANSACTION_NOT_FOUND: u32 = 0x1A3B;
pub const ERROR_RESOURCEMANAGER_NOT_FOUND: u32 = 0x1A3C;
pub const ERROR_ENLISTMENT_NOT_FOUND: u32 = 0x1A3D;
pub const ERROR_TRANSACTIONMANAGER_NOT_FOUND: u32 = 0x1A3E;
pub const ERROR_TRANSACTIONMANAGER_NOT_ONLINE: u32 = 0x1A3F;
pub const ERROR_TRANSACTIONMANAGER_RECOVERY_NAME_COLLISION: u32 = 0x1A40;
pub const ERROR_TRANSACTION_NOT_ROOT: u32 = 0x1A41;
pub const ERROR_TRANSACTION_OBJECT_EXPIRED: u32 = 0x1A42;
pub const ERROR_TRANSACTION_RESPONSE_NOT_ENLISTED: u32 = 0x1A43;
pub const ERROR_TRANSACTION_RECORD_TOO_LONG: u32 = 0x1A44;
pub const ERROR_IMPLICIT_TRANSACTION_NOT_SUPPORTED: u32 = 0x1A45;
pub const ERROR_TRANSACTION_INTEGRITY_VIOLATED: u32 = 0x1A46;
pub const ERROR_TRANSACTIONMANAGER_IDENTITY_MISMATCH: u32 = 0x1A47;
pub const ERROR_RM_CANNOT_BE_FROZEN_FOR_SNAPSHOT: u32 = 0x1A48;
pub const ERROR_TRANSACTION_MUST_WRITETHROUGH: u32 = 0x1A49;
pub const ERROR_TRANSACTION_NO_SUPERIOR: u32 = 0x1A4A;
pub const ERROR_HEURISTIC_DAMAGE_POSSIBLE: u32 = 0x1A4B;
pub const ERROR_TRANSACTIONAL_CONFLICT: u32 = 0x1A90;
pub const ERROR_RM_NOT_ACTIVE: u32 = 0x1A91;
pub const ERROR_RM_METADATA_CORRUPT: u32 = 0x1A92;
pub const ERROR_DIRECTORY_NOT_RM: u32 = 0x1A93;
pub const ERROR_TRANSACTIONS_UNSUPPORTED_REMOTE: u32 = 0x1A95;
pub const ERROR_LOG_RESIZE_INVALID_SIZE: u32 = 0x1A96;
pub const ERROR_OBJECT_NO_LONGER_EXISTS: u32 = 0x1A97;
pub const ERROR_STREAM_MINIVERSION_NOT_FOUND: u32 = 0x1A98;
pub const ERROR_STREAM_MINIVERSION_NOT_VALID: u32 = 0x1A99;
pub const ERROR_MINIVERSION_INACCESSIBLE_FROM_SPECIFIED_TRANSACTION: u32 = 0x1A9A;
pub const ERROR_CANT_OPEN_MINIVERSION_WITH_MODIFY_INTENT: u32 = 0x1A9B;
pub const ERROR_CANT_CREATE_MORE_STREAM_MINIVERSIONS: u32 = 0x1A9C;
pub const ERROR_REMOTE_FILE_VERSION_MISMATCH: u32 = 0x1A9E;
pub const ERROR_HANDLE_NO_LONGER_VALID: u32 = 0x1A9F;
pub const ERROR_NO_TXF_METADATA: u32 = 0x1AA0;
pub const ERROR_LOG_CORRUPTION_DETECTED: u32 = 0x1AA1;
pub const ERROR_CANT_RECOVER_WITH_HANDLE_OPEN: u32 = 0x1AA2;
pub const ERROR_RM_DISCONNECTED: u32 = 0x1AA3;
pub const ERROR_ENLISTMENT_NOT_SUPERIOR: u32 = 0x1AA4;
pub const ERROR_RECOVERY_NOT_NEEDED: u32 = 0x1AA5;
pub const ERROR_RM_ALREADY_STARTED: u32 = 0x1AA6;
pub const ERROR_FILE_IDENTITY_NOT_PERSISTENT: u32 = 0x1AA7;
pub const ERROR_CANT_BREAK_TRANSACTIONAL_DEPENDENCY: u32 = 0x1AA8;
pub const ERROR_CANT_CROSS_RM_BOUNDARY: u32 = 0x1AA9;
pub const ERROR_TXF_DIR_NOT_EMPTY: u32 = 0x1AAA;
pub const ERROR_INDOUBT_TRANSACTIONS_EXIST: u32 = 0x1AAB;
pub const ERROR_TM_VOLATILE: u32 = 0x1AAC;
pub const ERROR_ROLLBACK_TIMER_EXPIRED: u32 = 0x1AAD;
pub const ERROR_TXF_ATTRIBUTE_CORRUPT: u32 = 0x1AAE;
pub const ERROR_EFS_NOT_ALLOWED_IN_TRANSACTION: u32 = 0x1AAF;
pub const ERROR_TRANSACTIONAL_OPEN_NOT_ALLOWED: u32 = 0x1AB0;
pub const ERROR_LOG_GROWTH_FAILED: u32 = 0x1AB1;
pub const ERROR_TRANSACTED_MAPPING_UNSUPPORTED_REMOTE: u32 = 0x1AB2;
pub const ERROR_TXF_METADATA_ALREADY_PRESENT: u32 = 0x1AB3;
pub const ERROR_TRANSACTION_SCOPE_CALLBACKS_NOT_SET: u32 = 0x1AB4;
pub const ERROR_TRANSACTION_REQUIRED_PROMOTION: u32 = 0x1AB5;
pub const ERROR_CANNOT_EXECUTE_FILE_IN_TRANSACTION: u32 = 0x1AB6;
pub const ERROR_TRANSACTIONS_NOT_FROZEN: u32 = 0x1AB7;
pub const ERROR_TRANSACTION_FREEZE_IN_PROGRESS: u32 = 0x1AB8;
pub const ERROR_NOT_SNAPSHOT_VOLUME: u32 = 0x1AB9;
pub const ERROR_NO_SAVEPOINT_WITH_OPEN_FILES: u32 = 0x1ABA;
pub const ERROR_DATA_LOST_REPAIR: u32 = 0x1ABB;
pub const ERROR_SPARSE_NOT_ALLOWED_IN_TRANSACTION: u32 = 0x1ABC;
pub const ERROR_TM_IDENTITY_MISMATCH: u32 = 0x1ABD;
pub const ERROR_FLOATED_SECTION: u32 = 0x1ABE;
pub const ERROR_CANNOT_ACCEPT_TRANSACTED_WORK: u32 = 0x1ABF;
pub const ERROR_CANNOT_ABORT_TRANSACTIONS: u32 = 0x1AC0;
pub const ERROR_BAD_CLUSTERS: u32 = 0x1AC1;
pub const ERROR_COMPRESSION_NOT_ALLOWED_IN_TRANSACTION: u32 = 0x1AC2;
pub const ERROR_VOLUME_DIRTY: u32 = 0x1AC3;
pub const ERROR_NO_LINK_TRACKING_IN_TRANSACTION: u32 = 0x1AC4;
pub const ERROR_OPERATION_NOT_SUPPORTED_IN_TRANSACTION: u32 = 0x1AC5;
pub const ERROR_EXPIRED_HANDLE: u32 = 0x1AC6;
pub const ERROR_TRANSACTION_NOT_ENLISTED: u32 = 0x1AC7;
pub const ERROR_CTX_WINSTATION_NAME_INVALID: u32 = 0x1B59;
pub const ERROR_CTX_INVALID_PD: u32 = 0x1B5A;
pub const ERROR_CTX_PD_NOT_FOUND: u32 = 0x1B5B;
pub const ERROR_CTX_WD_NOT_FOUND: u32 = 0x1B5C;
pub const ERROR_CTX_CANNOT_MAKE_EVENTLOG_ENTRY: u32 = 0x1B5D;
pub const ERROR_CTX_SERVICE_NAME_COLLISION: u32 = 0x1B5E;
pub const ERROR_CTX_CLOSE_PENDING: u32 = 0x1B5F;
pub const ERROR_CTX_NO_OUTBUF: u32 = 0x1B60;
pub const ERROR_CTX_MODEM_INF_NOT_FOUND: u32 = 0x1B61;
pub const ERROR_CTX_INVALID_MODEMNAME: u32 = 0x1B62;
pub const ERROR_CTX_MODEM_RESPONSE_ERROR: u32 = 0x1B63;
pub const ERROR_CTX_MODEM_RESPONSE_TIMEOUT: u32 = 0x1B64;
pub const ERROR_CTX_MODEM_RESPONSE_NO_CARRIER: u32 = 0x1B65;
pub const ERROR_CTX_MODEM_RESPONSE_NO_DIALTONE: u32 = 0x1B66;
pub const ERROR_CTX_MODEM_RESPONSE_BUSY: u32 = 0x1B67;
pub const ERROR_CTX_MODEM_RESPONSE_VOICE: u32 = 0x1B68;
pub const ERROR_CTX_TD_ERROR: u32 = 0x1B69;
pub const ERROR_CTX_WINSTATION_NOT_FOUND: u32 = 0x1B6E;
pub const ERROR_CTX_WINSTATION_ALREADY_EXISTS: u32 = 0x1B6F;
pub const ERROR_CTX_WINSTATION_BUSY: u32 = 0x1B70;
pub const ERROR_CTX_BAD_VIDEO_MODE: u32 = 0x1B71;
pub const ERROR_CTX_GRAPHICS_INVALID: u32 = 0x1B7B;
pub const ERROR_CTX_LOGON_DISABLED: u32 = 0x1B7D;
pub const ERROR_CTX_NOT_CONSOLE: u32 = 0x1B7E;
pub const ERROR_CTX_CLIENT_QUERY_TIMEOUT: u32 = 0x1B80;
pub const ERROR_CTX_CONSOLE_DISCONNECT: u32 = 0x1B81;
pub const ERROR_CTX_CONSOLE_CONNECT: u32 = 0x1B82;
pub const ERROR_CTX_SHADOW_DENIED: u32 = 0x1B84;
pub const ERROR_CTX_WINSTATION_ACCESS_DENIED: u32 = 0x1B85;
pub const ERROR_CTX_INVALID_WD: u32 = 0x1B89;
pub const ERROR_CTX_SHADOW_INVALID: u32 = 0x1B8A;
pub const ERROR_CTX_SHADOW_DISABLED: u32 = 0x1B8B;
pub const ERROR_CTX_CLIENT_LICENSE_IN_USE: u32 = 0x1B8C;
pub const ERROR_CTX_CLIENT_LICENSE_NOT_SET: u32 = 0x1B8D;
pub const ERROR_CTX_LICENSE_NOT_AVAILABLE: u32 = 0x1B8E;
pub const ERROR_CTX_LICENSE_CLIENT_INVALID: u32 = 0x1B8F;
pub const ERROR_CTX_LICENSE_EXPIRED: u32 = 0x1B90;
pub const ERROR_CTX_SHADOW_NOT_RUNNING: u32 = 0x1B91;
pub const ERROR_CTX_SHADOW_ENDED_BY_MODE_CHANGE: u32 = 0x1B92;
pub const ERROR_ACTIVATION_COUNT_EXCEEDED: u32 = 0x1B93;
pub const ERROR_CTX_WINSTATIONS_DISABLED: u32 = 0x1B94;
pub const ERROR_CTX_ENCRYPTION_LEVEL_REQUIRED: u32 = 0x1B95;
pub const ERROR_CTX_SESSION_IN_USE: u32 = 0x1B96;
pub const ERROR_CTX_NO_FORCE_LOGOFF: u32 = 0x1B97;
pub const ERROR_CTX_ACCOUNT_RESTRICTION: u32 = 0x1B98;
pub const ERROR_RDP_PROTOCOL_ERROR: u32 = 0x1B99;
pub const ERROR_CTX_CDM_CONNECT: u32 = 0x1B9A;
pub const ERROR_CTX_CDM_DISCONNECT: u32 = 0x1B9B;
pub const ERROR_CTX_SECURITY_LAYER_ERROR: u32 = 0x1B9C;
pub const ERROR_TS_INCOMPATIBLE_SESSIONS: u32 = 0x1B9D;
pub const ERROR_TS_VIDEO_SUBSYSTEM_ERROR: u32 = 0x1B9E;
pub const ERROR_DS_NOT_INSTALLED: u32 = 0x2008;
pub const ERROR_DS_MEMBERSHIP_EVALUATED_LOCALLY: u32 = 0x2009;
pub const ERROR_DS_NO_ATTRIBUTE_OR_VALUE: u32 = 0x200A;
pub const ERROR_DS_INVALID_ATTRIBUTE_SYNTAX: u32 = 0x200B;
pub const ERROR_DS_ATTRIBUTE_TYPE_UNDEFINED: u32 = 0x200C;
pub const ERROR_DS_ATTRIBUTE_OR_VALUE_EXISTS: u32 = 0x200D;
pub const ERROR_DS_BUSY: u32 = 0x200E;
pub const ERROR_DS_UNAVAILABLE: u32 = 0x200F;
pub const ERROR_DS_NO_RIDS_ALLOCATED: u32 = 0x2010;
pub const ERROR_DS_NO_MORE_RIDS: u32 = 0x2011;
pub const ERROR_DS_INCORRECT_ROLE_OWNER: u32 = 0x2012;
pub const ERROR_DS_RIDMGR_INIT_ERROR: u32 = 0x2013;
pub const ERROR_DS_OBJ_CLASS_VIOLATION: u32 = 0x2014;
pub const ERROR_DS_CANT_ON_NON_LEAF: u32 = 0x2015;
pub const ERROR_DS_CANT_ON_RDN: u32 = 0x2016;
pub const ERROR_DS_CANT_MOD_OBJ_CLASS: u32 = 0x2017;
pub const ERROR_DS_CROSS_DOM_MOVE_ERROR: u32 = 0x2018;
pub const ERROR_DS_GC_NOT_AVAILABLE: u32 = 0x2019;
pub const ERROR_SHARED_POLICY: u32 = 0x201A;
pub const ERROR_POLICY_OBJECT_NOT_FOUND: u32 = 0x201B;
pub const ERROR_POLICY_ONLY_IN_DS: u32 = 0x201C;
pub const ERROR_PROMOTION_ACTIVE: u32 = 0x201D;
pub const ERROR_NO_PROMOTION_ACTIVE: u32 = 0x201E;
pub const ERROR_DS_OPERATIONS_ERROR: u32 = 0x2020;
pub const ERROR_DS_PROTOCOL_ERROR: u32 = 0x2021;
pub const ERROR_DS_TIMELIMIT_EXCEEDED: u32 = 0x2022;
pub const ERROR_DS_SIZELIMIT_EXCEEDED: u32 = 0x2023;
pub const ERROR_DS_ADMIN_LIMIT_EXCEEDED: u32 = 0x2024;
pub const ERROR_DS_COMPARE_FALSE: u32 = 0x2025;
pub const ERROR_DS_COMPARE_TRUE: u32 = 0x2026;
pub const ERROR_DS_AUTH_METHOD_NOT_SUPPORTED: u32 = 0x2027;
pub const ERROR_DS_STRONG_AUTH_REQUIRED: u32 = 0x2028;
pub const ERROR_DS_INAPPROPRIATE_AUTH: u32 = 0x2029;
pub const ERROR_DS_AUTH_UNKNOWN: u32 = 0x202A;
pub const ERROR_DS_REFERRAL: u32 = 0x202B;
pub const ERROR_DS_UNAVAILABLE_CRIT_EXTENSION: u32 = 0x202C;
pub const ERROR_DS_CONFIDENTIALITY_REQUIRED: u32 = 0x202D;
pub const ERROR_DS_INAPPROPRIATE_MATCHING: u32 = 0x202E;
pub const ERROR_DS_CONSTRAINT_VIOLATION: u32 = 0x202F;
pub const ERROR_DS_NO_SUCH_OBJECT: u32 = 0x2030;
pub const ERROR_DS_ALIAS_PROBLEM: u32 = 0x2031;
pub const ERROR_DS_INVALID_DN_SYNTAX: u32 = 0x2032;
pub const ERROR_DS_IS_LEAF: u32 = 0x2033;
pub const ERROR_DS_ALIAS_DEREF_PROBLEM: u32 = 0x2034;
pub const ERROR_DS_UNWILLING_TO_PERFORM: u32 = 0x2035;
pub const ERROR_DS_LOOP_DETECT: u32 = 0x2036;
pub const ERROR_DS_NAMING_VIOLATION: u32 = 0x2037;
pub const ERROR_DS_OBJECT_RESULTS_TOO_LARGE: u32 = 0x2038;
pub const ERROR_DS_AFFECTS_MULTIPLE_DSAS: u32 = 0x2039;
pub const ERROR_DS_SERVER_DOWN: u32 = 0x203A;
pub const ERROR_DS_LOCAL_ERROR: u32 = 0x203B;
pub const ERROR_DS_ENCODING_ERROR: u32 = 0x203C;
pub const ERROR_DS_DECODING_ERROR: u32 = 0x203D;
pub const ERROR_DS_FILTER_UNKNOWN: u32 = 0x203E;
pub const ERROR_DS_PARAM_ERROR: u32 = 0x203F;
pub const ERROR_DS_NOT_SUPPORTED: u32 = 0x2040;
pub const ERROR_DS_NO_RESULTS_RETURNED: u32 = 0x2041;
pub const ERROR_DS_CONTROL_NOT_FOUND: u32 = 0x2042;
pub const ERROR_DS_CLIENT_LOOP: u32 = 0x2043;
pub const ERROR_DS_REFERRAL_LIMIT_EXCEEDED: u32 = 0x2044;
pub const ERROR_DS_SORT_CONTROL_MISSING: u32 = 0x2045;
pub const ERROR_DS_OFFSET_RANGE_ERROR: u32 = 0x2046;
pub const ERROR_DS_RIDMGR_DISABLED: u32 = 0x2047;
pub const ERROR_DS_ROOT_MUST_BE_NC: u32 = 0x206D;
pub const ERROR_DS_ADD_REPLICA_INHIBITED: u32 = 0x206E;
pub const ERROR_DS_ATT_NOT_DEF_IN_SCHEMA: u32 = 0x206F;
pub const ERROR_DS_MAX_OBJ_SIZE_EXCEEDED: u32 = 0x2070;
pub const ERROR_DS_OBJ_STRING_NAME_EXISTS: u32 = 0x2071;
pub const ERROR_DS_NO_RDN_DEFINED_IN_SCHEMA: u32 = 0x2072;
pub const ERROR_DS_RDN_DOESNT_MATCH_SCHEMA: u32 = 0x2073;
pub const ERROR_DS_NO_REQUESTED_ATTS_FOUND: u32 = 0x2074;
pub const ERROR_DS_USER_BUFFER_TO_SMALL: u32 = 0x2075;
pub const ERROR_DS_ATT_IS_NOT_ON_OBJ: u32 = 0x2076;
pub const ERROR_DS_ILLEGAL_MOD_OPERATION: u32 = 0x2077;
pub const ERROR_DS_OBJ_TOO_LARGE: u32 = 0x2078;
pub const ERROR_DS_BAD_INSTANCE_TYPE: u32 = 0x2079;
pub const ERROR_DS_MASTERDSA_REQUIRED: u32 = 0x207A;
pub const ERROR_DS_OBJECT_CLASS_REQUIRED: u32 = 0x207B;
pub const ERROR_DS_MISSING_REQUIRED_ATT: u32 = 0x207C;
pub const ERROR_DS_ATT_NOT_DEF_FOR_CLASS: u32 = 0x207D;
pub const ERROR_DS_ATT_ALREADY_EXISTS: u32 = 0x207E;
pub const ERROR_DS_CANT_ADD_ATT_VALUES: u32 = 0x2080;
pub const ERROR_DS_SINGLE_VALUE_CONSTRAINT: u32 = 0x2081;
pub const ERROR_DS_RANGE_CONSTRAINT: u32 = 0x2082;
pub const ERROR_DS_ATT_VAL_ALREADY_EXISTS: u32 = 0x2083;
pub const ERROR_DS_CANT_REM_MISSING_ATT: u32 = 0x2084;
pub const ERROR_DS_CANT_REM_MISSING_ATT_VAL: u32 = 0x2085;
pub const ERROR_DS_ROOT_CANT_BE_SUBREF: u32 = 0x2086;
pub const ERROR_DS_NO_CHAINING: u32 = 0x2087;
pub const ERROR_DS_NO_CHAINED_EVAL: u32 = 0x2088;
pub const ERROR_DS_NO_PARENT_OBJECT: u32 = 0x2089;
pub const ERROR_DS_PARENT_IS_AN_ALIAS: u32 = 0x208A;
pub const ERROR_DS_CANT_MIX_MASTER_AND_REPS: u32 = 0x208B;
pub const ERROR_DS_CHILDREN_EXIST: u32 = 0x208C;
pub const ERROR_DS_OBJ_NOT_FOUND: u32 = 0x208D;
pub const ERROR_DS_ALIASED_OBJ_MISSING: u32 = 0x208E;
pub const ERROR_DS_BAD_NAME_SYNTAX: u32 = 0x208F;
pub const ERROR_DS_ALIAS_POINTS_TO_ALIAS: u32 = 0x2090;
pub const ERROR_DS_CANT_DEREF_ALIAS: u32 = 0x2091;
pub const ERROR_DS_OUT_OF_SCOPE: u32 = 0x2092;
pub const ERROR_DS_OBJECT_BEING_REMOVED: u32 = 0x2093;
pub const ERROR_DS_CANT_DELETE_DSA_OBJ: u32 = 0x2094;
pub const ERROR_DS_GENERIC_ERROR: u32 = 0x2095;
pub const ERROR_DS_DSA_MUST_BE_INT_MASTER: u32 = 0x2096;
pub const ERROR_DS_CLASS_NOT_DSA: u32 = 0x2097;
pub const ERROR_DS_INSUFF_ACCESS_RIGHTS: u32 = 0x2098;
pub const ERROR_DS_ILLEGAL_SUPERIOR: u32 = 0x2099;
pub const ERROR_DS_ATTRIBUTE_OWNED_BY_SAM: u32 = 0x209A;
pub const ERROR_DS_NAME_TOO_MANY_PARTS: u32 = 0x209B;
pub const ERROR_DS_NAME_TOO_LONG: u32 = 0x209C;
pub const ERROR_DS_NAME_VALUE_TOO_LONG: u32 = 0x209D;
pub const ERROR_DS_NAME_UNPARSEABLE: u32 = 0x209E;
pub const ERROR_DS_NAME_TYPE_UNKNOWN: u32 = 0x209F;
pub const ERROR_DS_NOT_AN_OBJECT: u32 = 0x20A0;
pub const ERROR_DS_SEC_DESC_TOO_SHORT: u32 = 0x20A1;
pub const ERROR_DS_SEC_DESC_INVALID: u32 = 0x20A2;
pub const ERROR_DS_NO_DELETED_NAME: u32 = 0x20A3;
pub const ERROR_DS_SUBREF_MUST_HAVE_PARENT: u32 = 0x20A4;
pub const ERROR_DS_NCNAME_MUST_BE_NC: u32 = 0x20A5;
pub const ERROR_DS_CANT_ADD_SYSTEM_ONLY: u32 = 0x20A6;
pub const ERROR_DS_CLASS_MUST_BE_CONCRETE: u32 = 0x20A7;
pub const ERROR_DS_INVALID_DMD: u32 = 0x20A8;
pub const ERROR_DS_OBJ_GUID_EXISTS: u32 = 0x20A9;
pub const ERROR_DS_NOT_ON_BACKLINK: u32 = 0x20AA;
pub const ERROR_DS_NO_CROSSREF_FOR_NC: u32 = 0x20AB;
pub const ERROR_DS_SHUTTING_DOWN: u32 = 0x20AC;
pub const ERROR_DS_UNKNOWN_OPERATION: u32 = 0x20AD;
pub const ERROR_DS_INVALID_ROLE_OWNER: u32 = 0x20AE;
pub const ERROR_DS_COULDNT_CONTACT_FSMO: u32 = 0x20AF;
pub const ERROR_DS_CROSS_NC_DN_RENAME: u32 = 0x20B0;
pub const ERROR_DS_CANT_MOD_SYSTEM_ONLY: u32 = 0x20B1;
pub const ERROR_DS_REPLICATOR_ONLY: u32 = 0x20B2;
pub const ERROR_DS_OBJ_CLASS_NOT_DEFINED: u32 = 0x20B3;
pub const ERROR_DS_OBJ_CLASS_NOT_SUBCLASS: u32 = 0x20B4;
pub const ERROR_DS_NAME_REFERENCE_INVALID: u32 = 0x20B5;
pub const ERROR_DS_CROSS_REF_EXISTS: u32 = 0x20B6;
pub const ERROR_DS_CANT_DEL_MASTER_CROSSREF: u32 = 0x20B7;
pub const ERROR_DS_SUBTREE_NOTIFY_NOT_NC_HEAD: u32 = 0x20B8;
pub const ERROR_DS_NOTIFY_FILTER_TOO_COMPLEX: u32 = 0x20B9;
pub const ERROR_DS_DUP_RDN: u32 = 0x20BA;
pub const ERROR_DS_DUP_OID: u32 = 0x20BB;
pub const ERROR_DS_DUP_MAPI_ID: u32 = 0x20BC;
pub const ERROR_DS_DUP_SCHEMA_ID_GUID: u32 = 0x20BD;
pub const ERROR_DS_DUP_LDAP_DISPLAY_NAME: u32 = 0x20BE;
pub const ERROR_DS_SEMANTIC_ATT_TEST: u32 = 0x20BF;
pub const ERROR_DS_SYNTAX_MISMATCH: u32 = 0x20C0;
pub const ERROR_DS_EXISTS_IN_MUST_HAVE: u32 = 0x20C1;
pub const ERROR_DS_EXISTS_IN_MAY_HAVE: u32 = 0x20C2;
pub const ERROR_DS_NONEXISTENT_MAY_HAVE: u32 = 0x20C3;
pub const ERROR_DS_NONEXISTENT_MUST_HAVE: u32 = 0x20C4;
pub const ERROR_DS_AUX_CLS_TEST_FAIL: u32 = 0x20C5;
pub const ERROR_DS_NONEXISTENT_POSS_SUP: u32 = 0x20C6;
pub const ERROR_DS_SUB_CLS_TEST_FAIL: u32 = 0x20C7;
pub const ERROR_DS_BAD_RDN_ATT_ID_SYNTAX: u32 = 0x20C8;
pub const ERROR_DS_EXISTS_IN_AUX_CLS: u32 = 0x20C9;
pub const ERROR_DS_EXISTS_IN_SUB_CLS: u32 = 0x20CA;
pub const ERROR_DS_EXISTS_IN_POSS_SUP: u32 = 0x20CB;
pub const ERROR_DS_RECALCSCHEMA_FAILED: u32 = 0x20CC;
pub const ERROR_DS_TREE_DELETE_NOT_FINISHED: u32 = 0x20CD;
pub const ERROR_DS_CANT_DELETE: u32 = 0x20CE;
pub const ERROR_DS_ATT_SCHEMA_REQ_ID: u32 = 0x20CF;
pub const ERROR_DS_BAD_ATT_SCHEMA_SYNTAX: u32 = 0x20D0;
pub const ERROR_DS_CANT_CACHE_ATT: u32 = 0x20D1;
pub const ERROR_DS_CANT_CACHE_CLASS: u32 = 0x20D2;
pub const ERROR_DS_CANT_REMOVE_ATT_CACHE: u32 = 0x20D3;
pub const ERROR_DS_CANT_REMOVE_CLASS_CACHE: u32 = 0x20D4;
pub const ERROR_DS_CANT_RETRIEVE_DN: u32 = 0x20D5;
pub const ERROR_DS_MISSING_SUPREF: u32 = 0x20D6;
pub const ERROR_DS_CANT_RETRIEVE_INSTANCE: u32 = 0x20D7;
pub const ERROR_DS_CODE_INCONSISTENCY: u32 = 0x20D8;
pub const ERROR_DS_DATABASE_ERROR: u32 = 0x20D9;
pub const ERROR_DS_GOVERNSID_MISSING: u32 = 0x20DA;
pub const ERROR_DS_MISSING_EXPECTED_ATT: u32 = 0x20DB;
pub const ERROR_DS_NCNAME_MISSING_CR_REF: u32 = 0x20DC;
pub const ERROR_DS_SECURITY_CHECKING_ERROR: u32 = 0x20DD;
pub const ERROR_DS_SCHEMA_NOT_LOADED: u32 = 0x20DE;
pub const ERROR_DS_SCHEMA_ALLOC_FAILED: u32 = 0x20DF;
pub const ERROR_DS_ATT_SCHEMA_REQ_SYNTAX: u32 = 0x20E0;
pub const ERROR_DS_GCVERIFY_ERROR: u32 = 0x20E1;
pub const ERROR_DS_DRA_SCHEMA_MISMATCH: u32 = 0x20E2;
pub const ERROR_DS_CANT_FIND_DSA_OBJ: u32 = 0x20E3;
pub const ERROR_DS_CANT_FIND_EXPECTED_NC: u32 = 0x20E4;
pub const ERROR_DS_CANT_FIND_NC_IN_CACHE: u32 = 0x20E5;
pub const ERROR_DS_CANT_RETRIEVE_CHILD: u32 = 0x20E6;
pub const ERROR_DS_SECURITY_ILLEGAL_MODIFY: u32 = 0x20E7;
pub const ERROR_DS_CANT_REPLACE_HIDDEN_REC: u32 = 0x20E8;
pub const ERROR_DS_BAD_HIERARCHY_FILE: u32 = 0x20E9;
pub const ERROR_DS_BUILD_HIERARCHY_TABLE_FAILED: u32 = 0x20EA;
pub const ERROR_DS_CONFIG_PARAM_MISSING: u32 = 0x20EB;
pub const ERROR_DS_COUNTING_AB_INDICES_FAILED: u32 = 0x20EC;
pub const ERROR_DS_HIERARCHY_TABLE_MALLOC_FAILED: u32 = 0x20ED;
pub const ERROR_DS_INTERNAL_FAILURE: u32 = 0x20EE;
pub const ERROR_DS_UNKNOWN_ERROR: u32 = 0x20EF;
pub const ERROR_DS_ROOT_REQUIRES_CLASS_TOP: u32 = 0x20F0;
pub const ERROR_DS_REFUSING_FSMO_ROLES: u32 = 0x20F1;
pub const ERROR_DS_MISSING_FSMO_SETTINGS: u32 = 0x20F2;
pub const ERROR_DS_UNABLE_TO_SURRENDER_ROLES: u32 = 0x20F3;
pub const ERROR_DS_DRA_GENERIC: u32 = 0x20F4;
pub const ERROR_DS_DRA_INVALID_PARAMETER: u32 = 0x20F5;
pub const ERROR_DS_DRA_BUSY: u32 = 0x20F6;
pub const ERROR_DS_DRA_BAD_DN: u32 = 0x20F7;
pub const ERROR_DS_DRA_BAD_NC: u32 = 0x20F8;
pub const ERROR_DS_DRA_DN_EXISTS: u32 = 0x20F9;
pub const ERROR_DS_DRA_INTERNAL_ERROR: u32 = 0x20FA;
pub const ERROR_DS_DRA_INCONSISTENT_DIT: u32 = 0x20FB;
pub const ERROR_DS_DRA_CONNECTION_FAILED: u32 = 0x20FC;
pub const ERROR_DS_DRA_BAD_INSTANCE_TYPE: u32 = 0x20FD;
pub const ERROR_DS_DRA_OUT_OF_MEM: u32 = 0x20FE;
pub const ERROR_DS_DRA_MAIL_PROBLEM: u32 = 0x20FF;
pub const ERROR_DS_DRA_REF_ALREADY_EXISTS: u32 = 0x2100;
pub const ERROR_DS_DRA_REF_NOT_FOUND: u32 = 0x2101;
pub const ERROR_DS_DRA_OBJ_IS_REP_SOURCE: u32 = 0x2102;
pub const ERROR_DS_DRA_DB_ERROR: u32 = 0x2103;
pub const ERROR_DS_DRA_NO_REPLICA: u32 = 0x2104;
pub const ERROR_DS_DRA_ACCESS_DENIED: u32 = 0x2105;
pub const ERROR_DS_DRA_NOT_SUPPORTED: u32 = 0x2106;
pub const ERROR_DS_DRA_RPC_CANCELLED: u32 = 0x2107;
pub const ERROR_DS_DRA_SOURCE_DISABLED: u32 = 0x2108;
pub const ERROR_DS_DRA_SINK_DISABLED: u32 = 0x2109;
pub const ERROR_DS_DRA_NAME_COLLISION: u32 = 0x210A;
pub const ERROR_DS_DRA_SOURCE_REINSTALLED: u32 = 0x210B;
pub const ERROR_DS_DRA_MISSING_PARENT: u32 = 0x210C;
pub const ERROR_DS_DRA_PREEMPTED: u32 = 0x210D;
pub const ERROR_DS_DRA_ABANDON_SYNC: u32 = 0x210E;
pub const ERROR_DS_DRA_SHUTDOWN: u32 = 0x210F;
pub const ERROR_DS_DRA_INCOMPATIBLE_PARTIAL_SET: u32 = 0x2110;
pub const ERROR_DS_DRA_SOURCE_IS_PARTIAL_REPLICA: u32 = 0x2111;
pub const ERROR_DS_DRA_EXTN_CONNECTION_FAILED: u32 = 0x2112;
pub const ERROR_DS_INSTALL_SCHEMA_MISMATCH: u32 = 0x2113;
pub const ERROR_DS_DUP_LINK_ID: u32 = 0x2114;
pub const ERROR_DS_NAME_ERROR_RESOLVING: u32 = 0x2115;
pub const ERROR_DS_NAME_ERROR_NOT_FOUND: u32 = 0x2116;
pub const ERROR_DS_NAME_ERROR_NOT_UNIQUE: u32 = 0x2117;
pub const ERROR_DS_NAME_ERROR_NO_MAPPING: u32 = 0x2118;
pub const ERROR_DS_NAME_ERROR_DOMAIN_ONLY: u32 = 0x2119;
pub const ERROR_DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: u32 = 0x211A;
pub const ERROR_DS_CONSTRUCTED_ATT_MOD: u32 = 0x211B;
pub const ERROR_DS_WRONG_OM_OBJ_CLASS: u32 = 0x211C;
pub const ERROR_DS_DRA_REPL_PENDING: u32 = 0x211D;
pub const ERROR_DS_DS_REQUIRED: u32 = 0x211E;
pub const ERROR_DS_INVALID_LDAP_DISPLAY_NAME: u32 = 0x211F;
pub const ERROR_DS_NON_BASE_SEARCH: u32 = 0x2120;
pub const ERROR_DS_CANT_RETRIEVE_ATTS: u32 = 0x2121;
pub const ERROR_DS_BACKLINK_WITHOUT_LINK: u32 = 0x2122;
pub const ERROR_DS_EPOCH_MISMATCH: u32 = 0x2123;
pub const ERROR_DS_SRC_NAME_MISMATCH: u32 = 0x2124;
pub const ERROR_DS_SRC_AND_DST_NC_IDENTICAL: u32 = 0x2125;
pub const ERROR_DS_DST_NC_MISMATCH: u32 = 0x2126;
pub const ERROR_DS_NOT_AUTHORITIVE_FOR_DST_NC: u32 = 0x2127;
pub const ERROR_DS_SRC_GUID_MISMATCH: u32 = 0x2128;
pub const ERROR_DS_CANT_MOVE_DELETED_OBJECT: u32 = 0x2129;
pub const ERROR_DS_PDC_OPERATION_IN_PROGRESS: u32 = 0x212A;
pub const ERROR_DS_CROSS_DOMAIN_CLEANUP_REQD: u32 = 0x212B;
pub const ERROR_DS_ILLEGAL_XDOM_MOVE_OPERATION: u32 = 0x212C;
pub const ERROR_DS_CANT_WITH_ACCT_GROUP_MEMBERSHPS: u32 = 0x212D;
pub const ERROR_DS_NC_MUST_HAVE_NC_PARENT: u32 = 0x212E;
pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE: u32 = 0x212F;
pub const ERROR_DS_DST_DOMAIN_NOT_NATIVE: u32 = 0x2130;
pub const ERROR_DS_MISSING_INFRASTRUCTURE_CONTAINER: u32 = 0x2131;
pub const ERROR_DS_CANT_MOVE_ACCOUNT_GROUP: u32 = 0x2132;
pub const ERROR_DS_CANT_MOVE_RESOURCE_GROUP: u32 = 0x2133;
pub const ERROR_DS_INVALID_SEARCH_FLAG: u32 = 0x2134;
pub const ERROR_DS_NO_TREE_DELETE_ABOVE_NC: u32 = 0x2135;
pub const ERROR_DS_COULDNT_LOCK_TREE_FOR_DELETE: u32 = 0x2136;
pub const ERROR_DS_COULDNT_IDENTIFY_OBJECTS_FOR_TREE_DELETE: u32 = 0x2137;
pub const ERROR_DS_SAM_INIT_FAILURE: u32 = 0x2138;
pub const ERROR_DS_SENSITIVE_GROUP_VIOLATION: u32 = 0x2139;
pub const ERROR_DS_CANT_MOD_PRIMARYGROUPID: u32 = 0x213A;
pub const ERROR_DS_ILLEGAL_BASE_SCHEMA_MOD: u32 = 0x213B;
pub const ERROR_DS_NONSAFE_SCHEMA_CHANGE: u32 = 0x213C;
pub const ERROR_DS_SCHEMA_UPDATE_DISALLOWED: u32 = 0x213D;
pub const ERROR_DS_CANT_CREATE_UNDER_SCHEMA: u32 = 0x213E;
pub const ERROR_DS_INSTALL_NO_SRC_SCH_VERSION: u32 = 0x213F;
pub const ERROR_DS_INSTALL_NO_SCH_VERSION_IN_INIFILE: u32 = 0x2140;
pub const ERROR_DS_INVALID_GROUP_TYPE: u32 = 0x2141;
pub const ERROR_DS_NO_NEST_GLOBALGROUP_IN_MIXEDDOMAIN: u32 = 0x2142;
pub const ERROR_DS_NO_NEST_LOCALGROUP_IN_MIXEDDOMAIN: u32 = 0x2143;
pub const ERROR_DS_GLOBAL_CANT_HAVE_LOCAL_MEMBER: u32 = 0x2144;
pub const ERROR_DS_GLOBAL_CANT_HAVE_UNIVERSAL_MEMBER: u32 = 0x2145;
pub const ERROR_DS_UNIVERSAL_CANT_HAVE_LOCAL_MEMBER: u32 = 0x2146;
pub const ERROR_DS_GLOBAL_CANT_HAVE_CROSSDOMAIN_MEMBER: u32 = 0x2147;
pub const ERROR_DS_LOCAL_CANT_HAVE_CROSSDOMAIN_LOCAL_MEMBER: u32 = 0x2148;
pub const ERROR_DS_HAVE_PRIMARY_MEMBERS: u32 = 0x2149;
pub const ERROR_DS_STRING_SD_CONVERSION_FAILED: u32 = 0x214A;
pub const ERROR_DS_NAMING_MASTER_GC: u32 = 0x214B;
pub const ERROR_DS_DNS_LOOKUP_FAILURE: u32 = 0x214C;
pub const ERROR_DS_COULDNT_UPDATE_SPNS: u32 = 0x214D;
pub const ERROR_DS_CANT_RETRIEVE_SD: u32 = 0x214E;
pub const ERROR_DS_KEY_NOT_UNIQUE: u32 = 0x214F;
pub const ERROR_DS_WRONG_LINKED_ATT_SYNTAX: u32 = 0x2150;
pub const ERROR_DS_SAM_NEED_BOOTKEY_PASSWORD: u32 = 0x2151;
pub const ERROR_DS_SAM_NEED_BOOTKEY_FLOPPY: u32 = 0x2152;
pub const ERROR_DS_CANT_START: u32 = 0x2153;
pub const ERROR_DS_INIT_FAILURE: u32 = 0x2154;
pub const ERROR_DS_NO_PKT_PRIVACY_ON_CONNECTION: u32 = 0x2155;
pub const ERROR_DS_SOURCE_DOMAIN_IN_FOREST: u32 = 0x2156;
pub const ERROR_DS_DESTINATION_DOMAIN_NOT_IN_FOREST: u32 = 0x2157;
pub const ERROR_DS_DESTINATION_AUDITING_NOT_ENABLED: u32 = 0x2158;
pub const ERROR_DS_CANT_FIND_DC_FOR_SRC_DOMAIN: u32 = 0x2159;
pub const ERROR_DS_SRC_OBJ_NOT_GROUP_OR_USER: u32 = 0x215A;
pub const ERROR_DS_SRC_SID_EXISTS_IN_FOREST: u32 = 0x215B;
pub const ERROR_DS_SRC_AND_DST_OBJECT_CLASS_MISMATCH: u32 = 0x215C;
pub const ERROR_SAM_INIT_FAILURE: u32 = 0x215D;
pub const ERROR_DS_DRA_SCHEMA_INFO_SHIP: u32 = 0x215E;
pub const ERROR_DS_DRA_SCHEMA_CONFLICT: u32 = 0x215F;
pub const ERROR_DS_DRA_EARLIER_SCHEMA_CONFLICT: u32 = 0x2160;
pub const ERROR_DS_DRA_OBJ_NC_MISMATCH: u32 = 0x2161;
pub const ERROR_DS_NC_STILL_HAS_DSAS: u32 = 0x2162;
pub const ERROR_DS_GC_REQUIRED: u32 = 0x2163;
pub const ERROR_DS_LOCAL_MEMBER_OF_LOCAL_ONLY: u32 = 0x2164;
pub const ERROR_DS_NO_FPO_IN_UNIVERSAL_GROUPS: u32 = 0x2165;
pub const ERROR_DS_CANT_ADD_TO_GC: u32 = 0x2166;
pub const ERROR_DS_NO_CHECKPOINT_WITH_PDC: u32 = 0x2167;
pub const ERROR_DS_SOURCE_AUDITING_NOT_ENABLED: u32 = 0x2168;
pub const ERROR_DS_CANT_CREATE_IN_NONDOMAIN_NC: u32 = 0x2169;
pub const ERROR_DS_INVALID_NAME_FOR_SPN: u32 = 0x216A;
pub const ERROR_DS_FILTER_USES_CONTRUCTED_ATTRS: u32 = 0x216B;
pub const ERROR_DS_UNICODEPWD_NOT_IN_QUOTES: u32 = 0x216C;
pub const ERROR_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED: u32 = 0x216D;
pub const ERROR_DS_MUST_BE_RUN_ON_DST_DC: u32 = 0x216E;
pub const ERROR_DS_SRC_DC_MUST_BE_SP4_OR_GREATER: u32 = 0x216F;
pub const ERROR_DS_CANT_TREE_DELETE_CRITICAL_OBJ: u32 = 0x2170;
pub const ERROR_DS_INIT_FAILURE_CONSOLE: u32 = 0x2171;
pub const ERROR_DS_SAM_INIT_FAILURE_CONSOLE: u32 = 0x2172;
pub const ERROR_DS_FOREST_VERSION_TOO_HIGH: u32 = 0x2173;
pub const ERROR_DS_DOMAIN_VERSION_TOO_HIGH: u32 = 0x2174;
pub const ERROR_DS_FOREST_VERSION_TOO_LOW: u32 = 0x2175;
pub const ERROR_DS_DOMAIN_VERSION_TOO_LOW: u32 = 0x2176;
pub const ERROR_DS_INCOMPATIBLE_VERSION: u32 = 0x2177;
pub const ERROR_DS_LOW_DSA_VERSION: u32 = 0x2178;
pub const ERROR_DS_NO_BEHAVIOR_VERSION_IN_MIXEDDOMAIN: u32 = 0x2179;
pub const ERROR_DS_NOT_SUPPORTED_SORT_ORDER: u32 = 0x217A;
pub const ERROR_DS_NAME_NOT_UNIQUE: u32 = 0x217B;
pub const ERROR_DS_MACHINE_ACCOUNT_CREATED_PRENT4: u32 = 0x217C;
pub const ERROR_DS_OUT_OF_VERSION_STORE: u32 = 0x217D;
pub const ERROR_DS_INCOMPATIBLE_CONTROLS_USED: u32 = 0x217E;
pub const ERROR_DS_NO_REF_DOMAIN: u32 = 0x217F;
pub const ERROR_DS_RESERVED_LINK_ID: u32 = 0x2180;
pub const ERROR_DS_LINK_ID_NOT_AVAILABLE: u32 = 0x2181;
pub const ERROR_DS_AG_CANT_HAVE_UNIVERSAL_MEMBER: u32 = 0x2182;
pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_INSTANCE_TYPE: u32 = 0x2183;
pub const ERROR_DS_NO_OBJECT_MOVE_IN_SCHEMA_NC: u32 = 0x2184;
pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_FLAG: u32 = 0x2185;
pub const ERROR_DS_MODIFYDN_WRONG_GRANDPARENT: u32 = 0x2186;
pub const ERROR_DS_NAME_ERROR_TRUST_REFERRAL: u32 = 0x2187;
pub const ERROR_NOT_SUPPORTED_ON_STANDARD_SERVER: u32 = 0x2188;
pub const ERROR_DS_CANT_ACCESS_REMOTE_PART_OF_AD: u32 = 0x2189;
pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE_V2: u32 = 0x218A;
pub const ERROR_DS_THREAD_LIMIT_EXCEEDED: u32 = 0x218B;
pub const ERROR_DS_NOT_CLOSEST: u32 = 0x218C;
pub const ERROR_DS_CANT_DERIVE_SPN_WITHOUT_SERVER_REF: u32 = 0x218D;
pub const ERROR_DS_SINGLE_USER_MODE_FAILED: u32 = 0x218E;
pub const ERROR_DS_NTDSCRIPT_SYNTAX_ERROR: u32 = 0x218F;
pub const ERROR_DS_NTDSCRIPT_PROCESS_ERROR: u32 = 0x2190;
pub const ERROR_DS_DIFFERENT_REPL_EPOCHS: u32 = 0x2191;
pub const ERROR_DS_DRS_EXTENSIONS_CHANGED: u32 = 0x2192;
pub const ERROR_DS_REPLICA_SET_CHANGE_NOT_ALLOWED_ON_DISABLED_CR: u32 = 0x2193;
pub const ERROR_DS_NO_MSDS_INTID: u32 = 0x2194;
pub const ERROR_DS_DUP_MSDS_INTID: u32 = 0x2195;
pub const ERROR_DS_EXISTS_IN_RDNATTID: u32 = 0x2196;
pub const ERROR_DS_AUTHORIZATION_FAILED: u32 = 0x2197;
pub const ERROR_DS_INVALID_SCRIPT: u32 = 0x2198;
pub const ERROR_DS_REMOTE_CROSSREF_OP_FAILED: u32 = 0x2199;
pub const ERROR_DS_CROSS_REF_BUSY: u32 = 0x219A;
pub const ERROR_DS_CANT_DERIVE_SPN_FOR_DELETED_DOMAIN: u32 = 0x219B;
pub const ERROR_DS_CANT_DEMOTE_WITH_WRITEABLE_NC: u32 = 0x219C;
pub const ERROR_DS_DUPLICATE_ID_FOUND: u32 = 0x219D;
pub const ERROR_DS_INSUFFICIENT_ATTR_TO_CREATE_OBJECT: u32 = 0x219E;
pub const ERROR_DS_GROUP_CONVERSION_ERROR: u32 = 0x219F;
pub const ERROR_DS_CANT_MOVE_APP_BASIC_GROUP: u32 = 0x21A0;
pub const ERROR_DS_CANT_MOVE_APP_QUERY_GROUP: u32 = 0x21A1;
pub const ERROR_DS_ROLE_NOT_VERIFIED: u32 = 0x21A2;
pub const ERROR_DS_WKO_CONTAINER_CANNOT_BE_SPECIAL: u32 = 0x21A3;
pub const ERROR_DS_DOMAIN_RENAME_IN_PROGRESS: u32 = 0x21A4;
pub const ERROR_DS_EXISTING_AD_CHILD_NC: u32 = 0x21A5;
pub const ERROR_DS_REPL_LIFETIME_EXCEEDED: u32 = 0x21A6;
pub const ERROR_DS_DISALLOWED_IN_SYSTEM_CONTAINER: u32 = 0x21A7;
pub const ERROR_DS_LDAP_SEND_QUEUE_FULL: u32 = 0x21A8;
pub const ERROR_DS_DRA_OUT_SCHEDULE_WINDOW: u32 = 0x21A9;
pub const ERROR_DS_POLICY_NOT_KNOWN: u32 = 0x21AA;
pub const ERROR_NO_SITE_SETTINGS_OBJECT: u32 = 0x21AB;
pub const ERROR_NO_SECRETS: u32 = 0x21AC;
pub const ERROR_NO_WRITABLE_DC_FOUND: u32 = 0x21AD;
pub const ERROR_DS_NO_SERVER_OBJECT: u32 = 0x21AE;
pub const ERROR_DS_NO_NTDSA_OBJECT: u32 = 0x21AF;
pub const ERROR_DS_NON_ASQ_SEARCH: u32 = 0x21B0;
pub const ERROR_DS_AUDIT_FAILURE: u32 = 0x21B1;
pub const ERROR_DS_INVALID_SEARCH_FLAG_SUBTREE: u32 = 0x21B2;
pub const ERROR_DS_INVALID_SEARCH_FLAG_TUPLE: u32 = 0x21B3;
pub const ERROR_DS_HIERARCHY_TABLE_TOO_DEEP: u32 = 0x21B4;
pub const ERROR_DS_DRA_CORRUPT_UTD_VECTOR: u32 = 0x21B5;
pub const ERROR_DS_DRA_SECRETS_DENIED: u32 = 0x21B6;
pub const ERROR_DS_RESERVED_MAPI_ID: u32 = 0x21B7;
pub const ERROR_DS_MAPI_ID_NOT_AVAILABLE: u32 = 0x21B8;
pub const ERROR_DS_DRA_MISSING_KRBTGT_SECRET: u32 = 0x21B9;
pub const ERROR_DS_DOMAIN_NAME_EXISTS_IN_FOREST: u32 = 0x21BA;
pub const ERROR_DS_FLAT_NAME_EXISTS_IN_FOREST: u32 = 0x21BB;
pub const ERROR_INVALID_USER_PRINCIPAL_NAME: u32 = 0x21BC;
pub const ERROR_DS_OID_MAPPED_GROUP_CANT_HAVE_MEMBERS: u32 = 0x21BD;
pub const ERROR_DS_OID_NOT_FOUND: u32 = 0x21BE;
pub const ERROR_DS_DRA_RECYCLED_TARGET: u32 = 0x21BF;
pub const ERROR_DS_DISALLOWED_NC_REDIRECT: u32 = 0x21C0;
pub const ERROR_DS_HIGH_ADLDS_FFL: u32 = 0x21C1;
pub const ERROR_DS_HIGH_DSA_VERSION: u32 = 0x21C2;
pub const ERROR_DS_LOW_ADLDS_FFL: u32 = 0x21C3;
pub const ERROR_DOMAIN_SID_SAME_AS_LOCAL_WORKSTATION: u32 = 0x21C4;
pub const ERROR_DS_UNDELETE_SAM_VALIDATION_FAILED: u32 = 0x21C5;
pub const ERROR_INCORRECT_ACCOUNT_TYPE: u32 = 0x21C6;
pub const ERROR_DS_SPN_VALUE_NOT_UNIQUE_IN_FOREST: u32 = 0x21C7;
pub const ERROR_DS_UPN_VALUE_NOT_UNIQUE_IN_FOREST: u32 = 0x21C8;
pub const ERROR_DS_MISSING_FOREST_TRUST: u32 = 0x21C9;
pub const ERROR_DS_VALUE_KEY_NOT_UNIQUE: u32 = 0x21CA;
pub const ERROR_WEAK_WHFBKEY_BLOCKED: u32 = 0x21CB;
pub const DNS_ERROR_RESPONSE_CODES_BASE: u32 = 0x2328;
pub const DNS_ERROR_RCODE_NO_ERROR: u32 = 0x0;
pub const DNS_ERROR_MASK: u32 = 0x2328;
pub const DNS_ERROR_RCODE_FORMAT_ERROR: u32 = 0x2329;
pub const DNS_ERROR_RCODE_SERVER_FAILURE: u32 = 0x232A;
pub const DNS_ERROR_RCODE_NAME_ERROR: u32 = 0x232B;
pub const DNS_ERROR_RCODE_NOT_IMPLEMENTED: u32 = 0x232C;
pub const DNS_ERROR_RCODE_REFUSED: u32 = 0x232D;
pub const DNS_ERROR_RCODE_YXDOMAIN: u32 = 0x232E;
pub const DNS_ERROR_RCODE_YXRRSET: u32 = 0x232F;
pub const DNS_ERROR_RCODE_NXRRSET: u32 = 0x2330;
pub const DNS_ERROR_RCODE_NOTAUTH: u32 = 0x2331;
pub const DNS_ERROR_RCODE_NOTZONE: u32 = 0x2332;
pub const DNS_ERROR_RCODE_BADSIG: u32 = 0x2338;
pub const DNS_ERROR_RCODE_BADKEY: u32 = 0x2339;
pub const DNS_ERROR_RCODE_BADTIME: u32 = 0x233A;
pub const DNS_ERROR_RCODE_LAST: u32 = 0x233A;
pub const DNS_ERROR_DNSSEC_BASE: u32 = 0x238C;
pub const DNS_ERROR_KEYMASTER_REQUIRED: u32 = 0x238D;
pub const DNS_ERROR_NOT_ALLOWED_ON_SIGNED_ZONE: u32 = 0x238E;
pub const DNS_ERROR_NSEC3_INCOMPATIBLE_WITH_RSA_SHA1: u32 = 0x238F;
pub const DNS_ERROR_NOT_ENOUGH_SIGNING_KEY_DESCRIPTORS: u32 = 0x2390;
pub const DNS_ERROR_UNSUPPORTED_ALGORITHM: u32 = 0x2391;
pub const DNS_ERROR_INVALID_KEY_SIZE: u32 = 0x2392;
pub const DNS_ERROR_SIGNING_KEY_NOT_ACCESSIBLE: u32 = 0x2393;
pub const DNS_ERROR_KSP_DOES_NOT_SUPPORT_PROTECTION: u32 = 0x2394;
pub const DNS_ERROR_UNEXPECTED_DATA_PROTECTION_ERROR: u32 = 0x2395;
pub const DNS_ERROR_UNEXPECTED_CNG_ERROR: u32 = 0x2396;
pub const DNS_ERROR_UNKNOWN_SIGNING_PARAMETER_VERSION: u32 = 0x2397;
pub const DNS_ERROR_KSP_NOT_ACCESSIBLE: u32 = 0x2398;
pub const DNS_ERROR_TOO_MANY_SKDS: u32 = 0x2399;
pub const DNS_ERROR_INVALID_ROLLOVER_PERIOD: u32 = 0x239A;
pub const DNS_ERROR_INVALID_INITIAL_ROLLOVER_OFFSET: u32 = 0x239B;
pub const DNS_ERROR_ROLLOVER_IN_PROGRESS: u32 = 0x239C;
pub const DNS_ERROR_STANDBY_KEY_NOT_PRESENT: u32 = 0x239D;
pub const DNS_ERROR_NOT_ALLOWED_ON_ZSK: u32 = 0x239E;
pub const DNS_ERROR_NOT_ALLOWED_ON_ACTIVE_SKD: u32 = 0x239F;
pub const DNS_ERROR_ROLLOVER_ALREADY_QUEUED: u32 = 0x23A0;
pub const DNS_ERROR_NOT_ALLOWED_ON_UNSIGNED_ZONE: u32 = 0x23A1;
pub const DNS_ERROR_BAD_KEYMASTER: u32 = 0x23A2;
pub const DNS_ERROR_INVALID_SIGNATURE_VALIDITY_PERIOD: u32 = 0x23A3;
pub const DNS_ERROR_INVALID_NSEC3_ITERATION_COUNT: u32 = 0x23A4;
pub const DNS_ERROR_DNSSEC_IS_DISABLED: u32 = 0x23A5;
pub const DNS_ERROR_INVALID_XML: u32 = 0x23A6;
pub const DNS_ERROR_NO_VALID_TRUST_ANCHORS: u32 = 0x23A7;
pub const DNS_ERROR_ROLLOVER_NOT_POKEABLE: u32 = 0x23A8;
pub const DNS_ERROR_NSEC3_NAME_COLLISION: u32 = 0x23A9;
pub const DNS_ERROR_NSEC_INCOMPATIBLE_WITH_NSEC3_RSA_SHA1: u32 = 0x23AA;
pub const DNS_ERROR_PACKET_FMT_BASE: u32 = 0x251C;
pub const DNS_ERROR_BAD_PACKET: u32 = 0x251E;
pub const DNS_ERROR_NO_PACKET: u32 = 0x251F;
pub const DNS_ERROR_RCODE: u32 = 0x2520;
pub const DNS_ERROR_UNSECURE_PACKET: u32 = 0x2521;
pub const DNS_ERROR_NO_MEMORY: u32 = 0xE;
pub const DNS_ERROR_INVALID_NAME: u32 = 0x7B;
pub const DNS_ERROR_INVALID_DATA: u32 = 0xD;
pub const DNS_ERROR_GENERAL_API_BASE: u32 = 0x254E;
pub const DNS_ERROR_INVALID_TYPE: u32 = 0x254F;
pub const DNS_ERROR_INVALID_IP_ADDRESS: u32 = 0x2550;
pub const DNS_ERROR_INVALID_PROPERTY: u32 = 0x2551;
pub const DNS_ERROR_TRY_AGAIN_LATER: u32 = 0x2552;
pub const DNS_ERROR_NOT_UNIQUE: u32 = 0x2553;
pub const DNS_ERROR_NON_RFC_NAME: u32 = 0x2554;
pub const DNS_ERROR_INVALID_NAME_CHAR: u32 = 0x2558;
pub const DNS_ERROR_NUMERIC_NAME: u32 = 0x2559;
pub const DNS_ERROR_NOT_ALLOWED_ON_ROOT_SERVER: u32 = 0x255A;
pub const DNS_ERROR_NOT_ALLOWED_UNDER_DELEGATION: u32 = 0x255B;
pub const DNS_ERROR_CANNOT_FIND_ROOT_HINTS: u32 = 0x255C;
pub const DNS_ERROR_INCONSISTENT_ROOT_HINTS: u32 = 0x255D;
pub const DNS_ERROR_DWORD_VALUE_TOO_SMALL: u32 = 0x255E;
pub const DNS_ERROR_DWORD_VALUE_TOO_LARGE: u32 = 0x255F;
pub const DNS_ERROR_BACKGROUND_LOADING: u32 = 0x2560;
pub const DNS_ERROR_NOT_ALLOWED_ON_RODC: u32 = 0x2561;
pub const DNS_ERROR_NOT_ALLOWED_UNDER_DNAME: u32 = 0x2562;
pub const DNS_ERROR_DELEGATION_REQUIRED: u32 = 0x2563;
pub const DNS_ERROR_INVALID_POLICY_TABLE: u32 = 0x2564;
pub const DNS_ERROR_ADDRESS_REQUIRED: u32 = 0x2565;
pub const DNS_ERROR_ZONE_BASE: u32 = 0x2580;
pub const DNS_ERROR_ZONE_DOES_NOT_EXIST: u32 = 0x2581;
pub const DNS_ERROR_NO_ZONE_INFO: u32 = 0x2582;
pub const DNS_ERROR_INVALID_ZONE_OPERATION: u32 = 0x2583;
pub const DNS_ERROR_ZONE_CONFIGURATION_ERROR: u32 = 0x2584;
pub const DNS_ERROR_ZONE_HAS_NO_SOA_RECORD: u32 = 0x2585;
pub const DNS_ERROR_ZONE_HAS_NO_NS_RECORDS: u32 = 0x2586;
pub const DNS_ERROR_ZONE_LOCKED: u32 = 0x2587;
pub const DNS_ERROR_ZONE_CREATION_FAILED: u32 = 0x2588;
pub const DNS_ERROR_ZONE_ALREADY_EXISTS: u32 = 0x2589;
pub const DNS_ERROR_AUTOZONE_ALREADY_EXISTS: u32 = 0x258A;
pub const DNS_ERROR_INVALID_ZONE_TYPE: u32 = 0x258B;
pub const DNS_ERROR_SECONDARY_REQUIRES_MASTER_IP: u32 = 0x258C;
pub const DNS_ERROR_ZONE_NOT_SECONDARY: u32 = 0x258D;
pub const DNS_ERROR_NEED_SECONDARY_ADDRESSES: u32 = 0x258E;
pub const DNS_ERROR_WINS_INIT_FAILED: u32 = 0x258F;
pub const DNS_ERROR_NEED_WINS_SERVERS: u32 = 0x2590;
pub const DNS_ERROR_NBSTAT_INIT_FAILED: u32 = 0x2591;
pub const DNS_ERROR_SOA_DELETE_INVALID: u32 = 0x2592;
pub const DNS_ERROR_FORWARDER_ALREADY_EXISTS: u32 = 0x2593;
pub const DNS_ERROR_ZONE_REQUIRES_MASTER_IP: u32 = 0x2594;
pub const DNS_ERROR_ZONE_IS_SHUTDOWN: u32 = 0x2595;
pub const DNS_ERROR_ZONE_LOCKED_FOR_SIGNING: u32 = 0x2596;
pub const DNS_ERROR_DATAFILE_BASE: u32 = 0x25B2;
pub const DNS_ERROR_PRIMARY_REQUIRES_DATAFILE: u32 = 0x25B3;
pub const DNS_ERROR_INVALID_DATAFILE_NAME: u32 = 0x25B4;
pub const DNS_ERROR_DATAFILE_OPEN_FAILURE: u32 = 0x25B5;
pub const DNS_ERROR_FILE_WRITEBACK_FAILED: u32 = 0x25B6;
pub const DNS_ERROR_DATAFILE_PARSING: u32 = 0x25B7;
pub const DNS_ERROR_DATABASE_BASE: u32 = 0x25E4;
pub const DNS_ERROR_RECORD_DOES_NOT_EXIST: u32 = 0x25E5;
pub const DNS_ERROR_RECORD_FORMAT: u32 = 0x25E6;
pub const DNS_ERROR_NODE_CREATION_FAILED: u32 = 0x25E7;
pub const DNS_ERROR_UNKNOWN_RECORD_TYPE: u32 = 0x25E8;
pub const DNS_ERROR_RECORD_TIMED_OUT: u32 = 0x25E9;
pub const DNS_ERROR_NAME_NOT_IN_ZONE: u32 = 0x25EA;
pub const DNS_ERROR_CNAME_LOOP: u32 = 0x25EB;
pub const DNS_ERROR_NODE_IS_CNAME: u32 = 0x25EC;
pub const DNS_ERROR_CNAME_COLLISION: u32 = 0x25ED;
pub const DNS_ERROR_RECORD_ONLY_AT_ZONE_ROOT: u32 = 0x25EE;
pub const DNS_ERROR_RECORD_ALREADY_EXISTS: u32 = 0x25EF;
pub const DNS_ERROR_SECONDARY_DATA: u32 = 0x25F0;
pub const DNS_ERROR_NO_CREATE_CACHE_DATA: u32 = 0x25F1;
pub const DNS_ERROR_NAME_DOES_NOT_EXIST: u32 = 0x25F2;
pub const DNS_ERROR_DS_UNAVAILABLE: u32 = 0x25F5;
pub const DNS_ERROR_DS_ZONE_ALREADY_EXISTS: u32 = 0x25F6;
pub const DNS_ERROR_NO_BOOTFILE_IF_DS_ZONE: u32 = 0x25F7;
pub const DNS_ERROR_NODE_IS_DNAME: u32 = 0x25F8;
pub const DNS_ERROR_DNAME_COLLISION: u32 = 0x25F9;
pub const DNS_ERROR_ALIAS_LOOP: u32 = 0x25FA;
pub const DNS_ERROR_OPERATION_BASE: u32 = 0x2616;
pub const DNS_ERROR_AXFR: u32 = 0x2618;
pub const DNS_ERROR_SECURE_BASE: u32 = 0x2648;
pub const DNS_ERROR_SETUP_BASE: u32 = 0x267A;
pub const DNS_ERROR_NO_TCPIP: u32 = 0x267B;
pub const DNS_ERROR_NO_DNS_SERVERS: u32 = 0x267C;
pub const DNS_ERROR_DP_BASE: u32 = 0x26AC;
pub const DNS_ERROR_DP_DOES_NOT_EXIST: u32 = 0x26AD;
pub const DNS_ERROR_DP_ALREADY_EXISTS: u32 = 0x26AE;
pub const DNS_ERROR_DP_NOT_ENLISTED: u32 = 0x26AF;
pub const DNS_ERROR_DP_ALREADY_ENLISTED: u32 = 0x26B0;
pub const DNS_ERROR_DP_NOT_AVAILABLE: u32 = 0x26B1;
pub const DNS_ERROR_DP_FSMO_ERROR: u32 = 0x26B2;
pub const DNS_ERROR_RRL_NOT_ENABLED: u32 = 0x26B7;
pub const DNS_ERROR_RRL_INVALID_WINDOW_SIZE: u32 = 0x26B8;
pub const DNS_ERROR_RRL_INVALID_IPV4_PREFIX: u32 = 0x26B9;
pub const DNS_ERROR_RRL_INVALID_IPV6_PREFIX: u32 = 0x26BA;
pub const DNS_ERROR_RRL_INVALID_TC_RATE: u32 = 0x26BB;
pub const DNS_ERROR_RRL_INVALID_LEAK_RATE: u32 = 0x26BC;
pub const DNS_ERROR_RRL_LEAK_RATE_LESSTHAN_TC_RATE: u32 = 0x26BD;
pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_ALREADY_EXISTS: u32 = 0x26C1;
pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_DOES_NOT_EXIST: u32 = 0x26C2;
pub const DNS_ERROR_VIRTUALIZATION_TREE_LOCKED: u32 = 0x26C3;
pub const DNS_ERROR_INVAILD_VIRTUALIZATION_INSTANCE_NAME: u32 = 0x26C4;
pub const DNS_ERROR_DEFAULT_VIRTUALIZATION_INSTANCE: u32 = 0x26C5;
pub const DNS_ERROR_ZONESCOPE_ALREADY_EXISTS: u32 = 0x26DF;
pub const DNS_ERROR_ZONESCOPE_DOES_NOT_EXIST: u32 = 0x26E0;
pub const DNS_ERROR_DEFAULT_ZONESCOPE: u32 = 0x26E1;
pub const DNS_ERROR_INVALID_ZONESCOPE_NAME: u32 = 0x26E2;
pub const DNS_ERROR_NOT_ALLOWED_WITH_ZONESCOPES: u32 = 0x26E3;
pub const DNS_ERROR_LOAD_ZONESCOPE_FAILED: u32 = 0x26E4;
pub const DNS_ERROR_ZONESCOPE_FILE_WRITEBACK_FAILED: u32 = 0x26E5;
pub const DNS_ERROR_INVALID_SCOPE_NAME: u32 = 0x26E6;
pub const DNS_ERROR_SCOPE_DOES_NOT_EXIST: u32 = 0x26E7;
pub const DNS_ERROR_DEFAULT_SCOPE: u32 = 0x26E8;
pub const DNS_ERROR_INVALID_SCOPE_OPERATION: u32 = 0x26E9;
pub const DNS_ERROR_SCOPE_LOCKED: u32 = 0x26EA;
pub const DNS_ERROR_SCOPE_ALREADY_EXISTS: u32 = 0x26EB;
pub const DNS_ERROR_POLICY_ALREADY_EXISTS: u32 = 0x26F3;
pub const DNS_ERROR_POLICY_DOES_NOT_EXIST: u32 = 0x26F4;
pub const DNS_ERROR_POLICY_INVALID_CRITERIA: u32 = 0x26F5;
pub const DNS_ERROR_POLICY_INVALID_SETTINGS: u32 = 0x26F6;
pub const DNS_ERROR_CLIENT_SUBNET_IS_ACCESSED: u32 = 0x26F7;
pub const DNS_ERROR_CLIENT_SUBNET_DOES_NOT_EXIST: u32 = 0x26F8;
pub const DNS_ERROR_CLIENT_SUBNET_ALREADY_EXISTS: u32 = 0x26F9;
pub const DNS_ERROR_SUBNET_DOES_NOT_EXIST: u32 = 0x26FA;
pub const DNS_ERROR_SUBNET_ALREADY_EXISTS: u32 = 0x26FB;
pub const DNS_ERROR_POLICY_LOCKED: u32 = 0x26FC;
pub const DNS_ERROR_POLICY_INVALID_WEIGHT: u32 = 0x26FD;
pub const DNS_ERROR_POLICY_INVALID_NAME: u32 = 0x26FE;
pub const DNS_ERROR_POLICY_MISSING_CRITERIA: u32 = 0x26FF;
pub const DNS_ERROR_INVALID_CLIENT_SUBNET_NAME: u32 = 0x2700;
pub const DNS_ERROR_POLICY_PROCESSING_ORDER_INVALID: u32 = 0x2701;
pub const DNS_ERROR_POLICY_SCOPE_MISSING: u32 = 0x2702;
pub const DNS_ERROR_POLICY_SCOPE_NOT_ALLOWED: u32 = 0x2703;
pub const DNS_ERROR_SERVERSCOPE_IS_REFERENCED: u32 = 0x2704;
pub const DNS_ERROR_ZONESCOPE_IS_REFERENCED: u32 = 0x2705;
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_CLIENT_SUBNET: u32 = 0x2706;
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TRANSPORT_PROTOCOL: u32 = 0x2707;
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_NETWORK_PROTOCOL: u32 = 0x2708;
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_INTERFACE: u32 = 0x2709;
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_FQDN: u32 = 0x270A;
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_QUERY_TYPE: u32 = 0x270B;
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TIME_OF_DAY: u32 = 0x270C;
pub const ERROR_IPSEC_QM_POLICY_EXISTS: u32 = 0x32C8;
pub const ERROR_IPSEC_QM_POLICY_NOT_FOUND: u32 = 0x32C9;
pub const ERROR_IPSEC_QM_POLICY_IN_USE: u32 = 0x32CA;
pub const ERROR_IPSEC_MM_POLICY_EXISTS: u32 = 0x32CB;
pub const ERROR_IPSEC_MM_POLICY_NOT_FOUND: u32 = 0x32CC;
pub const ERROR_IPSEC_MM_POLICY_IN_USE: u32 = 0x32CD;
pub const ERROR_IPSEC_MM_FILTER_EXISTS: u32 = 0x32CE;
pub const ERROR_IPSEC_MM_FILTER_NOT_FOUND: u32 = 0x32CF;
pub const ERROR_IPSEC_TRANSPORT_FILTER_EXISTS: u32 = 0x32D0;
pub const ERROR_IPSEC_TRANSPORT_FILTER_NOT_FOUND: u32 = 0x32D1;
pub const ERROR_IPSEC_MM_AUTH_EXISTS: u32 = 0x32D2;
pub const ERROR_IPSEC_MM_AUTH_NOT_FOUND: u32 = 0x32D3;
pub const ERROR_IPSEC_MM_AUTH_IN_USE: u32 = 0x32D4;
pub const ERROR_IPSEC_DEFAULT_MM_POLICY_NOT_FOUND: u32 = 0x32D5;
pub const ERROR_IPSEC_DEFAULT_MM_AUTH_NOT_FOUND: u32 = 0x32D6;
pub const ERROR_IPSEC_DEFAULT_QM_POLICY_NOT_FOUND: u32 = 0x32D7;
pub const ERROR_IPSEC_TUNNEL_FILTER_EXISTS: u32 = 0x32D8;
pub const ERROR_IPSEC_TUNNEL_FILTER_NOT_FOUND: u32 = 0x32D9;
pub const ERROR_IPSEC_MM_FILTER_PENDING_DELETION: u32 = 0x32DA;
pub const ERROR_IPSEC_TRANSPORT_FILTER_PENDING_DELETION: u32 = 0x32DB;
pub const ERROR_IPSEC_TUNNEL_FILTER_PENDING_DELETION: u32 = 0x32DC;
pub const ERROR_IPSEC_MM_POLICY_PENDING_DELETION: u32 = 0x32DD;
pub const ERROR_IPSEC_MM_AUTH_PENDING_DELETION: u32 = 0x32DE;
pub const ERROR_IPSEC_QM_POLICY_PENDING_DELETION: u32 = 0x32DF;
pub const ERROR_IPSEC_IKE_NEG_STATUS_BEGIN: u32 = 0x35E8;
pub const ERROR_IPSEC_IKE_AUTH_FAIL: u32 = 0x35E9;
pub const ERROR_IPSEC_IKE_ATTRIB_FAIL: u32 = 0x35EA;
pub const ERROR_IPSEC_IKE_NEGOTIATION_PENDING: u32 = 0x35EB;
pub const ERROR_IPSEC_IKE_GENERAL_PROCESSING_ERROR: u32 = 0x35EC;
pub const ERROR_IPSEC_IKE_TIMED_OUT: u32 = 0x35ED;
pub const ERROR_IPSEC_IKE_NO_CERT: u32 = 0x35EE;
pub const ERROR_IPSEC_IKE_SA_DELETED: u32 = 0x35EF;
pub const ERROR_IPSEC_IKE_SA_REAPED: u32 = 0x35F0;
pub const ERROR_IPSEC_IKE_MM_ACQUIRE_DROP: u32 = 0x35F1;
pub const ERROR_IPSEC_IKE_QM_ACQUIRE_DROP: u32 = 0x35F2;
pub const ERROR_IPSEC_IKE_QUEUE_DROP_MM: u32 = 0x35F3;
pub const ERROR_IPSEC_IKE_QUEUE_DROP_NO_MM: u32 = 0x35F4;
pub const ERROR_IPSEC_IKE_DROP_NO_RESPONSE: u32 = 0x35F5;
pub const ERROR_IPSEC_IKE_MM_DELAY_DROP: u32 = 0x35F6;
pub const ERROR_IPSEC_IKE_QM_DELAY_DROP: u32 = 0x35F7;
pub const ERROR_IPSEC_IKE_ERROR: u32 = 0x35F8;
pub const ERROR_IPSEC_IKE_CRL_FAILED: u32 = 0x35F9;
pub const ERROR_IPSEC_IKE_INVALID_KEY_USAGE: u32 = 0x35FA;
pub const ERROR_IPSEC_IKE_INVALID_CERT_TYPE: u32 = 0x35FB;
pub const ERROR_IPSEC_IKE_NO_PRIVATE_KEY: u32 = 0x35FC;
pub const ERROR_IPSEC_IKE_SIMULTANEOUS_REKEY: u32 = 0x35FD;
pub const ERROR_IPSEC_IKE_DH_FAIL: u32 = 0x35FE;
pub const ERROR_IPSEC_IKE_CRITICAL_PAYLOAD_NOT_RECOGNIZED: u32 = 0x35FF;
pub const ERROR_IPSEC_IKE_INVALID_HEADER: u32 = 0x3600;
pub const ERROR_IPSEC_IKE_NO_POLICY: u32 = 0x3601;
pub const ERROR_IPSEC_IKE_INVALID_SIGNATURE: u32 = 0x3602;
pub const ERROR_IPSEC_IKE_KERBEROS_ERROR: u32 = 0x3603;
pub const ERROR_IPSEC_IKE_NO_PUBLIC_KEY: u32 = 0x3604;
pub const ERROR_IPSEC_IKE_PROCESS_ERR: u32 = 0x3605;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_SA: u32 = 0x3606;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_PROP: u32 = 0x3607;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_TRANS: u32 = 0x3608;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_KE: u32 = 0x3609;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_ID: u32 = 0x360A;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT: u32 = 0x360B;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT_REQ: u32 = 0x360C;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_HASH: u32 = 0x360D;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_SIG: u32 = 0x360E;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NONCE: u32 = 0x360F;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NOTIFY: u32 = 0x3610;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_DELETE: u32 = 0x3611;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_VENDOR: u32 = 0x3612;
pub const ERROR_IPSEC_IKE_INVALID_PAYLOAD: u32 = 0x3613;
pub const ERROR_IPSEC_IKE_LOAD_SOFT_SA: u32 = 0x3614;
pub const ERROR_IPSEC_IKE_SOFT_SA_TORN_DOWN: u32 = 0x3615;
pub const ERROR_IPSEC_IKE_INVALID_COOKIE: u32 = 0x3616;
pub const ERROR_IPSEC_IKE_NO_PEER_CERT: u32 = 0x3617;
pub const ERROR_IPSEC_IKE_PEER_CRL_FAILED: u32 = 0x3618;
pub const ERROR_IPSEC_IKE_POLICY_CHANGE: u32 = 0x3619;
pub const ERROR_IPSEC_IKE_NO_MM_POLICY: u32 = 0x361A;
pub const ERROR_IPSEC_IKE_NOTCBPRIV: u32 = 0x361B;
pub const ERROR_IPSEC_IKE_SECLOADFAIL: u32 = 0x361C;
pub const ERROR_IPSEC_IKE_FAILSSPINIT: u32 = 0x361D;
pub const ERROR_IPSEC_IKE_FAILQUERYSSP: u32 = 0x361E;
pub const ERROR_IPSEC_IKE_SRVACQFAIL: u32 = 0x361F;
pub const ERROR_IPSEC_IKE_SRVQUERYCRED: u32 = 0x3620;
pub const ERROR_IPSEC_IKE_GETSPIFAIL: u32 = 0x3621;
pub const ERROR_IPSEC_IKE_INVALID_FILTER: u32 = 0x3622;
pub const ERROR_IPSEC_IKE_OUT_OF_MEMORY: u32 = 0x3623;
pub const ERROR_IPSEC_IKE_ADD_UPDATE_KEY_FAILED: u32 = 0x3624;
pub const ERROR_IPSEC_IKE_INVALID_POLICY: u32 = 0x3625;
pub const ERROR_IPSEC_IKE_UNKNOWN_DOI: u32 = 0x3626;
pub const ERROR_IPSEC_IKE_INVALID_SITUATION: u32 = 0x3627;
pub const ERROR_IPSEC_IKE_DH_FAILURE: u32 = 0x3628;
pub const ERROR_IPSEC_IKE_INVALID_GROUP: u32 = 0x3629;
pub const ERROR_IPSEC_IKE_ENCRYPT: u32 = 0x362A;
pub const ERROR_IPSEC_IKE_DECRYPT: u32 = 0x362B;
pub const ERROR_IPSEC_IKE_POLICY_MATCH: u32 = 0x362C;
pub const ERROR_IPSEC_IKE_UNSUPPORTED_ID: u32 = 0x362D;
pub const ERROR_IPSEC_IKE_INVALID_HASH: u32 = 0x362E;
pub const ERROR_IPSEC_IKE_INVALID_HASH_ALG: u32 = 0x362F;
pub const ERROR_IPSEC_IKE_INVALID_HASH_SIZE: u32 = 0x3630;
pub const ERROR_IPSEC_IKE_INVALID_ENCRYPT_ALG: u32 = 0x3631;
pub const ERROR_IPSEC_IKE_INVALID_AUTH_ALG: u32 = 0x3632;
pub const ERROR_IPSEC_IKE_INVALID_SIG: u32 = 0x3633;
pub const ERROR_IPSEC_IKE_LOAD_FAILED: u32 = 0x3634;
pub const ERROR_IPSEC_IKE_RPC_DELETE: u32 = 0x3635;
pub const ERROR_IPSEC_IKE_BENIGN_REINIT: u32 = 0x3636;
pub const ERROR_IPSEC_IKE_INVALID_RESPONDER_LIFETIME_NOTIFY: u32 = 0x3637;
pub const ERROR_IPSEC_IKE_INVALID_MAJOR_VERSION: u32 = 0x3638;
pub const ERROR_IPSEC_IKE_INVALID_CERT_KEYLEN: u32 = 0x3639;
pub const ERROR_IPSEC_IKE_MM_LIMIT: u32 = 0x363A;
pub const ERROR_IPSEC_IKE_NEGOTIATION_DISABLED: u32 = 0x363B;
pub const ERROR_IPSEC_IKE_QM_LIMIT: u32 = 0x363C;
pub const ERROR_IPSEC_IKE_MM_EXPIRED: u32 = 0x363D;
pub const ERROR_IPSEC_IKE_PEER_MM_ASSUMED_INVALID: u32 = 0x363E;
pub const ERROR_IPSEC_IKE_CERT_CHAIN_POLICY_MISMATCH: u32 = 0x363F;
pub const ERROR_IPSEC_IKE_UNEXPECTED_MESSAGE_ID: u32 = 0x3640;
pub const ERROR_IPSEC_IKE_INVALID_AUTH_PAYLOAD: u32 = 0x3641;
pub const ERROR_IPSEC_IKE_DOS_COOKIE_SENT: u32 = 0x3642;
pub const ERROR_IPSEC_IKE_SHUTTING_DOWN: u32 = 0x3643;
pub const ERROR_IPSEC_IKE_CGA_AUTH_FAILED: u32 = 0x3644;
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NATOA: u32 = 0x3645;
pub const ERROR_IPSEC_IKE_INVALID_MM_FOR_QM: u32 = 0x3646;
pub const ERROR_IPSEC_IKE_QM_EXPIRED: u32 = 0x3647;
pub const ERROR_IPSEC_IKE_TOO_MANY_FILTERS: u32 = 0x3648;
pub const ERROR_IPSEC_IKE_NEG_STATUS_END: u32 = 0x3649;
pub const ERROR_IPSEC_IKE_KILL_DUMMY_NAP_TUNNEL: u32 = 0x364A;
pub const ERROR_IPSEC_IKE_INNER_IP_ASSIGNMENT_FAILURE: u32 = 0x364B;
pub const ERROR_IPSEC_IKE_REQUIRE_CP_PAYLOAD_MISSING: u32 = 0x364C;
pub const ERROR_IPSEC_KEY_MODULE_IMPERSONATION_NEGOTIATION_PENDING: u32 = 0x364D;
pub const ERROR_IPSEC_IKE_COEXISTENCE_SUPPRESS: u32 = 0x364E;
pub const ERROR_IPSEC_IKE_RATELIMIT_DROP: u32 = 0x364F;
pub const ERROR_IPSEC_IKE_PEER_DOESNT_SUPPORT_MOBIKE: u32 = 0x3650;
pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE: u32 = 0x3651;
pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_FAILURE: u32 = 0x3652;
pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE_WITH_OPTIONAL_RETRY: u32 = 0x3653;
pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_AND_CERTMAP_FAILURE: u32 = 0x3654;
pub const ERROR_IPSEC_IKE_NEG_STATUS_EXTENDED_END: u32 = 0x3655;
pub const ERROR_IPSEC_BAD_SPI: u32 = 0x3656;
pub const ERROR_IPSEC_SA_LIFETIME_EXPIRED: u32 = 0x3657;
pub const ERROR_IPSEC_WRONG_SA: u32 = 0x3658;
pub const ERROR_IPSEC_REPLAY_CHECK_FAILED: u32 = 0x3659;
pub const ERROR_IPSEC_INVALID_PACKET: u32 = 0x365A;
pub const ERROR_IPSEC_INTEGRITY_CHECK_FAILED: u32 = 0x365B;
pub const ERROR_IPSEC_CLEAR_TEXT_DROP: u32 = 0x365C;
pub const ERROR_IPSEC_AUTH_FIREWALL_DROP: u32 = 0x365D;
pub const ERROR_IPSEC_THROTTLE_DROP: u32 = 0x365E;
pub const ERROR_IPSEC_DOSP_BLOCK: u32 = 0x3665;
pub const ERROR_IPSEC_DOSP_RECEIVED_MULTICAST: u32 = 0x3666;
pub const ERROR_IPSEC_DOSP_INVALID_PACKET: u32 = 0x3667;
pub const ERROR_IPSEC_DOSP_STATE_LOOKUP_FAILED: u32 = 0x3668;
pub const ERROR_IPSEC_DOSP_MAX_ENTRIES: u32 = 0x3669;
pub const ERROR_IPSEC_DOSP_KEYMOD_NOT_ALLOWED: u32 = 0x366A;
pub const ERROR_IPSEC_DOSP_NOT_INSTALLED: u32 = 0x366B;
pub const ERROR_IPSEC_DOSP_MAX_PER_IP_RATELIMIT_QUEUES: u32 = 0x366C;
pub const ERROR_SXS_SECTION_NOT_FOUND: u32 = 0x36B0;
pub const ERROR_SXS_CANT_GEN_ACTCTX: u32 = 0x36B1;
pub const ERROR_SXS_INVALID_ACTCTXDATA_FORMAT: u32 = 0x36B2;
pub const ERROR_SXS_ASSEMBLY_NOT_FOUND: u32 = 0x36B3;
pub const ERROR_SXS_MANIFEST_FORMAT_ERROR: u32 = 0x36B4;
pub const ERROR_SXS_MANIFEST_PARSE_ERROR: u32 = 0x36B5;
pub const ERROR_SXS_ACTIVATION_CONTEXT_DISABLED: u32 = 0x36B6;
pub const ERROR_SXS_KEY_NOT_FOUND: u32 = 0x36B7;
pub const ERROR_SXS_VERSION_CONFLICT: u32 = 0x36B8;
pub const ERROR_SXS_WRONG_SECTION_TYPE: u32 = 0x36B9;
pub const ERROR_SXS_THREAD_QUERIES_DISABLED: u32 = 0x36BA;
pub const ERROR_SXS_PROCESS_DEFAULT_ALREADY_SET: u32 = 0x36BB;
pub const ERROR_SXS_UNKNOWN_ENCODING_GROUP: u32 = 0x36BC;
pub const ERROR_SXS_UNKNOWN_ENCODING: u32 = 0x36BD;
pub const ERROR_SXS_INVALID_XML_NAMESPACE_URI: u32 = 0x36BE;
pub const ERROR_SXS_ROOT_MANIFEST_DEPENDENCY_NOT_INSTALLED: u32 = 0x36BF;
pub const ERROR_SXS_LEAF_MANIFEST_DEPENDENCY_NOT_INSTALLED: u32 = 0x36C0;
pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE: u32 = 0x36C1;
pub const ERROR_SXS_MANIFEST_MISSING_REQUIRED_DEFAULT_NAMESPACE: u32 = 0x36C2;
pub const ERROR_SXS_MANIFEST_INVALID_REQUIRED_DEFAULT_NAMESPACE: u32 = 0x36C3;
pub const ERROR_SXS_PRIVATE_MANIFEST_CROSS_PATH_WITH_REPARSE_POINT: u32 = 0x36C4;
pub const ERROR_SXS_DUPLICATE_DLL_NAME: u32 = 0x36C5;
pub const ERROR_SXS_DUPLICATE_WINDOWCLASS_NAME: u32 = 0x36C6;
pub const ERROR_SXS_DUPLICATE_CLSID: u32 = 0x36C7;
pub const ERROR_SXS_DUPLICATE_IID: u32 = 0x36C8;
pub const ERROR_SXS_DUPLICATE_TLBID: u32 = 0x36C9;
pub const ERROR_SXS_DUPLICATE_PROGID: u32 = 0x36CA;
pub const ERROR_SXS_DUPLICATE_ASSEMBLY_NAME: u32 = 0x36CB;
pub const ERROR_SXS_FILE_HASH_MISMATCH: u32 = 0x36CC;
pub const ERROR_SXS_POLICY_PARSE_ERROR: u32 = 0x36CD;
pub const ERROR_SXS_XML_E_MISSINGQUOTE: u32 = 0x36CE;
pub const ERROR_SXS_XML_E_COMMENTSYNTAX: u32 = 0x36CF;
pub const ERROR_SXS_XML_E_BADSTARTNAMECHAR: u32 = 0x36D0;
pub const ERROR_SXS_XML_E_BADNAMECHAR: u32 = 0x36D1;
pub const ERROR_SXS_XML_E_BADCHARINSTRING: u32 = 0x36D2;
pub const ERROR_SXS_XML_E_XMLDECLSYNTAX: u32 = 0x36D3;
pub const ERROR_SXS_XML_E_BADCHARDATA: u32 = 0x36D4;
pub const ERROR_SXS_XML_E_MISSINGWHITESPACE: u32 = 0x36D5;
pub const ERROR_SXS_XML_E_EXPECTINGTAGEND: u32 = 0x36D6;
pub const ERROR_SXS_XML_E_MISSINGSEMICOLON: u32 = 0x36D7;
pub const ERROR_SXS_XML_E_UNBALANCEDPAREN: u32 = 0x36D8;
pub const ERROR_SXS_XML_E_INTERNALERROR: u32 = 0x36D9;
pub const ERROR_SXS_XML_E_UNEXPECTED_WHITESPACE: u32 = 0x36DA;
pub const ERROR_SXS_XML_E_INCOMPLETE_ENCODING: u32 = 0x36DB;
pub const ERROR_SXS_XML_E_MISSING_PAREN: u32 = 0x36DC;
pub const ERROR_SXS_XML_E_EXPECTINGCLOSEQUOTE: u32 = 0x36DD;
pub const ERROR_SXS_XML_E_MULTIPLE_COLONS: u32 = 0x36DE;
pub const ERROR_SXS_XML_E_INVALID_DECIMAL: u32 = 0x36DF;
pub const ERROR_SXS_XML_E_INVALID_HEXIDECIMAL: u32 = 0x36E0;
pub const ERROR_SXS_XML_E_INVALID_UNICODE: u32 = 0x36E1;
pub const ERROR_SXS_XML_E_WHITESPACEORQUESTIONMARK: u32 = 0x36E2;
pub const ERROR_SXS_XML_E_UNEXPECTEDENDTAG: u32 = 0x36E3;
pub const ERROR_SXS_XML_E_UNCLOSEDTAG: u32 = 0x36E4;
pub const ERROR_SXS_XML_E_DUPLICATEATTRIBUTE: u32 = 0x36E5;
pub const ERROR_SXS_XML_E_MULTIPLEROOTS: u32 = 0x36E6;
pub const ERROR_SXS_XML_E_INVALIDATROOTLEVEL: u32 = 0x36E7;
pub const ERROR_SXS_XML_E_BADXMLDECL: u32 = 0x36E8;
pub const ERROR_SXS_XML_E_MISSINGROOT: u32 = 0x36E9;
pub const ERROR_SXS_XML_E_UNEXPECTEDEOF: u32 = 0x36EA;
pub const ERROR_SXS_XML_E_BADPEREFINSUBSET: u32 = 0x36EB;
pub const ERROR_SXS_XML_E_UNCLOSEDSTARTTAG: u32 = 0x36EC;
pub const ERROR_SXS_XML_E_UNCLOSEDENDTAG: u32 = 0x36ED;
pub const ERROR_SXS_XML_E_UNCLOSEDSTRING: u32 = 0x36EE;
pub const ERROR_SXS_XML_E_UNCLOSEDCOMMENT: u32 = 0x36EF;
pub const ERROR_SXS_XML_E_UNCLOSEDDECL: u32 = 0x36F0;
pub const ERROR_SXS_XML_E_UNCLOSEDCDATA: u32 = 0x36F1;
pub const ERROR_SXS_XML_E_RESERVEDNAMESPACE: u32 = 0x36F2;
pub const ERROR_SXS_XML_E_INVALIDENCODING: u32 = 0x36F3;
pub const ERROR_SXS_XML_E_INVALIDSWITCH: u32 = 0x36F4;
pub const ERROR_SXS_XML_E_BADXMLCASE: u32 = 0x36F5;
pub const ERROR_SXS_XML_E_INVALID_STANDALONE: u32 = 0x36F6;
pub const ERROR_SXS_XML_E_UNEXPECTED_STANDALONE: u32 = 0x36F7;
pub const ERROR_SXS_XML_E_INVALID_VERSION: u32 = 0x36F8;
pub const ERROR_SXS_XML_E_MISSINGEQUALS: u32 = 0x36F9;
pub const ERROR_SXS_PROTECTION_RECOVERY_FAILED: u32 = 0x36FA;
pub const ERROR_SXS_PROTECTION_PUBLIC_KEY_TOO_SHORT: u32 = 0x36FB;
pub const ERROR_SXS_PROTECTION_CATALOG_NOT_VALID: u32 = 0x36FC;
pub const ERROR_SXS_UNTRANSLATABLE_HRESULT: u32 = 0x36FD;
pub const ERROR_SXS_PROTECTION_CATALOG_FILE_MISSING: u32 = 0x36FE;
pub const ERROR_SXS_MISSING_ASSEMBLY_IDENTITY_ATTRIBUTE: u32 = 0x36FF;
pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE_NAME: u32 = 0x3700;
pub const ERROR_SXS_ASSEMBLY_MISSING: u32 = 0x3701;
pub const ERROR_SXS_CORRUPT_ACTIVATION_STACK: u32 = 0x3702;
pub const ERROR_SXS_CORRUPTION: u32 = 0x3703;
pub const ERROR_SXS_EARLY_DEACTIVATION: u32 = 0x3704;
pub const ERROR_SXS_INVALID_DEACTIVATION: u32 = 0x3705;
pub const ERROR_SXS_MULTIPLE_DEACTIVATION: u32 = 0x3706;
pub const ERROR_SXS_PROCESS_TERMINATION_REQUESTED: u32 = 0x3707;
pub const ERROR_SXS_RELEASE_ACTIVATION_CONTEXT: u32 = 0x3708;
pub const ERROR_SXS_SYSTEM_DEFAULT_ACTIVATION_CONTEXT_EMPTY: u32 = 0x3709;
pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_VALUE: u32 = 0x370A;
pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_NAME: u32 = 0x370B;
pub const ERROR_SXS_IDENTITY_DUPLICATE_ATTRIBUTE: u32 = 0x370C;
pub const ERROR_SXS_IDENTITY_PARSE_ERROR: u32 = 0x370D;
pub const ERROR_MALFORMED_SUBSTITUTION_STRING: u32 = 0x370E;
pub const ERROR_SXS_INCORRECT_PUBLIC_KEY_TOKEN: u32 = 0x370F;
pub const ERROR_UNMAPPED_SUBSTITUTION_STRING: u32 = 0x3710;
pub const ERROR_SXS_ASSEMBLY_NOT_LOCKED: u32 = 0x3711;
pub const ERROR_SXS_COMPONENT_STORE_CORRUPT: u32 = 0x3712;
pub const ERROR_ADVANCED_INSTALLER_FAILED: u32 = 0x3713;
pub const ERROR_XML_ENCODING_MISMATCH: u32 = 0x3714;
pub const ERROR_SXS_MANIFEST_IDENTITY_SAME_BUT_CONTENTS_DIFFERENT: u32 = 0x3715;
pub const ERROR_SXS_IDENTITIES_DIFFERENT: u32 = 0x3716;
pub const ERROR_SXS_ASSEMBLY_IS_NOT_A_DEPLOYMENT: u32 = 0x3717;
pub const ERROR_SXS_FILE_NOT_PART_OF_ASSEMBLY: u32 = 0x3718;
pub const ERROR_SXS_MANIFEST_TOO_BIG: u32 = 0x3719;
pub const ERROR_SXS_SETTING_NOT_REGISTERED: u32 = 0x371A;
pub const ERROR_SXS_TRANSACTION_CLOSURE_INCOMPLETE: u32 = 0x371B;
pub const ERROR_SMI_PRIMITIVE_INSTALLER_FAILED: u32 = 0x371C;
pub const ERROR_GENERIC_COMMAND_FAILED: u32 = 0x371D;
pub const ERROR_SXS_FILE_HASH_MISSING: u32 = 0x371E;
pub const ERROR_SXS_DUPLICATE_ACTIVATABLE_CLASS: u32 = 0x371F;
pub const ERROR_EVT_INVALID_CHANNEL_PATH: u32 = 0x3A98;
pub const ERROR_EVT_INVALID_QUERY: u32 = 0x3A99;
pub const ERROR_EVT_PUBLISHER_METADATA_NOT_FOUND: u32 = 0x3A9A;
pub const ERROR_EVT_EVENT_TEMPLATE_NOT_FOUND: u32 = 0x3A9B;
pub const ERROR_EVT_INVALID_PUBLISHER_NAME: u32 = 0x3A9C;
pub const ERROR_EVT_INVALID_EVENT_DATA: u32 = 0x3A9D;
pub const ERROR_EVT_CHANNEL_NOT_FOUND: u32 = 0x3A9F;
pub const ERROR_EVT_MALFORMED_XML_TEXT: u32 = 0x3AA0;
pub const ERROR_EVT_SUBSCRIPTION_TO_DIRECT_CHANNEL: u32 = 0x3AA1;
pub const ERROR_EVT_CONFIGURATION_ERROR: u32 = 0x3AA2;
pub const ERROR_EVT_QUERY_RESULT_STALE: u32 = 0x3AA3;
pub const ERROR_EVT_QUERY_RESULT_INVALID_POSITION: u32 = 0x3AA4;
pub const ERROR_EVT_NON_VALIDATING_MSXML: u32 = 0x3AA5;
pub const ERROR_EVT_FILTER_ALREADYSCOPED: u32 = 0x3AA6;
pub const ERROR_EVT_FILTER_NOTELTSET: u32 = 0x3AA7;
pub const ERROR_EVT_FILTER_INVARG: u32 = 0x3AA8;
pub const ERROR_EVT_FILTER_INVTEST: u32 = 0x3AA9;
pub const ERROR_EVT_FILTER_INVTYPE: u32 = 0x3AAA;
pub const ERROR_EVT_FILTER_PARSEERR: u32 = 0x3AAB;
pub const ERROR_EVT_FILTER_UNSUPPORTEDOP: u32 = 0x3AAC;
pub const ERROR_EVT_FILTER_UNEXPECTEDTOKEN: u32 = 0x3AAD;
pub const ERROR_EVT_INVALID_OPERATION_OVER_ENABLED_DIRECT_CHANNEL: u32 = 0x3AAE;
pub const ERROR_EVT_INVALID_CHANNEL_PROPERTY_VALUE: u32 = 0x3AAF;
pub const ERROR_EVT_INVALID_PUBLISHER_PROPERTY_VALUE: u32 = 0x3AB0;
pub const ERROR_EVT_CHANNEL_CANNOT_ACTIVATE: u32 = 0x3AB1;
pub const ERROR_EVT_FILTER_TOO_COMPLEX: u32 = 0x3AB2;
pub const ERROR_EVT_MESSAGE_NOT_FOUND: u32 = 0x3AB3;
pub const ERROR_EVT_MESSAGE_ID_NOT_FOUND: u32 = 0x3AB4;
pub const ERROR_EVT_UNRESOLVED_VALUE_INSERT: u32 = 0x3AB5;
pub const ERROR_EVT_UNRESOLVED_PARAMETER_INSERT: u32 = 0x3AB6;
pub const ERROR_EVT_MAX_INSERTS_REACHED: u32 = 0x3AB7;
pub const ERROR_EVT_EVENT_DEFINITION_NOT_FOUND: u32 = 0x3AB8;
pub const ERROR_EVT_MESSAGE_LOCALE_NOT_FOUND: u32 = 0x3AB9;
pub const ERROR_EVT_VERSION_TOO_OLD: u32 = 0x3ABA;
pub const ERROR_EVT_VERSION_TOO_NEW: u32 = 0x3ABB;
pub const ERROR_EVT_CANNOT_OPEN_CHANNEL_OF_QUERY: u32 = 0x3ABC;
pub const ERROR_EVT_PUBLISHER_DISABLED: u32 = 0x3ABD;
pub const ERROR_EVT_FILTER_OUT_OF_RANGE: u32 = 0x3ABE;
pub const ERROR_EC_SUBSCRIPTION_CANNOT_ACTIVATE: u32 = 0x3AE8;
pub const ERROR_EC_LOG_DISABLED: u32 = 0x3AE9;
pub const ERROR_EC_CIRCULAR_FORWARDING: u32 = 0x3AEA;
pub const ERROR_EC_CREDSTORE_FULL: u32 = 0x3AEB;
pub const ERROR_EC_CRED_NOT_FOUND: u32 = 0x3AEC;
pub const ERROR_EC_NO_ACTIVE_CHANNEL: u32 = 0x3AED;
pub const ERROR_MUI_FILE_NOT_FOUND: u32 = 0x3AFC;
pub const ERROR_MUI_INVALID_FILE: u32 = 0x3AFD;
pub const ERROR_MUI_INVALID_RC_CONFIG: u32 = 0x3AFE;
pub const ERROR_MUI_INVALID_LOCALE_NAME: u32 = 0x3AFF;
pub const ERROR_MUI_INVALID_ULTIMATEFALLBACK_NAME: u32 = 0x3B00;
pub const ERROR_MUI_FILE_NOT_LOADED: u32 = 0x3B01;
pub const ERROR_RESOURCE_ENUM_USER_STOP: u32 = 0x3B02;
pub const ERROR_MUI_INTLSETTINGS_UILANG_NOT_INSTALLED: u32 = 0x3B03;
pub const ERROR_MUI_INTLSETTINGS_INVALID_LOCALE_NAME: u32 = 0x3B04;
pub const ERROR_MRM_RUNTIME_NO_DEFAULT_OR_NEUTRAL_RESOURCE: u32 = 0x3B06;
pub const ERROR_MRM_INVALID_PRICONFIG: u32 = 0x3B07;
pub const ERROR_MRM_INVALID_FILE_TYPE: u32 = 0x3B08;
pub const ERROR_MRM_UNKNOWN_QUALIFIER: u32 = 0x3B09;
pub const ERROR_MRM_INVALID_QUALIFIER_VALUE: u32 = 0x3B0A;
pub const ERROR_MRM_NO_CANDIDATE: u32 = 0x3B0B;
pub const ERROR_MRM_NO_MATCH_OR_DEFAULT_CANDIDATE: u32 = 0x3B0C;
pub const ERROR_MRM_RESOURCE_TYPE_MISMATCH: u32 = 0x3B0D;
pub const ERROR_MRM_DUPLICATE_MAP_NAME: u32 = 0x3B0E;
pub const ERROR_MRM_DUPLICATE_ENTRY: u32 = 0x3B0F;
pub const ERROR_MRM_INVALID_RESOURCE_IDENTIFIER: u32 = 0x3B10;
pub const ERROR_MRM_FILEPATH_TOO_LONG: u32 = 0x3B11;
pub const ERROR_MRM_UNSUPPORTED_DIRECTORY_TYPE: u32 = 0x3B12;
pub const ERROR_MRM_INVALID_PRI_FILE: u32 = 0x3B16;
pub const ERROR_MRM_NAMED_RESOURCE_NOT_FOUND: u32 = 0x3B17;
pub const ERROR_MRM_MAP_NOT_FOUND: u32 = 0x3B1F;
pub const ERROR_MRM_UNSUPPORTED_PROFILE_TYPE: u32 = 0x3B20;
pub const ERROR_MRM_INVALID_QUALIFIER_OPERATOR: u32 = 0x3B21;
pub const ERROR_MRM_INDETERMINATE_QUALIFIER_VALUE: u32 = 0x3B22;
pub const ERROR_MRM_AUTOMERGE_ENABLED: u32 = 0x3B23;
pub const ERROR_MRM_TOO_MANY_RESOURCES: u32 = 0x3B24;
pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_MERGE: u32 = 0x3B25;
pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_LOAD_UNLOAD_PRI_FILE: u32 = 0x3B26;
pub const ERROR_MRM_NO_CURRENT_VIEW_ON_THREAD: u32 = 0x3B27;
pub const ERROR_DIFFERENT_PROFILE_RESOURCE_MANAGER_EXIST: u32 = 0x3B28;
pub const ERROR_OPERATION_NOT_ALLOWED_FROM_SYSTEM_COMPONENT: u32 = 0x3B29;
pub const ERROR_MRM_DIRECT_REF_TO_NON_DEFAULT_RESOURCE: u32 = 0x3B2A;
pub const ERROR_MRM_GENERATION_COUNT_MISMATCH: u32 = 0x3B2B;
pub const ERROR_PRI_MERGE_VERSION_MISMATCH: u32 = 0x3B2C;
pub const ERROR_PRI_MERGE_MISSING_SCHEMA: u32 = 0x3B2D;
pub const ERROR_PRI_MERGE_LOAD_FILE_FAILED: u32 = 0x3B2E;
pub const ERROR_PRI_MERGE_ADD_FILE_FAILED: u32 = 0x3B2F;
pub const ERROR_PRI_MERGE_WRITE_FILE_FAILED: u32 = 0x3B30;
pub const ERROR_PRI_MERGE_MULTIPLE_PACKAGE_FAMILIES_NOT_ALLOWED: u32 = 0x3B31;
pub const ERROR_PRI_MERGE_MULTIPLE_MAIN_PACKAGES_NOT_ALLOWED: u32 = 0x3B32;
pub const ERROR_PRI_MERGE_BUNDLE_PACKAGES_NOT_ALLOWED: u32 = 0x3B33;
pub const ERROR_PRI_MERGE_MAIN_PACKAGE_REQUIRED: u32 = 0x3B34;
pub const ERROR_PRI_MERGE_RESOURCE_PACKAGE_REQUIRED: u32 = 0x3B35;
pub const ERROR_PRI_MERGE_INVALID_FILE_NAME: u32 = 0x3B36;
pub const ERROR_MRM_PACKAGE_NOT_FOUND: u32 = 0x3B37;
pub const ERROR_MRM_MISSING_DEFAULT_LANGUAGE: u32 = 0x3B38;
pub const ERROR_MCA_INVALID_CAPABILITIES_STRING: u32 = 0x3B60;
pub const ERROR_MCA_INVALID_VCP_VERSION: u32 = 0x3B61;
pub const ERROR_MCA_MONITOR_VIOLATES_MCCS_SPECIFICATION: u32 = 0x3B62;
pub const ERROR_MCA_MCCS_VERSION_MISMATCH: u32 = 0x3B63;
pub const ERROR_MCA_UNSUPPORTED_MCCS_VERSION: u32 = 0x3B64;
pub const ERROR_MCA_INTERNAL_ERROR: u32 = 0x3B65;
pub const ERROR_MCA_INVALID_TECHNOLOGY_TYPE_RETURNED: u32 = 0x3B66;
pub const ERROR_MCA_UNSUPPORTED_COLOR_TEMPERATURE: u32 = 0x3B67;
pub const ERROR_AMBIGUOUS_SYSTEM_DEVICE: u32 = 0x3B92;
pub const ERROR_SYSTEM_DEVICE_NOT_FOUND: u32 = 0x3BC3;
pub const ERROR_HASH_NOT_SUPPORTED: u32 = 0x3BC4;
pub const ERROR_HASH_NOT_PRESENT: u32 = 0x3BC5;
pub const ERROR_SECONDARY_IC_PROVIDER_NOT_REGISTERED: u32 = 0x3BD9;
pub const ERROR_GPIO_CLIENT_INFORMATION_INVALID: u32 = 0x3BDA;
pub const ERROR_GPIO_VERSION_NOT_SUPPORTED: u32 = 0x3BDB;
pub const ERROR_GPIO_INVALID_REGISTRATION_PACKET: u32 = 0x3BDC;
pub const ERROR_GPIO_OPERATION_DENIED: u32 = 0x3BDD;
pub const ERROR_GPIO_INCOMPATIBLE_CONNECT_MODE: u32 = 0x3BDE;
pub const ERROR_GPIO_INTERRUPT_ALREADY_UNMASKED: u32 = 0x3BDF;
pub const ERROR_CANNOT_SWITCH_RUNLEVEL: u32 = 0x3C28;
pub const ERROR_INVALID_RUNLEVEL_SETTING: u32 = 0x3C29;
pub const ERROR_RUNLEVEL_SWITCH_TIMEOUT: u32 = 0x3C2A;
pub const ERROR_RUNLEVEL_SWITCH_AGENT_TIMEOUT: u32 = 0x3C2B;
pub const ERROR_RUNLEVEL_SWITCH_IN_PROGRESS: u32 = 0x3C2C;
pub const ERROR_SERVICES_FAILED_AUTOSTART: u32 = 0x3C2D;
pub const ERROR_COM_TASK_STOP_PENDING: u32 = 0x3C8D;
pub const ERROR_INSTALL_OPEN_PACKAGE_FAILED: u32 = 0x3CF0;
pub const ERROR_INSTALL_PACKAGE_NOT_FOUND: u32 = 0x3CF1;
pub const ERROR_INSTALL_INVALID_PACKAGE: u32 = 0x3CF2;
pub const ERROR_INSTALL_RESOLVE_DEPENDENCY_FAILED: u32 = 0x3CF3;
pub const ERROR_INSTALL_OUT_OF_DISK_SPACE: u32 = 0x3CF4;
pub const ERROR_INSTALL_NETWORK_FAILURE: u32 = 0x3CF5;
pub const ERROR_INSTALL_REGISTRATION_FAILURE: u32 = 0x3CF6;
pub const ERROR_INSTALL_DEREGISTRATION_FAILURE: u32 = 0x3CF7;
pub const ERROR_INSTALL_CANCEL: u32 = 0x3CF8;
pub const ERROR_INSTALL_FAILED: u32 = 0x3CF9;
pub const ERROR_REMOVE_FAILED: u32 = 0x3CFA;
pub const ERROR_PACKAGE_ALREADY_EXISTS: u32 = 0x3CFB;
pub const ERROR_NEEDS_REMEDIATION: u32 = 0x3CFC;
pub const ERROR_INSTALL_PREREQUISITE_FAILED: u32 = 0x3CFD;
pub const ERROR_PACKAGE_REPOSITORY_CORRUPTED: u32 = 0x3CFE;
pub const ERROR_INSTALL_POLICY_FAILURE: u32 = 0x3CFF;
pub const ERROR_PACKAGE_UPDATING: u32 = 0x3D00;
pub const ERROR_DEPLOYMENT_BLOCKED_BY_POLICY: u32 = 0x3D01;
pub const ERROR_PACKAGES_IN_USE: u32 = 0x3D02;
pub const ERROR_RECOVERY_FILE_CORRUPT: u32 = 0x3D03;
pub const ERROR_INVALID_STAGED_SIGNATURE: u32 = 0x3D04;
pub const ERROR_DELETING_EXISTING_APPLICATIONDATA_STORE_FAILED: u32 = 0x3D05;
pub const ERROR_INSTALL_PACKAGE_DOWNGRADE: u32 = 0x3D06;
pub const ERROR_SYSTEM_NEEDS_REMEDIATION: u32 = 0x3D07;
pub const ERROR_APPX_INTEGRITY_FAILURE_CLR_NGEN: u32 = 0x3D08;
pub const ERROR_RESILIENCY_FILE_CORRUPT: u32 = 0x3D09;
pub const ERROR_INSTALL_FIREWALL_SERVICE_NOT_RUNNING: u32 = 0x3D0A;
pub const ERROR_PACKAGE_MOVE_FAILED: u32 = 0x3D0B;
pub const ERROR_INSTALL_VOLUME_NOT_EMPTY: u32 = 0x3D0C;
pub const ERROR_INSTALL_VOLUME_OFFLINE: u32 = 0x3D0D;
pub const ERROR_INSTALL_VOLUME_CORRUPT: u32 = 0x3D0E;
pub const ERROR_NEEDS_REGISTRATION: u32 = 0x3D0F;
pub const ERROR_INSTALL_WRONG_PROCESSOR_ARCHITECTURE: u32 = 0x3D10;
pub const ERROR_DEV_SIDELOAD_LIMIT_EXCEEDED: u32 = 0x3D11;
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE: u32 = 0x3D12;
pub const ERROR_PACKAGE_NOT_SUPPORTED_ON_FILESYSTEM: u32 = 0x3D13;
pub const ERROR_PACKAGE_MOVE_BLOCKED_BY_STREAMING: u32 = 0x3D14;
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_APPLICATIONID_NOT_UNIQUE: u32 = 0x3D15;
pub const ERROR_PACKAGE_STAGING_ONHOLD: u32 = 0x3D16;
pub const ERROR_INSTALL_INVALID_RELATED_SET_UPDATE: u32 = 0x3D17;
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY: u32 = 0x3D18;
pub const ERROR_DEPLOYMENT_BLOCKED_BY_USER_LOG_OFF: u32 = 0x3D19;
pub const ERROR_PROVISION_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_PROVISIONED: u32 = 0x3D1A;
pub const ERROR_PACKAGES_REPUTATION_CHECK_FAILED: u32 = 0x3D1B;
pub const ERROR_PACKAGES_REPUTATION_CHECK_TIMEDOUT: u32 = 0x3D1C;
pub const ERROR_DEPLOYMENT_OPTION_NOT_SUPPORTED: u32 = 0x3D1D;
pub const ERROR_APPINSTALLER_ACTIVATION_BLOCKED: u32 = 0x3D1E;
pub const ERROR_REGISTRATION_FROM_REMOTE_DRIVE_NOT_SUPPORTED: u32 = 0x3D1F;
pub const ERROR_APPX_RAW_DATA_WRITE_FAILED: u32 = 0x3D20;
pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_PACKAGE: u32 = 0x3D21;
pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_MACHINE: u32 = 0x3D22;
pub const ERROR_DEPLOYMENT_BLOCKED_BY_PROFILE_POLICY: u32 = 0x3D23;
pub const ERROR_DEPLOYMENT_FAILED_CONFLICTING_MUTABLE_PACKAGE_DIRECTORY: u32 = 0x3D24;
pub const ERROR_SINGLETON_RESOURCE_INSTALLED_IN_ACTIVE_USER: u32 = 0x3D25;
pub const ERROR_DIFFERENT_VERSION_OF_PACKAGED_SERVICE_INSTALLED: u32 = 0x3D26;
pub const ERROR_SERVICE_EXISTS_AS_NON_PACKAGED_SERVICE: u32 = 0x3D27;
pub const ERROR_PACKAGED_SERVICE_REQUIRES_ADMIN_PRIVILEGES: u32 = 0x3D28;
pub const ERROR_REDIRECTION_TO_DEFAULT_ACCOUNT_NOT_ALLOWED: u32 = 0x3D29;
pub const ERROR_PACKAGE_LACKS_CAPABILITY_TO_DEPLOY_ON_HOST: u32 = 0x3D2A;
pub const ERROR_UNSIGNED_PACKAGE_INVALID_CONTENT: u32 = 0x3D2B;
pub const ERROR_UNSIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: u32 = 0x3D2C;
pub const ERROR_SIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: u32 = 0x3D2D;
pub const ERROR_PACKAGE_EXTERNAL_LOCATION_NOT_ALLOWED: u32 = 0x3D2E;
pub const ERROR_INSTALL_FULLTRUST_HOSTRUNTIME_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY: u32 = 0x3D2F;
pub const ERROR_PACKAGE_LACKS_CAPABILITY_FOR_MANDATORY_STARTUPTASKS: u32 = 0x3D30;
pub const ERROR_INSTALL_RESOLVE_HOSTRUNTIME_DEPENDENCY_FAILED: u32 = 0x3D31;
pub const ERROR_MACHINE_SCOPE_NOT_ALLOWED: u32 = 0x3D32;
pub const ERROR_CLASSIC_COMPAT_MODE_NOT_ALLOWED: u32 = 0x3D33;
pub const ERROR_STAGEFROMUPDATEAGENT_PACKAGE_NOT_APPLICABLE: u32 = 0x3D34;
pub const ERROR_PACKAGE_NOT_REGISTERED_FOR_USER: u32 = 0x3D35;
pub const ERROR_STATE_LOAD_STORE_FAILED: u32 = 0x3DB8;
pub const ERROR_STATE_GET_VERSION_FAILED: u32 = 0x3DB9;
pub const ERROR_STATE_SET_VERSION_FAILED: u32 = 0x3DBA;
pub const ERROR_STATE_STRUCTURED_RESET_FAILED: u32 = 0x3DBB;
pub const ERROR_STATE_OPEN_CONTAINER_FAILED: u32 = 0x3DBC;
pub const ERROR_STATE_CREATE_CONTAINER_FAILED: u32 = 0x3DBD;
pub const ERROR_STATE_DELETE_CONTAINER_FAILED: u32 = 0x3DBE;
pub const ERROR_STATE_READ_SETTING_FAILED: u32 = 0x3DBF;
pub const ERROR_STATE_WRITE_SETTING_FAILED: u32 = 0x3DC0;
pub const ERROR_STATE_DELETE_SETTING_FAILED: u32 = 0x3DC1;
pub const ERROR_STATE_QUERY_SETTING_FAILED: u32 = 0x3DC2;
pub const ERROR_STATE_READ_COMPOSITE_SETTING_FAILED: u32 = 0x3DC3;
pub const ERROR_STATE_WRITE_COMPOSITE_SETTING_FAILED: u32 = 0x3DC4;
pub const ERROR_STATE_ENUMERATE_CONTAINER_FAILED: u32 = 0x3DC5;
pub const ERROR_STATE_ENUMERATE_SETTINGS_FAILED: u32 = 0x3DC6;
pub const ERROR_STATE_COMPOSITE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: u32 = 0x3DC7;
pub const ERROR_STATE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: u32 = 0x3DC8;
pub const ERROR_STATE_SETTING_NAME_SIZE_LIMIT_EXCEEDED: u32 = 0x3DC9;
pub const ERROR_STATE_CONTAINER_NAME_SIZE_LIMIT_EXCEEDED: u32 = 0x3DCA;
pub const ERROR_API_UNAVAILABLE: u32 = 0x3DE1;
pub const ERROR_NDIS_INTERFACE_CLOSING: u32 = 0x80340002;
pub const ERROR_NDIS_BAD_VERSION: u32 = 0x80340004;
pub const ERROR_NDIS_BAD_CHARACTERISTICS: u32 = 0x80340005;
pub const ERROR_NDIS_ADAPTER_NOT_FOUND: u32 = 0x80340006;
pub const ERROR_NDIS_OPEN_FAILED: u32 = 0x80340007;
pub const ERROR_NDIS_DEVICE_FAILED: u32 = 0x80340008;
pub const ERROR_NDIS_MULTICAST_FULL: u32 = 0x80340009;
pub const ERROR_NDIS_MULTICAST_EXISTS: u32 = 0x8034000A;
pub const ERROR_NDIS_MULTICAST_NOT_FOUND: u32 = 0x8034000B;
pub const ERROR_NDIS_REQUEST_ABORTED: u32 = 0x8034000C;
pub const ERROR_NDIS_RESET_IN_PROGRESS: u32 = 0x8034000D;
pub const ERROR_NDIS_NOT_SUPPORTED: u32 = 0x803400BB;
pub const ERROR_NDIS_INVALID_PACKET: u32 = 0x8034000F;
pub const ERROR_NDIS_ADAPTER_NOT_READY: u32 = 0x80340011;
pub const ERROR_NDIS_INVALID_LENGTH: u32 = 0x80340014;
pub const ERROR_NDIS_INVALID_DATA: u32 = 0x80340015;
pub const ERROR_NDIS_BUFFER_TOO_SHORT: u32 = 0x80340016;
pub const ERROR_NDIS_INVALID_OID: u32 = 0x80340017;
pub const ERROR_NDIS_ADAPTER_REMOVED: u32 = 0x80340018;
pub const ERROR_NDIS_UNSUPPORTED_MEDIA: u32 = 0x80340019;
pub const ERROR_NDIS_GROUP_ADDRESS_IN_USE: u32 = 0x8034001A;
pub const ERROR_NDIS_FILE_NOT_FOUND: u32 = 0x8034001B;
pub const ERROR_NDIS_ERROR_READING_FILE: u32 = 0x8034001C;
pub const ERROR_NDIS_ALREADY_MAPPED: u32 = 0x8034001D;
pub const ERROR_NDIS_RESOURCE_CONFLICT: u32 = 0x8034001E;
pub const ERROR_NDIS_MEDIA_DISCONNECTED: u32 = 0x8034001F;
pub const ERROR_NDIS_INVALID_ADDRESS: u32 = 0x80340022;
pub const ERROR_NDIS_INVALID_DEVICE_REQUEST: u32 = 0x80340010;
pub const ERROR_NDIS_PAUSED: u32 = 0x8034002A;
pub const ERROR_NDIS_INTERFACE_NOT_FOUND: u32 = 0x8034002B;
pub const ERROR_NDIS_UNSUPPORTED_REVISION: u32 = 0x8034002C;
pub const ERROR_NDIS_INVALID_PORT: u32 = 0x8034002D;
pub const ERROR_NDIS_INVALID_PORT_STATE: u32 = 0x8034002E;
pub const ERROR_NDIS_LOW_POWER_STATE: u32 = 0x8034002F;
pub const ERROR_NDIS_REINIT_REQUIRED: u32 = 0x80340030;
pub const ERROR_NDIS_NO_QUEUES: u32 = 0x80340031;
pub const ERROR_NDIS_DOT11_AUTO_CONFIG_ENABLED: u32 = 0x80342000;
pub const ERROR_NDIS_DOT11_MEDIA_IN_USE: u32 = 0x80342001;
pub const ERROR_NDIS_DOT11_POWER_STATE_INVALID: u32 = 0x80342002;
pub const ERROR_NDIS_PM_WOL_PATTERN_LIST_FULL: u32 = 0x80342003;
pub const ERROR_NDIS_PM_PROTOCOL_OFFLOAD_LIST_FULL: u32 = 0x80342004;
pub const ERROR_NDIS_DOT11_AP_CHANNEL_CURRENTLY_NOT_AVAILABLE: u32 = 0x80342005;
pub const ERROR_NDIS_DOT11_AP_BAND_CURRENTLY_NOT_AVAILABLE: u32 = 0x80342006;
pub const ERROR_NDIS_DOT11_AP_CHANNEL_NOT_ALLOWED: u32 = 0x80342007;
pub const ERROR_NDIS_DOT11_AP_BAND_NOT_ALLOWED: u32 = 0x80342008;
pub const ERROR_NDIS_INDICATION_REQUIRED: u32 = 0x340001;
pub const ERROR_NDIS_OFFLOAD_POLICY: u32 = 0xC034100F;
pub const ERROR_NDIS_OFFLOAD_CONNECTION_REJECTED: u32 = 0xC0341012;
pub const ERROR_NDIS_OFFLOAD_PATH_REJECTED: u32 = 0xC0341013;
pub const ERROR_HV_INVALID_HYPERCALL_CODE: u32 = 0xC0350002;
pub const ERROR_HV_INVALID_HYPERCALL_INPUT: u32 = 0xC0350003;
pub const ERROR_HV_INVALID_ALIGNMENT: u32 = 0xC0350004;
pub const ERROR_HV_INVALID_PARAMETER: u32 = 0xC0350005;
pub const ERROR_HV_ACCESS_DENIED: u32 = 0xC0350006;
pub const ERROR_HV_INVALID_PARTITION_STATE: u32 = 0xC0350007;
pub const ERROR_HV_OPERATION_DENIED: u32 = 0xC0350008;
pub const ERROR_HV_UNKNOWN_PROPERTY: u32 = 0xC0350009;
pub const ERROR_HV_PROPERTY_VALUE_OUT_OF_RANGE: u32 = 0xC035000A;
pub const ERROR_HV_INSUFFICIENT_MEMORY: u32 = 0xC035000B;
pub const ERROR_HV_PARTITION_TOO_DEEP: u32 = 0xC035000C;
pub const ERROR_HV_INVALID_PARTITION_ID: u32 = 0xC035000D;
pub const ERROR_HV_INVALID_VP_INDEX: u32 = 0xC035000E;
pub const ERROR_HV_INVALID_PORT_ID: u32 = 0xC0350011;
pub const ERROR_HV_INVALID_CONNECTION_ID: u32 = 0xC0350012;
pub const ERROR_HV_INSUFFICIENT_BUFFERS: u32 = 0xC0350013;
pub const ERROR_HV_NOT_ACKNOWLEDGED: u32 = 0xC0350014;
pub const ERROR_HV_INVALID_VP_STATE: u32 = 0xC0350015;
pub const ERROR_HV_ACKNOWLEDGED: u32 = 0xC0350016;
pub const ERROR_HV_INVALID_SAVE_RESTORE_STATE: u32 = 0xC0350017;
pub const ERROR_HV_INVALID_SYNIC_STATE: u32 = 0xC0350018;
pub const ERROR_HV_OBJECT_IN_USE: u32 = 0xC0350019;
pub const ERROR_HV_INVALID_PROXIMITY_DOMAIN_INFO: u32 = 0xC035001A;
pub const ERROR_HV_NO_DATA: u32 = 0xC035001B;
pub const ERROR_HV_INACTIVE: u32 = 0xC035001C;
pub const ERROR_HV_NO_RESOURCES: u32 = 0xC035001D;
pub const ERROR_HV_FEATURE_UNAVAILABLE: u32 = 0xC035001E;
pub const ERROR_HV_INSUFFICIENT_BUFFER: u32 = 0xC0350033;
pub const ERROR_HV_INSUFFICIENT_DEVICE_DOMAINS: u32 = 0xC0350038;
pub const ERROR_HV_CPUID_FEATURE_VALIDATION: u32 = 0xC035003C;
pub const ERROR_HV_CPUID_XSAVE_FEATURE_VALIDATION: u32 = 0xC035003D;
pub const ERROR_HV_PROCESSOR_STARTUP_TIMEOUT: u32 = 0xC035003E;
pub const ERROR_HV_SMX_ENABLED: u32 = 0xC035003F;
pub const ERROR_HV_INVALID_LP_INDEX: u32 = 0xC0350041;
pub const ERROR_HV_INVALID_REGISTER_VALUE: u32 = 0xC0350050;
pub const ERROR_HV_INVALID_VTL_STATE: u32 = 0xC0350051;
pub const ERROR_HV_NX_NOT_DETECTED: u32 = 0xC0350055;
pub const ERROR_HV_INVALID_DEVICE_ID: u32 = 0xC0350057;
pub const ERROR_HV_INVALID_DEVICE_STATE: u32 = 0xC0350058;
pub const ERROR_HV_PENDING_PAGE_REQUESTS: u32 = 0x350059;
pub const ERROR_HV_PAGE_REQUEST_INVALID: u32 = 0xC0350060;
pub const ERROR_HV_INVALID_CPU_GROUP_ID: u32 = 0xC035006F;
pub const ERROR_HV_INVALID_CPU_GROUP_STATE: u32 = 0xC0350070;
pub const ERROR_HV_OPERATION_FAILED: u32 = 0xC0350071;
pub const ERROR_HV_NOT_ALLOWED_WITH_NESTED_VIRT_ACTIVE: u32 = 0xC0350072;
pub const ERROR_HV_INSUFFICIENT_ROOT_MEMORY: u32 = 0xC0350073;
pub const ERROR_HV_EVENT_BUFFER_ALREADY_FREED: u32 = 0xC0350074;
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_MEMORY: u32 = 0xC0350075;
pub const ERROR_HV_DEVICE_NOT_IN_DOMAIN: u32 = 0xC0350076;
pub const ERROR_HV_NESTED_VM_EXIT: u32 = 0xC0350077;
pub const ERROR_HV_MSR_ACCESS_FAILED: u32 = 0xC0350080;
pub const ERROR_HV_NOT_PRESENT: u32 = 0xC0351000;
pub const ERROR_VID_DUPLICATE_HANDLER: u32 = 0xC0370001;
pub const ERROR_VID_TOO_MANY_HANDLERS: u32 = 0xC0370002;
pub const ERROR_VID_QUEUE_FULL: u32 = 0xC0370003;
pub const ERROR_VID_HANDLER_NOT_PRESENT: u32 = 0xC0370004;
pub const ERROR_VID_INVALID_OBJECT_NAME: u32 = 0xC0370005;
pub const ERROR_VID_PARTITION_NAME_TOO_LONG: u32 = 0xC0370006;
pub const ERROR_VID_MESSAGE_QUEUE_NAME_TOO_LONG: u32 = 0xC0370007;
pub const ERROR_VID_PARTITION_ALREADY_EXISTS: u32 = 0xC0370008;
pub const ERROR_VID_PARTITION_DOES_NOT_EXIST: u32 = 0xC0370009;
pub const ERROR_VID_PARTITION_NAME_NOT_FOUND: u32 = 0xC037000A;
pub const ERROR_VID_MESSAGE_QUEUE_ALREADY_EXISTS: u32 = 0xC037000B;
pub const ERROR_VID_EXCEEDED_MBP_ENTRY_MAP_LIMIT: u32 = 0xC037000C;
pub const ERROR_VID_MB_STILL_REFERENCED: u32 = 0xC037000D;
pub const ERROR_VID_CHILD_GPA_PAGE_SET_CORRUPTED: u32 = 0xC037000E;
pub const ERROR_VID_INVALID_NUMA_SETTINGS: u32 = 0xC037000F;
pub const ERROR_VID_INVALID_NUMA_NODE_INDEX: u32 = 0xC0370010;
pub const ERROR_VID_NOTIFICATION_QUEUE_ALREADY_ASSOCIATED: u32 = 0xC0370011;
pub const ERROR_VID_INVALID_MEMORY_BLOCK_HANDLE: u32 = 0xC0370012;
pub const ERROR_VID_PAGE_RANGE_OVERFLOW: u32 = 0xC0370013;
pub const ERROR_VID_INVALID_MESSAGE_QUEUE_HANDLE: u32 = 0xC0370014;
pub const ERROR_VID_INVALID_GPA_RANGE_HANDLE: u32 = 0xC0370015;
pub const ERROR_VID_NO_MEMORY_BLOCK_NOTIFICATION_QUEUE: u32 = 0xC0370016;
pub const ERROR_VID_MEMORY_BLOCK_LOCK_COUNT_EXCEEDED: u32 = 0xC0370017;
pub const ERROR_VID_INVALID_PPM_HANDLE: u32 = 0xC0370018;
pub const ERROR_VID_MBPS_ARE_LOCKED: u32 = 0xC0370019;
pub const ERROR_VID_MESSAGE_QUEUE_CLOSED: u32 = 0xC037001A;
pub const ERROR_VID_VIRTUAL_PROCESSOR_LIMIT_EXCEEDED: u32 = 0xC037001B;
pub const ERROR_VID_STOP_PENDING: u32 = 0xC037001C;
pub const ERROR_VID_INVALID_PROCESSOR_STATE: u32 = 0xC037001D;
pub const ERROR_VID_EXCEEDED_KM_CONTEXT_COUNT_LIMIT: u32 = 0xC037001E;
pub const ERROR_VID_KM_INTERFACE_ALREADY_INITIALIZED: u32 = 0xC037001F;
pub const ERROR_VID_MB_PROPERTY_ALREADY_SET_RESET: u32 = 0xC0370020;
pub const ERROR_VID_MMIO_RANGE_DESTROYED: u32 = 0xC0370021;
pub const ERROR_VID_INVALID_CHILD_GPA_PAGE_SET: u32 = 0xC0370022;
pub const ERROR_VID_RESERVE_PAGE_SET_IS_BEING_USED: u32 = 0xC0370023;
pub const ERROR_VID_RESERVE_PAGE_SET_TOO_SMALL: u32 = 0xC0370024;
pub const ERROR_VID_MBP_ALREADY_LOCKED_USING_RESERVED_PAGE: u32 = 0xC0370025;
pub const ERROR_VID_MBP_COUNT_EXCEEDED_LIMIT: u32 = 0xC0370026;
pub const ERROR_VID_SAVED_STATE_CORRUPT: u32 = 0xC0370027;
pub const ERROR_VID_SAVED_STATE_UNRECOGNIZED_ITEM: u32 = 0xC0370028;
pub const ERROR_VID_SAVED_STATE_INCOMPATIBLE: u32 = 0xC0370029;
pub const ERROR_VID_VTL_ACCESS_DENIED: u32 = 0xC037002A;
pub const ERROR_VMCOMPUTE_TERMINATED_DURING_START: u32 = 0xC0370100;
pub const ERROR_VMCOMPUTE_IMAGE_MISMATCH: u32 = 0xC0370101;
pub const ERROR_VMCOMPUTE_HYPERV_NOT_INSTALLED: u32 = 0xC0370102;
pub const ERROR_VMCOMPUTE_OPERATION_PENDING: u32 = 0xC0370103;
pub const ERROR_VMCOMPUTE_TOO_MANY_NOTIFICATIONS: u32 = 0xC0370104;
pub const ERROR_VMCOMPUTE_INVALID_STATE: u32 = 0xC0370105;
pub const ERROR_VMCOMPUTE_UNEXPECTED_EXIT: u32 = 0xC0370106;
pub const ERROR_VMCOMPUTE_TERMINATED: u32 = 0xC0370107;
pub const ERROR_VMCOMPUTE_CONNECT_FAILED: u32 = 0xC0370108;
pub const ERROR_VMCOMPUTE_TIMEOUT: u32 = 0xC0370109;
pub const ERROR_VMCOMPUTE_CONNECTION_CLOSED: u32 = 0xC037010A;
pub const ERROR_VMCOMPUTE_UNKNOWN_MESSAGE: u32 = 0xC037010B;
pub const ERROR_VMCOMPUTE_UNSUPPORTED_PROTOCOL_VERSION: u32 = 0xC037010C;
pub const ERROR_VMCOMPUTE_INVALID_JSON: u32 = 0xC037010D;
pub const ERROR_VMCOMPUTE_SYSTEM_NOT_FOUND: u32 = 0xC037010E;
pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_EXISTS: u32 = 0xC037010F;
pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_STOPPED: u32 = 0xC0370110;
pub const ERROR_VMCOMPUTE_PROTOCOL_ERROR: u32 = 0xC0370111;
pub const ERROR_VMCOMPUTE_INVALID_LAYER: u32 = 0xC0370112;
pub const ERROR_VMCOMPUTE_WINDOWS_INSIDER_REQUIRED: u32 = 0xC0370113;
pub const ERROR_VNET_VIRTUAL_SWITCH_NAME_NOT_FOUND: u32 = 0xC0370200;
pub const ERROR_VID_REMOTE_NODE_PARENT_GPA_PAGES_USED: u32 = 0x80370001;
pub const ERROR_VSMB_SAVED_STATE_FILE_NOT_FOUND: u32 = 0xC0370400;
pub const ERROR_VSMB_SAVED_STATE_CORRUPT: u32 = 0xC0370401;
pub const ERROR_VOLMGR_INCOMPLETE_REGENERATION: u32 = 0x80380001;
pub const ERROR_VOLMGR_INCOMPLETE_DISK_MIGRATION: u32 = 0x80380002;
pub const ERROR_VOLMGR_DATABASE_FULL: u32 = 0xC0380001;
pub const ERROR_VOLMGR_DISK_CONFIGURATION_CORRUPTED: u32 = 0xC0380002;
pub const ERROR_VOLMGR_DISK_CONFIGURATION_NOT_IN_SYNC: u32 = 0xC0380003;
pub const ERROR_VOLMGR_PACK_CONFIG_UPDATE_FAILED: u32 = 0xC0380004;
pub const ERROR_VOLMGR_DISK_CONTAINS_NON_SIMPLE_VOLUME: u32 = 0xC0380005;
pub const ERROR_VOLMGR_DISK_DUPLICATE: u32 = 0xC0380006;
pub const ERROR_VOLMGR_DISK_DYNAMIC: u32 = 0xC0380007;
pub const ERROR_VOLMGR_DISK_ID_INVALID: u32 = 0xC0380008;
pub const ERROR_VOLMGR_DISK_INVALID: u32 = 0xC0380009;
pub const ERROR_VOLMGR_DISK_LAST_VOTER: u32 = 0xC038000A;
pub const ERROR_VOLMGR_DISK_LAYOUT_INVALID: u32 = 0xC038000B;
pub const ERROR_VOLMGR_DISK_LAYOUT_NON_BASIC_BETWEEN_BASIC_PARTITIONS: u32 = 0xC038000C;
pub const ERROR_VOLMGR_DISK_LAYOUT_NOT_CYLINDER_ALIGNED: u32 = 0xC038000D;
pub const ERROR_VOLMGR_DISK_LAYOUT_PARTITIONS_TOO_SMALL: u32 = 0xC038000E;
pub const ERROR_VOLMGR_DISK_LAYOUT_PRIMARY_BETWEEN_LOGICAL_PARTITIONS: u32 = 0xC038000F;
pub const ERROR_VOLMGR_DISK_LAYOUT_TOO_MANY_PARTITIONS: u32 = 0xC0380010;
pub const ERROR_VOLMGR_DISK_MISSING: u32 = 0xC0380011;
pub const ERROR_VOLMGR_DISK_NOT_EMPTY: u32 = 0xC0380012;
pub const ERROR_VOLMGR_DISK_NOT_ENOUGH_SPACE: u32 = 0xC0380013;
pub const ERROR_VOLMGR_DISK_REVECTORING_FAILED: u32 = 0xC0380014;
pub const ERROR_VOLMGR_DISK_SECTOR_SIZE_INVALID: u32 = 0xC0380015;
pub const ERROR_VOLMGR_DISK_SET_NOT_CONTAINED: u32 = 0xC0380016;
pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_MEMBERS: u32 = 0xC0380017;
pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_PLEXES: u32 = 0xC0380018;
pub const ERROR_VOLMGR_DYNAMIC_DISK_NOT_SUPPORTED: u32 = 0xC0380019;
pub const ERROR_VOLMGR_EXTENT_ALREADY_USED: u32 = 0xC038001A;
pub const ERROR_VOLMGR_EXTENT_NOT_CONTIGUOUS: u32 = 0xC038001B;
pub const ERROR_VOLMGR_EXTENT_NOT_IN_PUBLIC_REGION: u32 = 0xC038001C;
pub const ERROR_VOLMGR_EXTENT_NOT_SECTOR_ALIGNED: u32 = 0xC038001D;
pub const ERROR_VOLMGR_EXTENT_OVERLAPS_EBR_PARTITION: u32 = 0xC038001E;
pub const ERROR_VOLMGR_EXTENT_VOLUME_LENGTHS_DO_NOT_MATCH: u32 = 0xC038001F;
pub const ERROR_VOLMGR_FAULT_TOLERANT_NOT_SUPPORTED: u32 = 0xC0380020;
pub const ERROR_VOLMGR_INTERLEAVE_LENGTH_INVALID: u32 = 0xC0380021;
pub const ERROR_VOLMGR_MAXIMUM_REGISTERED_USERS: u32 = 0xC0380022;
pub const ERROR_VOLMGR_MEMBER_IN_SYNC: u32 = 0xC0380023;
pub const ERROR_VOLMGR_MEMBER_INDEX_DUPLICATE: u32 = 0xC0380024;
pub const ERROR_VOLMGR_MEMBER_INDEX_INVALID: u32 = 0xC0380025;
pub const ERROR_VOLMGR_MEMBER_MISSING: u32 = 0xC0380026;
pub const ERROR_VOLMGR_MEMBER_NOT_DETACHED: u32 = 0xC0380027;
pub const ERROR_VOLMGR_MEMBER_REGENERATING: u32 = 0xC0380028;
pub const ERROR_VOLMGR_ALL_DISKS_FAILED: u32 = 0xC0380029;
pub const ERROR_VOLMGR_NO_REGISTERED_USERS: u32 = 0xC038002A;
pub const ERROR_VOLMGR_NO_SUCH_USER: u32 = 0xC038002B;
pub const ERROR_VOLMGR_NOTIFICATION_RESET: u32 = 0xC038002C;
pub const ERROR_VOLMGR_NUMBER_OF_MEMBERS_INVALID: u32 = 0xC038002D;
pub const ERROR_VOLMGR_NUMBER_OF_PLEXES_INVALID: u32 = 0xC038002E;
pub const ERROR_VOLMGR_PACK_DUPLICATE: u32 = 0xC038002F;
pub const ERROR_VOLMGR_PACK_ID_INVALID: u32 = 0xC0380030;
pub const ERROR_VOLMGR_PACK_INVALID: u32 = 0xC0380031;
pub const ERROR_VOLMGR_PACK_NAME_INVALID: u32 = 0xC0380032;
pub const ERROR_VOLMGR_PACK_OFFLINE: u32 = 0xC0380033;
pub const ERROR_VOLMGR_PACK_HAS_QUORUM: u32 = 0xC0380034;
pub const ERROR_VOLMGR_PACK_WITHOUT_QUORUM: u32 = 0xC0380035;
pub const ERROR_VOLMGR_PARTITION_STYLE_INVALID: u32 = 0xC0380036;
pub const ERROR_VOLMGR_PARTITION_UPDATE_FAILED: u32 = 0xC0380037;
pub const ERROR_VOLMGR_PLEX_IN_SYNC: u32 = 0xC0380038;
pub const ERROR_VOLMGR_PLEX_INDEX_DUPLICATE: u32 = 0xC0380039;
pub const ERROR_VOLMGR_PLEX_INDEX_INVALID: u32 = 0xC038003A;
pub const ERROR_VOLMGR_PLEX_LAST_ACTIVE: u32 = 0xC038003B;
pub const ERROR_VOLMGR_PLEX_MISSING: u32 = 0xC038003C;
pub const ERROR_VOLMGR_PLEX_REGENERATING: u32 = 0xC038003D;
pub const ERROR_VOLMGR_PLEX_TYPE_INVALID: u32 = 0xC038003E;
pub const ERROR_VOLMGR_PLEX_NOT_RAID5: u32 = 0xC038003F;
pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE: u32 = 0xC0380040;
pub const ERROR_VOLMGR_STRUCTURE_SIZE_INVALID: u32 = 0xC0380041;
pub const ERROR_VOLMGR_TOO_MANY_NOTIFICATION_REQUESTS: u32 = 0xC0380042;
pub const ERROR_VOLMGR_TRANSACTION_IN_PROGRESS: u32 = 0xC0380043;
pub const ERROR_VOLMGR_UNEXPECTED_DISK_LAYOUT_CHANGE: u32 = 0xC0380044;
pub const ERROR_VOLMGR_VOLUME_CONTAINS_MISSING_DISK: u32 = 0xC0380045;
pub const ERROR_VOLMGR_VOLUME_ID_INVALID: u32 = 0xC0380046;
pub const ERROR_VOLMGR_VOLUME_LENGTH_INVALID: u32 = 0xC0380047;
pub const ERROR_VOLMGR_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: u32 = 0xC0380048;
pub const ERROR_VOLMGR_VOLUME_NOT_MIRRORED: u32 = 0xC0380049;
pub const ERROR_VOLMGR_VOLUME_NOT_RETAINED: u32 = 0xC038004A;
pub const ERROR_VOLMGR_VOLUME_OFFLINE: u32 = 0xC038004B;
pub const ERROR_VOLMGR_VOLUME_RETAINED: u32 = 0xC038004C;
pub const ERROR_VOLMGR_NUMBER_OF_EXTENTS_INVALID: u32 = 0xC038004D;
pub const ERROR_VOLMGR_DIFFERENT_SECTOR_SIZE: u32 = 0xC038004E;
pub const ERROR_VOLMGR_BAD_BOOT_DISK: u32 = 0xC038004F;
pub const ERROR_VOLMGR_PACK_CONFIG_OFFLINE: u32 = 0xC0380050;
pub const ERROR_VOLMGR_PACK_CONFIG_ONLINE: u32 = 0xC0380051;
pub const ERROR_VOLMGR_NOT_PRIMARY_PACK: u32 = 0xC0380052;
pub const ERROR_VOLMGR_PACK_LOG_UPDATE_FAILED: u32 = 0xC0380053;
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_PLEX_INVALID: u32 = 0xC0380054;
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_MEMBER_INVALID: u32 = 0xC0380055;
pub const ERROR_VOLMGR_VOLUME_MIRRORED: u32 = 0xC0380056;
pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE_SPANNED: u32 = 0xC0380057;
pub const ERROR_VOLMGR_NO_VALID_LOG_COPIES: u32 = 0xC0380058;
pub const ERROR_VOLMGR_PRIMARY_PACK_PRESENT: u32 = 0xC0380059;
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_INVALID: u32 = 0xC038005A;
pub const ERROR_VOLMGR_MIRROR_NOT_SUPPORTED: u32 = 0xC038005B;
pub const ERROR_VOLMGR_RAID5_NOT_SUPPORTED: u32 = 0xC038005C;
pub const ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED: u32 = 0x80390001;
pub const ERROR_BCD_TOO_MANY_ELEMENTS: u32 = 0xC0390002;
pub const ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED: u32 = 0x80390003;
pub const ERROR_VHD_DRIVE_FOOTER_MISSING: u32 = 0xC03A0001;
pub const ERROR_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH: u32 = 0xC03A0002;
pub const ERROR_VHD_DRIVE_FOOTER_CORRUPT: u32 = 0xC03A0003;
pub const ERROR_VHD_FORMAT_UNKNOWN: u32 = 0xC03A0004;
pub const ERROR_VHD_FORMAT_UNSUPPORTED_VERSION: u32 = 0xC03A0005;
pub const ERROR_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH: u32 = 0xC03A0006;
pub const ERROR_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION: u32 = 0xC03A0007;
pub const ERROR_VHD_SPARSE_HEADER_CORRUPT: u32 = 0xC03A0008;
pub const ERROR_VHD_BLOCK_ALLOCATION_FAILURE: u32 = 0xC03A0009;
pub const ERROR_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT: u32 = 0xC03A000A;
pub const ERROR_VHD_INVALID_BLOCK_SIZE: u32 = 0xC03A000B;
pub const ERROR_VHD_BITMAP_MISMATCH: u32 = 0xC03A000C;
pub const ERROR_VHD_PARENT_VHD_NOT_FOUND: u32 = 0xC03A000D;
pub const ERROR_VHD_CHILD_PARENT_ID_MISMATCH: u32 = 0xC03A000E;
pub const ERROR_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH: u32 = 0xC03A000F;
pub const ERROR_VHD_METADATA_READ_FAILURE: u32 = 0xC03A0010;
pub const ERROR_VHD_METADATA_WRITE_FAILURE: u32 = 0xC03A0011;
pub const ERROR_VHD_INVALID_SIZE: u32 = 0xC03A0012;
pub const ERROR_VHD_INVALID_FILE_SIZE: u32 = 0xC03A0013;
pub const ERROR_VIRTDISK_PROVIDER_NOT_FOUND: u32 = 0xC03A0014;
pub const ERROR_VIRTDISK_NOT_VIRTUAL_DISK: u32 = 0xC03A0015;
pub const ERROR_VHD_PARENT_VHD_ACCESS_DENIED: u32 = 0xC03A0016;
pub const ERROR_VHD_CHILD_PARENT_SIZE_MISMATCH: u32 = 0xC03A0017;
pub const ERROR_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED: u32 = 0xC03A0018;
pub const ERROR_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT: u32 = 0xC03A0019;
pub const ERROR_VIRTUAL_DISK_LIMITATION: u32 = 0xC03A001A;
pub const ERROR_VHD_INVALID_TYPE: u32 = 0xC03A001B;
pub const ERROR_VHD_INVALID_STATE: u32 = 0xC03A001C;
pub const ERROR_VIRTDISK_UNSUPPORTED_DISK_SECTOR_SIZE: u32 = 0xC03A001D;
pub const ERROR_VIRTDISK_DISK_ALREADY_OWNED: u32 = 0xC03A001E;
pub const ERROR_VIRTDISK_DISK_ONLINE_AND_WRITABLE: u32 = 0xC03A001F;
pub const ERROR_CTLOG_TRACKING_NOT_INITIALIZED: u32 = 0xC03A0020;
pub const ERROR_CTLOG_LOGFILE_SIZE_EXCEEDED_MAXSIZE: u32 = 0xC03A0021;
pub const ERROR_CTLOG_VHD_CHANGED_OFFLINE: u32 = 0xC03A0022;
pub const ERROR_CTLOG_INVALID_TRACKING_STATE: u32 = 0xC03A0023;
pub const ERROR_CTLOG_INCONSISTENT_TRACKING_FILE: u32 = 0xC03A0024;
pub const ERROR_VHD_RESIZE_WOULD_TRUNCATE_DATA: u32 = 0xC03A0025;
pub const ERROR_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE: u32 = 0xC03A0026;
pub const ERROR_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE: u32 = 0xC03A0027;
pub const ERROR_VHD_METADATA_FULL: u32 = 0xC03A0028;
pub const ERROR_VHD_INVALID_CHANGE_TRACKING_ID: u32 = 0xC03A0029;
pub const ERROR_VHD_CHANGE_TRACKING_DISABLED: u32 = 0xC03A002A;
pub const ERROR_VHD_MISSING_CHANGE_TRACKING_INFORMATION: u32 = 0xC03A0030;
pub const ERROR_QUERY_STORAGE_ERROR: u32 = 0x803A0001;
pub struct Windowinfo {
    pub cbSize: u32,
    pub rcWindow: RECT,
    pub rcClient: RECT,
    pub dwStyle: u32,
    pub dwExStyle: u32,
    pub dwWindowStatus: u32,
    pub cxWindowBorders: u32,
    pub cyWindowBorders: u32,
    pub atomWindowType: u16,
    pub wCreatorVersion: u16,
}
pub struct Windowplacement {
    pub length: u32,
    pub flags: u32,
    pub showCmd: u32,
    pub ptMinPosition: POINT,
    pub ptMaxPosition: POINT,
    pub rcNormalPosition: RECT,
}
pub const WPF_ASYNCWINDOWPLACEMENT: u32 = 0x4;
pub const WPF_RESTORETOMAXIMIZED: u32 = 0x2;
pub const WPF_SETMINPOSITION: u32 = 0x1;
pub struct Windowpos {
    pub hwnd: HWND,
    pub hwndInsertAfter: HWND,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub flags: u32,
}
pub const WH_CALLWNDPROC: i32 = 0x4;
pub const WH_CALLWNDPROCRET: i32 = 0xC;
pub const WH_CBT: i32 = 0x5;
pub const WH_DEBUG: i32 = 0x9;
pub const WH_FOREGROUNDIDLE: i32 = 0xB;
pub const WH_GETMESSAGE: i32 = 0x3;
pub const WH_JOURNALPLAYBACK: i32 = 0x1;
pub const WH_JOURNALRECORD: i32 = 0x0;
pub const WH_KEYBOARD: i32 = 0x2;
pub const WH_KEYBOARD_LL: i32 = 0xD;
pub const WH_MOUSE: i32 = 0x7;
pub const WH_MOUSE_LL: i32 = 0xE;
pub const WH_MSGFILTER: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const WH_SHELL: i32 = 0xA;
pub const WH_SYSMSGFILTER: i32 = 0x6;
pub const WTA_NONCLIENT: i32 = 0x1;
pub const WDA_NONE: u32 = 0x0;
pub const WDA_MONITOR: u32 = 0x1;
pub const WDA_EXCLUDEFROMCAPTURE: u32 = 0x11;
pub const WS_EX_DLGMODALFRAME: u32 = 0x1;
pub const WS_EX_NOPARENTNOTIFY: u32 = 0x4;
pub const WS_EX_TOPMOST: u32 = 0x8;
pub const WS_EX_ACCEPTFILES: u32 = 0x10;
pub const WS_EX_TRANSPARENT: u32 = 0x20;
pub const WS_EX_MDICHILD: u32 = 0x40;
pub const WS_EX_TOOLWINDOW: u32 = 0x80;
pub const WS_EX_WINDOWEDGE: u32 = 0x100;
pub const WS_EX_CLIENTEDGE: u32 = 0x200;
pub const WS_EX_CONTEXTHELP: u32 = 0x400;
pub const WS_EX_RIGHT: u32 = 0x1000;
pub const WS_EX_LEFT: u32 = 0x0;
pub const WS_EX_RTLREADING: u32 = 0x2000;
pub const WS_EX_LTRREADING: u32 = 0x0;
pub const WS_EX_LEFTSCROLLBAR: u32 = 0x4000;
pub const WS_EX_RIGHTSCROLLBAR: u32 = 0x0;
pub const WS_EX_CONTROLPARENT: u32 = 0x10000;
pub const WS_EX_STATICEDGE: u32 = 0x20000;
pub const WS_EX_APPWINDOW: u32 = 0x40000;
pub const WS_EX_OVERLAPPEDWINDOW: u32 = 0x300;
pub const WS_EX_PALETTEWINDOW: u32 = 0x188;
pub const WS_EX_LAYERED: u32 = 0x80000;
pub const WS_EX_NOINHERITLAYOUT: u32 = 0x100000;
pub const WS_EX_NOREDIRECTIONBITMAP: u32 = 0x200000;
pub const WS_EX_LAYOUTRTL: u32 = 0x400000;
pub const WS_EX_COMPOSITED: u32 = 0x2000000;
pub const WS_EX_NOACTIVATE: u32 = 0x8000000;
pub const GWL_EXSTYLE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC;
pub const GWLP_HINSTANCE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFA;
pub const GWLP_HWNDPARENT: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8;
pub const GWLP_ID: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4;
pub const GWL_STYLE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0;
pub const GWLP_USERDATA: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEB;
pub const GWLP_WNDPROC: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC;
pub const GWL_HINSTANCE: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFA;
pub const GWL_ID: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4;
pub const GWL_USERDATA: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEB;
pub const GWL_WNDPROC: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC;
pub const GWL_HWNDPARENT: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8;
pub const MSGFLT_ALLOW: u32 = 0x1;
pub const MSGFLT_DISALLOW: u32 = 0x2;
pub const MSGFLT_RESET: u32 = 0x0;
pub const WS_OVERLAPPED: u32 = 0x0;
pub const WS_POPUP: u32 = 0x80000000;
pub const WS_CHILD: u32 = 0x40000000;
pub const WS_MINIMIZE: u32 = 0x20000000;
pub const WS_VISIBLE: u32 = 0x10000000;
pub const WS_DISABLED: u32 = 0x8000000;
pub const WS_CLIPSIBLINGS: u32 = 0x4000000;
pub const WS_CLIPCHILDREN: u32 = 0x2000000;
pub const WS_MAXIMIZE: u32 = 0x1000000;
pub const WS_CAPTION: u32 = 0xC00000;
pub const WS_BORDER: u32 = 0x800000;
pub const WS_DLGFRAME: u32 = 0x400000;
pub const WS_VSCROLL: u32 = 0x200000;
pub const WS_HSCROLL: u32 = 0x100000;
pub const WS_SYSMENU: u32 = 0x80000;
pub const WS_THICKFRAME: u32 = 0x40000;
pub const WS_GROUP: u32 = 0x20000;
pub const WS_TABSTOP: u32 = 0x10000;
pub const WS_MINIMIZEBOX: u32 = 0x20000;
pub const WS_MAXIMIZEBOX: u32 = 0x10000;
pub const WS_TILED: u32 = 0x0;
pub const WS_ICONIC: u32 = 0x20000000;
pub const WS_SIZEBOX: u32 = 0x40000;
pub const WS_TILEDWINDOW: u32 = 0xCF0000;
pub const WS_OVERLAPPEDWINDOW: u32 = 0xCF0000;
pub const WS_POPUPWINDOW: u32 = 0x80880000;
pub const WS_CHILDWINDOW: u32 = 0x40000000;
pub const WS_ACTIVECAPTION: u32 = 0x1;
pub struct Wndclassa<'a> {
    pub style: u32,
    pub lpfnWndProc: todo_fn,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: &'a Cow<'a, str>,
    pub lpszClassName: &'a Cow<'a, str>,
}
pub struct Wndclassexa<'a> {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: todo_fn,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: &'a Cow<'a, str>,
    pub lpszClassName: &'a Cow<'a, str>,
    pub hIconSm: HICON,
}
pub struct Wndclassexw<'a> {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: todo_fn,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: &'a Cow<'a, OsStr>,
    pub lpszClassName: &'a Cow<'a, OsStr>,
    pub hIconSm: HICON,
}
pub struct Wndclassw<'a> {
    pub style: u32,
    pub lpfnWndProc: todo_fn,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: &'a Cow<'a, OsStr>,
    pub lpszClassName: &'a Cow<'a, OsStr>,
}
pub const CS_VREDRAW: u32 = 0x1;
pub const CS_HREDRAW: u32 = 0x2;
pub const CS_DBLCLKS: u32 = 0x8;
pub const CS_OWNDC: u32 = 0x20;
pub const CS_CLASSDC: u32 = 0x40;
pub const CS_PARENTDC: u32 = 0x80;
pub const CS_NOCLOSE: u32 = 0x200;
pub const CS_SAVEBITS: u32 = 0x800;
pub const CS_BYTEALIGNCLIENT: u32 = 0x1000;
pub const CS_BYTEALIGNWINDOW: u32 = 0x2000;
pub const CS_GLOBALCLASS: u32 = 0x4000;
pub const CS_IME: u32 = 0x10000;
pub const CS_DROPSHADOW: u32 = 0x20000;
pub const WB_CLASSIFY: u32 = 0x3;
pub const WB_ISDELIMITER: u32 = 0x2;
pub const WB_LEFT: u32 = 0x0;
pub const WB_LEFTBREAK: u32 = 0x6;
pub const WB_MOVEWORDLEFT: u32 = 0x4;
pub const WB_MOVEWORDRIGHT: u32 = 0x5;
pub const WB_RIGHT: u32 = 0x1;
pub const WB_RIGHTBREAK: u32 = 0x7;
pub const WSB_PROP_CXHSCROLL: i32 = 0x2;
pub const WSB_PROP_CXHTHUMB: i32 = 0x10;
pub const WSB_PROP_CXVSCROLL: i32 = 0x8;
pub const WSB_PROP_CYHSCROLL: i32 = 0x4;
pub const WSB_PROP_CYVSCROLL: i32 = 0x1;
pub const WSB_PROP_CYVTHUMB: i32 = 0x20;
pub const WSB_PROP_HBKGCOLOR: i32 = 0x80;
pub const WSB_PROP_HSTYLE: i32 = 0x200;
pub const WSB_PROP_PALETTE: i32 = 0x800;
pub const WSB_PROP_VBKGCOLOR: i32 = 0x40;
pub const WSB_PROP_VSTYLE: i32 = 0x100;
pub const WSB_PROP_WINSTYLE: i32 = 0x400;
pub struct WtaOptions {
    pub dwFlags: u32,
    pub dwMask: u32,
}
pub const WTSAT_UNKNOWN: i32 = 0x0;
pub const WTSAT_RGB: i32 = 0x1;
pub const WTSAT_ARGB: i32 = 0x2;
pub const WTS_DEFAULT: i32 = 0x0;
pub const WTS_LOWQUALITY: i32 = 0x1;
pub const WTS_CACHED: i32 = 0x2;
pub const WTSCF_DEFAULT: i32 = 0x0;
pub const WTSCF_APPSTYLE: i32 = 0x1;
pub const WTSCF_SQUARE: i32 = 0x2;
pub const WTSCF_WIDE: i32 = 0x4;
pub const WTSCF_FAST: i32 = 0x8;
pub const WTS_NONE: i32 = 0x0;
pub const WTS_EXTRACT: i32 = 0x0;
pub const WTS_INCACHEONLY: i32 = 0x1;
pub const WTS_FASTEXTRACT: i32 = 0x2;
pub const WTS_FORCEEXTRACTION: i32 = 0x4;
pub const WTS_SLOWRECLAIM: i32 = 0x8;
pub const WTS_EXTRACTDONOTCACHE: i32 = 0x20;
pub const WTS_SCALETOREQUESTEDSIZE: i32 = 0x40;
pub const WTS_SKIPFASTEXTRACT: i32 = 0x80;
pub const WTS_EXTRACTINPROC: i32 = 0x100;
pub const WTS_CROPTOSQUARE: i32 = 0x200;
pub const WTS_INSTANCESURROGATE: i32 = 0x400;
pub const WTS_REQUIRESURROGATE: i32 = 0x800;
pub const WTS_APPSTYLE: i32 = 0x2000;
pub const WTS_WIDETHUMBNAILS: i32 = 0x4000;
pub const WTS_IDEALCACHESIZEONLY: i32 = 0x8000;
pub const WTS_SCALEUP: i32 = 0x10000;
pub struct WtsThumbnailid {
    pub rgbKey: [u8; 16],
}
pub struct WebBrowser {
}
pub struct WebBrowserV1 {
}
pub struct WebWizardHost {
}
pub struct WinBioCredentialProvider {
}
pub struct Xform {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
pub struct AppconstrainRegistration {
}
pub struct AppstateRegistration {
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Anonymous2EStruct1 {
    pub field0: u32,
    pub field1: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union Anonymous2EStruct3 {
    pub field0: u32,
    pub field1: u32,
}
pub const CDBE_TYPE_MUSIC: i32 = 0x1;
pub const CDBE_TYPE_DATA: i32 = 0x2;
pub const CDBE_TYPE_ALL: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
pub const ECF_DEFAULT: i32 = 0x0;
pub const ECF_HASSUBCOMMANDS: i32 = 0x1;
pub const ECF_HASSPLITBUTTON: i32 = 0x2;
pub const ECF_HIDELABEL: i32 = 0x4;
pub const ECF_ISSEPARATOR: i32 = 0x8;
pub const ECF_HASLUASHIELD: i32 = 0x10;
pub const ECF_SEPARATORBEFORE: i32 = 0x20;
pub const ECF_SEPARATORAFTER: i32 = 0x40;
pub const ECF_ISDROPDOWN: i32 = 0x80;
pub const ECF_TOGGLEABLE: i32 = 0x100;
pub const ECF_AUTOMENUICONS: i32 = 0x200;
pub const ECS_ENABLED: i32 = 0x0;
pub const ECS_DISABLED: i32 = 0x1;
pub const ECS_HIDDEN: i32 = 0x2;
pub const ECS_CHECKBOX: i32 = 0x4;
pub const ECS_CHECKED: i32 = 0x8;
pub const ECS_RADIOCHECK: i32 = 0x10;
pub const EPS_DONTCARE: i32 = 0x0;
pub const EPS_DEFAULT_ON: i32 = 0x1;
pub const EPS_DEFAULT_OFF: i32 = 0x2;
pub const EPS_STATEMASK: i32 = 0xFFFF;
pub const EPS_INITIALSTATE: i32 = 0x10000;
pub const EPS_FORCE: i32 = 0x20000;
pub const EXPPS_FILETYPES: i32 = 0x1;
pub const HLSHORTCUTF_DEFAULT: i32 = 0x0;
pub const HLSHORTCUTF_DONTACTUALLYCREATE: i32 = 0x1;
pub const HLSHORTCUTF_USEFILENAMEFROMFRIENDLYNAME: i32 = 0x2;
pub const HLSHORTCUTF_USEUNIQUEFILENAME: i32 = 0x4;
pub const HLSHORTCUTF_MAYUSEEXISTINGSHORTCUT: i32 = 0x8;
pub const HLSR_HOME: i32 = 0x0;
pub const HLSR_SEARCHPAGE: i32 = 0x1;
pub const HLSR_HISTORYFOLDER: i32 = 0x2;
pub const HLTRANSLATEF_DEFAULT: i32 = 0x0;
pub const HLTRANSLATEF_DONTAPPLYDEFAULTPREFIX: i32 = 0x1;
pub const KFDF_LOCAL_REDIRECT_ONLY: i32 = 0x2;
pub const KFDF_ROAMABLE: i32 = 0x4;
pub const KFDF_PRECREATE: i32 = 0x8;
pub const KFDF_STREAM: i32 = 0x10;
pub const KFDF_PUBLISHEXPANDEDPATH: i32 = 0x20;
pub const KFDF_NO_REDIRECT_UI: i32 = 0x40;
pub const KF_REDIRECTION_CAPABILITIES_ALLOW_ALL: i32 = 0xFF;
pub const KF_REDIRECTION_CAPABILITIES_REDIRECTABLE: i32 = 0x1;
pub const KF_REDIRECTION_CAPABILITIES_DENY_ALL: i32 = 0xFFF00;
pub const KF_REDIRECTION_CAPABILITIES_DENY_POLICY_REDIRECTED: i32 = 0x100;
pub const KF_REDIRECTION_CAPABILITIES_DENY_POLICY: i32 = 0x200;
pub const KF_REDIRECTION_CAPABILITIES_DENY_PERMISSIONS: i32 = 0x400;
pub const KF_REDIRECT_USER_EXCLUSIVE: i32 = 0x1;
pub const KF_REDIRECT_COPY_SOURCE_DACL: i32 = 0x2;
pub const KF_REDIRECT_OWNER_USER: i32 = 0x4;
pub const KF_REDIRECT_SET_OWNER_EXPLICIT: i32 = 0x8;
pub const KF_REDIRECT_CHECK_ONLY: i32 = 0x10;
pub const KF_REDIRECT_WITH_UI: i32 = 0x20;
pub const KF_REDIRECT_UNPIN: i32 = 0x40;
pub const KF_REDIRECT_PIN: i32 = 0x80;
pub const KF_REDIRECT_COPY_CONTENTS: i32 = 0x200;
pub const KF_REDIRECT_DEL_SOURCE_CONTENTS: i32 = 0x400;
pub const KF_REDIRECT_EXCLUDE_ALL_KNOWN_SUBFOLDERS: i32 = 0x800;
pub const LIM_SMALL: i32 = 0x0;
pub const LIM_LARGE: i32 = 0x1;
pub const NMCII_NONE: i32 = 0x0;
pub const NMCII_ITEMS: i32 = 0x1;
pub const NMCII_FOLDERS: i32 = 0x2;
pub const NMCSAEI_SELECT: i32 = 0x0;
pub const NMCSAEI_EDIT: i32 = 0x1;
pub const NSTCECT_LBUTTON: i32 = 0x1;
pub const NSTCECT_MBUTTON: i32 = 0x2;
pub const NSTCECT_RBUTTON: i32 = 0x3;
pub const NSTCECT_BUTTON: i32 = 0x3;
pub const NSTCECT_DBLCLICK: i32 = 0x4;
pub const NSTCEHT_NOWHERE: i32 = 0x1;
pub const NSTCEHT_ONITEMICON: i32 = 0x2;
pub const NSTCEHT_ONITEMLABEL: i32 = 0x4;
pub const NSTCEHT_ONITEMINDENT: i32 = 0x8;
pub const NSTCEHT_ONITEMBUTTON: i32 = 0x10;
pub const NSTCEHT_ONITEMRIGHT: i32 = 0x20;
pub const NSTCEHT_ONITEMSTATEICON: i32 = 0x40;
pub const NSTCEHT_ONITEM: i32 = 0x46;
pub const NSTCEHT_ONITEMTABBUTTON: i32 = 0x1000;
pub const NSTCIS_NONE: i32 = 0x0;
pub const NSTCIS_SELECTED: i32 = 0x1;
pub const NSTCIS_EXPANDED: i32 = 0x2;
pub const NSTCIS_BOLD: i32 = 0x4;
pub const NSTCIS_DISABLED: i32 = 0x8;
pub const NSTCIS_SELECTEDNOEXPAND: i32 = 0x10;
pub const NSTCRS_VISIBLE: i32 = 0x0;
pub const NSTCRS_HIDDEN: i32 = 0x1;
pub const NSTCRS_EXPANDED: i32 = 0x2;
pub const NSTCS_HASEXPANDOS: i32 = 0x1;
pub const NSTCS_HASLINES: i32 = 0x2;
pub const NSTCS_SINGLECLICKEXPAND: i32 = 0x4;
pub const NSTCS_FULLROWSELECT: i32 = 0x8;
pub const NSTCS_SPRINGEXPAND: i32 = 0x10;
pub const NSTCS_HORIZONTALSCROLL: i32 = 0x20;
pub const NSTCS_ROOTHASEXPANDO: i32 = 0x40;
pub const NSTCS_SHOWSELECTIONALWAYS: i32 = 0x80;
pub const NSTCS_NOINFOTIP: i32 = 0x200;
pub const NSTCS_EVENHEIGHT: i32 = 0x400;
pub const NSTCS_NOREPLACEOPEN: i32 = 0x800;
pub const NSTCS_DISABLEDRAGDROP: i32 = 0x1000;
pub const NSTCS_NOORDERSTREAM: i32 = 0x2000;
pub const NSTCS_RICHTOOLTIP: i32 = 0x4000;
pub const NSTCS_BORDER: i32 = 0x8000;
pub const NSTCS_NOEDITLABELS: i32 = 0x10000;
pub const NSTCS_TABSTOP: i32 = 0x20000;
pub const NSTCS_FAVORITESMODE: i32 = 0x80000;
pub const NSTCS_AUTOHSCROLL: i32 = 0x100000;
pub const NSTCS_FADEINOUTEXPANDOS: i32 = 0x200000;
pub const NSTCS_EMPTYTEXT: i32 = 0x400000;
pub const NSTCS_CHECKBOXES: i32 = 0x800000;
pub const NSTCS_PARTIALCHECKBOXES: i32 = 0x1000000;
pub const NSTCS_EXCLUSIONCHECKBOXES: i32 = 0x2000000;
pub const NSTCS_DIMMEDCHECKBOXES: i32 = 0x4000000;
pub const NSTCS_NOINDENTCHECKS: i32 = 0x8000000;
pub const NSTCS_ALLOWJUNCTIONS: i32 = 0x10000000;
pub const NSTCS_SHOWTABSBUTTON: i32 = 0x20000000;
pub const NSTCS_SHOWDELETEBUTTON: i32 = 0x40000000;
pub const NSTCS_SHOWREFRESHBUTTON: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000;
pub const OPPROGDLG_DEFAULT: i32 = 0x0;
pub const OPPROGDLG_ENABLEPAUSE: i32 = 0x80;
pub const OPPROGDLG_ALLOWUNDO: i32 = 0x100;
pub const OPPROGDLG_DONTDISPLAYSOURCEPATH: i32 = 0x200;
pub const OPPROGDLG_DONTDISPLAYDESTPATH: i32 = 0x400;
pub const OPPROGDLG_NOMULTIDAYESTIMATES: i32 = 0x800;
pub const OPPROGDLG_DONTDISPLAYLOCATIONS: i32 = 0x1000;
pub const PDM_DEFAULT: i32 = 0x0;
pub const PDM_RUN: i32 = 0x1;
pub const PDM_PREFLIGHT: i32 = 0x2;
pub const PDM_UNDOING: i32 = 0x4;
pub const PDM_ERRORSBLOCKING: i32 = 0x8;
pub const PDM_INDETERMINATE: i32 = 0x10;
pub const SHCONTF_CHECKING_FOR_CHILDREN: i32 = 0x10;
pub const SHCONTF_FOLDERS: i32 = 0x20;
pub const SHCONTF_NONFOLDERS: i32 = 0x40;
pub const SHCONTF_INCLUDEHIDDEN: i32 = 0x80;
pub const SHCONTF_INIT_ON_FIRST_NEXT: i32 = 0x100;
pub const SHCONTF_NETPRINTERSRCH: i32 = 0x200;
pub const SHCONTF_SHAREABLE: i32 = 0x400;
pub const SHCONTF_STORAGE: i32 = 0x800;
pub const SHCONTF_NAVIGATION_ENUM: i32 = 0x1000;
pub const SHCONTF_FASTITEMS: i32 = 0x2000;
pub const SHCONTF_FLATLIST: i32 = 0x4000;
pub const SHCONTF_ENABLE_ASYNC: i32 = 0x8000;
pub const SHCONTF_INCLUDESUPERHIDDEN: i32 = 0x10000;
pub const SHGDN_NORMAL: i32 = 0x0;
pub const SHGDN_INFOLDER: i32 = 0x1;
pub const SHGDN_FOREDITING: i32 = 0x1000;
pub const SHGDN_FORADDRESSBAR: i32 = 0x4000;
pub const SHGDN_FORPARSING: i32 = 0x8000;
pub const SICHINT_DISPLAY: i32 = 0x0;
pub const SICHINT_ALLFIELDS: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000;
pub const SICHINT_CANONICAL: i32 = 0x10000000;
pub const SICHINT_TEST_FILESYSPATH_IF_NOT_EQUAL: i32 = 0x20000000;
pub const SPBEGINF_NORMAL: i32 = 0x0;
pub const SPBEGINF_AUTOTIME: i32 = 0x2;
pub const SPBEGINF_NOPROGRESSBAR: i32 = 0x10;
pub const SPBEGINF_MARQUEEPROGRESS: i32 = 0x20;
pub const SPBEGINF_NOCANCELBUTTON: i32 = 0x40;
pub const SPINITF_NORMAL: i32 = 0x0;
pub const SPINITF_MODAL: i32 = 0x1;
pub const SPINITF_NOMINIMIZE: i32 = 0x8;
pub const SV3CVW3_DEFAULT: i32 = 0x0;
pub const SV3CVW3_NONINTERACTIVE: i32 = 0x1;
pub const SV3CVW3_FORCEVIEWMODE: i32 = 0x2;
pub const SV3CVW3_FORCEFOLDERFLAGS: i32 = 0x4;
pub const SVGIO_BACKGROUND: i32 = 0x0;
pub const SVGIO_SELECTION: i32 = 0x1;
pub const SVGIO_ALLVIEW: i32 = 0x2;
pub const SVGIO_CHECKED: i32 = 0x3;
pub const SVGIO_TYPE_MASK: i32 = 0xF;
pub const SVGIO_FLAG_VIEWORDER: i32 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000;
pub const SVSI_DESELECT: i32 = 0x0;
pub const SVSI_SELECT: i32 = 0x1;
pub const SVSI_EDIT: i32 = 0x3;
pub const SVSI_DESELECTOTHERS: i32 = 0x4;
pub const SVSI_ENSUREVISIBLE: i32 = 0x8;
pub const SVSI_FOCUSED: i32 = 0x10;
pub const SVSI_TRANSLATEPT: i32 = 0x20;
pub const SVSI_SELECTIONMARK: i32 = 0x40;
pub const SVSI_POSITIONITEM: i32 = 0x80;
pub const SVSI_CHECK: i32 = 0x100;
pub const SVSI_CHECK2: i32 = 0x200;
pub const SVSI_KEYBOARDSELECT: i32 = 0x401;
pub const SVSI_NOTAKEFOCUS: i32 = 0x40000000;
pub const TS_NONE: i32 = 0x0;
pub const TS_PERFORMING: i32 = 0x1;
pub const TS_PREPARING: i32 = 0x2;
pub const TS_INDETERMINATE: i32 = 0x4;
pub const TSF_NORMAL: i32 = 0x0;
pub const TSF_FAIL_EXIST: i32 = 0x0;
pub const TSF_RENAME_EXIST: i32 = 0x1;
pub const TSF_OVERWRITE_EXIST: i32 = 0x2;
pub const TSF_ALLOW_DECRYPTION: i32 = 0x4;
pub const TSF_NO_SECURITY: i32 = 0x8;
pub const TSF_COPY_CREATION_TIME: i32 = 0x10;
pub const TSF_COPY_WRITE_TIME: i32 = 0x20;
pub const TSF_USE_FULL_ACCESS: i32 = 0x40;
pub const TSF_DELETE_RECYCLE_IF_POSSIBLE: i32 = 0x80;
pub const TSF_COPY_HARD_LINK: i32 = 0x100;
pub const TSF_COPY_LOCALIZED_NAME: i32 = 0x200;
pub const TSF_MOVE_AS_COPY_DELETE: i32 = 0x400;
pub const TSF_SUSPEND_SHELLEVENTS: i32 = 0x800;
pub const IURL_INVOKECOMMAND_FL_ALLOW_UI: i32 = 0x1;
pub const IURL_INVOKECOMMAND_FL_USE_DEFAULT_VERB: i32 = 0x2;
pub const IURL_INVOKECOMMAND_FL_DDEWAIT: i32 = 0x4;
pub const IURL_INVOKECOMMAND_FL_ASYNCOK: i32 = 0x8;
pub const IURL_INVOKECOMMAND_FL_LOG_USAGE: i32 = 0x10;
pub const IURL_SETURL_FL_GUESS_PROTOCOL: i32 = 0x1;
pub const IURL_SETURL_FL_USE_DEFAULT_PROTOCOL: i32 = 0x2;
pub const MIMEASSOCDLG_FL_REGISTER_ASSOC: i32 = 0x1;
pub struct ProviderInfo<'a> {
    pub pi_R0_1val: todo_fn,
    pub pi_R0_allvals: todo_fn,
    pub pi_R3_1val: todo_fn,
    pub pi_R3_allvals: todo_fn,
    pub pi_flags: u32,
    pub pi_key_context: &'a mut todo_void,
}
pub struct PvalueA<'a> {
    pub pv_valuename: &'a mut Cow<'a, str>,
    pub pv_valuelen: i32,
    pub pv_value_context: &'a mut todo_void,
    pub pv_type: u32,
}
pub struct PvalueW<'a> {
    pub pv_valuename: &'a mut Cow<'a, OsStr>,
    pub pv_valuelen: i32,
    pub pv_value_context: &'a mut todo_void,
    pub pv_type: u32,
}
pub const TRANSLATEURL_FL_GUESS_PROTOCOL: i32 = 0x1;
pub const TRANSLATEURL_FL_USE_DEFAULT_PROTOCOL: i32 = 0x2;
pub const URLASSOCDLG_FL_USE_DEFAULT_NAME: i32 = 0x1;
pub const URLASSOCDLG_FL_REGISTER_ASSOC: i32 = 0x2;
pub struct UrlinvokecommandinfoA<'a> {
    pub dwcbSize: u32,
    pub dwFlags: u32,
    pub hwndParent: HWND,
    pub pcszVerb: &'a Cow<'a, str>,
}
pub struct UrlinvokecommandinfoW<'a> {
    pub dwcbSize: u32,
    pub dwFlags: u32,
    pub hwndParent: HWND,
    pub pcszVerb: &'a Cow<'a, OsStr>,
}
pub struct ValContext<'a> {
    pub valuelen: i32,
    pub value_context: &'a mut todo_void,
    pub val_buff_ptr: &'a mut todo_void,
}
// TODO: CloseHandle
// TODO: CompareObjectHandles
// TODO: DuplicateHandle
// TODO: GetHandleInformation
// TODO: GetLastError
// TODO: RtlNtStatusToDosError
// TODO: SetHandleInformation
// TODO: SetLastError
// TODO: SetLastErrorEx
// TODO: SysAddRefString
// TODO: SysAllocString
// TODO: SysAllocStringByteLen
// TODO: SysAllocStringLen
// TODO: SysFreeString
// TODO: SysReAllocString
// TODO: SysReAllocStringLen
// TODO: SysReleaseString
// TODO: SysStringByteLen
// TODO: SysStringLen
// TODO: GetRegistryValueWithFallbackW
// TODO: RegConnectRegistryA
// TODO: RegConnectRegistryExA
// TODO: RegConnectRegistryExW
// TODO: RegConnectRegistryW
// TODO: RegCopyTreeA
// TODO: RegCopyTreeW
// TODO: RegDeleteTreeA
// TODO: RegDeleteTreeW
// TODO: RegDeleteValueA
// TODO: RegDeleteValueW
// TODO: RegDisablePredefinedCache
// TODO: RegDisablePredefinedCacheEx
// TODO: RegEnumValueA
// TODO: RegEnumValueW
// TODO: RegGetValueA
// TODO: RegGetValueW
// TODO: RegLoadMUIStringA
// TODO: RegLoadMUIStringW
// TODO: RegOpenCurrentUser
// TODO: RegOpenUserClassesRoot
// TODO: RegQueryMultipleValuesA
// TODO: RegQueryMultipleValuesW
// TODO: RegQueryValueA
// TODO: RegQueryValueExA
// TODO: RegQueryValueExW
// TODO: RegQueryValueW
// TODO: RegSetValueA
// TODO: RegSetValueExA
// TODO: RegSetValueExW
// TODO: RegSetValueW
// TODO: AbortPath
// TODO: AddFontMemResourceEx
// TODO: AddFontResourceA
// TODO: AddFontResourceExA
// TODO: AddFontResourceExW
// TODO: AddFontResourceW
// TODO: AlphaBlend
// TODO: AngleArc
// TODO: AnimatePalette
// TODO: Arc
// TODO: ArcTo
// TODO: BeginPaint
// TODO: BeginPath
// TODO: BitBlt
// TODO: CancelDC
// TODO: ChangeDisplaySettingsA
// TODO: ChangeDisplaySettingsExA
// TODO: ChangeDisplaySettingsExW
// TODO: ChangeDisplaySettingsW
// TODO: Chord
// TODO: ClientToScreen
// TODO: CloseEnhMetaFile
// TODO: CloseFigure
// TODO: CloseMetaFile
// TODO: CombineRgn
// TODO: CombineTransform
// TODO: CopyEnhMetaFileA
// TODO: CopyEnhMetaFileW
// TODO: CopyMetaFileA
// TODO: CopyMetaFileW
// TODO: CopyRect
// TODO: CreateBitmap
// TODO: CreateBitmapIndirect
// TODO: CreateBrushIndirect
// TODO: CreateCompatibleBitmap
// TODO: CreateCompatibleDC
// TODO: CreateDCA
// TODO: CreateDCW
// TODO: CreateDIBPatternBrush
// TODO: CreateDIBPatternBrushPt
// TODO: CreateDIBSection
// TODO: CreateDIBitmap
// TODO: CreateDiscardableBitmap
// TODO: CreateEllipticRgn
// TODO: CreateEllipticRgnIndirect
// TODO: CreateEnhMetaFileA
// TODO: CreateEnhMetaFileW
// TODO: CreateFontA
// TODO: CreateFontIndirectA
// TODO: CreateFontIndirectExA
// TODO: CreateFontIndirectExW
// TODO: CreateFontIndirectW
// TODO: CreateFontPackage
// TODO: CreateFontW
// TODO: CreateHalftonePalette
// TODO: CreateHatchBrush
// TODO: CreateICA
// TODO: CreateICW
// TODO: CreateMetaFileA
// TODO: CreateMetaFileW
// TODO: CreatePalette
// TODO: CreatePatternBrush
// TODO: CreatePen
// TODO: CreatePenIndirect
// TODO: CreatePolyPolygonRgn
// TODO: CreatePolygonRgn
// TODO: CreateRectRgn
// TODO: CreateRectRgnIndirect
// TODO: CreateRoundRectRgn
// TODO: CreateScalableFontResourceA
// TODO: CreateScalableFontResourceW
// TODO: CreateSolidBrush
// TODO: DPtoLP
// TODO: DeleteDC
// TODO: DeleteEnhMetaFile
// TODO: DeleteMetaFile
// TODO: DeleteObject
// TODO: DrawAnimatedRects
// TODO: DrawCaption
// TODO: DrawEdge
// TODO: DrawEscape
// TODO: DrawFocusRect
// TODO: DrawFrameControl
// TODO: DrawStateA
// TODO: DrawStateW
// TODO: DrawTextA
// TODO: DrawTextExA
// TODO: DrawTextExW
// TODO: DrawTextW
// TODO: Ellipse
// TODO: EndPaint
// TODO: EndPath
// TODO: EnumDisplayDevicesA
// TODO: EnumDisplayDevicesW
// TODO: EnumDisplayMonitors
// TODO: EnumDisplaySettingsA
// TODO: EnumDisplaySettingsExA
// TODO: EnumDisplaySettingsExW
// TODO: EnumDisplaySettingsW
// TODO: EnumEnhMetaFile
// TODO: EnumFontFamiliesA
// TODO: EnumFontFamiliesExA
// TODO: EnumFontFamiliesExW
// TODO: EnumFontFamiliesW
// TODO: EnumFontsA
// TODO: EnumFontsW
// TODO: EnumMetaFile
// TODO: EnumObjects
// TODO: EqualRect
// TODO: EqualRgn
// TODO: ExcludeClipRect
// TODO: ExcludeUpdateRgn
// TODO: ExtCreatePen
// TODO: ExtCreateRegion
// TODO: ExtFloodFill
// TODO: ExtSelectClipRgn
// TODO: ExtTextOutA
// TODO: ExtTextOutW
// TODO: FillPath
// TODO: FillRect
// TODO: FillRgn
// TODO: FixBrushOrgEx
// TODO: FlattenPath
// TODO: FloodFill
// TODO: FrameRect
// TODO: FrameRgn
// TODO: GdiAlphaBlend
// TODO: GdiComment
// TODO: GdiFlush
// TODO: GdiGetBatchLimit
// TODO: GdiGradientFill
// TODO: GdiSetBatchLimit
// TODO: GdiTransparentBlt
// TODO: GetArcDirection
// TODO: GetAspectRatioFilterEx
// TODO: GetBitmapBits
// TODO: GetBitmapDimensionEx
// TODO: GetBkColor
// TODO: GetBkMode
// TODO: GetBoundsRect
// TODO: GetBrushOrgEx
// TODO: GetCharABCWidthsA
// TODO: GetCharABCWidthsFloatA
// TODO: GetCharABCWidthsFloatW
// TODO: GetCharABCWidthsI
// TODO: GetCharABCWidthsW
// TODO: GetCharWidth32A
// TODO: GetCharWidth32W
// TODO: GetCharWidthA
// TODO: GetCharWidthFloatA
// TODO: GetCharWidthFloatW
// TODO: GetCharWidthI
// TODO: GetCharWidthW
// TODO: GetCharacterPlacementA
// TODO: GetCharacterPlacementW
// TODO: GetClipBox
// TODO: GetClipRgn
// TODO: GetColorAdjustment
// TODO: GetCurrentObject
// TODO: GetCurrentPositionEx
// TODO: GetDC
// TODO: GetDCBrushColor
// TODO: GetDCEx
// TODO: GetDCOrgEx
// TODO: GetDCPenColor
// TODO: GetDIBColorTable
// TODO: GetDIBits
// TODO: GetDeviceCaps
// TODO: GetEnhMetaFileA
// TODO: GetEnhMetaFileBits
// TODO: GetEnhMetaFileDescriptionA
// TODO: GetEnhMetaFileDescriptionW
// TODO: GetEnhMetaFileHeader
// TODO: GetEnhMetaFilePaletteEntries
// TODO: GetEnhMetaFileW
// TODO: GetFontData
// TODO: GetFontLanguageInfo
// TODO: GetFontUnicodeRanges
// TODO: GetGlyphIndicesA
// TODO: GetGlyphIndicesW
// TODO: GetGlyphOutlineA
// TODO: GetGlyphOutlineW
// TODO: GetGraphicsMode
// TODO: GetKerningPairsA
// TODO: GetKerningPairsW
// TODO: GetLayout
// TODO: GetMapMode
// TODO: GetMetaFileA
// TODO: GetMetaFileBitsEx
// TODO: GetMetaFileW
// TODO: GetMetaRgn
// TODO: GetMiterLimit
// TODO: GetMonitorInfoA
// TODO: GetMonitorInfoW
// TODO: GetNearestColor
// TODO: GetNearestPaletteIndex
// TODO: GetObjectA
// TODO: GetObjectType
// TODO: GetObjectW
// TODO: GetOutlineTextMetricsA
// TODO: GetOutlineTextMetricsW
// TODO: GetPaletteEntries
// TODO: GetPath
// TODO: GetPixel
// TODO: GetPolyFillMode
// TODO: GetROP2
// TODO: GetRandomRgn
// TODO: GetRasterizerCaps
// TODO: GetRegionData
// TODO: GetRgnBox
// TODO: GetStockObject
// TODO: GetStretchBltMode
// TODO: GetSysColorBrush
// TODO: GetSystemPaletteEntries
// TODO: GetSystemPaletteUse
// TODO: GetTabbedTextExtentA
// TODO: GetTabbedTextExtentW
// TODO: GetTextAlign
// TODO: GetTextCharacterExtra
// TODO: GetTextColor
// TODO: GetTextExtentExPointA
// TODO: GetTextExtentExPointI
// TODO: GetTextExtentExPointW
// TODO: GetTextExtentPoint32A
// TODO: GetTextExtentPoint32W
// TODO: GetTextExtentPointA
// TODO: GetTextExtentPointI
// TODO: GetTextExtentPointW
// TODO: GetTextFaceA
// TODO: GetTextFaceW
// TODO: GetTextMetricsA
// TODO: GetTextMetricsW
// TODO: GetUpdateRect
// TODO: GetUpdateRgn
// TODO: GetViewportExtEx
// TODO: GetViewportOrgEx
// TODO: GetWinMetaFileBits
// TODO: GetWindowDC
// TODO: GetWindowExtEx
// TODO: GetWindowOrgEx
// TODO: GetWindowRgn
// TODO: GetWindowRgnBox
// TODO: GetWorldTransform
// TODO: GradientFill
// TODO: GrayStringA
// TODO: GrayStringW
// TODO: InflateRect
// TODO: IntersectClipRect
// TODO: IntersectRect
// TODO: InvalidateRect
// TODO: InvalidateRgn
// TODO: InvertRect
// TODO: InvertRgn
// TODO: IsRectEmpty
// TODO: LPtoDP
// TODO: LineDDA
// TODO: LineTo
// TODO: LoadBitmapA
// TODO: LoadBitmapW
// TODO: LockWindowUpdate
// TODO: MapWindowPoints
// TODO: MaskBlt
// TODO: MergeFontPackage
// TODO: ModifyWorldTransform
// TODO: MonitorFromPoint
// TODO: MonitorFromRect
// TODO: MonitorFromWindow
// TODO: MoveToEx
// TODO: OffsetClipRgn
// TODO: OffsetRect
// TODO: OffsetRgn
// TODO: OffsetViewportOrgEx
// TODO: OffsetWindowOrgEx
// TODO: PaintDesktop
// TODO: PaintRgn
// TODO: PatBlt
// TODO: PathToRegion
// TODO: Pie
// TODO: PlayEnhMetaFile
// TODO: PlayEnhMetaFileRecord
// TODO: PlayMetaFile
// TODO: PlayMetaFileRecord
// TODO: PlgBlt
// TODO: PolyBezier
// TODO: PolyBezierTo
// TODO: PolyDraw
// TODO: PolyPolygon
// TODO: PolyPolyline
// TODO: PolyTextOutA
// TODO: PolyTextOutW
// TODO: Polygon
// TODO: Polyline
// TODO: PolylineTo
// TODO: PtInRect
// TODO: PtInRegion
// TODO: PtVisible
// TODO: RealizePalette
// TODO: RectInRegion
// TODO: RectVisible
// TODO: Rectangle
// TODO: RedrawWindow
// TODO: ReleaseDC
// TODO: RemoveFontMemResourceEx
// TODO: RemoveFontResourceA
// TODO: RemoveFontResourceExA
// TODO: RemoveFontResourceExW
// TODO: RemoveFontResourceW
// TODO: ResetDCA
// TODO: ResetDCW
// TODO: ResizePalette
// TODO: RestoreDC
// TODO: RoundRect
// TODO: SaveDC
// TODO: ScaleViewportExtEx
// TODO: ScaleWindowExtEx
// TODO: ScreenToClient
// TODO: SelectClipPath
// TODO: SelectClipRgn
// TODO: SelectObject
// TODO: SelectPalette
// TODO: SetArcDirection
// TODO: SetBitmapBits
// TODO: SetBitmapDimensionEx
// TODO: SetBkColor
// TODO: SetBkMode
// TODO: SetBoundsRect
// TODO: SetBrushOrgEx
// TODO: SetColorAdjustment
// TODO: SetDCBrushColor
// TODO: SetDCPenColor
// TODO: SetDIBColorTable
// TODO: SetDIBits
// TODO: SetDIBitsToDevice
// TODO: SetEnhMetaFileBits
// TODO: SetGraphicsMode
// TODO: SetLayout
// TODO: SetMapMode
// TODO: SetMapperFlags
// TODO: SetMetaFileBitsEx
// TODO: SetMetaRgn
// TODO: SetMiterLimit
// TODO: SetPaletteEntries
// TODO: SetPixel
// TODO: SetPixelV
// TODO: SetPolyFillMode
// TODO: SetROP2
// TODO: SetRect
// TODO: SetRectEmpty
// TODO: SetRectRgn
// TODO: SetStretchBltMode
// TODO: SetSystemPaletteUse
// TODO: SetTextAlign
// TODO: SetTextCharacterExtra
// TODO: SetTextColor
// TODO: SetTextJustification
// TODO: SetViewportExtEx
// TODO: SetViewportOrgEx
// TODO: SetWindowExtEx
// TODO: SetWindowOrgEx
// TODO: SetWindowRgn
// TODO: SetWorldTransform
// TODO: StretchBlt
// TODO: StretchDIBits
// TODO: StrokeAndFillPath
// TODO: StrokePath
// TODO: SubtractRect
// TODO: TTCharToUnicode
// TODO: TTDeleteEmbeddedFont
// TODO: TTEmbedFont
// TODO: TTEmbedFontEx
// TODO: TTEmbedFontFromFileA
// TODO: TTEnableEmbeddingForFacename
// TODO: TTGetEmbeddedFontInfo
// TODO: TTGetEmbeddingType
// TODO: TTGetNewFontName
// TODO: TTIsEmbeddingEnabled
// TODO: TTIsEmbeddingEnabledForFacename
// TODO: TTLoadEmbeddedFont
// TODO: TTRunValidationTests
// TODO: TTRunValidationTestsEx
// TODO: TabbedTextOutA
// TODO: TabbedTextOutW
// TODO: TextOutA
// TODO: TextOutW
// TODO: TransparentBlt
// TODO: UnionRect
// TODO: UnrealizeObject
// TODO: UpdateColors
// TODO: UpdateWindow
// TODO: ValidateRect
// TODO: ValidateRgn
// TODO: WidenPath
// TODO: WindowFromDC
// TODO: wglSwapMultipleBuffers
// TODO: EnableMouseInPointer
// TODO: GetPointerCursorId
// TODO: GetPointerDevice
// TODO: GetPointerDeviceCursors
// TODO: GetPointerDeviceProperties
// TODO: GetPointerDeviceRects
// TODO: GetPointerDevices
// TODO: GetPointerFrameInfo
// TODO: GetPointerFrameInfoHistory
// TODO: GetPointerFramePenInfo
// TODO: GetPointerFramePenInfoHistory
// TODO: GetPointerFrameTouchInfo
// TODO: GetPointerFrameTouchInfoHistory
// TODO: GetPointerInfo
// TODO: GetPointerInfoHistory
// TODO: GetPointerInputTransform
// TODO: GetPointerPenInfo
// TODO: GetPointerPenInfoHistory
// TODO: GetPointerTouchInfo
// TODO: GetPointerTouchInfoHistory
// TODO: GetPointerType
// TODO: GetRawPointerDeviceData
// TODO: GetUnpredictedMessagePos
// TODO: InitializeTouchInjection
// TODO: InjectSyntheticPointerInput
// TODO: InjectTouchInput
// TODO: IsMouseInPointerEnabled
// TODO: SkipPointerFrameMessages
// TODO: BeginBufferedAnimation
// TODO: BeginBufferedPaint
// TODO: BeginPanningFeedback
// TODO: BufferedPaintClear
// TODO: BufferedPaintInit
// TODO: BufferedPaintRenderAnimation
// TODO: BufferedPaintSetAlpha
// TODO: BufferedPaintStopAllAnimations
// TODO: BufferedPaintUnInit
// TODO: CheckDlgButton
// TODO: CheckRadioButton
// TODO: CloseThemeData
// TODO: CreateMappedBitmap
// TODO: CreatePropertySheetPageA
// TODO: CreatePropertySheetPageW
// TODO: CreateStatusWindowA
// TODO: CreateStatusWindowW
// TODO: CreateSyntheticPointerDevice
// TODO: CreateToolbarEx
// TODO: CreateUpDownControl
// TODO: DPA_Clone
// TODO: DPA_Create
// TODO: DPA_CreateEx
// TODO: DPA_DeleteAllPtrs
// TODO: DPA_DeletePtr
// TODO: DPA_Destroy
// TODO: DPA_DestroyCallback
// TODO: DPA_EnumCallback
// TODO: DPA_GetPtr
// TODO: DPA_GetPtrIndex
// TODO: DPA_GetSize
// TODO: DPA_Grow
// TODO: DPA_InsertPtr
// TODO: DPA_Merge
// TODO: DPA_Search
// TODO: DPA_SetPtr
// TODO: DPA_Sort
// TODO: DSA_Clone
// TODO: DSA_Create
// TODO: DSA_DeleteAllItems
// TODO: DSA_DeleteItem
// TODO: DSA_Destroy
// TODO: DSA_DestroyCallback
// TODO: DSA_EnumCallback
// TODO: DSA_GetItem
// TODO: DSA_GetItemPtr
// TODO: DSA_GetSize
// TODO: DSA_InsertItem
// TODO: DSA_SetItem
// TODO: DSA_Sort
// TODO: DestroyPropertySheetPage
// TODO: DestroySyntheticPointerDevice
// TODO: DlgDirListA
// TODO: DlgDirListComboBoxA
// TODO: DlgDirListComboBoxW
// TODO: DlgDirListW
// TODO: DlgDirSelectComboBoxExA
// TODO: DlgDirSelectComboBoxExW
// TODO: DlgDirSelectExA
// TODO: DlgDirSelectExW
// TODO: DrawInsert
// TODO: DrawShadowText
// TODO: DrawStatusTextA
// TODO: DrawStatusTextW
// TODO: DrawThemeBackground
// TODO: DrawThemeBackgroundEx
// TODO: DrawThemeEdge
// TODO: DrawThemeIcon
// TODO: DrawThemeParentBackground
// TODO: DrawThemeParentBackgroundEx
// TODO: DrawThemeText
// TODO: DrawThemeTextEx
// TODO: EnableScrollBar
// TODO: EnableThemeDialogTexture
// TODO: EnableTheming
// TODO: EndBufferedAnimation
// TODO: EndBufferedPaint
// TODO: EndPanningFeedback
// TODO: EvaluateProximityToPolygon
// TODO: EvaluateProximityToRect
// TODO: FlatSB_EnableScrollBar
// TODO: FlatSB_GetScrollInfo
// TODO: FlatSB_GetScrollPos
// TODO: FlatSB_GetScrollProp
// TODO: FlatSB_GetScrollRange
// TODO: FlatSB_SetScrollInfo
// TODO: FlatSB_SetScrollPos
// TODO: FlatSB_SetScrollProp
// TODO: FlatSB_SetScrollRange
// TODO: FlatSB_ShowScrollBar
// TODO: GetBufferedPaintBits
// TODO: GetBufferedPaintDC
// TODO: GetBufferedPaintTargetDC
// TODO: GetBufferedPaintTargetRect
// TODO: GetComboBoxInfo
// TODO: GetCurrentThemeName
// TODO: GetEffectiveClientRect
// TODO: GetListBoxInfo
// TODO: GetMUILanguage
// TODO: GetThemeAnimationProperty
// TODO: GetThemeAnimationTransform
// TODO: GetThemeAppProperties
// TODO: GetThemeBackgroundContentRect
// TODO: GetThemeBackgroundExtent
// TODO: GetThemeBackgroundRegion
// TODO: GetThemeBitmap
// TODO: GetThemeBool
// TODO: GetThemeColor
// TODO: GetThemeDocumentationProperty
// TODO: GetThemeEnumValue
// TODO: GetThemeFilename
// TODO: GetThemeFont
// TODO: GetThemeInt
// TODO: GetThemeIntList
// TODO: GetThemeMargins
// TODO: GetThemeMetric
// TODO: GetThemePartSize
// TODO: GetThemePosition
// TODO: GetThemePropertyOrigin
// TODO: GetThemeRect
// TODO: GetThemeStream
// TODO: GetThemeString
// TODO: GetThemeSysBool
// TODO: GetThemeSysColor
// TODO: GetThemeSysColorBrush
// TODO: GetThemeSysFont
// TODO: GetThemeSysInt
// TODO: GetThemeSysSize
// TODO: GetThemeSysString
// TODO: GetThemeTextExtent
// TODO: GetThemeTextMetrics
// TODO: GetThemeTimingFunction
// TODO: GetThemeTransitionDuration
// TODO: GetWindowFeedbackSetting
// TODO: GetWindowTheme
// TODO: HIMAGELIST_QueryInterface
// TODO: HitTestThemeBackground
// TODO: InitCommonControls
// TODO: InitCommonControlsEx
// TODO: InitMUILanguage
// TODO: InitializeFlatSB
// TODO: IsAppThemed
// TODO: IsCharLowerW
// TODO: IsCompositionActive
// TODO: IsDlgButtonChecked
// TODO: IsThemeActive
// TODO: IsThemeBackgroundPartiallyTransparent
// TODO: IsThemeDialogTextureEnabled
// TODO: IsThemePartDefined
// TODO: LBItemFromPt
// TODO: LoadIconMetric
// TODO: LoadIconWithScaleDown
// TODO: MakeDragList
// TODO: MenuHelp
// TODO: OpenThemeData
// TODO: OpenThemeDataEx
// TODO: PackTouchHitTestingProximityEvaluation
// TODO: PropertySheetA
// TODO: PropertySheetW
// TODO: RegisterPointerDeviceNotifications
// TODO: RegisterTouchHitTestingWindow
// TODO: SetScrollInfo
// TODO: SetScrollPos
// TODO: SetScrollRange
// TODO: SetThemeAppProperties
// TODO: SetWindowFeedbackSetting
// TODO: SetWindowTheme
// TODO: SetWindowThemeAttribute
// TODO: ShowHideMenuCtl
// TODO: ShowScrollBar
// TODO: Str_SetPtrW
// TODO: TaskDialog
// TODO: TaskDialogIndirect
// TODO: UninitializeFlatSB
// TODO: UpdatePanningFeedback
// TODO: ChrCmpIA
// TODO: ChrCmpIW
// TODO: ColorAdjustLuma
// TODO: ColorHLSToRGB
// TODO: ColorRGBToHLS
// TODO: CommandLineToArgvW
// TODO: CreateProfile
// TODO: DAD_AutoScroll
// TODO: DAD_DragEnterEx
// TODO: DAD_DragLeave
// TODO: DAD_DragMove
// TODO: DAD_SetDragImage
// TODO: DAD_ShowDragImage
// TODO: DefSubclassProc
// TODO: DeleteProfileA
// TODO: DeleteProfileW
// TODO: DoEnvironmentSubstA
// TODO: DoEnvironmentSubstW
// TODO: DragAcceptFiles
// TODO: DragFinish
// TODO: DragQueryFileA
// TODO: DragQueryFileW
// TODO: DragQueryPoint
// TODO: DriveType
// TODO: DuplicateIcon
// TODO: ExtractAssociatedIconA
// TODO: ExtractAssociatedIconExA
// TODO: ExtractAssociatedIconExW
// TODO: ExtractAssociatedIconW
// TODO: ExtractIconA
// TODO: ExtractIconExA
// TODO: ExtractIconExW
// TODO: ExtractIconW
// TODO: FindExecutableA
// TODO: FindExecutableW
// TODO: GetAcceptLanguagesA
// TODO: GetAcceptLanguagesW
// TODO: GetAllUsersProfileDirectoryA
// TODO: GetAllUsersProfileDirectoryW
// TODO: GetCurrentProcessExplicitAppUserModelID
// TODO: GetDefaultUserProfileDirectoryA
// TODO: GetDefaultUserProfileDirectoryW
// TODO: GetFileNameFromBrowse
// TODO: GetMenuContextHelpId
// TODO: GetMenuPosFromID
// TODO: GetProfileType
// TODO: GetProfilesDirectoryA
// TODO: GetProfilesDirectoryW
// TODO: GetUserProfileDirectoryA
// TODO: GetUserProfileDirectoryW
// TODO: GetWindowContextHelpId
// TODO: GetWindowSubclass
// TODO: HMONITOR_UserFree
// TODO: HMONITOR_UserFree64
// TODO: HMONITOR_UserMarshal
// TODO: HMONITOR_UserMarshal64
// TODO: HMONITOR_UserSize
// TODO: HMONITOR_UserSize64
// TODO: HMONITOR_UserUnmarshal
// TODO: HMONITOR_UserUnmarshal64
// TODO: HashData
// TODO: ImportPrivacySettings
// TODO: InitNetworkAddressControl
// TODO: IntlStrEqWorkerA
// TODO: IntlStrEqWorkerW
// TODO: IsCharSpaceA
// TODO: IsCharSpaceW
// TODO: IsInternetESCEnabled
// TODO: IsLFNDriveA
// TODO: IsLFNDriveW
// TODO: IsNetDrive
// TODO: IsOS
// TODO: IsUserAnAdmin
// TODO: LoadUserProfileA
// TODO: LoadUserProfileW
// TODO: ParseURLA
// TODO: ParseURLW
// TODO: PathAddBackslashA
// TODO: PathAddBackslashW
// TODO: PathAddExtensionA
// TODO: PathAddExtensionW
// TODO: PathAllocCanonicalize
// TODO: PathAllocCombine
// TODO: PathAppendA
// TODO: PathAppendW
// TODO: PathBuildRootA
// TODO: PathBuildRootW
// TODO: PathCanonicalizeA
// TODO: PathCanonicalizeW
// TODO: PathCchAddBackslash
// TODO: PathCchAddBackslashEx
// TODO: PathCchAddExtension
// TODO: PathCchAppend
// TODO: PathCchAppendEx
// TODO: PathCchCanonicalize
// TODO: PathCchCanonicalizeEx
// TODO: PathCchCombine
// TODO: PathCchCombineEx
// TODO: PathCchFindExtension
// TODO: PathCchIsRoot
// TODO: PathCchRemoveBackslash
// TODO: PathCchRemoveBackslashEx
// TODO: PathCchRemoveExtension
// TODO: PathCchRemoveFileSpec
// TODO: PathCchRenameExtension
// TODO: PathCchSkipRoot
// TODO: PathCchStripPrefix
// TODO: PathCchStripToRoot
// TODO: PathCleanupSpec
// TODO: PathCombineA
// TODO: PathCombineW
// TODO: PathCommonPrefixA
// TODO: PathCommonPrefixW
// TODO: PathCompactPathA
// TODO: PathCompactPathExA
// TODO: PathCompactPathExW
// TODO: PathCompactPathW
// TODO: PathCreateFromUrlA
// TODO: PathCreateFromUrlAlloc
// TODO: PathCreateFromUrlW
// TODO: PathFileExistsA
// TODO: PathFileExistsW
// TODO: PathFindExtensionA
// TODO: PathFindExtensionW
// TODO: PathFindFileNameA
// TODO: PathFindFileNameW
// TODO: PathFindNextComponentA
// TODO: PathFindNextComponentW
// TODO: PathFindOnPathA
// TODO: PathFindOnPathW
// TODO: PathFindSuffixArrayA
// TODO: PathFindSuffixArrayW
// TODO: PathGetArgsA
// TODO: PathGetArgsW
// TODO: PathGetCharTypeA
// TODO: PathGetCharTypeW
// TODO: PathGetDriveNumberA
// TODO: PathGetDriveNumberW
// TODO: PathGetShortPath
// TODO: PathIsContentTypeA
// TODO: PathIsContentTypeW
// TODO: PathIsDirectoryA
// TODO: PathIsDirectoryEmptyA
// TODO: PathIsDirectoryEmptyW
// TODO: PathIsDirectoryW
// TODO: PathIsExe
// TODO: PathIsFileSpecA
// TODO: PathIsFileSpecW
// TODO: PathIsLFNFileSpecA
// TODO: PathIsLFNFileSpecW
// TODO: PathIsNetworkPathA
// TODO: PathIsNetworkPathW
// TODO: PathIsPrefixA
// TODO: PathIsPrefixW
// TODO: PathIsRelativeA
// TODO: PathIsRelativeW
// TODO: PathIsRootA
// TODO: PathIsRootW
// TODO: PathIsSameRootA
// TODO: PathIsSameRootW
// TODO: PathIsSlowA
// TODO: PathIsSlowW
// TODO: PathIsSystemFolderA
// TODO: PathIsSystemFolderW
// TODO: PathIsUNCA
// TODO: PathIsUNCEx
// TODO: PathIsUNCServerA
// TODO: PathIsUNCServerShareA
// TODO: PathIsUNCServerShareW
// TODO: PathIsUNCServerW
// TODO: PathIsUNCW
// TODO: PathIsURLA
// TODO: PathIsURLW
// TODO: PathMakePrettyA
// TODO: PathMakePrettyW
// TODO: PathMakeSystemFolderA
// TODO: PathMakeSystemFolderW
// TODO: PathMakeUniqueName
// TODO: PathMatchSpecA
// TODO: PathMatchSpecExA
// TODO: PathMatchSpecExW
// TODO: PathMatchSpecW
// TODO: PathParseIconLocationA
// TODO: PathParseIconLocationW
// TODO: PathQualify
// TODO: PathQuoteSpacesA
// TODO: PathQuoteSpacesW
// TODO: PathRelativePathToA
// TODO: PathRelativePathToW
// TODO: PathRemoveArgsA
// TODO: PathRemoveArgsW
// TODO: PathRemoveBackslashA
// TODO: PathRemoveBackslashW
// TODO: PathRemoveBlanksA
// TODO: PathRemoveBlanksW
// TODO: PathRemoveExtensionA
// TODO: PathRemoveExtensionW
// TODO: PathRemoveFileSpecA
// TODO: PathRemoveFileSpecW
// TODO: PathRenameExtensionA
// TODO: PathRenameExtensionW
// TODO: PathResolve
// TODO: PathSearchAndQualifyA
// TODO: PathSearchAndQualifyW
// TODO: PathSetDlgItemPathA
// TODO: PathSetDlgItemPathW
// TODO: PathSkipRootA
// TODO: PathSkipRootW
// TODO: PathStripPathA
// TODO: PathStripPathW
// TODO: PathStripToRootA
// TODO: PathStripToRootW
// TODO: PathUnExpandEnvStringsA
// TODO: PathUnExpandEnvStringsW
// TODO: PathUndecorateA
// TODO: PathUndecorateW
// TODO: PathUnmakeSystemFolderA
// TODO: PathUnmakeSystemFolderW
// TODO: PathUnquoteSpacesA
// TODO: PathUnquoteSpacesW
// TODO: PathYetAnotherMakeUniqueName
// TODO: PickIconDlg
// TODO: QISearch
// TODO: ReadCabinetState
// TODO: RealDriveType
// TODO: RegisterAppConstrainedChangeNotification
// TODO: RegisterAppStateChangeNotification
// TODO: RegisterScaleChangeEvent
// TODO: RegisterScaleChangeNotifications
// TODO: RemoveWindowSubclass
// TODO: RestartDialog
// TODO: RestartDialogEx
// TODO: RevokeScaleChangeNotifications
// TODO: SHAddFromPropSheetExtArray
// TODO: SHAddToRecentDocs
// TODO: SHAlloc
// TODO: SHAllocShared
// TODO: SHAnsiToAnsi
// TODO: SHAnsiToUnicode
// TODO: SHAppBarMessage
// TODO: SHAssocEnumHandlers
// TODO: SHAssocEnumHandlersForProtocolByApplication
// TODO: SHAutoComplete
// TODO: SHCLSIDFromString
// TODO: SHEmptyRecycleBinA
// TODO: SHEmptyRecycleBinW
// TODO: SHEvaluateSystemCommandTemplate
// TODO: SHFileOperationA
// TODO: SHFileOperationW
// TODO: SHFind_InitMenuPopup
// TODO: SHFlushSFCache
// TODO: SHFormatDateTimeA
// TODO: SHFormatDateTimeW
// TODO: SHFormatDrive
// TODO: SHFree
// TODO: SHFreeNameMappings
// TODO: SHFreeShared
// TODO: SHGlobalCounterDecrement
// TODO: SHGlobalCounterGetValue
// TODO: SHGlobalCounterIncrement
// TODO: SHInvokePrinterCommandA
// TODO: SHInvokePrinterCommandW
// TODO: SHIsFileAvailableOffline
// TODO: SHIsLowMemoryMachine
// TODO: SHLimitInputEdit
// TODO: SHLoadInProc
// TODO: SHLoadIndirectString
// TODO: SHLoadNonloadedIconOverlayIdentifiers
// TODO: SHLockShared
// TODO: SHMessageBoxCheckA
// TODO: SHMessageBoxCheckW
// TODO: SHObjectProperties
// TODO: SHOpenWithDialog
// TODO: SHShellFolderView_Message
// TODO: SHShowManageLibraryUI
// TODO: SHStartNetConnectionDialogW
// TODO: SHStrDupA
// TODO: SHStrDupW
// TODO: SHStripMneumonicA
// TODO: SHStripMneumonicW
// TODO: SHTestTokenMembership
// TODO: SHUnicodeToAnsi
// TODO: SHUnicodeToUnicode
// TODO: SHUnlockShared
// TODO: SHUpdateImageA
// TODO: SHUpdateImageW
// TODO: SHValidateUNC
// TODO: SetCurrentProcessExplicitAppUserModelID
// TODO: SetMenuContextHelpId
// TODO: SetWindowContextHelpId
// TODO: SetWindowSubclass
// TODO: ShellAboutA
// TODO: ShellAboutW
// TODO: ShellExecuteA
// TODO: ShellExecuteW
// TODO: ShellMessageBoxA
// TODO: ShellMessageBoxW
// TODO: Shell_GetCachedImageIndex
// TODO: Shell_GetCachedImageIndexA
// TODO: Shell_GetCachedImageIndexW
// TODO: Shell_MergeMenus
// TODO: Shell_NotifyIconA
// TODO: Shell_NotifyIconGetRect
// TODO: Shell_NotifyIconW
// TODO: StrCSpnA
// TODO: StrCSpnIA
// TODO: StrCSpnIW
// TODO: StrCSpnW
// TODO: StrCatBuffA
// TODO: StrCatBuffW
// TODO: StrCatChainW
// TODO: StrCatW
// TODO: StrChrA
// TODO: StrChrIA
// TODO: StrChrIW
// TODO: StrChrNIW
// TODO: StrChrNW
// TODO: StrChrW
// TODO: StrCmpCA
// TODO: StrCmpCW
// TODO: StrCmpICA
// TODO: StrCmpICW
// TODO: StrCmpIW
// TODO: StrCmpLogicalW
// TODO: StrCmpNA
// TODO: StrCmpNCA
// TODO: StrCmpNCW
// TODO: StrCmpNIA
// TODO: StrCmpNICA
// TODO: StrCmpNICW
// TODO: StrCmpNIW
// TODO: StrCmpNW
// TODO: StrCmpW
// TODO: StrCpyNW
// TODO: StrCpyW
// TODO: StrDupA
// TODO: StrDupW
// TODO: StrFormatByteSize64A
// TODO: StrFormatByteSizeA
// TODO: StrFormatByteSizeEx
// TODO: StrFormatByteSizeW
// TODO: StrFormatKBSizeA
// TODO: StrFormatKBSizeW
// TODO: StrFromTimeIntervalA
// TODO: StrFromTimeIntervalW
// TODO: StrIsIntlEqualA
// TODO: StrIsIntlEqualW
// TODO: StrNCatA
// TODO: StrNCatW
// TODO: StrPBrkA
// TODO: StrPBrkW
// TODO: StrRChrA
// TODO: StrRChrIA
// TODO: StrRChrIW
// TODO: StrRChrW
// TODO: StrRStrIA
// TODO: StrRStrIW
// TODO: StrSpnA
// TODO: StrSpnW
// TODO: StrStrA
// TODO: StrStrIA
// TODO: StrStrIW
// TODO: StrStrNIW
// TODO: StrStrNW
// TODO: StrStrW
// TODO: StrToInt64ExA
// TODO: StrToInt64ExW
// TODO: StrToIntA
// TODO: StrToIntExA
// TODO: StrToIntExW
// TODO: StrToIntW
// TODO: StrTrimA
// TODO: StrTrimW
// TODO: UnloadUserProfile
// TODO: UnregisterAppConstrainedChangeNotification
// TODO: UnregisterAppStateChangeNotification
// TODO: UnregisterScaleChangeEvent
// TODO: UrlApplySchemeA
// TODO: UrlApplySchemeW
// TODO: UrlCanonicalizeA
// TODO: UrlCanonicalizeW
// TODO: UrlCombineA
// TODO: UrlCombineW
// TODO: UrlCompareA
// TODO: UrlCompareW
// TODO: UrlCreateFromPathA
// TODO: UrlCreateFromPathW
// TODO: UrlEscapeA
// TODO: UrlEscapeW
// TODO: UrlFixupW
// TODO: UrlGetLocationA
// TODO: UrlGetLocationW
// TODO: UrlGetPartA
// TODO: UrlGetPartW
// TODO: UrlHashA
// TODO: UrlHashW
// TODO: UrlIsA
// TODO: UrlIsNoHistoryA
// TODO: UrlIsNoHistoryW
// TODO: UrlIsOpaqueA
// TODO: UrlIsOpaqueW
// TODO: UrlIsW
// TODO: UrlUnescapeA
// TODO: UrlUnescapeW
// TODO: WhichPlatform
// TODO: Win32DeleteFile
// TODO: WinHelpA
// TODO: WinHelpW
// TODO: WriteCabinetState
// TODO: wnsprintfA
// TODO: wnsprintfW
// TODO: wvnsprintfA
// TODO: wvnsprintfW
// TODO: AdjustWindowRect
// TODO: AdjustWindowRectEx
// TODO: AllowSetForegroundWindow
// TODO: AnimateWindow
// TODO: AnyPopup
// TODO: AppendMenuA
// TODO: AppendMenuW
// TODO: ArrangeIconicWindows
// TODO: BeginDeferWindowPos
// TODO: BringWindowToTop
// TODO: CalculatePopupWindowPosition
// TODO: CallMsgFilterA
// TODO: CallMsgFilterW
// TODO: CallNextHookEx
// TODO: CallWindowProcA
// TODO: CallWindowProcW
// TODO: CancelShutdown
// TODO: CascadeWindows
// TODO: ChangeMenuA
// TODO: ChangeMenuW
// TODO: ChangeWindowMessageFilter
// TODO: ChangeWindowMessageFilterEx
// TODO: CharLowerA
// TODO: CharLowerBuffA
// TODO: CharLowerBuffW
// TODO: CharLowerW
// TODO: CharNextA
// TODO: CharNextExA
// TODO: CharNextW
// TODO: CharPrevA
// TODO: CharPrevExA
// TODO: CharPrevW
// TODO: CharToOemA
// TODO: CharToOemBuffA
// TODO: CharToOemBuffW
// TODO: CharToOemW
// TODO: CharUpperA
// TODO: CharUpperBuffA
// TODO: CharUpperBuffW
// TODO: CharUpperW
// TODO: CheckMenuItem
// TODO: CheckMenuRadioItem
// TODO: ChildWindowFromPoint
// TODO: ChildWindowFromPointEx
// TODO: ClipCursor
// TODO: CloseWindow
// TODO: CopyAcceleratorTableA
// TODO: CopyAcceleratorTableW
// TODO: CopyIcon
// TODO: CopyImage
// TODO: CreateAcceleratorTableA
// TODO: CreateAcceleratorTableW
// TODO: CreateCaret
// TODO: CreateCursor
// TODO: CreateDialogIndirectParamA
// TODO: CreateDialogIndirectParamW
// TODO: CreateDialogParamA
// TODO: CreateDialogParamW
// TODO: CreateIcon
// TODO: CreateIconFromResource
// TODO: CreateIconFromResourceEx
// TODO: CreateIconIndirect
// TODO: CreateMDIWindowA
// TODO: CreateMDIWindowW
// TODO: CreateMenu
// TODO: CreatePopupMenu
// TODO: CreateResourceIndexer
// TODO: CreateWindowExA
// TODO: CreateWindowExW
// TODO: DefDlgProcA
// TODO: DefDlgProcW
// TODO: DefFrameProcA
// TODO: DefFrameProcW
// TODO: DefMDIChildProcA
// TODO: DefMDIChildProcW
// TODO: DefWindowProcA
// TODO: DefWindowProcW
// TODO: DeferWindowPos
// TODO: DeleteMenu
// TODO: DeregisterShellHookWindow
// TODO: DestroyAcceleratorTable
// TODO: DestroyCaret
// TODO: DestroyCursor
// TODO: DestroyIcon
// TODO: DestroyIndexedResults
// TODO: DestroyMenu
// TODO: DestroyResourceIndexer
// TODO: DestroyWindow
// TODO: DialogBoxIndirectParamA
// TODO: DialogBoxIndirectParamW
// TODO: DialogBoxParamA
// TODO: DialogBoxParamW
// TODO: DisableProcessWindowsGhosting
// TODO: DispatchMessageA
// TODO: DispatchMessageW
// TODO: DragObject
// TODO: DrawIcon
// TODO: DrawIconEx
// TODO: DrawMenuBar
// TODO: EnableMenuItem
// TODO: EndDeferWindowPos
// TODO: EndDialog
// TODO: EndMenu
// TODO: EnumChildWindows
// TODO: EnumPropsA
// TODO: EnumPropsExA
// TODO: EnumPropsExW
// TODO: EnumPropsW
// TODO: EnumThreadWindows
// TODO: EnumWindows
// TODO: FindWindowA
// TODO: FindWindowExA
// TODO: FindWindowExW
// TODO: FindWindowW
// TODO: FlashWindow
// TODO: FlashWindowEx
// TODO: GetAltTabInfoA
// TODO: GetAltTabInfoW
// TODO: GetAncestor
// TODO: GetCaretBlinkTime
// TODO: GetCaretPos
// TODO: GetClassInfoA
// TODO: GetClassInfoExA
// TODO: GetClassInfoExW
// TODO: GetClassInfoW
// TODO: GetClassLongA
// TODO: GetClassLongPtrA
// TODO: GetClassLongPtrW
// TODO: GetClassLongW
// TODO: GetClassNameA
// TODO: GetClassNameW
// TODO: GetClassWord
// TODO: GetClientRect
// TODO: GetClipCursor
// TODO: GetCursor
// TODO: GetCursorInfo
// TODO: GetCursorPos
// TODO: GetDesktopWindow
// TODO: GetDialogBaseUnits
// TODO: GetDlgCtrlID
// TODO: GetDlgItem
// TODO: GetDlgItemInt
// TODO: GetDlgItemTextA
// TODO: GetDlgItemTextW
// TODO: GetForegroundWindow
// TODO: GetGUIThreadInfo
// TODO: GetIconInfo
// TODO: GetIconInfoExA
// TODO: GetIconInfoExW
// TODO: GetInputState
// TODO: GetLastActivePopup
// TODO: GetLayeredWindowAttributes
// TODO: GetMenu
// TODO: GetMenuBarInfo
// TODO: GetMenuCheckMarkDimensions
// TODO: GetMenuDefaultItem
// TODO: GetMenuInfo
// TODO: GetMenuItemCount
// TODO: GetMenuItemID
// TODO: GetMenuItemInfoA
// TODO: GetMenuItemInfoW
// TODO: GetMenuItemRect
// TODO: GetMenuState
// TODO: GetMenuStringA
// TODO: GetMenuStringW
// TODO: GetMessageA
// TODO: GetMessageExtraInfo
// TODO: GetMessagePos
// TODO: GetMessageTime
// TODO: GetMessageW
// TODO: GetNextDlgGroupItem
// TODO: GetNextDlgTabItem
// TODO: GetParent
// TODO: GetPhysicalCursorPos
// TODO: GetProcessDefaultLayout
// TODO: GetPropA
// TODO: GetPropW
// TODO: GetQueueStatus
// TODO: GetScrollBarInfo
// TODO: GetScrollInfo
// TODO: GetScrollPos
// TODO: GetScrollRange
// TODO: GetShellWindow
// TODO: GetSubMenu
// TODO: GetSysColor
// TODO: GetSystemMenu
// TODO: GetSystemMetrics
// TODO: GetTitleBarInfo
// TODO: GetTopWindow
// TODO: GetWindow
// TODO: GetWindowDisplayAffinity
// TODO: GetWindowInfo
// TODO: GetWindowLongA
// TODO: GetWindowLongPtrA
// TODO: GetWindowLongPtrW
// TODO: GetWindowLongW
// TODO: GetWindowModuleFileNameA
// TODO: GetWindowModuleFileNameW
// TODO: GetWindowPlacement
// TODO: GetWindowRect
// TODO: GetWindowTextA
// TODO: GetWindowTextLengthA
// TODO: GetWindowTextLengthW
// TODO: GetWindowTextW
// TODO: GetWindowThreadProcessId
// TODO: GetWindowWord
// TODO: HideCaret
// TODO: HiliteMenuItem
// TODO: InSendMessage
// TODO: InSendMessageEx
// TODO: IndexFilePath
// TODO: InheritWindowMonitor
// TODO: InsertMenuA
// TODO: InsertMenuItemA
// TODO: InsertMenuItemW
// TODO: InsertMenuW
// TODO: InternalGetWindowText
// TODO: IsCharAlphaA
// TODO: IsCharAlphaNumericA
// TODO: IsCharAlphaNumericW
// TODO: IsCharAlphaW
// TODO: IsCharLowerA
// TODO: IsCharUpperA
// TODO: IsCharUpperW
// TODO: IsChild
// TODO: IsDialogMessageA
// TODO: IsDialogMessageW
// TODO: IsGUIThread
// TODO: IsHungAppWindow
// TODO: IsIconic
// TODO: IsMenu
// TODO: IsProcessDPIAware
// TODO: IsWindow
// TODO: IsWindowUnicode
// TODO: IsWindowVisible
// TODO: IsWow64Message
// TODO: IsZoomed
// TODO: KillTimer
// TODO: LoadAcceleratorsA
// TODO: LoadAcceleratorsW
// TODO: LoadCursorA
// TODO: LoadCursorFromFileA
// TODO: LoadCursorFromFileW
// TODO: LoadCursorW
// TODO: LoadIconA
// TODO: LoadIconW
// TODO: LoadImageA
// TODO: LoadImageW
// TODO: LoadMenuA
// TODO: LoadMenuIndirectA
// TODO: LoadMenuIndirectW
// TODO: LoadMenuW
// TODO: LoadStringA
// TODO: LoadStringW
// TODO: LockSetForegroundWindow
// TODO: LogicalToPhysicalPoint
// TODO: LookupIconIdFromDirectory
// TODO: LookupIconIdFromDirectoryEx
// TODO: MapDialogRect
// TODO: MenuItemFromPoint
// TODO: MessageBoxA
// TODO: MessageBoxExA
// TODO: MessageBoxExW
// TODO: MessageBoxIndirectA
// TODO: MessageBoxIndirectW
// TODO: MessageBoxW
// TODO: ModifyMenuA
// TODO: ModifyMenuW
// TODO: MoveWindow
// TODO: MrmCreateConfig
// TODO: MrmCreateConfigInMemory
// TODO: MrmCreateResourceFile
// TODO: MrmCreateResourceFileInMemory
// TODO: MrmCreateResourceFileWithChecksum
// TODO: MrmCreateResourceIndexer
// TODO: MrmCreateResourceIndexerFromPreviousPriData
// TODO: MrmCreateResourceIndexerFromPreviousPriFile
// TODO: MrmCreateResourceIndexerFromPreviousSchemaData
// TODO: MrmCreateResourceIndexerFromPreviousSchemaFile
// TODO: MrmCreateResourceIndexerWithFlags
// TODO: MrmDestroyIndexerAndMessages
// TODO: MrmDumpPriDataInMemory
// TODO: MrmDumpPriFile
// TODO: MrmDumpPriFileInMemory
// TODO: MrmFreeMemory
// TODO: MrmGetPriFileContentChecksum
// TODO: MrmIndexEmbeddedData
// TODO: MrmIndexFile
// TODO: MrmIndexFileAutoQualifiers
// TODO: MrmIndexResourceContainerAutoQualifiers
// TODO: MrmIndexString
// TODO: MrmPeekResourceIndexerMessages
// TODO: MsgWaitForMultipleObjects
// TODO: MsgWaitForMultipleObjectsEx
// TODO: OemToCharA
// TODO: OemToCharBuffA
// TODO: OemToCharBuffW
// TODO: OemToCharW
// TODO: OpenIcon
// TODO: PeekMessageA
// TODO: PeekMessageW
// TODO: PhysicalToLogicalPoint
// TODO: PostMessageA
// TODO: PostMessageW
// TODO: PostQuitMessage
// TODO: PostThreadMessageA
// TODO: PostThreadMessageW
// TODO: PrivateExtractIconsA
// TODO: PrivateExtractIconsW
// TODO: RealChildWindowFromPoint
// TODO: RealGetWindowClassA
// TODO: RealGetWindowClassW
// TODO: RegisterClassA
// TODO: RegisterClassExA
// TODO: RegisterClassExW
// TODO: RegisterClassW
// TODO: RegisterShellHookWindow
// TODO: RegisterWindowMessageA
// TODO: RegisterWindowMessageW
// TODO: RemoveMenu
// TODO: RemovePropA
// TODO: RemovePropW
// TODO: ReplyMessage
// TODO: ScrollDC
// TODO: ScrollWindow
// TODO: ScrollWindowEx
// TODO: SendDlgItemMessageA
// TODO: SendDlgItemMessageW
// TODO: SendMessageA
// TODO: SendMessageCallbackA
// TODO: SendMessageCallbackW
// TODO: SendMessageTimeoutA
// TODO: SendMessageTimeoutW
// TODO: SendMessageW
// TODO: SendNotifyMessageA
// TODO: SendNotifyMessageW
// TODO: SetCaretBlinkTime
// TODO: SetCaretPos
// TODO: SetClassLongA
// TODO: SetClassLongPtrA
// TODO: SetClassLongPtrW
// TODO: SetClassLongW
// TODO: SetClassWord
// TODO: SetCoalescableTimer
// TODO: SetCursor
// TODO: SetCursorPos
// TODO: SetDebugErrorLevel
// TODO: SetDlgItemInt
// TODO: SetDlgItemTextA
// TODO: SetDlgItemTextW
// TODO: SetForegroundWindow
// TODO: SetLayeredWindowAttributes
// TODO: SetMenu
// TODO: SetMenuDefaultItem
// TODO: SetMenuInfo
// TODO: SetMenuItemBitmaps
// TODO: SetMenuItemInfoA
// TODO: SetMenuItemInfoW
// TODO: SetMessageExtraInfo
// TODO: SetMessageQueue
// TODO: SetParent
// TODO: SetPhysicalCursorPos
// TODO: SetProcessDPIAware
// TODO: SetProcessDefaultLayout
// TODO: SetPropA
// TODO: SetPropW
// TODO: SetSysColors
// TODO: SetSystemCursor
// TODO: SetTimer
// TODO: SetWindowDisplayAffinity
// TODO: SetWindowLongA
// TODO: SetWindowLongPtrA
// TODO: SetWindowLongPtrW
// TODO: SetWindowLongW
// TODO: SetWindowPlacement
// TODO: SetWindowPos
// TODO: SetWindowTextA
// TODO: SetWindowTextW
// TODO: SetWindowWord
// TODO: SetWindowsHookA
// TODO: SetWindowsHookExA
// TODO: SetWindowsHookExW
// TODO: SetWindowsHookW
// TODO: ShowCaret
// TODO: ShowCursor
// TODO: ShowOwnedPopups
// TODO: ShowWindow
// TODO: ShowWindowAsync
// TODO: SoundSentry
// TODO: SwitchToThisWindow
// TODO: SystemParametersInfoA
// TODO: SystemParametersInfoW
// TODO: TileWindows
// TODO: TrackPopupMenu
// TODO: TrackPopupMenuEx
// TODO: TranslateAcceleratorA
// TODO: TranslateAcceleratorW
// TODO: TranslateMDISysAccel
// TODO: TranslateMessage
// TODO: UnhookWindowsHook
// TODO: UnhookWindowsHookEx
// TODO: UnregisterClassA
// TODO: UnregisterClassW
// TODO: UpdateLayeredWindow
// TODO: UpdateLayeredWindowIndirect
// TODO: WaitMessage
// TODO: WindowFromPhysicalPoint
// TODO: WindowFromPoint
// TODO: wsprintfA
// TODO: wsprintfW
// TODO: wvsprintfA
// TODO: wvsprintfW
// TODO: AddDllDirectory
// TODO: BeginUpdateResourceA
// TODO: BeginUpdateResourceW
// TODO: DisableThreadLibraryCalls
// TODO: EndUpdateResourceA
// TODO: EndUpdateResourceW
// TODO: EnumResourceLanguagesA
// TODO: EnumResourceLanguagesExA
// TODO: EnumResourceLanguagesExW
// TODO: EnumResourceLanguagesW
// TODO: EnumResourceNamesA
// TODO: EnumResourceNamesExA
// TODO: EnumResourceNamesExW
// TODO: EnumResourceNamesW
// TODO: EnumResourceTypesA
// TODO: EnumResourceTypesExA
// TODO: EnumResourceTypesExW
// TODO: EnumResourceTypesW
// TODO: FindResourceA
// TODO: FindResourceExA
// TODO: FindResourceExW
// TODO: FindResourceW
// TODO: FreeLibrary
// TODO: FreeLibraryAndExitThread
// TODO: FreeResource
// TODO: GetDllDirectoryA
// TODO: GetDllDirectoryW
// TODO: GetModuleFileNameA
// TODO: GetModuleFileNameW
// TODO: GetModuleHandleA
// TODO: GetModuleHandleExA
// TODO: GetModuleHandleExW
// TODO: GetModuleHandleW
// TODO: GetProcAddress
// TODO: LoadLibraryA
// TODO: LoadLibraryExA
// TODO: LoadLibraryExW
// TODO: LoadLibraryW
// TODO: LoadModule
// TODO: LoadPackagedLibrary
// TODO: LoadResource
// TODO: LockResource
// TODO: RemoveDllDirectory
// TODO: SetDefaultDllDirectories
// TODO: SetDllDirectoryA
// TODO: SetDllDirectoryW
// TODO: SizeofResource
// TODO: UpdateResourceA
// TODO: UpdateResourceW
