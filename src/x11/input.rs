use crate::{
    xinput2_sys, XAtom, XDisplay, XPropertyChangeMode, XPropertyData, XPropertyDataFormat,
    XPropertyHolder,
};
use x11::xinput2;

#[derive(Debug, Clone)]
pub struct XInputDevice<'a> {
    display: &'a XDisplay,
    id: i32,
}

impl<'a> XInputDevice<'a> {
    /// Wraps an existing XInput device.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the device to wrap
    /// * `display` - The display the device belongs to
    pub fn from_id(id: i32, display: &'a XDisplay) -> Self {
        Self { id, display }
    }

    /// Creates an XInput device which represents all devices.
    ///
    /// # Arguments
    ///
    /// * `display` - The display the device belongs to
    pub fn all(display: &'a XDisplay) -> Self {
        Self {
            id: xinput2::XIAllDevices,
            display,
        }
    }

    /// Creates an XInput device which represents all master devices.
    ///
    /// # Arguments
    ///
    /// * `display` - The display the device belongs to
    pub fn all_master(display: &'a XDisplay) -> Self {
        Self {
            id: xinput2::XIAllMasterDevices,
            display,
        }
    }

    /// Retrieves the id of the device.
    pub fn id(&self) -> i32 {
        self.id
    }
}

impl<'a> XPropertyHolder for XInputDevice<'a> {
    fn get_property(
        &self,
        property: XAtom,
        offset: i64,
        length: i64,
        delete: bool,
        ty: XAtom,
    ) -> Option<(XPropertyData, usize)> {
        let mut actual_type = 0;
        let mut actual_format = 0;
        let mut item_count = 0;
        let mut remaining_bytes = 0;
        let mut data = std::ptr::null_mut();

        let delete = i32::from(delete);

        unsafe {
            xinput2_sys::XIGetProperty(
                self.display.handle(),
                self.id,
                property.handle(),
                offset,
                length,
                delete,
                ty.handle(),
                &mut actual_type,
                &mut actual_format,
                &mut item_count,
                &mut remaining_bytes,
                &mut data,
            )
        };

        XPropertyDataFormat::from_native(actual_format).map(|format| {
            let actual_type = unsafe { XAtom::new(actual_type, self.display) };
            let data = unsafe { XPropertyData::new(format, actual_type, item_count as _, data) };

            (data, remaining_bytes as _)
        })
    }

    unsafe fn change_property_unsafe(
        &self,
        property: XAtom,
        ty: XAtom,
        format: XPropertyDataFormat,
        mode: XPropertyChangeMode,
        data: *mut u8,
        element_count: usize,
    ) {
        xinput2_sys::XIChangeProperty(
            self.display.handle(),
            self.id,
            property.handle(),
            ty.handle(),
            format.to_native(),
            mode as _,
            data,
            element_count as _,
        );
    }

    fn delete_property(&self, property: XAtom) {
        unsafe { xinput2_sys::XIDeleteProperty(self.display.handle(), self.id, property.handle()) };
    }
}
