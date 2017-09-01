use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ControlCommand {
    /// Begin logical evaluation mode. In evaluation mode, objects that are encounted are
    /// added to an evaluation stack, rather than simply echoed into the main text output stream.
    /// As they're pushed onto the stack, they may be processed by other commands, functions, etc.
    EvalStart,

    /// The topmost object on the evaluation stack is popped and appended to the output stream
    /// (main story output).
    EvalOutput,

    /// End logical evaluation mode. Future objects will be appended to the output stream rather
    /// than to the evaluation stack.
    EvalEnd,

    /// Duplicate the topmost object on the evaluation stack. Useful since some commands consume
    /// objects on the evaluation stack.
    Duplicate,

    /// Pops a value from the evaluation stack, without appending to the output stream.
    PopEvaluatedValue,

    /// pop the callstack - used for returning from a tunnel or function respectively. They are
    /// specified independently for error checking, since the callstack is aware of whether each
    /// element was pushed as a tunnel or function in the first place.
    PopFunction,
    PopTunnel,

    /// Begin string evaluation mode. Adds a marker to the output stream, and goes into content
    /// mode (from evaluation mode). Must have already been in evaluation mode when this is
    /// encounted.
    BeginString,

    /// End string evaluation mode. All content after the previous Begin marker is concatenated
    /// together, removed from the output stream, and appended as a string value to the evaluation
    /// stack. Re-enters evaluation mode immediately afterwards.
    EndString,

    /// No-operation. Does nothing, but is useful as an addressable piece of content to divert to.
    NoOp,

    /// Pushes an integer with the current number of choices to the evaluation stack.
    ChoiceCount,

    /// Pops from the evaluation stack, expecting to see a divert target for a knot, stitch,
    /// gather or choice. Pushes an integer with the number of turns since that target was last
    /// visited by the story engine.
    TurnsSince,

    ReadCount,
    Random,
    SeedRandom,
    VisitIndex,

    /// Pops an integer, expected to be the number of elements in a sequence that's being entered.
    /// In return, it pushes an integer with the next sequence shuffle index to the evaluation
    /// stack. This shuffle index is derived from the number of elements in the sequence, the
    /// number of elements in it, and the story's random seed from when it was first begun.
    SequenceShuffleIndex,

    /// Clones/starts a new thread, as used with the <- knot syntax in ink. This essentially
    /// clones the entire callstack, branching it.
    StartThread,

    /// Tries to close/pop the active thread, otherwise marks the story flow safe to exit without
    /// a loose end warning.
    Done,

    /// Ends the story flow immediately, closes all active threads, unwinds the callstack, and
    /// removes any choices that were previously created.
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