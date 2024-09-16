//! `tree-sitter` helpers for extracting and locating concrete features
//! in the original YAML.

use anyhow::Result;

use super::{ConcreteLocation, Feature, JobOrKeys, StepOrKeys, WorkflowLocation};
use crate::models::Workflow;

pub(crate) struct Locator {}

impl Locator {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn concretize<'w>(
        &self,
        workflow: &'w Workflow,
        location: &WorkflowLocation,
    ) -> Result<Feature<'w>> {
        let mut builder = yamlpath::QueryBuilder::new();

        builder = match &location.job_or_key {
            Some(JobOrKeys::Job(job)) => {
                builder = builder.key("jobs").key(job.id);

                match &job.step_or_keys {
                    Some(StepOrKeys::Step(step)) => builder.key("steps").index(step.index),
                    Some(StepOrKeys::Keys(keys)) => builder.keys(keys.iter().map(|k| *k)),
                    None => builder,
                }
            }
            Some(JobOrKeys::Keys(keys)) => builder.keys(keys.iter().map(|k| *k)),
            None => panic!("API misuse: workflow location must specify a top-level key or job"),
        };

        let query = builder.build();
        let feature = workflow.document.query(&query)?;

        Ok(Feature {
            location: ConcreteLocation::from(&feature.location),
            feature: workflow.document.extract(&feature),
        })
    }
}