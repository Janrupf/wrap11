use crate::{xlib_sys, XAtom};
use std::ops::Deref;

/// Describes the possible format of a X11 property.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum XPropertyDataFormat {
    /// One property element is 8 bits long
    Bit8,

    /// One property element is 16 bits long
    Bit16,

    /// One property element is 32 bits long
    Bit32,
}

impl XPropertyDataFormat {
    /// Attempts to convert the format from the X11 native representation.
    ///
    /// # Arguments
    ///
    /// * `format` - The native format, must be one of 8, 16 or 32
    pub fn from_native(format: i32) -> Option<Self> {
        match format {
            8 => Some(XPropertyDataFormat::Bit8),
            16 => Some(XPropertyDataFormat::Bit16),
            32 => Some(XPropertyDataFormat::Bit32),
            _ => None,
        }
    }

    /// Converts this format to the native representation.
    pub fn to_native(&self) -> i32 {
        match self {
            XPropertyDataFormat::Bit8 => 8,
            XPropertyDataFormat::Bit16 => 16,
            XPropertyDataFormat::Bit32 => 32,
        }
    }

    /// Returns the amount of bytes per property.
    pub fn byte_count(&self) -> usize {
        match self {
            XPropertyDataFormat::Bit8 => 1,
            XPropertyDataFormat::Bit16 => 2,
            XPropertyDataFormat::Bit32 => 4,
        }
    }

    /// Returns the amount of bytes for a property array.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the array
    pub fn byte_count_array(&self, length: usize) -> usize {
        self.byte_count() * length
    }
}

/// Represents data held by a property.
#[derive(Debug)]
pub struct XPropertyData<'a> {
    format: XPropertyDataFormat,
    actual_type: XAtom<'a>,
    item_count: usize,
    data: *mut u8,
}

impl<'a> XPropertyData<'a> {
    /// Wraps native property data.
    ///
    /// # Arguments
    ///
    /// * `format` - The format of the data
    /// * `actual_type` - The actual type of the data as reported by the X server
    /// * `item_count` - The amount of properties stored in the data
    /// * `data` - A pointer to the beginning of the stored data
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(
        format: XPropertyDataFormat,
        actual_type: XAtom<'a>,
        item_count: usize,
        data: *mut u8,
    ) -> Self {
        Self {
            format,
            actual_type,
            item_count,
            data,
        }
    }

    /// Retrieves the format of the property elements.
    pub fn format(&self) -> XPropertyDataFormat {
        self.format
    }

    /// Retrieves the type of the property elements as reported by the X server.
    pub fn ty(&self) -> XAtom<'a> {
        self.actual_type
    }

    /// Retrieves the amount of properties in the data.
    pub fn length(&self) -> usize {
        self.item_count
    }

    /// Retrieves the size of the entire data in bytes.
    pub fn byte_size(&self) -> usize {
        self.format.byte_count_array(self.item_count)
    }

    /// Retrieves the data as a slice.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data, self.byte_size()) }
    }

    /// Interprets the data as a pointer of a specific type.
    ///
    /// # Panics
    ///
    /// If the size of the stored data is smaller than the size of the requested type.
    pub fn get_as_ptr<T>(&self) -> *const T {
        assert!(self.byte_size() >= std::mem::size_of::<T>());

        self.data as _
    }

    /// Interprets the data as a mutable pointer of a specific type.
    ///
    /// # Panics
    ///
    /// If the size of the stored data is smaller than the size of the requested type.
    pub fn get_as_mut_ptr<T>(&self) -> *mut T {
        assert!(self.byte_size() >= std::mem::size_of::<T>());

        self.data as _
    }

    /// Interprets the data as a reference of a specific type.
    ///
    /// # Panics
    ///
    /// If the size of the stored data is smaller than the size of the requested type.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that the underlying data is valid for the requested type.
    pub unsafe fn get_as_ref<T>(&self) -> &T {
        &*self.get_as_ptr::<T>()
    }

    /// Interprets the data as a mutable reference of a specific type.
    ///
    /// # Panics
    ///
    /// If the size of the stored data is smaller than the size of the requested type.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that the underlying data is valid for the requested type.
    pub unsafe fn get_as_mut_ref<T>(&mut self) -> &mut T {
        &mut *self.get_as_mut_ptr::<T>()
    }
}

impl<'a> Deref for XPropertyData<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a> AsRef<[u8]> for XPropertyData<'a> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> Drop for XPropertyData<'a> {
    fn drop(&mut self) {
        unsafe { xlib_sys::XFree(self.data as _) };
    }
}

