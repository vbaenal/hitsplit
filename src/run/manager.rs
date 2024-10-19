use std::time::Duration;

use crate::{Error, HitSplit};

pub fn prev_split(app: &mut HitSplit) -> Result<(), Error> {
    if app.selected_split > 0 {
        app.selected_split -= 1;
        let split = match &app.loaded_category.as_ref() {
            Some(c) => match c.splits.get(app.selected_split) {
                Some(split) => split,
                None => {
                    return Err(Error::new(
                        "Split not selected".to_string(),
                        "None".to_string(),
                    ))
                }
            },
            None => {
                return Err(Error::new(
                    "Category not loaded".to_string(),
                    "None".to_string(),
                ))
            }
        };
        app.chrono
            .load_chrono(split.real_time, &app.config.chrono_format);
    }
    Ok(())
}

pub fn next_split(app: &mut HitSplit) -> Result<(), Error> {
    if let Some(category) = app.loaded_category.as_mut() {
        if category.splits.len() > app.selected_split + 1 {
            app.selected_split += 1;
            app.chrono.clear_elapsed();
        } else if app.config.next_split_as_reset {
            app.selected_split = 0;
            let pbs = category.splits.iter().map(|split| split.pb).sum::<u16>();
            let hits = category.splits.iter().map(|split| split.hits).sum::<u16>();
            if pbs > hits {
                set_pb(app)?;
            }
            reset(app)?;
        }
    }
    Ok(())
}

pub fn sub_hit(app: &mut HitSplit) -> Result<(), Error> {
    if let Some(category) = app.loaded_category.as_mut() {
        let split = match category.splits.get_mut(app.selected_split) {
            Some(s) => s,
            None => {
                return Err(Error::new(
                    "Could not substract hit. Split not selected.".to_string(),
                    "None".to_string(),
                ))
            }
        };
        if split.hits > 0 {
            split.hits -= 1;
        }
    }
    Ok(())
}

pub fn add_hit(app: &mut HitSplit) -> Result<(), Error> {
    if let Some(category) = app.loaded_category.as_mut() {
        match category.splits.get_mut(app.selected_split) {
            Some(split) => split.hits += 1,
            None => {
                return Err(Error::new(
                    "Could not add hit. Split not selected.".to_string(),
                    "None".to_string(),
                ))
            }
        };
    }
    Ok(())
}

pub fn reset(app: &mut HitSplit) -> Result<(), Error> {
    if let Some(category) = app.loaded_category.as_mut() {
        category.splits.iter_mut().for_each(|split| {
            split.hits = 0;
            split.real_time = Duration::default();
        });
        app.selected_split = 0;
        app.chrono.reset();
    }
    Ok(())
}

pub fn set_pb(app: &mut HitSplit) -> Result<(), Error> {
    if let Some(category) = app.loaded_category.as_mut() {
        category.splits.iter_mut().for_each(|split| {
            split.pb = split.hits;
        });
    }
    Ok(())
}

pub fn start_chrono(app: &mut HitSplit) -> Result<(), Error> {
    app.chrono.start();
    Ok(())
}

pub fn pause_chrono(app: &mut HitSplit) -> Result<(), Error> {
    app.chrono.pause()
}
