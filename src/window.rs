// MIT/Apache2 License

use crate::Key;
use winapi::{ctypes::c_int, shared::minwindef::DWORD, um::winuser};

pub type Window = Key;

bitflags::bitflags! {
    #[doc = "Extended window style"]
    pub struct ExtendedWindowStyle : DWORD {
        const ACCEPT_FILES = winuser::WS_EX_ACCEPTFILES;
        const APP_WINDOW = winuser::WS_EX_APPWINDOW;
        const CLIENT_EDGE = winuser::WS_EX_CLIENTEDGE;
        const COMPOSITED = winuser::WS_EX_COMPOSITED;
        const CONTEXT_HELP = winuser::WS_EX_CONTEXTHELP;
        const CONTROL_PARENT = winuser::WS_EX_CONTROLPARENT;
        const DLG_MODAL_FRAME = winuser::WS_EX_DLGMODALFRAME;
        const LAYERED = winuser::WS_EX_LAYERED;
        const LAYOUT_RTL = winuser::WS_EX_LAYOUTRTL;
        const LEFT = winuser::WS_EX_LEFT;
        const LEFT_SCROLL_BAR = winuser::WS_EX_LEFTSCROLLBAR;
        const LTR_READING = winuser::WS_EX_LTRREADING;
        const MDI_CHILD = winuser::WS_EX_MDICHILD;
        const NO_ACTIVATE = winuser::WS_EX_NOACTIVATE;
        const NO_INHERIT_LAYOUT = winuser::WS_EX_NOINHERITLAYOUT;
        const NO_PARENT_NOTIFY = winuser::WS_EX_NOPARENTNOTIFY;
        const NO_REDIRECTION_BITMAP = winuser::WS_EX_NOREDIRECTIONBITMAP;
        const OVERLAPPED_WINDOW = winuser::WS_EX_OVERLAPPEDWINDOW;
        const PALETTE_WINDOW = winuser::WS_EX_PALETTEWINDOW;
        const RIGHT = winuser::WS_EX_RIGHT;
        const RIGHT_SCROLL_BAR = winuser::WS_EX_RIGHTSCROLLBAR;
        const RTL_READING = winuser::WS_EX_RTLREADING;
        const STATIC_EDGE = winuser::WS_EX_STATICEDGE;
        const TOOL_WINDOW = winuser::WS_EX_TOOLWINDOW;
        const TOP_MOST = winuser::WS_EX_TOPMOST;
        const TRANSPARENT = winuser::WS_EX_TRANSPARENT;
        const WINDOW_EDGE = winuser::WS_EX_WINDOWEDGE;
    }
}


bitflags::bitflags! {
    #[doc = "Window style"]
    pub struct WindowStyle : DWORD {
        const BORDER = winuser::WS_BORDER;
        const CAPTION = winuser::WS_CAPTION;
        const CHILD = winuser::WS_CHILD;
        const CHILD_WINDOW = winuser::WS_CHILDWINDOW;
        const CLIP_CHILDREN = winuser::WS_CLIPCHILDREN;
        const CLIP_SIBLINGS = winuser::WS_CLIPSIBLINGS;
        const DISABLED = winuser::WS_DISABLED;
        const DLG_FRAME = winuser::WS_DLGFRAME;
        const GROUP = winuser::WS_GROUP;
        const H_SCROLL = winuser::WS_HSCROLL;
        const ICONIC = winuser::WS_ICONIC;
        const MAXIMIZE = winuser::WS_MAXIMIZE;
        const MAXIMIZE_BOX = winuser::WS_MAXIMIZEBOX;
        const MINIMIZE = winuser::WS_MINIMIZE;
        const MINMIZE_BOX = winuser::WS_MINIMIZEBOX;
        const OVERLAPPED = winuser::WS_OVERLAPPED;
        const OVERLAPPED_WINDOW = winuser::WS_OVERLAPPEDWINDOW;
        const POPUP = winuser::WS_POPUP;
        const POPUP_WINDOW = winuser::WS_POPUPWINDOW;
        const SIZE_BOX = winuser::WS_SIZEBOX;
        const SYS_MENU = winuser::WS_SYSMENU;
        const TAB_SETOP = winuser::WS_TABSTOP;
        const THICK_FRAME = winuser::WS_THICKFRAME;
        const TILED = winuser::WS_TILED;
        const TILED_WINDOW = winuser::WS_TILEDWINDOW;
        const VISIBLE = winuser::WS_VISIBLE;
        const V_SCROLL = winuser::WS_VSCROLL;
    }
}

bitflags::bitflags! {
    #[doc = "Commands for show_window()"]
    pub struct ShowWindowCommand : c_int {
        const FORCE_MINIMIZE = winuser::SW_FORCEMINIMIZE;
        const HIDE = winuser::SW_HIDE;
        const MAXIMIZE = winuser::SW_MAXIMIZE;
        const MINIMIZE = winuser::SW_MINIMIZE;
        const RESOTRE = winuser::SW_RESTORE;
        const SHOW = winuser::SW_SHOW;
        const SHOW_DEFAULT = winuser::SW_SHOWDEFAULT;
        const SHOW_MAXIMIZED = winuser::SW_SHOWMAXIMIZED;
        const SHOW_MINIMIZED = winuser::SW_SHOWMINIMIZED;
        const SHOW_MIN_NO_ACTIVE = winuser::SW_SHOWMINNOACTIVE;
        const SHOW_NA = winuser::SW_SHOWNA;
        const SHOW_NO_ACTIVATE = winuser::SW_SHOWNOACTIVATE;
        const SHOW_NORMAL = winuser::SW_SHOWNORMAL;
    }
}
