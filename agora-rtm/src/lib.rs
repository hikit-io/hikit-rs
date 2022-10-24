mod ffi;
use std::ffi::CString;

pub use ffi::root::agora::*;

pub struct RtmService(rtm::IRtmService);

impl RtmService {
    pub fn new() -> Self {
        unsafe {
            let cli = rtm::createRtmService().read();
            Self(cli)
        }
    }

    pub fn login(&mut self, token: &str) -> rtm::LOGIN_ERR_CODE::Type {
        let token = CString::new(token).unwrap();
        unsafe {
            let res = ((*self.0.vtable_).login)(&mut self.0, token.as_ptr(), token.as_ptr());
            // match res {
            //     0 => rtm::LOGIN_ERR_CODE::LOGIN_ERR_OK,
            // }
            rtm::LOGIN_ERR_CODE::LOGIN_ERR_OK
        }
    }
}

#[cfg(test)]
mod tests {

    use std::ffi::{CStr, CString};

    #[test]
    fn it_works() {
        use super::*;
        let mut rtm = RtmService::new();
        let login = rtm.login("token");
        if login == rtm::LOGIN_ERR_CODE::LOGIN_ERR_OK {}
    }
}
