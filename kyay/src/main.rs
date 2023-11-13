use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use eframe::{egui, App, AppCreator, CreationContext, NativeOptions};
use kyay::{Definition, Dictionary, DictionaryEntry};
use wa::Ipa;

enum HistoryItem {
    AddWord(wa::Syllable),
    RemoveWord {
        old: DictionaryEntry,
    },
    ModifyWord {
        old: DictionaryEntry,
    },
    RenameWord {
        from: wa::Syllable,
        to: wa::Syllable,
    },
}

struct KyayApp {
    path: PathBuf,
    selected: Option<wa::Syllable>,
    dictionary: kyay::Dictionary,
    history: Vec<HistoryItem>,

    add_word_text: String,
    add_tag_text: String,
}

impl KyayApp {
    pub fn save_to_json(&self) {
        let writer = BufWriter::new(
            OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(&self.path)
                .expect("Could not open Dictionary JSON file for writing."),
        );

        serde_json::to_writer_pretty(writer, &self.dictionary)
            .expect("Could not save to Dictionary JSON file.");
    }

    pub fn get_word(&mut self, word: &wa::Syllable) -> Option<DictionaryEntry> {
        self.dictionary
            .get(word)
            .map(|defs| DictionaryEntry(*word, defs.clone()))
    }

    pub fn add_word(&mut self, word: wa::Syllable) {
        if let std::collections::hash_map::Entry::Vacant(entry) = self.dictionary.entry(word) {
            entry.insert(vec![]);
            self.history.push(HistoryItem::AddWord(word))
        }
    }

    pub fn remove_word(&mut self, word: &wa::Syllable) -> Option<Vec<Definition>> {
        if let Some(entry) = self.dictionary.remove(word) {
            self.history.push(HistoryItem::RemoveWord {
                old: DictionaryEntry(*word, entry.clone()),
            });

            if self.selected == Some(*word) {
                self.selected = None;
            }

            return Some(entry);
        }

        None
    }

    pub fn modify_word(&mut self, word: &wa::Syllable, f: impl FnOnce(&mut Vec<Definition>)) {
        if let Some(defs) = self.dictionary.get_mut(word) {
            let old = DictionaryEntry(*word, defs.clone());
            f(defs);

            if &old.1 != defs {
                self.history.push(HistoryItem::ModifyWord { old })
            }
        }
    }

    pub fn rename_word(&mut self, from: &wa::Syllable, to: &wa::Syllable) {
        let from_defs = self.dictionary.remove(from);
        let to_defs = self.dictionary.remove(to);

        if let Some(from_defs) = from_defs {
            self.dictionary.insert(*to, from_defs);
        }

        if let Some(to_defs) = to_defs {
            self.dictionary.insert(*from, to_defs);
        }

        self.history.push(HistoryItem::RenameWord {
            from: *from,
            to: *to,
        });

        if self.selected == Some(*from) {
            self.selected = Some(*to);
        }
    }

    pub fn add_def(&mut self, word: &wa::Syllable, definition: Definition) {
        self.modify_word(word, |defs| defs.push(definition))
    }

    pub fn remove_def(&mut self, word: &wa::Syllable, index: usize) {
        self.modify_word(word, |defs| {
            if index < defs.len() {
                defs.remove(index);
            }
        })
    }

    pub fn move_def(&mut self, word: &wa::Syllable, from_index: usize, to_index: usize) {
        self.modify_word(word, |defs| {
            if from_index < defs.len() && to_index < defs.len() {
                defs.swap(from_index, to_index);
            }
        })
    }

    pub fn undo(&mut self) {
        if let Some(item) = self.history.pop() {
            match item {
                HistoryItem::AddWord(word) => {
                    self.dictionary.remove(&word);
                    if self.selected == Some(word) {
                        self.selected = None;
                    }
                }
                HistoryItem::RemoveWord { old } | HistoryItem::ModifyWord { old } => {
                    self.dictionary.insert(old.0, old.1);
                }

                HistoryItem::RenameWord { from, to } => {
                    let from_defs = self.dictionary.remove(&from);
                    let to_defs = self.dictionary.remove(&to);

                    if let Some(from_defs) = from_defs {
                        self.dictionary.insert(to, from_defs);
                    }

                    if let Some(to_defs) = to_defs {
                        self.dictionary.insert(from, to_defs);
                    }
                }
            }
        }
    }
}

