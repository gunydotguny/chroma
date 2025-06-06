use chroma_log::CollectionRecord;

use crate::compactor::types::CompactionJob;

pub(crate) trait SchedulerPolicy: Send + Sync + SchedulerPolicyClone {
    fn determine(&self, collections: Vec<CollectionRecord>, number_jobs: i32)
        -> Vec<CompactionJob>;
}

pub(crate) trait SchedulerPolicyClone {
    fn clone_box(&self) -> Box<dyn SchedulerPolicy>;
}

impl<T> SchedulerPolicyClone for T
where
    T: 'static + SchedulerPolicy + Clone,
{
    fn clone_box(&self) -> Box<dyn SchedulerPolicy> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SchedulerPolicy> {
    fn clone(&self) -> Box<dyn SchedulerPolicy> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub(crate) struct LasCompactionTimeSchedulerPolicy {}

impl SchedulerPolicy for LasCompactionTimeSchedulerPolicy {
    fn determine(
        &self,
        collections: Vec<CollectionRecord>,
        number_jobs: i32,
    ) -> Vec<CompactionJob> {
        let mut collections = collections;
        collections.sort_by(|a, b| a.last_compaction_time.cmp(&b.last_compaction_time));
        let number_tasks = if number_jobs > collections.len() as i32 {
            collections.len() as i32
        } else {
            number_jobs
        };
        let mut tasks = Vec::new();
        for collection in &collections[0..number_tasks as usize] {
            tasks.push(CompactionJob {
                collection_id: collection.collection_id,
            });
        }
        tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chroma_types::CollectionUuid;
    use std::str::FromStr;

    #[test]
    fn test_scheduler_policy() {
        let collection_uuid_1 =
            CollectionUuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();
        let collection_uuid_2 =
            CollectionUuid::from_str("00000000-0000-0000-0000-000000000002").unwrap();
        let scheduler_policy = LasCompactionTimeSchedulerPolicy {};
        let collections = vec![
            CollectionRecord {
                collection_id: collection_uuid_1,
                tenant_id: "test".to_string(),
                last_compaction_time: 1,
                first_record_time: 1,
                offset: 0,
                collection_version: 0,
                collection_logical_size_bytes: 100,
            },
            CollectionRecord {
                collection_id: collection_uuid_2,
                tenant_id: "test".to_string(),
                last_compaction_time: 0,
                first_record_time: 0,
                offset: 0,
                collection_version: 0,
                collection_logical_size_bytes: 100,
            },
        ];
        let jobs = scheduler_policy.determine(collections.clone(), 1);
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].collection_id, collection_uuid_2);

        let jobs = scheduler_policy.determine(collections.clone(), 2);
        assert_eq!(jobs.len(), 2);
        assert_eq!(jobs[0].collection_id, collection_uuid_2);
        assert_eq!(jobs[1].collection_id, collection_uuid_1);
    }
}
