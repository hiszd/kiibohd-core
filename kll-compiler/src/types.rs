use layouts_rs::{Layout, Layouts};
use std::collections::HashMap;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

pub type Index = Range<usize>;
pub type Indices = Vec<Index>;
pub type Map<'a> = HashMap<&'a str, &'a str>;

#[derive(Debug, Clone)]
pub enum Error {
    UnknownMatch { s: String },
}

pub fn format_indices(ranges: &[Index]) -> String {
    ranges
        .iter()
        .map(|range| format!("{}-{}", range.start, range.end))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn maybe_quote(text: &str) -> String {
    if text.contains(' ') {
        format!("\"{}\"", text)
    } else {
        text.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Mapping<'a>(pub TriggerList<'a>, pub TriggerMode, pub ResultList<'a>);

impl<'a> Mapping<'a> {
    pub fn implied_state(&self) -> Option<Vec<Self>> {
        // TODO Handle other combinations of implied state
        if let Some(triggerlists) = self.0.implied_state() {
            if let Some(resultlists) = self.2.implied_state() {
                // TODO Allow for other combinations other than just simple cases
                if triggerlists.len() == 2 && resultlists.len() == 2 {
                    Some(vec![
                        Self(
                            triggerlists[0].clone(),
                            self.1.clone(),
                            resultlists[0].clone(),
                        ),
                        Self(
                            triggerlists[1].clone(),
                            self.1.clone(),
                            resultlists[1].clone(),
                        ),
                    ])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> fmt::Display for Mapping<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

/// Intermediate data structure used as a hashable key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TriggerConditionList(pub Vec<Vec<kll_core::TriggerCondition>>);

#[derive(Debug, Clone)]
pub struct TriggerList<'a>(pub Vec<Vec<Trigger<'a>>>);

impl<'a> TriggerList<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &Trigger> + '_ {
        self.0.iter().flatten()
    }

    /// Converts the TriggerList into a kll-core trigger guide
    /// NOTE: The result of this type is *not* safely hashable
    ///       as the binary format can change due to internal rust
    ///       behaviour. Please use kll_core_capability_guide instead.
    pub fn kll_core_guide(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for combo in &self.0 {
            // Push the length of the combo
            buf.push(combo.len() as u8);
            // Push each combo element
            for elem in combo {
                unsafe {
                    buf.extend_from_slice(elem.kll_core_condition().bytes());
                }
            }
        }
        // Push final 0-length combo to indicate sequence has finished
        buf.push(0);
        buf
    }

    /// Converts the TriggerList into a kll-core result capability guide
    /// This type is safely hashable
    pub fn kll_core_condition_guide(&self) -> TriggerConditionList {
        let mut sequence_buf = Vec::new();
        for combo in &self.0 {
            let mut combo_buf = Vec::new();
            // Push each combo element
            for elem in combo {
                combo_buf.push(elem.kll_core_condition());
            }
            sequence_buf.push(combo_buf);
        }
        TriggerConditionList(sequence_buf)
    }

    fn implied_state(&self) -> Option<Vec<Self>> {
        // TODO
        // Return permutations of implied TriggerList states
        // TODO
        // S1 : U"A"; => S1(P) : U"A"(P); S1(R) : U"A"(R);
        assert!(
            self.0.len() == 1,
            "TriggerList must only have 1 sequence element. (may not be implemented yet)"
        );
        assert!(
            self.0[0].len() == 1,
            "TriggerList must only have 1 combo element. (feature may not be implemented yet)"
        );
        self.0[0][0].implied_state().map(|triggers| {
            vec![
                Self(vec![vec![triggers[0].clone()]]),
                Self(vec![vec![triggers[1].clone()]]),
            ]
        })
    }
}

impl<'a> fmt::Display for TriggerList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|combo| combo
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(" + "))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// Intermediate data structure used as a hashable key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResultCapabilitiesList(pub Vec<Vec<kll_core::Capability>>);

#[derive(Debug, Clone)]
pub struct ResultList<'a>(pub Vec<Vec<Action<'a>>>);

impl<'a> ResultList<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &Action> + '_ {
        self.0.iter().flatten()
    }

    /// Converts the ResultList into a kll-core result guide
    /// NOTE: The result of this type is *not* safely hashable
    ///       as the binary format can change due to internal rust
    ///       behaviour. Please use kll_core_capability_guide instead.
    pub fn kll_core_guide(&self, layouts: &mut Layouts) -> Vec<u8> {
        let mut buf = Vec::new();
        for combo in &self.0 {
            // Push the length of the combo
            buf.push(combo.len() as u8);
            // Push each combo element
            for elem in combo {
                unsafe {
                    buf.extend_from_slice(elem.kll_core_condition(layouts).bytes());
                }
            }
        }
        // Push final 0-length combo to indicate sequence has finished
        buf.push(0);
        buf
    }

    /// Converts the ResultList into a kll-core result capability guide
    /// This type is safely hashable
    pub fn kll_core_capability_guide(&self, layouts: &mut Layouts) -> ResultCapabilitiesList {
        let mut sequence_buf = Vec::new();
        for combo in &self.0 {
            let mut combo_buf = Vec::new();
            // Push each combo element
            for elem in combo {
                combo_buf.push(elem.kll_core_condition(layouts));
            }
            sequence_buf.push(combo_buf);
        }
        ResultCapabilitiesList(sequence_buf)
    }

    fn implied_state(&self) -> Option<Vec<Self>> {
        // TODO
        // Return permutations of implied ResultList states
        // TODO
        // S1 : U"A"; => S1(P) : U"A"(P); S1(R) : U"A"(R);
        assert!(
            self.0.len() == 1,
            "ResultList must only have 1 sequence element. (may not be implemented yet)"
        );
        assert!(
            self.0[0].len() == 1,
            "ResultList must only have 1 combo element. (feature may not be implemented yet)"
        );
        let results = self.0[0][0].implied_state().unwrap();
        Some(vec![
            Self(vec![vec![results[0].clone()]]),
            Self(vec![vec![results[1].clone()]]),
        ])
    }
}

impl<'a> fmt::Display for ResultList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|combo| combo
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(" + "))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Define((&'a str, &'a str)),
    Variable((&'a str, Option<usize>, &'a str)),
    Capability((&'a str, Capability<'a>)),
    Keymap(Mapping<'a>),
    Position((Indices, Position)),
    Pixelmap((Indices, PixelDef)),
    Animation((&'a str, Animation<'a>)),
    Frame((&'a str, Indices, Vec<Pixel<'a>>)),
    NOP,
}

impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Define((name, val)) => write!(f, "{} = {};", name, maybe_quote(val)),
            Self::Variable((name, index, val)) => {
                if let Some(index) = index {
                    write!(
                        f,
                        "{}[{}] = {};",
                        maybe_quote(name),
                        index,
                        maybe_quote(val)
                    )
                } else {
                    write!(f, "{} = {};", maybe_quote(name), maybe_quote(val))
                }
            }
            Self::Capability((name, cap)) => write!(f, "{} = {};", name, cap),
            Self::Keymap(mapping) => write!(f, "{};", mapping),
            Self::Position((indices, pos)) => {
                write!(f, "P[{}] <= {};", format_indices(indices), pos)
            }
            Self::Pixelmap((indices, map)) => write!(
                f,
                "P[{}]{} : {};",
                format_indices(indices),
                map.channels
                    .iter()
                    .map(|(c, w)| format!("{}:{}", c, w))
                    .collect::<Vec<String>>()
                    .join(", "),
                map.scancode
                    .map(|x| format!("S{}", x))
                    .unwrap_or_else(|| "None".to_string())
            ),
            Self::Animation((name, anim)) => write!(f, "A[{}] <= {:?};", name, anim.modifiers),
            Self::Frame((name, indices, frame)) => write!(
                f,
                "A[{}, {}] <= {:?};",
                name,
                format_indices(indices),
                frame
            ),
            Self::NOP => Ok(()),
        }
    }
}

