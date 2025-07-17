use assert_cmd::Command;
use serde_json::Value;

#[test]
fn test_classify_hex() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd.arg("--list").arg("classify").arg("0x1234").assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Should classify as hex among others
    let classifications = json.as_array().unwrap();
    assert!(classifications.iter().any(|v| v["encoding"].as_str() == Some("hex")));
}

#[test]
fn test_classify_base64() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd.arg("--list").arg("classify").arg("SGVsbG8gV29ybGQ=").assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    let json: Value = serde_json::from_str(&output).unwrap();
    
    // Should include base64 classification
    let classifications = json.as_array().unwrap();
    assert!(classifications.iter().any(|v| v["encoding"].as_str().unwrap().contains("base64")));
}

#[test]
fn test_convert_with_input_encoding() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["convert", "-i", "hex", "-o", "base64", "0x1234"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Without --list, should return just the converted string
    assert_eq!(output.trim(), "EjQ");
}

#[test]
fn test_convert_without_input_encoding_auto_classify() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["convert", "-o", "base64", "0x1234"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Without --list, should return just the best conversion
    assert_eq!(output.trim(), "EjQ");
}

#[test]
fn test_convert_multiple_outputs() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["--list", "convert", "-i", "hex", "-o", "base64,int,bin", "0xff"])
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
        .args(["classify-and-convert", "-o", "base64", "0x1234"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Without --list, should return just the converted string
    assert_eq!(output.trim(), "EjQ");
}

#[test]
fn test_hash_with_encoding() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["hash", "-a", "sha256", "-i", "hex", "0x1234"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Without --list, should return just the hash
    assert!(output.trim().starts_with("0x"));
}

#[test]
fn test_flatten_array() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["array", "flatten", "[[1, 2], [3, 4]]"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should return the flattened array as a string
    assert!(output.trim().starts_with("["));
}

#[test]
fn test_chunk_array() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["array", "chunk", "-c", "2", "[1, 2, 3, 4, 5, 6]"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should return the chunked array as a string
    assert!(output.trim().starts_with("["));
}

#[test]
fn test_reverse_array() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["array", "reverse", "-d", "1", "[1, 2, 3, 4, 5]"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should return the reversed array as a string
    assert!(output.trim().starts_with("["));
}

#[test]
fn test_rotate_array() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["array", "rotate", "-r", "2", "[1, 2, 3, 4, 5]"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should return the rotated array as a string
    assert!(output.trim().starts_with("["));
}

#[test]
fn test_generate() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["generate", "-e", "hex", "-b", "4"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should return the hex string directly
    assert!(output.trim().starts_with("0x"));
    assert_eq!(output.trim().len(), 10); // "0x" + 8 hex chars
}

#[test]
fn test_random() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["random", "-e", "base64", "-b", "16"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should produce valid base64 directly
    assert!(!output.trim().is_empty());
}

#[test]
fn test_pad_left() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["pad", "-p", "4", "-s", "left", "0x12"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should return the padded string directly
    assert_eq!(output.trim(), "0x00000012");
}

#[test]
fn test_pad_right() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["pad", "-p", "4", "-s", "right", "0x12"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should return the padded string directly
    assert_eq!(output.trim(), "0x12000000");
}

#[test]
fn test_classify_and_hash() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["classify-and-hash", "-a", "sha256", "Hello World"])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should return the hash directly
    assert!(output.trim().starts_with("0x"));
}


#[test]
fn test_convert_base64_to_hex_auto() {
    let mut cmd = Command::cargo_bin("alchemy").unwrap();
    let assert = cmd
        .args(["convert", "-o", "hex", "SGVsbG8gV29ybGQ="])
        .assert();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    assert.success();
    // Should auto-classify as base64 and convert to hex
    assert_eq!(output.trim(), "0x48656c6c6f20576f726c64");
}
