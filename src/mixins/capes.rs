fn optifine_capes(game: &mut Game<'_>) {
    let Some(remote) = game.remote.as_ref() else { return };
    let own = remote.profile;
    let mut players: Vec<_> = remote.level_players.iter().filter_map(|(id, uuid)| {
        let name = remote.level_player_names.get(id).or_else(|| remote.players.get(uuid))?;
        Some((Some(*id as u32), *uuid, mod_capez::cape_url(name)?))
    }).collect();
    if let Some(url) = remote.players.get(&own).and_then(|name| mod_capez::cape_url(name)) {
        players.push((None, own, url));
    }
    for (id, uuid, url) in players {
        let ready = game.skin_downloads.borrow_mut().state(horizon_client::skins::Kind::Cape, &url)
            == horizon_client::skins::Download::Ready;
        if !ready { continue; }
        let Some(hash) = horizon_client::skins::texture_hash(&url) else { continue };
        let cape = horizon_client::skins::cape_location(&hash);
        let corrected = mod_capez::corrected_cape(&cape, || {
            let digest = cape.strip_prefix("capes/")?;
            let path = std::path::Path::new("assets/skins").join(digest.get(..2)?).join(digest);
            let bytes = std::fs::read(path).ok()?;
            let (width, height, rgba) = horizon_client::hud::decode_bytes(&bytes).ok()?;
            mod_capez::normalize_cape(width, height, &rgba)
        });
        if let Some(image) = corrected {
            let changed = game.skin_downloads.borrow().image(&cape) != Some(image.as_slice());
            if changed { game.store_skin(cape.clone(), image); }
        }
        if !game.again_skin(&cape) { continue; }
        if let Some(mob) = game.mobs.iter_mut().find(|mob| Some(mob.id) == id && mob.cape.is_none()) {
            mob.cape = Some(cape.clone());
        }
        if uuid == own && game.settings.file.bool("modelPart_cape") {
            if let Some(mob) = game.player_model.as_mut().filter(|mob| mob.cape.is_none()) {
                mob.cape = Some(cape);
            }
        }
    }
}
