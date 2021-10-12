use crate::AccessMode;

pub mod buffer;
pub mod image;

impl AccessMode {
    pub(crate) fn to_wgpu_storage_texture_access(&self) -> wgpu::StorageTextureAccess {
        match self {
            AccessMode::ReadOnly => wgpu::StorageTextureAccess::ReadOnly,
            AccessMode::WriteOnly => wgpu::StorageTextureAccess::WriteOnly,
            AccessMode::ReadWrite => wgpu::StorageTextureAccess::ReadWrite,
        }
    }
}

/// Gives some information about the pixel format.
pub trait PixelInfo {
    fn byte_size() -> usize;
    fn wgpu_format() -> wgpu::TextureFormat;
    fn wgpu_texture_sample() -> wgpu::TextureSampleType;
}

macro_rules! pixel_info_impl {
    ($($name:ident, $size:expr, $format:expr, $sample:expr, #[$doc:meta]);+) => {
        use crate::primitives::PixelInfo;

        $(
            #[$doc]
            pub struct $name;

            impl PixelInfo for $name {
                fn byte_size() -> usize {
                    $size
                }

                fn wgpu_format() -> wgpu::TextureFormat {
                    $format
                }

                fn wgpu_texture_sample() -> wgpu::TextureSampleType {
                    $sample
                }
            }
        )+
    };
}

pub mod pixels {
    pixel_info_impl! {
        Rgba8Uint, 4, wgpu::TextureFormat::Rgba8Uint, wgpu::TextureSampleType::Uint, #[doc = "Red, green, blue, and alpha channels. 8 bit integer per channel. Unsigned in shader."];
        Rgba8UintNorm, 4, wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureSampleType::Float { filterable: false }, #[doc = "Red, green, blue, and alpha channels. 8 bit integer per channel. 0, 255 converted to/from float 0, 1 in shader."];
        Rgba8Sint, 4, wgpu::TextureFormat::Rgba8Sint, wgpu::TextureSampleType::Sint, #[doc = "Red, green, blue, and alpha channels. 8 bit integer per channel. Signed in shader."];
        Rgba8SintNorm, 4, wgpu::TextureFormat::Rgba8Snorm, wgpu::TextureSampleType::Float { filterable: false }, #[doc = "Red, green, blue, and alpha channels. 8 bit integer per channel. -127, 127 converted to/from float -1, 1 in shader."]
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "integrate-image")] {

        pub trait ImageToGpgpu {
            type GpgpuPixel: PixelInfo + GpgpuToImage;
            type NormGpgpuPixel: PixelInfo + GpgpuToImage;
        }

        pub trait GpgpuToImage {
            type ImgPixel: ::image::Pixel;
            type ImgPrimitive: ::image::Primitive;
        }

        macro_rules! image_to_gpgpu_impl {
            ($($img_pixel:ty, $pixel:ty, $norm:ty);+) => {
                $(
                    impl ImageToGpgpu for $img_pixel {
                        type GpgpuPixel = $pixel;
                        type NormGpgpuPixel = $norm;
                    }
                )+
            }
        }

        macro_rules! gpgpu_to_image_impl {
            ($($pixel:ty, $($gpgpu_pixel:ty),+);+) => {
                $(
                    $(
                        impl GpgpuToImage for $gpgpu_pixel {
                            type ImgPixel =  $pixel;
                            type ImgPrimitive = <$pixel as ::image::Pixel>::Subpixel;
                        }
                    )+
                )+
            }
        }

        gpgpu_to_image_impl! {
            ::image::Rgba<u8>, pixels::Rgba8Uint, pixels::Rgba8UintNorm, pixels::Rgba8Sint, pixels::Rgba8SintNorm
        }

        image_to_gpgpu_impl! {
            ::image::Rgba<u8>, pixels::Rgba8Uint, pixels::Rgba8UintNorm;
            ::image::Rgba<i8>, pixels::Rgba8Sint, pixels::Rgba8SintNorm
        }
    }
}
