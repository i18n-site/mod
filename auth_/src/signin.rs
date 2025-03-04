use header_host::header_host;
use set_cookie::SET_COOKIE;
use aok::{OK, Result};
use http::HeaderMap;
use ctx_::SetHeader;

#[iat::captcha]
pub async fn mail(
  address: &str,
  password: &str,
  headers: &HeaderMap,
  set_header: &SetHeader,
) -> Result<()> {
  let host = header_host(headers)?;
  let cookie = set_cookie::new(xtld::host_tld(host));
  let uid = 12;
  set_header.push(SET_COOKIE, cookie.set_max("u", ub64::u64_b64(uid)));

  OK
}
