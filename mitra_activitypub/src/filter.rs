use apx_core::url::http_uri::Hostname;
use wildmatch::WildMatch;

use mitra_models::{
    database::{
        get_database_client,
        DatabaseClient,
        DatabaseConnectionPool,
        DatabaseError,
        DatabaseTypeError,
    },
    filter_rules::{
        queries::get_filter_rules,
        types::{FilterRule, FilterAction},
    },
    profiles::types::DbActor,
};

use crate::utils::parse_http_url_from_db;

// Returns DatabaseError if actor data is not valid
// TODO: validation should happen during actor data deserialization
pub fn get_moderation_domain(
    actor: &DbActor,
) -> Result<Hostname, DatabaseError> {
    let http_url = if actor.is_portable() {
        // TODO: return None if gateway list is empty
        actor.gateways.first().ok_or(DatabaseTypeError)?
    } else {
        &actor.id
    };
    let hostname = parse_http_url_from_db(http_url)?.hostname();
    Ok(hostname)
}

#[derive(Clone)]
pub struct FederationFilter {
    rules: Vec<FilterRule>,
}

impl FederationFilter {
    pub async fn init(
        db_client: &impl DatabaseClient,
    ) -> Result<Self, DatabaseError> {
        let rules = get_filter_rules(db_client).await?;
        Ok(Self { rules })
    }

    pub async fn init_with_pool(
        db_pool: &DatabaseConnectionPool,
    ) -> Result<Self, DatabaseError> {
        let db_client = &**get_database_client(db_pool).await?;
        Self::init(db_client).await
    }

    pub fn is_action_required(
        &self,
        hostname: &str,
        action: FilterAction,
    ) -> bool {
        // Rules are checked in order. The last matching rule wins.
        let mut is_required = false;
        let applicable_rules = self.rules.iter()
            .filter(|rule| WildMatch::new(&rule.target).matches(hostname))
            .filter(|rule| rule.filter_action == action);
        // Apply rules, starting with less specific
        for rule in applicable_rules {
            is_required = !rule.is_reversed;
        };
        is_required
    }

    pub fn is_incoming_blocked(&self, hostname: &str) -> bool {
        self.is_action_required(hostname, FilterAction::RejectIncoming) ||
        self.is_action_required(hostname, FilterAction::Reject)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use mitra_models::{
        database::test_utils::create_test_database,
        filter_rules::queries::add_filter_rule,
    };
    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_federation_filter() {
        let db_client = &create_test_database().await;
        let target_1 = "*";
        let target_2 = "one.example";
        add_filter_rule(
            db_client,
            target_1,
            FilterAction::RejectIncoming,
            false, // block
        ).await.unwrap();
        add_filter_rule(
            db_client,
            target_2,
            FilterAction::RejectIncoming,
            true, // allow
        ).await.unwrap();
        let rules = get_filter_rules(db_client).await.unwrap();

        let filter = FederationFilter { rules };
        assert_eq!(filter.is_incoming_blocked("one.example"), false);
        assert_eq!(filter.is_incoming_blocked("two.example"), true);
        assert_eq!(filter.is_incoming_blocked("any.example"), true);
    }
}
