// src/lib.rs
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("channel_remap.h");

        type ChannelMapper;

        fn create_channel_mapper() -> UniquePtr<ChannelMapper>;
        fn build_remap_table(
            self: Pin<&mut ChannelMapper>,
            channel_map: &CxxVector<u32>,
            target_channel_map: &CxxVector<u32>,
        );
        fn get_remap_table(self: &ChannelMapper) -> &CxxVector<usize>;
    }
}

// Rust implementation
pub struct RustChannelMapper {
    remap_table: Vec<usize>,
}

impl RustChannelMapper {
    pub fn new() -> Self {
        Self {
            remap_table: Vec::new(),
        }
    }

    pub fn build_remap_table(
        &mut self,
        channel_map: &[u32],
        target_channel_map: &[u32],
    ) {
        let channel_count = target_channel_map.len();
        self.remap_table.clear();
        self.remap_table.reserve(channel_count);

        for i in 0..channel_count {
            let target_channel = target_channel_map[i];
            let index = channel_map
                .iter()
                .position(|&ch| ch == target_channel)
                .unwrap_or(channel_map.len());

            self.remap_table.push(index);
        }
    }

    pub fn get_remap_table(&self) -> &[usize] {
        &self.remap_table
    }
}

// Alternative: Pure Rust version without FFI
pub fn build_remap_table_rust(
    channel_map: &[u32],
    target_channel_map: &[u32],
) -> Vec<usize> {
    target_channel_map
        .iter()
        .map(|&target| {
            channel_map
                .iter()
                .position(|&ch| ch == target)
                .unwrap_or(channel_map.len())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remap_table() {
        let channel_map = vec![0, 1, 2, 3, 4, 5];
        let target_channel_map = vec![2, 0, 4, 1];

        let result = build_remap_table_rust(&channel_map, &target_channel_map);
        assert_eq!(result, vec![2, 0, 4, 1]);
    }

    #[test]
    fn test_remap_table_with_missing() {
        let channel_map = vec![0, 1, 2];
        let target_channel_map = vec![1, 5, 0]; // 5 doesn't exist

        let result = build_remap_table_rust(&channel_map, &target_channel_map);
        assert_eq!(result, vec![1, 3, 0]); // 3 is channel_map.len()
    }
}