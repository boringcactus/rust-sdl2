/* automatically generated by rust-bindgen */

pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type Uint8 = u8;
pub type Uint32 = u32;
extern "C" {
    pub fn rotozoomSurface(
        src: *mut SDL_Surface,
        angle: f64,
        zoom: f64,
        smooth: ::std::os::raw::c_int,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn rotozoomSurfaceXY(
        src: *mut SDL_Surface,
        angle: f64,
        zoomx: f64,
        zoomy: f64,
        smooth: ::std::os::raw::c_int,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn rotozoomSurfaceSize(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        angle: f64,
        zoom: f64,
        dstwidth: *mut ::std::os::raw::c_int,
        dstheight: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rotozoomSurfaceSizeXY(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        angle: f64,
        zoomx: f64,
        zoomy: f64,
        dstwidth: *mut ::std::os::raw::c_int,
        dstheight: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn zoomSurface(
        src: *mut SDL_Surface,
        zoomx: f64,
        zoomy: f64,
        smooth: ::std::os::raw::c_int,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn zoomSurfaceSize(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        zoomx: f64,
        zoomy: f64,
        dstwidth: *mut ::std::os::raw::c_int,
        dstheight: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn shrinkSurface(
        src: *mut SDL_Surface,
        factorx: ::std::os::raw::c_int,
        factory: ::std::os::raw::c_int,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn rotateSurface90Degrees(
        src: *mut SDL_Surface,
        numClockwiseTurns: ::std::os::raw::c_int,
    ) -> *mut SDL_Surface;
}
