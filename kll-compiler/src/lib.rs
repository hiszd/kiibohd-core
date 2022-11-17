#![feature(if_let_guard)]
#![feature(map_try_insert)]
#![allow(incomplete_features)]

pub mod emitters;
pub mod parser;
mod test;
pub mod types;

#[macro_use]
extern crate derive_object_merge;

use object_merge::Merge;
pub use parser::parse_int;
use parser::PestError;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
pub use types::{
    Action, Animation, AnimationResult, Capability, Key, KllFile, Mapping, PixelDef, Position,
    ResultList, ResultType, Statement, Trigger, TriggerList, TriggerMode, TriggerType,
};

pub use layouts_rs::Layouts;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value<'a> {
    List(Vec<&'a str>),
    Single(&'a str),
}

#[derive(Debug, Default, Clone, Merge)]
pub struct KllState<'a> {
    #[combine]
    pub defines: HashMap<&'a str, &'a str>,
    #[combine]
    pub variables: HashMap<&'a str, Value<'a>>,
    #[combine]
    pub capabilities: HashMap<&'a str, Capability<'a>>,
    #[combine]
    pub keymap: Vec<Mapping<'a>>,
    #[combine]
    pub positions: HashMap<usize, Position>,
    #[combine]
    pub pixelmap: HashMap<usize, PixelDef>,
    #[combine]
    pub animations: HashMap<&'a str, Animation<'a>>,
}

impl<'a> KllFile<'a> {
    pub fn into_struct(self) -> KllState<'a> {
        let mut kll = KllState::default();
        for statement in self.statements {
            match statement {
                Statement::Define((name, val)) => {
                    kll.defines.insert(name, val);
                }
                Statement::Variable((name, index, val)) => {
                    let entry = kll.variables.entry(name).or_insert_with(|| match index {
                        Some(_) => Value::List(vec![]),
                        None => Value::Single(val),
                    });
                    match entry {
                        Value::List(vec) => {
                            let index = index.unwrap(); // Should be set because this is an array
                            if index >= vec.len() {
                                vec.resize(index + 1, "");
                            }
                            vec[index] = val;
                        }
                        Value::Single(s) => {
                            *s = val;
                        }
                    };
                }
                Statement::Capability((name, cap)) => {
                    kll.capabilities.insert(name, cap);
                }
                Statement::Keymap(mapping) => {
                    kll.keymap.push(mapping);
                }
                Statement::Position((indices, pos)) => {
                    for range in indices {
                        for index in range {
                            kll.positions.insert(index, pos.clone());
                        }
                    }
                }
                Statement::Pixelmap((indices, map)) => {
                    for range in indices {
                        for index in range {
                            kll.pixelmap.insert(index, map.clone());
                        }
                    }
                }
                Statement::Animation((name, anim)) => {
                    kll.animations.insert(name, anim);
                }
                Statement::Frame((name, indices, frame)) => {
                    let animation = kll.animations.entry(name).or_default();
                    let frames = &mut animation.frames;
                    for range in indices {
                        for index in range {
                            if frames.len() <= index {
                                frames.resize(index + 1, vec![]);
                            }
                            frames[index] = frame.clone();
                        }
                    }
                }
                Statement::NOP => {}
            };
        }

        kll
    }
}

