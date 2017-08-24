use choice::Choice;
use error::{InkError, InkErrorCode};
use json_parser::RuntimeGraphBuilder;
use runtime_graph::RuntimeGraph;
use story_state::StoryState;

use std::io::Read;

pub const InkVersion: u32 = 17;
pub const InkVersionMinimumCompatible: u32 = 16;

pub struct Story {
    runtime_graph: RuntimeGraph,
    state: StoryState
}

enum StoryFlow<'a> {
    Continue(&'a str),
    WaitForChoice,
    End
}

impl Story {
    pub fn from_str(s: &str) -> Result<Story, InkError> {
        Story::new( RuntimeGraphBuilder::from_str(s)?)
    }

    pub fn from_slice(v: &[u8]) -> Result<Story, InkError> {
        Story::new( RuntimeGraphBuilder::from_slice(v)?)
    }

    pub fn from_reader<R>(rdr: R) -> Result<Story, InkError>
        where
            R: Read {
        Story::new( RuntimeGraphBuilder::from_reader(rdr)?)
    }

    fn new(runtime_graph: RuntimeGraph) -> Result<Story, InkError> {
        if runtime_graph.ink_version() > InkVersion {
            return Err(InkError::new(InkErrorCode::Message("Version of ink used to build story is newer than the current version of the engine".to_owned())));
        }
        else if runtime_graph.ink_version() < InkVersionMinimumCompatible {
            return Err(InkError::new(InkErrorCode::Message("Version of ink used to build story is too old to be loaded by this version of the engine".to_owned())));
        }

        let state = StoryState::new(&runtime_graph);

        Ok(Story {
            runtime_graph: runtime_graph,
            state: state
        })
    }

    /// The list of Choice objects available at the current point in
    /// the Story.
    pub fn current_choices(&self) -> Option<&Vec<Choice>> {
        None
    }

    /// The latest line of content.
    pub fn current_text(&self) -> Option<&str> {
        None
    }

    pub fn current_tags(&self) -> Option<&Vec<&str>> {
        None
    }

    /// Continue the story for one line of content, if possible.
    ///
    /// This returns Ok(StoryFlow::Continue(text)) for the next line of content,
    /// Ok(StoryFlow::WaitForChoice) if the user has to make a choice at this point
    /// of the Story or Ok(StoryFlow::End) if the Story end.
    ///
    /// # Errors
    ///
    /// This can fail if an error occur during the evaluation of the Story.
    pub fn advance(&self) -> Result<StoryFlow, InkError> {
        Ok(StoryFlow::End)
    }

    pub fn make_choice(&self, index: usize) -> bool {
        return false;
    }

}