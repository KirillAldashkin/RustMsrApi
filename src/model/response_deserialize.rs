use crate::model::{Error, Response};
use alloc::string::String;
use core::{fmt, marker::PhantomData};
use serde::{Deserialize, de};

const FIELDS: &[&str] = &["code", "msg", "data"];
const CODE_FAILURE_NOT_DATA: &str = "`code` indicates failure but `data` is valid";

struct Visitor<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> de::Visitor<'de> for Visitor<T> {
    type Value = super::Response<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an MSR API response")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut code: Option<u32> = None;
        let mut msg: Option<String> = None;
        let mut data: Option<Result<T, _>> = None;

        fn code_msg_consistent<T: de::Error>(msg: &str, code: u32) -> Result<(), T> {
            if !msg.is_empty() && (code == 0) {
                return Err(de::Error::custom(
                    "`code` indicates success but `msg` is not empty",
                ));
            }
            if msg.is_empty() && (code != 0) {
                return Err(de::Error::custom(
                    "`code` indicates failure but `msg` is empty",
                ));
            }
            Ok(())
        }

        while let Some(key) = map.next_key()? {
            match key {
                "code" => {
                    if code.is_some() {
                        return Err(de::Error::duplicate_field("code"));
                    }
                    let read = map.next_value()?;

                    if read == 0 {
                        if let Some(Err(err)) = data {
                            return Err(err);
                        }
                        // `data` is unsafe-ly assumed to be Ok below!
                    } else if data.as_ref().is_some_and(Result::is_ok) {
                        return Err(de::Error::custom(CODE_FAILURE_NOT_DATA));
                    }
                    if let Some(msg) = &msg {
                        code_msg_consistent(msg, read)?;
                    }

                    code = Some(read)
                }
                "msg" => {
                    if msg.is_some() {
                        return Err(de::Error::duplicate_field("msg"));
                    }
                    let read: String = map.next_value()?;

                    if let Some(code) = code {
                        code_msg_consistent(&read, code)?;
                    }

                    msg = Some(read)
                }
                "data" => {
                    if data.is_some() {
                        return Err(de::Error::duplicate_field("data"));
                    }
                    let read = map.next_value();

                    if let Some(code) = code {
                        if code == 0 {
                            if let Err(err) = read {
                                return Err(err);
                            }
                            // `read` is unsafe-ly assumed to be Ok below!
                        } else if read.is_ok() {
                            return Err(de::Error::custom(CODE_FAILURE_NOT_DATA));
                        }
                    }

                    data = Some(read)
                }
                field => return Err(de::Error::unknown_field(field, FIELDS)),
            }
        }

        let code = code.ok_or(de::Error::missing_field("code"))?;
        let msg = msg.ok_or(de::Error::missing_field("msg"))?;
        let data = data.ok_or(de::Error::missing_field("data"))?;

        Ok(if code != 0 {
            Response(Err(Error { code, msg }))
        } else {
            // SAFETY: `code == 0` and `data == Err` would fail in loop
            Response(Ok(unsafe { data.unwrap_unchecked() }))
        })
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for super::Response<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Response", FIELDS, Visitor(Default::default()))
    }
}
