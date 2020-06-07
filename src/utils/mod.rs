extern crate serde_json;

use serde_json as json;
use std::process;
use std::result;

#[allow(dead_code)]
/// Pretty print the JSON docuemnt.
pub fn format(v: &String) -> String {
    let val = _unmarshal(v);
    json::to_string_pretty(&val).unwrap()
}

/// Traverse a JSON document and return a list of serialized strings
/// that have been pretty formatted based on the path argument.
/// The implementation is a restricted subset of the jq utility.
pub fn traverse(v: &String, path: &String) -> Result<Vec<String>, bool> {
    let val: Vec<json::Value> = _traverse(v, path)?;
    let mut res = Vec::<String>::new();
    for s in val {
        res.push(json::to_string_pretty(&s).unwrap());
    }
    Ok(res)
}

/// Traverse the JSON document and return a list of JSON objects based on
/// the path parameter. The following syntax is supported,
/// jq . file
/// jq (\.\w|\.\[\d*\])+ file
fn _traverse(v: &String, path: &String) -> Result<Vec<json::Value>, bool> {
    let val: json::Value = _unmarshal(&v);

    let paths: Vec<&str> = path
        .split('.')
        .into_iter()
        .filter(|x: &&str| x.len() != 0)
        .collect();

    let mut res: Vec<json::Value> = vec![val];
    let mut t: result::Result<Vec<json::Value>, bool>;
    for p in paths {
        t = _parse_key(&mut res, &String::from(p));

        if t.is_err() {
            return Err(false);
        }

        res = t.unwrap();
    }

    Ok(res)
}

/// The parsing logic for Array and Non-Array object.
fn _parse_key(objs: &mut Vec<json::Value>, key: &String) -> Result<Vec<json::Value>, bool> {
    let mut res = Vec::<json::Value>::new();

    // Determine if the key is of array type. eg: .a[] or .a[3]
    let end = key.rfind("]").unwrap_or_default() as usize;
    if end != 0 {
        // Of the Array type
        let start = key.find("[").unwrap_or_default() as usize;
        if start == 0 {
            return Err(false);
        }
        let arr_name = &key[..start];
        for mut v in objs.drain(0..) {
            let obj = v.as_object_mut();

            if obj.is_none() {
                continue;
            }

            let arr = obj.unwrap().get_mut(arr_name);
            if arr.is_none() {
                return Err(false);
            }

            let ex = arr.unwrap().as_array_mut().unwrap();
            if start + 1 == end {
                // If the key does not have an index then return all the
                // objects in that array as a vector.
                ex.drain(0..).for_each(|x| res.push(x));
            } else {
                // If the key has an index, then fetch only that object.
                let idx = key[start + 1..end].parse::<usize>().unwrap();
                if idx >= ex.len() {
                    continue;
                }
                // Clone should be cheaper than remove and shift.
                // res.push(ex.remove(idx));
                res.push(ex[idx].clone());
            }
        }
        // Should only reach if the key object is invalid or corrupt.
        return Ok(res);
    } else {
        // Of the non-Array type
        for v in objs {
            let t = v.as_object();
            if t.is_none() {
                return Err(false);
            }

            let u = t.unwrap().get(key);
            if u.is_none() {
                continue;
            }
            res.push(u.unwrap().clone());
        }

        return Ok(res);
    }
}

/// Unmarshal a String to serde_json::Value which is then worked
/// on. In case it fails, the process exits for an error code.
pub fn _unmarshal(v: &String) -> json::Value {
    let val: json::Value = match json::from_str(v) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Failed to deserialize: {}", err);
            process::exit(1)
        }
    };

    return val;
}
