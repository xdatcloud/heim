use std::mem;

use winapi::um::sysinfoapi;
use winapi::shared::{ntdef, minwindef};

use crate::units;

pub mod power;
pub mod winternl;

const HI_T: f64 = 429.496_729_6;
const LO_T: f64 = 1e-7;

pub unsafe fn get_system_info() -> sysinfoapi::SYSTEM_INFO {
    let mut info: sysinfoapi::SYSTEM_INFO = mem::uninitialized();
    sysinfoapi::GetSystemInfo(&mut info);

    info
}

pub trait IntoTime {
    fn into_time(self) -> units::Time;
}

impl IntoTime for minwindef::FILETIME {
    #[inline]
    fn into_time(self) -> units::Time {
        let value = (HI_T * f64::from(self.dwHighDateTime))
            + (LO_T * f64::from(self.dwLowDateTime));

        units::Time::new::<units::second>(value)
    }
}

impl IntoTime for ntdef::LARGE_INTEGER {
    #[inline]
    fn into_time(self) -> units::Time {
        let s = unsafe { self.s() };
        let value = (HI_T * f64::from(s.HighPart))
            + (LO_T * f64::from(s.LowPart));

        units::Time::new::<units::second>(value)
    }
}