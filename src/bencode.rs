pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    // If encoded_value starts with a digit, it's a number
    #[warn(unused_variables)]
    if encoded_value.is_empty() {
        return (serde_json::Value::Null, encoded_value);
    }

    if let Some(start) = encoded_value.chars().next() {
        if start.is_digit(10) {
            return decode_string(encoded_value);
        } else if start == 'i' {
            return decode_number(encoded_value);
        } else if start == 'l' {
            return decode_list(encoded_value);
        }
    }

    // Unhandled case
    return (serde_json::Value::Null, encoded_value);
}


//use case implementation in idividual functions
pub fn decode_string(encoded_value: &str) -> (serde_json::Value, &str) {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let end = colon_index + 1 + number as usize;
    let string = &encoded_value[colon_index + 1..end];
    return (
        serde_json::Value::String(string.to_string()),
        &encoded_value[end..],
    );
}
pub fn decode_number(encoded_value: &str) -> (serde_json::Value, &str) {
    let end_index = encoded_value.find('e').unwrap();
    let number_string = &encoded_value[1..end_index];
    let number = number_string.parse::<i64>().unwrap();
    return (
        serde_json::Value::Number(number.into()),
        &encoded_value[end_index + 1..],
    );
}
pub fn decode_list(encoded_string: &str) -> (serde_json::Value, &str) {
    let mut values = Vec::new();
    let end_idx = encoded_string.rfind('e').unwrap();
    if end_idx == 1 {
        //empty list
        return (serde_json::Value::Array(values.into()), &"");
    }
    let encoded_list = &encoded_string[1..end_idx];
    let (mut value, mut rest) = decode_bencoded_value(encoded_list);
    values.push(value);
    while rest != "" {
        (value, rest) = decode_bencoded_value(rest);
        values.push(value);
    }
    (serde_json::Value::Array(values.into()), &"")
}
