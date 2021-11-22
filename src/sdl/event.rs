#![allow(non_upper_case_globals)]

use std::convert::TryFrom;

use sdl::*;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SdlEventType {
    FirstEvent = SDL_EventType_SDL_FIRSTEVENT,
    Quit = SDL_EventType_SDL_QUIT,
    AppTerminating = SDL_EventType_SDL_APP_TERMINATING,
    AppLowMemory = SDL_EventType_SDL_APP_LOWMEMORY,
    AppWillEnterBackground = SDL_EventType_SDL_APP_WILLENTERBACKGROUND,
    AppDidEnterBackground = SDL_EventType_SDL_APP_DIDENTERBACKGROUND,
    AppWillEnterForeground = SDL_EventType_SDL_APP_WILLENTERFOREGROUND,
    AppDidEnterForeground = SDL_EventType_SDL_APP_DIDENTERFOREGROUND,
    LocaleChanged = SDL_EventType_SDL_LOCALECHANGED,
    DisplayEvent = SDL_EventType_SDL_DISPLAYEVENT,
    WindowEvent = SDL_EventType_SDL_WINDOWEVENT,
    SysWmEvent = SDL_EventType_SDL_SYSWMEVENT,
    KeyDown = SDL_EventType_SDL_KEYDOWN,
    KeyUp = SDL_EventType_SDL_KEYUP,
    TextEditing = SDL_EventType_SDL_TEXTEDITING,
    TextInput = SDL_EventType_SDL_TEXTINPUT,
    KeyMapChanged = SDL_EventType_SDL_KEYMAPCHANGED,
    MouseMotion = SDL_EventType_SDL_MOUSEMOTION,
    MouseButtonDown = SDL_EventType_SDL_MOUSEBUTTONDOWN,
    MouseButtonUp = SDL_EventType_SDL_MOUSEBUTTONUP,
    MouseWheel = SDL_EventType_SDL_MOUSEWHEEL,
    JoyAxisMotion = SDL_EventType_SDL_JOYAXISMOTION,
    JoyBallMotion = SDL_EventType_SDL_JOYBALLMOTION,
    JoyHatMotion = SDL_EventType_SDL_JOYHATMOTION,
    JoyButtonDown = SDL_EventType_SDL_JOYBUTTONDOWN,
    JoyButtonUp = SDL_EventType_SDL_JOYBUTTONUP,
    JoyDeviceAdded = SDL_EventType_SDL_JOYDEVICEADDED,
    JoyDeviceRemoved = SDL_EventType_SDL_JOYDEVICEREMOVED,
    ControllerAxisMotion = SDL_EventType_SDL_CONTROLLERAXISMOTION,
    ControllerButtonDown = SDL_EventType_SDL_CONTROLLERBUTTONDOWN,
    ControllerButtonUp = SDL_EventType_SDL_CONTROLLERBUTTONUP,
    ControllerDeviceAdded = SDL_EventType_SDL_CONTROLLERDEVICEADDED,
    ControllerDeviceRemoved = SDL_EventType_SDL_CONTROLLERDEVICEREMOVED,
    ControllerDeviceRemapped = SDL_EventType_SDL_CONTROLLERDEVICEREMAPPED,
    ControllerTouchpadDown = SDL_EventType_SDL_CONTROLLERTOUCHPADDOWN,
    ControllerTouchpadMotion = SDL_EventType_SDL_CONTROLLERTOUCHPADMOTION,
    ControllerTouchpadUp = SDL_EventType_SDL_CONTROLLERTOUCHPADUP,
    ControllerSensorUpdate = SDL_EventType_SDL_CONTROLLERSENSORUPDATE,
    FingerDown = SDL_EventType_SDL_FINGERDOWN,
    FingerUp = SDL_EventType_SDL_FINGERUP,
    FingerMotion = SDL_EventType_SDL_FINGERMOTION,
    DollarGesture = SDL_EventType_SDL_DOLLARGESTURE,
    DollarRecord = SDL_EventType_SDL_DOLLARRECORD,
    MultiGesture = SDL_EventType_SDL_MULTIGESTURE,
    ClipboardUpdate = SDL_EventType_SDL_CLIPBOARDUPDATE,
    DropFile = SDL_EventType_SDL_DROPFILE,
    DropText = SDL_EventType_SDL_DROPTEXT,
    DropBegin = SDL_EventType_SDL_DROPBEGIN,
    DropComplete = SDL_EventType_SDL_DROPCOMPLETE,
    AudioDeviceAdded = SDL_EventType_SDL_AUDIODEVICEADDED,
    AudioDeviceRemoved = SDL_EventType_SDL_AUDIODEVICEREMOVED,
    SensorUpdate = SDL_EventType_SDL_SENSORUPDATE,
    RenderTargetsReset = SDL_EventType_SDL_RENDER_TARGETS_RESET,
    RenderDeviceReset = SDL_EventType_SDL_RENDER_DEVICE_RESET,
    UserEvent = SDL_EventType_SDL_USEREVENT,
    LastEvent = SDL_EventType_SDL_LASTEVENT,
}

#[derive(Debug, Copy, Clone)]
pub enum SdlEventError {
    InvalidValue(u32),
}

