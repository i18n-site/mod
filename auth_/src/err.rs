// DON'T EDIT , GENERATED BY hpcmod

#[macro_export]
macro_rules! err {
  () => {
    ih::err_code_li!(crate);
  };
}

pub mod password {
  pub const ERR: u32 = 1;
  pub const TOO_SHORT: u32 = 2;
}

pub mod address {
  pub const INVALID_MAIL: u32 = 3;
  pub const EXIST: u32 = 4;
  pub const DISABLE_TEMPMAIL: u32 = 5;
}