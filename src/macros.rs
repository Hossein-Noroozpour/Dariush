#[macro_export]
macro_rules! hr_check {
    ($hr:expr) => {
        if $hr < 0 {
            panic!("HRESULT check failed in file: {} in line: {}", file!(), line!());
        }
    };
}