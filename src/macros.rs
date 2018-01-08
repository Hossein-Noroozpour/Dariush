#[macro_export]
macro_rules! hr_check {
    ($hr:expr) => {
        unsafe {
            let hr = $hr;
            if hr < 0 {
                panic!("HRESULT: {} check failed in file: {} in line: {}", hr, file!(), line!());
            }
        }
    };
}
