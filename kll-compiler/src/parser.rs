use pest_consume::{match_nodes, Error, Parser};
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use crate::types::*;

pub type PestError = Error<Rule>;
type Result<T> = std::result::Result<T, PestError>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Parser)]
#[grammar = "kll.pest"]
pub struct KLLParser;

pub fn parse_int(s: &str) -> usize {
    //dbg!(s);
    if s.starts_with("0x") {
        usize::from_str_radix(s.trim_start_matches("0x"), 16).unwrap_or(0)
    } else {
        s.parse::<usize>().unwrap_or(0)
    }
}

#[pest_consume::parser]
impl KLLParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }
    fn word(input: Node) -> Result<&str> {
        Ok(input.as_str())
    }
    fn string(input: Node) -> Result<&str> {
        Ok(input.as_str().trim_matches('"'))
    }
    fn unistr(input: Node) -> Result<&str> {
        Ok(input.as_str().strip_prefix('u').unwrap().trim_matches('"'))
    }
    fn number(input: Node) -> Result<usize> {
        Ok(parse_int(input.as_str()))
    }
    fn range(input: Node) -> Result<(usize, usize)> {
        Ok(match_nodes!(input.into_children();
            [number(start)] => (start, start),
            [number(start), number(end)] => (start, end),
            [string(_name)] => (0, 0), // XXX (What table are we using?)
        ))
    }
    fn ids(input: Node) -> Result<Indices> {
        Ok(match_nodes!(input.into_children();
            [range(ranges)..] => ranges.map(|(start, end)| Range { start, end }).collect(),
        ))
    }

    fn name(input: Node) -> Result<&str> {
        Ok(input.as_str())
    }
    fn value(input: Node) -> Result<&str> {
        Ok(input.as_str())
    }
    fn array(input: Node) -> Result<(&str, Option<usize>)> {
        Ok(match_nodes!(input.into_children();
            [name(n)] => (n, None),
            [name(n), number(i)] => (n, Some(i)),
        ))
    }
    fn rhs(input: Node) -> Result<&str> {
        Ok(input.as_str())
    }

    fn kv(input: Node) -> Result<(&str, &str)> {
        Ok(match_nodes!(input.into_children();
            [word(k), word(v)] => (k,v),
            [word(k), ] => (k,"")
        ))
    }
    fn kvmap(input: Node) -> Result<HashMap<&str, &str>> {
        Ok(match_nodes!(input.into_children();
            [kv(kv)..] => kv.collect(),
        ))
    }

    fn function(input: Node) -> Result<Capability> {
        Ok(match_nodes!(input.into_children();
            [name(n)] => Capability {
                function: n,
                args: vec![],
            },
            [name(n), kvmap(args)] => Capability {
                function: n,
                args: args.keys().copied().collect(), // xxx
            }
        ))
    }

    fn binding(input: Node) -> Result<TriggerMode> {
        Ok(match input.as_str() {
            ":" => TriggerMode::Replace,
            "::" => TriggerMode::SoftReplace,
            ":+" => TriggerMode::Add,
            ":-" => TriggerMode::Remove,
            "i:" => TriggerMode::IsolateReplace,
            "i::" => TriggerMode::IsolateSoftReplace,
            "i:+" => TriggerMode::IsolateAdd,
            "i:-" => TriggerMode::IsolateRemove,
            _ => unreachable!(),
        })
    }

    fn pixel(input: Node) -> Result<Indices> {
        Ok(match_nodes!(input.into_children();
            [ids(ranges)] => ranges,
            [number(index)] => vec![ Range {start: index, end: index } ],
        ))
    }
    fn channel(input: Node) -> Result<PixelColor> {
        let color = input.as_str();
        //dbg!(color);
        if color.len() >= 2 {
            Ok(match (&color[0..1], &color[1..2]) {
                ("+", ":") | ("-", ":") => {
                    PixelColor::RelativeNoRoll(color[2..].parse::<isize>().unwrap())
                }
                (">", ">") | ("<", "<") => PixelColor::Shift(color[2..].parse::<isize>().unwrap()),
                ("+", _) | ("-", _) => PixelColor::Relative(color.parse::<isize>().unwrap()),
                _ => PixelColor::Rgb(parse_int(color)),
            })
        } else {
            Ok(PixelColor::Rgb(parse_int(color)))
        }
    }

    fn pixelval(input: Node) -> Result<Pixel> {
        Ok(match_nodes!(input.into_children();
            [usbcode(usbcode), channel(c)..] => {
                Pixel {
                    range: PixelRange {
                        index: None,
                        row: None,
                        col: None,
                        key: Some(usbcode),
                    },
                    channel_values: c.collect(),
                }
            },
            [scancode(scancode), channel(c)..] => {
                Pixel {
                    range: PixelRange {
                        index: None,
                        row: None,
                        col: None,
                        key: Some(Key::Scancode(scancode)),
                    },
                    channel_values: c.collect(),
                }
            },
            [kvmap(map), channel(c)..] => {
                // Retrieve range and print a message if there are problems
                // TODO - Print line number on error
                Pixel {
                    range: match PixelRange::from_map(map.clone()) {
                        Ok(range) => range,
                        Err(err) => {
                            panic!("Could not collect PixelRange from map({:?}) - Error({})", map, err);
                        }
                    }, // XXX Handle error
                    channel_values: c.collect(),
                }
            }
        ))
    }

    fn scancode(input: Node) -> Result<usize> {
        Ok(parse_int(input.as_str().strip_prefix('S').unwrap()))
    }
    fn charcode(input: Node) -> Result<Key> {
        let charcode = input.as_str().trim_matches('"');
        Ok(Key::Char(charcode))
    }
    fn unicode(input: Node) -> Result<Key> {
        let unicode = input.as_str().strip_prefix("U+").unwrap();
        Ok(Key::Char(unicode))
    }
    fn usbcode(input: Node) -> Result<Key> {
        let usbcode = input.as_str().strip_prefix('U').unwrap();
        Ok(Key::Usb(usbcode.trim_matches('"')))
    }
    fn consumer(input: Node) -> Result<Key> {
        let concode = input.as_str().strip_prefix("CONS").unwrap();
        Ok(Key::Consumer(concode.trim_matches('"')))
    }
    fn system(input: Node) -> Result<Key> {
        let syscode = input.as_str().strip_prefix("SYS").unwrap();
        Ok(Key::System(syscode.trim_matches('"')))
    }
    fn none(_input: Node) -> Result<&str> {
        Ok("None")
    }
    fn key(input: Node) -> Result<Key> {
        Ok(match_nodes!(input.into_children();
                [scancode(s)] => Key::Scancode(s),
                [charcode(key)] => key,
                [unicode(key)] => key,
                [usbcode(key)] => key,
                [consumer(key)] => key,
                [system(key)] => key,
                [none(_)] => Key::None,
        ))
    }

    fn layer_type(input: Node) -> Result<LayerMode> {
        Ok(LayerMode::from_str(input.as_str()).unwrap()) // XXX Handle error
    }

    fn layer(input: Node) -> Result<(LayerMode, Indices)> {
        Ok(match_nodes!(input.into_children();
            [layer_type(mode), ids(indices)] => (mode, indices),
            [layer_type(mode), number(index)] => (mode, vec![ Range { start: index, end: index } ]),
        ))
    }

    fn indicator(input: Node) -> Result<Indices> {
        Ok(match_nodes!(input.into_children();
            [ids(indices)] => indices,
            [number(index)] => vec![ Range { start: index, end: index } ],
            [string(_name)] => vec![ Range { start: 0, end: 0 } ], // XXX (Need LUT)
        ))
    }

    fn trig(input: Node) -> Result<(usize, usize, Option<usize>)> {
        Ok(match_nodes!(input.into_children();
            [number(bank), number(index)] => (bank, index, None),
            [number(bank), number(index), number(arg)] => (bank, index, Some(arg)),
        ))
    }
    fn trigger_type(input: Node) -> Result<TriggerType> {
        Ok(match_nodes!(input.into_children();
            [name(name)] => TriggerType::Animation(name),
            [key(trigger)] => TriggerType::Key(trigger),
            [layer(trigger)] => TriggerType::Layer(trigger),
            [indicator(trigger)] => TriggerType::Indicator(trigger),
            [trig((bank, index, arg))] => TriggerType::Generic((bank, index, arg)),
        ))
    }
    fn trigger(input: Node) -> Result<Trigger> {
        Ok(match_nodes!(input.into_children();
            [trigger_type(trigger)] => Trigger {
                trigger,
                state: None,
            },
            [trigger_type(trigger), kvmap(args)] => Trigger {
                trigger,
                state: Some(StateMap::from_map(args).unwrap()), // XXX Handle error
            },
        ))
    }

    fn result_type(input: Node) -> Result<ResultType> {
        Ok(match_nodes!(input.into_children();
            [charcode(key)] => ResultType::Output(key),
            [unicode(key)] => ResultType::Output(key),
            [usbcode(key)] => ResultType::Output(key),
            [consumer(key)] => ResultType::Output(key),
            [system(key)] => ResultType::Output(key),
            [pixelval(pixel)] => ResultType::Pixel(pixel),
            [animation_result(anim)] => ResultType::Animation(anim),
            [function(cap)] => ResultType::Capability((cap, None)), // XXX
            [layer(layer)] => ResultType::Layer(layer),
            [string(text)] => ResultType::Text(text),
            [unistr(text)] => ResultType::UnicodeText(text),
            [none(_)] => ResultType::NOP,
        ))
    }
    fn result(input: Node) -> Result<Action> {
        Ok(match_nodes!(input.into_children();
            [result_type(result)] => Action {
                result,
                state: None,
            },
            [result_type(result), kvmap(args)] => Action {
                result,
                state: Some(StateMap::from_map(args).unwrap()), // XXX Handle error
            },
        ))
    }
    fn result_group(input: Node) -> Result<Vec<Action>> {
        Ok(match_nodes!(input.into_children();
            [result(results)..] => results.collect(),
        ))
    }
    fn results(input: Node) -> Result<Vec<Vec<Action>>> {
        Ok(match_nodes!(input.into_children();
            [result_group(groups)..] => groups.collect(),
        ))
    }

    fn property(input: Node) -> Result<Statement> {
        let (name, index, value) = match_nodes!(input.into_children();
            [name(n), rhs(v)] => (n, None, v),
            [array((n,i)), rhs(v)] => (n, i, v),
            [string(n), rhs(v)] => (n, None, v),
        );
        Ok(Statement::Variable((name, index, value)))
    }
    fn define(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [name(n), value(v)] => Statement::Define((n, v))
        ))
    }
    fn capability(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [name(n), function(f)] =>  Statement::Capability((n, f)),
        ))
    }
    fn trigger_group(input: Node) -> Result<Vec<Trigger>> {
        Ok(match_nodes!(input.into_children();
            [trigger(triggers)..] => triggers.collect(),
        ))
    }
    fn triggers(input: Node) -> Result<Vec<Vec<Trigger>>> {
        Ok(match_nodes!(input.into_children();
            [trigger_group(groups)..] => groups.collect(),
        ))
    }
    fn mapping(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [triggers(triggers), binding(mode), results(results)] => Statement::Keymap(Mapping(TriggerList(triggers), mode, ResultList(results))),
        ))
    }
    fn position(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [range((start, end)), kvmap(map)] => {
                Statement::Position((vec![ Range { start, end } ], Position::from_map(map)))
            },
            [scancode(index), kvmap(map)] => {
                Statement::Position((vec![ Range { start: index, end: index }], Position::from_map(map)))
            },
            [pixel(indices), kvmap(map)] => {
                Statement::Position((indices, Position::from_map(map)))
            }
        ))
    }
    fn pixelmap(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [range((start, end)), kvmap(channelmap), scancode(scancode)] => {
                let pixel = PixelDef::new(channelmap, Some(scancode));
                Statement::Pixelmap((vec![ Range { start, end } ], pixel))
            },
            [pixel(indices), kvmap(channelmap), _none] => {
                let pixel = PixelDef::new(channelmap, None);
                Statement::Pixelmap((indices, pixel))
            },
            [pixel(indices), kvmap(channelmap), scancode(scancode)] => {
                let pixel = PixelDef::new(channelmap, Some(scancode));
                Statement::Pixelmap((indices, pixel))
            }
        ))
    }
    fn animdef(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [name(name), kvmap(args)] => {
                let animation = Animation {
                    modifiers: args,
                    frames: vec![],
                };

                Statement::Animation((name, animation))
            }
        ))
    }
    fn animation_result(input: Node) -> Result<AnimationResult> {
        Ok(match_nodes!(input.into_children();
            [name(name), kvmap(args)] => {
                AnimationResult {
                    name,
                    args: args.keys().copied().collect(), // xxx
                }
            }
        ))
    }
    fn animframe(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [name(name), ids(indices), pixelval(pixels)..] => {
                Statement::Frame((name, indices, pixels.collect()))
            },
            [name(name), number(index), pixelval(pixels)..] => {
                Statement::Frame((name, vec![ Range { start: index, end: index }], pixels.collect()))
            }
        ))
    }
    fn statement(input: Node) -> Result<Statement> {
        //dbg!(input.as_str());
        //dbg!(&input);
        //let _ = input.children().single().map(|n| dbg!(n.as_rule()));
        Ok(match_nodes!(input.into_children();
                [property(stmt)] => stmt,
                [define(stmt)] => stmt,
                [capability(stmt)] => stmt,
                [mapping(stmt)]=> stmt,
                [position(stmt)] => stmt,
                [pixelmap(stmt)] => stmt,
                [animdef(stmt)] => stmt,
                [animframe(stmt)] => stmt,
        ))
    }

    pub fn file(input: Node) -> Result<Vec<Statement>> {
        Ok(match_nodes!(input.into_children();
            [statement(statements).., _] => statements.collect(),
        ))
    }
}

impl<'a> KllFile<'a> {
    #[allow(clippy::should_implement_trait, clippy::result_large_err)]
    pub fn from_str(text: &str) -> Result<KllFile> {
        let inputs = KLLParser::parse(Rule::file, text)?;
        let input = inputs.single()?;

        let kll = KllFile {
            statements: KLLParser::file(input)?,
        };

        Ok(kll)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{KLLParser, Rule};
    use pest::Parser;

    #[test]
    fn test_numbers() {
        let number = KLLParser::parse(Rule::number, "1234");
        assert!(number.is_ok());

        let number = KLLParser::parse(Rule::number, "0x2f");
        assert!(number.is_ok());

        let not_number = KLLParser::parse(Rule::number, "this is not a number");
        assert!(not_number.is_err());
    }
}
