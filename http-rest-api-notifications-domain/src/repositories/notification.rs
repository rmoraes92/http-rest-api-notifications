use anyhow;

use crate::entities::notification::Notification;

pub trait CreateNotificationRepository {
    fn create(
        &self,
        notification: &Notification,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
