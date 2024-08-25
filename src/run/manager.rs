use std::time::Duration;

use crate::HitSplit;

pub fn prev_split(app: &mut HitSplit) {
    if app.selected_split > 0 {
        app.selected_split -= 1;
        let split = &app
            .loaded_category
            .as_ref()
            .unwrap()
            .splits
            .get(app.selected_split);
        app.chrono
            .load_chrono(split.unwrap().real_time, app.config.chrono_format);
    }
}

pub fn next_split(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        if category.splits.len() > app.selected_split + 1 {
            app.selected_split += 1;
            app.chrono.clear_elapsed();
        } else if app.config.next_split_as_reset {
            app.selected_split = 0;
            let pbs = category.splits.iter().map(|split| split.pb).sum::<u16>();
            let hits = category.splits.iter().map(|split| split.hits).sum::<u16>();
            if pbs > hits {
                set_pb(app);
            }
            reset(app);
        }
    }
}

pub fn sub_hit(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        let split = category.splits.get_mut(app.selected_split).unwrap();
        if split.hits > 0 {
            split.hits -= 1;
        }
    }
}

pub fn add_hit(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        category.splits.get_mut(app.selected_split).unwrap().hits += 1;
    }
}

pub fn reset(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        category.splits.iter_mut().for_each(|split| {
            split.hits = 0;
            split.real_time = Duration::default();
        });
        app.selected_split = 0;
        app.chrono.reset();
    }
}

pub fn set_pb(app: &mut HitSplit) {
    if let Some(category) = app.loaded_category.as_mut() {
        category.splits.iter_mut().for_each(|split| {
            split.pb = split.hits;
        });
    }
}

pub fn start_chrono(app: &mut HitSplit) {
    app.chrono.start();
}

pub fn pause_chrono(app: &mut HitSplit) {
    app.chrono.pause();
}
