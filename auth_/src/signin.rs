use header_host::header_host;
use set_cookie::SET_COOKIE;
use aok::{OK, Result};
use http::{Extensions, HeaderMap};
use ctx_::SetHeader;
use cookie_b::Browser;

#[iat::captcha]
pub async fn mail(
  address: &str,
  password: &str,
  headers: &HeaderMap,
  set_header: &SetHeader,
  browser: &Browser,
) -> Result<()> {
  let host = header_host(headers)?;
  let cookie = set_cookie::new(xtld::host_tld(host));
  let uid = 12;

  dbg!(browser);
  set_header.push(SET_COOKIE, cookie.set_max_for_js("u", ub64::u64_b64(uid)));
  OK
}
