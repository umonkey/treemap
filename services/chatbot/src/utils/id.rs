use snowflaker::next_id;

pub fn get_unique_id() -> anyhow::Result<u64> {
    next_id().map_err(|e| anyhow::anyhow!("Could not generate unique id: {}", e))
}
