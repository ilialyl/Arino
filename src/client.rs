use crate::database::cloud::get_credentials;

pub fn has_access() -> bool {
    if get_credentials().is_err() {
        return false;
    } else {
        return true;
    }
}
