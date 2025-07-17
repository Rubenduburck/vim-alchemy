use assert_cmd::Command;
use serde_json::Value;

#[test]
fn test_classify_hex() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd.arg("classify").arg("0x1234").assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Should classify as hex among others
    let classifications = json.as_array().unwrap();
    assert!(classifications.iter().any(|v| v.as_str() == Some("hex")));
}

#[test]
fn test_classify_base64() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd.arg("classify").arg("SGVsbG8gV29ybGQ=").assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Should include base64 classification
    let classifications = json.as_array().unwrap();
    assert!(classifications.iter().any(|v| v.as_str().unwrap().contains("base-64")));
}

#[test]
fn test_convert_with_input_encoding() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["convert", "-i", "hex", "-o", "base64", "0x1234"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Check the conversion result
    let hex_conversions = &json["hex"];
    assert!(hex_conversions.is_object());
    let base64_result = &hex_conversions["base64"];
    assert_eq!(base64_result["output"].as_str().unwrap(), "EjQ");
}

#[test]
fn test_convert_without_input_encoding_auto_classify() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["convert", "-o", "base64", "0x1234"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Should auto-classify as hex and convert
    let hex_conversions = &json["hex"];
    assert!(hex_conversions.is_object());
    let base64_result = &hex_conversions["base64"];
    assert_eq!(base64_result["output"].as_str().unwrap(), "EjQ");
}

#[test]
fn test_convert_multiple_outputs() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["convert", "-i", "hex", "-o", "base64,int,bin", "0xff"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    let hex_conversions = &json["hex"];
    assert!(hex_conversions["base64"].is_object());
    assert!(hex_conversions["int"].is_object());
    assert!(hex_conversions["bin"].is_object());
}

#[test]
fn test_classify_and_convert() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["classify-and-convert", "-o", "base64", "0x1234"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Should have base64 conversion
    assert!(json["base64"].is_string());
}

#[test]
fn test_hash_with_encoding() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["hash", "-a", "sha256", "-i", "hex", "0x1234"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    let hex_hash = &json["hex"]["sha256"]["output"];
    assert!(hex_hash.is_string());
    assert!(hex_hash.as_str().unwrap().starts_with("0x"));
}

#[test]
fn test_flatten_array() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["flatten-array", "[[1, 2], [3, 4]]"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Just verify it's a successful operation
    assert!(json.is_string());
}

#[test]
fn test_chunk_array() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["chunk-array", "-c", "2", "[1, 2, 3, 4, 5, 6]"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Verify it returns an array representation
    assert!(json.is_string());
    assert!(json.as_str().unwrap().starts_with("["));
}

#[test]
fn test_reverse_array() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["reverse-array", "-d", "1", "[1, 2, 3, 4, 5]"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Verify it returns an array representation
    assert!(json.is_string());
    assert!(json.as_str().unwrap().starts_with("["));
}

#[test]
fn test_rotate_array() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["rotate-array", "-r", "2", "[1, 2, 3, 4, 5]"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Verify it returns an array representation
    assert!(json.is_string());
    assert!(json.as_str().unwrap().starts_with("["));
}

#[test]
fn test_generate() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["generate", "-e", "hex", "-b", "4"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    let hex_str = json.as_str().unwrap();
    assert!(hex_str.starts_with("0x"));
    assert_eq!(hex_str.len(), 10); // "0x" + 8 hex chars
}

#[test]
fn test_random() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["random", "-e", "base64", "-b", "16"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Should produce valid base64
    assert!(json.is_string());
    let base64_str = json.as_str().unwrap();
    assert!(!base64_str.is_empty());
}

#[test]
fn test_pad_left() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["pad-left", "-p", "4", "0x12"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    assert_eq!(json.as_str().unwrap(), "0x00000012");
}

#[test]
fn test_pad_right() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["pad-right", "-p", "4", "0x12"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    assert_eq!(json.as_str().unwrap(), "0x12000000");
}

#[test]
fn test_classify_and_hash() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["classify-and-hash", "-a", "sha256", "Hello World"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Should have sha256 hash
    assert!(json["sha256"].is_string());
}


#[test]
fn test_convert_base64_to_hex_auto() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(&["convert", "-o", "hex", "SGVsbG8gV29ybGQ="])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Should find base64 classification and convert to hex
    let conversions = json.as_object().unwrap();
    let found_base64 = conversions.keys().any(|k| k.contains("base-64"));
    assert!(found_base64);
}