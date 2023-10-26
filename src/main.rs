use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
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


fn decode_string(encoded_value: &str) -> (serde_json::Value, &str) {
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
fn decode_number(encoded_value: &str) -> (serde_json::Value, &str) {
    let end_index = encoded_value.find('e').unwrap();
    let number_string = &encoded_value[1..end_index];
    let number = number_string.parse::<i64>().unwrap();
    return (
        serde_json::Value::Number(number.into()),
        &encoded_value[end_index + 1..],
    );

}
fn decode_list(encoded_string: &str) -> (serde_json::Value, &str) {
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


// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "decode" {
        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.0.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }}

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_decode_string() {
            let encoded_value = "5:hello";
            let (decoded, rest) = decode_string(encoded_value);
            assert_eq!(decoded, serde_json::Value::String("hello".to_string()));
            assert_eq!(rest, "");
        }
    
        #[test]
        fn test_decode_number() {
            let encoded_value = "i42e";
            let (decoded, rest) = decode_number(encoded_value);
            assert_eq!(decoded, serde_json::Value::Number(42.into()));
            assert_eq!(rest, "");
        }
    
        #[test]
        fn test_decode_list() {
            let encoded_value = "li42e5:helloe";
            let (decoded, rest) = decode_list(encoded_value);
            assert_eq!(
                decoded,
                serde_json::json!([42, "hello".to_string()])
            );
            assert_eq!(rest, "");
        }
    
        #[test]
        fn test_decode_bencoded_value_string() {
            let encoded_value = "5:hello";
            let (decoded, rest) = decode_bencoded_value(encoded_value);
            assert_eq!(decoded, serde_json::Value::String("hello".to_string()));
            assert_eq!(rest, "");
        }
    
        #[test]
        fn test_decode_bencoded_value_number() {
            let encoded_value = "i42e";
            let (decoded, rest) = decode_bencoded_value(encoded_value);
            assert_eq!(decoded, serde_json::Value::Number(42.into()));
            assert_eq!(rest, "");
        }
    
        #[test]
        fn test_decode_bencoded_value_list() {
            let encoded_value = "li42e5:helloe";
            let (decoded, rest) = decode_bencoded_value(encoded_value);
            assert_eq!(
                decoded,
                serde_json::json!([42, "hello".to_string()])
            );
            assert_eq!(rest, "");
        }
    
        #[test]
        fn test_decode_bencoded_value_empty() {
            let encoded_value = "";
            let (decoded, rest) = decode_bencoded_value(encoded_value);
            assert_eq!(decoded, serde_json::Value::Null);
            assert_eq!(rest, "");
        }
    }
    