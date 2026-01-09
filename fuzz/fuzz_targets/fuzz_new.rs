#![no_main]

use libfuzzer_sys::fuzz_target;
use posix_portable_filename::PortableFilename;

fuzz_target!(|data: &str| {
    // Should never panic, regardless of input
    let _ = PortableFilename::new(data);
});
