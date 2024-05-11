use std::{
    ffi::{c_int, CStr, NulError},
    str::Utf8Error,
};

use pigpiod_if2::*;

use crate::wrapper::W;

#[derive(Debug)]
pub enum Error {
    Pi(c_int),
    Simple(ErrorKind),
    Custom(Box<Custom>),
}

impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Self::other(value)
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::other(value)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorKind {
    // GpioInitialise failed
    InitFailed,
    // GPIO not 0-31
    BadUserGpio,
    // GPIO not 0-53
    BadGpio,
    // Mode not 0-7
    BadMode,
    // Level not 0-1
    BadLevel,
    // Pud not 0-2
    BadPud,
    // Pulsewidth not 0 or 500-2500
    BadPulsewidth,
    // Dutycycle outside set range
    BadDutycycle,
    // Timer not 0-9
    BadTimer,
    // Ms not 10-60000
    BadMs,
    // Timetype not 0-1
    BadTimetype,
    // Seconds < 0
    BadSeconds,
    // Micros not 0-999999
    BadMicros,
    // GpioSetTimerFunc failed
    TimerFailed,
    // Timeout not 0-60000
    BadWdogTimeout,
    // DEPRECATED
    NoAlertFunc,
    // Clock peripheral not 0-1
    BadClkPeriph,
    // DEPRECATED
    BadClkSource,
    // Clock micros not 1, 2, 4, 5, 8, or 10
    BadClkMicros,
    // Buf millis not 100-10000
    BadBufMillis,
    // Dutycycle range not 25-40000
    BadDutyrange,
    // Signum not 0-63
    BadSignum,
    // Can't open pathname
    BadPathname,
    // No handle available
    NoHandle,
    // Unknown handle
    BadHandle,
    // IfFlags > 4
    BadIfFlags,
    // DMA channel not 0-15
    BadChannel,
    // Socket port not 1024-32000
    BadSocketPort,
    // Unrecognized fifo command
    BadFifoCommand,
    // DMA secondary channel not 0-15
    BadSecoChannel,
    // Function called before gpioInitialise
    NotInitialised,
    // Function called after gpioInitialise
    Initialised,
    // Waveform mode not 0-3
    BadWaveMode,
    // Bad parameter in gpioCfgInternals call
    BadCfgInternal,
    // Baud rate not 50-250K(RX)/50-1M(TX)
    BadWaveBaud,
    // Waveform has too many pulses
    TooManyPulses,
    // Waveform has too many chars
    TooManyChars,
    // No bit bang serial read on GPIO
    NotSerialGpio,
    // Bad (null) serial structure parameter
    BadSerialStruc,
    // Bad (null) serial buf parameter
    BadSerialBuf,
    // GPIO operation not permitted
    NotPermitted,
    // One or more GPIO not permitted
    SomePermitted,
    // Bad WVSC subcommand
    BadWvscCommnd,
    // Bad WVSM subcommand
    BadWvsmCommnd,
    // Bad WVSP subcommand
    BadWvspCommnd,
    // Trigger pulse length not 1-100
    BadPulselen,
    // Invalid script
    BadScript,
    // Unknown script id
    BadScriptId,
    // Add serial data offset > 30 minutes
    BadSerOffset,
    // GPIO already in use
    GpioInUse,
    // Must read at least a byte at a time
    BadSerialCount,
    // Script parameter id not 0-9
    BadParamNum,
    // Script has duplicate tag
    DupTag,
    // Script has too many tags
    TooManyTags,
    // Illegal script command
    BadScriptCmd,
    // Script variable id not 0-149
    BadVarNum,
    // No more room for scripts
    NoScriptRoom,
    // Can't allocate temporary memory
    NoMemory,
    // Socket read failed
    SockReadFailed,
    // Socket write failed
    SockWritFailed,
    // Too many script parameters (> 10)
    TooManyParam,
    // Script initialising
    ScriptNotReady,
    // Script has unresolved tag
    BadTag,
    // Bad MICS delay (too large)
    BadMicsDelay,
    // Bad MILS delay (too large)
    BadMilsDelay,
    // Non existent wave id
    BadWaveId,
    // No more CBs for waveform
    TooManyCbs,
    // No more OOL for waveform
    TooManyOol,
    // Attempt to create an empty waveform
    EmptyWaveform,
    // No more waveforms
    NoWaveformId,
    // Can't open I2C device
    I2cOpenFailed,
    // Can't open serial device
    SerOpenFailed,
    // Can't open SPI device
    SpiOpenFailed,
    // Bad I2C bus
    BadI2cBus,
    // Bad I2C address
    BadI2cAddr,
    // Bad SPI channel
    BadSpiChannel,
    // Bad i2c/spi/ser open flags
    BadFlags,
    // Bad SPI speed
    BadSpiSpeed,
    // Bad serial device name
    BadSerDevice,
    // Bad serial baud rate
    BadSerSpeed,
    // Bad i2c/spi/ser parameter
    BadParam,
    // I2c write failed
    I2cWriteFailed,
    // I2c read failed
    I2cReadFailed,
    // Bad SPI count
    BadSpiCount,
    // Ser write failed
    SerWriteFailed,
    // Ser read failed
    SerReadFailed,
    // Ser read no data available
    SerReadNoData,
    // Unknown command
    UnknownCommand,
    // Spi xfer/read/write failed
    SpiXferFailed,
    // Bad (NULL) pointer
    BadPointer,
    // No auxiliary SPI on Pi A or B
    NoAuxSpi,
    // GPIO is not in use for PWM
    NotPwmGpio,
    // GPIO is not in use for servo pulses
    NotServoGpio,
    // GPIO has no hardware clock
    NotHclkGpio,
    // GPIO has no hardware PWM
    NotHpwmGpio,
    // Invalid hardware PWM frequency
    BadHpwmFreq,
    // Hardware PWM dutycycle not 0-1M
    BadHpwmDuty,
    // Invalid hardware clock frequency
    BadHclkFreq,
    // Need password to use hardware clock 1
    BadHclkPass,
    // Illegal, PWM in use for main clock
    HpwmIllegal,
    // Serial data bits not 1-32
    BadDatabits,
    // Serial (half) stop bits not 2-8
    BadStopbits,
    // Socket/pipe message too big
    MsgToobig,
    // Bad memory allocation mode
    BadMallocMode,
    // Too many I2C transaction segments
    TooManySegs,
    // An I2C transaction segment failed
    BadI2cSeg,
    // SMBus command not supported by driver
    BadSmbusCmd,
    // No bit bang I2C in progress on GPIO
    NotI2cGpio,
    // Bad I2C write length
    BadI2cWlen,
    // Bad I2C read length
    BadI2cRlen,
    // Bad I2C command
    BadI2cCmd,
    // Bad I2C baud rate, not 50-500k
    BadI2cBaud,
    // Bad chain loop count
    ChainLoopCnt,
    // Empty chain loop
    BadChainLoop,
    // Too many chain counters
    ChainCounter,
    // Bad chain command
    BadChainCmd,
    // Bad chain delay micros
    BadChainDelay,
    // Chain counters nested too deeply
    ChainNesting,
    // Chain is too long
    ChainTooBig,
    // Deprecated function removed
    Deprecated,
    // Bit bang serial invert not 0 or 1
    BadSerInvert,
    // Bad ISR edge value, not 0-2
    BadEdge,
    // Bad ISR initialisation
    BadIsrInit,
    // Loop forever must be last command
    BadForever,
    // Bad filter parameter
    BadFilter,
    // Bad pad number
    BadPad,
    // Bad pad drive strength
    BadStrength,
    // File open failed
    FilOpenFailed,
    // Bad file mode
    BadFileMode,
    // Bad file flag
    BadFileFlag,
    // Bad file read
    BadFileRead,
    // Bad file write
    BadFileWrite,
    // File not open for read
    FileNotRopen,
    // File not open for write
    FileNotWopen,
    // Bad file seek
    BadFileSeek,
    // No files match pattern
    NoFileMatch,
    // No permission to access file
    NoFileAccess,
    // File is a directory
    FileIsADir,
    // Bad shell return status
    BadShellStatus,
    // Bad script name
    BadScriptName,
    // Bad SPI baud rate, not 50-500k
    BadSpiBaud,
    // No bit bang SPI in progress on GPIO
    NotSpiGpio,
    // Bad event id
    BadEventId,
    // Used by Python
    CmdInterrupted,
    // Not available on BCM2711
    NotOnBcm2711,
    // Only available on BCM2711
    OnlyOnBcm2711,

    // Pigif errors
    BadSend,
    BadRecv,
    BadGetaddrinfo,
    BadConnect,
    BadSocket,
    BadNoib,
    DuplicateCallback,
    BadMalloc,
    BadCallback,
    NotifyFailed,
    CallbackNotFound,
    UnconnectedPi,
    TooManyPis,

    // Other errors i.e. from Rust or unknown
    Other,
}

