use std::{env, time::Duration};

pub trait EnvConfig<T> {
    fn get(env_var: &str, default: Option<T>) -> Option<Self> where Self:Sized;
}

impl EnvConfig<bool> for bool {
    fn get(env_var: &str, default: Option<bool>) -> Option<Self> {
        let result = env::var(env_var.to_uppercase());
        if result.is_err() {
            return default
        }

        let text = result.unwrap();
        let lower = text.to_lowercase();
        if lower == "1" || lower == "true" || lower == "t" {
            return Some(true)
        }

        return Some(false)
    }
}

impl EnvConfig<String> for String {
    fn get(env_var: &str, default: Option<String>) -> Option<Self> {
        let result = env::var(env_var.to_uppercase());
        if result.is_err() {
            return default
        }

        return Some(result.unwrap());
    }
}

impl EnvConfig<Duration> for Duration {
    fn get(env_var: &str, default: Option<Duration>) -> Option<Self> {
        let result = env::var(env_var.to_uppercase());
        if result.is_err() {
            return default
        }

        let dur_text = result.unwrap();
        let interval_text = dur_text[0..(dur_text.len() - 1)].to_string();
        let interval_result= interval_text.parse::<u64>();
        if interval_result.is_err() {
            return default;
        }

        let interval = interval_result.unwrap();
        match dur_text[(dur_text.len() - 1)..].to_string().as_str() {
            "s" => Some(Duration::from_secs(interval)),
            "m" => Some(Duration::from_secs(interval*60)),
            "h" => Some(Duration::from_secs(interval*60*60)),
            "d" => Some(Duration::from_secs(interval*60*60*24)),
            _ => default,
        }
    }
}


macro_rules! impl_numeric {
    ($t0:ty) => {
        impl EnvConfig<$t0> for $t0 {
            fn get(env_var: &str, default: Option<$t0>) -> Option<Self> {
                let result = env::var(env_var.to_uppercase());
                if result.is_err() {
                    return default
                }

                let result = result.unwrap().parse::<$t0>();
                if result.is_err() {
                    return default
                }

                return Some(result.unwrap());
            }
        }
    }
}

impl_numeric!(u8);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);
impl_numeric!(u128);
impl_numeric!(usize);
impl_numeric!(i8);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
impl_numeric!(i128);
impl_numeric!(isize);
impl_numeric!(f32);
impl_numeric!(f64);

#[cfg(test)]
mod tests {
    use env::set_var;

    use super::*;

    #[test]
    fn test_var_init_bool() {
        let x: bool = EnvConfig::get("xyz", Some(true)).unwrap();
        assert!(x == true)
    }

    #[test]
    fn test_var_init_number() {
        let x: i32 = EnvConfig::get("xyz", Some(100)).unwrap();
        assert!(x == 100)
    }

    #[test]
    fn test_var_init_number_2() {
        set_var("TEST_ABCD", "150");
        let x: i32 = EnvConfig::get("TEST_ABCD", None).unwrap();
        assert!(x == 150)
    }

    #[test]
    fn test_var_init_number_3() {
        let x: Option<i64> = EnvConfig::get("TEST_ABCD_EFG", None);
        assert!(x.is_none())
    }

    #[test]
    fn test_var_init_duration() {
        set_var("TEST_DUR1", "15s");
        let x: Duration = EnvConfig::get("TEST_DUR1", None).unwrap();
        assert!(x.as_secs() == 15)
    }

    #[test]
    fn test_var_init_duration_m() {
        set_var("TEST_DUR2", "10m");
        let x: Duration = EnvConfig::get("TEST_DUR2", None).unwrap();
        assert!(x.as_secs() == 600)
    }

    #[test]
    fn test_var_init_duration_h() {
        set_var("TEST_DUR3", "10h");
        let x: Duration = EnvConfig::get("TEST_DUR3", None).unwrap();
        assert!(x.as_secs() == 36000)
    }

    #[test]
    fn test_var_init_duration_d() {
        set_var("TEST_DUR3", "2d");
        let x: Duration = EnvConfig::get("TEST_DUR3", None).unwrap();
        assert!(x.as_secs() == 172800)
    }
}