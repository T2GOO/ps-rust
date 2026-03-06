// engine/pixel_ops.rs — Filtres et ajustements (CPU)

/// Applique un flou gaussien simplifié (box blur) sur un buffer RGBA
pub fn box_blur(pixels: &mut Vec<u8>, width: u32, height: u32, radius: u32) {
    let w = width as usize;
    let h = height as usize;
    let r = radius as usize;
    let mut temp = pixels.clone();

    // Passe horizontale
    for y in 0..h {
        for x in 0..w {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;
            let mut count = 0u32;

            for dx in 0..=(2 * r) {
                let sx = x + dx;
                if sx < r || sx >= w + r { continue; }
                let sx = sx - r;
                let idx = (y * w + sx) * 4;
                r_sum += pixels[idx] as u32;
                g_sum += pixels[idx + 1] as u32;
                b_sum += pixels[idx + 2] as u32;
                a_sum += pixels[idx + 3] as u32;
                count += 1;
            }

            let idx = (y * w + x) * 4;
            temp[idx]     = (r_sum / count) as u8;
            temp[idx + 1] = (g_sum / count) as u8;
            temp[idx + 2] = (b_sum / count) as u8;
            temp[idx + 3] = (a_sum / count) as u8;
        }
    }

    // Passe verticale
    *pixels = temp.clone();
    for y in 0..h {
        for x in 0..w {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;
            let mut count = 0u32;

            for dy in 0..=(2 * r) {
                let sy = y + dy;
                if sy < r || sy >= h + r { continue; }
                let sy = sy - r;
                let idx = (sy * w + x) * 4;
                r_sum += temp[idx] as u32;
                g_sum += temp[idx + 1] as u32;
                b_sum += temp[idx + 2] as u32;
                a_sum += temp[idx + 3] as u32;
                count += 1;
            }

            let idx = (y * w + x) * 4;
            pixels[idx]     = (r_sum / count) as u8;
            pixels[idx + 1] = (g_sum / count) as u8;
            pixels[idx + 2] = (b_sum / count) as u8;
            pixels[idx + 3] = (a_sum / count) as u8;
        }
    }
}

/// Ajustement niveaux (brightness/contrast simplifié)
pub fn adjust_levels(pixels: &mut Vec<u8>, brightness: f32, contrast: f32) {
    let factor = (259.0 * (contrast + 255.0)) / (255.0 * (259.0 - contrast));

    for chunk in pixels.chunks_mut(4) {
        for c in 0..3 {
            let v = chunk[c] as f32;
            let v = factor * (v - 128.0) + 128.0 + brightness;
            chunk[c] = v.clamp(0.0, 255.0) as u8;
        }
    }
}

/// Inversion de couleurs
pub fn invert(pixels: &mut Vec<u8>) {
    for chunk in pixels.chunks_mut(4) {
        chunk[0] = 255 - chunk[0];
        chunk[1] = 255 - chunk[1];
        chunk[2] = 255 - chunk[2];
        // Ne pas inverser l'alpha
    }
}
