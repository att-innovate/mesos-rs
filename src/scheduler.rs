use scheduler_client::SchedulerClient;
use proto::*;

pub trait Scheduler {
    fn subscribed(&mut self,
                  client: &SchedulerClient,
                  framework_id: &FrameworkID,
                  heartbeat_interval_seconds: Option<f64>);
    fn offers(&mut self, client: &SchedulerClient, offers: Vec<&Offer>);
    fn inverse_offers(&mut self,
                      client: &SchedulerClient,
                      inverse_offers: Vec<&InverseOffer>);
    fn rescind(&mut self, client: &SchedulerClient, offer_id: &OfferID);
    fn update(&mut self, client: &SchedulerClient, status: &TaskStatus);
    fn message(&mut self,
               client: &SchedulerClient,
               agent_id: &AgentID,
               executor_id: &ExecutorID,
               data: Vec<u8>);
    fn failure(&mut self,
               client: &SchedulerClient,
               agent_id: Option<&AgentID>,
               executor_id: Option<&ExecutorID>,
               status: Option<i32>);
    fn error(&mut self, client: &SchedulerClient, message: String);
    fn heartbeat(&mut self, client: &SchedulerClient) {
    }
    fn disconnected(&mut self);
}
