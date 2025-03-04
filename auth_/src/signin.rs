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
  let host = if let Some(host) = headers.get("x-forwarded-host") {
    host
  } else if let Some(host) = headers.get("host") {
    host
  } else {
    Err(anyhow!("no host"))?;
    unreachable!();
  }
  .to_str()?;

  let tld = xtld::host_tld(host);
  dbg!(&headers);

  let browser_id = "xxx";

  // set_header.push(
  //   SET_COOKIE,
  //   format!("b={browser_id}; Domain={tld}; Max-Age=99999999; Secure; HttpOnly;  Path=/; Partitioned;"),
  // );

  OK
}
