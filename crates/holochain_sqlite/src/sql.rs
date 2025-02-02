pub mod sql_cell {
    pub(crate) const SCHEMA: &str = include_str!("sql/cell/schema.sql");
    pub const UPDATE_INTEGRATE_OPS: &str = include_str!("sql/cell/update_integrate_ops.sql");

    pub const FETCH_OP_HASHES_P1: &str =
        include_str!("sql/cell/fetch_hashes/fetch_op_hashes_p1.sql");
    pub const FETCH_OP_HASHES_P2: &str =
        include_str!("sql/cell/fetch_hashes/fetch_op_hashes_p2.sql");

    pub const FETCH_OP: &str = include_str!("sql/cell/fetch_op.sql");

    pub mod schedule {
        pub const UPDATE: &str = include_str!("sql/cell/schedule/update.sql");
        pub const DELETE: &str = include_str!("sql/cell/schedule/delete.sql");
        pub const EXPIRED: &str = include_str!("sql/cell/schedule/expired.sql");
        pub const DELETE_ALL_EPHEMERAL: &str =
            include_str!("sql/cell/schedule/delete_all_ephemeral.sql");
        pub const DELETE_LIVE_EPHEMERAL: &str =
            include_str!("sql/cell/schedule/delete_live_ephemeral.sql");
    }
    pub mod state_dump {
        pub const DHT_OPS_IN_INTEGRATION_LIMBO: &str =
            include_str!("sql/cell/state_dump/dht_ops_in_integration_limbo.sql");
        pub const DHT_OPS_INTEGRATED: &str =
            include_str!("sql/cell/state_dump/dht_ops_integrated.sql");
        pub const DHT_OPS_IN_VALIDATION_LIMBO: &str =
            include_str!("sql/cell/state_dump/dht_ops_in_validation_limbo.sql");
        pub const DHT_OPS_ROW_ID: &str = include_str!("sql/cell/state_dump/dht_ops_row_id.sql");
    }
}

pub(crate) mod sql_conductor {
    pub(crate) const SCHEMA: &str = include_str!("sql/conductor/schema.sql");
}

pub(crate) mod sql_wasm {
    pub(crate) const SCHEMA: &str = include_str!("sql/wasm/schema.sql");
}

pub(crate) mod sql_p2p_agent_store {
    pub(crate) const SCHEMA: &str = include_str!("sql/p2p_agent_store/schema.sql");
    pub(crate) const INSERT: &str = include_str!("sql/p2p_agent_store/insert.sql");
    pub(crate) const SELECT_ALL: &str = include_str!("sql/p2p_agent_store/select_all.sql");
    pub(crate) const SELECT: &str = include_str!("sql/p2p_agent_store/select.sql");
    pub(crate) const GOSSIP_QUERY: &str = include_str!("sql/p2p_agent_store/gossip_query.sql");
    pub(crate) const QUERY_NEAR_BASIS: &str =
        include_str!("sql/p2p_agent_store/query_near_basis.sql");
    pub(crate) const PRUNE: &str = include_str!("sql/p2p_agent_store/prune.sql");
}

pub(crate) mod sql_p2p_metrics {
    pub(crate) const SCHEMA: &str = include_str!("sql/p2p_metrics/schema.sql");
    pub(crate) const INSERT: &str = include_str!("sql/p2p_metrics/insert.sql");
    pub(crate) const QUERY_LAST_SYNC: &str = include_str!("sql/p2p_metrics/query_last_sync.sql");
    pub(crate) const QUERY_OLDEST: &str = include_str!("sql/p2p_metrics/query_oldest.sql");
}
