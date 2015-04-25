use super::State;

#[test]
fn test_insert_to_and_get_from_root_state() {
    let mut state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    state.insert(None, key.clone(), val.clone());
    assert_eq!(&val, state.get(None, &key).unwrap());
}

#[test]
fn test_insert_to_and_get_from_child_state() {
    let root_state = State::new("user".to_string());
    let mut state = State::new_chained(&root_state);
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    state.insert(None, key.clone(), val.clone());
    assert_eq!(&val, state.get(None, &key).unwrap());
}

#[test]
fn test_insert_to_root_state_and_get_from_child_state() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    root_state.insert(None, key.clone(), val.clone());
    let state = State::new_chained(&root_state);
    assert_eq!(&val, state.get(None, &key).unwrap());
}

#[test]
fn test_insert_to_child_state_and_get_none_from_root_state() {
    let root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    let mut state = State::new_chained(&root_state);
    state.insert(None, key.clone(), val.clone());
    assert!(root_state.get(None, &key).is_none());
}

#[test]
fn test_shadow_val_in_root_state() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val_in_root = e_number!(10.5);
    root_state.insert(None, key.clone(), val_in_root);
    let val_in_child = e_number!(0_f64);
    let mut state = State::new_chained(&root_state);
    state.insert(None, key.clone(), val_in_child.clone());
    assert_eq!(&val_in_child, state.get(None, &key).unwrap());
}

#[test]
fn test_insert_to_and_get_from_root_state_with_namespace() {
    let mut state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    state.insert(Some("user".to_string()), key.clone(), val.clone());
    assert_eq!(&val, state.get(Some("user".to_string()).as_ref(), &key).unwrap());
}

#[test]
fn test_insert_to_and_get_from_child_state_with_namespace() {
    let root_state = State::new("user".to_string());
    let mut state = State::new_chained(&root_state);
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    state.insert(Some("user_chained".to_string()), key.clone(), val.clone());
    assert_eq!(&val, state.get(Some("user_chained".to_string()).as_ref(), &key).unwrap());
}

#[test]
fn test_insert_to_root_state_and_get_from_child_state_with_namespace() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    root_state.insert(Some("user".to_string()), key.clone(), val.clone());
    let state = State::new_chained(&root_state);
    assert_eq!(&val, state.get(Some("user".to_string()).as_ref(), &key).unwrap());
}

#[test]
fn test_insert_to_child_state_and_get_none_from_root_state_with_namespace() {
    let root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    let mut state = State::new_chained(&root_state);
    state.insert(Some("user".to_string()), key.clone(), val.clone());
    assert!(root_state.get(Some("user".to_string()).as_ref(), &key).is_none());
}

#[test]
fn test_shadow_val_in_root_state_with_namespace() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val_in_root = e_number!(10.5);
    root_state.insert(Some("user".to_string()), key.clone(), val_in_root);
    let val_in_child = e_number!(0_f64);
    let mut state = State::new_chained(&root_state);
    state.insert(Some("user_chained".to_string()), key.clone(), val_in_child.clone());
    assert_eq!(&val_in_child, state.get(None, &key).unwrap());
}
