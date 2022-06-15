
#[repr(transparent)]
#[derive(Copy, Clone)]
struct CreatedHdcWrapper(safer_wingui::CreatedHdc);

impl From<safer_wingui::CreatedHdc> for CreatedHdcWrapper {
    fn from(handle: safer_wingui::CreatedHdc) -> Self {
        Self(handle)
    }
}

impl From<CreatedHdcWrapper> for safer_wingui::CreatedHdc {
    fn from(wrapper: CreatedHdcWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for CreatedHdcWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CreatedHdc <Tag = YawwTag> {
    inner: Object<CreatedHdcWrapper, Tag>,
}

impl<Tag> CreatedHdc<Tag> {
    pub(crate) fn new(inner: Object<CreatedHdcWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HaccelWrapper(safer_wingui::Haccel);

impl From<safer_wingui::Haccel> for HaccelWrapper {
    fn from(handle: safer_wingui::Haccel) -> Self {
        Self(handle)
    }
}

impl From<HaccelWrapper> for safer_wingui::Haccel {
    fn from(wrapper: HaccelWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HaccelWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Haccel <Tag = YawwTag> {
    inner: Object<HaccelWrapper, Tag>,
}

impl<Tag> Haccel<Tag> {
    pub(crate) fn new(inner: Object<HaccelWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HandleWrapper(safer_wingui::Handle);

impl From<safer_wingui::Handle> for HandleWrapper {
    fn from(handle: safer_wingui::Handle) -> Self {
        Self(handle)
    }
}

impl From<HandleWrapper> for safer_wingui::Handle {
    fn from(wrapper: HandleWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HandleWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Handle <Tag = YawwTag> {
    inner: Object<HandleWrapper, Tag>,
}

impl<Tag> Handle<Tag> {
    pub(crate) fn new(inner: Object<HandleWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HbitmapWrapper(safer_wingui::Hbitmap);

impl From<safer_wingui::Hbitmap> for HbitmapWrapper {
    fn from(handle: safer_wingui::Hbitmap) -> Self {
        Self(handle)
    }
}

impl From<HbitmapWrapper> for safer_wingui::Hbitmap {
    fn from(wrapper: HbitmapWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HbitmapWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hbitmap <Tag = YawwTag> {
    inner: Object<HbitmapWrapper, Tag>,
}

impl<Tag> Hbitmap<Tag> {
    pub(crate) fn new(inner: Object<HbitmapWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HbrushWrapper(safer_wingui::Hbrush);

impl From<safer_wingui::Hbrush> for HbrushWrapper {
    fn from(handle: safer_wingui::Hbrush) -> Self {
        Self(handle)
    }
}

impl From<HbrushWrapper> for safer_wingui::Hbrush {
    fn from(wrapper: HbrushWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HbrushWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hbrush <Tag = YawwTag> {
    inner: Object<HbrushWrapper, Tag>,
}

impl<Tag> Hbrush<Tag> {
    pub(crate) fn new(inner: Object<HbrushWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HcursorWrapper(safer_wingui::Hcursor);

impl From<safer_wingui::Hcursor> for HcursorWrapper {
    fn from(handle: safer_wingui::Hcursor) -> Self {
        Self(handle)
    }
}

impl From<HcursorWrapper> for safer_wingui::Hcursor {
    fn from(wrapper: HcursorWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HcursorWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hcursor <Tag = YawwTag> {
    inner: Object<HcursorWrapper, Tag>,
}

impl<Tag> Hcursor<Tag> {
    pub(crate) fn new(inner: Object<HcursorWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HdcWrapper(safer_wingui::Hdc);

impl From<safer_wingui::Hdc> for HdcWrapper {
    fn from(handle: safer_wingui::Hdc) -> Self {
        Self(handle)
    }
}

impl From<HdcWrapper> for safer_wingui::Hdc {
    fn from(wrapper: HdcWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HdcWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hdc <Tag = YawwTag> {
    inner: Object<HdcWrapper, Tag>,
}

impl<Tag> Hdc<Tag> {
    pub(crate) fn new(inner: Object<HdcWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HdpaWrapper(safer_wingui::Hdpa);

impl From<safer_wingui::Hdpa> for HdpaWrapper {
    fn from(handle: safer_wingui::Hdpa) -> Self {
        Self(handle)
    }
}

impl From<HdpaWrapper> for safer_wingui::Hdpa {
    fn from(wrapper: HdpaWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HdpaWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hdpa <Tag = YawwTag> {
    inner: Object<HdpaWrapper, Tag>,
}

impl<Tag> Hdpa<Tag> {
    pub(crate) fn new(inner: Object<HdpaWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HdropWrapper(safer_wingui::Hdrop);

impl From<safer_wingui::Hdrop> for HdropWrapper {
    fn from(handle: safer_wingui::Hdrop) -> Self {
        Self(handle)
    }
}

impl From<HdropWrapper> for safer_wingui::Hdrop {
    fn from(wrapper: HdropWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HdropWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hdrop <Tag = YawwTag> {
    inner: Object<HdropWrapper, Tag>,
}

impl<Tag> Hdrop<Tag> {
    pub(crate) fn new(inner: Object<HdropWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HdsaWrapper(safer_wingui::Hdsa);

impl From<safer_wingui::Hdsa> for HdsaWrapper {
    fn from(handle: safer_wingui::Hdsa) -> Self {
        Self(handle)
    }
}

impl From<HdsaWrapper> for safer_wingui::Hdsa {
    fn from(wrapper: HdsaWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HdsaWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hdsa <Tag = YawwTag> {
    inner: Object<HdsaWrapper, Tag>,
}

impl<Tag> Hdsa<Tag> {
    pub(crate) fn new(inner: Object<HdsaWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HenhmetafileWrapper(safer_wingui::Henhmetafile);

impl From<safer_wingui::Henhmetafile> for HenhmetafileWrapper {
    fn from(handle: safer_wingui::Henhmetafile) -> Self {
        Self(handle)
    }
}

impl From<HenhmetafileWrapper> for safer_wingui::Henhmetafile {
    fn from(wrapper: HenhmetafileWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HenhmetafileWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Henhmetafile <Tag = YawwTag> {
    inner: Object<HenhmetafileWrapper, Tag>,
}

impl<Tag> Henhmetafile<Tag> {
    pub(crate) fn new(inner: Object<HenhmetafileWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HfontWrapper(safer_wingui::Hfont);

impl From<safer_wingui::Hfont> for HfontWrapper {
    fn from(handle: safer_wingui::Hfont) -> Self {
        Self(handle)
    }
}

impl From<HfontWrapper> for safer_wingui::Hfont {
    fn from(wrapper: HfontWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HfontWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hfont <Tag = YawwTag> {
    inner: Object<HfontWrapper, Tag>,
}

impl<Tag> Hfont<Tag> {
    pub(crate) fn new(inner: Object<HfontWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HgdiobjWrapper(safer_wingui::Hgdiobj);

impl From<safer_wingui::Hgdiobj> for HgdiobjWrapper {
    fn from(handle: safer_wingui::Hgdiobj) -> Self {
        Self(handle)
    }
}

impl From<HgdiobjWrapper> for safer_wingui::Hgdiobj {
    fn from(wrapper: HgdiobjWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HgdiobjWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hgdiobj <Tag = YawwTag> {
    inner: Object<HgdiobjWrapper, Tag>,
}

impl<Tag> Hgdiobj<Tag> {
    pub(crate) fn new(inner: Object<HgdiobjWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HhookWrapper(safer_wingui::Hhook);

impl From<safer_wingui::Hhook> for HhookWrapper {
    fn from(handle: safer_wingui::Hhook) -> Self {
        Self(handle)
    }
}

impl From<HhookWrapper> for safer_wingui::Hhook {
    fn from(wrapper: HhookWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HhookWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hhook <Tag = YawwTag> {
    inner: Object<HhookWrapper, Tag>,
}

impl<Tag> Hhook<Tag> {
    pub(crate) fn new(inner: Object<HhookWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HiconWrapper(safer_wingui::Hicon);

impl From<safer_wingui::Hicon> for HiconWrapper {
    fn from(handle: safer_wingui::Hicon) -> Self {
        Self(handle)
    }
}

impl From<HiconWrapper> for safer_wingui::Hicon {
    fn from(wrapper: HiconWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HiconWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hicon <Tag = YawwTag> {
    inner: Object<HiconWrapper, Tag>,
}

impl<Tag> Hicon<Tag> {
    pub(crate) fn new(inner: Object<HiconWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HimagelistWrapper(safer_wingui::Himagelist);

impl From<safer_wingui::Himagelist> for HimagelistWrapper {
    fn from(handle: safer_wingui::Himagelist) -> Self {
        Self(handle)
    }
}

impl From<HimagelistWrapper> for safer_wingui::Himagelist {
    fn from(wrapper: HimagelistWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HimagelistWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Himagelist <Tag = YawwTag> {
    inner: Object<HimagelistWrapper, Tag>,
}

impl<Tag> Himagelist<Tag> {
    pub(crate) fn new(inner: Object<HimagelistWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HinstanceWrapper(safer_wingui::Hinstance);

impl From<safer_wingui::Hinstance> for HinstanceWrapper {
    fn from(handle: safer_wingui::Hinstance) -> Self {
        Self(handle)
    }
}

impl From<HinstanceWrapper> for safer_wingui::Hinstance {
    fn from(wrapper: HinstanceWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HinstanceWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hinstance <Tag = YawwTag> {
    inner: Object<HinstanceWrapper, Tag>,
}

impl<Tag> Hinstance<Tag> {
    pub(crate) fn new(inner: Object<HinstanceWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HkeyWrapper(safer_wingui::Hkey);

impl From<safer_wingui::Hkey> for HkeyWrapper {
    fn from(handle: safer_wingui::Hkey) -> Self {
        Self(handle)
    }
}

impl From<HkeyWrapper> for safer_wingui::Hkey {
    fn from(wrapper: HkeyWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HkeyWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hkey <Tag = YawwTag> {
    inner: Object<HkeyWrapper, Tag>,
}

impl<Tag> Hkey<Tag> {
    pub(crate) fn new(inner: Object<HkeyWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HmenuWrapper(safer_wingui::Hmenu);

impl From<safer_wingui::Hmenu> for HmenuWrapper {
    fn from(handle: safer_wingui::Hmenu) -> Self {
        Self(handle)
    }
}

impl From<HmenuWrapper> for safer_wingui::Hmenu {
    fn from(wrapper: HmenuWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HmenuWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hmenu <Tag = YawwTag> {
    inner: Object<HmenuWrapper, Tag>,
}

impl<Tag> Hmenu<Tag> {
    pub(crate) fn new(inner: Object<HmenuWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HmetafileWrapper(safer_wingui::Hmetafile);

impl From<safer_wingui::Hmetafile> for HmetafileWrapper {
    fn from(handle: safer_wingui::Hmetafile) -> Self {
        Self(handle)
    }
}

impl From<HmetafileWrapper> for safer_wingui::Hmetafile {
    fn from(wrapper: HmetafileWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HmetafileWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hmetafile <Tag = YawwTag> {
    inner: Object<HmetafileWrapper, Tag>,
}

impl<Tag> Hmetafile<Tag> {
    pub(crate) fn new(inner: Object<HmetafileWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HmonitorWrapper(safer_wingui::Hmonitor);

impl From<safer_wingui::Hmonitor> for HmonitorWrapper {
    fn from(handle: safer_wingui::Hmonitor) -> Self {
        Self(handle)
    }
}

impl From<HmonitorWrapper> for safer_wingui::Hmonitor {
    fn from(wrapper: HmonitorWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HmonitorWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hmonitor <Tag = YawwTag> {
    inner: Object<HmonitorWrapper, Tag>,
}

impl<Tag> Hmonitor<Tag> {
    pub(crate) fn new(inner: Object<HmonitorWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HpaletteWrapper(safer_wingui::Hpalette);

impl From<safer_wingui::Hpalette> for HpaletteWrapper {
    fn from(handle: safer_wingui::Hpalette) -> Self {
        Self(handle)
    }
}

impl From<HpaletteWrapper> for safer_wingui::Hpalette {
    fn from(wrapper: HpaletteWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HpaletteWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hpalette <Tag = YawwTag> {
    inner: Object<HpaletteWrapper, Tag>,
}

impl<Tag> Hpalette<Tag> {
    pub(crate) fn new(inner: Object<HpaletteWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HpenWrapper(safer_wingui::Hpen);

impl From<safer_wingui::Hpen> for HpenWrapper {
    fn from(handle: safer_wingui::Hpen) -> Self {
        Self(handle)
    }
}

impl From<HpenWrapper> for safer_wingui::Hpen {
    fn from(wrapper: HpenWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HpenWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hpen <Tag = YawwTag> {
    inner: Object<HpenWrapper, Tag>,
}

impl<Tag> Hpen<Tag> {
    pub(crate) fn new(inner: Object<HpenWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HpropsheetpageWrapper(safer_wingui::Hpropsheetpage);

impl From<safer_wingui::Hpropsheetpage> for HpropsheetpageWrapper {
    fn from(handle: safer_wingui::Hpropsheetpage) -> Self {
        Self(handle)
    }
}

impl From<HpropsheetpageWrapper> for safer_wingui::Hpropsheetpage {
    fn from(wrapper: HpropsheetpageWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HpropsheetpageWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hpropsheetpage <Tag = YawwTag> {
    inner: Object<HpropsheetpageWrapper, Tag>,
}

impl<Tag> Hpropsheetpage<Tag> {
    pub(crate) fn new(inner: Object<HpropsheetpageWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HpsxaWrapper(safer_wingui::Hpsxa);

impl From<safer_wingui::Hpsxa> for HpsxaWrapper {
    fn from(handle: safer_wingui::Hpsxa) -> Self {
        Self(handle)
    }
}

impl From<HpsxaWrapper> for safer_wingui::Hpsxa {
    fn from(wrapper: HpsxaWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HpsxaWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hpsxa <Tag = YawwTag> {
    inner: Object<HpsxaWrapper, Tag>,
}

impl<Tag> Hpsxa<Tag> {
    pub(crate) fn new(inner: Object<HpsxaWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HrgnWrapper(safer_wingui::Hrgn);

impl From<safer_wingui::Hrgn> for HrgnWrapper {
    fn from(handle: safer_wingui::Hrgn) -> Self {
        Self(handle)
    }
}

impl From<HrgnWrapper> for safer_wingui::Hrgn {
    fn from(wrapper: HrgnWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HrgnWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hrgn <Tag = YawwTag> {
    inner: Object<HrgnWrapper, Tag>,
}

impl<Tag> Hrgn<Tag> {
    pub(crate) fn new(inner: Object<HrgnWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HrsrcWrapper(safer_wingui::Hrsrc);

impl From<safer_wingui::Hrsrc> for HrsrcWrapper {
    fn from(handle: safer_wingui::Hrsrc) -> Self {
        Self(handle)
    }
}

impl From<HrsrcWrapper> for safer_wingui::Hrsrc {
    fn from(wrapper: HrsrcWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HrsrcWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hrsrc <Tag = YawwTag> {
    inner: Object<HrsrcWrapper, Tag>,
}

impl<Tag> Hrsrc<Tag> {
    pub(crate) fn new(inner: Object<HrsrcWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HsyntheticpointerdeviceWrapper(safer_wingui::Hsyntheticpointerdevice);

impl From<safer_wingui::Hsyntheticpointerdevice> for HsyntheticpointerdeviceWrapper {
    fn from(handle: safer_wingui::Hsyntheticpointerdevice) -> Self {
        Self(handle)
    }
}

impl From<HsyntheticpointerdeviceWrapper> for safer_wingui::Hsyntheticpointerdevice {
    fn from(wrapper: HsyntheticpointerdeviceWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HsyntheticpointerdeviceWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hsyntheticpointerdevice <Tag = YawwTag> {
    inner: Object<HsyntheticpointerdeviceWrapper, Tag>,
}

impl<Tag> Hsyntheticpointerdevice<Tag> {
    pub(crate) fn new(inner: Object<HsyntheticpointerdeviceWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HtreeitemWrapper(safer_wingui::Htreeitem);

impl From<safer_wingui::Htreeitem> for HtreeitemWrapper {
    fn from(handle: safer_wingui::Htreeitem) -> Self {
        Self(handle)
    }
}

impl From<HtreeitemWrapper> for safer_wingui::Htreeitem {
    fn from(wrapper: HtreeitemWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HtreeitemWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Htreeitem <Tag = YawwTag> {
    inner: Object<HtreeitemWrapper, Tag>,
}

impl<Tag> Htreeitem<Tag> {
    pub(crate) fn new(inner: Object<HtreeitemWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HwndWrapper(safer_wingui::Hwnd);

impl From<safer_wingui::Hwnd> for HwndWrapper {
    fn from(handle: safer_wingui::Hwnd) -> Self {
        Self(handle)
    }
}

impl From<HwndWrapper> for safer_wingui::Hwnd {
    fn from(wrapper: HwndWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HwndWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Hwnd <Tag = YawwTag> {
    inner: Object<HwndWrapper, Tag>,
}

impl<Tag> Hwnd<Tag> {
    pub(crate) fn new(inner: Object<HwndWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HdcMetdataEnhFileHandleWrapper(safer_wingui::HdcMetdataEnhFileHandle);

impl From<safer_wingui::HdcMetdataEnhFileHandle> for HdcMetdataEnhFileHandleWrapper {
    fn from(handle: safer_wingui::HdcMetdataEnhFileHandle) -> Self {
        Self(handle)
    }
}

impl From<HdcMetdataEnhFileHandleWrapper> for safer_wingui::HdcMetdataEnhFileHandle {
    fn from(wrapper: HdcMetdataEnhFileHandleWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HdcMetdataEnhFileHandleWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct HdcMetdataEnhFileHandle <Tag = YawwTag> {
    inner: Object<HdcMetdataEnhFileHandleWrapper, Tag>,
}

impl<Tag> HdcMetdataEnhFileHandle<Tag> {
    pub(crate) fn new(inner: Object<HdcMetdataEnhFileHandleWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct HdcMetdataFileHandleWrapper(safer_wingui::HdcMetdataFileHandle);

impl From<safer_wingui::HdcMetdataFileHandle> for HdcMetdataFileHandleWrapper {
    fn from(handle: safer_wingui::HdcMetdataFileHandle) -> Self {
        Self(handle)
    }
}

impl From<HdcMetdataFileHandleWrapper> for safer_wingui::HdcMetdataFileHandle {
    fn from(wrapper: HdcMetdataFileHandleWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for HdcMetdataFileHandleWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct HdcMetdataFileHandle <Tag = YawwTag> {
    inner: Object<HdcMetdataFileHandleWrapper, Tag>,
}

impl<Tag> HdcMetdataFileHandle<Tag> {
    pub(crate) fn new(inner: Object<HdcMetdataFileHandleWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct LparamWrapper(safer_wingui::Lparam);

impl From<safer_wingui::Lparam> for LparamWrapper {
    fn from(handle: safer_wingui::Lparam) -> Self {
        Self(handle)
    }
}

impl From<LparamWrapper> for safer_wingui::Lparam {
    fn from(wrapper: LparamWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for LparamWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Lparam <Tag = YawwTag> {
    inner: Object<LparamWrapper, Tag>,
}

impl<Tag> Lparam<Tag> {
    pub(crate) fn new(inner: Object<LparamWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct LresultWrapper(safer_wingui::Lresult);

impl From<safer_wingui::Lresult> for LresultWrapper {
    fn from(handle: safer_wingui::Lresult) -> Self {
        Self(handle)
    }
}

impl From<LresultWrapper> for safer_wingui::Lresult {
    fn from(wrapper: LresultWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for LresultWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Lresult <Tag = YawwTag> {
    inner: Object<LresultWrapper, Tag>,
}

impl<Tag> Lresult<Tag> {
    pub(crate) fn new(inner: Object<LresultWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct ShandlePtrWrapper(safer_wingui::ShandlePtr);

impl From<safer_wingui::ShandlePtr> for ShandlePtrWrapper {
    fn from(handle: safer_wingui::ShandlePtr) -> Self {
        Self(handle)
    }
}

impl From<ShandlePtrWrapper> for safer_wingui::ShandlePtr {
    fn from(wrapper: ShandlePtrWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for ShandlePtrWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ShandlePtr <Tag = YawwTag> {
    inner: Object<ShandlePtrWrapper, Tag>,
}

impl<Tag> ShandlePtr<Tag> {
    pub(crate) fn new(inner: Object<ShandlePtrWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    

#[repr(transparent)]
#[derive(Copy, Clone)]
struct ShFindChangeNotificationHandleWrapper(safer_wingui::ShFindChangeNotificationHandle);

impl From<safer_wingui::ShFindChangeNotificationHandle> for ShFindChangeNotificationHandleWrapper {
    fn from(handle: safer_wingui::ShFindChangeNotificationHandle) -> Self {
        Self(handle)
    }
}

impl From<ShFindChangeNotificationHandleWrapper> for safer_wingui::ShFindChangeNotificationHandle {
    fn from(wrapper: ShFindChangeNotificationHandleWrapper) -> Self {
        wrapper.0
    }
} 

unsafe impl Compatible for ShFindChangeNotificationHandleWrapper {
    fn representative(&self) -> usize {
        self.into_raw() as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ShFindChangeNotificationHandle <Tag = YawwTag> {
    inner: Object<ShFindChangeNotificationHandleWrapper, Tag>,
}

impl<Tag> ShFindChangeNotificationHandle<Tag> {
    pub(crate) fn new(inner: Object<ShFindChangeNotificationHandleWrapper, Tag>) -> Self {
        Self {
            inner,
        }
    }
}
                    
