use image::DynamicImage;
use image::ImageBuffer;
use image::Rgba;
use image::RgbaImage;
use imageproc::geometric_transformations::warp_into_with;
use imageproc::geometric_transformations::Interpolation;
use std::cmp::min;

/// Creates a Smash Ultimate Minecraft Steve inspired render from the given Minecraft skin texture.
pub fn create_render(minecraft_skin_texture: &RgbaImage) -> RgbaImage {
    let lighting = image::load_from_memory(include_bytes!("../lighting.png"))
        .unwrap()
        .into_rgba();
    let lighting_leg_l2 = image::load_from_memory(include_bytes!("../lighting_leg_l2.png"))
        .unwrap()
        .into_rgba();
    let lighting_leg_r2 = image::load_from_memory(include_bytes!("../lighting_leg_r2.png"))
        .unwrap()
        .into_rgba();
    let lighting_arm_l2 = image::load_from_memory(include_bytes!("../lighting_arm_l2.png"))
        .unwrap()
        .into_rgba();
    let lighting_arm_r2 = image::load_from_memory(include_bytes!("../lighting_arm_r2.png"))
        .unwrap()
        .into_rgba();
    let lighting_chest2 = image::load_from_memory(include_bytes!("../lighting_chest2.png"))
        .unwrap()
        .into_rgba();
    let lighting_head2 = image::load_from_memory(include_bytes!("../lighting_head2.png"))
        .unwrap()
        .into_rgba();

    // TODO: Refactor image loading to be cleaner.
    // At least 16 bit precision is required for the texture sampling to look decent.
    let head_uvs = match image::load_from_memory(include_bytes!("../head_uvs.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let head_uvs2 = match image::load_from_memory(include_bytes!("../head_uvs2.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let chest_uvs = match image::load_from_memory(include_bytes!("../chest_uvs.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let chest_uvs2 = match image::load_from_memory(include_bytes!("../chest_uvs2.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let leg_rl_uvs = match image::load_from_memory(include_bytes!("../leg_rl_uvs.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let leg_l_uvs2 = match image::load_from_memory(include_bytes!("../leg_l_uvs2.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let leg_r_uvs2 = match image::load_from_memory(include_bytes!("../leg_r_uvs2.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let arm_l_uvs = match image::load_from_memory(include_bytes!("../arm_l_uvs.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let arm_r_uvs = match image::load_from_memory(include_bytes!("../arm_r_uvs.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let arm_l_uvs2 = match image::load_from_memory(include_bytes!("../arm_l_uvs2.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };
    let arm_r_uvs2 = match image::load_from_memory(include_bytes!("../arm_r_uvs2.png")).unwrap() {
        DynamicImage::ImageRgba16(buffer) => buffer,
        _ => panic!("Expected RGBA 16 bit for UVs"),
    };

    let mut output = ImageBuffer::new(head_uvs.dimensions().0, head_uvs.dimensions().1);

    // TODO: There may be some optimizations possible for pixels that have 0 alpha.
    // TODO: Threading?

    // Steve has simple geometry, so blend layers from back to front rather than using a depth map.
    blend_layer_with_base(&mut output, &leg_rl_uvs, minecraft_skin_texture, &lighting);
    blend_layer_with_base(&mut output, &arm_l_uvs, minecraft_skin_texture, &lighting);
    blend_layer_with_base(&mut output, &head_uvs, minecraft_skin_texture, &lighting);
    blend_layer_with_base(&mut output, &chest_uvs, minecraft_skin_texture, &lighting);
    blend_layer_with_base(
        &mut output,
        &arm_l_uvs2,
        minecraft_skin_texture,
        &lighting_arm_l2,
    );
    blend_layer_with_base(
        &mut output,
        &chest_uvs2,
        minecraft_skin_texture,
        &lighting_chest2,
    );
    blend_layer_with_base(
        &mut output,
        &head_uvs2,
        minecraft_skin_texture,
        &lighting_head2,
    );
    blend_layer_with_base(
        &mut output,
        &leg_l_uvs2,
        minecraft_skin_texture,
        &lighting_leg_l2,
    );
    blend_layer_with_base(
        &mut output,
        &leg_r_uvs2,
        minecraft_skin_texture,
        &lighting_leg_r2,
    );
    blend_layer_with_base(&mut output, &arm_r_uvs, minecraft_skin_texture, &lighting);
    blend_layer_with_base(
        &mut output,
        &arm_r_uvs2,
        minecraft_skin_texture,
        &lighting_arm_r2,
    );

    output
}

/// Creates a render with the dimensions and alpha of the reference chara file
/// by transforming the render using the given transformations.
pub fn create_chara_image(
    render: &RgbaImage,
    chara_reference: &RgbaImage,
    scale: f32,
    translate_x: f32,
    translate_y: f32,
) -> RgbaImage {
    let mut output = ImageBuffer::new(
        chara_reference.dimensions().0,
        chara_reference.dimensions().1,
    );

    // Align the render with the target chara image.
    // warp_into_with defines the preimage, so invert the transformation.
    warp_into_with(
        &render,
        |x, y| ((x - translate_x) / scale, (y - translate_y) / scale),
        Interpolation::Bilinear,
        Rgba([0u8, 0u8, 0u8, 0u8]),
        &mut output,
    );

    // Use the reference image's alpha for appropriate masking on some portraits.
    copy_alpha(&mut output, &chara_reference);

    output
}

fn blend_layer_with_base(
    base: &mut RgbaImage,
    layer_uvs: &ImageBuffer<Rgba<u16>, Vec<u16>>,
    texture: &RgbaImage,
    lighting: &RgbaImage,
) {
    for x in 0..base.width() {
        for y in 0..base.height() {
            let (current_r, current_g, current_b, current_a) =
                normalize_rgba_u8(base.get_pixel(x, y));

            let (u, v, _, uv_alpha) = normalize_rgba_u16(layer_uvs.get_pixel(x, y));
            let head_color = sample_texture(texture, u, v);
            let (layer_r, layer_g, layer_b, head_a) = normalize_rgba_u8(head_color);

            let (light_r, light_g, light_b, _) = normalize_rgba_u8(lighting.get_pixel(x, y));

            // The lighting pass is scaled down by a factor of 0.25 to fit into 8 bits per channel.
            // Multiplying by 4 is a bit too bright, so use 2 instead.
            let apply_lighting = |color: f32, light: f32| color * light * 2f32;

            let get_result = |val1: f32, val2: f32, lighting: f32| {
                let lighting_result = apply_lighting(val2, lighting);
                alpha_blend(val1, lighting_result, head_a * uv_alpha)
            };

            // Use the uv map alpha as well to prevent blending outside the masked region.
            let r = get_result(current_r, layer_r, light_r);
            let g = get_result(current_g, layer_g, light_g);
            let b = get_result(current_b, layer_b, light_b);
            let alpha_final = current_a + head_a * uv_alpha;

            *base.get_pixel_mut(x, y) = Rgba([
                to_u8_clamped(r),
                to_u8_clamped(g),
                to_u8_clamped(b),
                to_u8_clamped(alpha_final),
            ]);
        }
    }
}

fn copy_alpha(target: &mut RgbaImage, source: &RgbaImage) {
    // TODO: There may be a cleaner/more efficient way to do this.
    for x in 0..target.width() {
        for y in 0..target.height() {
            let current = target.get_pixel_mut(x, y);
            let alpha = source.get_pixel(x, y)[3];
            *current = Rgba([current[0], current[1], current[2], alpha]);
        }
    }
}

// TODO: Use a generic type?
// This will work for f64 as well.
// Integer types may not work because of overflow.
fn alpha_blend(val1: f32, val2: f32, alpha: f32) -> f32 {
    val1 * (1f32 - alpha) + val2 * alpha
}

// TODO: There's probably a more generic type than RgbaImage that supports width/height and indexing.
fn sample_texture(image: &RgbaImage, u: f32, v: f32) -> &Rgba<u8> {
    // Flip v to transform from an origin at the bottom left (OpenGL) to top left (image).
    let (x, y) = interpolate_nearest(u, 1f32 - v, image.dimensions().0, image.dimensions().1);
    image.get_pixel(x, y)
}

fn interpolate_nearest(x: f32, y: f32, width: u32, height: u32) -> (u32, u32) {
    // Nearest neighbor interpolation often performs some sort of rounding.
    // UVs are snapped to pixel corners in the exported UV map, so just floor the UVs instead.
    // Clamp to the edges for out of bounds indices.
    let nearest = |f: f32, max_val: u32| min((f * max_val as f32).floor() as u32, max_val - 1);
    (nearest(x, width), nearest(y, height))
}

fn normalize_rgba_u8(pixel: &Rgba<u8>) -> (f32, f32, f32, f32) {
    // 0u16 -> 0.0f32, 65535u16 -> 1.0f32
    let normalize = |u| u as f32 / 255f32;
    (
        normalize(pixel[0]),
        normalize(pixel[1]),
        normalize(pixel[2]),
        normalize(pixel[3]),
    )
}

fn normalize_rgba_u16(pixel: &Rgba<u16>) -> (f32, f32, f32, f32) {
    // 0u16 -> 0.0f32, 65535u16 -> 1.0f32
    let normalize = |u| u as f32 / 65535f32;
    (
        normalize(pixel[0]),
        normalize(pixel[1]),
        normalize(pixel[2]),
        normalize(pixel[3]),
    )
}

fn to_u8_clamped(x: f32) -> u8 {
    // Pick the nearest integer so values close to 1.0 are still converted to 255u8.
    let result = (x * 255f32).round();
    if result < 0.0f32 {
        return 0u8;
    } else if result > 255f32 {
        return 255u8;
    } else {
        return result as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_nearest_8x8_edges() {
        assert_eq!(interpolate_nearest(0f32, 0f32, 8u32, 8u32), (0u32, 0u32));
        assert_eq!(interpolate_nearest(0f32, 1f32, 8u32, 8u32), (0u32, 7u32));
        assert_eq!(interpolate_nearest(1f32, 0f32, 8u32, 8u32), (7u32, 0u32));
        assert_eq!(interpolate_nearest(1f32, 1f32, 8u32, 8u32), (7u32, 7u32));
    }

    #[test]
    fn test_interpolate_nearest_out_of_bounds() {
        assert_eq!(interpolate_nearest(0f32, 1.5f32, 8u32, 8u32), (0u32, 7u32));
        assert_eq!(interpolate_nearest(1.5f32, 0f32, 8u32, 8u32), (7u32, 0u32));
        assert_eq!(
            interpolate_nearest(1.5f32, 1.5f32, 8u32, 8u32),
            (7u32, 7u32)
        );
    }

    #[test]
    fn test_normalize_u8() {
        assert_eq!(
            normalize_rgba_u8(&Rgba([0u8, 0u8, 0u8, 0u8])),
            (0f32, 0f32, 0f32, 0f32)
        );
        assert_eq!(
            normalize_rgba_u8(&Rgba([255u8, 255u8, 255u8, 255u8])),
            (1f32, 1f32, 1f32, 1f32)
        );
    }

    #[test]
    fn test_normalize_u16() {
        assert_eq!(
            normalize_rgba_u16(&Rgba([0u16, 0u16, 0u16, 0u16])),
            (0f32, 0f32, 0f32, 0f32)
        );
        assert_eq!(
            normalize_rgba_u16(&Rgba([65535u16, 65535u16, 65535u16, 65535u16])),
            (1f32, 1f32, 1f32, 1f32)
        );
    }

    #[test]
    fn test_to_u8_clamped() {
        assert_eq!(to_u8_clamped(0.999f32), 255u8);
        assert_eq!(to_u8_clamped(-1.5f32), 0u8);
        assert_eq!(to_u8_clamped(0f32), 0u8);
        assert_eq!(to_u8_clamped(0.5f32), 128u8);
        assert_eq!(to_u8_clamped(1f32), 255u8);
        assert_eq!(to_u8_clamped(1.01f32), 255u8);
    }
}
