use header_host::header_host;
use cookie_set::SET_COOKIE;
use aok::{OK, Result, anyhow};
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
  let cookie = cookie::Cookie::new(xtld::host_tld(host));

  let uid = "xxxx";

  set_header.push(SET_COOKIE, cookie.set_max("u", uid));

  OK
}
