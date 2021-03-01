/*
 *   Copyright (c) 2021 Jill Please <jillplspls@gmail.com>
 *   All rights reserved.
 */

// TODO: CHECK IF PATH EXISTS

#[macro_export]
macro_rules! import_sheet {
    ($path:expr,$resources:expr,$progress_counter:expr) => {{
        let loader = ($resources)
            .get_mut::<amethyst::assets::DefaultLoader>()
            .expect("oof1");

        let texture = amethyst::assets::Loader::load(&*loader, &($path.to_owned() + ".png"));
        let sprites = amethyst::assets::Loader::load(&*loader, &($path.to_owned() + ".ron"));

        let sheet: Handle<SpriteSheet> = amethyst::assets::Loader::load_from_data(
            &*loader,
            amethyst::renderer::SpriteSheet { texture, sprites },
            &mut $progress_counter,
            &$resources.get().expect("oof2"),
        );

        sheet
    }};
}