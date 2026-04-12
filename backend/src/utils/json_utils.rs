use serde_json::Value;

pub fn merge_values(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (key, value) in overlay_map {
                merge_values(base_map.entry(key).or_insert(Value::Null), value);
            }
        }
        (base, overlay) => {
            *base = overlay;
        }
    }
}

pub fn get_value_by_path(value: &Value, path: &str) -> Option<&Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;
    
    for part in parts {
        current = current.get(part)?;
    }
    
    Some(current)
}

pub fn set_value_by_path(value: &mut Value, path: &str, new_value: Value) -> bool {
    let parts: Vec<&str> = path.split('.').collect();
    if parts.is_empty() {
        return false;
    }
    
    let mut current = value;
    for part in &parts[..parts.len()-1] {
        if !current.is_object() {
            return false;
        }
        current = current.get_mut(part).unwrap_or(&mut Value::Null);
    }
    
    if let Value::Object(map) = current {
        map.insert(parts.last().unwrap().to_string(), new_value);
        true
    } else {
        false
    }
}
