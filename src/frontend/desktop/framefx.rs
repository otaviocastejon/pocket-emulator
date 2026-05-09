use crate::storage::VideoFilter;

pub fn copy_frame(src: &[u8], dst: &mut [u8], scratch: &mut [u8], filter: VideoFilter) {
    copy_frame_sized(src, dst, scratch, filter, 160);
}

pub fn copy_frame_sized(
    src: &[u8],
    dst: &mut [u8],
    scratch: &mut [u8],
    filter: VideoFilter,
    width: usize,
) {
    match filter {
        VideoFilter::Sharp => dst.copy_from_slice(src),
        VideoFilter::Smooth => {
            smooth_frame_rgba(src, scratch, width);
            dst.copy_from_slice(scratch);
        }
    }
}

fn smooth_frame_rgba(src: &[u8], dst: &mut [u8], width: usize) {
    if src.len() != dst.len() || src.is_empty() || width == 0 {
        return;
    }
    let pixel_count = src.len() / 4;
    let height = pixel_count / width;
    for y in 0..height {
        for x in 0..width {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;
            let mut a: u32 = 0;
            let mut n: u32 = 0;
            for oy in 0..=1usize {
                for ox in 0..=1usize {
                    let sx = (x + ox).min(width - 1);
                    let sy = (y + oy).min(height - 1);
                    let idx = (sy * width + sx) * 4;
                    r += src[idx] as u32;
                    g += src[idx + 1] as u32;
                    b += src[idx + 2] as u32;
                    a += src[idx + 3] as u32;
                    n += 1;
                }
            }
            let out_idx = (y * width + x) * 4;
            dst[out_idx] = (r / n) as u8;
            dst[out_idx + 1] = (g / n) as u8;
            dst[out_idx + 2] = (b / n) as u8;
            dst[out_idx + 3] = (a / n) as u8;
        }
    }
}
