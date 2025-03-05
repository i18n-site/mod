use header_host::header_host;
use aok::{OK, Result};
use xkv::{
  R,
  fred::interfaces::{KeysInterface, SortedSetsInterface},
};
use set_cookie::{MAX, SET_COOKIE};
use http::{Extensions, HeaderMap};
use ctx_::SetHeader;
use cookie_b::Browser;
use xbin::concat;
use simple_useragent::UserAgentParser;

use crate::r::{R_BROWSER, R_USER_BROWSER};

#[static_init::dynamic]
pub static UA: UserAgentParser = UserAgentParser::new();

#[iat::captcha]
pub async fn mail(
  address: &str,
  password: &str,
  headers: &HeaderMap,
  set_header: &SetHeader,
  browser: &Browser,
) -> Result<()> {
  /*
    根据域名id，邮箱确认用户，不同网站可以复用一套后台
  */

  let host = header_host(headers)?;
  let cookie = set_cookie::new(xtld::host_tld(host));
  let uid = 12;

  let now = sts::sec() as f64;

  let uid_bin = &intbin::u64_bin(uid)[..];

  let key = concat!(R_BROWSER, browser.bin);

  /*

  browser -> user 浏览器登录了的用户
  user -> browser 用户在哪些设备上登录

  */
  let p = R.pipeline();
  let _: () = p
    .zadd(key, None, None, false, false, (now, uid_bin))
    .await?;
  let _: () = p.expire(key, MAX as _, None).await?;
  let _: () = p
    .zadd(
      concat!(R_USER_BROWSER, uid_bin),
      None,
      None,
      false,
      false,
      (now, &browser.bin[..]),
    )
    .await?;

  dbg!(headers);
  let _: () = p.last().await?;

  set_header.push(SET_COOKIE, cookie.set_max_for_js("u", ub64::u64_b64(uid)));
  OK
}