/// Describes how the change of a window property is performed.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum XPropertyChangeMode {
    Replace = xlib_sys::PropModeReplace,
    Prepend = xlib_sys::PropModePrepend,
    Append = xlib_sys::PropModeAppend,
}

/// Implemented by types which can hold properties.
pub trait XPropertyHolder {
    /// Attempts to retrieve a property.
    ///
    /// This functions returns (if available) the read data and amount of remaining bytes.
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `offset` - The byte offset into the property to start reading at
    /// * `length` - The maximal amount of bytes to read
    /// * `delete` - Whether the property should be deleted upon retrieval
    /// * `ty` - The X atom identifying the expected type of the property
    fn get_property(
        &self,
        property: XAtom,
        offset: i64,
        length: i64,
        delete: bool,
        ty: XAtom,
    ) -> Option<(XPropertyData, usize)>;

    /// Changes a property,
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `ty` - The X atom identifying the property type
    /// * `format` - The format of the property
    /// * `mode` - How the property should be changed
    /// * `data` - The data to work with (interpretation depends on `mode`)
    /// * `element_count` - The amount of elements stored in `data`
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    unsafe fn change_property_unsafe(
        &self,
        property: XAtom,
        ty: XAtom,
        format: XPropertyDataFormat,
        mode: XPropertyChangeMode,
        data: *mut u8,
        element_count: usize,
    );

    /// Deletes a property if it exists from the window.
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    fn delete_property(&self, property: XAtom);

    /// Changes a property in 8 bit format,
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `ty` - The X atom identifying the property type
    /// * `mode` - How the property should be changed
    /// * `data` - The data to work with (interpretation depends on `mode`)
    fn change_property8(&self, property: XAtom, ty: XAtom, mode: XPropertyChangeMode, data: &[u8]) {
        // XChangeProperty never writes to data, but it is not defined as const in C
        #[allow(mutable_transmutes)]
        let data = unsafe { std::mem::transmute::<_, &mut [u8]>(data) };
        let element_count = data.len();

        unsafe {
            self.change_property_unsafe(
                property,
                ty,
                XPropertyDataFormat::Bit8,
                mode,
                data.as_mut_ptr(),
                element_count,
            )
        };
    }

    /// Changes a property in 16 bit format,
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `ty` - The X atom identifying the property type
    /// * `mode` - How the property should be changed
    /// * `data` - The data to work with (interpretation depends on `mode`)
    fn change_property16(
        &self,
        property: XAtom,
        ty: XAtom,
        mode: XPropertyChangeMode,
        data: &[i16],
    ) {
        // XChangeProperty never writes to data, but it is not defined as const in C
        #[allow(mutable_transmutes)]
        let data = unsafe { std::mem::transmute::<_, &mut [i16]>(data) };
        let element_count = data.len();

        unsafe {
            self.change_property_unsafe(
                property,
                ty,
                XPropertyDataFormat::Bit16,
                mode,
                data.as_mut_ptr() as _,
                element_count,
            )
        };
    }

    /// Changes a property in 32 bit format,
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `ty` - The X atom identifying the property type
    /// * `mode` - How the property should be changed
    /// * `data` - The data to work with (interpretation depends on `mode`)
    fn change_property32(
        &self,
        property: XAtom,
        ty: XAtom,
        mode: XPropertyChangeMode,
        data: &[i32],
    ) {
        // XChangeProperty never writes to data, but it is not defined as const in C
        #[allow(mutable_transmutes)]
        let data = unsafe { std::mem::transmute::<_, &mut [i32]>(data) };
        let element_count = data.len();

        unsafe {
            self.change_property_unsafe(
                property,
                ty,
                XPropertyDataFormat::Bit32,
                mode,
                data.as_mut_ptr() as _,
                element_count,
            )
        };
    }

    /// Reads an X11 property completely be automatically determining its length.
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `delete` - Whether the property should be deleted upon retrieval
    /// * `ty` - The X atom identifying the property type
    fn get_property_completely(
        &self,
        property: XAtom,
        delete: bool,
        ty: XAtom,
    ) -> Option<XPropertyData> {
        let (data, remaining) = self.get_property(property, 0, 0, false, ty)?;

        if remaining < 1 {
            // Short circuit: property has 0 length
            return Some(data);
        }

        let (data, _) = self.get_property(property, 0, (remaining / 4) as _, delete, ty)?;
        Some(data)
    }
}
