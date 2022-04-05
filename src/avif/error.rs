use libavif_sys as sys;

/// The errors that may occur while processing an image
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("UnknownError")]
    UnknownError,
    #[error("InvalidFtyp")]
    InvalidFtyp,
    #[error("NoContent")]
    NoContent,
    #[error("NoYuvFormatSelected")]
    NoYuvFormatSelected,
    #[error("ReformatFailed")]
    ReformatFailed,
    #[error("UnsupportedDepth")]
    UnsupportedDepth,
    #[error("EncodeColorFailed")]
    EncodeColorFailed,
    #[error("EncodeAlphaFailed")]
    EncodeAlphaFailed,
    #[error("BmffParseFailed")]
    BmffParseFailed,
    #[error("NoAv1ItemsFound")]
    NoAv1ItemsFound,
    #[error("DecodeColorFailed")]
    DecodeColorFailed,
    #[error("DecodeAlphaFailed")]
    DecodeAlphaFailed,
    #[error("ColorAlphaSizeMismatch")]
    ColorAlphaSizeMismatch,
    #[error("IspeSizeMismatch")]
    IspeSizeMismatch,
    #[error("NoCodecAvailable")]
    NoCodecAvailable,
    #[error("NoImagesRemaining")]
    NoImagesRemaining,
    #[error("InvalidExifPayload")]
    InvalidExifPayload,
    #[error("InvalidImageGrid")]
    InvalidImageGrid,
    #[error("InvalidCodecSpecificOption")]
    InvalidCodecSpecificOption,
    #[error("TruncatedData")]
    TruncatedData,
    #[error("IoNotSet")]
    IoNotSet,
    #[error("IoError")]
    IoError,
    #[error("WaitingOnIo")]
    WaitingOnIo,
    #[error("InvalidArgument")]
    InvalidArgument,
    #[error("NotImplemented")]
    NotImplemented,
    /// libavif operation failed with result `code`
    #[error("Other: {0}")]
    OtherLibav(u32),
}

impl Error {
    pub(crate) fn from_code(code: std::os::raw::c_uint) -> Result<(), Error> {
        match code {
            sys::AVIF_RESULT_OK => Ok(()),
            sys::AVIF_RESULT_UNKNOWN_ERROR => Err(Self::UnknownError),
            sys::AVIF_RESULT_INVALID_FTYP => Err(Self::InvalidFtyp),
            sys::AVIF_RESULT_NO_CONTENT => Err(Self::NoContent),
            sys::AVIF_RESULT_NO_YUV_FORMAT_SELECTED => Err(Self::NoYuvFormatSelected),
            sys::AVIF_RESULT_REFORMAT_FAILED => Err(Self::ReformatFailed),
            sys::AVIF_RESULT_UNSUPPORTED_DEPTH => Err(Self::UnsupportedDepth),
            sys::AVIF_RESULT_ENCODE_COLOR_FAILED => Err(Self::EncodeColorFailed),
            sys::AVIF_RESULT_ENCODE_ALPHA_FAILED => Err(Self::EncodeAlphaFailed),
            sys::AVIF_RESULT_BMFF_PARSE_FAILED => Err(Self::BmffParseFailed),
            sys::AVIF_RESULT_NO_AV1_ITEMS_FOUND => Err(Self::NoAv1ItemsFound),
            sys::AVIF_RESULT_DECODE_COLOR_FAILED => Err(Self::DecodeColorFailed),
            sys::AVIF_RESULT_DECODE_ALPHA_FAILED => Err(Self::DecodeAlphaFailed),
            sys::AVIF_RESULT_COLOR_ALPHA_SIZE_MISMATCH => Err(Self::ColorAlphaSizeMismatch),
            sys::AVIF_RESULT_ISPE_SIZE_MISMATCH => Err(Self::IspeSizeMismatch),
            sys::AVIF_RESULT_NO_CODEC_AVAILABLE => Err(Self::NoCodecAvailable),
            sys::AVIF_RESULT_NO_IMAGES_REMAINING => Err(Self::NoImagesRemaining),
            sys::AVIF_RESULT_INVALID_EXIF_PAYLOAD => Err(Self::InvalidExifPayload),
            sys::AVIF_RESULT_INVALID_IMAGE_GRID => Err(Self::InvalidImageGrid),
            sys::AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION => Err(Self::InvalidCodecSpecificOption),
            sys::AVIF_RESULT_TRUNCATED_DATA => Err(Self::TruncatedData),
            sys::AVIF_RESULT_IO_NOT_SET => Err(Self::IoNotSet),
            sys::AVIF_RESULT_IO_ERROR => Err(Self::IoError),
            sys::AVIF_RESULT_WAITING_ON_IO => Err(Self::WaitingOnIo),
            sys::AVIF_RESULT_INVALID_ARGUMENT => Err(Self::InvalidArgument),
            sys::AVIF_RESULT_NOT_IMPLEMENTED => Err(Self::NotImplemented),
            code => Err(Self::OtherLibav(code)),
        }
    }
}
