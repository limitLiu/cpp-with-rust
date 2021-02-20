#![allow(non_camel_case_types)]

#[cfg(target_os = "macos")]
#[repr(C)]
pub struct MachTimebaseInfo {
  numer: u32,
  denom: u32,
}

type mach_timebase_info_t = *const MachTimebaseInfo;
type mach_timebase_data_info_t = MachTimebaseInfo;

extern {
  fn mach_absolute_time() -> u64;
  fn mach_timebase_info(info: mach_timebase_info_t) -> i32;
}

lazy_static! {
  static ref TIMER: Timer = unsafe { Timer::new() };
}

struct Timer {
  offset: u64,
  frequency: u64,
  initialized: bool,
}

impl Timer {
  pub unsafe fn new() -> Self {
    let info: mach_timebase_data_info_t = MachTimebaseInfo { numer: 0, denom: 0 };
    mach_timebase_info(&info);

    Self {
      offset: mach_absolute_time(),
      frequency: (info.denom as f64 * 1e9 / info.numer as f64) as u64,
      initialized: true,
    }
  }
}

#[no_mangle]
extern "C" fn get_time() -> f64 {
  if !TIMER.initialized {
    return 0.0;
  }
  return (unsafe { mach_absolute_time() } as f64 - TIMER.offset as f64) / TIMER.frequency as f64;
}

