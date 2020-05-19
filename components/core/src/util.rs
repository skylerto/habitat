pub mod docker;
#[cfg(not(windows))]
pub mod posix_perm;
pub mod serde_string;
pub mod sys;
#[cfg(windows)]
pub mod win_perm;

use std::mem;

/// Same as `Result::ok()`, but logs the error case. Useful for
/// ignoring error cases, while still leaving a paper trail.
#[macro_export]
macro_rules! ok_log {
    ($result:expr) => {
        match $result {
            Ok(val) => Some(val),
            Err(e) => {
                warn!("Intentionally ignored error ({}:{}): {:?}",
                      file!(),
                      line!(),
                      e);
                None
            }
        }
    };
}

/// returns the common arguments to pass to pwsh.exe when spawning a powershell instance.
/// These arguments are optimized for a background powershell process running hooks.
/// The NonInteractive flag specifies that the console is not intended to interact with
/// human input and allows ctrl+break signals to trigger a graceful termination similar to
/// a SIGTERM on linux rather than an interactive debugging prompt. The ExecutionPolicy
/// ensures that if a more strict policy exists in the Windows Registry (ex "AllSigned"),
/// hook execution will not fail because hook scripts are never signed. RemoteSigned is the
/// default policy and just requires remote scripts to be signeed. Supervisor hooks are
/// always local so "RemoteSigned" does not interfere with supervisor behavior.
#[cfg(windows)]
pub fn pwsh_args(command: &str) -> Vec<&str> {
    vec!["-NonInteractive",
         "-ExecutionPolicy",
         "RemoteSigned",
         "-Command",
         command]
}

/// Provide a way to convert numeric types safely to i64
pub trait ToI64 {
    fn to_i64(self) -> i64;
}

impl ToI64 for usize {
    fn to_i64(self) -> i64 {
        if mem::size_of::<usize>() >= mem::size_of::<i64>() && self > i64::max_value() as usize {
            if cfg!(debug_assertions) {
                panic!("Tried to convert an out-of-range usize ({}) to i64", self);
            } else {
                error!("Tried to convert an out-of-range usize ({}) to i64; using \
                        i64::max_value()",
                       self);
                i64::max_value()
            }
        } else {
            self as i64
        }
    }
}

impl ToI64 for u64 {
    fn to_i64(self) -> i64 {
        if self > i64::max_value() as u64 {
            if cfg!(debug_assertions) {
                panic!("Tried to convert an out-of-range u64 ({}) to i64", self);
            } else {
                error!("Tried to convert an out-of-range u64 ({}) to i64; using i64::max_value()",
                       self);
                i64::max_value()
            }
        } else {
            self as i64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_of_usize_to_i64() {
        let just_right: usize = 42;
        let zero: usize = 0;

        assert_eq!(just_right.to_i64(), 42);
        assert_eq!(zero.to_i64(), 0);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn conversion_of_too_big_usize_panics_in_debug_mode() {
        let too_big = usize::max_value();
        too_big.to_i64();
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn conversion_of_too_big_usize_caps_in_release_mode() {
        let too_big = usize::max_value();
        assert_eq!(too_big.to_i64(), i64::max_value());
    }

    #[test]
    fn conversion_of_u64_to_i64() {
        let just_right: u64 = 42;
        let zero: u64 = 0;

        assert_eq!(just_right.to_i64(), 42);
        assert_eq!(zero.to_i64(), 0);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn conversion_of_too_big_u64_panics_in_debug_mode() {
        let too_big = u64::max_value();
        too_big.to_i64();
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn conversion_of_too_big_u64_caps_in_release_mode() {
        let too_big = u64::max_value();
        assert_eq!(too_big.to_i64(), i64::max_value());
    }
}