#[derive(Debug, Default, Clone, Merge)]
pub struct Position {
    pub x: f32,  // mm
    pub y: f32,  // mm
    pub z: f32,  // mm
    pub rx: f32, // deg
    pub ry: f32, // deg
    pub rz: f32, // deg
}

impl Position {
    pub fn from_map(map: Map) -> Self {
        let mut pos = Position::default();
        for (k, v) in map.iter() {
            let v = v.parse::<f32>().unwrap();
            match *k {
                "x" => pos.x = v,
                "y" => pos.y = v,
                "z" => pos.z = v,
                "rx" => pos.rx = v,
                "ry" => pos.ry = v,
                "rz" => pos.rz = v,
                _ => {}
            }
        }

        pos
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.x != 0. {
            write!(f, "x:{}", self.x)?;
        }
        if self.y != 0. {
            write!(f, "y:{}", self.y)?;
        }
        if self.z != 0. {
            write!(f, "z:{}", self.z)?;
        }
        if self.rx != 0. {
            write!(f, "x:{}", self.rx)?;
        }
        if self.ry != 0. {
            write!(f, "y:{}", self.ry)?;
        }
        if self.rz != 0. {
            write!(f, "z:{}", self.rz)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Merge)]
pub struct PixelDef {
    #[combine]
    pub channels: Vec<(usize, usize)>,
    pub scancode: Option<usize>,
}

impl PixelDef {
    pub fn new(channelmap: Map, scancode: Option<usize>) -> Self {
        let channels = channelmap
            .iter()
            .map(|(k, v)| {
                let k = k.parse::<usize>().unwrap();
                let v = v.parse::<usize>().unwrap();
                (k, v)
            })
            .collect::<Vec<_>>();

        PixelDef { scancode, channels }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Animation<'a> {
    pub modifiers: Map<'a>,
    pub frames: Vec<Vec<Pixel<'a>>>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Capability<'a> {
    pub function: &'a str,
    //pub args: Map<'a>, // XXX: Can't hash a HashMap
    pub args: Vec<&'a str>,
}

impl<'a> Capability<'a> {
    pub fn new(function: &'a str, args: Vec<&'a str>) -> Self {
        Capability { function, args }
    }
}

impl<'a> fmt::Display for Capability<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({:?})",
            self.function,
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LayerMode {
    Normal,
    Shift,
    Latch,
    Lock,
}

impl FromStr for LayerMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Layer" => Self::Normal,
            "LayerShift" => Self::Shift,
            "LayerLatch" => Self::Latch,
            "LayerLock" => Self::Lock,
            _ => {
                return Err(Error::UnknownMatch { s: s.to_string() });
            }
        })
    }
}

