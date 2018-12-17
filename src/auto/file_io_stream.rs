// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use FileInfo;
use IOStream;
use Seekable;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct FileIOStream(Object<ffi::GFileIOStream, ffi::GFileIOStreamClass>): IOStream, Seekable;

    match fn {
        get_type => || ffi::g_file_io_stream_get_type(),
    }
}

pub trait FileIOStreamExt: 'static {
    fn get_etag(&self) -> Option<GString>;

    fn query_info<'a, P: Into<Option<&'a Cancellable>>>(&self, attributes: &str, cancellable: P) -> Result<FileInfo, Error>;

    fn query_info_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, io_priority: glib::Priority, cancellable: P, callback: Q);

    #[cfg(feature = "futures")]
    fn query_info_async_future(&self, attributes: &str, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, FileInfo), Error = (Self, Error)>> where Self: Sized + Clone;
}

impl<O: IsA<FileIOStream>> FileIOStreamExt for O {
    fn get_etag(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::g_file_io_stream_get_etag(self.to_glib_none().0))
        }
    }

    fn query_info<'a, P: Into<Option<&'a Cancellable>>>(&self, attributes: &str, cancellable: P) -> Result<FileInfo, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_io_stream_query_info(self.to_glib_none().0, attributes.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn query_info_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, io_priority: glib::Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn query_info_async_trampoline<Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_io_stream_query_info_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = query_info_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_io_stream_query_info_async(self.to_glib_none().0, attributes.to_glib_none().0, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn query_info_async_future(&self, attributes: &str, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, FileInfo), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let attributes = String::from(attributes);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.query_info_async(
                 &attributes,
                 io_priority,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }
}

impl fmt::Display for FileIOStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileIOStream")
    }
}
