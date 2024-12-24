use crate::pfn::*;
use vulkan_core::*;

#[no_mangle]
pub unsafe extern "C" fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks) {
    VK_DESTROY_INSTANCE.unwrap()(instance, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumeratePhysicalDevices(
    instance: VkInstance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut VkPhysicalDevice,
) -> VkResult {
    VK_ENUMERATE_PHYSICAL_DEVICES.unwrap()(instance, pPhysicalDeviceCount, pPhysicalDevices)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFeatures(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures,
) {
    VK_GET_PHYSICAL_DEVICE_FEATURES.unwrap()(physicalDevice, pFeatures)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties,
) {
    VK_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES.unwrap()(physicalDevice, format, pFormatProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceImageFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    pImageFormatProperties: *mut VkImageFormatProperties,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES.unwrap()(physicalDevice, format, type_, tiling, usage, flags, pImageFormatProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceProperties(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties,
) {
    VK_GET_PHYSICAL_DEVICE_PROPERTIES.unwrap()(physicalDevice, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceQueueFamilyProperties(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties,
) {
    VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES.unwrap()(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceMemoryProperties(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
) {
    VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES.unwrap()(physicalDevice, pMemoryProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDevice(
    physicalDevice: VkPhysicalDevice,
    pCreateInfo: *const VkDeviceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDevice: *mut VkDevice,
) -> VkResult {
    VK_CREATE_DEVICE.unwrap()(physicalDevice, pCreateInfo, pAllocator, pDevice)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks) {
    VK_DESTROY_DEVICE.unwrap()(device, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateDeviceExtensionProperties(
    physicalDevice: VkPhysicalDevice,
    pLayerName: *const ::std::os::raw::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult {
    VK_ENUMERATE_DEVICE_EXTENSION_PROPERTIES.unwrap()(physicalDevice, pLayerName, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceLayerProperties(
    pPropertyCount: *mut u32,
    pProperties: *mut VkLayerProperties,
) -> VkResult {
    VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES.unwrap()(pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateDeviceLayerProperties(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkLayerProperties,
) -> VkResult {
    VK_ENUMERATE_DEVICE_LAYER_PROPERTIES.unwrap()(physicalDevice, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceQueue(
    device: VkDevice,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: *mut VkQueue,
) {
    VK_GET_DEVICE_QUEUE.unwrap()(device, queueFamilyIndex, queueIndex, pQueue)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueSubmit(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo,
    fence: VkFence,
) -> VkResult {
    VK_QUEUE_SUBMIT.unwrap()(queue, submitCount, pSubmits, fence)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueWaitIdle(queue: VkQueue) -> VkResult {
    VK_QUEUE_WAIT_IDLE.unwrap()(queue)
}

#[no_mangle]
pub unsafe extern "C" fn vkDeviceWaitIdle(device: VkDevice) -> VkResult {
    VK_DEVICE_WAIT_IDLE.unwrap()(device)
}

#[no_mangle]
pub unsafe extern "C" fn vkAllocateMemory(
    device: VkDevice,
    pAllocateInfo: *const VkMemoryAllocateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pMemory: *mut VkDeviceMemory,
) -> VkResult {
    VK_ALLOCATE_MEMORY.unwrap()(device, pAllocateInfo, pAllocator, pMemory)
}

#[no_mangle]
pub unsafe extern "C" fn vkFreeMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_FREE_MEMORY.unwrap()(device, memory, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkMapMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    ppData: *mut *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_MAP_MEMORY.unwrap()(device, memory, offset, size, flags, ppData)
}

#[no_mangle]
pub unsafe extern "C" fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory) {
    VK_UNMAP_MEMORY.unwrap()(device, memory)
}

#[no_mangle]
pub unsafe extern "C" fn vkFlushMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult {
    VK_FLUSH_MAPPED_MEMORY_RANGES.unwrap()(device, memoryRangeCount, pMemoryRanges)
}

#[no_mangle]
pub unsafe extern "C" fn vkInvalidateMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult {
    VK_INVALIDATE_MAPPED_MEMORY_RANGES.unwrap()(device, memoryRangeCount, pMemoryRanges)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceMemoryCommitment(
    device: VkDevice,
    memory: VkDeviceMemory,
    pCommittedMemoryInBytes: *mut VkDeviceSize,
) {
    VK_GET_DEVICE_MEMORY_COMMITMENT.unwrap()(device, memory, pCommittedMemoryInBytes)
}

#[no_mangle]
pub unsafe extern "C" fn vkBindBufferMemory(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    VK_BIND_BUFFER_MEMORY.unwrap()(device, buffer, memory, memoryOffset)
}

#[no_mangle]
pub unsafe extern "C" fn vkBindImageMemory(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    VK_BIND_IMAGE_MEMORY.unwrap()(device, image, memory, memoryOffset)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferMemoryRequirements(
    device: VkDevice,
    buffer: VkBuffer,
    pMemoryRequirements: *mut VkMemoryRequirements,
) {
    VK_GET_BUFFER_MEMORY_REQUIREMENTS.unwrap()(device, buffer, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pMemoryRequirements: *mut VkMemoryRequirements,
) {
    VK_GET_IMAGE_MEMORY_REQUIREMENTS.unwrap()(device, image, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSparseMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
) {
    VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS.unwrap()(device, image, pSparseMemoryRequirementCount, pSparseMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSparseImageFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    samples: VkSampleCountFlagBits,
    usage: VkImageUsageFlags,
    tiling: VkImageTiling,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties,
) {
    VK_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES.unwrap()(physicalDevice, format, type_, samples, usage, tiling, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueBindSparse(
    queue: VkQueue,
    bindInfoCount: u32,
    pBindInfo: *const VkBindSparseInfo,
    fence: VkFence,
) -> VkResult {
    VK_QUEUE_BIND_SPARSE.unwrap()(queue, bindInfoCount, pBindInfo, fence)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateFence(
    device: VkDevice,
    pCreateInfo: *const VkFenceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult {
    VK_CREATE_FENCE.unwrap()(device, pCreateInfo, pAllocator, pFence)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyFence(
    device: VkDevice,
    fence: VkFence,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_FENCE.unwrap()(device, fence, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult {
    VK_RESET_FENCES.unwrap()(device, fenceCount, pFences)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult {
    VK_GET_FENCE_STATUS.unwrap()(device, fence)
}

#[no_mangle]
pub unsafe extern "C" fn vkWaitForFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: *const VkFence,
    waitAll: VkBool32,
    timeout: u64,
) -> VkResult {
    VK_WAIT_FOR_FENCES.unwrap()(device, fenceCount, pFences, waitAll, timeout)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSemaphore(
    device: VkDevice,
    pCreateInfo: *const VkSemaphoreCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSemaphore: *mut VkSemaphore,
) -> VkResult {
    VK_CREATE_SEMAPHORE.unwrap()(device, pCreateInfo, pAllocator, pSemaphore)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySemaphore(
    device: VkDevice,
    semaphore: VkSemaphore,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_SEMAPHORE.unwrap()(device, semaphore, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateEvent(
    device: VkDevice,
    pCreateInfo: *const VkEventCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pEvent: *mut VkEvent,
) -> VkResult {
    VK_CREATE_EVENT.unwrap()(device, pCreateInfo, pAllocator, pEvent)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyEvent(
    device: VkDevice,
    event: VkEvent,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_EVENT.unwrap()(device, event, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult {
    VK_GET_EVENT_STATUS.unwrap()(device, event)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    VK_SET_EVENT.unwrap()(device, event)
}

#[no_mangle]
pub unsafe extern "C" fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    VK_RESET_EVENT.unwrap()(device, event)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateQueryPool(
    device: VkDevice,
    pCreateInfo: *const VkQueryPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pQueryPool: *mut VkQueryPool,
) -> VkResult {
    VK_CREATE_QUERY_POOL.unwrap()(device, pCreateInfo, pAllocator, pQueryPool)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_QUERY_POOL.unwrap()(device, queryPool, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetQueryPoolResults(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) -> VkResult {
    VK_GET_QUERY_POOL_RESULTS.unwrap()(device, queryPool, firstQuery, queryCount, dataSize, pData, stride, flags)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateBuffer(
    device: VkDevice,
    pCreateInfo: *const VkBufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pBuffer: *mut VkBuffer,
) -> VkResult {
    VK_CREATE_BUFFER.unwrap()(device, pCreateInfo, pAllocator, pBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyBuffer(
    device: VkDevice,
    buffer: VkBuffer,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_BUFFER.unwrap()(device, buffer, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateBufferView(
    device: VkDevice,
    pCreateInfo: *const VkBufferViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkBufferView,
) -> VkResult {
    VK_CREATE_BUFFER_VIEW.unwrap()(device, pCreateInfo, pAllocator, pView)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyBufferView(
    device: VkDevice,
    bufferView: VkBufferView,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_BUFFER_VIEW.unwrap()(device, bufferView, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateImage(
    device: VkDevice,
    pCreateInfo: *const VkImageCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pImage: *mut VkImage,
) -> VkResult {
    VK_CREATE_IMAGE.unwrap()(device, pCreateInfo, pAllocator, pImage)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyImage(
    device: VkDevice,
    image: VkImage,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_IMAGE.unwrap()(device, image, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSubresourceLayout(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource,
    pLayout: *mut VkSubresourceLayout,
) {
    VK_GET_IMAGE_SUBRESOURCE_LAYOUT.unwrap()(device, image, pSubresource, pLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateImageView(
    device: VkDevice,
    pCreateInfo: *const VkImageViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkImageView,
) -> VkResult {
    VK_CREATE_IMAGE_VIEW.unwrap()(device, pCreateInfo, pAllocator, pView)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyImageView(
    device: VkDevice,
    imageView: VkImageView,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_IMAGE_VIEW.unwrap()(device, imageView, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateShaderModule(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pShaderModule: *mut VkShaderModule,
) -> VkResult {
    VK_CREATE_SHADER_MODULE.unwrap()(device, pCreateInfo, pAllocator, pShaderModule)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyShaderModule(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_SHADER_MODULE.unwrap()(device, shaderModule, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreatePipelineCache(
    device: VkDevice,
    pCreateInfo: *const VkPipelineCacheCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineCache: *mut VkPipelineCache,
) -> VkResult {
    VK_CREATE_PIPELINE_CACHE.unwrap()(device, pCreateInfo, pAllocator, pPipelineCache)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPipelineCache(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_PIPELINE_CACHE.unwrap()(device, pipelineCache, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineCacheData(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pDataSize: *mut usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_PIPELINE_CACHE_DATA.unwrap()(device, pipelineCache, pDataSize, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkMergePipelineCaches(
    device: VkDevice,
    dstCache: VkPipelineCache,
    srcCacheCount: u32,
    pSrcCaches: *const VkPipelineCache,
) -> VkResult {
    VK_MERGE_PIPELINE_CACHES.unwrap()(device, dstCache, srcCacheCount, pSrcCaches)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateGraphicsPipelines(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkGraphicsPipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    VK_CREATE_GRAPHICS_PIPELINES.unwrap()(device, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateComputePipelines(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkComputePipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    VK_CREATE_COMPUTE_PIPELINES.unwrap()(device, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPipeline(
    device: VkDevice,
    pipeline: VkPipeline,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_PIPELINE.unwrap()(device, pipeline, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreatePipelineLayout(
    device: VkDevice,
    pCreateInfo: *const VkPipelineLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineLayout: *mut VkPipelineLayout,
) -> VkResult {
    VK_CREATE_PIPELINE_LAYOUT.unwrap()(device, pCreateInfo, pAllocator, pPipelineLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPipelineLayout(
    device: VkDevice,
    pipelineLayout: VkPipelineLayout,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_PIPELINE_LAYOUT.unwrap()(device, pipelineLayout, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSampler(
    device: VkDevice,
    pCreateInfo: *const VkSamplerCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSampler: *mut VkSampler,
) -> VkResult {
    VK_CREATE_SAMPLER.unwrap()(device, pCreateInfo, pAllocator, pSampler)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySampler(
    device: VkDevice,
    sampler: VkSampler,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_SAMPLER.unwrap()(device, sampler, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDescriptorSetLayout(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSetLayout: *mut VkDescriptorSetLayout,
) -> VkResult {
    VK_CREATE_DESCRIPTOR_SET_LAYOUT.unwrap()(device, pCreateInfo, pAllocator, pSetLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDescriptorSetLayout(
    device: VkDevice,
    descriptorSetLayout: VkDescriptorSetLayout,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_DESCRIPTOR_SET_LAYOUT.unwrap()(device, descriptorSetLayout, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDescriptorPool(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorPool: *mut VkDescriptorPool,
) -> VkResult {
    VK_CREATE_DESCRIPTOR_POOL.unwrap()(device, pCreateInfo, pAllocator, pDescriptorPool)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_DESCRIPTOR_POOL.unwrap()(device, descriptorPool, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkResetDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    flags: VkDescriptorPoolResetFlags,
) -> VkResult {
    VK_RESET_DESCRIPTOR_POOL.unwrap()(device, descriptorPool, flags)
}

#[no_mangle]
pub unsafe extern "C" fn vkAllocateDescriptorSets(
    device: VkDevice,
    pAllocateInfo: *const VkDescriptorSetAllocateInfo,
    pDescriptorSets: *mut VkDescriptorSet,
) -> VkResult {
    VK_ALLOCATE_DESCRIPTOR_SETS.unwrap()(device, pAllocateInfo, pDescriptorSets)
}

#[no_mangle]
pub unsafe extern "C" fn vkFreeDescriptorSets(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
) -> VkResult {
    VK_FREE_DESCRIPTOR_SETS.unwrap()(device, descriptorPool, descriptorSetCount, pDescriptorSets)
}

#[no_mangle]
pub unsafe extern "C" fn vkUpdateDescriptorSets(
    device: VkDevice,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
    descriptorCopyCount: u32,
    pDescriptorCopies: *const VkCopyDescriptorSet,
) {
    VK_UPDATE_DESCRIPTOR_SETS.unwrap()(device, descriptorWriteCount, pDescriptorWrites, descriptorCopyCount, pDescriptorCopies)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateFramebuffer(
    device: VkDevice,
    pCreateInfo: *const VkFramebufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFramebuffer: *mut VkFramebuffer,
) -> VkResult {
    VK_CREATE_FRAMEBUFFER.unwrap()(device, pCreateInfo, pAllocator, pFramebuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyFramebuffer(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_FRAMEBUFFER.unwrap()(device, framebuffer, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateRenderPass(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult {
    VK_CREATE_RENDER_PASS.unwrap()(device, pCreateInfo, pAllocator, pRenderPass)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyRenderPass(
    device: VkDevice,
    renderPass: VkRenderPass,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_RENDER_PASS.unwrap()(device, renderPass, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRenderAreaGranularity(
    device: VkDevice,
    renderPass: VkRenderPass,
    pGranularity: *mut VkExtent2D,
) {
    VK_GET_RENDER_AREA_GRANULARITY.unwrap()(device, renderPass, pGranularity)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateCommandPool(
    device: VkDevice,
    pCreateInfo: *const VkCommandPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pCommandPool: *mut VkCommandPool,
) -> VkResult {
    VK_CREATE_COMMAND_POOL.unwrap()(device, pCreateInfo, pAllocator, pCommandPool)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_COMMAND_POOL.unwrap()(device, commandPool, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkResetCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolResetFlags,
) -> VkResult {
    VK_RESET_COMMAND_POOL.unwrap()(device, commandPool, flags)
}

#[no_mangle]
pub unsafe extern "C" fn vkAllocateCommandBuffers(
    device: VkDevice,
    pAllocateInfo: *const VkCommandBufferAllocateInfo,
    pCommandBuffers: *mut VkCommandBuffer,
) -> VkResult {
    VK_ALLOCATE_COMMAND_BUFFERS.unwrap()(device, pAllocateInfo, pCommandBuffers)
}

#[no_mangle]
pub unsafe extern "C" fn vkFreeCommandBuffers(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
) {
    VK_FREE_COMMAND_BUFFERS.unwrap()(device, commandPool, commandBufferCount, pCommandBuffers)
}

#[no_mangle]
pub unsafe extern "C" fn vkBeginCommandBuffer(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: *const VkCommandBufferBeginInfo,
) -> VkResult {
    VK_BEGIN_COMMAND_BUFFER.unwrap()(commandBuffer, pBeginInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult {
    VK_END_COMMAND_BUFFER.unwrap()(commandBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkResetCommandBuffer(
    commandBuffer: VkCommandBuffer,
    flags: VkCommandBufferResetFlags,
) -> VkResult {
    VK_RESET_COMMAND_BUFFER.unwrap()(commandBuffer, flags)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindPipeline(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
) {
    VK_CMD_BIND_PIPELINE.unwrap()(commandBuffer, pipelineBindPoint, pipeline)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewport(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: *const VkViewport,
) {
    VK_CMD_SET_VIEWPORT.unwrap()(commandBuffer, firstViewport, viewportCount, pViewports)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetScissor(
    commandBuffer: VkCommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: *const VkRect2D,
) {
    VK_CMD_SET_SCISSOR.unwrap()(commandBuffer, firstScissor, scissorCount, pScissors)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32) {
    VK_CMD_SET_LINE_WIDTH.unwrap()(commandBuffer, lineWidth)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBias(
    commandBuffer: VkCommandBuffer,
    depthBiasConstantFactor: f32,
    depthBiasClamp: f32,
    depthBiasSlopeFactor: f32,
) {
    VK_CMD_SET_DEPTH_BIAS.unwrap()(commandBuffer, depthBiasConstantFactor, depthBiasClamp, depthBiasSlopeFactor)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const f32) {
    VK_CMD_SET_BLEND_CONSTANTS.unwrap()(commandBuffer, blendConstants)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBounds(
    commandBuffer: VkCommandBuffer,
    minDepthBounds: f32,
    maxDepthBounds: f32,
) {
    VK_CMD_SET_DEPTH_BOUNDS.unwrap()(commandBuffer, minDepthBounds, maxDepthBounds)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilCompareMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    compareMask: u32,
) {
    VK_CMD_SET_STENCIL_COMPARE_MASK.unwrap()(commandBuffer, faceMask, compareMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilWriteMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    writeMask: u32,
) {
    VK_CMD_SET_STENCIL_WRITE_MASK.unwrap()(commandBuffer, faceMask, writeMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilReference(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    reference: u32,
) {
    VK_CMD_SET_STENCIL_REFERENCE.unwrap()(commandBuffer, faceMask, reference)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindDescriptorSets(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
    dynamicOffsetCount: u32,
    pDynamicOffsets: *const u32,
) {
    VK_CMD_BIND_DESCRIPTOR_SETS.unwrap()(commandBuffer, pipelineBindPoint, layout, firstSet, descriptorSetCount, pDescriptorSets, dynamicOffsetCount, pDynamicOffsets)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindIndexBuffer(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    indexType: VkIndexType,
) {
    VK_CMD_BIND_INDEX_BUFFER.unwrap()(commandBuffer, buffer, offset, indexType)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindVertexBuffers(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
) {
    VK_CMD_BIND_VERTEX_BUFFERS.unwrap()(commandBuffer, firstBinding, bindingCount, pBuffers, pOffsets)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDraw(
    commandBuffer: VkCommandBuffer,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
) {
    VK_CMD_DRAW.unwrap()(commandBuffer, vertexCount, instanceCount, firstVertex, firstInstance)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndexed(
    commandBuffer: VkCommandBuffer,
    indexCount: u32,
    instanceCount: u32,
    firstIndex: u32,
    vertexOffset: i32,
    firstInstance: u32,
) {
    VK_CMD_DRAW_INDEXED.unwrap()(commandBuffer, indexCount, instanceCount, firstIndex, vertexOffset, firstInstance)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_INDIRECT.unwrap()(commandBuffer, buffer, offset, drawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndexedIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_INDEXED_INDIRECT.unwrap()(commandBuffer, buffer, offset, drawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDispatch(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    VK_CMD_DISPATCH.unwrap()(commandBuffer, groupCountX, groupCountY, groupCountZ)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDispatchIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    VK_CMD_DISPATCH_INDIRECT.unwrap()(commandBuffer, buffer, offset)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBuffer(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferCopy,
) {
    VK_CMD_COPY_BUFFER.unwrap()(commandBuffer, srcBuffer, dstBuffer, regionCount, pRegions)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImage(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageCopy,
) {
    VK_CMD_COPY_IMAGE.unwrap()(commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBlitImage(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageBlit,
    filter: VkFilter,
) {
    VK_CMD_BLIT_IMAGE.unwrap()(commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions, filter)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBufferToImage(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
) {
    VK_CMD_COPY_BUFFER_TO_IMAGE.unwrap()(commandBuffer, srcBuffer, dstImage, dstImageLayout, regionCount, pRegions)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImageToBuffer(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
) {
    VK_CMD_COPY_IMAGE_TO_BUFFER.unwrap()(commandBuffer, srcImage, srcImageLayout, dstBuffer, regionCount, pRegions)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdUpdateBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    dataSize: VkDeviceSize,
    pData: *const ::std::os::raw::c_void,
) {
    VK_CMD_UPDATE_BUFFER.unwrap()(commandBuffer, dstBuffer, dstOffset, dataSize, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdFillBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    size: VkDeviceSize,
    data: u32,
) {
    VK_CMD_FILL_BUFFER.unwrap()(commandBuffer, dstBuffer, dstOffset, size, data)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdClearColorImage(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pColor: *const VkClearColorValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
) {
    VK_CMD_CLEAR_COLOR_IMAGE.unwrap()(commandBuffer, image, imageLayout, pColor, rangeCount, pRanges)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdClearDepthStencilImage(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pDepthStencil: *const VkClearDepthStencilValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
) {
    VK_CMD_CLEAR_DEPTH_STENCIL_IMAGE.unwrap()(commandBuffer, image, imageLayout, pDepthStencil, rangeCount, pRanges)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdClearAttachments(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pAttachments: *const VkClearAttachment,
    rectCount: u32,
    pRects: *const VkClearRect,
) {
    VK_CMD_CLEAR_ATTACHMENTS.unwrap()(commandBuffer, attachmentCount, pAttachments, rectCount, pRects)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResolveImage(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageResolve,
) {
    VK_CMD_RESOLVE_IMAGE.unwrap()(commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    VK_CMD_SET_EVENT.unwrap()(commandBuffer, event, stageMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    VK_CMD_RESET_EVENT.unwrap()(commandBuffer, event, stageMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWaitEvents(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const VkMemoryBarrier,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const VkImageMemoryBarrier,
) {
    VK_CMD_WAIT_EVENTS.unwrap()(commandBuffer, eventCount, pEvents, srcStageMask, dstStageMask, memoryBarrierCount, pMemoryBarriers, bufferMemoryBarrierCount, pBufferMemoryBarriers, imageMemoryBarrierCount, pImageMemoryBarriers)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPipelineBarrier(
    commandBuffer: VkCommandBuffer,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    dependencyFlags: VkDependencyFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const VkMemoryBarrier,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const VkImageMemoryBarrier,
) {
    VK_CMD_PIPELINE_BARRIER.unwrap()(commandBuffer, srcStageMask, dstStageMask, dependencyFlags, memoryBarrierCount, pMemoryBarriers, bufferMemoryBarrierCount, pBufferMemoryBarriers, imageMemoryBarrierCount, pImageMemoryBarriers)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginQuery(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
) {
    VK_CMD_BEGIN_QUERY.unwrap()(commandBuffer, queryPool, query, flags)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32) {
    VK_CMD_END_QUERY.unwrap()(commandBuffer, queryPool, query)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResetQueryPool(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    VK_CMD_RESET_QUERY_POOL.unwrap()(commandBuffer, queryPool, firstQuery, queryCount)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteTimestamp(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    queryPool: VkQueryPool,
    query: u32,
) {
    VK_CMD_WRITE_TIMESTAMP.unwrap()(commandBuffer, pipelineStage, queryPool, query)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyQueryPoolResults(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) {
    VK_CMD_COPY_QUERY_POOL_RESULTS.unwrap()(commandBuffer, queryPool, firstQuery, queryCount, dstBuffer, dstOffset, stride, flags)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushConstants(
    commandBuffer: VkCommandBuffer,
    layout: VkPipelineLayout,
    stageFlags: VkShaderStageFlags,
    offset: u32,
    size: u32,
    pValues: *const ::std::os::raw::c_void,
) {
    VK_CMD_PUSH_CONSTANTS.unwrap()(commandBuffer, layout, stageFlags, offset, size, pValues)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginRenderPass(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    contents: VkSubpassContents,
) {
    VK_CMD_BEGIN_RENDER_PASS.unwrap()(commandBuffer, pRenderPassBegin, contents)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents) {
    VK_CMD_NEXT_SUBPASS.unwrap()(commandBuffer, contents)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) {
    VK_CMD_END_RENDER_PASS.unwrap()(commandBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdExecuteCommands(
    commandBuffer: VkCommandBuffer,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
) {
    VK_CMD_EXECUTE_COMMANDS.unwrap()(commandBuffer, commandBufferCount, pCommandBuffers)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceVersion(pApiVersion: *mut u32) -> VkResult {
    VK_ENUMERATE_INSTANCE_VERSION.unwrap()(pApiVersion)
}

#[no_mangle]
pub unsafe extern "C" fn vkBindBufferMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindBufferMemoryInfo,
) -> VkResult {
    VK_BIND_BUFFER_MEMORY2.unwrap()(device, bindInfoCount, pBindInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkBindImageMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindImageMemoryInfo,
) -> VkResult {
    VK_BIND_IMAGE_MEMORY2.unwrap()(device, bindInfoCount, pBindInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceGroupPeerMemoryFeatures(
    device: VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags,
) {
    VK_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES.unwrap()(device, heapIndex, localDeviceIndex, remoteDeviceIndex, pPeerMemoryFeatures)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32) {
    VK_CMD_SET_DEVICE_MASK.unwrap()(commandBuffer, deviceMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDispatchBase(
    commandBuffer: VkCommandBuffer,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    VK_CMD_DISPATCH_BASE.unwrap()(commandBuffer, baseGroupX, baseGroupY, baseGroupZ, groupCountX, groupCountY, groupCountZ)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumeratePhysicalDeviceGroups(
    instance: VkInstance,
    pPhysicalDeviceGroupCount: *mut u32,
    pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
) -> VkResult {
    VK_ENUMERATE_PHYSICAL_DEVICE_GROUPS.unwrap()(instance, pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageMemoryRequirements2(
    device: VkDevice,
    pInfo: *const VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_IMAGE_MEMORY_REQUIREMENTS2.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferMemoryRequirements2(
    device: VkDevice,
    pInfo: *const VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_BUFFER_MEMORY_REQUIREMENTS2.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSparseMemoryRequirements2(
    device: VkDevice,
    pInfo: *const VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
) {
    VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2.unwrap()(device, pInfo, pSparseMemoryRequirementCount, pSparseMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFeatures2(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures2,
) {
    VK_GET_PHYSICAL_DEVICE_FEATURES2.unwrap()(physicalDevice, pFeatures)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceProperties2(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_PROPERTIES2.unwrap()(physicalDevice, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2.unwrap()(physicalDevice, format, pFormatProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceImageFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
    pImageFormatProperties: *mut VkImageFormatProperties2,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2.unwrap()(physicalDevice, pImageFormatInfo, pImageFormatProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceQueueFamilyProperties2(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2.unwrap()(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceMemoryProperties2(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2.unwrap()(physicalDevice, pMemoryProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSparseImageFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2.unwrap()(physicalDevice, pFormatInfo, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkTrimCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolTrimFlags,
) {
    VK_TRIM_COMMAND_POOL.unwrap()(device, commandPool, flags)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceQueue2(
    device: VkDevice,
    pQueueInfo: *const VkDeviceQueueInfo2,
    pQueue: *mut VkQueue,
) {
    VK_GET_DEVICE_QUEUE2.unwrap()(device, pQueueInfo, pQueue)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSamplerYcbcrConversion(
    device: VkDevice,
    pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult {
    VK_CREATE_SAMPLER_YCBCR_CONVERSION.unwrap()(device, pCreateInfo, pAllocator, pYcbcrConversion)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySamplerYcbcrConversion(
    device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_SAMPLER_YCBCR_CONVERSION.unwrap()(device, ycbcrConversion, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDescriptorUpdateTemplate(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult {
    VK_CREATE_DESCRIPTOR_UPDATE_TEMPLATE.unwrap()(device, pCreateInfo, pAllocator, pDescriptorUpdateTemplate)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDescriptorUpdateTemplate(
    device: VkDevice,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE.unwrap()(device, descriptorUpdateTemplate, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkUpdateDescriptorSetWithTemplate(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pData: *const ::std::os::raw::c_void,
) {
    VK_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE.unwrap()(device, descriptorSet, descriptorUpdateTemplate, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalBufferProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
    pExternalBufferProperties: *mut VkExternalBufferProperties,
) {
    VK_GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES.unwrap()(physicalDevice, pExternalBufferInfo, pExternalBufferProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalFenceProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
    pExternalFenceProperties: *mut VkExternalFenceProperties,
) {
    VK_GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES.unwrap()(physicalDevice, pExternalFenceInfo, pExternalFenceProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalSemaphoreProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
    pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
) {
    VK_GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES.unwrap()(physicalDevice, pExternalSemaphoreInfo, pExternalSemaphoreProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetLayoutSupport(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pSupport: *mut VkDescriptorSetLayoutSupport,
) {
    VK_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT.unwrap()(device, pCreateInfo, pSupport)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndirectCount(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_INDIRECT_COUNT.unwrap()(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndexedIndirectCount(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_INDEXED_INDIRECT_COUNT.unwrap()(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateRenderPass2(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo2,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult {
    VK_CREATE_RENDER_PASS2.unwrap()(device, pCreateInfo, pAllocator, pRenderPass)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginRenderPass2(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
) {
    VK_CMD_BEGIN_RENDER_PASS2.unwrap()(commandBuffer, pRenderPassBegin, pSubpassBeginInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdNextSubpass2(
    commandBuffer: VkCommandBuffer,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
    pSubpassEndInfo: *const VkSubpassEndInfo,
) {
    VK_CMD_NEXT_SUBPASS2.unwrap()(commandBuffer, pSubpassBeginInfo, pSubpassEndInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndRenderPass2(
    commandBuffer: VkCommandBuffer,
    pSubpassEndInfo: *const VkSubpassEndInfo,
) {
    VK_CMD_END_RENDER_PASS2.unwrap()(commandBuffer, pSubpassEndInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkResetQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    VK_RESET_QUERY_POOL.unwrap()(device, queryPool, firstQuery, queryCount)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSemaphoreCounterValue(
    device: VkDevice,
    semaphore: VkSemaphore,
    pValue: *mut u64,
) -> VkResult {
    VK_GET_SEMAPHORE_COUNTER_VALUE.unwrap()(device, semaphore, pValue)
}

#[no_mangle]
pub unsafe extern "C" fn vkWaitSemaphores(
    device: VkDevice,
    pWaitInfo: *const VkSemaphoreWaitInfo,
    timeout: u64,
) -> VkResult {
    VK_WAIT_SEMAPHORES.unwrap()(device, pWaitInfo, timeout)
}

#[no_mangle]
pub unsafe extern "C" fn vkSignalSemaphore(
    device: VkDevice,
    pSignalInfo: *const VkSemaphoreSignalInfo,
) -> VkResult {
    VK_SIGNAL_SEMAPHORE.unwrap()(device, pSignalInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferDeviceAddress(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress {
    VK_GET_BUFFER_DEVICE_ADDRESS.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> u64 {
    VK_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceMemoryOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo,
) -> u64 {
    VK_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceToolProperties(
    physicalDevice: VkPhysicalDevice,
    pToolCount: *mut u32,
    pToolProperties: *mut VkPhysicalDeviceToolProperties,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_TOOL_PROPERTIES.unwrap()(physicalDevice, pToolCount, pToolProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreatePrivateDataSlot(
    device: VkDevice,
    pCreateInfo: *const VkPrivateDataSlotCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPrivateDataSlot: *mut VkPrivateDataSlot,
) -> VkResult {
    VK_CREATE_PRIVATE_DATA_SLOT.unwrap()(device, pCreateInfo, pAllocator, pPrivateDataSlot)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPrivateDataSlot(
    device: VkDevice,
    privateDataSlot: VkPrivateDataSlot,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_PRIVATE_DATA_SLOT.unwrap()(device, privateDataSlot, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetPrivateData(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
) -> VkResult {
    VK_SET_PRIVATE_DATA.unwrap()(device, objectType, objectHandle, privateDataSlot, data)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPrivateData(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: *mut u64,
) {
    VK_GET_PRIVATE_DATA.unwrap()(device, objectType, objectHandle, privateDataSlot, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    pDependencyInfo: *const VkDependencyInfo,
) {
    VK_CMD_SET_EVENT2.unwrap()(commandBuffer, event, pDependencyInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags2,
) {
    VK_CMD_RESET_EVENT2.unwrap()(commandBuffer, event, stageMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWaitEvents2(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    pDependencyInfos: *const VkDependencyInfo,
) {
    VK_CMD_WAIT_EVENTS2.unwrap()(commandBuffer, eventCount, pEvents, pDependencyInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPipelineBarrier2(
    commandBuffer: VkCommandBuffer,
    pDependencyInfo: *const VkDependencyInfo,
) {
    VK_CMD_PIPELINE_BARRIER2.unwrap()(commandBuffer, pDependencyInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteTimestamp2(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
) {
    VK_CMD_WRITE_TIMESTAMP2.unwrap()(commandBuffer, stage, queryPool, query)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueSubmit2(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo2,
    fence: VkFence,
) -> VkResult {
    VK_QUEUE_SUBMIT2.unwrap()(queue, submitCount, pSubmits, fence)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferInfo: *const VkCopyBufferInfo2,
) {
    VK_CMD_COPY_BUFFER2.unwrap()(commandBuffer, pCopyBufferInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImage2(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2) {
    VK_CMD_COPY_IMAGE2.unwrap()(commandBuffer, pCopyImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBufferToImage2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2,
) {
    VK_CMD_COPY_BUFFER_TO_IMAGE2.unwrap()(commandBuffer, pCopyBufferToImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImageToBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2,
) {
    VK_CMD_COPY_IMAGE_TO_BUFFER2.unwrap()(commandBuffer, pCopyImageToBufferInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBlitImage2(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2) {
    VK_CMD_BLIT_IMAGE2.unwrap()(commandBuffer, pBlitImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResolveImage2(
    commandBuffer: VkCommandBuffer,
    pResolveImageInfo: *const VkResolveImageInfo2,
) {
    VK_CMD_RESOLVE_IMAGE2.unwrap()(commandBuffer, pResolveImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginRendering(
    commandBuffer: VkCommandBuffer,
    pRenderingInfo: *const VkRenderingInfo,
) {
    VK_CMD_BEGIN_RENDERING.unwrap()(commandBuffer, pRenderingInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndRendering(commandBuffer: VkCommandBuffer) {
    VK_CMD_END_RENDERING.unwrap()(commandBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCullMode(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags) {
    VK_CMD_SET_CULL_MODE.unwrap()(commandBuffer, cullMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetFrontFace(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace) {
    VK_CMD_SET_FRONT_FACE.unwrap()(commandBuffer, frontFace)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPrimitiveTopology(
    commandBuffer: VkCommandBuffer,
    primitiveTopology: VkPrimitiveTopology,
) {
    VK_CMD_SET_PRIMITIVE_TOPOLOGY.unwrap()(commandBuffer, primitiveTopology)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportWithCount(
    commandBuffer: VkCommandBuffer,
    viewportCount: u32,
    pViewports: *const VkViewport,
) {
    VK_CMD_SET_VIEWPORT_WITH_COUNT.unwrap()(commandBuffer, viewportCount, pViewports)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetScissorWithCount(
    commandBuffer: VkCommandBuffer,
    scissorCount: u32,
    pScissors: *const VkRect2D,
) {
    VK_CMD_SET_SCISSOR_WITH_COUNT.unwrap()(commandBuffer, scissorCount, pScissors)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindVertexBuffers2(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
    pStrides: *const VkDeviceSize,
) {
    VK_CMD_BIND_VERTEX_BUFFERS2.unwrap()(commandBuffer, firstBinding, bindingCount, pBuffers, pOffsets, pSizes, pStrides)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthTestEnable(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32) {
    VK_CMD_SET_DEPTH_TEST_ENABLE.unwrap()(commandBuffer, depthTestEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthWriteEnable(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32) {
    VK_CMD_SET_DEPTH_WRITE_ENABLE.unwrap()(commandBuffer, depthWriteEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthCompareOp(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp) {
    VK_CMD_SET_DEPTH_COMPARE_OP.unwrap()(commandBuffer, depthCompareOp)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBoundsTestEnable(
    commandBuffer: VkCommandBuffer,
    depthBoundsTestEnable: VkBool32,
) {
    VK_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE.unwrap()(commandBuffer, depthBoundsTestEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilTestEnable(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32) {
    VK_CMD_SET_STENCIL_TEST_ENABLE.unwrap()(commandBuffer, stencilTestEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilOp(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
) {
    VK_CMD_SET_STENCIL_OP.unwrap()(commandBuffer, faceMask, failOp, passOp, depthFailOp, compareOp)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRasterizerDiscardEnable(
    commandBuffer: VkCommandBuffer,
    rasterizerDiscardEnable: VkBool32,
) {
    VK_CMD_SET_RASTERIZER_DISCARD_ENABLE.unwrap()(commandBuffer, rasterizerDiscardEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBiasEnable(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32) {
    VK_CMD_SET_DEPTH_BIAS_ENABLE.unwrap()(commandBuffer, depthBiasEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPrimitiveRestartEnable(
    commandBuffer: VkCommandBuffer,
    primitiveRestartEnable: VkBool32,
) {
    VK_CMD_SET_PRIMITIVE_RESTART_ENABLE.unwrap()(commandBuffer, primitiveRestartEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceBufferMemoryRequirements(
    device: VkDevice,
    pInfo: *const VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceImageMemoryRequirements(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceImageSparseMemoryRequirements(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
) {
    VK_GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS.unwrap()(device, pInfo, pSparseMemoryRequirementCount, pSparseMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineStipple(
    commandBuffer: VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    VK_CMD_SET_LINE_STIPPLE.unwrap()(commandBuffer, lineStippleFactor, lineStipplePattern)
}

#[no_mangle]
pub unsafe extern "C" fn vkMapMemory2(
    device: VkDevice,
    pMemoryMapInfo: *const VkMemoryMapInfo,
    ppData: *mut *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_MAP_MEMORY2.unwrap()(device, pMemoryMapInfo, ppData)
}

#[no_mangle]
pub unsafe extern "C" fn vkUnmapMemory2(device: VkDevice, pMemoryUnmapInfo: *const VkMemoryUnmapInfo)
    -> VkResult {
    VK_UNMAP_MEMORY2.unwrap()(device, pMemoryUnmapInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindIndexBuffer2(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    indexType: VkIndexType,
) {
    VK_CMD_BIND_INDEX_BUFFER2.unwrap()(commandBuffer, buffer, offset, size, indexType)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRenderingAreaGranularity(
    device: VkDevice,
    pRenderingAreaInfo: *const VkRenderingAreaInfo,
    pGranularity: *mut VkExtent2D,
) {
    VK_GET_RENDERING_AREA_GRANULARITY.unwrap()(device, pRenderingAreaInfo, pGranularity)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceImageSubresourceLayout(
    device: VkDevice,
    pInfo: *const VkDeviceImageSubresourceInfo,
    pLayout: *mut VkSubresourceLayout2,
) {
    VK_GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT.unwrap()(device, pInfo, pLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSubresourceLayout2(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource2,
    pLayout: *mut VkSubresourceLayout2,
) {
    VK_GET_IMAGE_SUBRESOURCE_LAYOUT2.unwrap()(device, image, pSubresource, pLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSet(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
) {
    VK_CMD_PUSH_DESCRIPTOR_SET.unwrap()(commandBuffer, pipelineBindPoint, layout, set, descriptorWriteCount, pDescriptorWrites)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSetWithTemplate(
    commandBuffer: VkCommandBuffer,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    layout: VkPipelineLayout,
    set: u32,
    pData: *const ::std::os::raw::c_void,
) {
    VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE.unwrap()(commandBuffer, descriptorUpdateTemplate, layout, set, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRenderingAttachmentLocations(
    commandBuffer: VkCommandBuffer,
    pLocationInfo: *const VkRenderingAttachmentLocationInfo,
) {
    VK_CMD_SET_RENDERING_ATTACHMENT_LOCATIONS.unwrap()(commandBuffer, pLocationInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRenderingInputAttachmentIndices(
    commandBuffer: VkCommandBuffer,
    pInputAttachmentIndexInfo: *const VkRenderingInputAttachmentIndexInfo,
) {
    VK_CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES.unwrap()(commandBuffer, pInputAttachmentIndexInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindDescriptorSets2(
    commandBuffer: VkCommandBuffer,
    pBindDescriptorSetsInfo: *const VkBindDescriptorSetsInfo,
) {
    VK_CMD_BIND_DESCRIPTOR_SETS2.unwrap()(commandBuffer, pBindDescriptorSetsInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushConstants2(
    commandBuffer: VkCommandBuffer,
    pPushConstantsInfo: *const VkPushConstantsInfo,
) {
    VK_CMD_PUSH_CONSTANTS2.unwrap()(commandBuffer, pPushConstantsInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSet2(
    commandBuffer: VkCommandBuffer,
    pPushDescriptorSetInfo: *const VkPushDescriptorSetInfo,
) {
    VK_CMD_PUSH_DESCRIPTOR_SET2.unwrap()(commandBuffer, pPushDescriptorSetInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSetWithTemplate2(
    commandBuffer: VkCommandBuffer,
    pPushDescriptorSetWithTemplateInfo: *const VkPushDescriptorSetWithTemplateInfo,
) {
    VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2.unwrap()(commandBuffer, pPushDescriptorSetWithTemplateInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMemoryToImage(
    device: VkDevice,
    pCopyMemoryToImageInfo: *const VkCopyMemoryToImageInfo,
) -> VkResult {
    VK_COPY_MEMORY_TO_IMAGE.unwrap()(device, pCopyMemoryToImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyImageToMemory(
    device: VkDevice,
    pCopyImageToMemoryInfo: *const VkCopyImageToMemoryInfo,
) -> VkResult {
    VK_COPY_IMAGE_TO_MEMORY.unwrap()(device, pCopyImageToMemoryInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyImageToImage(
    device: VkDevice,
    pCopyImageToImageInfo: *const VkCopyImageToImageInfo,
) -> VkResult {
    VK_COPY_IMAGE_TO_IMAGE.unwrap()(device, pCopyImageToImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkTransitionImageLayout(
    device: VkDevice,
    transitionCount: u32,
    pTransitions: *const VkHostImageLayoutTransitionInfo,
) -> VkResult {
    VK_TRANSITION_IMAGE_LAYOUT.unwrap()(device, transitionCount, pTransitions)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceGroupPresentCapabilitiesKHR(
    device: VkDevice,
    pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR,
) -> VkResult {
    VK_GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR.unwrap()(device, pDeviceGroupPresentCapabilities)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceGroupSurfacePresentModesKHR(
    device: VkDevice,
    surface: VkSurfaceKHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult {
    VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR.unwrap()(device, surface, pModes)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDevicePresentRectanglesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pRectCount: *mut u32,
    pRects: *mut VkRect2D,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_PRESENT_RECTANGLES_KHR.unwrap()(physicalDevice, surface, pRectCount, pRects)
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireNextImage2KHR(
    device: VkDevice,
    pAcquireInfo: *const VkAcquireNextImageInfoKHR,
    pImageIndex: *mut u32,
) -> VkResult {
    VK_ACQUIRE_NEXT_IMAGE2_KHR.unwrap()(device, pAcquireInfo, pImageIndex)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPropertiesKHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES_KHR.unwrap()(physicalDevice, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlanePropertiesKHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES_KHR.unwrap()(physicalDevice, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayPlaneSupportedDisplaysKHR(
    physicalDevice: VkPhysicalDevice,
    planeIndex: u32,
    pDisplayCount: *mut u32,
    pDisplays: *mut VkDisplayKHR,
) -> VkResult {
    VK_GET_DISPLAY_PLANE_SUPPORTED_DISPLAYS_KHR.unwrap()(physicalDevice, planeIndex, pDisplayCount, pDisplays)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayModePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModePropertiesKHR,
) -> VkResult {
    VK_GET_DISPLAY_MODE_PROPERTIES_KHR.unwrap()(physicalDevice, display, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDisplayModeKHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pCreateInfo: *const VkDisplayModeCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pMode: *mut VkDisplayModeKHR,
) -> VkResult {
    VK_CREATE_DISPLAY_MODE_KHR.unwrap()(physicalDevice, display, pCreateInfo, pAllocator, pMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayPlaneCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    mode: VkDisplayModeKHR,
    planeIndex: u32,
    pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR,
) -> VkResult {
    VK_GET_DISPLAY_PLANE_CAPABILITIES_KHR.unwrap()(physicalDevice, mode, planeIndex, pCapabilities)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDisplayPlaneSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    VK_CREATE_DISPLAY_PLANE_SURFACE_KHR.unwrap()(instance, pCreateInfo, pAllocator, pSurface)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSharedSwapchainsKHR(
    device: VkDevice,
    swapchainCount: u32,
    pCreateInfos: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchains: *mut VkSwapchainKHR,
) -> VkResult {
    VK_CREATE_SHARED_SWAPCHAINS_KHR.unwrap()(device, swapchainCount, pCreateInfos, pAllocator, pSwapchains)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceVideoCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    pVideoProfile: *const VkVideoProfileInfoKHR,
    pCapabilities: *mut VkVideoCapabilitiesKHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_VIDEO_CAPABILITIES_KHR.unwrap()(physicalDevice, pVideoProfile, pCapabilities)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceVideoFormatPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pVideoFormatInfo: *const VkPhysicalDeviceVideoFormatInfoKHR,
    pVideoFormatPropertyCount: *mut u32,
    pVideoFormatProperties: *mut VkVideoFormatPropertiesKHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_VIDEO_FORMAT_PROPERTIES_KHR.unwrap()(physicalDevice, pVideoFormatInfo, pVideoFormatPropertyCount, pVideoFormatProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateVideoSessionKHR(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSession: *mut VkVideoSessionKHR,
) -> VkResult {
    VK_CREATE_VIDEO_SESSION_KHR.unwrap()(device, pCreateInfo, pAllocator, pVideoSession)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyVideoSessionKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_VIDEO_SESSION_KHR.unwrap()(device, videoSession, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetVideoSessionMemoryRequirementsKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pMemoryRequirementsCount: *mut u32,
    pMemoryRequirements: *mut VkVideoSessionMemoryRequirementsKHR,
) -> VkResult {
    VK_GET_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR.unwrap()(device, videoSession, pMemoryRequirementsCount, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkBindVideoSessionMemoryKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    bindSessionMemoryInfoCount: u32,
    pBindSessionMemoryInfos: *const VkBindVideoSessionMemoryInfoKHR,
) -> VkResult {
    VK_BIND_VIDEO_SESSION_MEMORY_KHR.unwrap()(device, videoSession, bindSessionMemoryInfoCount, pBindSessionMemoryInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateVideoSessionParametersKHR(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionParametersCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSessionParameters: *mut VkVideoSessionParametersKHR,
) -> VkResult {
    VK_CREATE_VIDEO_SESSION_PARAMETERS_KHR.unwrap()(device, pCreateInfo, pAllocator, pVideoSessionParameters)
}

#[no_mangle]
pub unsafe extern "C" fn vkUpdateVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pUpdateInfo: *const VkVideoSessionParametersUpdateInfoKHR,
) -> VkResult {
    VK_UPDATE_VIDEO_SESSION_PARAMETERS_KHR.unwrap()(device, videoSessionParameters, pUpdateInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_VIDEO_SESSION_PARAMETERS_KHR.unwrap()(device, videoSessionParameters, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: *const VkVideoBeginCodingInfoKHR,
) {
    VK_CMD_BEGIN_VIDEO_CODING_KHR.unwrap()(commandBuffer, pBeginInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pEndCodingInfo: *const VkVideoEndCodingInfoKHR,
) {
    VK_CMD_END_VIDEO_CODING_KHR.unwrap()(commandBuffer, pEndCodingInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdControlVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pCodingControlInfo: *const VkVideoCodingControlInfoKHR,
) {
    VK_CMD_CONTROL_VIDEO_CODING_KHR.unwrap()(commandBuffer, pCodingControlInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDecodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pDecodeInfo: *const VkVideoDecodeInfoKHR,
) {
    VK_CMD_DECODE_VIDEO_KHR.unwrap()(commandBuffer, pDecodeInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginRenderingKHR(
    commandBuffer: VkCommandBuffer,
    pRenderingInfo: *const VkRenderingInfo,
) {
    VK_CMD_BEGIN_RENDERING_KHR.unwrap()(commandBuffer, pRenderingInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndRenderingKHR(commandBuffer: VkCommandBuffer) {
    VK_CMD_END_RENDERING_KHR.unwrap()(commandBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFeatures2KHR(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures2,
) {
    VK_GET_PHYSICAL_DEVICE_FEATURES2_KHR.unwrap()(physicalDevice, pFeatures)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_PROPERTIES2_KHR.unwrap()(physicalDevice, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFormatProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2_KHR.unwrap()(physicalDevice, format, pFormatProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceImageFormatProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
    pImageFormatProperties: *mut VkImageFormatProperties2,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2_KHR.unwrap()(physicalDevice, pImageFormatInfo, pImageFormatProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2_KHR.unwrap()(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceMemoryProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2_KHR.unwrap()(physicalDevice, pMemoryProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties2,
) {
    VK_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2_KHR.unwrap()(physicalDevice, pFormatInfo, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceGroupPeerMemoryFeaturesKHR(
    device: VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags,
) {
    VK_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_KHR.unwrap()(device, heapIndex, localDeviceIndex, remoteDeviceIndex, pPeerMemoryFeatures)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDeviceMaskKHR(commandBuffer: VkCommandBuffer, deviceMask: u32) {
    VK_CMD_SET_DEVICE_MASK_KHR.unwrap()(commandBuffer, deviceMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDispatchBaseKHR(
    commandBuffer: VkCommandBuffer,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    VK_CMD_DISPATCH_BASE_KHR.unwrap()(commandBuffer, baseGroupX, baseGroupY, baseGroupZ, groupCountX, groupCountY, groupCountZ)
}

#[no_mangle]
pub unsafe extern "C" fn vkTrimCommandPoolKHR(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolTrimFlags,
) {
    VK_TRIM_COMMAND_POOL_KHR.unwrap()(device, commandPool, flags)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumeratePhysicalDeviceGroupsKHR(
    instance: VkInstance,
    pPhysicalDeviceGroupCount: *mut u32,
    pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
) -> VkResult {
    VK_ENUMERATE_PHYSICAL_DEVICE_GROUPS_KHR.unwrap()(instance, pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
    pExternalBufferProperties: *mut VkExternalBufferProperties,
) {
    VK_GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES_KHR.unwrap()(physicalDevice, pExternalBufferInfo, pExternalBufferProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryFdKHR(
    device: VkDevice,
    pGetFdInfo: *const VkMemoryGetFdInfoKHR,
    pFd: *mut ::std::os::raw::c_int,
) -> VkResult {
    VK_GET_MEMORY_FD_KHR.unwrap()(device, pGetFdInfo, pFd)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryFdPropertiesKHR(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    fd: ::std::os::raw::c_int,
    pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR,
) -> VkResult {
    VK_GET_MEMORY_FD_PROPERTIES_KHR.unwrap()(device, handleType, fd, pMemoryFdProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
    pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
) {
    VK_GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR.unwrap()(physicalDevice, pExternalSemaphoreInfo, pExternalSemaphoreProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkImportSemaphoreFdKHR(
    device: VkDevice,
    pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR,
) -> VkResult {
    VK_IMPORT_SEMAPHORE_FD_KHR.unwrap()(device, pImportSemaphoreFdInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSemaphoreFdKHR(
    device: VkDevice,
    pGetFdInfo: *const VkSemaphoreGetFdInfoKHR,
    pFd: *mut ::std::os::raw::c_int,
) -> VkResult {
    VK_GET_SEMAPHORE_FD_KHR.unwrap()(device, pGetFdInfo, pFd)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSetKHR(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
) {
    VK_CMD_PUSH_DESCRIPTOR_SET_KHR.unwrap()(commandBuffer, pipelineBindPoint, layout, set, descriptorWriteCount, pDescriptorWrites)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSetWithTemplateKHR(
    commandBuffer: VkCommandBuffer,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    layout: VkPipelineLayout,
    set: u32,
    pData: *const ::std::os::raw::c_void,
) {
    VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_KHR.unwrap()(commandBuffer, descriptorUpdateTemplate, layout, set, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDescriptorUpdateTemplateKHR(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult {
    VK_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_KHR.unwrap()(device, pCreateInfo, pAllocator, pDescriptorUpdateTemplate)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDescriptorUpdateTemplateKHR(
    device: VkDevice,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_KHR.unwrap()(device, descriptorUpdateTemplate, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkUpdateDescriptorSetWithTemplateKHR(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pData: *const ::std::os::raw::c_void,
) {
    VK_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_KHR.unwrap()(device, descriptorSet, descriptorUpdateTemplate, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateRenderPass2KHR(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo2,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult {
    VK_CREATE_RENDER_PASS2_KHR.unwrap()(device, pCreateInfo, pAllocator, pRenderPass)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginRenderPass2KHR(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
) {
    VK_CMD_BEGIN_RENDER_PASS2_KHR.unwrap()(commandBuffer, pRenderPassBegin, pSubpassBeginInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdNextSubpass2KHR(
    commandBuffer: VkCommandBuffer,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
    pSubpassEndInfo: *const VkSubpassEndInfo,
) {
    VK_CMD_NEXT_SUBPASS2_KHR.unwrap()(commandBuffer, pSubpassBeginInfo, pSubpassEndInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndRenderPass2KHR(
    commandBuffer: VkCommandBuffer,
    pSubpassEndInfo: *const VkSubpassEndInfo,
) {
    VK_CMD_END_RENDER_PASS2_KHR.unwrap()(commandBuffer, pSubpassEndInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSwapchainStatusKHR(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult {
    VK_GET_SWAPCHAIN_STATUS_KHR.unwrap()(device, swapchain)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalFencePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
    pExternalFenceProperties: *mut VkExternalFenceProperties,
) {
    VK_GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES_KHR.unwrap()(physicalDevice, pExternalFenceInfo, pExternalFenceProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkImportFenceFdKHR(
    device: VkDevice,
    pImportFenceFdInfo: *const VkImportFenceFdInfoKHR,
) -> VkResult {
    VK_IMPORT_FENCE_FD_KHR.unwrap()(device, pImportFenceFdInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFenceFdKHR(
    device: VkDevice,
    pGetFdInfo: *const VkFenceGetFdInfoKHR,
    pFd: *mut ::std::os::raw::c_int,
) -> VkResult {
    VK_GET_FENCE_FD_KHR.unwrap()(device, pGetFdInfo, pFd)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: *mut u32,
    pCounters: *mut VkPerformanceCounterKHR,
    pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR,
) -> VkResult {
    VK_ENUMERATE_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_COUNTERS_KHR.unwrap()(physicalDevice, queueFamilyIndex, pCounterCount, pCounters, pCounterDescriptions)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
    physicalDevice: VkPhysicalDevice,
    pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR,
    pNumPasses: *mut u32,
) {
    VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_PASSES_KHR.unwrap()(physicalDevice, pPerformanceQueryCreateInfo, pNumPasses)
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireProfilingLockKHR(
    device: VkDevice,
    pInfo: *const VkAcquireProfilingLockInfoKHR,
) -> VkResult {
    VK_ACQUIRE_PROFILING_LOCK_KHR.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkReleaseProfilingLockKHR(device: VkDevice) {
    VK_RELEASE_PROFILING_LOCK_KHR.unwrap()(device)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayProperties2KHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES2_KHR.unwrap()(physicalDevice, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlaneProperties2KHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES2_KHR.unwrap()(physicalDevice, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayModeProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModeProperties2KHR,
) -> VkResult {
    VK_GET_DISPLAY_MODE_PROPERTIES2_KHR.unwrap()(physicalDevice, display, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayPlaneCapabilities2KHR(
    physicalDevice: VkPhysicalDevice,
    pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR,
    pCapabilities: *mut VkDisplayPlaneCapabilities2KHR,
) -> VkResult {
    VK_GET_DISPLAY_PLANE_CAPABILITIES2_KHR.unwrap()(physicalDevice, pDisplayPlaneInfo, pCapabilities)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageMemoryRequirements2KHR(
    device: VkDevice,
    pInfo: *const VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_IMAGE_MEMORY_REQUIREMENTS2_KHR.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferMemoryRequirements2KHR(
    device: VkDevice,
    pInfo: *const VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_BUFFER_MEMORY_REQUIREMENTS2_KHR.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSparseMemoryRequirements2KHR(
    device: VkDevice,
    pInfo: *const VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
) {
    VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2_KHR.unwrap()(device, pInfo, pSparseMemoryRequirementCount, pSparseMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSamplerYcbcrConversionKHR(
    device: VkDevice,
    pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult {
    VK_CREATE_SAMPLER_YCBCR_CONVERSION_KHR.unwrap()(device, pCreateInfo, pAllocator, pYcbcrConversion)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySamplerYcbcrConversionKHR(
    device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_SAMPLER_YCBCR_CONVERSION_KHR.unwrap()(device, ycbcrConversion, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkBindBufferMemory2KHR(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindBufferMemoryInfo,
) -> VkResult {
    VK_BIND_BUFFER_MEMORY2_KHR.unwrap()(device, bindInfoCount, pBindInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkBindImageMemory2KHR(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindImageMemoryInfo,
) -> VkResult {
    VK_BIND_IMAGE_MEMORY2_KHR.unwrap()(device, bindInfoCount, pBindInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetLayoutSupportKHR(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pSupport: *mut VkDescriptorSetLayoutSupport,
) {
    VK_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR.unwrap()(device, pCreateInfo, pSupport)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndirectCountKHR(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_INDIRECT_COUNT_KHR.unwrap()(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndexedIndirectCountKHR(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_KHR.unwrap()(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSemaphoreCounterValueKHR(
    device: VkDevice,
    semaphore: VkSemaphore,
    pValue: *mut u64,
) -> VkResult {
    VK_GET_SEMAPHORE_COUNTER_VALUE_KHR.unwrap()(device, semaphore, pValue)
}

#[no_mangle]
pub unsafe extern "C" fn vkWaitSemaphoresKHR(
    device: VkDevice,
    pWaitInfo: *const VkSemaphoreWaitInfo,
    timeout: u64,
) -> VkResult {
    VK_WAIT_SEMAPHORES_KHR.unwrap()(device, pWaitInfo, timeout)
}

#[no_mangle]
pub unsafe extern "C" fn vkSignalSemaphoreKHR(
    device: VkDevice,
    pSignalInfo: *const VkSemaphoreSignalInfo,
) -> VkResult {
    VK_SIGNAL_SEMAPHORE_KHR.unwrap()(device, pSignalInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFragmentShadingRatesKHR(
    physicalDevice: VkPhysicalDevice,
    pFragmentShadingRateCount: *mut u32,
    pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATES_KHR.unwrap()(physicalDevice, pFragmentShadingRateCount, pFragmentShadingRates)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetFragmentShadingRateKHR(
    commandBuffer: VkCommandBuffer,
    pFragmentSize: *const VkExtent2D,
    combinerOps: *const VkFragmentShadingRateCombinerOpKHR,
) {
    VK_CMD_SET_FRAGMENT_SHADING_RATE_KHR.unwrap()(commandBuffer, pFragmentSize, combinerOps)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRenderingAttachmentLocationsKHR(
    commandBuffer: VkCommandBuffer,
    pLocationInfo: *const VkRenderingAttachmentLocationInfo,
) {
    VK_CMD_SET_RENDERING_ATTACHMENT_LOCATIONS_KHR.unwrap()(commandBuffer, pLocationInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRenderingInputAttachmentIndicesKHR(
    commandBuffer: VkCommandBuffer,
    pInputAttachmentIndexInfo: *const VkRenderingInputAttachmentIndexInfo,
) {
    VK_CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES_KHR.unwrap()(commandBuffer, pInputAttachmentIndexInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkWaitForPresentKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    presentId: u64,
    timeout: u64,
) -> VkResult {
    VK_WAIT_FOR_PRESENT_KHR.unwrap()(device, swapchain, presentId, timeout)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferDeviceAddressKHR(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress {
    VK_GET_BUFFER_DEVICE_ADDRESS_KHR.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferOpaqueCaptureAddressKHR(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> u64 {
    VK_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_KHR.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceMemoryOpaqueCaptureAddressKHR(
    device: VkDevice,
    pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo,
) -> u64 {
    VK_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_KHR.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDeferredOperationKHR(
    device: VkDevice,
    pAllocator: *const VkAllocationCallbacks,
    pDeferredOperation: *mut VkDeferredOperationKHR,
) -> VkResult {
    VK_CREATE_DEFERRED_OPERATION_KHR.unwrap()(device, pAllocator, pDeferredOperation)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDeferredOperationKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_DEFERRED_OPERATION_KHR.unwrap()(device, operation, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeferredOperationMaxConcurrencyKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> u32 {
    VK_GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR.unwrap()(device, operation)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeferredOperationResultKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    VK_GET_DEFERRED_OPERATION_RESULT_KHR.unwrap()(device, operation)
}

#[no_mangle]
pub unsafe extern "C" fn vkDeferredOperationJoinKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    VK_DEFERRED_OPERATION_JOIN_KHR.unwrap()(device, operation)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineExecutablePropertiesKHR(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoKHR,
    pExecutableCount: *mut u32,
    pProperties: *mut VkPipelineExecutablePropertiesKHR,
) -> VkResult {
    VK_GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR.unwrap()(device, pPipelineInfo, pExecutableCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineExecutableStatisticsKHR(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR,
    pStatisticCount: *mut u32,
    pStatistics: *mut VkPipelineExecutableStatisticKHR,
) -> VkResult {
    VK_GET_PIPELINE_EXECUTABLE_STATISTICS_KHR.unwrap()(device, pExecutableInfo, pStatisticCount, pStatistics)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineExecutableInternalRepresentationsKHR(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR,
    pInternalRepresentationCount: *mut u32,
    pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR,
) -> VkResult {
    VK_GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR.unwrap()(device, pExecutableInfo, pInternalRepresentationCount, pInternalRepresentations)
}

#[no_mangle]
pub unsafe extern "C" fn vkMapMemory2KHR(
    device: VkDevice,
    pMemoryMapInfo: *const VkMemoryMapInfo,
    ppData: *mut *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_MAP_MEMORY2_KHR.unwrap()(device, pMemoryMapInfo, ppData)
}

#[no_mangle]
pub unsafe extern "C" fn vkUnmapMemory2KHR(
    device: VkDevice,
    pMemoryUnmapInfo: *const VkMemoryUnmapInfo,
) -> VkResult {
    VK_UNMAP_MEMORY2_KHR.unwrap()(device, pMemoryUnmapInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pQualityLevelInfo: *const VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR,
    pQualityLevelProperties: *mut VkVideoEncodeQualityLevelPropertiesKHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR.unwrap()(physicalDevice, pQualityLevelInfo, pQualityLevelProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetEncodedVideoSessionParametersKHR(
    device: VkDevice,
    pVideoSessionParametersInfo: *const VkVideoEncodeSessionParametersGetInfoKHR,
    pFeedbackInfo: *mut VkVideoEncodeSessionParametersFeedbackInfoKHR,
    pDataSize: *mut usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_ENCODED_VIDEO_SESSION_PARAMETERS_KHR.unwrap()(device, pVideoSessionParametersInfo, pFeedbackInfo, pDataSize, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEncodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pEncodeInfo: *const VkVideoEncodeInfoKHR,
) {
    VK_CMD_ENCODE_VIDEO_KHR.unwrap()(commandBuffer, pEncodeInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetEvent2KHR(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    pDependencyInfo: *const VkDependencyInfo,
) {
    VK_CMD_SET_EVENT2_KHR.unwrap()(commandBuffer, event, pDependencyInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResetEvent2KHR(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags2,
) {
    VK_CMD_RESET_EVENT2_KHR.unwrap()(commandBuffer, event, stageMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWaitEvents2KHR(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    pDependencyInfos: *const VkDependencyInfo,
) {
    VK_CMD_WAIT_EVENTS2_KHR.unwrap()(commandBuffer, eventCount, pEvents, pDependencyInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPipelineBarrier2KHR(
    commandBuffer: VkCommandBuffer,
    pDependencyInfo: *const VkDependencyInfo,
) {
    VK_CMD_PIPELINE_BARRIER2_KHR.unwrap()(commandBuffer, pDependencyInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteTimestamp2KHR(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
) {
    VK_CMD_WRITE_TIMESTAMP2_KHR.unwrap()(commandBuffer, stage, queryPool, query)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueSubmit2KHR(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo2,
    fence: VkFence,
) -> VkResult {
    VK_QUEUE_SUBMIT2_KHR.unwrap()(queue, submitCount, pSubmits, fence)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBuffer2KHR(
    commandBuffer: VkCommandBuffer,
    pCopyBufferInfo: *const VkCopyBufferInfo2,
) {
    VK_CMD_COPY_BUFFER2_KHR.unwrap()(commandBuffer, pCopyBufferInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImage2KHR(
    commandBuffer: VkCommandBuffer,
    pCopyImageInfo: *const VkCopyImageInfo2,
) {
    VK_CMD_COPY_IMAGE2_KHR.unwrap()(commandBuffer, pCopyImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBufferToImage2KHR(
    commandBuffer: VkCommandBuffer,
    pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2,
) {
    VK_CMD_COPY_BUFFER_TO_IMAGE2_KHR.unwrap()(commandBuffer, pCopyBufferToImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImageToBuffer2KHR(
    commandBuffer: VkCommandBuffer,
    pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2,
) {
    VK_CMD_COPY_IMAGE_TO_BUFFER2_KHR.unwrap()(commandBuffer, pCopyImageToBufferInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBlitImage2KHR(
    commandBuffer: VkCommandBuffer,
    pBlitImageInfo: *const VkBlitImageInfo2,
) {
    VK_CMD_BLIT_IMAGE2_KHR.unwrap()(commandBuffer, pBlitImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResolveImage2KHR(
    commandBuffer: VkCommandBuffer,
    pResolveImageInfo: *const VkResolveImageInfo2,
) {
    VK_CMD_RESOLVE_IMAGE2_KHR.unwrap()(commandBuffer, pResolveImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdTraceRaysIndirect2KHR(
    commandBuffer: VkCommandBuffer,
    indirectDeviceAddress: VkDeviceAddress,
) {
    VK_CMD_TRACE_RAYS_INDIRECT2_KHR.unwrap()(commandBuffer, indirectDeviceAddress)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceBufferMemoryRequirementsKHR(
    device: VkDevice,
    pInfo: *const VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceImageMemoryRequirementsKHR(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceImageSparseMemoryRequirementsKHR(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
) {
    VK_GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_KHR.unwrap()(device, pInfo, pSparseMemoryRequirementCount, pSparseMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindIndexBuffer2KHR(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    indexType: VkIndexType,
) {
    VK_CMD_BIND_INDEX_BUFFER2_KHR.unwrap()(commandBuffer, buffer, offset, size, indexType)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRenderingAreaGranularityKHR(
    device: VkDevice,
    pRenderingAreaInfo: *const VkRenderingAreaInfo,
    pGranularity: *mut VkExtent2D,
) {
    VK_GET_RENDERING_AREA_GRANULARITY_KHR.unwrap()(device, pRenderingAreaInfo, pGranularity)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceImageSubresourceLayoutKHR(
    device: VkDevice,
    pInfo: *const VkDeviceImageSubresourceInfo,
    pLayout: *mut VkSubresourceLayout2,
) {
    VK_GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT_KHR.unwrap()(device, pInfo, pLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSubresourceLayout2KHR(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource2,
    pLayout: *mut VkSubresourceLayout2,
) {
    VK_GET_IMAGE_SUBRESOURCE_LAYOUT2_KHR.unwrap()(device, image, pSubresource, pLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreatePipelineBinariesKHR(
    device: VkDevice,
    pCreateInfo: *const VkPipelineBinaryCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pBinaries: *mut VkPipelineBinaryHandlesInfoKHR,
) -> VkResult {
    VK_CREATE_PIPELINE_BINARIES_KHR.unwrap()(device, pCreateInfo, pAllocator, pBinaries)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPipelineBinaryKHR(
    device: VkDevice,
    pipelineBinary: VkPipelineBinaryKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_PIPELINE_BINARY_KHR.unwrap()(device, pipelineBinary, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineKeyKHR(
    device: VkDevice,
    pPipelineCreateInfo: *const VkPipelineCreateInfoKHR,
    pPipelineKey: *mut VkPipelineBinaryKeyKHR,
) -> VkResult {
    VK_GET_PIPELINE_KEY_KHR.unwrap()(device, pPipelineCreateInfo, pPipelineKey)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineBinaryDataKHR(
    device: VkDevice,
    pInfo: *const VkPipelineBinaryDataInfoKHR,
    pPipelineBinaryKey: *mut VkPipelineBinaryKeyKHR,
    pPipelineBinaryDataSize: *mut usize,
    pPipelineBinaryData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_PIPELINE_BINARY_DATA_KHR.unwrap()(device, pInfo, pPipelineBinaryKey, pPipelineBinaryDataSize, pPipelineBinaryData)
}

#[no_mangle]
pub unsafe extern "C" fn vkReleaseCapturedPipelineDataKHR(
    device: VkDevice,
    pInfo: *const VkReleaseCapturedPipelineDataInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
) -> VkResult {
    VK_RELEASE_CAPTURED_PIPELINE_DATA_KHR.unwrap()(device, pInfo, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesKHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR.unwrap()(physicalDevice, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineStippleKHR(
    commandBuffer: VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    VK_CMD_SET_LINE_STIPPLE_KHR.unwrap()(commandBuffer, lineStippleFactor, lineStipplePattern)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceCalibrateableTimeDomainsKHR(
    physicalDevice: VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut VkTimeDomainKHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_CALIBRATEABLE_TIME_DOMAINS_KHR.unwrap()(physicalDevice, pTimeDomainCount, pTimeDomains)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetCalibratedTimestampsKHR(
    device: VkDevice,
    timestampCount: u32,
    pTimestampInfos: *const VkCalibratedTimestampInfoKHR,
    pTimestamps: *mut u64,
    pMaxDeviation: *mut u64,
) -> VkResult {
    VK_GET_CALIBRATED_TIMESTAMPS_KHR.unwrap()(device, timestampCount, pTimestampInfos, pTimestamps, pMaxDeviation)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindDescriptorSets2KHR(
    commandBuffer: VkCommandBuffer,
    pBindDescriptorSetsInfo: *const VkBindDescriptorSetsInfo,
) {
    VK_CMD_BIND_DESCRIPTOR_SETS2_KHR.unwrap()(commandBuffer, pBindDescriptorSetsInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushConstants2KHR(
    commandBuffer: VkCommandBuffer,
    pPushConstantsInfo: *const VkPushConstantsInfo,
) {
    VK_CMD_PUSH_CONSTANTS2_KHR.unwrap()(commandBuffer, pPushConstantsInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSet2KHR(
    commandBuffer: VkCommandBuffer,
    pPushDescriptorSetInfo: *const VkPushDescriptorSetInfo,
) {
    VK_CMD_PUSH_DESCRIPTOR_SET2_KHR.unwrap()(commandBuffer, pPushDescriptorSetInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSetWithTemplate2KHR(
    commandBuffer: VkCommandBuffer,
    pPushDescriptorSetWithTemplateInfo: *const VkPushDescriptorSetWithTemplateInfo,
) {
    VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2_KHR.unwrap()(commandBuffer, pPushDescriptorSetWithTemplateInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDescriptorBufferOffsets2EXT(
    commandBuffer: VkCommandBuffer,
    pSetDescriptorBufferOffsetsInfo: *const VkSetDescriptorBufferOffsetsInfoEXT,
) {
    VK_CMD_SET_DESCRIPTOR_BUFFER_OFFSETS2_EXT.unwrap()(commandBuffer, pSetDescriptorBufferOffsetsInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindDescriptorBufferEmbeddedSamplers2EXT(
    commandBuffer: VkCommandBuffer,
    pBindDescriptorBufferEmbeddedSamplersInfo : * const VkBindDescriptorBufferEmbeddedSamplersInfoEXT,
) {
    VK_CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS2_EXT.unwrap()(commandBuffer, pBindDescriptorBufferEmbeddedSamplersInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDebugReportCallbackEXT(
    instance: VkInstance,
    pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pCallback: *mut VkDebugReportCallbackEXT,
) -> VkResult {
    VK_CREATE_DEBUG_REPORT_CALLBACK_EXT.unwrap()(instance, pCreateInfo, pAllocator, pCallback)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDebugReportCallbackEXT(
    instance: VkInstance,
    callback: VkDebugReportCallbackEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_DEBUG_REPORT_CALLBACK_EXT.unwrap()(instance, callback, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkDebugReportMessageEXT(
    instance: VkInstance,
    flags: VkDebugReportFlagsEXT,
    objectType: VkDebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    messageCode: i32,
    pLayerPrefix: *const ::std::os::raw::c_char,
    pMessage: *const ::std::os::raw::c_char,
) {
    VK_DEBUG_REPORT_MESSAGE_EXT.unwrap()(instance, flags, objectType, object, location, messageCode, pLayerPrefix, pMessage)
}

#[no_mangle]
pub unsafe extern "C" fn vkDebugMarkerSetObjectTagEXT(
    device: VkDevice,
    pTagInfo: *const VkDebugMarkerObjectTagInfoEXT,
) -> VkResult {
    VK_DEBUG_MARKER_SET_OBJECT_TAG_EXT.unwrap()(device, pTagInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkDebugMarkerSetObjectNameEXT(
    device: VkDevice,
    pNameInfo: *const VkDebugMarkerObjectNameInfoEXT,
) -> VkResult {
    VK_DEBUG_MARKER_SET_OBJECT_NAME_EXT.unwrap()(device, pNameInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDebugMarkerBeginEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
) {
    VK_CMD_DEBUG_MARKER_BEGIN_EXT.unwrap()(commandBuffer, pMarkerInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer) {
    VK_CMD_DEBUG_MARKER_END_EXT.unwrap()(commandBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDebugMarkerInsertEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
) {
    VK_CMD_DEBUG_MARKER_INSERT_EXT.unwrap()(commandBuffer, pMarkerInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindTransformFeedbackBuffersEXT(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
) {
    VK_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS_EXT.unwrap()(commandBuffer, firstBinding, bindingCount, pBuffers, pOffsets, pSizes)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginTransformFeedbackEXT(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
) {
    VK_CMD_BEGIN_TRANSFORM_FEEDBACK_EXT.unwrap()(commandBuffer, firstCounterBuffer, counterBufferCount, pCounterBuffers, pCounterBufferOffsets)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndTransformFeedbackEXT(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
) {
    VK_CMD_END_TRANSFORM_FEEDBACK_EXT.unwrap()(commandBuffer, firstCounterBuffer, counterBufferCount, pCounterBuffers, pCounterBufferOffsets)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginQueryIndexedEXT(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
    index: u32,
) {
    VK_CMD_BEGIN_QUERY_INDEXED_EXT.unwrap()(commandBuffer, queryPool, query, flags, index)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndQueryIndexedEXT(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    index: u32,
) {
    VK_CMD_END_QUERY_INDEXED_EXT.unwrap()(commandBuffer, queryPool, query, index)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndirectByteCountEXT(
    commandBuffer: VkCommandBuffer,
    instanceCount: u32,
    firstInstance: u32,
    counterBuffer: VkBuffer,
    counterBufferOffset: VkDeviceSize,
    counterOffset: u32,
    vertexStride: u32,
) {
    VK_CMD_DRAW_INDIRECT_BYTE_COUNT_EXT.unwrap()(commandBuffer, instanceCount, firstInstance, counterBuffer, counterBufferOffset, counterOffset, vertexStride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateCuModuleNVX(
    device: VkDevice,
    pCreateInfo: *const VkCuModuleCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pModule: *mut VkCuModuleNVX,
) -> VkResult {
    VK_CREATE_CU_MODULE_NVX.unwrap()(device, pCreateInfo, pAllocator, pModule)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateCuFunctionNVX(
    device: VkDevice,
    pCreateInfo: *const VkCuFunctionCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pFunction: *mut VkCuFunctionNVX,
) -> VkResult {
    VK_CREATE_CU_FUNCTION_NVX.unwrap()(device, pCreateInfo, pAllocator, pFunction)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyCuModuleNVX(
    device: VkDevice,
    module: VkCuModuleNVX,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_CU_MODULE_NVX.unwrap()(device, module, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyCuFunctionNVX(
    device: VkDevice,
    function: VkCuFunctionNVX,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_CU_FUNCTION_NVX.unwrap()(device, function, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCuLaunchKernelNVX(
    commandBuffer: VkCommandBuffer,
    pLaunchInfo: *const VkCuLaunchInfoNVX,
) {
    VK_CMD_CU_LAUNCH_KERNEL_NVX.unwrap()(commandBuffer, pLaunchInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageViewHandleNVX(device: VkDevice, pInfo: *const VkImageViewHandleInfoNVX)
    -> u32 {
    VK_GET_IMAGE_VIEW_HANDLE_NVX.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageViewHandle64NVX(
    device: VkDevice,
    pInfo: *const VkImageViewHandleInfoNVX,
) -> u64 {
    VK_GET_IMAGE_VIEW_HANDLE64_NVX.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageViewAddressNVX(
    device: VkDevice,
    imageView: VkImageView,
    pProperties: *mut VkImageViewAddressPropertiesNVX,
) -> VkResult {
    VK_GET_IMAGE_VIEW_ADDRESS_NVX.unwrap()(device, imageView, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndirectCountAMD(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_INDIRECT_COUNT_AMD.unwrap()(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndexedIndirectCountAMD(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD.unwrap()(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetShaderInfoAMD(
    device: VkDevice,
    pipeline: VkPipeline,
    shaderStage: VkShaderStageFlagBits,
    infoType: VkShaderInfoTypeAMD,
    pInfoSize: *mut usize,
    pInfo: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_SHADER_INFO_AMD.unwrap()(device, pipeline, shaderStage, infoType, pInfoSize, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
    pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_NV.unwrap()(physicalDevice, format, type_, tiling, usage, flags, externalHandleType, pExternalImageFormatProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginConditionalRenderingEXT(
    commandBuffer: VkCommandBuffer,
    pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT,
) {
    VK_CMD_BEGIN_CONDITIONAL_RENDERING_EXT.unwrap()(commandBuffer, pConditionalRenderingBegin)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndConditionalRenderingEXT(commandBuffer: VkCommandBuffer) {
    VK_CMD_END_CONDITIONAL_RENDERING_EXT.unwrap()(commandBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportWScalingNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportWScalings: *const VkViewportWScalingNV,
) {
    VK_CMD_SET_VIEWPORT_WSCALING_NV.unwrap()(commandBuffer, firstViewport, viewportCount, pViewportWScalings)
}

#[no_mangle]
pub unsafe extern "C" fn vkReleaseDisplayEXT(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR)
    -> VkResult {
    VK_RELEASE_DISPLAY_EXT.unwrap()(physicalDevice, display)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_EXT.unwrap()(physicalDevice, surface, pSurfaceCapabilities)
}

#[no_mangle]
pub unsafe extern "C" fn vkDisplayPowerControlEXT(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayPowerInfo: *const VkDisplayPowerInfoEXT,
) -> VkResult {
    VK_DISPLAY_POWER_CONTROL_EXT.unwrap()(device, display, pDisplayPowerInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkRegisterDeviceEventEXT(
    device: VkDevice,
    pDeviceEventInfo: *const VkDeviceEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult {
    VK_REGISTER_DEVICE_EVENT_EXT.unwrap()(device, pDeviceEventInfo, pAllocator, pFence)
}

#[no_mangle]
pub unsafe extern "C" fn vkRegisterDisplayEventEXT(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayEventInfo: *const VkDisplayEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult {
    VK_REGISTER_DISPLAY_EVENT_EXT.unwrap()(device, display, pDisplayEventInfo, pAllocator, pFence)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSwapchainCounterEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    counter: VkSurfaceCounterFlagBitsEXT,
    pCounterValue: *mut u64,
) -> VkResult {
    VK_GET_SWAPCHAIN_COUNTER_EXT.unwrap()(device, swapchain, counter, pCounterValue)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRefreshCycleDurationGOOGLE(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE,
) -> VkResult {
    VK_GET_REFRESH_CYCLE_DURATION_GOOGLE.unwrap()(device, swapchain, pDisplayTimingProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPastPresentationTimingGOOGLE(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pPresentationTimingCount: *mut u32,
    pPresentationTimings: *mut VkPastPresentationTimingGOOGLE,
) -> VkResult {
    VK_GET_PAST_PRESENTATION_TIMING_GOOGLE.unwrap()(device, swapchain, pPresentationTimingCount, pPresentationTimings)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDiscardRectangleEXT(
    commandBuffer: VkCommandBuffer,
    firstDiscardRectangle: u32,
    discardRectangleCount: u32,
    pDiscardRectangles: *const VkRect2D,
) {
    VK_CMD_SET_DISCARD_RECTANGLE_EXT.unwrap()(commandBuffer, firstDiscardRectangle, discardRectangleCount, pDiscardRectangles)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDiscardRectangleEnableEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleEnable: VkBool32,
) {
    VK_CMD_SET_DISCARD_RECTANGLE_ENABLE_EXT.unwrap()(commandBuffer, discardRectangleEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDiscardRectangleModeEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleMode: VkDiscardRectangleModeEXT,
) {
    VK_CMD_SET_DISCARD_RECTANGLE_MODE_EXT.unwrap()(commandBuffer, discardRectangleMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetHdrMetadataEXT(
    device: VkDevice,
    swapchainCount: u32,
    pSwapchains: *const VkSwapchainKHR,
    pMetadata: *const VkHdrMetadataEXT,
) {
    VK_SET_HDR_METADATA_EXT.unwrap()(device, swapchainCount, pSwapchains, pMetadata)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetDebugUtilsObjectNameEXT(
    device: VkDevice,
    pNameInfo: *const VkDebugUtilsObjectNameInfoEXT,
) -> VkResult {
    VK_SET_DEBUG_UTILS_OBJECT_NAME_EXT.unwrap()(device, pNameInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetDebugUtilsObjectTagEXT(
    device: VkDevice,
    pTagInfo: *const VkDebugUtilsObjectTagInfoEXT,
) -> VkResult {
    VK_SET_DEBUG_UTILS_OBJECT_TAG_EXT.unwrap()(device, pTagInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueBeginDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT) {
    VK_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT.unwrap()(queue, pLabelInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueEndDebugUtilsLabelEXT(queue: VkQueue) {
    VK_QUEUE_END_DEBUG_UTILS_LABEL_EXT.unwrap()(queue)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueInsertDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT) {
    VK_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT.unwrap()(queue, pLabelInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    VK_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT.unwrap()(commandBuffer, pLabelInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer) {
    VK_CMD_END_DEBUG_UTILS_LABEL_EXT.unwrap()(commandBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdInsertDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    VK_CMD_INSERT_DEBUG_UTILS_LABEL_EXT.unwrap()(commandBuffer, pLabelInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDebugUtilsMessengerEXT(
    instance: VkInstance,
    pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMessenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult {
    VK_CREATE_DEBUG_UTILS_MESSENGER_EXT.unwrap()(instance, pCreateInfo, pAllocator, pMessenger)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDebugUtilsMessengerEXT(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_DEBUG_UTILS_MESSENGER_EXT.unwrap()(instance, messenger, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkSubmitDebugUtilsMessageEXT(
    instance: VkInstance,
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
) {
    VK_SUBMIT_DEBUG_UTILS_MESSAGE_EXT.unwrap()(instance, messageSeverity, messageTypes, pCallbackData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetSampleLocationsEXT(
    commandBuffer: VkCommandBuffer,
    pSampleLocationsInfo: *const VkSampleLocationsInfoEXT,
) {
    VK_CMD_SET_SAMPLE_LOCATIONS_EXT.unwrap()(commandBuffer, pSampleLocationsInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceMultisamplePropertiesEXT(
    physicalDevice: VkPhysicalDevice,
    samples: VkSampleCountFlagBits,
    pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
) {
    VK_GET_PHYSICAL_DEVICE_MULTISAMPLE_PROPERTIES_EXT.unwrap()(physicalDevice, samples, pMultisampleProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageDrmFormatModifierPropertiesEXT(
    device: VkDevice,
    image: VkImage,
    pProperties: *mut VkImageDrmFormatModifierPropertiesEXT,
) -> VkResult {
    VK_GET_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT.unwrap()(device, image, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateValidationCacheEXT(
    device: VkDevice,
    pCreateInfo: *const VkValidationCacheCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pValidationCache: *mut VkValidationCacheEXT,
) -> VkResult {
    VK_CREATE_VALIDATION_CACHE_EXT.unwrap()(device, pCreateInfo, pAllocator, pValidationCache)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyValidationCacheEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_VALIDATION_CACHE_EXT.unwrap()(device, validationCache, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkMergeValidationCachesEXT(
    device: VkDevice,
    dstCache: VkValidationCacheEXT,
    srcCacheCount: u32,
    pSrcCaches: *const VkValidationCacheEXT,
) -> VkResult {
    VK_MERGE_VALIDATION_CACHES_EXT.unwrap()(device, dstCache, srcCacheCount, pSrcCaches)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetValidationCacheDataEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pDataSize: *mut usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_VALIDATION_CACHE_DATA_EXT.unwrap()(device, validationCache, pDataSize, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindShadingRateImageNV(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    VK_CMD_BIND_SHADING_RATE_IMAGE_NV.unwrap()(commandBuffer, imageView, imageLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportShadingRatePaletteNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pShadingRatePalettes: *const VkShadingRatePaletteNV,
) {
    VK_CMD_SET_VIEWPORT_SHADING_RATE_PALETTE_NV.unwrap()(commandBuffer, firstViewport, viewportCount, pShadingRatePalettes)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoarseSampleOrderNV(
    commandBuffer: VkCommandBuffer,
    sampleOrderType: VkCoarseSampleOrderTypeNV,
    customSampleOrderCount: u32,
    pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
) {
    VK_CMD_SET_COARSE_SAMPLE_ORDER_NV.unwrap()(commandBuffer, sampleOrderType, customSampleOrderCount, pCustomSampleOrders)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateAccelerationStructureNV(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureNV,
) -> VkResult {
    VK_CREATE_ACCELERATION_STRUCTURE_NV.unwrap()(device, pCreateInfo, pAllocator, pAccelerationStructure)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyAccelerationStructureNV(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_ACCELERATION_STRUCTURE_NV.unwrap()(device, accelerationStructure, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureMemoryRequirementsNV(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2KHR,
) {
    VK_GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_NV.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkBindAccelerationStructureMemoryNV(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV,
) -> VkResult {
    VK_BIND_ACCELERATION_STRUCTURE_MEMORY_NV.unwrap()(device, bindInfoCount, pBindInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBuildAccelerationStructureNV(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkAccelerationStructureInfoNV,
    instanceData: VkBuffer,
    instanceOffset: VkDeviceSize,
    update: VkBool32,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    scratch: VkBuffer,
    scratchOffset: VkDeviceSize,
) {
    VK_CMD_BUILD_ACCELERATION_STRUCTURE_NV.unwrap()(commandBuffer, pInfo, instanceData, instanceOffset, update, dst, src, scratch, scratchOffset)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyAccelerationStructureNV(
    commandBuffer: VkCommandBuffer,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    mode: VkCopyAccelerationStructureModeKHR,
) {
    VK_CMD_COPY_ACCELERATION_STRUCTURE_NV.unwrap()(commandBuffer, dst, src, mode)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdTraceRaysNV(
    commandBuffer: VkCommandBuffer,
    raygenShaderBindingTableBuffer: VkBuffer,
    raygenShaderBindingOffset: VkDeviceSize,
    missShaderBindingTableBuffer: VkBuffer,
    missShaderBindingOffset: VkDeviceSize,
    missShaderBindingStride: VkDeviceSize,
    hitShaderBindingTableBuffer: VkBuffer,
    hitShaderBindingOffset: VkDeviceSize,
    hitShaderBindingStride: VkDeviceSize,
    callableShaderBindingTableBuffer: VkBuffer,
    callableShaderBindingOffset: VkDeviceSize,
    callableShaderBindingStride: VkDeviceSize,
    width: u32,
    height: u32,
    depth: u32,
) {
    VK_CMD_TRACE_RAYS_NV.unwrap()(commandBuffer, raygenShaderBindingTableBuffer, raygenShaderBindingOffset, missShaderBindingTableBuffer, missShaderBindingOffset, missShaderBindingStride, hitShaderBindingTableBuffer, hitShaderBindingOffset, hitShaderBindingStride, callableShaderBindingTableBuffer, callableShaderBindingOffset, callableShaderBindingStride, width, height, depth)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateRayTracingPipelinesNV(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    VK_CREATE_RAY_TRACING_PIPELINES_NV.unwrap()(device, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRayTracingShaderGroupHandlesKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_RAY_TRACING_SHADER_GROUP_HANDLES_KHR.unwrap()(device, pipeline, firstGroup, groupCount, dataSize, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRayTracingShaderGroupHandlesNV(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_RAY_TRACING_SHADER_GROUP_HANDLES_NV.unwrap()(device, pipeline, firstGroup, groupCount, dataSize, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureHandleNV(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_ACCELERATION_STRUCTURE_HANDLE_NV.unwrap()(device, accelerationStructure, dataSize, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteAccelerationStructuresPropertiesNV(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureNV,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    VK_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_NV.unwrap()(commandBuffer, accelerationStructureCount, pAccelerationStructures, queryType, queryPool, firstQuery)
}

#[no_mangle]
pub unsafe extern "C" fn vkCompileDeferredNV(device: VkDevice, pipeline: VkPipeline, shader: u32) -> VkResult {
    VK_COMPILE_DEFERRED_NV.unwrap()(device, pipeline, shader)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryHostPointerPropertiesEXT(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    pHostPointer: *const ::std::os::raw::c_void,
    pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
) -> VkResult {
    VK_GET_MEMORY_HOST_POINTER_PROPERTIES_EXT.unwrap()(device, handleType, pHostPointer, pMemoryHostPointerProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteBufferMarkerAMD(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
) {
    VK_CMD_WRITE_BUFFER_MARKER_AMD.unwrap()(commandBuffer, pipelineStage, dstBuffer, dstOffset, marker)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteBufferMarker2AMD(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
) {
    VK_CMD_WRITE_BUFFER_MARKER2_AMD.unwrap()(commandBuffer, stage, dstBuffer, dstOffset, marker)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
    physicalDevice: VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut VkTimeDomainKHR,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_CALIBRATEABLE_TIME_DOMAINS_EXT.unwrap()(physicalDevice, pTimeDomainCount, pTimeDomains)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetCalibratedTimestampsEXT(
    device: VkDevice,
    timestampCount: u32,
    pTimestampInfos: *const VkCalibratedTimestampInfoKHR,
    pTimestamps: *mut u64,
    pMaxDeviation: *mut u64,
) -> VkResult {
    VK_GET_CALIBRATED_TIMESTAMPS_EXT.unwrap()(device, timestampCount, pTimestampInfos, pTimestamps, pMaxDeviation)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksNV(commandBuffer: VkCommandBuffer, taskCount: u32, firstTask: u32) {
    VK_CMD_DRAW_MESH_TASKS_NV.unwrap()(commandBuffer, taskCount, firstTask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectNV(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_MESH_TASKS_INDIRECT_NV.unwrap()(commandBuffer, buffer, offset, drawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectCountNV(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_NV.unwrap()(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetExclusiveScissorEnableNV(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissorEnables: *const VkBool32,
) {
    VK_CMD_SET_EXCLUSIVE_SCISSOR_ENABLE_NV.unwrap()(commandBuffer, firstExclusiveScissor, exclusiveScissorCount, pExclusiveScissorEnables)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetExclusiveScissorNV(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissors: *const VkRect2D,
) {
    VK_CMD_SET_EXCLUSIVE_SCISSOR_NV.unwrap()(commandBuffer, firstExclusiveScissor, exclusiveScissorCount, pExclusiveScissors)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCheckpointNV(
    commandBuffer: VkCommandBuffer,
    pCheckpointMarker: *const ::std::os::raw::c_void,
) {
    VK_CMD_SET_CHECKPOINT_NV.unwrap()(commandBuffer, pCheckpointMarker)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetQueueCheckpointDataNV(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointDataNV,
) {
    VK_GET_QUEUE_CHECKPOINT_DATA_NV.unwrap()(queue, pCheckpointDataCount, pCheckpointData)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetQueueCheckpointData2NV(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointData2NV,
) {
    VK_GET_QUEUE_CHECKPOINT_DATA2_NV.unwrap()(queue, pCheckpointDataCount, pCheckpointData)
}

#[no_mangle]
pub unsafe extern "C" fn vkInitializePerformanceApiINTEL(
    device: VkDevice,
    pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL,
) -> VkResult {
    VK_INITIALIZE_PERFORMANCE_API_INTEL.unwrap()(device, pInitializeInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkUninitializePerformanceApiINTEL(device: VkDevice) {
    VK_UNINITIALIZE_PERFORMANCE_API_INTEL.unwrap()(device)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPerformanceMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceMarkerInfoINTEL,
) -> VkResult {
    VK_CMD_SET_PERFORMANCE_MARKER_INTEL.unwrap()(commandBuffer, pMarkerInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPerformanceStreamMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL,
) -> VkResult {
    VK_CMD_SET_PERFORMANCE_STREAM_MARKER_INTEL.unwrap()(commandBuffer, pMarkerInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPerformanceOverrideINTEL(
    commandBuffer: VkCommandBuffer,
    pOverrideInfo: *const VkPerformanceOverrideInfoINTEL,
) -> VkResult {
    VK_CMD_SET_PERFORMANCE_OVERRIDE_INTEL.unwrap()(commandBuffer, pOverrideInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquirePerformanceConfigurationINTEL(
    device: VkDevice,
    pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL,
    pConfiguration: *mut VkPerformanceConfigurationINTEL,
) -> VkResult {
    VK_ACQUIRE_PERFORMANCE_CONFIGURATION_INTEL.unwrap()(device, pAcquireInfo, pConfiguration)
}

#[no_mangle]
pub unsafe extern "C" fn vkReleasePerformanceConfigurationINTEL(
    device: VkDevice,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    VK_RELEASE_PERFORMANCE_CONFIGURATION_INTEL.unwrap()(device, configuration)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueSetPerformanceConfigurationINTEL(
    queue: VkQueue,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    VK_QUEUE_SET_PERFORMANCE_CONFIGURATION_INTEL.unwrap()(queue, configuration)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPerformanceParameterINTEL(
    device: VkDevice,
    parameter: VkPerformanceParameterTypeINTEL,
    pValue: *mut VkPerformanceValueINTEL,
) -> VkResult {
    VK_GET_PERFORMANCE_PARAMETER_INTEL.unwrap()(device, parameter, pValue)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetLocalDimmingAMD(
    device: VkDevice,
    swapChain: VkSwapchainKHR,
    localDimmingEnable: VkBool32,
) {
    VK_SET_LOCAL_DIMMING_AMD.unwrap()(device, swapChain, localDimmingEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferDeviceAddressEXT(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress {
    VK_GET_BUFFER_DEVICE_ADDRESS_EXT.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceToolPropertiesEXT(
    physicalDevice: VkPhysicalDevice,
    pToolCount: *mut u32,
    pToolProperties: *mut VkPhysicalDeviceToolProperties,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT.unwrap()(physicalDevice, pToolCount, pToolProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesNV,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV.unwrap()(physicalDevice, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
    physicalDevice: VkPhysicalDevice,
    pCombinationCount: *mut u32,
    pCombinations: *mut VkFramebufferMixedSamplesCombinationNV,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_SUPPORTED_FRAMEBUFFER_MIXED_SAMPLES_COMBINATIONS_NV.unwrap()(physicalDevice, pCombinationCount, pCombinations)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateHeadlessSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    VK_CREATE_HEADLESS_SURFACE_EXT.unwrap()(instance, pCreateInfo, pAllocator, pSurface)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineStippleEXT(
    commandBuffer: VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    VK_CMD_SET_LINE_STIPPLE_EXT.unwrap()(commandBuffer, lineStippleFactor, lineStipplePattern)
}

#[no_mangle]
pub unsafe extern "C" fn vkResetQueryPoolEXT(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    VK_RESET_QUERY_POOL_EXT.unwrap()(device, queryPool, firstQuery, queryCount)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCullModeEXT(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags) {
    VK_CMD_SET_CULL_MODE_EXT.unwrap()(commandBuffer, cullMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetFrontFaceEXT(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace) {
    VK_CMD_SET_FRONT_FACE_EXT.unwrap()(commandBuffer, frontFace)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPrimitiveTopologyEXT(
    commandBuffer: VkCommandBuffer,
    primitiveTopology: VkPrimitiveTopology,
) {
    VK_CMD_SET_PRIMITIVE_TOPOLOGY_EXT.unwrap()(commandBuffer, primitiveTopology)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportWithCountEXT(
    commandBuffer: VkCommandBuffer,
    viewportCount: u32,
    pViewports: *const VkViewport,
) {
    VK_CMD_SET_VIEWPORT_WITH_COUNT_EXT.unwrap()(commandBuffer, viewportCount, pViewports)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetScissorWithCountEXT(
    commandBuffer: VkCommandBuffer,
    scissorCount: u32,
    pScissors: *const VkRect2D,
) {
    VK_CMD_SET_SCISSOR_WITH_COUNT_EXT.unwrap()(commandBuffer, scissorCount, pScissors)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindVertexBuffers2EXT(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
    pStrides: *const VkDeviceSize,
) {
    VK_CMD_BIND_VERTEX_BUFFERS2_EXT.unwrap()(commandBuffer, firstBinding, bindingCount, pBuffers, pOffsets, pSizes, pStrides)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthTestEnableEXT(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32) {
    VK_CMD_SET_DEPTH_TEST_ENABLE_EXT.unwrap()(commandBuffer, depthTestEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthWriteEnableEXT(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32) {
    VK_CMD_SET_DEPTH_WRITE_ENABLE_EXT.unwrap()(commandBuffer, depthWriteEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthCompareOpEXT(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp) {
    VK_CMD_SET_DEPTH_COMPARE_OP_EXT.unwrap()(commandBuffer, depthCompareOp)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBoundsTestEnableEXT(
    commandBuffer: VkCommandBuffer,
    depthBoundsTestEnable: VkBool32,
) {
    VK_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_EXT.unwrap()(commandBuffer, depthBoundsTestEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilTestEnableEXT(
    commandBuffer: VkCommandBuffer,
    stencilTestEnable: VkBool32,
) {
    VK_CMD_SET_STENCIL_TEST_ENABLE_EXT.unwrap()(commandBuffer, stencilTestEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilOpEXT(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
) {
    VK_CMD_SET_STENCIL_OP_EXT.unwrap()(commandBuffer, faceMask, failOp, passOp, depthFailOp, compareOp)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMemoryToImageEXT(
    device: VkDevice,
    pCopyMemoryToImageInfo: *const VkCopyMemoryToImageInfo,
) -> VkResult {
    VK_COPY_MEMORY_TO_IMAGE_EXT.unwrap()(device, pCopyMemoryToImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyImageToMemoryEXT(
    device: VkDevice,
    pCopyImageToMemoryInfo: *const VkCopyImageToMemoryInfo,
) -> VkResult {
    VK_COPY_IMAGE_TO_MEMORY_EXT.unwrap()(device, pCopyImageToMemoryInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyImageToImageEXT(
    device: VkDevice,
    pCopyImageToImageInfo: *const VkCopyImageToImageInfo,
) -> VkResult {
    VK_COPY_IMAGE_TO_IMAGE_EXT.unwrap()(device, pCopyImageToImageInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkTransitionImageLayoutEXT(
    device: VkDevice,
    transitionCount: u32,
    pTransitions: *const VkHostImageLayoutTransitionInfo,
) -> VkResult {
    VK_TRANSITION_IMAGE_LAYOUT_EXT.unwrap()(device, transitionCount, pTransitions)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSubresourceLayout2EXT(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource2,
    pLayout: *mut VkSubresourceLayout2,
) {
    VK_GET_IMAGE_SUBRESOURCE_LAYOUT2_EXT.unwrap()(device, image, pSubresource, pLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkReleaseSwapchainImagesEXT(
    device: VkDevice,
    pReleaseInfo: *const VkReleaseSwapchainImagesInfoEXT,
) -> VkResult {
    VK_RELEASE_SWAPCHAIN_IMAGES_EXT.unwrap()(device, pReleaseInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetGeneratedCommandsMemoryRequirementsNV(
    device: VkDevice,
    pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_NV.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPreprocessGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
) {
    VK_CMD_PREPROCESS_GENERATED_COMMANDS_NV.unwrap()(commandBuffer, pGeneratedCommandsInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdExecuteGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
) {
    VK_CMD_EXECUTE_GENERATED_COMMANDS_NV.unwrap()(commandBuffer, isPreprocessed, pGeneratedCommandsInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindPipelineShaderGroupNV(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
    groupIndex: u32,
) {
    VK_CMD_BIND_PIPELINE_SHADER_GROUP_NV.unwrap()(commandBuffer, pipelineBindPoint, pipeline, groupIndex)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateIndirectCommandsLayoutNV(
    device: VkDevice,
    pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNV,
) -> VkResult {
    VK_CREATE_INDIRECT_COMMANDS_LAYOUT_NV.unwrap()(device, pCreateInfo, pAllocator, pIndirectCommandsLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyIndirectCommandsLayoutNV(
    device: VkDevice,
    indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_INDIRECT_COMMANDS_LAYOUT_NV.unwrap()(device, indirectCommandsLayout, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBias2EXT(
    commandBuffer: VkCommandBuffer,
    pDepthBiasInfo: *const VkDepthBiasInfoEXT,
) {
    VK_CMD_SET_DEPTH_BIAS2_EXT.unwrap()(commandBuffer, pDepthBiasInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    display: VkDisplayKHR,
) -> VkResult {
    VK_ACQUIRE_DRM_DISPLAY_EXT.unwrap()(physicalDevice, drmFd, display)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    connectorId: u32,
    display: *mut VkDisplayKHR,
) -> VkResult {
    VK_GET_DRM_DISPLAY_EXT.unwrap()(physicalDevice, drmFd, connectorId, display)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreatePrivateDataSlotEXT(
    device: VkDevice,
    pCreateInfo: *const VkPrivateDataSlotCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPrivateDataSlot: *mut VkPrivateDataSlot,
) -> VkResult {
    VK_CREATE_PRIVATE_DATA_SLOT_EXT.unwrap()(device, pCreateInfo, pAllocator, pPrivateDataSlot)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPrivateDataSlotEXT(
    device: VkDevice,
    privateDataSlot: VkPrivateDataSlot,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_PRIVATE_DATA_SLOT_EXT.unwrap()(device, privateDataSlot, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetPrivateDataEXT(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
) -> VkResult {
    VK_SET_PRIVATE_DATA_EXT.unwrap()(device, objectType, objectHandle, privateDataSlot, data)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPrivateDataEXT(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: *mut u64,
) {
    VK_GET_PRIVATE_DATA_EXT.unwrap()(device, objectType, objectHandle, privateDataSlot, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateCudaModuleNV(
    device: VkDevice,
    pCreateInfo: *const VkCudaModuleCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pModule: *mut VkCudaModuleNV,
) -> VkResult {
    VK_CREATE_CUDA_MODULE_NV.unwrap()(device, pCreateInfo, pAllocator, pModule)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetCudaModuleCacheNV(
    device: VkDevice,
    module: VkCudaModuleNV,
    pCacheSize: *mut usize,
    pCacheData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_CUDA_MODULE_CACHE_NV.unwrap()(device, module, pCacheSize, pCacheData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateCudaFunctionNV(
    device: VkDevice,
    pCreateInfo: *const VkCudaFunctionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pFunction: *mut VkCudaFunctionNV,
) -> VkResult {
    VK_CREATE_CUDA_FUNCTION_NV.unwrap()(device, pCreateInfo, pAllocator, pFunction)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyCudaModuleNV(
    device: VkDevice,
    module: VkCudaModuleNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_CUDA_MODULE_NV.unwrap()(device, module, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyCudaFunctionNV(
    device: VkDevice,
    function: VkCudaFunctionNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_CUDA_FUNCTION_NV.unwrap()(device, function, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCudaLaunchKernelNV(
    commandBuffer: VkCommandBuffer,
    pLaunchInfo: *const VkCudaLaunchInfoNV,
) {
    VK_CMD_CUDA_LAUNCH_KERNEL_NV.unwrap()(commandBuffer, pLaunchInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetLayoutSizeEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    pLayoutSizeInBytes: *mut VkDeviceSize,
) {
    VK_GET_DESCRIPTOR_SET_LAYOUT_SIZE_EXT.unwrap()(device, layout, pLayoutSizeInBytes)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetLayoutBindingOffsetEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    binding: u32,
    pOffset: *mut VkDeviceSize,
) {
    VK_GET_DESCRIPTOR_SET_LAYOUT_BINDING_OFFSET_EXT.unwrap()(device, layout, binding, pOffset)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorEXT(
    device: VkDevice,
    pDescriptorInfo: *const VkDescriptorGetInfoEXT,
    dataSize: usize,
    pDescriptor: *mut ::std::os::raw::c_void,
) {
    VK_GET_DESCRIPTOR_EXT.unwrap()(device, pDescriptorInfo, dataSize, pDescriptor)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindDescriptorBuffersEXT(
    commandBuffer: VkCommandBuffer,
    bufferCount: u32,
    pBindingInfos: *const VkDescriptorBufferBindingInfoEXT,
) {
    VK_CMD_BIND_DESCRIPTOR_BUFFERS_EXT.unwrap()(commandBuffer, bufferCount, pBindingInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDescriptorBufferOffsetsEXT(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    setCount: u32,
    pBufferIndices: *const u32,
    pOffsets: *const VkDeviceSize,
) {
    VK_CMD_SET_DESCRIPTOR_BUFFER_OFFSETS_EXT.unwrap()(commandBuffer, pipelineBindPoint, layout, firstSet, setCount, pBufferIndices, pOffsets)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindDescriptorBufferEmbeddedSamplersEXT(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
) {
    VK_CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_EXT.unwrap()(commandBuffer, pipelineBindPoint, layout, set)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkBufferCaptureDescriptorDataInfoEXT,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_BUFFER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT.unwrap()(device, pInfo, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkImageCaptureDescriptorDataInfoEXT,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_IMAGE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT.unwrap()(device, pInfo, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageViewOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkImageViewCaptureDescriptorDataInfoEXT,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_IMAGE_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT.unwrap()(device, pInfo, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSamplerOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkSamplerCaptureDescriptorDataInfoEXT,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_SAMPLER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT.unwrap()(device, pInfo, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureCaptureDescriptorDataInfoEXT,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_ACCELERATION_STRUCTURE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT.unwrap()(device, pInfo, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetFragmentShadingRateEnumNV(
    commandBuffer: VkCommandBuffer,
    shadingRate: VkFragmentShadingRateNV,
    combinerOps: *const VkFragmentShadingRateCombinerOpKHR,
) {
    VK_CMD_SET_FRAGMENT_SHADING_RATE_ENUM_NV.unwrap()(commandBuffer, shadingRate, combinerOps)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceFaultInfoEXT(
    device: VkDevice,
    pFaultCounts: *mut VkDeviceFaultCountsEXT,
    pFaultInfo: *mut VkDeviceFaultInfoEXT,
) -> VkResult {
    VK_GET_DEVICE_FAULT_INFO_EXT.unwrap()(device, pFaultCounts, pFaultInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetVertexInputEXT(
    commandBuffer: VkCommandBuffer,
    vertexBindingDescriptionCount: u32,
    pVertexBindingDescriptions: *const VkVertexInputBindingDescription2EXT,
    vertexAttributeDescriptionCount: u32,
    pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription2EXT,
) {
    VK_CMD_SET_VERTEX_INPUT_EXT.unwrap()(commandBuffer, vertexBindingDescriptionCount, pVertexBindingDescriptions, vertexAttributeDescriptionCount, pVertexAttributeDescriptions)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
    device: VkDevice,
    renderpass: VkRenderPass,
    pMaxWorkgroupSize: *mut VkExtent2D,
) -> VkResult {
    VK_GET_DEVICE_SUBPASS_SHADING_MAX_WORKGROUP_SIZE_HUAWEI.unwrap()(device, renderpass, pMaxWorkgroupSize)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSubpassShadingHUAWEI(commandBuffer: VkCommandBuffer) {
    VK_CMD_SUBPASS_SHADING_HUAWEI.unwrap()(commandBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindInvocationMaskHUAWEI(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    VK_CMD_BIND_INVOCATION_MASK_HUAWEI.unwrap()(commandBuffer, imageView, imageLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryRemoteAddressNV(
    device: VkDevice,
    pMemoryGetRemoteAddressInfo: *const VkMemoryGetRemoteAddressInfoNV,
    pAddress: *mut VkRemoteAddressNV,
) -> VkResult {
    VK_GET_MEMORY_REMOTE_ADDRESS_NV.unwrap()(device, pMemoryGetRemoteAddressInfo, pAddress)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelinePropertiesEXT(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoEXT,
    pPipelineProperties: *mut VkBaseOutStructure,
) -> VkResult {
    VK_GET_PIPELINE_PROPERTIES_EXT.unwrap()(device, pPipelineInfo, pPipelineProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPatchControlPointsEXT(commandBuffer: VkCommandBuffer, patchControlPoints: u32) {
    VK_CMD_SET_PATCH_CONTROL_POINTS_EXT.unwrap()(commandBuffer, patchControlPoints)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRasterizerDiscardEnableEXT(
    commandBuffer: VkCommandBuffer,
    rasterizerDiscardEnable: VkBool32,
) {
    VK_CMD_SET_RASTERIZER_DISCARD_ENABLE_EXT.unwrap()(commandBuffer, rasterizerDiscardEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBiasEnableEXT(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32) {
    VK_CMD_SET_DEPTH_BIAS_ENABLE_EXT.unwrap()(commandBuffer, depthBiasEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLogicOpEXT(commandBuffer: VkCommandBuffer, logicOp: VkLogicOp) {
    VK_CMD_SET_LOGIC_OP_EXT.unwrap()(commandBuffer, logicOp)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPrimitiveRestartEnableEXT(
    commandBuffer: VkCommandBuffer,
    primitiveRestartEnable: VkBool32,
) {
    VK_CMD_SET_PRIMITIVE_RESTART_ENABLE_EXT.unwrap()(commandBuffer, primitiveRestartEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorWriteEnableEXT(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pColorWriteEnables: *const VkBool32,
) {
    VK_CMD_SET_COLOR_WRITE_ENABLE_EXT.unwrap()(commandBuffer, attachmentCount, pColorWriteEnables)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMultiEXT(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pVertexInfo: *const VkMultiDrawInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
) {
    VK_CMD_DRAW_MULTI_EXT.unwrap()(commandBuffer, drawCount, pVertexInfo, instanceCount, firstInstance, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMultiIndexedEXT(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pIndexInfo: *const VkMultiDrawIndexedInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
    pVertexOffset: *const i32,
) {
    VK_CMD_DRAW_MULTI_INDEXED_EXT.unwrap()(commandBuffer, drawCount, pIndexInfo, instanceCount, firstInstance, stride, pVertexOffset)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateMicromapEXT(
    device: VkDevice,
    pCreateInfo: *const VkMicromapCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMicromap: *mut VkMicromapEXT,
) -> VkResult {
    VK_CREATE_MICROMAP_EXT.unwrap()(device, pCreateInfo, pAllocator, pMicromap)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyMicromapEXT(
    device: VkDevice,
    micromap: VkMicromapEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_MICROMAP_EXT.unwrap()(device, micromap, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBuildMicromapsEXT(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
) {
    VK_CMD_BUILD_MICROMAPS_EXT.unwrap()(commandBuffer, infoCount, pInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkBuildMicromapsEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
) -> VkResult {
    VK_BUILD_MICROMAPS_EXT.unwrap()(device, deferredOperation, infoCount, pInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapInfoEXT,
) -> VkResult {
    VK_COPY_MICROMAP_EXT.unwrap()(device, deferredOperation, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMicromapToMemoryEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
) -> VkResult {
    VK_COPY_MICROMAP_TO_MEMORY_EXT.unwrap()(device, deferredOperation, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMemoryToMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
) -> VkResult {
    VK_COPY_MEMORY_TO_MICROMAP_EXT.unwrap()(device, deferredOperation, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkWriteMicromapsPropertiesEXT(
    device: VkDevice,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
    stride: usize,
) -> VkResult {
    VK_WRITE_MICROMAPS_PROPERTIES_EXT.unwrap()(device, micromapCount, pMicromaps, queryType, dataSize, pData, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMicromapInfoEXT,
) {
    VK_CMD_COPY_MICROMAP_EXT.unwrap()(commandBuffer, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMicromapToMemoryEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
) {
    VK_CMD_COPY_MICROMAP_TO_MEMORY_EXT.unwrap()(commandBuffer, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMemoryToMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
) {
    VK_CMD_COPY_MEMORY_TO_MICROMAP_EXT.unwrap()(commandBuffer, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteMicromapsPropertiesEXT(
    commandBuffer: VkCommandBuffer,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    VK_CMD_WRITE_MICROMAPS_PROPERTIES_EXT.unwrap()(commandBuffer, micromapCount, pMicromaps, queryType, queryPool, firstQuery)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceMicromapCompatibilityEXT(
    device: VkDevice,
    pVersionInfo: *const VkMicromapVersionInfoEXT,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
) {
    VK_GET_DEVICE_MICROMAP_COMPATIBILITY_EXT.unwrap()(device, pVersionInfo, pCompatibility)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMicromapBuildSizesEXT(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkMicromapBuildInfoEXT,
    pSizeInfo: *mut VkMicromapBuildSizesInfoEXT,
) {
    VK_GET_MICROMAP_BUILD_SIZES_EXT.unwrap()(device, buildType, pBuildInfo, pSizeInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawClusterHUAWEI(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    VK_CMD_DRAW_CLUSTER_HUAWEI.unwrap()(commandBuffer, groupCountX, groupCountY, groupCountZ)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawClusterIndirectHUAWEI(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    VK_CMD_DRAW_CLUSTER_INDIRECT_HUAWEI.unwrap()(commandBuffer, buffer, offset)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetDeviceMemoryPriorityEXT(device: VkDevice, memory: VkDeviceMemory, priority: f32) {
    VK_SET_DEVICE_MEMORY_PRIORITY_EXT.unwrap()(device, memory, priority)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetLayoutHostMappingInfoVALVE(
    device: VkDevice,
    pBindingReference: *const VkDescriptorSetBindingReferenceVALVE,
    pHostMapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE,
) {
    VK_GET_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE.unwrap()(device, pBindingReference, pHostMapping)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetHostMappingVALVE(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    ppData: *mut *mut ::std::os::raw::c_void,
) {
    VK_GET_DESCRIPTOR_SET_HOST_MAPPING_VALVE.unwrap()(device, descriptorSet, ppData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMemoryIndirectNV(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
) {
    VK_CMD_COPY_MEMORY_INDIRECT_NV.unwrap()(commandBuffer, copyBufferAddress, copyCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMemoryToImageIndirectNV(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pImageSubresources: *const VkImageSubresourceLayers,
) {
    VK_CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_NV.unwrap()(commandBuffer, copyBufferAddress, copyCount, stride, dstImage, dstImageLayout, pImageSubresources)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDecompressMemoryNV(
    commandBuffer: VkCommandBuffer,
    decompressRegionCount: u32,
    pDecompressMemoryRegions: *const VkDecompressMemoryRegionNV,
) {
    VK_CMD_DECOMPRESS_MEMORY_NV.unwrap()(commandBuffer, decompressRegionCount, pDecompressMemoryRegions)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDecompressMemoryIndirectCountNV(
    commandBuffer: VkCommandBuffer,
    indirectCommandsAddress: VkDeviceAddress,
    indirectCommandsCountAddress: VkDeviceAddress,
    stride: u32,
) {
    VK_CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_NV.unwrap()(commandBuffer, indirectCommandsAddress, indirectCommandsCountAddress, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineIndirectMemoryRequirementsNV(
    device: VkDevice,
    pCreateInfo: *const VkComputePipelineCreateInfo,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_PIPELINE_INDIRECT_MEMORY_REQUIREMENTS_NV.unwrap()(device, pCreateInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdUpdatePipelineIndirectBufferNV(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
) {
    VK_CMD_UPDATE_PIPELINE_INDIRECT_BUFFER_NV.unwrap()(commandBuffer, pipelineBindPoint, pipeline)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineIndirectDeviceAddressNV(
    device: VkDevice,
    pInfo: *const VkPipelineIndirectDeviceAddressInfoNV,
) -> VkDeviceAddress {
    VK_GET_PIPELINE_INDIRECT_DEVICE_ADDRESS_NV.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthClampEnableEXT(commandBuffer: VkCommandBuffer, depthClampEnable: VkBool32) {
    VK_CMD_SET_DEPTH_CLAMP_ENABLE_EXT.unwrap()(commandBuffer, depthClampEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPolygonModeEXT(commandBuffer: VkCommandBuffer, polygonMode: VkPolygonMode) {
    VK_CMD_SET_POLYGON_MODE_EXT.unwrap()(commandBuffer, polygonMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRasterizationSamplesEXT(
    commandBuffer: VkCommandBuffer,
    rasterizationSamples: VkSampleCountFlagBits,
) {
    VK_CMD_SET_RASTERIZATION_SAMPLES_EXT.unwrap()(commandBuffer, rasterizationSamples)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetSampleMaskEXT(
    commandBuffer: VkCommandBuffer,
    samples: VkSampleCountFlagBits,
    pSampleMask: *const VkSampleMask,
) {
    VK_CMD_SET_SAMPLE_MASK_EXT.unwrap()(commandBuffer, samples, pSampleMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetAlphaToCoverageEnableEXT(
    commandBuffer: VkCommandBuffer,
    alphaToCoverageEnable: VkBool32,
) {
    VK_CMD_SET_ALPHA_TO_COVERAGE_ENABLE_EXT.unwrap()(commandBuffer, alphaToCoverageEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetAlphaToOneEnableEXT(commandBuffer: VkCommandBuffer, alphaToOneEnable: VkBool32) {
    VK_CMD_SET_ALPHA_TO_ONE_ENABLE_EXT.unwrap()(commandBuffer, alphaToOneEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLogicOpEnableEXT(commandBuffer: VkCommandBuffer, logicOpEnable: VkBool32) {
    VK_CMD_SET_LOGIC_OP_ENABLE_EXT.unwrap()(commandBuffer, logicOpEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorBlendEnableEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEnables: *const VkBool32,
) {
    VK_CMD_SET_COLOR_BLEND_ENABLE_EXT.unwrap()(commandBuffer, firstAttachment, attachmentCount, pColorBlendEnables)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorBlendEquationEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEquations: *const VkColorBlendEquationEXT,
) {
    VK_CMD_SET_COLOR_BLEND_EQUATION_EXT.unwrap()(commandBuffer, firstAttachment, attachmentCount, pColorBlendEquations)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorWriteMaskEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorWriteMasks: *const VkColorComponentFlags,
) {
    VK_CMD_SET_COLOR_WRITE_MASK_EXT.unwrap()(commandBuffer, firstAttachment, attachmentCount, pColorWriteMasks)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetTessellationDomainOriginEXT(
    commandBuffer: VkCommandBuffer,
    domainOrigin: VkTessellationDomainOrigin,
) {
    VK_CMD_SET_TESSELLATION_DOMAIN_ORIGIN_EXT.unwrap()(commandBuffer, domainOrigin)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRasterizationStreamEXT(commandBuffer: VkCommandBuffer, rasterizationStream: u32) {
    VK_CMD_SET_RASTERIZATION_STREAM_EXT.unwrap()(commandBuffer, rasterizationStream)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetConservativeRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
) {
    VK_CMD_SET_CONSERVATIVE_RASTERIZATION_MODE_EXT.unwrap()(commandBuffer, conservativeRasterizationMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetExtraPrimitiveOverestimationSizeEXT(
    commandBuffer: VkCommandBuffer,
    extraPrimitiveOverestimationSize: f32,
) {
    VK_CMD_SET_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT.unwrap()(commandBuffer, extraPrimitiveOverestimationSize)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthClipEnableEXT(commandBuffer: VkCommandBuffer, depthClipEnable: VkBool32) {
    VK_CMD_SET_DEPTH_CLIP_ENABLE_EXT.unwrap()(commandBuffer, depthClipEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetSampleLocationsEnableEXT(
    commandBuffer: VkCommandBuffer,
    sampleLocationsEnable: VkBool32,
) {
    VK_CMD_SET_SAMPLE_LOCATIONS_ENABLE_EXT.unwrap()(commandBuffer, sampleLocationsEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorBlendAdvancedEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendAdvanced: *const VkColorBlendAdvancedEXT,
) {
    VK_CMD_SET_COLOR_BLEND_ADVANCED_EXT.unwrap()(commandBuffer, firstAttachment, attachmentCount, pColorBlendAdvanced)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetProvokingVertexModeEXT(
    commandBuffer: VkCommandBuffer,
    provokingVertexMode: VkProvokingVertexModeEXT,
) {
    VK_CMD_SET_PROVOKING_VERTEX_MODE_EXT.unwrap()(commandBuffer, provokingVertexMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    lineRasterizationMode: VkLineRasterizationModeEXT,
) {
    VK_CMD_SET_LINE_RASTERIZATION_MODE_EXT.unwrap()(commandBuffer, lineRasterizationMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineStippleEnableEXT(
    commandBuffer: VkCommandBuffer,
    stippledLineEnable: VkBool32,
) {
    VK_CMD_SET_LINE_STIPPLE_ENABLE_EXT.unwrap()(commandBuffer, stippledLineEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthClipNegativeOneToOneEXT(
    commandBuffer: VkCommandBuffer,
    negativeOneToOne: VkBool32,
) {
    VK_CMD_SET_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT.unwrap()(commandBuffer, negativeOneToOne)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportWScalingEnableNV(
    commandBuffer: VkCommandBuffer,
    viewportWScalingEnable: VkBool32,
) {
    VK_CMD_SET_VIEWPORT_WSCALING_ENABLE_NV.unwrap()(commandBuffer, viewportWScalingEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportSwizzleNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportSwizzles: *const VkViewportSwizzleNV,
) {
    VK_CMD_SET_VIEWPORT_SWIZZLE_NV.unwrap()(commandBuffer, firstViewport, viewportCount, pViewportSwizzles)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageToColorEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorEnable: VkBool32,
) {
    VK_CMD_SET_COVERAGE_TO_COLOR_ENABLE_NV.unwrap()(commandBuffer, coverageToColorEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageToColorLocationNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorLocation: u32,
) {
    VK_CMD_SET_COVERAGE_TO_COLOR_LOCATION_NV.unwrap()(commandBuffer, coverageToColorLocation)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageModulationModeNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationMode: VkCoverageModulationModeNV,
) {
    VK_CMD_SET_COVERAGE_MODULATION_MODE_NV.unwrap()(commandBuffer, coverageModulationMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageModulationTableEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableEnable: VkBool32,
) {
    VK_CMD_SET_COVERAGE_MODULATION_TABLE_ENABLE_NV.unwrap()(commandBuffer, coverageModulationTableEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageModulationTableNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableCount: u32,
    pCoverageModulationTable: *const f32,
) {
    VK_CMD_SET_COVERAGE_MODULATION_TABLE_NV.unwrap()(commandBuffer, coverageModulationTableCount, pCoverageModulationTable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetShadingRateImageEnableNV(
    commandBuffer: VkCommandBuffer,
    shadingRateImageEnable: VkBool32,
) {
    VK_CMD_SET_SHADING_RATE_IMAGE_ENABLE_NV.unwrap()(commandBuffer, shadingRateImageEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRepresentativeFragmentTestEnableNV(
    commandBuffer: VkCommandBuffer,
    representativeFragmentTestEnable: VkBool32,
) {
    VK_CMD_SET_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV.unwrap()(commandBuffer, representativeFragmentTestEnable)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageReductionModeNV(
    commandBuffer: VkCommandBuffer,
    coverageReductionMode: VkCoverageReductionModeNV,
) {
    VK_CMD_SET_COVERAGE_REDUCTION_MODE_NV.unwrap()(commandBuffer, coverageReductionMode)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetShaderModuleIdentifierEXT(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
) {
    VK_GET_SHADER_MODULE_IDENTIFIER_EXT.unwrap()(device, shaderModule, pIdentifier)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetShaderModuleCreateInfoIdentifierEXT(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
) {
    VK_GET_SHADER_MODULE_CREATE_INFO_IDENTIFIER_EXT.unwrap()(device, pCreateInfo, pIdentifier)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceOpticalFlowImageFormatsNV(
    physicalDevice: VkPhysicalDevice,
    pOpticalFlowImageFormatInfo: *const VkOpticalFlowImageFormatInfoNV,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut VkOpticalFlowImageFormatPropertiesNV,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_OPTICAL_FLOW_IMAGE_FORMATS_NV.unwrap()(physicalDevice, pOpticalFlowImageFormatInfo, pFormatCount, pImageFormatProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateOpticalFlowSessionNV(
    device: VkDevice,
    pCreateInfo: *const VkOpticalFlowSessionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pSession: *mut VkOpticalFlowSessionNV,
) -> VkResult {
    VK_CREATE_OPTICAL_FLOW_SESSION_NV.unwrap()(device, pCreateInfo, pAllocator, pSession)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyOpticalFlowSessionNV(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_OPTICAL_FLOW_SESSION_NV.unwrap()(device, session, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkBindOpticalFlowSessionImageNV(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    bindingPoint: VkOpticalFlowSessionBindingPointNV,
    view: VkImageView,
    layout: VkImageLayout,
) -> VkResult {
    VK_BIND_OPTICAL_FLOW_SESSION_IMAGE_NV.unwrap()(device, session, bindingPoint, view, layout)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdOpticalFlowExecuteNV(
    commandBuffer: VkCommandBuffer,
    session: VkOpticalFlowSessionNV,
    pExecuteInfo: *const VkOpticalFlowExecuteInfoNV,
) {
    VK_CMD_OPTICAL_FLOW_EXECUTE_NV.unwrap()(commandBuffer, session, pExecuteInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkAntiLagUpdateAMD(device: VkDevice, pData: *const VkAntiLagDataAMD) {
    VK_ANTI_LAG_UPDATE_AMD.unwrap()(device, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateShadersEXT(
    device: VkDevice,
    createInfoCount: u32,
    pCreateInfos: *const VkShaderCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pShaders: *mut VkShaderEXT,
) -> VkResult {
    VK_CREATE_SHADERS_EXT.unwrap()(device, createInfoCount, pCreateInfos, pAllocator, pShaders)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyShaderEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_SHADER_EXT.unwrap()(device, shader, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetShaderBinaryDataEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pDataSize: *mut usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_SHADER_BINARY_DATA_EXT.unwrap()(device, shader, pDataSize, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindShadersEXT(
    commandBuffer: VkCommandBuffer,
    stageCount: u32,
    pStages: *const VkShaderStageFlagBits,
    pShaders: *const VkShaderEXT,
) {
    VK_CMD_BIND_SHADERS_EXT.unwrap()(commandBuffer, stageCount, pStages, pShaders)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthClampRangeEXT(
    commandBuffer: VkCommandBuffer,
    depthClampMode: VkDepthClampModeEXT,
    pDepthClampRange: *const VkDepthClampRangeEXT,
) {
    VK_CMD_SET_DEPTH_CLAMP_RANGE_EXT.unwrap()(commandBuffer, depthClampMode, pDepthClampRange)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFramebufferTilePropertiesQCOM(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pPropertiesCount: *mut u32,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult {
    VK_GET_FRAMEBUFFER_TILE_PROPERTIES_QCOM.unwrap()(device, framebuffer, pPropertiesCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDynamicRenderingTilePropertiesQCOM(
    device: VkDevice,
    pRenderingInfo: *const VkRenderingInfo,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult {
    VK_GET_DYNAMIC_RENDERING_TILE_PROPERTIES_QCOM.unwrap()(device, pRenderingInfo, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetLatencySleepModeNV(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSleepModeInfo: *const VkLatencySleepModeInfoNV,
) -> VkResult {
    VK_SET_LATENCY_SLEEP_MODE_NV.unwrap()(device, swapchain, pSleepModeInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkLatencySleepNV(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSleepInfo: *const VkLatencySleepInfoNV,
) -> VkResult {
    VK_LATENCY_SLEEP_NV.unwrap()(device, swapchain, pSleepInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkSetLatencyMarkerNV(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pLatencyMarkerInfo: *const VkSetLatencyMarkerInfoNV,
) {
    VK_SET_LATENCY_MARKER_NV.unwrap()(device, swapchain, pLatencyMarkerInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetLatencyTimingsNV(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pLatencyMarkerInfo: *mut VkGetLatencyMarkerInfoNV,
) {
    VK_GET_LATENCY_TIMINGS_NV.unwrap()(device, swapchain, pLatencyMarkerInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueNotifyOutOfBandNV(
    queue: VkQueue,
    pQueueTypeInfo: *const VkOutOfBandQueueTypeInfoNV,
) {
    VK_QUEUE_NOTIFY_OUT_OF_BAND_NV.unwrap()(queue, pQueueTypeInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetAttachmentFeedbackLoopEnableEXT(
    commandBuffer: VkCommandBuffer,
    aspectMask: VkImageAspectFlags,
) {
    VK_CMD_SET_ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT.unwrap()(commandBuffer, aspectMask)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetGeneratedCommandsMemoryRequirementsEXT(
    device: VkDevice,
    pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoEXT,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    VK_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_EXT.unwrap()(device, pInfo, pMemoryRequirements)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPreprocessGeneratedCommandsEXT(
    commandBuffer: VkCommandBuffer,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoEXT,
    stateCommandBuffer: VkCommandBuffer,
) {
    VK_CMD_PREPROCESS_GENERATED_COMMANDS_EXT.unwrap()(commandBuffer, pGeneratedCommandsInfo, stateCommandBuffer)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdExecuteGeneratedCommandsEXT(
    commandBuffer: VkCommandBuffer,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoEXT,
) {
    VK_CMD_EXECUTE_GENERATED_COMMANDS_EXT.unwrap()(commandBuffer, isPreprocessed, pGeneratedCommandsInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateIndirectCommandsLayoutEXT(
    device: VkDevice,
    pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutEXT,
) -> VkResult {
    VK_CREATE_INDIRECT_COMMANDS_LAYOUT_EXT.unwrap()(device, pCreateInfo, pAllocator, pIndirectCommandsLayout)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyIndirectCommandsLayoutEXT(
    device: VkDevice,
    indirectCommandsLayout: VkIndirectCommandsLayoutEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_INDIRECT_COMMANDS_LAYOUT_EXT.unwrap()(device, indirectCommandsLayout, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateIndirectExecutionSetEXT(
    device: VkDevice,
    pCreateInfo: *const VkIndirectExecutionSetCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pIndirectExecutionSet: *mut VkIndirectExecutionSetEXT,
) -> VkResult {
    VK_CREATE_INDIRECT_EXECUTION_SET_EXT.unwrap()(device, pCreateInfo, pAllocator, pIndirectExecutionSet)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyIndirectExecutionSetEXT(
    device: VkDevice,
    indirectExecutionSet: VkIndirectExecutionSetEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_INDIRECT_EXECUTION_SET_EXT.unwrap()(device, indirectExecutionSet, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkUpdateIndirectExecutionSetPipelineEXT(
    device: VkDevice,
    indirectExecutionSet: VkIndirectExecutionSetEXT,
    executionSetWriteCount: u32,
    pExecutionSetWrites: *const VkWriteIndirectExecutionSetPipelineEXT,
) {
    VK_UPDATE_INDIRECT_EXECUTION_SET_PIPELINE_EXT.unwrap()(device, indirectExecutionSet, executionSetWriteCount, pExecutionSetWrites)
}

#[no_mangle]
pub unsafe extern "C" fn vkUpdateIndirectExecutionSetShaderEXT(
    device: VkDevice,
    indirectExecutionSet: VkIndirectExecutionSetEXT,
    executionSetWriteCount: u32,
    pExecutionSetWrites: *const VkWriteIndirectExecutionSetShaderEXT,
) {
    VK_UPDATE_INDIRECT_EXECUTION_SET_SHADER_EXT.unwrap()(device, indirectExecutionSet, executionSetWriteCount, pExecutionSetWrites)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixFlexibleDimensionsPropertiesNV,
) -> VkResult {
    VK_GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV.unwrap()(physicalDevice, pPropertyCount, pProperties)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateAccelerationStructureKHR(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureKHR,
) -> VkResult {
    VK_CREATE_ACCELERATION_STRUCTURE_KHR.unwrap()(device, pCreateInfo, pAllocator, pAccelerationStructure)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyAccelerationStructureKHR(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    VK_DESTROY_ACCELERATION_STRUCTURE_KHR.unwrap()(device, accelerationStructure, pAllocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBuildAccelerationStructuresKHR(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
) {
    VK_CMD_BUILD_ACCELERATION_STRUCTURES_KHR.unwrap()(commandBuffer, infoCount, pInfos, ppBuildRangeInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBuildAccelerationStructuresIndirectKHR(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pIndirectDeviceAddresses: *const VkDeviceAddress,
    pIndirectStrides: *const u32,
    ppMaxPrimitiveCounts: *const *const u32,
) {
    VK_CMD_BUILD_ACCELERATION_STRUCTURES_INDIRECT_KHR.unwrap()(commandBuffer, infoCount, pInfos, pIndirectDeviceAddresses, pIndirectStrides, ppMaxPrimitiveCounts)
}

#[no_mangle]
pub unsafe extern "C" fn vkBuildAccelerationStructuresKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
) -> VkResult {
    VK_BUILD_ACCELERATION_STRUCTURES_KHR.unwrap()(device, deferredOperation, infoCount, pInfos, ppBuildRangeInfos)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
) -> VkResult {
    VK_COPY_ACCELERATION_STRUCTURE_KHR.unwrap()(device, deferredOperation, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyAccelerationStructureToMemoryKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
) -> VkResult {
    VK_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR.unwrap()(device, deferredOperation, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMemoryToAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
) -> VkResult {
    VK_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR.unwrap()(device, deferredOperation, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkWriteAccelerationStructuresPropertiesKHR(
    device: VkDevice,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
    stride: usize,
) -> VkResult {
    VK_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR.unwrap()(device, accelerationStructureCount, pAccelerationStructures, queryType, dataSize, pData, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
) {
    VK_CMD_COPY_ACCELERATION_STRUCTURE_KHR.unwrap()(commandBuffer, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyAccelerationStructureToMemoryKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
) {
    VK_CMD_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR.unwrap()(commandBuffer, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMemoryToAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
) {
    VK_CMD_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR.unwrap()(commandBuffer, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureDeviceAddressKHR(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR,
) -> VkDeviceAddress {
    VK_GET_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_KHR.unwrap()(device, pInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteAccelerationStructuresPropertiesKHR(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    VK_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR.unwrap()(commandBuffer, accelerationStructureCount, pAccelerationStructures, queryType, queryPool, firstQuery)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceAccelerationStructureCompatibilityKHR(
    device: VkDevice,
    pVersionInfo: *const VkAccelerationStructureVersionInfoKHR,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
) {
    VK_GET_DEVICE_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR.unwrap()(device, pVersionInfo, pCompatibility)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureBuildSizesKHR(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pMaxPrimitiveCounts: *const u32,
    pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
) {
    VK_GET_ACCELERATION_STRUCTURE_BUILD_SIZES_KHR.unwrap()(device, buildType, pBuildInfo, pMaxPrimitiveCounts, pSizeInfo)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdTraceRaysKHR(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
) {
    VK_CMD_TRACE_RAYS_KHR.unwrap()(commandBuffer, pRaygenShaderBindingTable, pMissShaderBindingTable, pHitShaderBindingTable, pCallableShaderBindingTable, width, height, depth)
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateRayTracingPipelinesKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    VK_CREATE_RAY_TRACING_PIPELINES_KHR.unwrap()(device, deferredOperation, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult {
    VK_GET_RAY_TRACING_CAPTURE_REPLAY_SHADER_GROUP_HANDLES_KHR.unwrap()(device, pipeline, firstGroup, groupCount, dataSize, pData)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdTraceRaysIndirectKHR(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    indirectDeviceAddress: VkDeviceAddress,
) {
    VK_CMD_TRACE_RAYS_INDIRECT_KHR.unwrap()(commandBuffer, pRaygenShaderBindingTable, pMissShaderBindingTable, pHitShaderBindingTable, pCallableShaderBindingTable, indirectDeviceAddress)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRayTracingShaderGroupStackSizeKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    group: u32,
    groupShader: VkShaderGroupShaderKHR,
) -> VkDeviceSize {
    VK_GET_RAY_TRACING_SHADER_GROUP_STACK_SIZE_KHR.unwrap()(device, pipeline, group, groupShader)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRayTracingPipelineStackSizeKHR(
    commandBuffer: VkCommandBuffer,
    pipelineStackSize: u32,
) {
    VK_CMD_SET_RAY_TRACING_PIPELINE_STACK_SIZE_KHR.unwrap()(commandBuffer, pipelineStackSize)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksEXT(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    VK_CMD_DRAW_MESH_TASKS_EXT.unwrap()(commandBuffer, groupCountX, groupCountY, groupCountZ)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectEXT(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_MESH_TASKS_INDIRECT_EXT.unwrap()(commandBuffer, buffer, offset, drawCount, stride)
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectCountEXT(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_EXT.unwrap()(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride)
}