impl fmt::Display for LayerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "Layer"),
            Self::Shift => write!(f, "LayerShift"),
            Self::Latch => write!(f, "LayerLatch"),
            Self::Lock => write!(f, "LayerLock"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TriggerType<'a> {
    Key(Key<'a>),
    Layer((LayerMode, Indices)),
    Indicator(Indices),
    Generic((usize, usize, Option<usize>)),
    Animation(&'a str),
}

impl<'a> fmt::Display for TriggerType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(trigger) => write!(f, "{}", trigger),
            Self::Layer((mode, layer)) => write!(f, "{}[{}]", mode, format_indices(layer)),
            Self::Indicator(indicators) => {
                if indicators.len() > 1 {
                    write!(f, "I[{}]", format_indices(indicators))
                } else {
                    write!(f, "I{}", format_indices(indicators))
                }
            }
            Self::Generic((bank, index, param)) => {
                if let Some(param) = &param {
                    write!(f, "T[{}, {}]({})", bank, index, param)
                } else {
                    write!(f, "T[{}, {}]", bank, index)
                }
            }
            Self::Animation(name) => write!(f, "A[{}]", name),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Trigger<'a> {
    pub trigger: TriggerType<'a>,
    pub state: Option<StateMap>,
}

impl<'a> Trigger<'a> {
    /// Converts to a kll-core TriggerCondition
    ///
    /// If no scheduling is defined, automatically generate
    /// state scheduling parameters.
    /// e.g. For S1 : U"A";
    ///         S1(P) : U"A"(P);
    ///         S1(R) : U"A"(R);
    /// kll-core does not automatically deduce states like the original
    /// controller firmware did.
    /// TODO ^ Use a kll-compiler function to automatically duplicate so we don't have to do it
    /// here.
    pub fn kll_core_condition(&self) -> kll_core::TriggerCondition {
        // State must be defined
        // generate_state_scheduling() function can be used to compute if
        // it's not defined.
        assert!(self.state.is_some(), "state *must* be defined, use generate_state_scheduling() to convert implied state into implicit state.");
        assert!(self.state.as_ref().unwrap().states.len() == 1, "StateMap *must* only have a single state defined, need to expand into a sequence first.");

        match &self.trigger {
            TriggerType::Key(key) => {
                match key {
                    Key::Scancode(index) => {
                        kll_core::TriggerCondition::Switch {
                            state: self.state.as_ref().unwrap().states[0].kind.phro(),
                            index: *index as u16,
                            loop_condition_index: 0, // TODO
                        }
                    }
                    // NOTE: Only Scancodes are valid here
                    //       The compiler should have turned everything
                    //       into scancodes at this point.
                    _ => kll_core::TriggerCondition::None,
                }
            }
            TriggerType::Layer((_mode, _indices)) => {
                panic!("Missing Layer");
                /*
                kll_core::TriggerCondition::Layer {
                    state: kll_core::trigger::LayerState::ShiftActivate, // TODO compute
                    loop_condition_index: 0,                             // TODO
                    layer: 0,                                            // TODO
                }
                */
            }
            TriggerType::Indicator(_indices) => {
                panic!("Missing indicator");
                /*
                kll_core::TriggerCondition::HidLed {
                    state: kll_core::trigger::Aodo::Activate, // TODO compute
                    loop_condition_index: 0,                  // TODO
                    index: 0,                                 // TODO
                }
                */
            }
            TriggerType::Generic((_bank, _index, _param)) => {
                panic!("Missing Generic");
                // TODO
                //kll_core::TriggerCondition::None
            }
            TriggerType::Animation(_name) => {
                panic!("Missing Animation");
                /*
                kll_core::TriggerCondition::Animation {
                    state: kll_core::trigger::Dro::Done, // TODO compute
                    index: 0,                            // TODO
                    loop_condition_index: 0,             // TODO
                }
                */
            }
        }
    }

    /// Generates state scheduling from implied state
    /// Converts S1 : U"A"; to (trigger part)
    ///    S1(P) : U"A"(P);
    ///    S1(R) : U"A"(R);
    fn implied_state(&self) -> Option<Vec<Self>> {
        // No state (implied state), generate new triggers
        if self.state.is_none() {
            Some(vec![
                Self {
                    trigger: self.trigger.clone(),
                    state: Some(StateMap::new(vec![State {
                        kind: StateType::Press,
                        time: None,
                    }])),
                },
                Self {
                    trigger: self.trigger.clone(),
                    state: Some(StateMap::new(vec![State {
                        kind: StateType::Release,
                        time: None,
                    }])),
                },
            ])
        } else {
            None
        }
    }
}

impl<'a> fmt::Display for Trigger<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(state) = &self.state {
            write!(f, "{}({})", self.trigger, state)
        } else {
            write!(f, "{}", self.trigger)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State {
    pub kind: StateType,
    pub time: Option<usize>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(time) = self.time {
            write!(f, "{}:{}", self.kind, time)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct StateMap {
    pub states: Vec<State>,
}

impl StateMap {
    pub fn new(states: Vec<State>) -> Self {
        Self { states }
    }

    pub fn from_map(map: Map) -> Result<Self, Error> {
        let mut states = vec![];
        for (k, v) in map.iter() {
            let mut state = State {
                kind: StateType::from_str(k)?,
                time: None,
            };
            if let Ok(v) = v.parse::<usize>() {
                state.time = Some(v);
            }
            states.push(state);
        }

        Ok(StateMap { states })
    }
}

impl fmt::Display for StateMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.states
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum StateType {
    // Key
    Press,         // (P)
    Hold,          // (H)
    Release,       // (R)
    Unpressed,     // (O)
    UniquePress,   // (UP)
    UniqueRelease, // (UR)
    Analog(usize), // 0-100

    // Other
    Activate,   // (A)
    On,         // (On)
    Deactivate, // (D)
    Off,        // (Off)
}

impl StateType {
    /// Converts StateType into a kll_core phro state
    pub fn phro(&self) -> kll_core::trigger::Phro {
        match self {
            StateType::Hold => kll_core::trigger::Phro::Hold,
            StateType::Off => kll_core::trigger::Phro::Off,
            StateType::Press => kll_core::trigger::Phro::Press,
            StateType::Release => kll_core::trigger::Phro::Release,
            _ => {
                panic!("Invalid phro StateType: {:?}", self);
            }
        }
    }

    /// Converts StateType into a kll_core CapabilityState
    pub fn capability_state(&self) -> kll_core::CapabilityState {
        match self {
            StateType::Press => kll_core::CapabilityState::Initial,
            StateType::Release => kll_core::CapabilityState::Last,
            _ => {
                panic!("{:?} not implemented/unsupported", self);
            }
        }
    }
}

impl FromStr for StateType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            // Key
            "P" => Self::Press,
            "H" => Self::Hold,
            "R" => Self::Release,
            "O" => Self::Unpressed,
            "UP" => Self::UniquePress,
            "UR" => Self::UniqueRelease,

            // Other
            "A" => Self::Activate,
            "On" => Self::On,
            "D" => Self::Deactivate,
            "Off" => Self::Off,
            _ => {
                return Err(Error::UnknownMatch { s: s.to_string() });
            }
        })
    }
}

