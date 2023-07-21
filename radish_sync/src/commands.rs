use crate::RADISH_DB;
use resp::Value;

pub fn handle_client_request(devoder_msg: Value) -> Vec<u8> {
  let reply = if let Value::Array(v) = decoded_msg {
    match &v[0] {
      Value::Bulk(ref s) if s == "GET" || s == "get" => handle_get(v),
      Value::Bulk(ref s) if s == "SET" || s == "set" => handle_set(v),
      other => unimplemented!("{:?} is not supported as of now", other),
    }
  } else {
    Err(Value::Error("Invalid Command".to_owned()))
  };
  match reply {
    Ok(result) | Err(result) => result.encode(),
  }
}

pub fn handle_get(v: Vec<Value>) -> Result<Value, Value> {
  let v = v.iter().skip(1).collect::<Vec<_>>();
  if v.is_empty() {
    return Err(Value::Error("Expected 1 argument for GET command".to_owned()))
  }
  let db_ref = RADISH_DB.lock().unwrap();
  let reply = if let Value::Bulk(ref s) = &v[0] {
    db_ref.get(s).map(|e| Value::Bulk(e.to_string())).unwrap_or(Value::Null)
  } else {
    Value:Null
  }
  Ok(reply)
}

pub fn handle_set(v: Vec<Value>) -> Result<Value, Value> {
  let v = v.iter().skip(1).collect::<Vec<_>>();
  if v.is_empty() || v.len() < 2 {
    return Err(Value::Error("Expected 2 argument for SET command".to_owned()))
  }
  match (&v[0], &v[1]) {
    (Value::Bulk(k), Value::Bulk(v)) => { let _ RADISH_DB.lock().unwrap().insert(k.to_string(), v.to_string()) }
    _ => unimplemented!("SET not implemented for {:?}", v),
  }
  Ok(Value::String("OK".to_owned()))
}