impl TryFrom<u32> for SdlEventType {
    type Error = SdlEventError; // TODO Add error type

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            SDL_EventType_SDL_FIRSTEVENT => Ok(Self::FirstEvent),
            SDL_EventType_SDL_QUIT => Ok(Self::Quit),
            SDL_EventType_SDL_APP_TERMINATING => Ok(Self::AppTerminating),
            SDL_EventType_SDL_APP_LOWMEMORY => Ok(Self::AppLowMemory),
            SDL_EventType_SDL_APP_WILLENTERBACKGROUND => Ok(Self::AppWillEnterBackground),
            SDL_EventType_SDL_APP_DIDENTERBACKGROUND => Ok(Self::AppDidEnterBackground),
            SDL_EventType_SDL_APP_WILLENTERFOREGROUND => Ok(Self::AppWillEnterForeground),
            SDL_EventType_SDL_APP_DIDENTERFOREGROUND => Ok(Self::AppDidEnterForeground),
            SDL_EventType_SDL_LOCALECHANGED => Ok(Self::LocaleChanged),
            SDL_EventType_SDL_DISPLAYEVENT => Ok(Self::DisplayEvent),
            SDL_EventType_SDL_WINDOWEVENT => Ok(Self::WindowEvent),
            SDL_EventType_SDL_SYSWMEVENT => Ok(Self::SysWmEvent),
            SDL_EventType_SDL_KEYDOWN => Ok(Self::KeyDown),
            SDL_EventType_SDL_KEYUP => Ok(Self::KeyUp),
            SDL_EventType_SDL_TEXTEDITING => Ok(Self::TextEditing),
            SDL_EventType_SDL_TEXTINPUT => Ok(Self::TextInput),
            SDL_EventType_SDL_KEYMAPCHANGED => Ok(Self::KeyMapChanged),
            SDL_EventType_SDL_MOUSEMOTION => Ok(Self::MouseMotion),
            SDL_EventType_SDL_MOUSEBUTTONDOWN => Ok(Self::MouseButtonDown),
            SDL_EventType_SDL_MOUSEBUTTONUP => Ok(Self::MouseButtonUp),
            SDL_EventType_SDL_MOUSEWHEEL => Ok(Self::MouseWheel),
            SDL_EventType_SDL_JOYAXISMOTION => Ok(Self::JoyAxisMotion),
            SDL_EventType_SDL_JOYBALLMOTION => Ok(Self::JoyBallMotion),
            SDL_EventType_SDL_JOYHATMOTION => Ok(Self::JoyHatMotion),
            SDL_EventType_SDL_JOYBUTTONDOWN => Ok(Self::JoyButtonDown),
            SDL_EventType_SDL_JOYBUTTONUP => Ok(Self::JoyButtonUp),
            SDL_EventType_SDL_JOYDEVICEADDED => Ok(Self::JoyDeviceAdded),
            SDL_EventType_SDL_JOYDEVICEREMOVED => Ok(Self::JoyDeviceRemoved),
            SDL_EventType_SDL_CONTROLLERAXISMOTION => Ok(Self::ControllerAxisMotion),
            SDL_EventType_SDL_CONTROLLERBUTTONDOWN => Ok(Self::ControllerButtonDown),
            SDL_EventType_SDL_CONTROLLERBUTTONUP => Ok(Self::ControllerButtonUp),
            SDL_EventType_SDL_CONTROLLERDEVICEADDED => Ok(Self::ControllerDeviceAdded),
            SDL_EventType_SDL_CONTROLLERDEVICEREMOVED => Ok(Self::ControllerDeviceRemoved),
            SDL_EventType_SDL_CONTROLLERDEVICEREMAPPED => Ok(Self::ControllerDeviceRemapped),
            SDL_EventType_SDL_CONTROLLERTOUCHPADDOWN => Ok(Self::ControllerTouchpadDown),
            SDL_EventType_SDL_CONTROLLERTOUCHPADMOTION => Ok(Self::ControllerTouchpadMotion),
            SDL_EventType_SDL_CONTROLLERTOUCHPADUP => Ok(Self::ControllerTouchpadUp),
            SDL_EventType_SDL_CONTROLLERSENSORUPDATE => Ok(Self::ControllerSensorUpdate),
            SDL_EventType_SDL_FINGERDOWN => Ok(Self::FingerDown),
            SDL_EventType_SDL_FINGERUP => Ok(Self::FingerUp),
            SDL_EventType_SDL_FINGERMOTION => Ok(Self::FingerMotion),
            SDL_EventType_SDL_DOLLARGESTURE => Ok(Self::DollarGesture),
            SDL_EventType_SDL_DOLLARRECORD => Ok(Self::DollarRecord),
            SDL_EventType_SDL_MULTIGESTURE => Ok(Self::MultiGesture),
            SDL_EventType_SDL_CLIPBOARDUPDATE => Ok(Self::ClipboardUpdate),
            SDL_EventType_SDL_DROPFILE => Ok(Self::DropFile),
            SDL_EventType_SDL_DROPTEXT => Ok(Self::DropText),
            SDL_EventType_SDL_DROPBEGIN => Ok(Self::DropBegin),
            SDL_EventType_SDL_DROPCOMPLETE => Ok(Self::DropComplete),
            SDL_EventType_SDL_AUDIODEVICEADDED => Ok(Self::AudioDeviceAdded),
            SDL_EventType_SDL_AUDIODEVICEREMOVED => Ok(Self::AudioDeviceRemoved),
            SDL_EventType_SDL_SENSORUPDATE => Ok(Self::SensorUpdate),
            SDL_EventType_SDL_RENDER_TARGETS_RESET => Ok(Self::RenderTargetsReset),
            SDL_EventType_SDL_RENDER_DEVICE_RESET => Ok(Self::RenderDeviceReset),
            SDL_EventType_SDL_USEREVENT => Ok(Self::UserEvent),
            SDL_EventType_SDL_LASTEVENT => Ok(Self::LastEvent),
            v => Err(SdlEventError::InvalidValue(v)),
        }
    }
}
