use mod_api::{self, Hooks};

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {}

pub fn cape_url(name: &str) -> Option<String> {
    if name.is_empty()
        || name.len() > 16
        || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        return None;
    }
    Some(format!("http://s.optifine.net/capes/{name}.png"))
}

pub fn normalize_cape(width: u32, height: u32, rgba: &[u8]) -> Option<Vec<u8>> {
    let (w, h) = (width as usize, height as usize);
    if w == 0 || h == 0 || w > 8192 || h > 8192 || rgba.len() < w.checked_mul(h)?.checked_mul(4)? {
        return None;
    }
    let (mut padded_w, mut padded_h) = (64, 32);
    while padded_w < w || padded_h < h {
        padded_w *= 2;
        padded_h *= 2;
    }
    let mut out = vec![0; 64 * 64 * 4];
    for y in 0..32 {
        for x in 0..64 {
            let (sx, sy) = (x * padded_w / 64, y * padded_h / 32);
            if sx < w && sy < h {
                let (src, dst) = ((sy * w + sx) * 4, (y * 64 + x) * 4);
                out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
            }
        }
    }
    Some(out)
}

pub fn corrected_cape(location: &str, load: impl FnOnce() -> Option<Vec<u8>>) -> Option<Vec<u8>> {
    use std::sync::{Mutex, OnceLock};
    static CACHE: OnceLock<Mutex<std::collections::HashMap<String, Vec<u8>>>> = OnceLock::new();
    let mut cache = CACHE.get_or_init(Default::default).lock().ok()?;
    if let Some(image) = cache.get(location) {
        return Some(image.clone());
    }
    let image = load()?;
    if cache.len() >= 128 {
        cache.clear();
    }
    cache.insert(location.to_owned(), image.clone());
    Some(image)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cape_is_padded_before_scaling_to_atlas() {
        let image = vec![255; 92 * 44 * 4];
        let result = super::normalize_cape(92, 44, &image).unwrap();
        assert_eq!(&result[(21 * 64 + 45) * 4..][..4], &[255; 4]);
        assert_eq!(&result[(21 * 64 + 46) * 4..][..4], &[0; 4]);
        assert_eq!(&result[(22 * 64) * 4..][..4], &[0; 4]);
    }

    #[test]
    fn optifine_url_accepts_only_player_names() {
        assert_eq!(
            super::cape_url("Notch"),
            Some("http://s.optifine.net/capes/Notch.png".into())
        );
        assert_eq!(super::cape_url("../escape"), None);
        assert_eq!(super::cape_url(""), None);
        assert_eq!(super::cape_url("abcdefghijklmnopq"), None);
    }
}
