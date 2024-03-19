#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum ProcessingStatus {
  Complete = 0,
  CompleteWithWarning = 1,
  Incomplete = 2,
  ChildIncomplete = 3,
  CompleteAndAbort = 4,
}

impl ProcessingStatus {
  pub fn is_complete(&self) -> bool {
    if *self == ProcessingStatus::Complete || *self == ProcessingStatus::CompleteWithWarning {
      return true;
    }
    false
  }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ProcessingStep {
  Begin,
  EntityRefs,
  EntityVariables,
  Children,
  SpecialChildren,
}

#[derive(Debug)]
pub struct ProcessingContext {
  #[allow(dead_code)]
  step: ProcessingStep,
}

impl ProcessingContext {
  pub fn new() -> ProcessingContext {
    ProcessingContext {
      step: ProcessingStep::Begin,
    }
  }

  pub fn create_for_child(&self) -> ProcessingContext {
    ProcessingContext {
      step: ProcessingStep::Begin,
    }
  }
}
