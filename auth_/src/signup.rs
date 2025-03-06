//! 邮件注册流程:
//! 1. 注册并发送激活邮件 这时候用户已经登录 但是不能发帖等写操作
//! 2. 验证激活邮件 这时候可以进行写操作了
//!
//! 后台设计:
//! 每个用户 ID 会有一个状态，表示是冻结还是可用
//! 不同的站点有不同的 site id 和 browser id 一起编码到请求头

use xkv::{R, fred::interfaces::FunctionInterface};
use http::HeaderMap;
use aok::{OK, Result, Void};

/// 发送注册的激活邮件
#[iat::captcha]
pub async fn mail(address: &str, password: &str, headers: &HeaderMap) -> Void {
  let host = header_host::tld(headers)?;
  let host_id: u64 = R.fcall(r_::ZSET_ID, &["hostId"], &[host]).await?;

  crate::err_json!();

  let (mail, mail_tld) = xmail::norm_tld(address);

  if !mail_tld.contains('.') || mail_tld.starts_with('.') {
    err!(address INVALID_MAIL);
  }

  if password.len() < 6 {
    err!(password TOO_SHORT);
  }

  err!();

  OK
}

pub enum SignupMailVerify {
  Ok,
  InvalidAddress,
  PasswordTooShort,
}

pub async fn mail_verify(address: &str, code: &str) -> Result<SignupMailVerify> {
  Ok(SignupMailVerify::Ok)
}
