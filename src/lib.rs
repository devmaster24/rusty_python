use std::ffi::{CStr, c_short};
use std::os::raw::c_char;
use std::str;
use std::time::Instant;
use jsonschema::JSONSchema;
use serde_json::Value;


// Turn a C-string into a string slice and print to console:
#[no_mangle]
pub extern "C" fn print_string(c_string_ptr: *const c_char) {
    let bytes = unsafe { CStr::from_ptr(c_string_ptr).to_bytes() };
    let str_slice = str::from_utf8(bytes).unwrap();
    println!("{}", str_slice);
}


#[no_mangle]
pub extern "C" fn rusty_validate_json(schema: *const c_char, json_data: *const c_char) -> c_short {
    let start = Instant::now();

    let schema_bytes = unsafe { CStr::from_ptr(schema).to_bytes() };
    let data_bytes = unsafe { CStr::from_ptr(json_data).to_bytes() };

    let schema_str = str::from_utf8(schema_bytes).unwrap();
    let data = str::from_utf8(data_bytes).unwrap();

    let res = validate_schema(schema_str, data).into();

    let elapsed_time = start.elapsed();
    println!("Running slow_function() took {} seconds.", elapsed_time.as_secs_f32());

    res
}

fn validate_schema(str_schema: &str, data: &str) -> bool {
    let schema: Value = serde_json::from_str(&str_schema)
        .map_err(|e| e.to_string())
        .expect("Not valid JSON!");

    let compiled_schema = JSONSchema::compile(&schema)
        .map_err(|e| e.to_string())
        .expect("Invalid Schema!");

    let payload = serde_json::from_str(&data)
        .expect("Invalid JSON!");
    

    let result = match compiled_schema.validate(&payload) {
        Ok(_) => { true }
        Err(errors) => {
            for error in errors {
                let path = if error.instance_path.to_string() == "".to_string() { 
                    "<root>".to_string()
                } 
                else { 
                    error.instance_path.to_string() 
                };
    
                println!("Validation error at {} - {}", path, error);
            }
            false
        }
    };

    result
}