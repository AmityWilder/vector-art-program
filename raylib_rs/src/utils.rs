pub enum TraceLogLevel {
    LogTrace,
    LogDebug,
    LogInfo,
    LogWarning,
    LogError,
    LogFatal,
}

#[macro_export]
macro_rules! tracelog {
    ($log_level:expr, $($args:tt)+) => {
        println!("{}: {}", match $log_level {
            $crate::utils::TraceLogLevel::LogTrace => "TRACE",
            $crate::utils::TraceLogLevel::LogDebug => "DEBUG",
            $crate::utils::TraceLogLevel::LogInfo => "INFO",
            $crate::utils::TraceLogLevel::LogWarning => "WARNING",
            $crate::utils::TraceLogLevel::LogError => "ERROR",
            $crate::utils::TraceLogLevel::LogFatal => "FATAL",
        }, format!($($args)+))
    };
}