use std::time::Duration;

use egui::Color32;
use egui_extras::TableRow;
use serde::{Deserialize, Serialize};

use crate::{
    run::{
        chrono::{duration_chrono_format, ChronometerFormat},
        split::Split,
    },
    HitSplit,
};

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Column {
    Icon,
    SplitName,
    Hits,
    Difference,
    PersonalBest,
    Chrono,
    ChronoAcum,
}

impl Column {
    fn position(&self) -> usize {
        match self {
            Column::Icon => 0,
            Column::SplitName => 1,
            Column::Hits => 2,
            Column::Difference => 3,
            Column::PersonalBest => 4,
            Column::Chrono => 5,
            Column::ChronoAcum => 6,
        }
    }

    pub fn header(&self, app: &HitSplit, tr: &mut TableRow) {
        match self {
            Column::Icon => tr.col(|_ui| {}),
            Column::SplitName => tr.col(|ui| {
                ui.strong(format!(
                    "Split ({}/{})",
                    app.selected_split + 1,
                    app.num_splits_category
                ));
            }),
            Column::Hits => tr.col(|ui| {
                ui.strong("Hits");
            }),
            Column::Difference => tr.col(|ui| {
                ui.strong("Diff");
            }),
            Column::PersonalBest => tr.col(|ui| {
                ui.strong("PB");
            }),
            Column::Chrono => tr.col(|ui| {
                ui.strong("Chrono");
            }),
            Column::ChronoAcum => tr.col(|ui| {
                ui.strong("Chrono Ac.");
            }),
        };
    }

    pub fn body(
        &self,
        app: &HitSplit,
        index: usize,
        split: &Split,
        label_color: Color32,
        chrono_format: &ChronometerFormat,
        row: &mut TableRow,
    ) {
        match self {
            Column::Icon => {
                row.col(|ui| {
                    if let Some(p) = &split.icon_path {
                        let path = p.as_path().to_str().unwrap();
                        ui.add(
                            egui::Image::new(format!("file://{path}"))
                                .max_height(app.config.font_size),
                        );
                    }
                });
            }
            Column::SplitName => {
                let mut name = split.name.clone();
                if index == app.selected_split {
                    name = format!("> {}", name);
                }

                row.col(|ui| {
                    ui.colored_label(label_color, name);
                });
            }
            Column::Hits => {
                row.col(|ui| {
                    ui.colored_label(label_color, split.hits.to_string());
                });
            }
            Column::Difference => {
                row.col(|ui| {
                    ui.colored_label(
                        label_color,
                        (i32::from(split.hits) - i32::from(split.pb)).to_string(),
                    );
                });
            }
            Column::PersonalBest => {
                row.col(|ui| {
                    ui.colored_label(label_color, split.pb.to_string());
                });
            }
            Column::Chrono => {
                row.col(|ui| {
                    ui.colored_label(
                        label_color,
                        duration_chrono_format(split.real_time, chrono_format),
                    );
                });
            }
            Column::ChronoAcum => {
                row.col(|ui| {
                    if let Some(category) = &app.loaded_category {
                        let acum: Duration = category
                            .splits
                            .iter()
                            .take_while(|s| s.uuid != split.uuid)
                            .map(|s| s.real_time)
                            .sum();
                        ui.colored_label(
                            label_color,
                            duration_chrono_format(acum + split.real_time, chrono_format),
                        );
                    }
                });
            }
        }
    }

    pub fn total(&self, app: &HitSplit, label_color: Color32, row: &mut TableRow) {
        match self {
            Column::Icon => row.col(|_| {}),
            Column::SplitName => row.col(|ui| {
                ui.colored_label(label_color, "Total: ");
            }),
            Column::Hits => row.col(|ui| {
                let hits = match &app.loaded_category {
                    Some(category) => category.splits.iter().map(|split| split.hits).sum::<u16>(),
                    None => 0,
                };

                ui.colored_label(label_color, hits.to_string());
            }),
            Column::Difference => row.col(|ui| {
                let diff = match &app.loaded_category {
                    Some(category) => category
                        .splits
                        .iter()
                        .map(|split| i32::from(split.hits) - i32::from(split.pb))
                        .sum::<i32>(),
                    None => 0,
                };
                ui.colored_label(label_color, diff.to_string());
            }),
            Column::PersonalBest => row.col(|ui| {
                let pb = match &app.loaded_category {
                    Some(category) => category.splits.iter().map(|split| split.pb).sum::<u16>(),
                    None => 0,
                };
                ui.colored_label(label_color, pb.to_string());
            }),
            Column::Chrono => row.col(|ui| {
                if let Some(category) = &app.loaded_category {
                    let acum: Duration = category.splits.iter().map(|s| s.real_time).sum();
                    let time: String = duration_chrono_format(acum, &app.config.chrono_format);
                    ui.colored_label(label_color, time);
                }
            }),
            Column::ChronoAcum => row.col(|ui| {
                if let Some(category) = &app.loaded_category {
                    let acum: Duration = category.splits.iter().map(|s| s.real_time).sum();
                    let time: String = duration_chrono_format(acum, &app.config.chrono_format);
                    ui.colored_label(label_color, time);
                }
            }),
        };
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColumnVec(Vec<Column>);

impl ColumnVec {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<Column> {
        self.0.iter()
    }

    pub fn contains(&self, column: &Column) -> bool {
        self.0.contains(column)
    }

    fn to_mask(&self) -> usize {
        self.iter().map(|c| 1 << c.position()).sum()
    }

    pub fn push(&mut self, column: &Column) {
        let mask = self.to_mask();
        let index = (0..column.position()).map(|i| 1 & (mask >> i)).sum();
        self.0.insert(index, *column);
        self.0.dedup();
    }

    pub fn remove(&mut self, column: &Column) {
        self.0.retain(|c| c != column);
    }
}

impl Default for ColumnVec {
    fn default() -> Self {
        Self(vec![
            Column::Icon,
            Column::SplitName,
            Column::Hits,
            Column::Difference,
            Column::PersonalBest,
            Column::Chrono,
            Column::ChronoAcum,
        ])
    }
}