impl App for KyayApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if ui
                    .add(
                        egui::TextEdit::singleline(&mut self.add_word_text)
                            .hint_text("Add word..."),
                    )
                    .lost_focus()
                    && ctx.input(|input| input.key_pressed(egui::Key::Enter))
                {
                    if let Ok(word) = self.add_word_text.parse::<wa::Syllable>() {
                        self.add_word_text = String::new();

                        self.add_word(word);
                    }
                }

                ui.separator();

                if ui.button("Save").clicked() {
                    self.save_to_json();
                }

                if ui.button("Undo").clicked() {
                    self.undo();
                }
            });
        });

        egui::SidePanel::new(egui::panel::Side::Left, "words_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for word in self.dictionary.keys() {
                    let word_txt = word.to_string();

                    let is_selected = Some(*word) == self.selected;

                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                        if ui.selectable_label(is_selected, word_txt).clicked() {
                            self.selected = Some(*word);
                        }
                    });
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(selected) = self.selected {
                    ui.label(
                        egui::RichText::new(selected.to_string())
                            .font(egui::FontId::proportional(40.0)),
                    );
                    ui.label(format!("[{}]", selected.ipa()));

                    ui.separator();

                    if let Some(defs) = self.dictionary.get(&selected).cloned() {
                        for (i, def) in defs.into_iter().enumerate() {
                            ui.with_layout(
                                egui::Layout::top_down_justified(egui::Align::Min),
                                |ui| {
                                    egui::Frame::none()
                                        .stroke(egui::Stroke::new(
                                            3.0,
                                            egui::Color32::from_rgb(220, 220, 220),
                                        ))
                                        .inner_margin(5.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                for (j, tag) in def.tags.iter().enumerate() {
                                                    ui.label(
                                                        egui::RichText::new(tag)
                                                            .font(egui::FontId::proportional(10.0)),
                                                    );
                                                    if ui.button("-").clicked() {
                                                        self.modify_word(&selected, |defs| {
                                                            defs[i].tags.remove(j);
                                                        })
                                                    }
                                                }

                                                ui.separator();

                                                if ui
                                                    .add(
                                                        egui::TextEdit::singleline(
                                                            &mut self.add_tag_text,
                                                        )
                                                        .hint_text("Add tag..."),
                                                    )
                                                    .lost_focus()
                                                    && ctx.input(|input| {
                                                        input.key_pressed(egui::Key::Enter)
                                                    })
                                                {
                                                    let text = self.add_tag_text.clone();
                                                    self.modify_word(&selected, |def| {
                                                        def[i].tags.push(text);
                                                    });

                                                    self.add_tag_text = String::new();
                                                }
                                            });

                                            let mut text = def.definition.clone();

                                            if ui.text_edit_multiline(&mut text).changed() {
                                                self.modify_word(&selected, |defs| {
                                                    defs[i].definition = text;
                                                });
                                            };

                                            if ui.button("Remove this definition").clicked() {
                                                self.remove_def(&selected, i);
                                            }
                                        });
                                },
                            );
                        }

                        if ui.button("Add Definition").clicked() {
                            self.add_def(
                                &selected,
                                Definition {
                                    tags: Vec::new(),
                                    definition: String::new(),
                                },
                            )
                        }
                    }
                }
            })
        });
    }
}

fn main() {
    env_logger::init();

    let mut args = std::env::args();

    let path = PathBuf::from(args.nth(1).expect("No path specified."));
    let dictionary = if path.exists() {
        serde_json::from_reader(BufReader::new(
            File::open(&path).expect("Could not open Dictionary JSON file."),
        ))
        .expect("Could not read Dictionary JSON file.")
    } else {
        Dictionary::new()
    };

    eframe::run_native(
        "kyáy-pó pyá-pó zẽy-zẽ-wũ",
        NativeOptions {
            ..Default::default()
        },
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();

            // Install my own font (maybe supporting non-latin characters):
            let mut font_data =
                egui::FontData::from_static(include_bytes!("../../fonts/Arial.ttf"));
            fonts.font_data.insert("my_font".to_owned(), font_data);

            // Put my font first (highest priority):
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "my_font".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            cc.egui_ctx
                .set_pixels_per_point(cc.egui_ctx.pixels_per_point() * 2.0);

            Box::new(KyayApp {
                path,
                dictionary,
                history: Vec::new(),
                selected: None,
                add_word_text: String::new(),
                add_tag_text: String::new(),
            })
        }),
    )
    .expect("Could not start eframe.");
}