#[derive(Debug)]
pub struct Custom {
    kind: ErrorKind,
    error: Box<dyn std::error::Error + Send + Sync>,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        fn _new(kind: ErrorKind, error: Box<dyn std::error::Error + Send + Sync>) -> Error {
            Error::Custom(Box::new(Custom { kind, error }))
        }

        _new(kind, error.into())
    }

    pub fn other<E>(error: E) -> Error
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::new(ErrorKind::Other, error)
    }

    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::Pi(code) => ErrorKind::from(*code),
            Error::Simple(kind) => *kind,
            Error::Custom(c) => c.kind,
        }
    }

    pub fn get_ref(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        match &self {
            Error::Custom(c) => Some(&*c.error),
            _ => None,
        }
    }

    pub fn into_inner(self) -> Option<Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self {
            Error::Custom(c) => Some(c.error),
            _ => None,
        }
    }

    pub fn from_raw_pi_error(code: c_int) -> Error {
        Error::Pi(code)
    }

    pub fn raw_pi_error(&self) -> Option<c_int> {
        match self {
            Error::Pi(code) => Some(*code),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pi(code) => {
                let detail = unsafe { CStr::from_ptr(pigpio_error(*code)) };
                write!(f, "{} (pi error {})", W(detail), code)
            }
            Self::Simple(kind) => write!(f, "{}", kind.as_str()),
            Self::Custom(c) => c.error.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Custom(c) => c.error.source(),
            _ => None,
        }
    }
}

