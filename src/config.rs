//! Pool configuration.
use std::fmt;
use time::Duration;
use debug_builders::DebugStruct;

use {ErrorHandler, NoopErrorHandler};

/// A builder for `Config`.
///
/// See the documentation of `Config` for more details about the default value
/// and meaning of the configuration parameters.
#[derive(Debug)]
pub struct Builder<E> {
    c: Config<E>,
}

impl<E> Builder<E> {
    /// Constructs a new `Builder`.
    ///
    /// Parameters are initialized with their default values.
    #[inline]
    pub fn new() -> Builder<E> {
        Builder {
            c: Config::default(),
        }
    }

    /// Sets `pool_size`.
    ///
    /// # Panics
    ///
    /// Panics if `pool_size` is 0.
    #[inline]
    pub fn pool_size(mut self, pool_size: u32) -> Builder<E> {
        assert!(pool_size > 0, "pool_size must be positive");
        self.c.pool_size = pool_size;
        self
    }

    /// Sets `helper_threads`.
    ///
    /// # Panics
    ///
    /// Panics if `helper_threads` is 0.
    #[inline]
    pub fn helper_threads(mut self, helper_threads: u32) -> Builder<E> {
        assert!(helper_threads > 0, "helper_threads must be positive");
        self.c.helper_threads = helper_threads;
        self
    }

    /// Sets `test_on_check_out`.
    #[inline]
    pub fn test_on_check_out(mut self, test_on_check_out: bool) -> Builder<E> {
        self.c.test_on_check_out = test_on_check_out;
        self
    }

    /// Sets `initialization_fail_fast`.
    #[inline]
    pub fn initialization_fail_fast(mut self, initialization_fail_fast: bool) -> Builder<E> {
        self.c.initialization_fail_fast = initialization_fail_fast;
        self
    }

    /// Sets `connection_timeout`.
    ///
    /// # Panics
    ///
    /// Panics if `connection_timeout` is nonpositive.
    #[inline]
    pub fn connection_timeout(mut self, connection_timeout: Duration) -> Builder<E> {
        assert!(connection_timeout > Duration::zero(), "connection_timeout must be positive");
        self.c.connection_timeout = connection_timeout;
        self
    }

    /// Sets the `error_handler`.
    #[inline]
    pub fn error_handler(mut self, error_handler: Box<ErrorHandler<E>>) -> Builder<E> {
        self.c.error_handler = error_handler;
        self
    }

    /// Consumes the `Builder`, turning it into a `Config`.
    #[inline]
    pub fn build(self) -> Config<E> {
        self.c
    }
}

/// A struct specifying the runtime configuration of a pool.
///
/// `Config` implements `Default`, which provides a set of reasonable default
/// values. It can be constructed using a `Builder`.
pub struct Config<E> {
    pool_size: u32,
    helper_threads: u32,
    test_on_check_out: bool,
    initialization_fail_fast: bool,
    connection_timeout: Duration,
    error_handler: Box<ErrorHandler<E>>,
}

impl<E> fmt::Debug for Config<E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        DebugStruct::new(fmt, "Config")
            .field("pool_size", &self.pool_size)
            .field("helper_threads", &self.helper_threads)
            .field("test_on_check_out", &self.test_on_check_out)
            .field("initialization_fail_fast", &self.initialization_fail_fast)
            .field("connection_timeout", &self.connection_timeout)
            .finish()
    }
}

impl<E> Default for Config<E> {
    #[inline]
    fn default() -> Config<E> {
        Config {
            pool_size: 10,
            helper_threads: 3,
            test_on_check_out: true,
            initialization_fail_fast: true,
            connection_timeout: Duration::seconds(30),
            error_handler: Box::new(NoopErrorHandler),
        }
    }
}

impl<E> Config<E> {
    /// Creates a new `Builder` which can be used to construct a customized
    /// `Config`.
    ///
    /// All parameters are initialized to their default values.
    #[inline]
    pub fn builder() -> Builder<E> {
        Builder::new()
    }

    /// The number of connections managed by the pool.
    ///
    /// Defaults to 10.
    #[inline]
    pub fn pool_size(&self) -> u32 {
        self.pool_size
    }

    /// The number of threads that the pool will use for asynchronous
    /// operations such as connection creation and health checks.
    ///
    /// Defaults to 3.
    #[inline]
    pub fn helper_threads(&self) -> u32 {
        self.helper_threads
    }

    /// If true, the health of a connection will be verified via a call to
    /// `ConnectionManager::is_valid` before it is checked out of the pool.
    ///
    /// Defaults to true.
    #[inline]
    pub fn test_on_check_out(&self) -> bool {
        self.test_on_check_out
    }

    /// If true, `Pool::new` will synchronously initialize its connections,
    /// returning an error if they could not be created.
    ///
    /// Defaults to true.
    #[inline]
    pub fn initialization_fail_fast(&self) -> bool {
        self.initialization_fail_fast
    }

    /// Calls to `Pool::get` will wait this long for a connection to become
    /// available before returning an error.
    ///
    /// Defaults to 30 seconds.
    #[inline]
    pub fn connection_timeout(&self) -> Duration {
        self.connection_timeout
    }

    /// The handler for error reported in the pool.
    ///
    /// Defaults to `r2d2::NoopErrorHandler`.
    #[inline]
    pub fn error_handler(&self) -> &ErrorHandler<E> {
        &*self.error_handler
    }
}
