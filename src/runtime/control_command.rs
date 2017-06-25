use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ControlCommand {
    EvalStart,
    EvalOutput,
    EvalEnd,
    Duplicate,
    PopEvaluatedValue,
    PopFunction,
    PopTunnel,
    BeginString,
    EndString,
    NoOp,
    ChoiceCount,
    TurnsSince,
    ReadCount,
    Random,
    SeedRandom,
    VisitIndex,
    SequenceShuffleIndex,
    StartThread,
    Done,
    End,
    ListFromInt,
    ListRange
}

impl fmt::Display for ControlCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ControlCommand::EvalStart => write!(f, "ev"),
            ControlCommand::EvalOutput => write!(f, "out"),
            ControlCommand::EvalEnd => write!(f, "/ev"),
            ControlCommand::Duplicate => write!(f, "du"),
            ControlCommand::PopEvaluatedValue => write!(f, "pop"),
            ControlCommand::PopFunction => write!(f, "~ret"),
            ControlCommand::PopTunnel => write!(f, "->->"),
            ControlCommand::BeginString => write!(f, "str"),
            ControlCommand::EndString => write!(f, "/str"),
            ControlCommand::NoOp => write!(f, "nop"),
            ControlCommand::ChoiceCount => write!(f, "choiceCnt"),
            ControlCommand::TurnsSince => write!(f, "turns"),
            ControlCommand::ReadCount => write!(f, "readc"),
            ControlCommand::Random => write!(f, "rnd"),
            ControlCommand::SeedRandom => write!(f, "srnd"),
            ControlCommand::VisitIndex => write!(f, "visit"),
            ControlCommand::SequenceShuffleIndex => write!(f, "seq"),
            ControlCommand::StartThread => write!(f, "thread"),
            ControlCommand::Done => write!(f, "done"),
            ControlCommand::End => write!(f, "end"),
            ControlCommand::ListFromInt => write!(f, "listInt"),
            ControlCommand::ListRange => write!(f, "range"),
        }
    }
}