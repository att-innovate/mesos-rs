use mesos::{Scheduler, SchedulerClient, run_protobuf_scheduler};
use mesos::proto::*;

pub struct TestScheduler;

impl Scheduler for TestScheduler {
    fn subscribed(&mut self, client: &SchedulerClient, framework_id: &FrameworkID, heartbeat_interval_seconds: Option<f64>) {
        println!("received subscribed");
    }

    fn offers(&mut self, client: &SchedulerClient, offers: Vec<Offer>, inverse_offers: Vec<InverseOffer>) {
        println!("received offers");

        let mut offer_ids = vec![];
        for offer in offers {
            offer_ids.push(offer.get_id().clone());
        }
        for offer in inverse_offers {
            offer_ids.push(offer.get_id().clone());
        }

        let filters = Filters::new();

        client.decline(offer_ids, filters).unwrap();
    }

    fn rescind(&mut self, client: &SchedulerClient, offer_id: &OfferID) {
        println!("received rescind");
    }

    fn update(&mut self, client: &SchedulerClient, status: &TaskStatus) {
        println!("received update");
    }

    fn message(&mut self, client: &SchedulerClient, agent_id: &AgentID, executor_id: &ExecutorID, data: Vec<u8>) {
        println!("received message");
    }

    fn failure(&mut self, client: &SchedulerClient, agent_id: &AgentID, executor_id: Option<&ExecutorID>, status: Option<i32>) {
        println!("received failure");
    }

    fn error(&mut self, client: &SchedulerClient, message: String) {
        println!("received error");
    }

    fn heartbeat(&mut self, client: &SchedulerClient) {
        println!("received heartbeat");
    }
}

#[test]
fn main() {
    let url = "http://localhost:5050/api/v1/scheduler".to_string();
    let user = "root".to_string();
    let name = "rust http".to_string();
    let framework_timeout = 0f64;
    let framework_id = None;

    let mut scheduler = TestScheduler;
    run_protobuf_scheduler(url, user, name, framework_timeout, &mut scheduler, framework_id);
}
