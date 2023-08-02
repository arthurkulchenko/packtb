use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub(crate) fn shorten_url(url: &str) -> String {
  let mut sha = Sha256::new();
  sha.input_str(url);
  let mut new_url = sha.result_str();
  new_url.truncate(5);
  format!("https://u.rl/{}", new_url)
}
