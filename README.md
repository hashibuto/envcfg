# envcfg
helper tool for configuring data from the environment

## usage
```
use envcfg;
use std::time::Duration;

type Config struct {
  cores: u8,
  threading_enabled: bool,
  max_execution_duration: Duration,
  name: String,
  quantity: i64,
}

let config = Config{
  cores:                  envcfg::get("APP_NUM_CORES", Some(4)).unwrap(),
  threading_enabled:      envcfg::get("APP_THREADING_ENABLED", None).unwrap(),
  max_execution_duration: envcfg::get("APP_MAX_EXEC_DURATION", Some(Duration::from_secs(60 * 30))).unwrap(),
  name:                   envcfg::get("APP_NAME", None).unwrap(),
  quantity:               envcfg::get("APP_QUANTITY", Some(1000)).unwrap(),
};
```

The above code will populate the struct with values from the environment and if the specified value can't be found, then from the supplied default.  If the default is None, unwrapping the result will cause the code to panic.  Defaults should only be specified where failing to specify would otherwise result in a panic.

## rules
- string - Strings will be parsed and returned verbatim.
- bool - booleans will be marked as true if the variable contains "1", "t" or any variant in the capitalization of "true", otherwise it will be assumed false, unless unset.
- numbers - numbers will be parsed to their assigned type and must not violate their upper and lower bounds.
- durations - durations will be parsed using the following format `<N...><D>` where `N` is an ordinary number and `D` is one of `s`, `m`, `h`, `d`, representing seconds, minutes, hours, and days.  a valid value would be `3d` or `20m`.

envcfg does not need to work with structs, any values will do, so long as they are typed as one of the following:  `float`, `int`, `String`, `bool` or `Duration`.
```
  let v: value = envcfg::get("APP_MAX_VALUE", None).unwrap();
```