impl From<c_int> for ErrorKind {
    fn from(value: c_int) -> Self {
        match value {
            -1 => Self::InitFailed,
            -2 => Self::BadUserGpio,
            -3 => Self::BadGpio,
            -4 => Self::BadMode,
            -5 => Self::BadLevel,
            -6 => Self::BadPud,
            -7 => Self::BadPulsewidth,
            -8 => Self::BadDutycycle,
            -9 => Self::BadTimer,
            -10 => Self::BadMs,
            -11 => Self::BadTimetype,
            -12 => Self::BadSeconds,
            -13 => Self::BadMicros,
            -14 => Self::TimerFailed,
            -15 => Self::BadWdogTimeout,
            -16 => Self::NoAlertFunc,
            -17 => Self::BadClkPeriph,
            -18 => Self::BadClkSource,
            -19 => Self::BadClkMicros,
            -20 => Self::BadBufMillis,
            -21 => Self::BadDutyrange,
            -22 => Self::BadSignum,
            -23 => Self::BadPathname,
            -24 => Self::NoHandle,
            -25 => Self::BadHandle,
            -26 => Self::BadIfFlags,
            -27 => Self::BadChannel,
            -28 => Self::BadSocketPort,
            -29 => Self::BadFifoCommand,
            -30 => Self::BadSecoChannel,
            -31 => Self::NotInitialised,
            -32 => Self::Initialised,
            -33 => Self::BadWaveMode,
            -34 => Self::BadCfgInternal,
            -35 => Self::BadWaveBaud,
            -36 => Self::TooManyPulses,
            -37 => Self::TooManyChars,
            -38 => Self::NotSerialGpio,
            -39 => Self::BadSerialStruc,
            -40 => Self::BadSerialBuf,
            -41 => Self::NotPermitted,
            -42 => Self::SomePermitted,
            -43 => Self::BadWvscCommnd,
            -44 => Self::BadWvsmCommnd,
            -45 => Self::BadWvspCommnd,
            -46 => Self::BadPulselen,
            -47 => Self::BadScript,
            -48 => Self::BadScriptId,
            -49 => Self::BadSerOffset,
            -50 => Self::GpioInUse,
            -51 => Self::BadSerialCount,
            -52 => Self::BadParamNum,
            -53 => Self::DupTag,
            -54 => Self::TooManyTags,
            -55 => Self::BadScriptCmd,
            -56 => Self::BadVarNum,
            -57 => Self::NoScriptRoom,
            -58 => Self::NoMemory,
            -59 => Self::SockReadFailed,
            -60 => Self::SockWritFailed,
            -61 => Self::TooManyParam,
            -62 => Self::ScriptNotReady,
            -63 => Self::BadTag,
            -64 => Self::BadMicsDelay,
            -65 => Self::BadMilsDelay,
            -66 => Self::BadWaveId,
            -67 => Self::TooManyCbs,
            -68 => Self::TooManyOol,
            -69 => Self::EmptyWaveform,
            -70 => Self::NoWaveformId,
            -71 => Self::I2cOpenFailed,
            -72 => Self::SerOpenFailed,
            -73 => Self::SpiOpenFailed,
            -74 => Self::BadI2cBus,
            -75 => Self::BadI2cAddr,
            -76 => Self::BadSpiChannel,
            -77 => Self::BadFlags,
            -78 => Self::BadSpiSpeed,
            -79 => Self::BadSerDevice,
            -80 => Self::BadSerSpeed,
            -81 => Self::BadParam,
            -82 => Self::I2cWriteFailed,
            -83 => Self::I2cReadFailed,
            -84 => Self::BadSpiCount,
            -85 => Self::SerWriteFailed,
            -86 => Self::SerReadFailed,
            -87 => Self::SerReadNoData,
            -88 => Self::UnknownCommand,
            -89 => Self::SpiXferFailed,
            -90 => Self::BadPointer,
            -91 => Self::NoAuxSpi,
            -92 => Self::NotPwmGpio,
            -93 => Self::NotServoGpio,
            -94 => Self::NotHclkGpio,
            -95 => Self::NotHpwmGpio,
            -96 => Self::BadHpwmFreq,
            -97 => Self::BadHpwmDuty,
            -98 => Self::BadHclkFreq,
            -99 => Self::BadHclkPass,
            -100 => Self::HpwmIllegal,
            -101 => Self::BadDatabits,
            -102 => Self::BadStopbits,
            -103 => Self::MsgToobig,
            -104 => Self::BadMallocMode,
            -105 => Self::TooManySegs,
            -106 => Self::BadI2cSeg,
            -107 => Self::BadSmbusCmd,
            -108 => Self::NotI2cGpio,
            -109 => Self::BadI2cWlen,
            -110 => Self::BadI2cRlen,
            -111 => Self::BadI2cCmd,
            -112 => Self::BadI2cBaud,
            -113 => Self::ChainLoopCnt,
            -114 => Self::BadChainLoop,
            -115 => Self::ChainCounter,
            -116 => Self::BadChainCmd,
            -117 => Self::BadChainDelay,
            -118 => Self::ChainNesting,
            -119 => Self::ChainTooBig,
            -120 => Self::Deprecated,
            -121 => Self::BadSerInvert,
            -122 => Self::BadEdge,
            -123 => Self::BadIsrInit,
            -124 => Self::BadForever,
            -125 => Self::BadFilter,
            -126 => Self::BadPad,
            -127 => Self::BadStrength,
            -128 => Self::FilOpenFailed,
            -129 => Self::BadFileMode,
            -130 => Self::BadFileFlag,
            -131 => Self::BadFileRead,
            -132 => Self::BadFileWrite,
            -133 => Self::FileNotRopen,
            -134 => Self::FileNotWopen,
            -135 => Self::BadFileSeek,
            -136 => Self::NoFileMatch,
            -137 => Self::NoFileAccess,
            -138 => Self::FileIsADir,
            -139 => Self::BadShellStatus,
            -140 => Self::BadScriptName,
            -141 => Self::BadSpiBaud,
            -142 => Self::NotSpiGpio,
            -143 => Self::BadEventId,
            -144 => Self::CmdInterrupted,
            -145 => Self::NotOnBcm2711,
            -146 => Self::OnlyOnBcm2711,
            -2000 => Self::BadSend,
            -2001 => Self::BadRecv,
            -2002 => Self::BadGetaddrinfo,
            -2003 => Self::BadConnect,
            -2004 => Self::BadSocket,
            -2005 => Self::BadNoib,
            -2006 => Self::DuplicateCallback,
            -2007 => Self::BadMalloc,
            -2008 => Self::BadCallback,
            -2009 => Self::NotifyFailed,
            -2010 => Self::CallbackNotFound,
            -2011 => Self::UnconnectedPi,
            -2012 => Self::TooManyPis,
            _ => Self::Other,
        }
    }
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InitFailed => "pigpio initialisation failed",
            Self::BadUserGpio => "GPIO not 0-31",
            Self::BadGpio => "GPIO not 0-53",
            Self::BadMode => "mode not 0-7",
            Self::BadLevel => "level not 0-1",
            Self::BadPud => "pud not 0-2",
            Self::BadPulsewidth => "pulsewidth not 0 or 500-2500",
            Self::BadDutycycle => "dutycycle not 0-range (default 255)",
            Self::BadTimer => "timer not 0-9",
            Self::BadMs => "ms not 10-60000",
            Self::BadTimetype => "timetype not 0-1",
            Self::BadSeconds => "seconds < 0",
            Self::BadMicros => "micros not 0-999999",
            Self::TimerFailed => "gpioSetTimerFunc failed",
            Self::BadWdogTimeout => "timeout not 0-60000",
            Self::NoAlertFunc => "DEPRECATED",
            Self::BadClkPeriph => "clock peripheral not 0-1",
            Self::BadClkSource => "DEPRECATED",
            Self::BadClkMicros => "clock micros not 1, 2, 4, 5, 8, or 10",
            Self::BadBufMillis => "buf millis not 100-10000",
            Self::BadDutyrange => "dutycycle range not 25-40000",
            Self::BadSignum => "signum not 0-63",
            Self::BadPathname => "can't open pathname",
            Self::NoHandle => "no handle available",
            Self::BadHandle => "unknown handle",
            Self::BadIfFlags => "ifFlags > 4",
            Self::BadChannel => "DMA channel not 0-14",
            Self::BadSocketPort => "socket port not 1024-30000",
            Self::BadFifoCommand => "unknown fifo command",
            Self::BadSecoChannel => "DMA secondary channel not 0-14",
            Self::NotInitialised => "function called before gpioInitialise",
            Self::Initialised => "function called after gpioInitialise",
            Self::BadWaveMode => "waveform mode not 0-1",
            Self::BadCfgInternal => "bad parameter in gpioCfgInternals call",
            Self::BadWaveBaud => "baud rate not 50-250K(RX)/50-1M(TX)",
            Self::TooManyPulses => "waveform has too many pulses",
            Self::TooManyChars => "waveform has too many chars",
            Self::NotSerialGpio => "no bit bang serial read in progress on GPIO",
            Self::BadSerialStruc => "bad (null) serial structure parameter",
            Self::BadSerialBuf => "bad (null) serial buf parameter",
            Self::NotPermitted => "no permission to update GPIO",
            Self::SomePermitted => "no permission to update one or more GPIO",
            Self::BadWvscCommnd => "bad WVSC subcommand",
            Self::BadWvsmCommnd => "bad WVSM subcommand",
            Self::BadWvspCommnd => "bad WVSP subcommand",
            Self::BadPulselen => "trigger pulse length not 1-100",
            Self::BadScript => "invalid script",
            Self::BadScriptId => "unknown script id",
            Self::BadSerOffset => "add serial data offset > 30 minute",
            Self::GpioInUse => "GPIO already in use",
            Self::BadSerialCount => "must read at least a byte at a time",
            Self::BadParamNum => "script parameter id not 0-9",
            Self::DupTag => "script has duplicate tag",
            Self::TooManyTags => "script has too many tags",
            Self::BadScriptCmd => "illegal script command",
            Self::BadVarNum => "script variable id not 0-149",
            Self::NoScriptRoom => "no more room for scripts",
            Self::NoMemory => "can't allocate temporary memory",
            Self::SockReadFailed => "socket read failed",
            Self::SockWritFailed => "socket write failed",
            Self::TooManyParam => "too many script parameters (> 10)",
            Self::ScriptNotReady => "script initialising",
            Self::BadTag => "script has unresolved tag",
            Self::BadMicsDelay => "bad MICS delay (too large)",
            Self::BadMilsDelay => "bad MILS delay (too large)",
            Self::BadWaveId => "non existent wave id",
            Self::TooManyCbs => "No more CBs for waveform",
            Self::TooManyOol => "No more OOL for waveform",
            Self::EmptyWaveform => "attempt to create an empty waveform",
            Self::NoWaveformId => "no more waveform ids",
            Self::I2cOpenFailed => "can't open I2C device",
            Self::SerOpenFailed => "can't open serial device",
            Self::SpiOpenFailed => "can't open SPI device",
            Self::BadI2cBus => "bad I2C bus",
            Self::BadI2cAddr => "bad I2C address",
            Self::BadSpiChannel => "bad SPI channel",
            Self::BadFlags => "bad i2c/spi/ser open flags",
            Self::BadSpiSpeed => "bad SPI speed",
            Self::BadSerDevice => "bad serial device name",
            Self::BadSerSpeed => "bad serial baud rate",
            Self::BadParam => "bad i2c/spi/ser parameter",
            Self::I2cWriteFailed => "I2C write failed",
            Self::I2cReadFailed => "I2C read failed",
            Self::BadSpiCount => "bad SPI count",
            Self::SerWriteFailed => "ser write failed",
            Self::SerReadFailed => "ser read failed",
            Self::SerReadNoData => "ser read no data available",
            Self::UnknownCommand => "unknown command",
            Self::SpiXferFailed => "spi xfer/read/write failed",
            Self::BadPointer => "bad (NULL) pointer",
            Self::NoAuxSpi => "no auxiliary SPI on Pi A or B",
            Self::NotPwmGpio => "GPIO is not in use for PWM",
            Self::NotServoGpio => "GPIO is not in use for servo pulses",
            Self::NotHclkGpio => "GPIO has no hardware clock",
            Self::NotHpwmGpio => "GPIO has no hardware PWM",
            Self::BadHpwmFreq => "invalid hardware PWM frequency",
            Self::BadHpwmDuty => "hardware PWM dutycycle not 0-1M",
            Self::BadHclkFreq => "invalid hardware clock frequency",
            Self::BadHclkPass => "need password to use hardware clock 1",
            Self::HpwmIllegal => "illegal, PWM in use for main clock",
            Self::BadDatabits => "serial data bits not 1-32",
            Self::BadStopbits => "serial (half) stop bits not 2-8",
            Self::MsgToobig => "socket/pipe message too big",
            Self::BadMallocMode => "bad memory allocation mode",
            Self::TooManySegs => "too many I2C transaction segments",
            Self::BadI2cSeg => "an I2C transaction segment failed",
            Self::BadSmbusCmd => "SMBus command not supported by driver",
            Self::NotI2cGpio => "no bit bang I2C in progress on GPIO",
            Self::BadI2cWlen => "bad I2C write length",
            Self::BadI2cRlen => "bad I2C read length",
            Self::BadI2cCmd => "bad I2C command",
            Self::BadI2cBaud => "bad I2C baud rate, not 50-500k",
            Self::ChainLoopCnt => "bad chain loop count",
            Self::BadChainLoop => "empty chain loop",
            Self::ChainCounter => "too many chain counters",
            Self::BadChainCmd => "bad chain command",
            Self::BadChainDelay => "bad chain delay micros",
            Self::ChainNesting => "chain counters nested too deeply",
            Self::ChainTooBig => "chain is too long",
            Self::Deprecated => "deprecated function removed",
            Self::BadSerInvert => "bit bang serial invert not 0 or 1",
            Self::BadEdge => "bad ISR edge, not 1, 1, or 2",
            Self::BadIsrInit => "bad ISR initialisation",
            Self::BadForever => "loop forever must be last chain command",
            Self::BadFilter => "bad filter parameter",
            Self::BadPad => "bad pad number",
            Self::BadStrength => "bad pad drive strength",
            Self::FilOpenFailed => "file open failed",
            Self::BadFileMode => "bad file mode",
            Self::BadFileFlag => "bad file flag",
            Self::BadFileRead => "bad file read",
            Self::BadFileWrite => "bad file write",
            Self::FileNotRopen => "file not open for read",
            Self::FileNotWopen => "file not open for write",
            Self::BadFileSeek => "bad file seek",
            Self::NoFileMatch => "no files match pattern",
            Self::NoFileAccess => "no permission to access file",
            Self::FileIsADir => "file is a directory",
            Self::BadShellStatus => "bad shell return status",
            Self::BadScriptName => "bad script name",
            Self::BadSpiBaud => "bad SPI baud rate, not 50-500k",
            Self::NotSpiGpio => "no bit bang SPI in progress on GPIO",
            Self::BadEventId => "bad event id",
            Self::CmdInterrupted => "command interrupted, Python",
            Self::NotOnBcm2711 => "not available on BCM2711",
            Self::OnlyOnBcm2711 => "only available on BCM2711",
            Self::BadSend => "failed to send to pigpiod",
            Self::BadRecv => "failed to receive from pigpiod",
            Self::BadGetaddrinfo => "failed to find address of pigpiod",
            Self::BadConnect => "failed to connect to pigpiod",
            Self::BadSocket => "failed to create socket",
            Self::BadNoib => "failed to open notification in band",
            Self::DuplicateCallback => "identical callback exists",
            Self::BadMalloc => "failed to malloc",
            Self::BadCallback => "bad callback parameter",
            Self::NotifyFailed => "failed to create notification thread",
            Self::CallbackNotFound => "callback not found",
            Self::UnconnectedPi => "not connected to Pi",
            Self::TooManyPis => "too many connected Pis",
            Self::Other => "unknown error",
        }
    }
}
