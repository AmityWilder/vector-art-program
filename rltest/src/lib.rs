use std::{error::Error, fmt, ops::{Deref, DerefMut}, sync::Mutex};
pub use raylib::prelude::*;

#[macro_export]
macro_rules! benchmark {
    ([$reps:literal] $(#[$name:ident] $test:block)+) => {
        $(
            #[inline] fn $name() $test
            let start = std::time::Instant::now();
            for _ in 0..$reps { $name() }
            let time = start.elapsed();
            println!(concat!("{} reps of {:?} took {:?}"), $reps, stringify!($name), time);
        )+
    };
}

#[macro_export]
macro_rules! rl_assert {
    ($assertion:expr) => {
        if !($assertion) {
            Err($crate::RlTestFailure::FailedAssertion(stringify!($assertion).to_string())).into();
        }
    };

    ($assertion:expr, $($arg:tt)+) => {
        if !($assertion) {
            Err($crate::RlTestFailure::FailedAssertion(format!($($arg)*))).into();
        }
    };
}

#[macro_export]
macro_rules! rl_assert_eq {
    ($lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            return Err($crate::RlTestFailure::FailedAssertion(format!(concat!(stringify!($lhs), " == ", stringify!($lhs), "\n left: {:?}\nright: {:?}"), $lhs, $rhs))).into();
        }
    };

    ($lhs:expr, $rhs:expr, $($arg:tt)+) => {
        if $lhs != $rhs {
            return Err($crate::RlTestFailure::FailedAssertion(format!("{}\n left: {:?}\nright: {:?}", format!($($arg)*), $lhs, $rhs))).into();
        }
    };
}

#[derive(Debug)]
pub enum RlTestFailure {
    Cancelled,
    FailedAssertion(String),
    FailedCustom(String),
    FailedResourceLoad(String),
    Other(Box<dyn Error + 'static>),
}

impl fmt::Display for RlTestFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => write!(f, "test cancelled manually"),
            Self::FailedAssertion(msg) => write!(f, "assertion failed: {msg}"),
            Self::FailedCustom(msg) => write!(f, "custom failed: {msg}"),
            Self::FailedResourceLoad(msg) => write!(f, "failed to load resource: {msg}"),
            Self::Other(err) => write!(f, "other: {err}"),
        }
    }
}

impl Error for RlTestFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Other(error) => Some(error.as_ref()),

            | Self::Cancelled
            | Self::FailedAssertion(_)
            | Self::FailedCustom(_)
            | Self::FailedResourceLoad(_)
                => None,
        }
    }
}

pub type Result = std::result::Result<(), RlTestFailure>;

pub struct TestDrawnToken(());

pub enum TestStatus {
    Finished(Result),
    Continue(TestDrawnToken),
}
pub use TestStatus::*;

impl From<Result> for TestStatus {
    fn from(value: Result) -> Self {
        Self::Finished(value)
    }
}

#[macro_export]
macro_rules! success {
    () => {
        return Ok(()).into();
    };
}

#[macro_export]
macro_rules! failure {
    ($($arg:tt)+) => {
        return Err($crate::RlTestFailure::FailedCustom(format!($($arg)+))).into();
    };
}

/// Ensures only one test runs at a time
static TALKING_STICK: Mutex<()> = Mutex::new(());

pub fn rl_test<'a, F>(
    title: &'static str,
    w: u16,
    h: u16,
    fps: u32,
    main_fn: F,
) -> Result
where
    F: 'a + for<'b> FnOnce(&'b mut RaylibTestWrapper) -> Result,
{
    let _permission_to_live = TALKING_STICK.lock()
        .unwrap_or_else(|err| {
            TALKING_STICK.clear_poison();
            err.into_inner()
        });
    {
        let (mut rl, thread) = raylib::prelude::init()
            .title(title)
            .size(w as i32, h as i32)
            .build();

        rl.set_target_fps(fps);

        let mut wrapper = RaylibTestWrapper { rl, thread };
        main_fn(&mut wrapper)
    }
}

pub struct RaylibTestWrapper {
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl Deref for RaylibTestWrapper {
    type Target = RaylibHandle;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.rl
    }
}

impl DerefMut for RaylibTestWrapper {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rl
    }
}

impl<'a> RaylibTestWrapper {
    pub fn run<'b, F>(
        &'b mut self,
        mut tick_fn: F,
    ) -> Result
    where
        F: 'b + for<'c, 'd> FnMut(&'d mut RaylibTestHandle<'c>) -> TestStatus,
    {
        while !self.rl.window_should_close() {
            if let Finished(conclusion) = tick_fn(&mut RaylibTestHandle(self)) {
                return conclusion;
            }
        }
        Err(RlTestFailure::Cancelled)
    }

    pub fn load_texture(&mut self, filename: &str) -> std::result::Result<Texture2D, RlTestFailure> {
        self.rl.load_texture(&self.thread, filename)
            .map_err(|e| RlTestFailure::FailedCustom(e))
    }

    pub fn load_texture_from_image(&mut self, image: &Image) -> std::result::Result<Texture2D, RlTestFailure> {
        self.rl.load_texture_from_image(&self.thread, image)
            .map_err(|e| RlTestFailure::FailedCustom(e))
    }
}

pub struct RaylibTestHandle<'c>(&'c mut RaylibTestWrapper);

impl<'c> Deref for RaylibTestHandle<'c> {
    type Target = RaylibTestWrapper;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'c> DerefMut for RaylibTestHandle<'c> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'c> RaylibTestHandle<'c> {
    pub fn begin_drawing<'d, F>(
        &'d mut self,
        clear_color: Color,
        draw_fn: F,
    ) -> TestStatus // will always be Continue, test is not allowed to conclude during draw phase.
    where
        F: 'd + for<'e, 'f, 'g> Fn(&'g mut RaylibTestDrawHandle<'e, 'f>) -> (),
    {
        let mut d  = self.0.rl.begin_drawing(&self.0.thread);
        d.clear_background(clear_color);
        draw_fn(&mut RaylibTestDrawHandle(&mut d));
        d.draw_fps(0, 0);
        Continue(TestDrawnToken(()))
    }
}

pub struct RaylibTestDrawHandle<'e, 'f>(&'f mut RaylibDrawHandle<'e>);

impl<'e, 'f> Deref for RaylibTestDrawHandle<'e, 'f> {
    type Target = RaylibDrawHandle<'e>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'e, 'f> DerefMut for RaylibTestDrawHandle<'e, 'f> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'e, 'f> RaylibDraw for RaylibTestDrawHandle<'e, 'f> {}
