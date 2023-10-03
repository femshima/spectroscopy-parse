#[derive(Debug, Clone, Default)]
pub enum Unit {
    /// %T
    Transmittance,
    // Reflectance,
    Absorbance,
    /// SB
    Intensity,
    Fluorescence,

    Cd,
    HtVoltage,

    /// cm^{-1}
    Wavenumber,
    /// nm
    Wavelength,

    Time,

    Unknown(u32),

    #[default]
    None,
}

impl From<u32> for Unit {
    fn from(value: u32) -> Self {
        match value {
            0x0 => Self::Transmittance,
            0x3 => Self::Absorbance,
            0x8 => Self::Intensity,
            0xe => Self::Fluorescence,

            0x1001 => Self::Cd,
            0x2001 => Self::HtVoltage,

            0x10000100 => Self::Wavenumber,
            0x10000103 => Self::Wavelength,

            0x20000203 => Self::Time,

            v => Self::Unknown(v),
        }
    }
}
