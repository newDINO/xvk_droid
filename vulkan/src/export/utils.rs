use super::INITS;

pub unsafe fn find_memory_type(supported_bit: u32, required_type: u32) -> Option<u32> {
    for i in 0..INITS.memory_properties.memoryTypeCount {
        let supported = (supported_bit & (1 << i)) != 0;
        let needed = (INITS.memory_properties.memoryTypes[i as usize].propertyFlags & required_type) != 0;
        if supported && needed {
            return Some(i)
        }
    }
    None
}