mod bencode; 
use bencode::decode_bencoded_value;
use std::env;

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
    }
}
//Unit test for every use case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_string() {
        let encoded_value = "5:hello";
        let (decoded, rest) = bencode::decode_string(encoded_value);
        assert_eq!(decoded, serde_json::Value::String("hello".to_string()));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_decode_number() {
        let encoded_value = "i42e";
        let (decoded, rest) = bencode::decode_number(encoded_value);
        assert_eq!(decoded, serde_json::Value::Number(42.into()));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_decode_list() {
        let encoded_value = "li42e5:helloe";
        let (decoded, rest) = bencode::decode_list(encoded_value);
        assert_eq!(decoded, serde_json::json!([42, "hello".to_string()]));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_decode_bencoded_value_string() {
        let encoded_value = "5:hello";
        let (decoded, rest) = bencode::decode_bencoded_value(encoded_value);
        assert_eq!(decoded, serde_json::Value::String("hello".to_string()));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_decode_bencoded_value_number() {
        let encoded_value = "i42e";
        let (decoded, rest) = bencode::decode_bencoded_value(encoded_value);
        assert_eq!(decoded, serde_json::Value::Number(42.into()));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_decode_bencoded_value_list() {
        let encoded_value = "li42e5:helloe";
        let (decoded, rest) = bencode::decode_bencoded_value(encoded_value);
        assert_eq!(decoded, serde_json::json!([42, "hello".to_string()]));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_decode_bencoded_value_empty() {
        let encoded_value = "";
        let (decoded, rest) = bencode::decode_bencoded_value(encoded_value);
        assert_eq!(decoded, serde_json::Value::Null);
        assert_eq!(rest, "");
    }
}