impl<'a> KllState<'a> {
    pub fn triggers(&self) -> impl Iterator<Item = &Trigger> + '_ {
        let groups = self
            .keymap
            .iter()
            .map(|Mapping(trigger_groups, _, _)| trigger_groups);
        let combos = groups.into_iter().map(|tl| tl.iter());
        combos.into_iter().flatten()
    }

    pub fn trigger_lists(&self) -> impl Iterator<Item = &TriggerList> + '_ {
        self.keymap
            .iter()
            .map(|Mapping(trigger_groups, _, _)| trigger_groups)
    }

    pub fn actions(&self) -> impl Iterator<Item = &Action> + '_ {
        let groups = self
            .keymap
            .iter()
            .map(|Mapping(_, _, result_groups)| result_groups);
        let combos = groups.into_iter().map(|rl| rl.iter());
        combos.into_iter().flatten()
    }

    pub fn result_lists(&self) -> impl Iterator<Item = &ResultList> + '_ {
        self.keymap
            .iter()
            .map(|Mapping(_, _, result_groups)| result_groups)
    }

    pub fn trigger_result_lists(&self) -> impl Iterator<Item = (&TriggerList, &ResultList)> + '_ {
        self.keymap
            .iter()
            .map(|Mapping(trigger_groups, _, result_groups)| (trigger_groups, result_groups))
    }

    pub fn scancode_map(&self) -> HashMap<&str, usize> {
        self.keymap
            .iter()
            .filter_map(|Mapping(trigger_groups, _, result_groups)| match 1 {
                _ if trigger_groups.iter().count() == 1 && result_groups.iter().count() == 1 => match 1 {
                    _ if let (TriggerType::Key(Key::Scancode(s)), ResultType::Output(Key::Usb(u))) = (&trigger_groups.iter().next().unwrap().trigger, &result_groups.iter().next().unwrap().result)  => Some((*u, *s)),
                    _ => None,
                },
                _ => None
            })
        .collect::<HashMap<&str, usize>>()
    }

    pub fn scancodes(&self) -> Vec<usize> {
        self.triggers()
            .filter_map(|t| match &t.trigger {
                TriggerType::Key(key) => Some(key),
                _ => None,
            })
            .filter_map(|key| match key {
                Key::Scancode(s) => Some(*s),
                _ => None,
            })
            .collect()
    }

    pub fn animations(&self) -> impl Iterator<Item = &AnimationResult> + '_ {
        self.actions().filter_map(|action| match &action.result {
            ResultType::Animation(anim) => Some(anim),
            _ => None,
        })
    }

    pub fn unicode_strings(&self) -> HashSet<String> {
        self.actions()
            .filter_map(|action| match &action.result {
                ResultType::UnicodeText(text) => Some(text.to_string()),
                _ => None,
            })
            .collect()
    }

    pub fn reduce(&self, base: KllState<'a>) -> Vec<Mapping<'a>> {
        let scancode_map = base.scancode_map();
        let mut new_keymap: Vec<Mapping> = self
            .keymap
            .iter()
            .map(|Mapping(trigger_groups, mode, result_groups)| {
                let new_triggers = TriggerList(match mode {
                    TriggerMode::SoftReplace => trigger_groups.0.clone(),
                    _ => trigger_groups
                        .0
                        .iter()
                        .map(|combo| {
                            combo
                                .iter()
                                .map(|t| match &t.trigger {
                                    TriggerType::Key(Key::Usb(u)) => {
                                        let s = scancode_map.get(u).unwrap();
                                        Trigger {
                                            trigger: TriggerType::Key(Key::Scancode(*s)),
                                            state: t.state.clone(),
                                        }
                                    }
                                    _ => t.clone(),
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                });
                let new_results = ResultList(match mode {
                    TriggerMode::SoftReplace => result_groups.0.clone(),
                    _ => result_groups
                        .0
                        .iter()
                        .map(|combo| {
                            combo
                                .iter()
                                .map(|r| match &r.result {
                                    ResultType::Output(Key::Usb(u)) => Action {
                                        result: ResultType::Capability((
                                            Capability::new("usbKeyOut", vec![u]),
                                            None,
                                        )),
                                        state: r.state.clone(),
                                    },
                                    _ => r.clone(),
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                });

                Mapping(new_triggers, mode.clone(), new_results)
            })
            .collect::<Vec<_>>();

        new_keymap.sort_by(|a, b| {
            let a = format!("{}", a);
            let b = format!("{}", b);
            alphanumeric_sort::compare_path(a, b)
        });

        new_keymap
    }

    /// Replaces any implied (KLL) state with explicit state
    pub fn generate_state_scheduling(&mut self) {
        // Generate explicit state for each Mapping
        // Determine if we should use Result/Option for implied_state (probably option?)
        // Build new keymap Vector with new keymapping
        let mut new_keymap = Vec::new();
        for mapping in &self.keymap {
            if let Some(mut mapping_vec) = mapping.implied_state() {
                new_keymap.append(&mut mapping_vec);
            } else {
                new_keymap.push(mapping.clone());
            }
        }

        self.keymap = new_keymap;
    }
}

#[derive(Debug, Default, Clone)]
pub struct KllDatastore<'a> {
    pub scancode_range: Range<usize>,
    pub unicode_strings: HashSet<String>,
    pub unique_triggers: HashSet<Trigger<'a>>,
    pub unique_results: HashSet<Action<'a>>,
    pub unique_animations: HashSet<AnimationResult<'a>>,
}

impl<'a> KllDatastore<'a> {
    pub fn get_scancode_range(state: &KllState) -> Range<usize> {
        let mut range = Range {
            start: 0xFFFF,
            end: 0,
        };

        for scancode in state.scancodes() {
            if scancode < range.start {
                range.start = scancode;
            }
            if scancode > range.end {
                range.end = scancode;
            }
        }

        if range.start == 0xFFFF {
            range.start = 0; // No keys found
        }

        assert!(range.start <= range.end);
        range
    }

    pub fn new(state: &'a KllState<'a>) -> KllDatastore<'a> {
        KllDatastore {
            unicode_strings: state.unicode_strings(),
            scancode_range: KllDatastore::get_scancode_range(state),
            unique_triggers: state.triggers().cloned().collect(),
            unique_results: state.actions().cloned().collect(),
            unique_animations: state.animations().cloned().collect(),
        }
    }
}

#[allow(clippy::result_large_err)]
pub fn parse(text: &str) -> Result<KllFile, PestError> {
    KllFile::from_str(text)
}

// Holds owned version of all files
// All other data structures are borrowed from this
pub struct Filestore {
    files: HashMap<PathBuf, String>,
}

impl Filestore {
    pub fn new() -> Self {
        Filestore {
            files: HashMap::new(),
        }
    }
    pub fn load_file(&mut self, path: &Path) {
        //dbg!(&path);
        let raw_text = fs::read_to_string(path).expect("cannot read file");
        self.files.insert(path.to_path_buf(), raw_text);
    }

    pub fn get_file<'a>(&'a self, path: &Path) -> KllState<'a> {
        let raw_text = self.files.get(path).unwrap();
        parse(raw_text).unwrap().into_struct()
    }
}

impl Default for Filestore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct KllGroups<'a> {
    config: Vec<KllState<'a>>,
    base: Vec<KllState<'a>>,
    default: Vec<KllState<'a>>,
    partials: Vec<KllState<'a>>,
}

impl<'a> KllGroups<'a> {
    pub fn new(
        filestore: &'a Filestore,
        config: &[PathBuf],
        base: &[PathBuf],
        default: &[PathBuf],
        partials: &[PathBuf],
    ) -> Self {
        KllGroups {
            config: config.iter().map(|p| filestore.get_file(p)).collect(),
            base: base.iter().map(|p| filestore.get_file(p)).collect(),
            default: default.iter().map(|p| filestore.get_file(p)).collect(),
            partials: partials.iter().map(|p| filestore.get_file(p)).collect(),
        }
    }

    pub fn config(&self) -> KllState<'a> {
        let mut configs = self.config.iter();
        let mut config = configs.next().unwrap().clone();
        for c in configs {
            config.merge(c);
        }
        config
    }

    pub fn basemap(&self) -> KllState<'a> {
        let mut layouts = self.base.iter();
        let mut layout = layouts.next().unwrap().clone();
        for base in layouts {
            layout.merge(base);
        }
        layout
    }
    pub fn defaultmap(&self) -> KllState<'a> {
        let mut layout = self.basemap();
        for default in &self.default {
            layout.merge(default);
        }
        layout
    }
    pub fn partialmaps(&self) -> Vec<KllState<'a>> {
        let mut partials: Vec<KllState> = vec![];
        for partial in &self.partials {
            let mut layout = self.basemap();
            layout.merge(partial);
            partials.push(layout);
        }

        partials
    }
    /// Combines default and partialmaps as a list of layers (defaultmap being 0)
    pub fn layers(&self) -> Vec<KllState<'a>> {
        let mut layers: Vec<KllState> = vec![self.defaultmap()];
        for layer in self.partialmaps() {
            layers.push(layer);
        }

        layers
    }
}
