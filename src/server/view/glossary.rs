pub fn identify_received(username: &str) {
    println!(r#"{{"type":"IDENTIFY","username":"{username}"}}"#);
}

pub fn identify_success_response(username: &str) {
    println!(
        r#"{{"type":"RESPONSE","operation":"IDENTIFY","result":"SUCCESS","extra":"{username}"}}"#
    );
}

pub fn identify_user_already_exists_response(username: &str) {
    println!(
        r#"{{"type":"RESPONSE","operation":"IDENTIFY","result":"USER_ALREADY_EXISTS","extra":"{username}"}}"#
    );
}

pub fn new_user(username: &str) {
    println!(r#"{{"type":"NEW_USER","username":"{username}"}}"#);
}