impl fmt::Display for StateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Key
            Self::Press => write!(f, "P"),
            Self::Hold => write!(f, "H"),
            Self::Release => write!(f, "R"),
            Self::Unpressed => write!(f, "O"),
            Self::UniquePress => write!(f, "UP"),
            Self::UniqueRelease => write!(f, "UR"),
            Self::Analog(v) => write!(f, "{}", v),

            // Other
            Self::Activate => write!(f, "A"),
            Self::On => write!(f, "On"),
            Self::Deactivate => write!(f, "D"),
            Self::Off => write!(f, "Off"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Key<'a> {
    Scancode(usize),
    Char(&'a str),
    Usb(&'a str),
    Consumer(&'a str),
    System(&'a str),
    Unicode(&'a str),
    None,
}

/// Converts from string to int, returing None if None was passed as the input
fn parse_int_option(input: Option<&String>) -> Option<usize> {
    use crate::parser::parse_int;
    input.map(|val| parse_int(val))
}

impl<'a> Key<'a> {
    pub fn value(&self, layout: &Layout) -> Option<usize> {
        match self {
            Key::Scancode(num) => Some(*num),
            Key::Char(c) => parse_int_option(layout.from_hid_keyboard.get(*c)),
            Key::Usb(name) => parse_int_option(layout.from_hid_keyboard.get(*name)),
            Key::Consumer(name) => parse_int_option(layout.from_hid_consumer.get(*name)),
            Key::System(name) => parse_int_option(layout.from_hid_sysctrl.get(*name)),
            Key::Unicode(_) => None, // xxx
            Key::None => None,
        }
    }
}

impl<'a> fmt::Display for Key<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Scancode(num) => write!(f, "S{}", num),
            Key::Char(num) => write!(f, "'{}'", num),
            Key::Usb(name) => write!(f, "U\"{}\"", name),
            Key::Consumer(name) => write!(f, "CONS\"{}\"", name),
            Key::System(name) => write!(f, "SYS\"{}\"", name),
            Key::Unicode(name) => write!(f, "U+{}", name),
            Key::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ResultType<'a> {
    Output(Key<'a>),
    Layer((LayerMode, Indices)),
    Animation(AnimationResult<'a>),
    Pixel(Pixel<'a>),
    PixelLayer(Pixel<'a>),
    Capability((Capability<'a>, Option<StateMap>)),
    Text(&'a str),
    UnicodeText(&'a str),
    NOP,
}

impl<'a> fmt::Display for ResultType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Output(trigger) => write!(f, "{}", trigger),
            Self::Layer((mode, layers)) => write!(f, "{}[{}]", mode, format_indices(layers)),
            Self::Animation(trigger) => write!(f, "{}", trigger),
            Self::Pixel(trigger) => write!(f, "{}", trigger),
            Self::PixelLayer(trigger) => write!(f, "{}", trigger),
            Self::Capability((trigger, state)) => {
                if let Some(state) = state {
                    write!(f, "{}({})", trigger, state)
                } else {
                    write!(f, "{}", trigger)
                }
            }
            Self::Text(text) => write!(f, "\"{}\"", text),
            Self::UnicodeText(text) => write!(f, "u\"{}\"", text),
            Self::NOP => write!(f, "None"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Action<'a> {
    pub result: ResultType<'a>,
    pub state: Option<StateMap>,
}

impl<'a> Action<'a> {
    /// Converts to a kll-core Capability definition
    pub fn kll_core_condition(&self, layouts: &mut Layouts) -> kll_core::Capability {
        // State must be defined
        // generate_state_scheduling() function can be used to compute if
        // it's not defined.
        assert!(self.state.is_some(), "state *must* be defined, use generate_state_scheduling() to convert implied state into implicit state.");
        assert!(self.state.as_ref().unwrap().states.len() == 1, "StateMap *must* only have a single state defined, need to expand into a sequence first.");

        let layout = layouts.get_layout("base/base.json");
        match &self.result {
            ResultType::Output(key) => {
                let id = match key.value(&layout) {
                    Some(id) => id,
                    None => {
                        panic!("{:?} doesn't match a USB HID key.", key);
                    }
                };
                match key {
                    Key::Usb(_value) => {
                        kll_core::Capability::HidKeyboard {
                            state: self.state.as_ref().unwrap().states[0]
                                .kind
                                .capability_state(),
                            loop_condition_index: 0, // TODO
                            id: kll_core::kll_hid::Keyboard::from(id as u16),
                        }
                    }
                    _ => kll_core::Capability::NoOp {
                        state: kll_core::CapabilityState::None,
                        loop_condition_index: 0,
                    },
                }
            }
            ResultType::Layer((_mode, _indices)) => {
                panic!("Incomplete {:?}", &self.result);
            }
            ResultType::Animation(_animation_result) => {
                panic!("Incomplete {:?}", &self.result);
            }
            ResultType::Capability((_capability, _state)) => {
                panic!("Incomplete {:?}", &self.result);
            }
            ResultType::Text(_text) => {
                panic!("Incomplete {:?}", &self.result);
            }
            ResultType::UnicodeText(_text) => {
                panic!("Incomplete {:?}", &self.result);
            }
            ResultType::NOP => kll_core::Capability::NoOp {
                state: kll_core::CapabilityState::None,
                loop_condition_index: 0,
            },
            _ => {
                panic!("Incomplete {:?}", &self.result);
            }
        }
    }

    /// Generates state scheduling from implied state
    /// Converts S1 : U"A"; to (action/result part)
    ///    S1(P) : U"A"(P);
    ///    S1(R) : U"A"(R);
    fn implied_state(&self) -> Option<Vec<Self>> {
        // No state (implied state), generate new actions
        if self.state.is_none() {
            Some(vec![
                Self {
                    result: self.result.clone(),
                    state: Some(StateMap::new(vec![State {
                        kind: StateType::Press,
                        time: None,
                    }])),
                },
                Self {
                    result: self.result.clone(),
                    state: Some(StateMap::new(vec![State {
                        kind: StateType::Release,
                        time: None,
                    }])),
                },
            ])
        } else {
            None
        }
    }
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(state) = &self.state {
            write!(f, "{}:{}", self.result, state)
        } else {
            write!(f, "{}", self.result)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TriggerMode {
    Replace,            // :
    SoftReplace,        // ::
    Add,                // :+
    Remove,             // :-
    IsolateReplace,     // i:
    IsolateSoftReplace, // i::
    IsolateAdd,         // i:+
    IsolateRemove,      // i:-
}

impl FromStr for TriggerMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ":" => Self::Replace,
            "::" => Self::SoftReplace,
            ":+" => Self::Add,
            ":-" => Self::Remove,
            "i:" => Self::IsolateReplace,
            "i::" => Self::IsolateSoftReplace,
            "i:+" => Self::IsolateAdd,
            "i:-" => Self::IsolateRemove,
            _ => {
                return Err(Error::UnknownMatch { s: s.to_string() });
            }
        })
    }
}

impl fmt::Display for TriggerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Replace => write!(f, ":"),
            Self::SoftReplace => write!(f, "::"),
            Self::Add => write!(f, ":+"),
            Self::Remove => write!(f, ":-"),
            Self::IsolateReplace => write!(f, "i:"),
            Self::IsolateSoftReplace => write!(f, "i::"),
            Self::IsolateAdd => write!(f, "i:+"),
            Self::IsolateRemove => write!(f, "i:-"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PixelAddr {
    Absolute(usize),
    RelativeInt(usize),
    RelativePercent(usize),
}

impl FromStr for PixelAddr {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PixelAddr::Absolute(s.parse::<usize>()?))
    }
}

impl fmt::Display for PixelAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absolute(v) => write!(f, "{}", v),
            Self::RelativeInt(v) => write!(f, "{:+}", v),
            Self::RelativePercent(v) => write!(f, "{:+}%", v),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PixelRange<'a> {
    pub index: Option<PixelAddr>,
    pub row: Option<PixelAddr>,
    pub col: Option<PixelAddr>,
    pub key: Option<Key<'a>>,
}

impl<'a> PixelRange<'a> {
    pub fn from_map(map: Map) -> Result<Self, <usize as FromStr>::Err> {
        let mut pos = PixelRange::default();
        for (k, v) in map.iter() {
            match *k {
                "i" => pos.index = Some(PixelAddr::from_str(v)?),
                "r" => pos.row = Some(PixelAddr::from_str(v)?),
                "c" => pos.col = Some(PixelAddr::from_str(v)?),
                _ => {}
            }
        }

        Ok(pos)
    }
}

impl<'a> fmt::Display for PixelRange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(index) = &self.index {
            write!(f, "{}", index)?;
        }
        if let Some(row) = &self.row {
            write!(f, "r:{}", row)?;
        }
        if let Some(col) = &self.col {
            write!(f, "c:{}", col)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AnimationResult<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> fmt::Display for AnimationResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A[{}]({})", self.name, self.args.join(", "))
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Pixel<'a> {
    pub range: PixelRange<'a>,
    pub channel_values: Vec<PixelColor>,
}

impl<'a> fmt::Display for Pixel<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "P[{}]({})",
            self.range,
            self.channel_values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PixelColor {
    Rgb(usize),
    Relative(isize),
    RelativeNoRoll(isize),
    Shift(isize),
}

impl fmt::Display for PixelColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgb(v) => write!(f, "{}", v),
            Self::Relative(v) => write!(f, "{:+}", v),
            Self::RelativeNoRoll(v) => write!(f, ":{:+}", v),
            Self::Shift(v) => write!(f, "<{:+}", v),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct KllFile<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl<'a> fmt::Display for KllFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        Ok(())
    }
}
