use crate::RADISH_DB;
use resp::Value;

pub fn handle_client_request(decoder_msg: Value) -> Vec<u8> {
  let reply = if let Value::Array(v) = decoder_msg {
    match &v[0] {
      Value::Bulk(ref s) if s == "GET" || s == "get" => handle_get(v),
      Value::Bulk(ref s) if s == "SET" || s == "set" => handle_set(v),
      Value::Bulk(ref s) if s == "COMMAND" || s == "command" => handle_get(v),
      other => unimplemented!("{:?} is not supported as of now", other),
    }
  } else {
    Err(Value::Error("Invalid Command".to_owned()))
  };
  match reply { Ok(result) | Err(result) => result.encode(), }
}

pub fn handle_get(input: Vec<Value>) -> Result<Value, Value> {
  let input = input.iter().skip(1).collect::<Vec<_>>();
  if input.is_empty() { return Err(Value::Error("Expected 1 argument for GET command".to_owned())) }

  let db_ref = RADISH_DB.lock().unwrap();
  let reply = if let Value::Bulk(ref value) = &input[0] {
    db_ref.get(value).map(|e| Value::Bulk(e.to_string())).unwrap_or(Value::Null)
  } else {
    Value::Null
  };
  Ok(reply)
}

pub fn handle_set(input: Vec<Value>) -> Result<Value, Value> {
  let input = input.iter().skip(1).collect::<Vec<_>>();
  if input.is_empty() || input.len() < 2 { return Err(Value::Error("Expected 2 argument for SET command".to_owned())) }

  match (&input[0], &input[1]) {
    (Value::Bulk(key), Value::Bulk(value)) => {
      let _ = RADISH_DB.lock().unwrap().insert(key.to_string(), value.to_string());
    },
    _ => unimplemented!("SET not implemented for {:?}", input),
  }
  Ok(Value::String("OK".to_owned()))
}
