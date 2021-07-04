

pub fn gen_mime_type(mime_types: String) -> String {
  let k = MIME_TYPE.get(mime_types).unwrap_or("");
  k
}