//
// 3.1.1  Offset 00h: CAP (Controller Capabilities)
//
#[derive(Debug, Clone, Copy)]
enum NVME_AMS_OPTION {
    NVME_AMS_ROUND_ROBIN = 0,
    NVME_AMS_WEIGHTED_ROUND_ROBIN_URGENT = 1,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CONTROLLER_CAPABILITIES {
    MQES: u16, // RO - Maximum Queue Entries Supported (MQES)
    CQR: bool, // RO - Contiguous Queues Required (CQR)

    // Bit 17, 18 - AMS; RO - Arbitration Mechanism Supported (AMS)
    AMS_WeightedRoundRobinWithUrgent: bool, // Bit 17: Weighted Round Robin with Urgent;
    AMS_VendorSpecific: bool,               // Bit 18: Vendor Specific.

    Reserved0: u8, // RO - bit 19 ~ 23
    TO: u8,        // RO - Timeout (TO)
    DSTRD: u8,     // RO - Doorbell Stride (DSTRD)
    NSSRS: bool,   // RO - NVM Subsystem Reset Supported (NSSRS)

    // Bit 37 ~ 44 - CSS; RO - Command Sets Supported (CSS)
    CSS_NVM: bool,        // Bit 37: NVM command set
    CSS_Reserved0: bool,  // Bit 38: Reserved
    CSS_Reserved1: bool,  // Bit 39: Reserved
    CSS_Reserved2: bool,  // Bit 40: Reserved
    CSS_Reserved3: bool,  // Bit 41: Reserved
    CSS_Reserved4: bool,  // Bit 42: Reserved
    CSS_MultipleIo: bool, // Bit 43: One or more IO command sets
    CSS_AdminOnly: bool,  // Bit 44: Only Admin command set (no IO command set)

    Reserved2: u8, // RO - bit 45 ~ 47
    MPSMIN: u8,    // RO - Memory Page Size Minimum (MPSMIN)
    MPSMAX: u8,    // RO - Memory Page Size Maximum (MPSMAX)
    Reserved3: u8, // RO - bit 56 ~ 63
}

//
// 3.1.2  Offset 08h: VS (Version)
//
#[derive(Debug, Clone, Copy)]
struct NVME_VERSION_STRUCT {
    //LSB
    TER: u8, // Tertiary Version Number (TER)
    MNR: u8, // Minor Version Number (MNR)
    MJR: u16, // Major Version Number (MJR)
             //MSB
}

union NVME_VERSION {
    version: NVME_VERSION_STRUCT,
    AsUlong: u32,
}

//
// 3.1.5  Offset 14h: CC (Controller Configuration)
//
#[derive(Debug, Clone, Copy)]
enum NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS {
    NVME_CC_SHN_NO_NOTIFICATION = 0,
    NVME_CC_SHN_NORMAL_SHUTDOWN = 1,
    NVME_CC_SHN_ABRUPT_SHUTDOWN = 2,
}

#[derive(Debug, Clone, Copy)]
enum NVME_CSS_COMMAND_SETS {
    NVME_CSS_NVM_COMMAND_SET = 0,
    NVME_CSS_ALL_SUPPORTED_IO_COMMAND_SET = 6,
    NVME_CSS_ADMIN_COMMAND_SET_ONLY = 7,
}

#[derive(Debug, Clone, Copy)]
union NVME_CONTROLLER_CONFIGURATION {
    DUMMYSTRUCTNAME: NVME_CONTROLLER_CONFIGURATION_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CONTROLLER_CONFIGURATION_STRUCT {
    EN: u32,        // RW - Enable (EN)
    Reserved0: u32, // RO
    CSS: u32,       // RW - I/O  Command Set Selected (CSS)
    MPS: u32,       // RW - Memory Page Size (MPS)
    AMS: u32,       // RW - Arbitration Mechanism Selected (AMS)
    SHN: u32,       // RW - Shutdown Notification (SHN)
    IOSQES: u32,    // RW - I/O  Submission Queue Entry Size (IOSQES)
    IOCQES: u32,    // RW - I/O  Completion Queue Entry Size (IOCQES)
    Reserved1: u32, // RO
}

//
// 3.1.6  Offset 1Ch: CSTS (Controller Status)
//
#[derive(Debug, Clone, Copy)]
enum NVME_CSTS_SHST_SHUTDOWN_STATUS {
    NVME_CSTS_SHST_NO_SHUTDOWN = 0,
    NVME_CSTS_SHST_SHUTDOWN_IN_PROCESS = 1,
    NVME_CSTS_SHST_SHUTDOWN_COMPLETED = 2,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CONTROLLER_STATUS {
    RDY: bool,      // RO - Ready (RDY)
    CFS: bool,      // RO - Controller Fatal Status (CFS)
    SHST: u8,       // RO - Shutdown Status (SHST)
    NSSRO: bool,    // RW1C - NVM Subsystem Reset Occurred (NSSRO)
    PP: bool,       // RO - Processing Paused (PP)
    Reserved0: u32, // RO
}

//
// 3.1.7  Offset 20h: NSSR (NVM Subsystem Reset)
//
#[derive(Debug, Clone, Copy)]
struct NVME_NVM_SUBSYSTEM_RESET {
    NSSRC: u32, // RW - NVM Subsystem Reset Control (NSSRC)
}

//
// 3.1.8  Offset 24h: AQA (Admin Queue Attributes)
//
#[derive(Debug, Clone, Copy)]
union NVME_ADMIN_QUEUE_ATTRIBUTES {
    bits: u32,
    fields: NVME_ADMIN_QUEUE_ATTRIBUTES_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_ADMIN_QUEUE_ATTRIBUTES_FIELDS {
    ASQS: u16,     // RW - Admin  Submission Queue Size (ASQS)
    Reserved0: u8, // RO
    ACQS: u16,     // RW - Admin  Completion Queue Size (ACQS)
    Reserved1: u8, // RO
}
//
// 3.1.9  Offset 28h: ASQ (Admin Submission Queue Base Address)
//
#[derive(Debug, Clone, Copy)]
union NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS {
    DUMMYSTRUCTNAME: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_STRUCT,
    AsUlonglong: u64,
}

#[derive(Debug, Clone, Copy)]
struct NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_STRUCT {
    Reserved0: u64, // RO
    ASQB: u64,      // RW - Admin Submission Queue Base (ASQB)
}

//
// 3.1.10  Offset 30h: ACQ (Admin Completion Queue Base Address)
//
#[derive(Debug, Clone, Copy)]
union NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS {
    DUMMYSTRUCTNAME: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_STRUCT,
    AsUlonglong: u64,
}

#[derive(Debug, Clone, Copy)]
struct NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_STRUCT {
    Reserved0: u64, // RO
    ACQB: u64,      // RW - Admin Completion Queue Base (ACQB)
}
//
// 3.1.11 Offset 38h: CMBLOC (Controller Memory Buffer Location)
//
#[derive(Debug, Clone, Copy)]
union NVME_CONTROLLER_MEMORY_BUFFER_LOCATION {
    DUMMYSTRUCTNAME: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_STRUCT {
    BIR: u32,      // RO - Base Indicator Register (BIR)
    Reserved: u32, // RO
    OFST: u32,     // RO - Offset (OFST)
}

//
// 3.1.12 Offset 3Ch: CMBSZ (Controller Memory Buffer Size)
//
enum NVME_CMBSZ_SIZE_UNITS {
    NVME_CMBSZ_SIZE_UNITS_4KB = 0,
    NVME_CMBSZ_SIZE_UNITS_64KB = 1,
    NVME_CMBSZ_SIZE_UNITS_1MB = 2,
    NVME_CMBSZ_SIZE_UNITS_16MB = 3,
    NVME_CMBSZ_SIZE_UNITS_256MB = 4,
    NVME_CMBSZ_SIZE_UNITS_4GB = 5,
    NVME_CMBSZ_SIZE_UNITS_64GB = 6,
}

#[derive(Debug, Clone, Copy)]
union NVME_CONTROLLER_MEMORY_BUFFER_SIZE {
    DUMMYSTRUCTNAME: NVME_CONTROLLER_MEMORY_BUFFER_SIZE_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CONTROLLER_MEMORY_BUFFER_SIZE_STRUCT {
    SQS: u32,      // RO - Submission Queue Support (SQS)
    CQS: u32,      // RO - Completion Queue Support (CQS)
    LISTS: u32,    // RO - PRP SGL List Support (LISTS)
    RDS: u32,      // RO - Read Data Support (RDS)
    WDS: u32,      // RO - Write Data Support (WDS)
    Reserved: u32, // RO
    SZU: u32,      // RO - Size Units (SZU)
    SZ: u32,       // RO - Size (SZ)
}

//
// 3.1.13  Offset (1000h + ((2y) * (4 << CAP.DSTRD))): SQyTDBL (Submission Queue y Tail Doorbell)
//
#[derive(Debug, Clone, Copy)]
union NVME_SUBMISSION_QUEUE_TAIL_DOORBELL {
    DUMMYSTRUCTNAME: NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_STRUCT {
    SQT: u16,       // RW - Submission Queue Tail (SQT)
    Reserved0: u16, // RO
}

//
// 3.1.14  Offset  (1000h + ((2y + 1) * (4 << CAP.DSTRD))): CQyHDBL (Completion Queue y Head Doorbell)
//
#[derive(Debug, Clone, Copy)]
union NVME_COMPLETION_QUEUE_HEAD_DOORBELL {
    DUMMYSTRUCTNAME: NVME_COMPLETION_QUEUE_HEAD_DOORBELL_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_COMPLETION_QUEUE_HEAD_DOORBELL_STRUCT {
    CQH: u16,       // RW - Completion Queue Head (CQH)
    Reserved0: u16, // RO
}

struct NVME_CONTROLLER_REGISTERS {
    CAP: NVME_CONTROLLER_CAPABILITIES, // Controller Capabilities; 8 bytes
    VS: NVME_VERSION,                  // Version
    INTMS: u32,                        // Interrupt Mask Set
    INTMC: u32,                        // Interrupt Mask Clear
    CC: NVME_CONTROLLER_CONFIGURATION, // Controller Configuration
    Reserved0: u32,
    CSTS: NVME_CONTROLLER_STATUS,                   // Controller Status
    NSSR: NVME_NVM_SUBSYSTEM_RESET,                 // NVM Subsystem Reset (Optional)
    AQA: NVME_ADMIN_QUEUE_ATTRIBUTES,               // Admin Queue Attributes
    ASQ: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS,  // Admin Submission Queue Base Address; 8 bytes
    ACQ: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS,  // Admin Completion Queue Base Address; 8 bytes
    CMBLOC: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION, // Controller Memory Buffer Location (Optional)
    CMBSZ: NVME_CONTROLLER_MEMORY_BUFFER_SIZE,      // Controller Memory Buffer Size (Optional)
    Reserved2: [u32; 944],                          // 40h ~ EFFh
    Reserved3: [u32; 64],                           // F00h ~ FFFh, Command Set Specific
    Doorbells: [u32; 0], // Start of the first Doorbell register. (Admin SQ Tail Doorbell)
}

//
// Command completion status
// The "Phase Tag" field and "Status Field" are separated in spec. We define them in the same data structure to ease the memory access from software.
//
#[repr(C)]
union NVME_COMMAND_STATUS {
    DUMMYSTRUCTNAME: NVME_COMMAND_STATUS_STRUCT,
    AsUshort: u16,
}

#[repr(C)]
struct NVME_COMMAND_STATUS_STRUCT {
    P: u16,   // Phase Tag (P)
    SC: u16,  // Status Code (SC)
    SCT: u16, // Status Code Type (SCT)
    Reserved: u16,
    M: u16,   // More (M)
    DNR: u16, // Do Not Retry (DNR)
}

//
// Command completion entry
//
#[repr(C)]
struct NVME_COMPLETION_ENTRY {
    DW0: u32,
    DW1: u32,
    DW2: NVME_COMPLETION_ENTRY_DW2,
    DW3: NVME_COMPLETION_ENTRY_DW3,
}

#[repr(C)]
union NVME_COMPLETION_ENTRY_DW2 {
    DUMMYSTRUCTNAME: NVME_COMPLETION_ENTRY_DW2_STRUCT,
    AsUlong: u32,
}

#[repr(C)]
struct NVME_COMPLETION_ENTRY_DW2_STRUCT {
    SQHD: u16, // SQ Head Pointer (SQHD)
    SQID: u16, // SQ Identifier (SQID)
}

#[repr(C)]
union NVME_COMPLETION_ENTRY_DW3 {
    DUMMYSTRUCTNAME: NVME_COMPLETION_ENTRY_DW3_STRUCT,
    AsUlong: u32,
}

#[repr(C)]
struct NVME_COMPLETION_ENTRY_DW3_STRUCT {
    CID: u16, // Command Identifier (CID)
    Status: NVME_COMMAND_STATUS,
}

//
// Completion entry DW0 for NVME_ADMIN_COMMAND_ASYNC_EVENT_REQUEST
//

#[derive(Debug, Clone, Copy)]
enum NVME_ASYNC_EVENT_TYPES {
    NVME_ASYNC_EVENT_TYPE_ERROR_STATUS = 0,
    NVME_ASYNC_EVENT_TYPE_HEALTH_STATUS = 1,
    NVME_ASYNC_EVENT_TYPE_NOTICE = 2,
    NVME_ASYNC_EVENT_TYPE_IO_COMMAND_SET_STATUS = 6,
    NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC = 7,
}

//
// Error Status: NVME_ASYNC_EVENT_TYPE_ERROR_STATUS
//
enum NVME_ASYNC_EVENT_ERROR_STATUS_CODES {
    NVME_ASYNC_ERROR_INVALID_SUBMISSION_QUEUE = 0,
    NVME_ASYNC_ERROR_INVALID_DOORBELL_WRITE_VALUE = 1,
    NVME_ASYNC_ERROR_DIAG_FAILURE = 2,
    NVME_ASYNC_ERROR_PERSISTENT_INTERNAL_DEVICE_ERROR = 3,
    NVME_ASYNC_ERROR_TRANSIENT_INTERNAL_DEVICE_ERROR = 4,
    NVME_ASYNC_ERROR_FIRMWARE_IMAGE_LOAD_ERROR = 5,
}

//
// SMART/Health Status: NVME_ASYNC_EVENT_TYPE_HEALTH_STATUS
//
enum NVME_ASYNC_EVENT_HEALTH_STATUS_CODES {
    NVME_ASYNC_HEALTH_NVM_SUBSYSTEM_RELIABILITY = 0,
    NVME_ASYNC_HEALTH_TEMPERATURE_THRESHOLD = 1,
    NVME_ASYNC_HEALTH_SPARE_BELOW_THRESHOLD = 2,
}
// Notice Status: NVME_ASYNC_EVENT_TYPE_NOTICE
//
enum NVME_ASYNC_EVENT_NOTICE_CODES {
    NVME_ASYNC_NOTICE_NAMESPACE_ATTRIBUTE_CHANGED = 0,
    NVME_ASYNC_NOTICE_FIRMWARE_ACTIVATION_STARTING = 1,
    NVME_ASYNC_NOTICE_TELEMETRY_LOG_CHANGED = 2,
    NVME_ASYNC_NOTICE_ASYMMETRIC_ACCESS_CHANGE = 3,
    NVME_ASYNC_NOTICE_PREDICTABLE_LATENCY_EVENT_AGGREGATE_LOG_CHANGE = 4,
    NVME_ASYNC_NOTICE_LBA_STATUS_INFORMATION_ALERT = 5,
    NVME_ASYNC_NOTICE_ENDURANCE_GROUP_EVENT_AGGREGATE_LOG_CHANGE = 6,
    NVME_ASYNC_NOTICE_ZONE_DESCRIPTOR_CHANGED = 0xEF,
}

enum NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES {
    NVME_ASYNC_IO_CMD_SET_RESERVATION_LOG_PAGE_AVAILABLE = 0,
    NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED = 1,
    NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED_WITH_UNEXPECTED_DEALLOCATION = 2,
}

struct NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST {
    AsyncEventType: u32,
    Reserved0: u32,
    AsyncEventInfo: u32,
    LogPage: u32,
    Reserved1: u32,
}

enum NVME_STATUS_TYPES {
    NVME_STATUS_TYPE_GENERIC_COMMAND = 0,
    NVME_STATUS_TYPE_COMMAND_SPECIFIC = 1,
    NVME_STATUS_TYPE_MEDIA_ERROR = 2,
    NVME_STATUS_TYPE_VENDOR_SPECIFIC = 7,
}

//
//  Status Code (SC) of NVME_STATUS_TYPE_GENERIC_COMMAND
//
enum NVME_STATUS_GENERIC_COMMAND_CODES {
    NVME_STATUS_SUCCESS_COMPLETION = 0x00,
    NVME_STATUS_INVALID_COMMAND_OPCODE = 0x01,
    NVME_STATUS_INVALID_FIELD_IN_COMMAND = 0x02,
    NVME_STATUS_COMMAND_ID_CONFLICT = 0x03,
    NVME_STATUS_DATA_TRANSFER_ERROR = 0x04,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_POWER_LOSS_NOTIFICATION = 0x05,
    NVME_STATUS_INTERNAL_DEVICE_ERROR = 0x06,
    NVME_STATUS_COMMAND_ABORT_REQUESTED = 0x07,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_SQ_DELETION = 0x08,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_FUSED_COMMAND = 0x09,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_MISSING_COMMAND = 0x0A,
    NVME_STATUS_INVALID_NAMESPACE_OR_FORMAT = 0x0B,
    NVME_STATUS_COMMAND_SEQUENCE_ERROR = 0x0C,
    NVME_STATUS_INVALID_SGL_LAST_SEGMENT_DESCR = 0x0D,
    NVME_STATUS_INVALID_NUMBER_OF_SGL_DESCR = 0x0E,
    NVME_STATUS_DATA_SGL_LENGTH_INVALID = 0x0F,
    NVME_STATUS_METADATA_SGL_LENGTH_INVALID = 0x10,
    NVME_STATUS_SGL_DESCR_TYPE_INVALID = 0x11,
    NVME_STATUS_INVALID_USE_OF_CONTROLLER_MEMORY_BUFFER = 0x12,
    NVME_STATUS_PRP_OFFSET_INVALID = 0x13,
    NVME_STATUS_ATOMIC_WRITE_UNIT_EXCEEDED = 0x14,
    NVME_STATUS_OPERATION_DENIED = 0x15,
    NVME_STATUS_SGL_OFFSET_INVALID = 0x16,
    NVME_STATUS_RESERVED = 0x17,
    NVME_STATUS_HOST_IDENTIFIER_INCONSISTENT_FORMAT = 0x18,
    NVME_STATUS_KEEP_ALIVE_TIMEOUT_EXPIRED = 0x19,
    NVME_STATUS_KEEP_ALIVE_TIMEOUT_INVALID = 0x1A,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_PREEMPT_ABORT = 0x1B,
    NVME_STATUS_SANITIZE_FAILED = 0x1C,
    NVME_STATUS_SANITIZE_IN_PROGRESS = 0x1D,
    NVME_STATUS_SGL_DATA_BLOCK_GRANULARITY_INVALID = 0x1E,

    NVME_STATUS_DIRECTIVE_TYPE_INVALID = 0x70,
    NVME_STATUS_DIRECTIVE_ID_INVALID = 0x71,

    NVME_STATUS_NVM_LBA_OUT_OF_RANGE = 0x80,
    NVME_STATUS_NVM_CAPACITY_EXCEEDED = 0x81,
    NVME_STATUS_NVM_NAMESPACE_NOT_READY = 0x82,
    NVME_STATUS_NVM_RESERVATION_CONFLICT = 0x83,
    NVME_STATUS_FORMAT_IN_PROGRESS = 0x84,
}

//
//  Status Code (SC) of NVME_STATUS_TYPE_COMMAND_SPECIFIC
//
enum NVME_STATUS_COMMAND_SPECIFIC_CODES {
    NVME_STATUS_COMPLETION_QUEUE_INVALID = 0x00, // Create I/O Submission Queue
    NVME_STATUS_INVALID_QUEUE_IDENTIFIER = 0x01, // Create I/O Submission Queue, Create I/O Completion Queue, Delete I/O Completion Queue, Delete I/O Submission Queue
    NVME_STATUS_MAX_QUEUE_SIZE_EXCEEDED = 0x02, // Create I/O Submission Queue, Create I/O Completion Queue
    NVME_STATUS_ABORT_COMMAND_LIMIT_EXCEEDED = 0x03, // Abort
    NVME_STATUS_ASYNC_EVENT_REQUEST_LIMIT_EXCEEDED = 0x05, // Asynchronous Event Request
    NVME_STATUS_INVALID_FIRMWARE_SLOT = 0x06,   // Firmware Commit
    NVME_STATUS_INVALID_FIRMWARE_IMAGE = 0x07,  // Firmware Commit
    NVME_STATUS_INVALID_INTERRUPT_VECTOR = 0x08, // Create I/O Completion Queue
    NVME_STATUS_INVALID_LOG_PAGE = 0x09,        // Get Log Page
    NVME_STATUS_INVALID_FORMAT = 0x0A,          // Format NVM
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_CONVENTIONAL_RESET = 0x0B, // Firmware Commit
    NVME_STATUS_INVALID_QUEUE_DELETION = 0x0C,  // Delete I/O Completion Queue
    NVME_STATUS_FEATURE_ID_NOT_SAVEABLE = 0x0D, // Set Features
    NVME_STATUS_FEATURE_NOT_CHANGEABLE = 0x0E,  // Set Features
    NVME_STATUS_FEATURE_NOT_NAMESPACE_SPECIFIC = 0x0F, // Set Features
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_NVM_SUBSYSTEM_RESET = 0x10, // Firmware Commit
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_RESET = 0x11, // Firmware Commit
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_MAX_TIME_VIOLATION = 0x12, // Firmware Commit
    NVME_STATUS_FIRMWARE_ACTIVATION_PROHIBITED = 0x13, // Firmware Commit
    NVME_STATUS_OVERLAPPING_RANGE = 0x14, // Firmware Commit, Firmware Image Download, Set Features

    NVME_STATUS_NAMESPACE_INSUFFICIENT_CAPACITY = 0x15, // Namespace Management
    NVME_STATUS_NAMESPACE_IDENTIFIER_UNAVAILABLE = 0x16, // Namespace Management
    NVME_STATUS_NAMESPACE_ALREADY_ATTACHED = 0x18,      // Namespace Attachment
    NVME_STATUS_NAMESPACE_IS_PRIVATE = 0x19,            // Namespace Attachment
    NVME_STATUS_NAMESPACE_NOT_ATTACHED = 0x1A,          // Namespace Attachment
    NVME_STATUS_NAMESPACE_THIN_PROVISIONING_NOT_SUPPORTED = 0x1B, // Namespace Management
    NVME_STATUS_CONTROLLER_LIST_INVALID = 0x1C,         // Namespace Attachment

    NVME_STATUS_DEVICE_SELF_TEST_IN_PROGRESS = 0x1D, // Device Self-test

    NVME_STATUS_BOOT_PARTITION_WRITE_PROHIBITED = 0x1E, // Firmware Commit

    NVME_STATUS_INVALID_CONTROLLER_IDENTIFIER = 0x1F, // Virtualization Management
    NVME_STATUS_INVALID_SECONDARY_CONTROLLER_STATE = 0x20, // Virtualization Management
    NVME_STATUS_INVALID_NUMBER_OF_CONTROLLER_RESOURCES = 0x21, // Virtualization Management
    NVME_STATUS_INVALID_RESOURCE_IDENTIFIER = 0x22,   // Virtualization Management

    NVME_STATUS_SANITIZE_PROHIBITED_ON_PERSISTENT_MEMORY = 0x23, // Sanitize

    NVME_STATUS_INVALID_ANA_GROUP_IDENTIFIER = 0x24, // Namespace Management
    NVME_STATUS_ANA_ATTACH_FAILED = 0x25,            // Namespace Attachment

    NVME_IO_COMMAND_SET_NOT_SUPPORTED = 0x29, // Namespace Attachment/Management
    NVME_IO_COMMAND_SET_NOT_ENABLED = 0x2A,   // Namespace Attachment
    NVME_IO_COMMAND_SET_COMBINATION_REJECTED = 0x2B, // Set Features
    NVME_IO_COMMAND_SET_INVALID = 0x2C,       // Identify

    NVME_STATUS_STREAM_RESOURCE_ALLOCATION_FAILED = 0x7F, // Streams Directive
    NVME_STATUS_ZONE_INVALID_FORMAT = 0x7F,               // Namespace Management

    NVME_STATUS_NVM_CONFLICTING_ATTRIBUTES = 0x80, // Dataset Management, Read, Write
    NVME_STATUS_NVM_INVALID_PROTECTION_INFORMATION = 0x81, // Compare, Read, Write, Write Zeroes
    NVME_STATUS_NVM_ATTEMPTED_WRITE_TO_READ_ONLY_RANGE = 0x82, // Dataset Management, Write, Write Uncorrectable, Write Zeroes
    NVME_STATUS_NVM_COMMAND_SIZE_LIMIT_EXCEEDED = 0x83,        // Dataset Management

    NVME_STATUS_ZONE_BOUNDARY_ERROR = 0xB8, // Compare, Read, Verify, Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_FULL = 0xB9, // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_READ_ONLY = 0xBA, // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_OFFLINE = 0xBB, // Compare, Read, Verify, Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_INVALID_WRITE = 0xBC, // Write, Write Uncorrectable, Write Zeroes, Copy
    NVME_STATUS_ZONE_TOO_MANY_ACTIVE = 0xBD, // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append, Zone Management Send
    NVME_STATUS_ZONE_TOO_MANY_OPEN = 0xBE, // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append, Zone Management Send
    NVME_STATUS_ZONE_INVALID_STATE_TRANSITION = 0xBF, // Zone Management Send
}

//
//  Status Code (SC) of NVME_STATUS_TYPE_MEDIA_ERROR
//
enum NVME_STATUS_MEDIA_ERROR_CODES {
    NVME_STATUS_NVM_WRITE_FAULT = 0x80,
    NVME_STATUS_NVM_UNRECOVERED_READ_ERROR = 0x81,
    NVME_STATUS_NVM_END_TO_END_GUARD_CHECK_ERROR = 0x82,
    NVME_STATUS_NVM_END_TO_END_APPLICATION_TAG_CHECK_ERROR = 0x83,
    NVME_STATUS_NVM_END_TO_END_REFERENCE_TAG_CHECK_ERROR = 0x84,
    NVME_STATUS_NVM_COMPARE_FAILURE = 0x85,
    NVME_STATUS_NVM_ACCESS_DENIED = 0x86,
    NVME_STATUS_NVM_DEALLOCATED_OR_UNWRITTEN_LOGICAL_BLOCK = 0x87,
}

//
// Admin Command Set
//
enum NVME_ADMIN_COMMANDS {
    NVME_ADMIN_COMMAND_DELETE_IO_SQ = 0x00,
    NVME_ADMIN_COMMAND_CREATE_IO_SQ = 0x01,
    NVME_ADMIN_COMMAND_GET_LOG_PAGE = 0x02,
    NVME_ADMIN_COMMAND_DELETE_IO_CQ = 0x04,
    NVME_ADMIN_COMMAND_CREATE_IO_CQ = 0x05,
    NVME_ADMIN_COMMAND_IDENTIFY = 0x06,
    NVME_ADMIN_COMMAND_ABORT = 0x08,
    NVME_ADMIN_COMMAND_SET_FEATURES = 0x09,
    NVME_ADMIN_COMMAND_GET_FEATURES = 0x0A,
    NVME_ADMIN_COMMAND_ASYNC_EVENT_REQUEST = 0x0C,
    NVME_ADMIN_COMMAND_NAMESPACE_MANAGEMENT = 0x0D,

    NVME_ADMIN_COMMAND_FIRMWARE_ACTIVATE = 0x10,
    NVME_ADMIN_COMMAND_FIRMWARE_COMMIT = 0x10, // "Firmware Activate" command has been renamed to "Firmware Commit" command in spec v1.2
    NVME_ADMIN_COMMAND_FIRMWARE_IMAGE_DOWNLOAD = 0x11,
    NVME_ADMIN_COMMAND_DEVICE_SELF_TEST = 0x14,
    NVME_ADMIN_COMMAND_NAMESPACE_ATTACHMENT = 0x15,

    NVME_ADMIN_COMMAND_DIRECTIVE_SEND = 0x19,
    NVME_ADMIN_COMMAND_DIRECTIVE_RECEIVE = 0x1A,
    NVME_ADMIN_COMMAND_VIRTUALIZATION_MANAGEMENT = 0x1C,
    NVME_ADMIN_COMMAND_NVME_MI_SEND = 0x1D,
    NVME_ADMIN_COMMAND_NVME_MI_RECEIVE = 0x1E,

    NVME_ADMIN_COMMAND_DOORBELL_BUFFER_CONFIG = 0x7C,

    NVME_ADMIN_COMMAND_FORMAT_NVM = 0x80,
    NVME_ADMIN_COMMAND_SECURITY_SEND = 0x81,
    NVME_ADMIN_COMMAND_SECURITY_RECEIVE = 0x82,
    NVME_ADMIN_COMMAND_SANITIZE = 0x84,
    NVME_ADMIN_COMMAND_GET_LBA_STATUS = 0x86,
}

//
// Features for Get/Set Features command
//
enum NVME_FEATURES {
    NVME_FEATURE_ARBITRATION = 0x01,
    NVME_FEATURE_POWER_MANAGEMENT = 0x02,
    NVME_FEATURE_LBA_RANGE_TYPE = 0x03,
    NVME_FEATURE_TEMPERATURE_THRESHOLD = 0x04,
    NVME_FEATURE_ERROR_RECOVERY = 0x05,
    NVME_FEATURE_VOLATILE_WRITE_CACHE = 0x06,
    NVME_FEATURE_NUMBER_OF_QUEUES = 0x07,
    NVME_FEATURE_INTERRUPT_COALESCING = 0x08,
    NVME_FEATURE_INTERRUPT_VECTOR_CONFIG = 0x09,
    NVME_FEATURE_WRITE_ATOMICITY = 0x0A,
    NVME_FEATURE_ASYNC_EVENT_CONFIG = 0x0B,
    NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION = 0x0C,
    NVME_FEATURE_HOST_MEMORY_BUFFER = 0x0D,
    NVME_FEATURE_TIMESTAMP = 0x0E,
    NVME_FEATURE_KEEP_ALIVE = 0x0F,
    NVME_FEATURE_HOST_CONTROLLED_THERMAL_MANAGEMENT = 0x10,
    NVME_FEATURE_NONOPERATIONAL_POWER_STATE = 0x11,
    NVME_FEATURE_READ_RECOVERY_LEVEL_CONFIG = 0x12,
    NVME_FEATURE_PREDICTABLE_LATENCY_MODE_CONFIG = 0x13,
    NVME_FEATURE_PREDICTABLE_LATENCY_MODE_WINDOW = 0x14,
    NVME_FEATURE_LBA_STATUS_INFORMATION_REPORT_INTERVAL = 0x15,
    NVME_FEATURE_HOST_BEHAVIOR_SUPPORT = 0x16,
    NVME_FEATURE_SANITIZE_CONFIG = 0x17,
    NVME_FEATURE_ENDURANCE_GROUP_EVENT_CONFIG = 0x18,
    NVME_FEATURE_IO_COMMAND_SET_PROFILE = 0x19,

    NVME_FEATURE_ENHANCED_CONTROLLER_METADATA = 0x7D,
    NVME_FEATURE_CONTROLLER_METADATA = 0x7E,
    NVME_FEATURE_NAMESPACE_METADATA = 0x7F,

    NVME_FEATURE_NVM_SOFTWARE_PROGRESS_MARKER = 0x80,
    NVME_FEATURE_NVM_HOST_IDENTIFIER = 0x81,
    NVME_FEATURE_NVM_RESERVATION_NOTIFICATION_MASK = 0x82,
    NVME_FEATURE_NVM_RESERVATION_PERSISTANCE = 0x83,
    NVME_FEATURE_NVM_NAMESPACE_WRITE_PROTECTION_CONFIG = 0x84,

    NVME_FEATURE_ERROR_INJECTION = 0xC0, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_CLEAR_FW_UPDATE_HISTORY = 0xC1, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_READONLY_WRITETHROUGH_MODE = 0xC2, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS = 0xC3, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_ENABLE_IEEE1667_SILO = 0xC4, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_PLP_HEALTH_MONITOR = 0xC5,   // This is from OCP NVMe Cloud SSD spec.
}

//
// Abort command: parameter
//
#[repr(C)]
union NVME_CDW10_ABORT {
    DUMMYSTRUCTNAME: NVME_CDW10_ABORT_STRUCT,
    AsUlong: u32,
}

#[repr(C)]
struct NVME_CDW10_ABORT_STRUCT {
    SQID: u8, // Submission Queue Identifier (SQID)
    CID: u16, // Command Identifier (CID)
}

//
// Identify Command of Controller or Namespace Structure (CNS)
//
enum NVME_IDENTIFY_CNS_CODES {
    NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE = 0x0,
    NVME_IDENTIFY_CNS_CONTROLLER = 0x1,
    NVME_IDENTIFY_CNS_ACTIVE_NAMESPACES = 0x2, // A list of up to 1024 active namespace IDs is returned to the host containing active namespaces with a namespace identifier greater than the value specified in the Namespace Identifier (CDW1.NSID) field.
    NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE = 0x3,
    NVME_IDENTIFY_CNS_NVM_SET = 0x4,

    NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE_IO_COMMAND_SET = 0x5,
    NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET = 0x6,
    NVME_IDENTIFY_CNS_ACTIVE_NAMESPACE_LIST_IO_COMMAND_SET = 0x7,

    NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_LIST = 0x10,
    NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE = 0x11,
    NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NSID = 0x12,
    NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NVM_SUBSYSTEM = 0x13,
    NVME_IDENTIFY_CNS_PRIMARY_CONTROLLER_CAPABILITIES = 0x14,
    NVME_IDENTIFY_CNS_SECONDARY_CONTROLLER_LIST = 0x15,
    NVME_IDENTIFY_CNS_NAMESPACE_GRANULARITY_LIST = 0x16,
    NVME_IDENTIFY_CNS_UUID_LIST = 0x17,
    NVME_IDENTIFY_CNS_DOMAIN_LIST = 0x18,
    NVME_IDENTIFY_CNS_ENDURANCE_GROUP_LIST = 0x19,

    NVME_IDENTIFY_CNS_ALLOCATED_NAMSPACE_LIST_IO_COMMAND_SET = 0x1A,
    NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_IO_COMMAND_SET = 0x1B,
    NVME_IDENTIFY_CNS_IO_COMMAND_SET = 0x1C,
}

//
// Identify Command Set Identifiers (CSI)
//
enum NVME_COMMAND_SET_IDENTIFIERS {
    NVME_COMMAND_SET_NVM = 0x0,
    NVME_COMMAND_SET_KEY_VALUE = 0x1,
    NVME_COMMAND_SET_ZONED_NAMESPACE = 0x2,
}

union NVME_CDW10_IDENTIFY {
    DUMMYSTRUCTNAME: NVME_CDW10_IDENTIFY_STRUCT,
    AsUlong: u32,
}

struct NVME_CDW10_IDENTIFY_STRUCT {
    CNS: u8, // Controller or Namespace Structure (CNS, Defined in NVME_IDENTIFY_CNS_CODES)
    Reserved: u8,
    CNTID: u16, // Controller Identifier (CNTID)
}

union NVME_CDW11_IDENTIFY {
    DUMMYSTRUCTNAME: NVME_CDW11_IDENTIFY_STRUCT,
    DUMMYSTRUCTNAME2: NVME_CDW11_IDENTIFY_STRUCT2,
    AsUlong: u32,
}

struct NVME_CDW11_IDENTIFY_STRUCT {
    NVMSETID: u16, // NVM Set Identifier
    Reserved: u16,
}

struct NVME_CDW11_IDENTIFY_STRUCT2 {
    CNSID: u32, // CNS Specific Identifier (NVM Set ID/Domain ID/Endurance Group ID)
    Reserved2: u8,
    CSI: u8, // Command Set Identifier (CSI, Defined in NVME_COMMAND_SET_IDENTIFIERS)
}

//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE (0x0)
//
union NVME_LBA_FORMAT {
    DUMMYSTRUCTNAME: NVME_LBA_FORMAT_STRUCT,
    AsUlong: u32,
}

struct NVME_LBA_FORMAT_STRUCT {
    MS: u16,   // bit 0:15     Metadata Size (MS)
    LBADS: u8, // bit 16:23    LBA  Data  Size (LBADS)

    RP: u8,        // bit 24:25    Relative Performance (RP)
    Reserved0: u8, // bit 26:31
}

//
//

#[repr(C)]
union NVM_RESERVATION_CAPABILITIES {
    bits: u8,
    fields: NVM_RESERVATION_CAPABILITIES_FIELDS,
}

#[repr(C)]
struct NVM_RESERVATION_CAPABILITIES_FIELDS {
    PersistThroughPowerLoss: u8,
    WriteExclusiveReservation: u8,
    ExclusiveAccessReservation: u8,
    WriteExclusiveRegistrantsOnlyReservation: u8,
    ExclusiveAccessRegistrantsOnlyReservation: u8,
    WriteExclusiveAllRegistrantsReservation: u8,
    ExclusiveAccessAllRegistrantsReservation: u8,
    Reserved: u8,
}

//
//

#[repr(C)]
struct NVME_IDENTIFY_NAMESPACE_DATA {
    NSZE: u64,                               // byte 0:7.    M - Namespace Size (NSZE)
    NCAP: u64,                               // byte 8:15    M - Namespace Capacity (NCAP)
    NUSE: u64,                               // byte 16:23   M - Namespace Utilization (NUSE)
    NSFEAT: NamespaceFeatures,               // byte 24      M - Namespace Features (NSFEAT)
    NLBAF: u8,                               // byte 25      M - Number of LBA Formats (NLBAF)
    FLBAS: FormattedLbaSize,                 // byte 26      M - Formatted LBA Size (FLBAS)
    MC: MetadataCapabilities,                // byte 27      M - Metadata Capabilities (MC)
    DPC: DataProtectionCapabilities, // byte 28  M - End-to-end Data Protection Capabilities (DPC)
    DPS: DataProtectionTypeSettings, // byte 29  M - End-to-end Data Protection Type Settings (DPS)
    NMIC: NamespaceMultiPathIoCapabilities, // byte 30  O - Namespace Multi-path I/O and Namespace Sharing Capabilities (NMIC)
    RESCAP: NvmReservationCapabilities,     // byte 31  O - Reservation Capabilities (RESCAP)
    FPI: FormatProgressIndicator,           // byte 32  O - Format Progress Indicator (FPI)
    DLFEAT: DeallocatedLogicalBlockFeatures, // byte 33
    NAWUN: u16,       // byte 34:35 O - Namespace Atomic Write Unit Normal (NAWUN)
    NAWUPF: u16,      // byte 36:37 O - Namespace Atomic Write Unit Power Fail (NAWUPF)
    NACWU: u16,       // byte 38:39 O - Namespace Atomic Compare & Write Unit (NACWU)
    NABSN: u16,       // byte 40:41 O - Namespace Atomic Boundary Size Normal (NABSN)
    NABO: u16,        // byte 42:43 O - Namespace Atomic Boundary Offset (NABO)
    NABSPF: u16,      // byte 44:45 O - Namespace Atomic Boundary Size Power Fail (NABSPF)
    NOIOB: u16,       // byte 46:47 O - Namespace Optimal IO Boundary (NOIOB)
    NVMCAP: [u8; 16], // byte 48:63 O - NVM Capacity (NVMCAP)
    NPWG: u16,        // byte 64:65 O - Namespace Preferred Write Granularity (NPWG)
    NPWA: u16,        // byte 66:67 O - Namespace Preferred Write Alignment (NPWA)
    NPDG: u16,        // byte 68:69 O - Namespace Preferred Deallocate Granularity (NPDG)
    NPDA: u16,        // byte 70:71 O - Namespace Preferred Deallocate Alignment (NPDA)

    NOWS: u16, // byte 72:73 O - Namespace Optimal Write Size (NOWS)

    MSSRL: u16,          // byte 74:75 O - Maximum Single Source Range Length(MSSRL)
    MCL: u32,            // byte 76:79 O - Maximum Copy Length(MCL)
    MSRC: u8,            // byte 80 O - Maximum Source Range Count(MSRC)
    Reserved2: [u8; 11], // byte 81:91

    ANAGRPID: u32, // byte 92:95 O - ANA Group Identifier (ANAGRPID)

    Reserved3: [u8; 3], // byte 96:98

    NSATTR: NamespaceAttributes, // byte 99 O - Namespace Attributes{

    NVMSETID: u16, // byte 100:101 O - Associated NVM Set Identifier

    ENDGID: u16, // byte 102:103 O - Associated Endurance Group Identier

    NGUID: [u8; 16], // byte 104:119 O - Namespace Globally Unique Identifier (NGUID)

    EUI64: [u8; 8], // byte 120:127 M - IEEE Extended Unique Identifier (EUI64)

    LBAF: [NVME_LBA_FORMAT; 16], // byte 128:131 M - LBA Format 0 Support (LBAF0)
    // byte 132:135 O - LBA Format 1 Support (LBAF1)
    // byte 136:139 O - LBA Format 2 Support (LBAF2)
    // byte 140:143 O - LBA Format 3 Support (LBAF3)
    // byte 144:147 O - LBA Format 4 Support (LBAF4)
    // byte 148:151 O - LBA Format 5 Support (LBAF5)
    // byte 152:155 O - LBA Format 6 Support (LBAF6)
    // byte 156:159 O - LBA Format 7 Support (LBAF7)
    // byte 160:163 O - LBA Format 8 Support (LBAF8)
    // byte 164:167 O - LBA Format 9 Support (LBAF9)
    // byte 168:171 O - LBA Format 10 Support (LBAF10)
    // byte 172:175 O - LBA Format 11 Support (LBAF11)
    // byte 176:179 O - LBA Format 12 Support (LBAF12)
    // byte 180:183 O - LBA Format 13 Support (LBAF13)
    // byte 184:187 O - LBA Format 14 Support (LBAF14)
    // byte 188:191 O - LBA Format 15 Support (LBAF15)
    Reserved4: [u8; 192], // byte 192:383

    VS: [u8; 3712], // byte 384:4095 O - Vendor Specific (VS): This range of bytes is allocated for vendor specific usage.
}

#[repr(C)]
struct NamespaceFeatures {
    ThinProvisioning: u8,
    NameSpaceAtomicWriteUnit: u8,
    DeallocatedOrUnwrittenError: u8,
    SkipReuseUI: u8,
    NameSpaceIoOptimization: u8,
    Reserved: u8,
}

#[repr(C)]
struct FormattedLbaSize {
    LbaFormatIndex: u8,
    MetadataInExtendedDataLBA: u8,
    Reserved: u8,
}

#[repr(C)]
struct MetadataCapabilities {
    MetadataInExtendedDataLBA: u8,
    MetadataInSeparateBuffer: u8,
    Reserved: u8,
}

#[repr(C)]
struct DataProtectionCapabilities {
    ProtectionInfoType1: u8,
    ProtectionInfoType2: u8,
    ProtectionInfoType3: u8,
    InfoAtBeginningOfMetadata: u8,
    InfoAtEndOfMetadata: u8,
    Reserved: u8,
}

#[repr(C)]
struct DataProtectionTypeSettings {
    ProtectionInfoTypeEnabled: u8,
    InfoAtBeginningOfMetadata: u8,
    Reserved: u8,
}

#[repr(C)]
struct NamespaceMultiPathIoCapabilities {
    SharedNameSpace: u8,
    Reserved: u8,
}

#[repr(C)]
struct NvmReservationCapabilities {
    PersistThroughPowerLoss: u8,
    WriteExclusiveReservation: u8,
    ExclusiveAccessReservation: u8,
    WriteExclusiveRegistrantsOnlyReservation: u8,
    ExclusiveAccessRegistrantsOnlyReservation: u8,
    WriteExclusiveAllRegistrantsReservation: u8,
    ExclusiveAccessAllRegistrantsReservation: u8,
    Reserved: u8,
}

#[repr(C)]
struct FormatProgressIndicator {
    PercentageRemained: u8,
    Supported: u8,
}

#[repr(C)]
struct DeallocatedLogicalBlockFeatures {
    ReadBehavior: u8,
    WriteZeroes: u8,
    GuardFieldWithCRC: u8,
    Reserved: u8,
}

struct NamespaceAttributes {
    WriteProtected: u8, // Write Protected
    Reserved: u8,       // Reserved
} // byte 99 O - Namespace Attributes

//
// Output of NVME_IDENTIFY_CNS_CONTROLLER (0x01)
//
#[repr(C)]
struct NVME_POWER_STATE_DESC {
    MP: u16, // bit 0:15.    Maximum  Power (MP)

    Reserved0: u8, // bit 16:23

    MPS: u8,       // bit 24: Max Power Scale (MPS)
    NOPS: u8,      // bit 25: Non-Operational State (NOPS)
    Reserved1: u8, // bit 26:31

    ENLAT: u32, // bit 32:63.   Entry Latency (ENLAT)
    EXLAT: u32, // bit 64:95.   Exit Latency (EXLAT)

    RRT: u8,       // bit 96:100.  Relative Read Throughput (RRT)
    Reserved2: u8, // bit 101:103

    RRL: u8,       // bit 104:108  Relative Read Latency (RRL)
    Reserved3: u8, // bit 109:111

    RWT: u8,       // bit 112:116  Relative Write Throughput (RWT)
    Reserved4: u8, // bit 117:119

    RWL: u8,       // bit 120:124  Relative Write Latency (RWL)
    Reserved5: u8, // bit 125:127

    IDLP: u16, // bit 128:143  Idle Power (IDLP)

    Reserved6: u8, // bit 144:149
    IPS: u8,       // bit 150:151  Idle Power Scale (IPS)

    Reserved7: u8, // bit 152:159

    ACTP: u16, // bit 160:175  Active Power (ACTP)

    APW: u8,       // bit 176:178  Active Power Workload (APW)
    Reserved8: u8, // bit 179:181
    APS: u8,       // bit 182:183  Active Power Scale (APS)

    Reserved9: [u8; 9], // bit 184:255.
}

#[derive(Debug, Clone, Copy)]
struct NVME_IDENTIFY_CONTROLLER_DATA {
    VID: u16,                         // byte 0:1.    M - PCI Vendor ID (VID)
    SSVID: u16,                       // byte 2:3.    M - PCI Subsystem Vendor ID (SSVID)
    SN: [u8; 20],                     // byte 4: 23.  M - Serial Number (SN)
    MN: [u8; 40],                     // byte 24:63.  M - Model Number (MN)
    FR: [u8; 8],                      // byte 64:71.  M - Firmware Revision (FR)
    RAB: u8,                          // byte 72.     M - Recommended Arbitration Burst (RAB)
    IEEE: [u8; 3], // byte 73:75.  M - IEEE OUI Identifier (IEEE). Controller Vendor code.
    CMIC: CMIC, // byte 76.     O - Controller Multi-Path I/O and Namespace Sharing Capabilities (CMIC)
    MDTS: u8,   // byte 77.     M - Maximum Data Transfer Size (MDTS)
    CNTLID: u16, // byte 78:79.   M - Controller ID (CNTLID)
    VER: u32,   // byte 80:83.   M - Version (VER)
    RTD3R: u32, // byte 84:87.   M - RTD3 Resume Latency (RTD3R)
    RTD3E: u32, // byte 88:91.   M - RTD3 Entry Latency (RTD3E)
    OAES: OAES, // byte 92:95.   M - Optional Asynchronous Events Supported (OAES)
    CTRATT: CTRATT, // byte 96:99.   M - Controller Attributes (CTRATT)
    RRLS: RRLS, // byte 100:101. O - Read Recovery Levels Supported (RRLS)
    Reserved0: [u8; 9], // byte 102:110.
    CNTRLTYPE: u8, // byte 111.     M - Controller Type
    FGUID: [u8; 16], // byte 112:127. O - FRU Globally Unique Identifier (FGUID)
    CRDT1: u16, // byte 128:129. O - Command Retry Delay Time 1
    CRDT2: u16, // byte 130:131. O - Command Retry Delay Time 1
    CRDT3: u16, // byte 132:133. O - Command Retry Delay Time 1
    Reserved0_1: [u8; 106], // byte 134:239.
    ReservedForManagement: [u8; 16], // byte 240:255.  Refer to the NVMe Management Interface Specification for definition.
    OACS: OACS,                      // byte 256:257. M - Optional Admin Command Support (OACS)
    ACL: u8,                         // byte 258.    M - Abort Command Limit (ACL)
    AERL: u8,                        // byte 259.    M - Asynchronous Event Request Limit (AERL)
    FRMW: FRMW,                      // byte 260.    M - Firmware Updates (FRMW)
    LPA: LPA,                        // byte 261.    M - Log Page Attributes (LPA)
    ELPE: u8,                        // byte 262.    M - Error Log Page Entries (ELPE)
    NPSS: u8,                        // byte 263.    M - Number of Power States Support (NPSS)
    AVSCC: AVSCC, // byte 264.    M - Admin Vendor Specific Command Configuration (AVSCC)
    APSTA: APSTA, // byte 265.     O - Autonomous Power State Transition Attributes (APSTA)
    WCTEMP: u16,  // byte 266:267. M - Warning Composite Temperature Threshold (WCTEMP)
    CCTEMP: u16,  // byte 268:269. M - Critical Composite Temperature Threshold (CCTEMP)
    MTFA: u16,    // byte 270:271. O - Maximum Time for Firmware Activation (MTFA)
    HMPRE: u32,   // byte 272:275. O - Host Memory Buffer Preferred Size (HMPRE)
    HMMIN: u32,   // byte 276:279. O - Host Memory Buffer Minimum Size (HMMIN)
    TNVMCAP: [u8; 16], // byte 280:295. O - Total NVM Capacity (TNVMCAP)
    UNVMCAP: [u8; 16], // byte 296:311. O - Unallocated NVM Capacity (UNVMCAP)
    RPMBS: RPMBS, // byte 312:315. O - Replay Protected Memory Block Support (RPMBS)
    EDSTT: u16,   // byte 316:317. O - Extended Device Self-test Time (EDSTT)
    DSTO: u8,     // byte 318.     O - Device Self-test Options (DSTO)
    FWUG: u8,     // byte 319.     M - Firmware Update Granularity (FWUG)
    KAS: u16,     // byte 320:321  M - Keep Alive Support (KAS)
    HCTMA: HCTMA, // byte 322:323  O - Host Controlled Thermal Management Attributes (HCTMA)
    MNTMT: u16,   // byte 324:325  O - Minimum Thermal Management Temperature (MNTMT)
    MXTMT: u16,   // byte 326:327  O - Maximum Thermal Management Temperature (MXTMT)
    SANICAP: SANICAP, // byte 328:331  O - Sanitize Capabilities (SANICAP)
    HMMINDS: u32, // byte 332:335  O - Host Memory Buffer Minimum Descriptor Entry Size (HMMINDS)
    HMMAXD: u16,  // byte 336:337  O - Host Memory Maxiumum Descriptors Entries (HMMAXD)
    NSETIDMAX: u16, // byte 338:339  O - NVM Set Identifier Maximum
    ENDGIDMAX: u16, // byte 340:341  O - Endurance Group Identifier Maximum (ENDGIDMAX)
    ANATT: u8,    // byte 342      O - ANA Transition Time (ANATT)
    ANACAP: ANACAP, // byte 343      O - Asymmetric Namespace Access Capabilities (ANACAP)
    ANAGRPMAX: u32, // byte 344:347  O - ANA Group Identifier Maximum (ANAGRPMAX)
    NANAGRPID: u32, // byte 348:351  O - Number of ANA Group Identifiers (NANAGRPID)
    PELS: u32,    // byte 352:355  O - Persistent Event Log Size (PELS)
    Reserved1: [u8; 156], // byte 356:511.
    SQES: SQES,   // byte 512.    M - Submission Queue Entry Size (SQES)
    CQES: CQES,   // byte 513.    M - Completion Queue Entry Size (CQES)
    MAXCMD: u16,  // byte 514:515. M - Maximum Outstanding Commands (MAXCMD)
    NN: u32,      // byte 516:519. M - Number of Namespaces (NN)
    ONCS: ONCS,   // byte 520:521. M - Optional NVM Command Support (ONCS)
    FUSES: FUSES, // byte 522:523. M - Fused Operation Support (FUSES)
    FNA: FNA,     // byte 524.     M - Format NVM Attributes (FNA)
    VWC: VWC,     // byte 525.     M - Volatile Write Cache (VWC)
    AWUN: u16,    // byte 526:527. M - Atomic Write Unit Normal (AWUN)
    AWUPF: u16,   // byte 528:529. M - Atomic Write Unit Power Fail (AWUPF)
    NVSCC: NVSCC, // byte 530.     M - NVM Vendor Specific Command Configuration (NVSCC)
    NWPC: NWPC,   // byte 531.     M - Namespace Write Protection Capabilities (NWPC)
    ACWU: u16,    // byte 532:533  O - Atomic Compare & Write Unit (ACWU)
    Reserved4: [u8; 2], // byte 534:535.
    SGLS: SGLS,   // byte 536:539. O - SGL Support (SGLS)
    MNAN: u32,    // byte 540:543. O - Maximum Number of Allowed Namespace (MNAN)
    Reserved6: [u8; 224], // byte 544:767.
    SUBNQN: [u8; 256], // byte 768:1023. M - NVM Subsystem NVMe Qualified Name (SUBNQN)
    Reserved7: [u8; 768], // byte 1024:1791
    Reserved8: [u8; 256], // byte 1792:2047. Refer to NVMe over Fabrics Specification
    PDS: [NVME_POWER_STATE_DESC; 32], // byte 2048:3071. M - Power State Descriptors
    VS: [u8; 1024], // byte 3072 : 4095.
}

#[derive(Debug, Clone, Copy)]
struct CMIC {
    MultiPCIePorts: u8,
    MultiControllers: u8,
    SRIOV: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct OAES {
    Reserved0: u32,
    NamespaceAttributeChanged: u32,
    FirmwareActivation: u32,
    Reserved1: u32,
    AsymmetricAccessChanged: u32,
    PredictableLatencyAggregateLogChanged: u32,
    LbaStatusChanged: u32,
    EnduranceGroupAggregateLogChanged: u32,
    Reserved2: u32,
    ZoneInformation: u32,
    Reserved3: u32,
}

#[derive(Debug, Clone, Copy)]
struct CTRATT {
    HostIdentifier128Bit: u32,
    NOPSPMode: u32,
    NVMSets: u32,
    ReadRecoveryLevels: u32,
    EnduranceGroups: u32,
    PredictableLatencyMode: u32,
    TBKAS: u32,
    NamespaceGranularity: u32,
    SQAssociations: u32,
    UUIDList: u32,
    Reserved0: u32,
}

#[derive(Debug, Clone, Copy)]
struct RRLS {
    ReadRecoveryLevel0: u16,
    ReadRecoveryLevel1: u16,
    ReadRecoveryLevel2: u16,
    ReadRecoveryLevel3: u16,
    ReadRecoveryLevel4: u16,
    ReadRecoveryLevel5: u16,
    ReadRecoveryLevel6: u16,
    ReadRecoveryLevel7: u16,
    ReadRecoveryLevel8: u16,
    ReadRecoveryLevel9: u16,
    ReadRecoveryLevel10: u16,
    ReadRecoveryLevel11: u16,
    ReadRecoveryLevel12: u16,
    ReadRecoveryLevel13: u16,
    ReadRecoveryLevel14: u16,
    ReadRecoveryLevel15: u16,
}

#[derive(Debug, Clone, Copy)]
struct OACS {
    SecurityCommands: u16,
    FormatNVM: u16,
    FirmwareCommands: u16,
    NamespaceCommands: u16,
    DeviceSelfTest: u16,
    Directives: u16,
    NVMeMICommands: u16,
    VirtualizationMgmt: u16,
    DoorBellBufferConfig: u16,
    GetLBAStatus: u16,
    Reserved: u16,
}

#[derive(Debug, Clone, Copy)]
struct FRMW {
    Slot1ReadOnly: u8,
    SlotCount: u8,
    ActivationWithoutReset: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct LPA {
    SmartPagePerNamespace: u8,
    CommandEffectsLog: u8,
    LogPageExtendedData: u8,
    TelemetrySupport: u8,
    PersistentEventLog: u8,
    Reserved0: u8,
    TelemetryDataArea4: u8,
    Reserved1: u8,
}

#[derive(Debug, Clone, Copy)]
struct AVSCC {
    CommandFormatInSpec: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct APSTA {
    Supported: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct HCTMA {
    Supported: u16,
    Reserved: u16,
}

#[derive(Debug, Clone, Copy)]
struct SANICAP {
    CryptoErase: u32,
    BlockErase: u32,
    Overwrite: u32,
    Reserved: u32,
    NDI: u32,
    NODMMAS: u32,
}

#[derive(Debug, Clone, Copy)]
struct ANACAP {
    OptimizedState: u8,
    NonOptimizedState: u8,
    InaccessibleState: u8,
    PersistentLossState: u8,
    ChangeState: u8,
    Reserved: u8,
    StaticANAGRPID: u8,
    SupportNonZeroANAGRPID: u8,
}

#[derive(Debug, Clone, Copy)]
struct SQES {
    RequiredEntrySize: u8,
    MaxEntrySize: u8,
}

#[derive(Debug, Clone, Copy)]
struct CQES {
    RequiredEntrySize: u8,
    MaxEntrySize: u8,
}

#[derive(Debug, Clone, Copy)]
struct ONCS {
    Compare: u16,
    WriteUncorrectable: u16,
    DatasetManagement: u16,
    WriteZeroes: u16,
    FeatureField: u16,
    Reservations: u16,
    Timestamp: u16,
    Verify: u16,
    Reserved: u16,
}

#[derive(Debug, Clone, Copy)]
struct FUSES {
    CompareAndWrite: u16,
    Reserved: u16,
}

#[derive(Debug, Clone, Copy)]
struct FNA {
    FormatApplyToAll: u8,
    SecureEraseApplyToAll: u8,
    CryptographicEraseSupported: u8,
    FormatSupportNSIDAllF: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct VWC {
    Present: u8,
    FlushBehavior: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct NVSCC {
    CommandFormatInSpec: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct NWPC {
    WriteProtect: u8,
    UntilPowerCycle: u8,
    Permanent: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct SGLS {
    SGLSupported: u32,
    KeyedSGLData: u32,
    Reserved0: u32,
    BitBucketDescrSupported: u32,
    ByteAlignedContiguousPhysicalBuffer: u32,
    SGLLengthLargerThanDataLength: u32,
    MPTRSGLDescriptor: u32,
    AddressFieldSGLDataBlock: u32,
    TransportSGLData: u32,
    Reserved1: u32,
}

//
// Namespace Identfier Type (NIDT)
//
enum NVME_IDENTIFIER_TYPE {
    NVME_IDENTIFIER_TYPE_EUI64 = 0x1,
    NVME_IDENTIFIER_TYPE_NGUID = 0x2,
    NVME_IDENTIFIER_TYPE_UUID = 0x3,
    NVME_IDENTIFIER_TYPE_CSI = 0x4,
}

//
// Namespace Identfier Length (NIDL) for a given type defined by NVME_IDENTIFIER_TYPE
//
enum NVME_IDENTIFIER_TYPE_LENGTH {
    NVME_IDENTIFIER_TYPE_EUI64_LENGTH = 0x8,
    NVME_IDENTIFIER_TYPE_NGUID_LENGTH = 0x10,
    NVME_IDENTIFIER_TYPE_UUID_LENGTH = 0x10,
    NVME_IDENTIFIER_TYPE_CSI_LENGTH = 0x1,
}

//
// Output of NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE (0x03)
//
const NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE_SIZE: usize = 0x1000;

#[derive(Debug, Clone, Copy)]
struct NVME_IDENTIFY_NAMESPACE_DESCRIPTOR {
    NIDT: u8, // Namespace Identifier Type as defined in NVME_IDENTIFIER_TYPE
    NIDL: u8, // Namespace Identifier Length
    Reserved: [u8; 2],
    NID: [u8; ANYSIZE_ARRAY], // Namespace Identifier (Based on NVME_IDENTIFIER_TYPE)
}

#[derive(Debug, Clone, Copy)]
struct NVME_SET_ATTRIBUTES_ENTRY {
    Identifier: u16,
    ENDGID: u16,
    Reserved1: u32,
    Random4KBReadTypical: u32,
    OptimalWriteSize: u32,
    TotalCapacity: [u8; 16],
    UnallocatedCapacity: [u8; 16],
    Reserved2: [u8; 80],
}

#[derive(Debug, Clone, Copy)]
struct NVM_SET_LIST {
    IdentifierCount: u8,
    Reserved: [u8; 127],
    Entry: [NVME_SET_ATTRIBUTES_ENTRY; ANYSIZE_ARRAY],
}

#[derive(Debug, Clone, Copy)]
struct NVME_LBA_ZONE_FORMAT {
    ZoneSize: u64, // bit 0:63 Zone Size (MS)
    ZDES: u8,      // bit 64:71 Zone Descriptor Extension Size (ZDES)
    Reserved: [u8; 7],
}
#[derive(Debug, Clone, Copy)]
struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
    ZOC: ZOC,
    OZCS: OZCS,
    MAR: u32,
    MOR: u32,
    RRL: u32,
    FRL: u32,
    Reserved0: [u8; 2796],
    LBAEF: [NVME_LBA_ZONE_FORMAT; 16],
    Reserved1: [u8; 768],
    VS: [u8; 256],
}

#[derive(Debug, Clone, Copy)]
struct ZOC {
    VariableZoneCapacity: u16,
    ZoneExcursions: u16,
    Reserved: u16,
}

#[derive(Debug, Clone, Copy)]
struct OZCS {
    ReadAcrossZoneBoundaries: u16,
    Reserved: u16,
}
//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET (0x06) with Command Set Identifier (0x00)
//
#[derive(Debug, Clone, Copy)]
struct NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    VSL: u8,              // byte 0       O - Verify Size Limit (VZL)
    WZSL: u8,             // byte 1       O - Write Zeroes Size Limit (WZSL)
    WUSL: u8,             // byte 2       O - Write Uncorrectable Size Limit (WUSL)
    DMRL: u8,             // byte 3       O - Dataset Management Ranges Limit (DMRL)
    DMRSL: u32,           // byte 4:7     O - Dataset Management Range Size Limit (DMRSL)
    DMSL: u64,            // byte 8:15    O - Dataset Management Size Limit (DMSL)
    Reserved: [u8; 4080], // byte 16:4095
}

//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET (0x06) with Command Set Identifier (0x02)
//
#[derive(Debug, Clone, Copy)]
struct NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    ZASL: u8,             // byte 0.          O - Zone Append Size Limit (ZASL)
    Reserved: [u8; 4095], // byte 1:4095
}

//
// Output of NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NSID (0x12)/NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NVM_SUBSYSTEM (0x13)
//
#[derive(Debug, Clone, Copy)]
struct NVME_CONTROLLER_LIST {
    NumberOfIdentifiers: u16,
    ControllerID: [u16; 2047],
}

//
// Output of NVME_IDENTIFY_CNS_IO_COMMAND_SET (0x1C)
//
#[derive(Debug, Clone, Copy)]
struct NVME_IDENTIFY_IO_COMMAND_SET {
    IOCommandSetVector: [u64; 512],
}
//
// Data Structure of LBA Range Type entry
//
enum NVME_LBA_RANGE_TYPES {
    NVME_LBA_RANGE_TYPE_RESERVED = 0,
    NVME_LBA_RANGE_TYPE_FILESYSTEM = 1,
    NVME_LBA_RANGE_TYPE_RAID = 2,
    NVME_LBA_RANGE_TYPE_CACHE = 3,
    NVME_LBA_RANGE_TYPE_PAGE_SWAP_FILE = 4,
}

#[derive(Debug, Clone, Copy)]
struct NVME_LBA_RANGE_TYPE_ENTRY {
    Type: u8, // Type (Type): Specifies the Type of the LBA range.
    Attributes: NVME_LBA_RANGE_TYPE_ATTRIBUTES, // Attributes: Specifies attributes of the LBA range. Each bit defines an attribute.
    Reserved0: [u8; 14],
    SLBA: u64, // Starting LBA (SLBA): This field specifies the 64-bit address of the first logical block that is part of this LBA range.
    NLB: u64, // Number of Logical Blocks (NLB): This field specifies the number of logical blocks that are part of this LBA range. This is a 0s based value.
    GUID: [u8; 16], // Unique Identifier (GUID): This field is a global unique identifier that uniquely specifies the type of this LBA range. Well known Types may be defined and are published on the NVM Express website.
    Reserved1: [u8; 16],
}

#[derive(Debug, Clone, Copy)]
struct NVME_LBA_RANGE_TYPE_ATTRIBUTES {
    MayOverwritten: u8,
    Hidden: u8,
    Reserved: u8,
}

//
// Vendor defined log pages
//
enum NVME_VENDOR_LOG_PAGES {
    NVME_LOG_PAGE_WCS_DEVICE_SMART_ATTRIBUTES = 0xC0, // WCS device SMART Attributes log page
    NVME_LOG_PAGE_WCS_DEVICE_ERROR_RECOVERY = 0xC1,   // WCS device Error Recovery log page
}

//
// SMART Attributes Log Page GUID is defined in spec as byte stream: 0xAFD514C97C6F4F9CA4F2BFEA2810AFC5
// which is converted to GUID format as: {2810AFC5-BFEA-A4F2-9C4F-6F7CC914D5AF}
//
const GUID_WCS_DEVICE_SMART_ATTRIBUTES: [u8; 16] = [
    0x28, 0x10, 0xAF, 0xC5, 0xBF, 0xEA, 0xA4, 0xF2, 0x9C, 0x4F, 0x6F, 0x7C, 0xC9, 0x14, 0xD5, 0xAF,
];

//
// Error Recovery Log Page GUID is defined in spec as byte stream: 0x5A1983BA3DFD4DABAE3430FE2131D944
// which is converted to GUID format as: {2131D944-30FE-AE34-AB4D-FD3DBA83195A}
//
const GUID_WCS_DEVICE_ERROR_RECOVERY: [u8; 16] = [
    0x21, 0x31, 0xD9, 0x44, 0x30, 0xFE, 0xAE, 0x34, 0xAB, 0x4D, 0xFD, 0x3D, 0xBA, 0x83, 0x19, 0x5A,
];

//
// Notice Status: NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC
//
enum NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_CODES {
    NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_RESERVED = 0,
    NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_DEVICE_PANIC = 1,
}
#[derive(Debug, Clone, Copy)]
struct NVME_WCS_DEVICE_RESET_ACTION {
    ControllerReset: bool,
    NVMeSubsystemReset: bool,
    PCIeFLR: bool,
    PERST: bool,
    PowerCycle: bool,
    PCIeConventionalHotReset: bool,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct NVME_WCS_DEVICE_CAPABILITIES {
    PanicAEN: bool,
    PanicCFS: bool,
    Reserved: u32,
}

#[derive(Debug, Clone, Copy)]
enum NVME_WCS_DEVICE_RECOVERY_ACTION {
    NVMeDeviceRecoveryNoAction = 0,          // Requires no action
    NVMeDeviceRecoveryFormatNVM,             // Requires Format NVM
    NVMeDeviceRecoveryVendorSpecificCommand, // Requires Vendor Specific Command
    NVMeDeviceRecoveryVendorAnalysis,        // Requires Vendor Analysis
    NVMeDeviceRecoveryDeviceReplacement,     // Requires Device Replacement
    NVMeDeviceRecoverySanitize,              // Requires Sanitize
    NVMeDeviceRecoveryMax = 15,              // Not an actual action, denotes max action.
}
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
    VersionSpecificData: [u8; 494],
    LogPageVersionNumber: u16,
    LogPageGUID: [u8; 16], // GUID_WCS_DEVICE_SMART_ATTRIBUTES
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
    MediaUnitsWritten: [u8; 16],
    MediaUnitsRead: [u8; 16],
    BadUserNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_Count,
    BadSystemNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_Count,
    XORRecoveryCount: u64,
    UnrecoverableReadErrorCount: u64,
    SoftECCErrorCount: u64,
    EndToEndCorrectionCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_CorrectionCounts,
    PercentageSystemDataUsed: u8,
    RefreshCount: [u8; 7],
    UserDataEraseCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_EraseCounts,
    ThermalThrottling: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_ThermalThrottling,
    Reserved0: [u8; 6],
    PCIeCorrectableErrorCount: u64,
    IncompleteShutdownCount: u32,
    Reserved1: u32,
    PercentageFreeBlocks: u8,
    Reserved2: [u8; 7],
    CapacitorHealth: u16,
    Reserved3: [u8; 6],
    UnalignedIOCount: u64,
    SecurityVersionNumber: u64,
    NUSE: u64,
    PLPStartCount: [u8; 16],
    EnduranceEstimate: [u8; 16],
    Reserved4: [u8; 302],
    LogPageVersionNumber: u16,
    LogPageGUID: [u8; 16], // GUID_WCS_DEVICE_SMART_ATTRIBUTES
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_Count {
    RawCount: [u8; 6],
    Normalized: [u8; 2],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_CorrectionCounts {
    DetectedCounts: u32,
    CorrectedCounts: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_EraseCounts {
    MaximumCount: u32,
    MinimumCount: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_ThermalThrottling {
    EventCount: u8,
    Status: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
    PanicResetWaitTime: u16,
    PanicResetAction: NVME_WCS_DEVICE_RESET_ACTION,
    DriveRecoveryAction: u8,
    PanicId: u64,
    DeviceCapabilities: NVME_WCS_DEVICE_CAPABILITIES,
    VendorSpecificRecoveryCode: u8,
    Reserved0: [u8; 3],
    VendorSpecificCommandCDW12: u32,
    VendorSpecificCommandCDW13: u32,
    Reserved1: [u8; 466],
    LogPageVersionNumber: u16,
    LogPageGUID: [u8; 16], // GUID_WCS_DEVICE_ERROR_RECOVERY
}
//
// Parameters for NVME_ADMIN_COMMAND_CREATE_IO_CQ
//
#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_CREATE_IO_QUEUE {
    QID: u16,   // Queue Identifier (QID)
    QSIZE: u16, // Queue Size (QSIZE)
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_CREATE_IO_CQ {
    PC: bool,       // Physically Contiguous (PC)
    IEN: bool,      // Interrupts Enabled (IEN)
    Reserved0: u16, // Reserved
    IV: u16,        // Interrupt Vector (IV)
}

//
// Parameters for NVME_ADMIN_COMMAND_CREATE_IO_SQ
//
#[derive(Debug, Clone, Copy)]
enum NVME_NVM_QUEUE_PRIORITIES {
    NVME_NVM_QUEUE_PRIORITY_URGENT = 0,
    NVME_NVM_QUEUE_PRIORITY_HIGH = 1,
    NVME_NVM_QUEUE_PRIORITY_MEDIUM = 2,
    NVME_NVM_QUEUE_PRIORITY_LOW = 3,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_CREATE_IO_SQ {
    PC: bool,       // Physically Contiguous (PC)
    QPRIO: u8,      // Queue Priority (QPRIO)
    Reserved0: u16, // Reserved
    CQID: u16,      // Completion Queue Identifier (CQID)
}
#[derive(Debug, Clone, Copy)]
enum NVME_FEATURE_VALUE_CODES {
    NVME_FEATURE_VALUE_CURRENT = 0,
    NVME_FEATURE_VALUE_DEFAULT = 1,
    NVME_FEATURE_VALUE_SAVED = 2,
    NVME_FEATURE_VALUE_SUPPORTED_CAPABILITIES = 3,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_GET_FEATURES {
    FID: u8, // Feature Identifier (FID)
    SEL: u8, // Select (SEL): This field specifies which value of the attributes to return in the provided data.
    Reserved0: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_SET_FEATURES {
    FID: u8, // Feature Identifier (FID)
    Reserved0: u32,
    SV: u8, // Save (SV)
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_NUMBER_OF_QUEUES {
    NSQ: u16, // Number of IO Submission Queues.
    NCQ: u16, // Number of IO Completion Queues.
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_INTERRUPT_COALESCING {
    THR: u8,  // Aggregation Threshold (THR)
    TIME: u8, // Aggregation Time (TIME)
    Reserved0: u16,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG {
    IV: u16, // Interrupt Vector (IV)
    CD: u8,  // Coalescing Disabled (CD)
    Reserved0: u16,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL {
    DN: u8, // Disable Normal (DN)
    Reserved0: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE {
    NOPPME: u8, // Non-Operational Power State Permissive Mode Enable (NOPPME)
    Reserved0: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_LBA_RANGE_TYPE {
    NUM: u8, // Number of LBA Ranges (NUM)
    Reserved0: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_ARBITRATION {
    AB: u8, // Arbitration Burst (AB)
    Reserved0: u8,
    LPW: u8, // Low Priority Weight (LPW)
    MPW: u8, // Medium Priority Weight (MPW)
    HPW: u8, // High Priority Weight (HPW)
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE {
    WCE: u8, // Volatile Write Cache Enable (WCE)
    Reserved0: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY {
    SAVE: u8, // Save supported
    NSS: u8,  // Namespace specific
    MOD: u8,  // Changeable
    Reserved0: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG {
    CriticalWarnings: u8,            // SMART / Health Critical Warnings
    NsAttributeNotices: u8,          // Namespace Attributes Notices
    FwActivationNotices: u8,         // Firmware Activation Notices
    TelemetryLogNotices: u8,         // Telemetry Log Notices
    ANAChangeNotices: u8,            // Asymmetric Namespace Access Change Notices
    PredictableLogChangeNotices: u8, // Predictable Latency Event Aggregate Log Change Notices
    LBAStatusNotices: u8,            // LBA Status Information Notices
    EnduranceEventNotices: u8,       // Endurance Group Event Aggregate Log Change Notices
    Reserved0: u16,
    ZoneDescriptorNotices: u8, // Zone Descriptor Changed Notices
    Reserved1: u8,
}

//
// Parameter for NVME_FEATURE_POWER_MANAGEMENT
//
#[derive(Debug, Clone, Copy)]
union NVME_CDW11_FEATURE_POWER_MANAGEMENT {
    DUMMYSTRUCTNAME: NVME_CDW11_FEATURE_POWER_MANAGEMENT_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_POWER_MANAGEMENT_STRUCT {
    PS: u32,        // Power State (PS)
    Reserved0: u32, // Reserved
}

// Parameter for NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION
//
#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION {
    APSTE: u32, // Autonomous Power State Transition Enable (APSTE)
    Reserved0: u32,
}
//
// Parameter for NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION
// There is an array of 32 of these (one for each power state) in the data buffer.
//
#[derive(Debug, Clone, Copy)]
struct NVME_AUTO_POWER_STATE_TRANSITION_ENTRY {
    Reserved0: u8,                  // Bits 0-2 are reserved.
    IdleTransitionPowerState: u8, // Bits 3-7 - (ITPS) The non-operational power state for the controller to autonomously transition to after there is a continuous period of idle time in the current power state that exceeds time specified in the ITPT field.
    IdleTimePriorToTransition: u32, // Bits 8-31 - (ITPT) The amount of idle time (in ms) that occurs in this power state prior to transitioning to the Idle Transition Power State. A value of 0 disables APST for this power state.
    Reserved1: u32,                 // Bits 32-63 are reserved.
}

//
// Parameter for NVME_FEATURE_TEMPERATURE_THRESHOLD
//

//
// Following definitions are used in "THSEL" field.
//
#[derive(Debug, Clone, Copy)]
enum NVME_TEMPERATURE_THRESHOLD_TYPES {
    NVME_TEMPERATURE_OVER_THRESHOLD = 0,
    NVME_TEMPERATURE_UNDER_THRESHOLD = 1,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD {
    TMPTH: u16, // Temperature Threshold (TMPTH): Indicates the threshold for the temperature of the overall device (controller and NVM included) in units of Kelvin.
    TMPSEL: u8, // Threshold Temperature Select (TMPSEL)
    THSEL: u8,  // Threshold Type Select (THSEL)
    Reserved0: u16, // Reserved
}

//
// Parameter for NVME_FEATURE_ERROR_RECOVERY
//
#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_ERROR_RECOVERY {
    TLER: u16,      // Time limited error recovery (TLER)
    DULBE: bool,    // Deallocated or unwritten logical block error enable (DULBE)
    Reserved0: u16, // Reserved
}
// Parameters for NVME_FEATURE_HOST_MEMORY_BUFFER
//
#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER {
    EHM: u32, // Enable Host Memory (EHM) - Enables the host memory buffer.
    MR: u32, // Memory Return (MR) - Indicates if the host is returning previously allocated memory to the controller.
    Reserved: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER {
    HSIZE: u32, // Host Memory Buffer Size (HSIZE) - The size of the host memory buffer in memory page size (CC.MPS) units.
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER {
    Reserved: u32,
    HMDLLA: u32, // Host Memory Descriptor List Lower Address (HMDLLA) - 16-byte aligned, lower 32 bits of the physical location of the Host Memory Descriptor List.
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER {
    HMDLUA: u32, // Host Memory Descriptor List Upper Address (HMDLLA) - Upper 32 bits of the physical location of the Host Memory Descriptor List.
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER {
    HMDLEC: u32, // Host Memory Descriptor List Entry Count (HMDLEC) - Number of entries in the Host Memory Descriptor List.
}

//
// This structure is a single entry in the host memory descriptor list.
//
#[derive(Debug, Clone, Copy)]
struct NVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY {
    BADD: u64, // Buffer Address (BADD) - Physical host memory address aligned to the memory page size (CC.MPS)
    BSIZE: u32, // Buffer Size (BSIZE) - The number of contiguous memory page size (CC.MPS) units for this entry.
    Reserved: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE {
    DUMMYSTRUCTNAME: NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_STRUCT {
    IOCSCI: u32, // I/O command Set Profile
    Reserved: u32,
}
// Parameters for NVME_FEATURE_ENHANDED_CONTROLLER_METADATA, NVME_FEATURE_CONTROLLER_METADATA, NVME_FEATURE_NAMESPACE_METADATA
//
#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_GET_HOST_METADATA {
    GDHM: u32, // Generate Default Host Metadata (GDHM)
}

#[derive(Debug, Clone, Copy)]
enum NVME_HOST_METADATA_ELEMENT_ACTIONS {
    NVME_HOST_METADATA_ADD_REPLACE_ENTRY = 0,
    NVME_HOST_METADATA_DELETE_ENTRY_MULTIPLE = 1,
    NVME_HOST_METADATA_ADD_ENTRY_MULTIPLE = 2,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_SET_HOST_METADATA {
    Reserved0: u32,
    EA: u32, // Element Action (EA), value defined in enum NVME_HOST_METADATA_ELEMENT_ACTIONS
    Reserved1: u32,
}

#[derive(Debug, Clone, Copy)]
enum NVME_CONTROLLER_METADATA_ELEMENT_TYPES {
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_CONTROLLER_NAME = 0x1,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_NAME = 0x2,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_VERSION = 0x3,
    NVME_CONTROLLER_METADATA_PREBOOT_CONTROLLER_NAME = 0x4,
    NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_NAME = 0x5,
    NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_VERSION = 0x6,
    NVME_CONTROLLER_METADATA_SYSTEM_PROCESSOR_MODEL = 0x7,
    NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_NAME = 0x8,
    NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_VERSION = 0x9,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_NAME_AND_BUILD = 0xA,
    NVME_CONTROLLER_METADATA_SYSTEM_PRODUCT_NAME = 0xB,
    NVME_CONTROLLER_METADATA_FIRMWARE_VERSION = 0xC,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_FILENAME = 0xD,
    NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_NAME = 0xE,
    NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_VERSION = 0xF,
    NVME_CONTROLLER_METADATA_HOST_DETERMINED_FAILURE_RECORD = 0x10,
}

#[derive(Debug, Clone, Copy)]
enum NVME_NAMESPACE_METADATA_ELEMENT_TYPES {
    NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME = 0x1,
    NVME_NAMESPACE_METADATA_PREBOOT_NAMESPACE_NAME = 0x2,
    NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_1 = 0x3,
    NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_2 = 0x4,
}

#[derive(Debug, Clone, Copy)]
struct NVME_HOST_METADATA_ELEMENT_DESCRIPTOR {
    ET: u32, // Element Type (ET), value defined in enum NVME_CONTROLLER_METADATA_ELEMENT_TYPES, NVME_NAMESPACE_METADATA_ELEMENT_TYPES
    Reserved0: u32,
    ER: u32, // Element Revision (ER)
    Reserved1: u32,
    ELEN: u32,     // Element Length (ELEN), element value length in bytes
    EVAL: [u8; 0], // Element Value (EVAL), UTF-8 string
}

#[derive(Debug, Clone, Copy)]
struct NVME_FEATURE_HOST_METADATA_DATA {
    NumberOfMetadataElementDescriptors: u8,
    Reserved0: u8,
    MetadataElementDescriptors: [u8; 4094], // Use NVME_HOST_METADATA_ELEMENT_DESCRIPTOR to access this list.
}

//
// Parameter for NVME_FEATURE_ERROR_INJECTION
// This is from OCP NVMe Cloud SSD spec.
//
#[derive(Debug, Clone, Copy)]
union NVME_CDW11_FEATURE_ERROR_INJECTION {
    DUMMYSTRUCTNAME: NVME_CDW11_FEATURE_ERROR_INJECTION_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_ERROR_INJECTION_STRUCT {
    NUM: u32,       // Number of Error Injections.
    Reserved0: u32, // Reserved
}

//
// DWORD 0 for get feature command (Error Injection) shares the same format with DWORD 11 for set feature command (Error Injection).
//
#[derive(Debug, Clone, Copy)]
struct NVME_ERROR_INJECTION_ENTRY {
    Flags: NVME_ERROR_INJECTION_FLAGS,
    Reserved1: u8,
    ErrorInjectionType: u16,
    ErrorInjectionTypeSpecific: [u8; 28],
}

#[derive(Debug, Clone, Copy)]
union NVME_ERROR_INJECTION_FLAGS {
    DUMMYSTRUCTNAME: NVME_ERROR_INJECTION_FLAGS_STRUCT,
    AsUchar: u8,
}

#[derive(Debug, Clone, Copy)]
struct NVME_ERROR_INJECTION_FLAGS_STRUCT {
    Enable: u8,
    SingleInstance: u8,
    Reserved0: u8,
}

//
// Definitions are used in "Error Injection Type" field.
//
#[derive(Debug, Clone, Copy)]
enum NVME_ERROR_INJECTION_TYPES {
    NVME_ERROR_INJECTION_TYPE_RESERVED0 = 0,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_CPU_CONTROLLER_HANG = 1,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_HANG = 2,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_PLP_DEFECT = 3,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_LOGICAL_FW_ERROR = 4,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_CRITICAL = 5,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_NONCRITICAL = 6,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_CORRUPTION = 7,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_SRAM_CORRUPTION = 8,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_HW_MALFUNCTION = 9,
    NVME_ERROR_INJECTION_TYPE_RESERVED1 = 10,
    NVME_ERROR_INJECTION_TYPE_MAX = 0xFFFF,
}
// Parameter for set feature NVME_FEATURE_CLEAR_FW_UPDATE_HISTORY
// This is from OCP NVMe Cloud SSD spec.
//
#[derive(Debug, Clone, Copy)]
union NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY {
    DUMMYSTRUCTNAME: NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_STRUCT {
    Reserved0: u32,
    Clear: u32, // Clear Firmware Update History Log.
}

// Parameter for set feature NVME_FEATURE_READONLY_WRITETHROUGH_MODE
// This is from OCP NVMe Cloud SSD spec.
//
#[derive(Debug, Clone, Copy)]
union NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE {
    DUMMYSTRUCTNAME: NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_STRUCT {
    Reserved0: u32,
    EOLBehavior: u32, // End of Life Behavior.
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE {
    DUMMYSTRUCTNAME: NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_STRUCT {
    EOLBehavior: u32, // End of Life Behavior.
    Reserved0: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS {
    DUMMYSTRUCTNAME: NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_STRUCT {
    Reserved0: u32,
    Clear: u32, // Clear PCIe Error Counters.
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO {
    DUMMYSTRUCTNAME: NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_STRUCT {
    Reserved0: u32,
    Enable: u32, // Enable IEEE1667 Silo.
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO {
    DUMMYSTRUCTNAME: NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_STRUCT {
    Enabled: u32, // IEEE1667 Silo Enabled.
    Reserved0: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_HOST_IDENTIFIER {
    EXHID: u32, // Enable Extended Host Identifier (EXHID)
    Reserved: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_FEATURE_HOST_IDENTIFIER_DATA {
    HOSTID: [u8; 16], // Host Identifier (HOSTID)
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE {
    PTPL: u32, // Persist Through Power Loss (PTPL)
    Reserved: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK {
    Reserved: u32,
    REGPRE: u32, // Mask Registration Preempted Notification (REGPRE)
    RESREL: u32, // Mask Reservation Released Notification (RESREL)
    RESPRE: u32, // Mast Reservation Preempted Notification (RESPRE)
    Reserved1: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW11_FEATURES {
    NumberOfQueues: NVME_CDW11_FEATURE_NUMBER_OF_QUEUES,
    InterruptCoalescing: NVME_CDW11_FEATURE_INTERRUPT_COALESCING,
    InterruptVectorConfig: NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG,
    LbaRangeType: NVME_CDW11_FEATURE_LBA_RANGE_TYPE,
    Arbitration: NVME_CDW11_FEATURE_ARBITRATION,
    VolatileWriteCache: NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE,
    AsyncEventConfig: NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG,
    PowerManagement: NVME_CDW11_FEATURE_POWER_MANAGEMENT,
    AutoPowerStateTransition: NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION,
    TemperatureThreshold: NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD,
    ErrorRecovery: NVME_CDW11_FEATURE_ERROR_RECOVERY,
    HostMemoryBuffer: NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER,
    WriteAtomicityNormal: NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL,
    NonOperationalPowerState: NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE,
    IoCommandSetProfile: NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE,
    ErrorInjection: NVME_CDW11_FEATURE_ERROR_INJECTION,
    HostIdentifier: NVME_CDW11_FEATURE_HOST_IDENTIFIER,
    ReservationPersistence: NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE,
    ReservationNotificationMask: NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK,
    GetHostMetadata: NVME_CDW11_FEATURE_GET_HOST_METADATA,
    SetHostMetadata: NVME_CDW11_FEATURE_SET_HOST_METADATA,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW12_FEATURES {
    HostMemoryBuffer: NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW13_FEATURES {
    HostMemoryBuffer: NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW14_FEATURES {
    HostMemoryBuffer: NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW15_FEATURES {
    HostMemoryBuffer: NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER,
    AsUlong: u32,
}

//
// NVMe Maximum log size
//
const NVME_MAX_LOG_SIZE: usize = 0x1000;

//
// Parameters for NVME_ADMIN_COMMAND_GET_LOG_PAGE Command
//
#[derive(Debug, Clone, Copy)]
enum NVME_LOG_PAGES {
    NVME_LOG_PAGE_ERROR_INFO = 0x01,
    NVME_LOG_PAGE_HEALTH_INFO = 0x02,
    NVME_LOG_PAGE_FIRMWARE_SLOT_INFO = 0x03,
    NVME_LOG_PAGE_CHANGED_NAMESPACE_LIST = 0x04,
    NVME_LOG_PAGE_COMMAND_EFFECTS = 0x05,
    NVME_LOG_PAGE_DEVICE_SELF_TEST = 0x06,
    NVME_LOG_PAGE_TELEMETRY_HOST_INITIATED = 0x07,
    NVME_LOG_PAGE_TELEMETRY_CTLR_INITIATED = 0x08,
    NVME_LOG_PAGE_ENDURANCE_GROUP_INFORMATION = 0x09,
    NVME_LOG_PAGE_PREDICTABLE_LATENCY_NVM_SET = 0x0A,
    NVME_LOG_PAGE_PREDICTABLE_LATENCY_EVENT_AGGREGATE = 0x0B,
    NVME_LOG_PAGE_ASYMMETRIC_NAMESPACE_ACCESS = 0x0C,
    NVME_LOG_PAGE_PERSISTENT_EVENT_LOG = 0x0D,
    NVME_LOG_PAGE_LBA_STATUS_INFORMATION = 0x0E,
    NVME_LOG_PAGE_ENDURANCE_GROUP_EVENT_AGGREGATE = 0x0F,
    NVME_LOG_PAGE_RESERVATION_NOTIFICATION = 0x80,
    NVME_LOG_PAGE_SANITIZE_STATUS = 0x81,
    NVME_LOG_PAGE_CHANGED_ZONE_LIST = 0xBF,
}

//
// Get LOG PAGE format which confines to  < 1.3 NVMe Specification
//
#[derive(Debug, Clone, Copy)]
union NVME_CDW10_GET_LOG_PAGE {
    bits: u32,
    fields: NVME_CDW10_GET_LOG_PAGE_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_GET_LOG_PAGE_FIELDS {
    LID: u8, // Log Page Identifier (LID)
    Reserved0: u8,
    NUMD: u16, // Number of Dwords (NUMD)
    Reserved1: u8,
}

//
// Get LOG PAGE format which confines to  >= 1.3 NVMe Specification
//
#[derive(Debug, Clone, Copy)]
union NVME_CDW10_GET_LOG_PAGE_V13 {
    bits: u32,
    fields: NVME_CDW10_GET_LOG_PAGE_V13_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_GET_LOG_PAGE_V13_FIELDS {
    LID: u8, // Log Page Identifier (LID)
    LSP: u8, // Log Specific Field (LSP)
    Reserved0: u8,
    RAE: u8,    // Retain Asynchronous Event (RAE)
    NUMDL: u16, // Number of Lower Dwords (NUMDL)
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW11_GET_LOG_PAGE {
    bits: u32,
    fields: NVME_CDW11_GET_LOG_PAGE_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_GET_LOG_PAGE_FIELDS {
    NUMDU: u16,                 // Number of Upper Dwords (NUMDU)
    LogSpecificIdentifier: u16, // Log Specific Identifier
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW12_GET_LOG_PAGE {
    LPOL: u32, // Log Page Offset Lower (LPOL)
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW13_GET_LOG_PAGE {
    LPOU: u32, // Log Page Offset Upper (LPOU)
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW14_GET_LOG_PAGE {
    bits: u32,
    fields: NVME_CDW14_GET_LOG_PAGE_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW14_GET_LOG_PAGE_FIELDS {
    UUIDIndex: u8, // UUID Index
    Reserved: u8,
    CommandSetIdentifier: u8, // Command Set Identifier
}

#[derive(Debug, Clone, Copy)]
struct NVME_PARAMETER_ERROR_LOCATION {
    Byte: u8, // Byte in command that contained the error.
    Bit: u8,  // Bit in command that contained the error.
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct NVME_ERROR_INFO_LOG {
    ErrorCount: u64,
    SQID: u16,                   // Submission Queue ID
    CMDID: u16,                  // Command ID
    Status: NVME_COMMAND_STATUS, // Status Field: This field indicates the Status Field for the command  that completed.  The Status Field is located in bits 15:01, bit 00 corresponds to the Phase Tag posted for the command.
    ParameterErrorLocation: NVME_PARAMETER_ERROR_LOCATION,
    Lba: u64, // LBA: This field indicates the first LBA that experienced the error condition, if applicable.
    NameSpace: u32, // Namespace: This field indicates the namespace that the error is associated with, if applicable.
    VendorInfoAvailable: u8, // Vendor Specific Information Available
    Reserved0: [u8; 3],
    CommandSpecificInfo: u64, // This field contains command specific information. If used, the command definition specifies the information returned.
    Reserved1: [u8; 24],
}

#[derive(Debug, Clone, Copy)]
struct NVME_HEALTH_INFO_LOG {
    CriticalWarning: NVME_HEALTH_INFO_LOG_CRITICAL_WARNING, // Critical Warning
    Temperature: [u8; 2],                                   // Temperature
    AvailableSpare: u8,                                     // Available Spare
    AvailableSpareThreshold: u8,                            // Available Spare Threshold
    PercentageUsed: u8,                                     // Percentage Used
    Reserved0: [u8; 26],
    DataUnitRead: [u8; 16],                // Data Units Read
    DataUnitWritten: [u8; 16],             // Data Units Written
    HostReadCommands: [u8; 16],            // Host Read Commands
    HostWrittenCommands: [u8; 16],         // Host Write Commands
    ControllerBusyTime: [u8; 16],          // Controller Busy Time
    PowerCycle: [u8; 16],                  // Power Cycles
    PowerOnHours: [u8; 16],                // Power On Hours
    UnsafeShutdowns: [u8; 16],             // Unsafe Shutdowns
    MediaErrors: [u8; 16],                 // Media Errors
    ErrorInfoLogEntryCount: [u8; 16],      // Number of Error Information Log Entries
    WarningCompositeTemperatureTime: u32,  // Warning Composite Temperature Time
    CriticalCompositeTemperatureTime: u32, // Critical Composite Temperature Time
    TemperatureSensor1: u16,               // Temperature Sensor 1
    TemperatureSensor2: u16,               // Temperature Sensor 2
    TemperatureSensor3: u16,               // Temperature Sensor 3
    TemperatureSensor4: u16,               // Temperature Sensor 4
    TemperatureSensor5: u16,               // Temperature Sensor 5
    TemperatureSensor6: u16,               // Temperature Sensor 6
    TemperatureSensor7: u16,               // Temperature Sensor 7
    TemperatureSensor8: u16,               // Temperature Sensor 8
    Reserved1: [u8; 296],
}

#[derive(Debug, Clone, Copy)]
union NVME_HEALTH_INFO_LOG_CRITICAL_WARNING {
    bits: u8,
    fields: NVME_HEALTH_INFO_LOG_CRITICAL_WARNING_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_HEALTH_INFO_LOG_CRITICAL_WARNING_FIELDS {
    AvailableSpaceLow: u8,                // Available Space Low
    TemperatureThreshold: u8,             // Temperature Threshold
    ReliabilityDegraded: u8,              // Reliability Degraded
    ReadOnly: u8,                         // Read Only
    VolatileMemoryBackupDeviceFailed: u8, // Volatile Memory Backup Device Failed
    Reserved: u8,                         // Reserved
}

//
// "Telemetry Host-Initiated Log" structure definition.
//
const NVME_TELEMETRY_DATA_BLOCK_SIZE: usize = 0x200; // All NVMe Telemetry Data Blocks are 512 bytes in size.

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_TELEMETRY_HOST_INITIATED_LOG {
    LogIdentifier: u8,                           // Byte 0
    Reserved0: [u8; 4],                          // Bytes 1-4
    OrganizationID: [u8; 3],                     // Bytes 5-7 - IEEE OUI Identifier
    Area1LastBlock: u16,                         // Bytes 8-9
    Area2LastBlock: u16,                         // Bytes 10-11
    Area3LastBlock: u16,                         // Bytes 12-13
    Reserved1: [u8; 2],                          // Bytes 14-15
    Area4LastBlock: u32,                         // Bytes 16-19
    Reserved2: [u8; 361],                        // Bytes 20-380
    HostInitiatedDataGenerationNumber: u8,       // Byte 381
    ControllerInitiatedDataAvailable: u8,        // Byte 382
    ControllerInitiatedDataGenerationNumber: u8, // Byte 383
    ReasonIdentifier: [u8; 128],                 // Bytes 384-511
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
    LogIdentifier: u8,                           // Byte 0
    Reserved0: [u8; 4],                          // Bytes 1-4
    OrganizationID: [u8; 3],                     // Bytes 5-7 - IEEE OUI Identifier
    Area1LastBlock: u16,                         // Bytes 8-9
    Area2LastBlock: u16,                         // Bytes 10-11
    Area3LastBlock: u16,                         // Bytes 12-13
    Reserved1: [u8; 2],                          // Bytes 14-15
    Area4LastBlock: u32,                         // Bytes 16-19
    Reserved2: [u8; 362],                        // Bytes 20-381
    ControllerInitiatedDataAvailable: u8,        // Byte 382
    ControllerInitiatedDataGenerationNumber: u8, // Byte 383
    ReasonIdentifier: [u8; 128],                 // Bytes 384-511
}

//
// Information of log: NVME_LOG_PAGE_FIRMWARE_SLOT_INFO. Size: 512 bytes
//
#[derive(Debug, Clone, Copy)]
struct NVME_FIRMWARE_SLOT_INFO_LOG_AFI {
    ActiveSlot: u8, // Bits 2:0 indicates the firmware slot that contains the actively running firmware revision.
    Reserved0: u8,
    PendingActivateSlot: u8, // Bits 6:4 indicates the firmware slot that is going to be activated at the next controller reset.
    Reserved1: u8,
}

#[derive(Debug, Clone, Copy)]
struct NVME_FIRMWARE_SLOT_INFO_LOG {
    AFI: NVME_FIRMWARE_SLOT_INFO_LOG_AFI, // Active Firmware Info (AFI)
    Reserved0: [u8; 7],
    FRS: [u64; 7], // Firmware Revision for Slot 1 - 7(FRS1 - FRS7):  Contains the revision of the firmware downloaded to firmware slot 1 - 7.
    Reserved1: [u8; 448],
}

#[derive(Debug, Clone, Copy)]
struct NVME_CHANGED_NAMESPACE_LIST_LOG {
    NSID: [u32; 1024], // List of Namespace ID up to 1024 entries
}

#[derive(Debug, Clone, Copy)]
struct NVME_CHANGED_ZONE_LIST_LOG {
    ZoneIdentifiersCount: u16, // Number of Zone Identifiers
    Reserved: [u8; 6],
    ZoneIdentifier: [u64; 511], // List of Zone Identifiers up to 511 entries. Identifier contains Zone Start Logical Block Address(ZSLBA)
}

//
// Information of log: NVME_LOG_PAGE_COMMAND_EFFECTS. Size: 4096 bytes
#[derive(Debug, Clone, Copy)]
enum NVME_COMMAND_EFFECT_SUBMISSION_EXECUTION_LIMITS {
    NVME_COMMAND_EFFECT_SUBMISSION_EXECUTION_LIMIT_NONE = 0,
    NVME_COMMAND_EFFECT_SUBMISSION_EXECUTION_LIMIT_SINGLE_PER_NAMESPACE = 1,
    NVME_COMMAND_EFFECT_SUBMISSION_EXECUTION_LIMIT_SINGLE_PER_CONTROLLER = 2,
}

#[derive(Debug, Clone, Copy)]
union NVME_COMMAND_EFFECTS_DATA {
    bits: u32,
    fields: NVME_COMMAND_EFFECTS_DATA_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_COMMAND_EFFECTS_DATA_FIELDS {
    CSUPP: u32,     // Command Supported (CSUPP)
    LBCC: u32,      // Logical Block Content Change (LBCC)
    NCC: u32,       // Namespace Capability Change (NCC)
    NIC: u32,       // Namespace Inventory Change (NIC)
    CCC: u32,       // Controller Capability Change (CCC)
    Reserved0: u32, // Reserved
    CSE: u32,       // Command Submission and Execution (CSE)
    Reserved1: u32, // Reserved
}

#[derive(Debug, Clone, Copy)]
struct NVME_COMMAND_EFFECTS_LOG {
    ACS: [NVME_COMMAND_EFFECTS_DATA; 256], // Admin Command Supported
    IOCS: [NVME_COMMAND_EFFECTS_DATA; 256], // I/O Command Supported
    Reserved: [u8; 2048],
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_DEVICE_SELF_TEST_RESULT_DATA {
    Status: NVME_DEVICE_SELF_TEST_RESULT_DATA_Status,
    SegmentNumber: u8,
    ValidDiagnostics: NVME_DEVICE_SELF_TEST_RESULT_DATA_ValidDiagnostics,
    Reserved: u8,
    POH: u64,
    NSID: u32,
    FailingLBA: u64,
    StatusCodeType: NVME_DEVICE_SELF_TEST_RESULT_DATA_StatusCodeType,
    StatusCode: u8,
    VendorSpecific: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_DEVICE_SELF_TEST_RESULT_DATA_Status {
    Result: u8,
    CodeValue: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_DEVICE_SELF_TEST_RESULT_DATA_ValidDiagnostics {
    NSIDValid: u8,
    FLBAValid: u8,
    SCTValid: u8,
    SCValid: u8,
    Reserved: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_DEVICE_SELF_TEST_RESULT_DATA_StatusCodeType {
    AdditionalInfo: u8,
    Reserved: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_DEVICE_SELF_TEST_LOG {
    CurrentOperation: NVME_DEVICE_SELF_TEST_LOG_CurrentOperation,
    CurrentCompletion: NVME_DEVICE_SELF_TEST_LOG_CurrentCompletion,
    Reserved: [u8; 2],
    ResultData: [NVME_DEVICE_SELF_TEST_RESULT_DATA; 20],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_DEVICE_SELF_TEST_LOG_CurrentOperation {
    Status: u8,
    Reserved: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_DEVICE_SELF_TEST_LOG_CurrentCompletion {
    CompletePercent: u8,
    Reserved: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_ENDURANCE_GROUP_LOG {
    Reserved0: u32,
    AvailableSpareThreshold: u8,
    PercentageUsed: u8,
    Reserved1: [u8; 26],
    EnduranceEstimate: [u8; 16],
    DataUnitsRead: [u8; 16],
    DataUnitsWritten: [u8; 16],
    MediaUnitsWritten: [u8; 16],
    Reserved2: [u8; 416],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_PERSISTENT_EVENT_LOG_HEADER {
    LogIdentifier: u8,
    Reserved0: [u8; 3],
    TotalNumberOfEvents: u32,
    TotalLogLength: u64,
    LogRevision: u8,
    Reserved1: u8,
    LogHeaderLength: u16,
    Timestamp: u64,
    PowerOnHours: [u8; 16],
    PowerCycleCount: u64,
    PciVendorId: u16,
    PciSubsystemVendorId: u16,
    SerialNumber: [u8; 20],
    ModelNumber: [u8; 40],
    NVMSubsystemNVMeQualifiedName: [u8; 256],
    Reserved: [u8; 108],
    SupportedEventsBitmap: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER {
    EventType: u8,
    EventTypeRevision: u8,
    EventHeaderLength: u8,
    Reserved0: u8,
    ControllerIdentifier: u16,
    EventTimestamp: u64,
    Reserved1: [u8; 6],
    VendorSpecificInformationLength: u16,
    EventLength: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
enum NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES {
    NVME_PERSISTENT_EVENT_TYPE_RESERVED0 = 0x00,
    NVME_PERSISTENT_EVENT_TYPE_SMART_HEALTH_LOG_SNAPSHOT = 0x01,
    NVME_PERSISTENT_EVENT_TYPE_FIRMWARE_COMMIT = 0x02,
    NVME_PERSISTENT_EVENT_TYPE_TIMESTAMP_CHANGE = 0x03,
    NVME_PERSISTENT_EVENT_TYPE_POWER_ON_OR_RESET = 0x04,
    NVME_PERSISTENT_EVENT_TYPE_NVM_SUBSYSTEM_HARDWARE_ERROR = 0x05,
    NVME_PERSISTENT_EVENT_TYPE_CHANGE_NAMESPACE = 0x06,
    NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_START = 0x07,
    NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_COMPLETION = 0x08,
    NVME_PERSISTENT_EVENT_TYPE_SANITIZE_START = 0x09,
    NVME_PERSISTENT_EVENT_TYPE_SANITIZE_COMPLETION = 0x0A,
    NVME_PERSISTENT_EVENT_TYPE_SET_FEATURE = 0x0B,
    NVME_PERSISTENT_EVENT_TYPE_TELEMETRY_LOG_CREATED = 0x0C,
    NVME_PERSISTENT_EVENT_TYPE_THERMAL_EXCURSION = 0x0D,
    NVME_PERSISTENT_EVENT_TYPE_RESERVED1_BEGIN = 0x0E,
    NVME_PERSISTENT_EVENT_TYPE_RESERVED1_END = 0xDD,
    NVME_PERSISTENT_EVENT_TYPE_VENDOR_SPECIFIC_EVENT = 0xDE,
    NVME_PERSISTENT_EVENT_TYPE_TCG_DEFINED = 0xDF,
    NVME_PERSISTENT_EVENT_TYPE_RESERVED2_BEGIN = 0xE0,
    NVME_PERSISTENT_EVENT_TYPE_RESERVED2_END = 0xFF,
    NVME_PERSISTENT_EVENT_TYPE_MAX = 0xFF,
}

//
// Information of log: NVME_LOG_PAGE_RESERVATION_NOTIFICATION. Size: 64 bytes
//
#[derive(Debug, Clone, Copy)]
enum NVME_RESERVATION_NOTIFICATION_TYPES {
    NVME_RESERVATION_NOTIFICATION_TYPE_EMPTY_LOG_PAGE = 0,
    NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_PREEMPTED = 1,
    NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_RELEASED = 2,
    NVME_RESERVATION_NOTIFICATION_TYPE_RESERVATION_PREEMPTED = 3,
}

#[derive(Debug, Clone, Copy)]
struct NVME_RESERVATION_NOTIFICATION_LOG {
    LogPageCount: u64,         // Log Page Count
    LogPageType: u8,           // Reservation Notification Log Page Type.
    AvailableLogPageCount: u8, // Number of Available Log Pages
    Reserved0: [u8; 2],
    NameSpaceId: u32, // Namespace ID
    Reserved1: [u8; 48],
}

//
// Information of log: NVME_SANITIZE_STATUS_LOG. Size: 512 bytes
//
#[derive(Debug, Clone, Copy)]
enum NVME_SANITIZE_OPERATION_STATUS {
    NVME_SANITIZE_OPERATION_NONE = 0,
    NVME_SANITIZE_OPERATION_SUCCEEDED = 1,
    NVME_SANITIZE_OPERATION_IN_PROGRESS = 2,
    NVME_SANITIZE_OPERATION_FAILED = 3,
    NVME_SANITIZE_OPERATION_SUCCEEDED_WITH_FORCED_DEALLOCATION = 4,
}

#[derive(Debug, Clone, Copy)]
struct NVME_SANITIZE_STATUS {
    MostRecentSanitizeOperationStatus: u8,
    NumberCompletedPassesOfOverwrite: u8,
    GlobalDataErased: bool,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct NVME_SANITIZE_STATUS_LOG {
    SPROG: u16,
    SSTAT: NVME_SANITIZE_STATUS,
    SCDW10: u32,
    EstimatedTimeForOverwrite: u32,
    EstimatedTimeForBlockErase: u32,
    EstimatedTimeForCryptoErase: u32,
    EstimatedTimeForOverwriteWithNoDeallocateMediaModification: u32,
    EstimatedTimeForBlockEraseWithNoDeallocateMediaModification: u32,
    EstimatedTimeForCryptoEraseWithNoDeallocateMediaModification: u32,
    Reserved: [u8; 480],
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_FIRMWARE_DOWNLOAD {
    NUMD: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_FIRMWARE_DOWNLOAD {
    OFST: u32,
}

#[derive(Debug, Clone, Copy)]
enum NVME_FIRMWARE_ACTIVATE_ACTIONS {
    NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT = 0,
    NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE = 1,
    NVME_FIRMWARE_ACTIVATE_ACTION_ACTIVATE = 2,
    NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE_IMMEDIATE = 3,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW10_FIRMWARE_ACTIVATE {
    DUMMYSTRUCTNAME: NVME_CDW10_FIRMWARE_ACTIVATE_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_FIRMWARE_ACTIVATE_STRUCT {
    FS: u32,
    AA: u32,
    Reserved: u32,
}

//
// Parameters for FORMAT NVM Commands
//
#[derive(Debug, Clone, Copy)]
enum NVME_PROTECTION_INFORMATION_TYPES {
    NVME_PROTECTION_INFORMATION_NOT_ENABLED = 0,
    NVME_PROTECTION_INFORMATION_TYPE1 = 1,
    NVME_PROTECTION_INFORMATION_TYPE2 = 2,
    NVME_PROTECTION_INFORMATION_TYPE3 = 3,
}

#[derive(Debug, Clone, Copy)]
enum NVME_SECURE_ERASE_SETTINGS {
    NVME_SECURE_ERASE_NONE = 0,
    NVME_SECURE_ERASE_USER_DATA = 1,
    NVME_SECURE_ERASE_CRYPTOGRAPHIC = 2,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_FORMAT_NVM_STRUCT {
    LBAF: u32, // LBA Format (LBAF)
    MS: u32,   // Metadata Settings (MS)
    PI: u32,   // Protection Information (PI)
    PIL: u32,  // Protection Information Location (PIL)
    SES: u32,  // Secure Erase Settings (SES)
    ZF: u32,   // Zone Format (ZF)
    Reserved: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW10_FORMAT_NVM {
    DUMMYSTRUCTNAME: NVME_CDW10_FORMAT_NVM_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
enum NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE {
    NVME_MEDIA_ADDITIONALLY_MODIFIED_AFTER_SANITIZE_NOT_DEFINED = 0,
    NVME_MEDIA_NOT_ADDITIONALLY_MODIFIED_AFTER_SANITIZE = 1,
    NVME_MEDIA_ADDITIONALLY_MODIFIED_AFTER_SANITIZE = 2,
}

//
// Parameters for Sanitize.
//

#[derive(Debug, Clone, Copy)]
enum NVME_SANITIZE_ACTION {
    NVME_SANITIZE_ACTION_RESERVED = 0,
    NVME_SANITIZE_ACTION_EXIT_FAILURE_MODE = 1,
    NVME_SANITIZE_ACTION_START_BLOCK_ERASE_SANITIZE = 2,
    NVME_SANITIZE_ACTION_START_OVERWRITE_SANITIZE = 3,
    NVME_SANITIZE_ACTION_START_CRYPTO_ERASE_SANITIZE = 4,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_SANITIZE_STRUCT {
    SANACT: u32, // Sanitize Action (SANACT)
    AUSE: u32,   // Allow Unrestricted Sanitize Exit (AUSE)
    OWPASS: u32, // Overwrite Pass Count (OWPASS)
    OIPBP: u32,  // Overwrite Invert Pattern Between Passes (OIPBP)
    NDAS: u32,   // No Deallocate After Sanitize
    Reserved: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW10_SANITIZE {
    DUMMYSTRUCTNAME: NVME_CDW10_SANITIZE_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_SANITIZE_STRUCT {
    OVRPAT: u32, // Overwrite Pattern
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW11_SANITIZE {
    DUMMYSTRUCTNAME: NVME_CDW11_SANITIZE_STRUCT,
    AsUlong: u32,
}

//
// Parameters for RESERVATION Commands
//
#[derive(Debug, Clone, Copy)]
enum NVME_RESERVATION_TYPES {
    NVME_RESERVATION_TYPE_RESERVED = 0,
    NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE = 1,
    NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS = 2,
    NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_REGISTRANTS_ONLY = 3,
    NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_REGISTRANTS_ONLY = 4,
    NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_ALL_REGISTRANTS = 5,
    NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_ALL_REGISTRANTS = 6,
}

#[derive(Debug, Clone, Copy)]
enum NVME_RESERVATION_ACQUIRE_ACTIONS {
    NVME_RESERVATION_ACQUIRE_ACTION_ACQUIRE = 0,
    NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT = 1,
    NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT_AND_ABORT = 2,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW0_RESERVATION_PERSISTENCE {
    PTPL: u32, // Persist Through Power Loss (PTPL)
    Reserved: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW10_RESERVATION_ACQUIRE {
    bits: u32,
    fields: NVME_CDW10_RESERVATION_ACQUIRE_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_RESERVATION_ACQUIRE_FIELDS {
    RACQA: u32, // Reservation Acquire Action (RACQA)
    IEKEY: u32, // Ignore Existing Key (IEKEY)
    Reserved: u32,
    RTYPE: u32, // Reservation Type (RTYPE)
    Reserved1: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_RESERVATION_ACQUIRE_DATA_STRUCTURE {
    CRKEY: u64, // Current Reservation Key (CRKEY)
    PRKEY: u64, // Preempt Reservation Key (PRKEY)
}

#[derive(Debug, Clone, Copy)]
enum NVME_RESERVATION_REGISTER_ACTIONS {
    NVME_RESERVATION_REGISTER_ACTION_REGISTER = 0,
    NVME_RESERVATION_REGISTER_ACTION_UNREGISTER = 1,
    NVME_RESERVATION_REGISTER_ACTION_REPLACE = 2,
}

#[derive(Debug, Clone, Copy)]
enum NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES {
    NVME_RESERVATION_REGISTER_PTPL_STATE_NO_CHANGE = 0,
    NVME_RESERVATION_REGISTER_PTPL_STATE_RESERVED = 1,
    NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_0 = 2, // Reservations are released and registrants are cleared on a power on.
    NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_1 = 3, // Reservations and registrants persist across a power loss.
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW10_RESERVATION_REGISTER {
    bits: u32,
    fields: NVME_CDW10_RESERVATION_REGISTER_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_RESERVATION_REGISTER_FIELDS {
    RREGA: u32, // Reservation Register Action (RREGA)
    IEKEY: u32, // Ignore Existing Key (IEKEY)
    Reserved: u32,
    CPTPL: u32, // Change Persist Through Power Loss State (CPTPL)
}

//
// Reservation Register Data Structure
//
#[derive(Debug, Clone, Copy)]
struct NVME_RESERVATION_REGISTER_DATA_STRUCTURE {
    CRKEY: u64, // Current Reservation Key (CRKEY)
    NRKEY: u64, // New Reservation Key (NRKEY)
}

#[derive(Debug, Clone, Copy)]
enum NVME_RESERVATION_RELEASE_ACTIONS {
    NVME_RESERVATION_RELEASE_ACTION_RELEASE = 0,
    NVME_RESERVATION_RELEASE_ACTION_CLEAR = 1,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW10_RESERVATION_RELEASE {
    bits: u32,
    fields: NVME_CDW10_RESERVATION_RELEASE_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_RESERVATION_RELEASE_FIELDS {
    RRELA: u32, // Reservation Release Action (RRELA)
    IEKEY: u32, // IgnoreExistingKey (IEKEY)
    Reserved: u32,
    RTYPE: u32, // Reservation Type (RTYPE)
    Reserved1: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_RESERVATION_RELEASE_DATA_STRUCTURE {
    CRKEY: u64, // Current Reservation Key (CRKEY)
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW10_RESERVATION_REPORT {
    bits: u32,
    fields: NVME_CDW10_RESERVATION_REPORT_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_RESERVATION_REPORT_FIELDS {
    NUMD: u32, // Number of Dwords (NUMD), NOTE: 0's based value.
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW11_RESERVATION_REPORT {
    bits: u32,
    fields: NVME_CDW11_RESERVATION_REPORT_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_RESERVATION_REPORT_FIELDS {
    EDS: u32, // Extended Data Structure (EDS)
    Reserved: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct NVME_RESERVATION_REPORT_STATUS_HEADER {
    GEN: u32,    // Generation (Gen)
    RTYPE: u8,   // Reservation Type (RTYPE)
    REGCTL: u16, // Number of Registered Controllers (REGCTL)
    Reserved: [u8; 2],
    PTPLS: u8, // Persist Through Power Loss State (PTPLS)
    Reserved1: [u8; 14],
}

#[derive(Debug, Clone, Copy)]
struct NVME_REGISTERED_CONTROLLER_DATA {
    CNTLID: u16,                                  // Controller ID (CNTLID)
    RCSTS: NVME_REGISTERED_CONTROLLER_DATA_RCSTS, // Reservation Status (RCSTS)
    Reserved: [u8; 5],
    HOSTID: [u8; 8], // Host Identifier (HOSTID)
    RKEY: u64,       // Reservation Key (RKEY)
}

#[derive(Debug, Clone, Copy)]
struct NVME_REGISTERED_CONTROLLER_DATA_RCSTS {
    HoldReservation: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE {
    Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    RegisteredControllersData: [NVME_REGISTERED_CONTROLLER_DATA; 0], // ANYSIZE_ARRAY equivalent
}

#[derive(Debug, Clone, Copy)]
struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA {
    CNTLID: u16,                                           // Controller ID (CNTLID)
    RCSTS: NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_RCSTS, // Reservation Status (RCSTS)
    Reserved: [u8; 5],
    RKEY: u64,        // Reservation Key (RKEY)
    HOSTID: [u8; 16], // 128-bit Host Identifier (HOSTID)
    Reserved1: [u8; 32],
}

#[derive(Debug, Clone, Copy)]
struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_RCSTS {
    HoldReservation: u8,
    Reserved: u8,
}

#[derive(Debug, Clone, Copy)]
struct NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
    Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    Reserved1: [u8; 40],
    RegisteredControllersExtendedData: [NVME_REGISTERED_CONTROLLER_EXTENDED_DATA; 0], // ANYSIZE_ARRAY equivalent
}

//
// Parameters for Directives.
//
#[derive(Debug, Clone, Copy)]
enum NVME_DIRECTIVE_TYPES {
    NVME_DIRECTIVE_TYPE_IDENTIFY = 0x00,
    NVME_DIRECTIVE_TYPE_STREAMS = 0x01,
}

const NVME_STREAMS_ID_MIN: u16 = 1;
const NVME_STREAMS_ID_MAX: u16 = 0xFFFF;

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_DIRECTIVE_RECEIVE {
    NUMD: u32, // Number of Dwords (NUMD)
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW11_DIRECTIVE_RECEIVE {
    bits: u32,
    fields: NVME_CDW11_DIRECTIVE_RECEIVE_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_DIRECTIVE_RECEIVE_FIELDS {
    DOPER: u32, // Directive Operation
    DTYPE: u32, // Directive Type
    DSPEC: u32, // Directive Specific
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_DIRECTIVE_SEND {
    NUMD: u32, // Number of Dwords (NUMD)
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW11_DIRECTIVE_SEND {
    bits: u32,
    fields: NVME_CDW11_DIRECTIVE_SEND_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_DIRECTIVE_SEND_FIELDS {
    DOPER: u32, // Directive Operation
    DTYPE: u32, // Directive Type
    DSPEC: u32, // Directive Specific
}

#[derive(Debug, Clone, Copy)]
enum NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATIONS {
    NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATION_RETURN_PARAMETERS = 1,
}

#[derive(Debug, Clone, Copy)]
enum NVME_DIRECTIVE_SEND_IDENTIFY_OPERATIONS {
    NVME_DIRECTIVE_SEND_IDENTIFY_OPERATION_ENABLE_DIRECTIVE = 1,
}

#[derive(Debug, Clone, Copy)]
struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR {
    Identify: u8,
    Streams: u8,
    Reserved0: u8,
    Reserved1: [u8; 31],
}

#[derive(Debug, Clone, Copy)]
struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS {
    DirectivesSupported: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
    DirectivesEnabled: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
    // Reserved: [u8; 4032], // Uncomment if needed
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE {
    bits: u32,
    fields: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_FIELDS {
    ENDIR: u32, // Enable Directive
    Reserved0: u32,
    DTYPE: u32, // Directive Type
    Reserved1: u32,
}

//
// Parameters for the Streams Directive Type
//
#[derive(Debug, Clone, Copy)]
enum NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS {
    NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_RETURN_PARAMETERS = 1,
    NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_GET_STATUS = 2,
    NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_ALLOCATE_RESOURCES = 3,
}

#[derive(Debug, Clone, Copy)]
enum NVME_DIRECTIVE_SEND_STREAMS_OPERATIONS {
    NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_IDENTIFIER = 1,
    NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_RESOURCES = 2,
}

#[derive(Debug, Clone, Copy)]
struct NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS {
    MSL: u16,  // Max Streams Limit
    NSSA: u16, // NVM Subsystem Streams Available
    NSSO: u16, // NVM Subsystem Streams Open
    Reserved0: [u8; 10],
    SWS: u32, // Stream Write Size
    SGS: u16, // Stream Granularity Size
    NSA: u16, // Namespace Streams Allocated
    NSO: u16, // Namespace Streams Open
    Reserved1: [u8; 6],
}

const NVME_STREAMS_GET_STATUS_MAX_IDS: usize = 65535;

#[derive(Debug, Clone, Copy)]
struct NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
    OpenStreamCount: u16, // Number of currently open streams.
    StreamIdentifiers: [u16; NVME_STREAMS_GET_STATUS_MAX_IDS], // Array of stream IDs that are currently open.
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    bits: u32,
    fields: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_FIELDS,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_FIELDS {
    NSR: u16, // Namespace Streams Requested
    Reserved: u16,
}

#[derive(Debug, Clone, Copy)]
struct NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    NSA: u16, // Namespace Streams Allocated
    Reserved: u16,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW12_DIRECTIVE_SEND {
    EnableDirective: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
union NVME_CDW12_DIRECTIVE_RECEIVE {
    AllocateResources: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES,
    AsUlong: u32,
}

//
// Parameters for SECURITY SEND / RECEIVE Commands
//
#[derive(Debug, Clone, Copy)]
union NVME_CDW10_SECURITY_SEND_RECEIVE {
    DUMMYSTRUCTNAME: NVME_CDW10_SECURITY_SEND_RECEIVE_STRUCT,
    AsUlong: u32,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW10_SECURITY_SEND_RECEIVE_STRUCT {
    Reserved0: u32, // Reserved0
    SPSP: u32,      // SP Specific (SPSP)
    SECP: u32,      // Security Protocol (SECP)
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_SECURITY_SEND {
    TL: u32, // Transfer Length (TL)
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW11_SECURITY_RECEIVE {
    AL: u32, // Transfer Length (AL)
}

#[derive(Debug, Clone, Copy)]
enum NVME_NVM_COMMANDS {
    NVME_NVM_COMMAND_FLUSH = 0x00,
    NVME_NVM_COMMAND_WRITE = 0x01,
    NVME_NVM_COMMAND_READ = 0x02,
    NVME_NVM_COMMAND_WRITE_UNCORRECTABLE = 0x04,
    NVME_NVM_COMMAND_COMPARE = 0x05,
    NVME_NVM_COMMAND_WRITE_ZEROES = 0x08,
    NVME_NVM_COMMAND_DATASET_MANAGEMENT = 0x09,
    NVME_NVM_COMMAND_VERIFY = 0x0C,
    NVME_NVM_COMMAND_RESERVATION_REGISTER = 0x0D,
    NVME_NVM_COMMAND_RESERVATION_REPORT = 0x0E,
    NVME_NVM_COMMAND_RESERVATION_ACQUIRE = 0x11,
    NVME_NVM_COMMAND_RESERVATION_RELEASE = 0x15,
    NVME_NVM_COMMAND_COPY = 0x19,
    NVME_NVM_COMMAND_ZONE_MANAGEMENT_SEND = 0x79,
    NVME_NVM_COMMAND_ZONE_MANAGEMENT_RECEIVE = 0x7A,
    NVME_NVM_COMMAND_ZONE_APPEND = 0x7D,
}

//
// Data structure of CDW12 for Read/Write command
//
#[derive(Debug, Clone, Copy)]
struct NVME_CDW12_READ_WRITE {
    NLB: u16, // Number of Logical Blocks (NLB)
    Reserved0: u8,
    DTYPE: u8, // Directive Type (DTYPE)
    Reserved1: u8,
    PRINFO: u8, // Protection Information Field (PRINFO)
    FUA: bool,  // Force Unit Access (FUA)
    LR: bool,   // Limited Retry (LR)
}

#[derive(Debug, Clone, Copy)]
enum NVME_ACCESS_FREQUENCIES {
    NVME_ACCESS_FREQUENCY_NONE = 0, // No frequency information provided.
    NVME_ACCESS_FREQUENCY_TYPICAL = 1, // Typical number of reads and writes expected for this LBA range.
    NVME_ACCESS_FREQUENCY_INFR_WRITE_INFR_READ = 2, // Infrequent writes and infrequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_INFR_WRITE_FR_READ = 3, // Infrequent writes and frequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_FR_WRITE_INFR_READ = 4, // Frequent writes and infrequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_FR_WRITE_FR_READ = 5, // Frequent writes and frequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_ONE_TIME_READ = 6, // One time read. E.g. command is due to virus scan, backup, file copy, or archive.
    NVME_ACCESS_FREQUENCY_SPECULATIVE_READ = 7, // Speculative read. The command is part of a prefetch operation.
    NVME_ACCESS_FREQUENCY_WILL_BE_OVERWRITTEN = 8, // The LBA range is going to be overwritten in the near future.
}

#[derive(Debug, Clone, Copy)]
enum NVME_ACCESS_LATENCIES {
    NVME_ACCESS_LATENCY_NONE = 0, // None.  No latency information provided.
    NVME_ACCESS_LATENCY_IDLE = 1, // Idle. Longer latency acceptable
    NVME_ACCESS_LATENCY_NORMAL = 2, // Normal. Typical latency.
    NVME_ACCESS_LATENCY_LOW = 3,  // Low. Smallest possible latency
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW13_READ_WRITE_DSM {
    AccessFrequency: u8,
    AccessLatency: u8,
    SequentialRequest: bool,
    Incompressible: bool,
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW13_READ_WRITE {
    DSM: NVME_CDW13_READ_WRITE_DSM, // Dataset Management (DSM)
    Reserved: u8,
    DSPEC: u16, // Directive Specific Value
}

#[derive(Debug, Clone, Copy)]
struct NVME_CDW15_READ_WRITE {
    ELBAT: u16,  // Expected Logical Block Application Tag (ELBAT)
    ELBATM: u16, // Expected Logical Block Application Tag Mask (ELBATM)
}

#[repr(C)]
union NVME_COMMAND_DWORD0 {
    DUMMYSTRUCTNAME: NVME_COMMAND_DWORD0_STRUCT,
    AsUlong: u32,
}

#[repr(C)]
struct NVME_COMMAND_DWORD0_STRUCT {
    OPC: u8,  // Opcode (OPC)
    FUSE: u8, // Fused Operation (FUSE)
    Reserved0: u8,
    PSDT: u8, // PRP or SGL for Data Transfer (PSDT)
    CID: u16, // Command Identifier (CID)
}

#[repr(C)]
enum NVME_FUSED_OPERATION_CODES {
    NVME_FUSED_OPERATION_NORMAL = 0,
    NVME_FUSED_OPERATION_FIRST_CMD = 1,
    NVME_FUSED_OPERATION_SECOND_CMD = 2,
}

#[repr(C)]
union NVME_PRP_ENTRY {
    DUMMYSTRUCTNAME: NVME_PRP_ENTRY_STRUCT,
    AsUlonglong: u64,
}

#[repr(C)]
struct NVME_PRP_ENTRY_STRUCT {
    Reserved0: u64,
    PBAO: u64, // Page Base Address and Offset (PBAO)
}

const NVME_NAMESPACE_ALL: u32 = 0xFFFFFFFF;

//
// NVMe command data structure
//
#[repr(C)]
union NVME_COMMAND {
    CDW0: NVME_COMMAND_DWORD0,
    NSID: u32,
    Reserved0: [u32; 2],
    MPTR: u64,
    PRP1: u64,
    PRP2: u64,
    u: NVME_COMMAND_UNION,
}

#[repr(C)]
union NVME_COMMAND_UNION {
    GENERAL: NVME_COMMAND_GENERAL,
    IDENTIFY: NVME_COMMAND_IDENTIFY,
    ABORT: NVME_COMMAND_ABORT,
    GETFEATURES: NVME_COMMAND_GETFEATURES,
    SETFEATURES: NVME_COMMAND_SETFEATURES,
    GETLOGPAGE: NVME_COMMAND_GETLOGPAGE,
    CREATEIOCQ: NVME_COMMAND_CREATEIOCQ,
    CREATEIOSQ: NVME_COMMAND_CREATEIOSQ,
    DATASETMANAGEMENT: NVME_COMMAND_DATASETMANAGEMENT,
    SECURITYSEND: NVME_COMMAND_SECURITYSEND,
    SECURITYRECEIVE: NVME_COMMAND_SECURITYRECEIVE,
    FIRMWAREDOWNLOAD: NVME_COMMAND_FIRMWAREDOWNLOAD,
    FIRMWAREACTIVATE: NVME_COMMAND_FIRMWAREACTIVATE,
    FORMATNVM: NVME_COMMAND_FORMATNVM,
    DIRECTIVERECEIVE: NVME_COMMAND_DIRECTIVERECEIVE,
    DIRECTIVESEND: NVME_COMMAND_DIRECTIVESEND,
    SANITIZE: NVME_COMMAND_SANITIZE,
    READWRITE: NVME_COMMAND_READWRITE,
    RESERVATIONACQUIRE: NVME_COMMAND_RESERVATIONACQUIRE,
    RESERVATIONREGISTER: NVME_COMMAND_RESERVATIONREGISTER,
    RESERVATIONRELEASE: NVME_COMMAND_RESERVATIONRELEASE,
    RESERVATIONREPORT: NVME_COMMAND_RESERVATIONREPORT,
    ZONEMANAGEMENTSEND: NVME_COMMAND_ZONEMANAGEMENTSEND,
    ZONEMANAGEMENTRECEIVE: NVME_COMMAND_ZONEMANAGEMENTRECEIVE,
    ZONEAPPEND: NVME_COMMAND_ZONEAPPEND,
}

#[repr(C)]
struct NVME_COMMAND_GENERAL {
    CDW10: u32,
    CDW11: u32,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_IDENTIFY {
    CDW10: NVME_CDW10_IDENTIFY,
    CDW11: NVME_CDW11_IDENTIFY,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_ABORT {
    CDW10: NVME_CDW10_ABORT,
    CDW11: u32,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_GETFEATURES {
    CDW10: NVME_CDW10_GET_FEATURES,
    CDW11: NVME_CDW11_FEATURES,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_SETFEATURES {
    CDW10: NVME_CDW10_SET_FEATURES,
    CDW11: NVME_CDW11_FEATURES,
    CDW12: NVME_CDW12_FEATURES,
    CDW13: NVME_CDW13_FEATURES,
    CDW14: NVME_CDW14_FEATURES,
    CDW15: NVME_CDW15_FEATURES,
}

#[repr(C)]
struct NVME_COMMAND_GETLOGPAGE {
    CDW10: NVME_CDW10_GET_LOG_PAGE,
    CDW11: NVME_CDW11_GET_LOG_PAGE,
    CDW12: NVME_CDW12_GET_LOG_PAGE,
    CDW13: NVME_CDW13_GET_LOG_PAGE,
    CDW14: NVME_CDW14_GET_LOG_PAGE,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_CREATEIOCQ {
    CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    CDW11: NVME_CDW11_CREATE_IO_CQ,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_CREATEIOSQ {
    CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    CDW11: NVME_CDW11_CREATE_IO_SQ,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_DATASETMANAGEMENT {
    CDW10: NVME_CDW10_DATASET_MANAGEMENT,
    CDW11: NVME_CDW11_DATASET_MANAGEMENT,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_SECURITYSEND {
    CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    CDW11: NVME_CDW11_SECURITY_SEND,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_SECURITYRECEIVE {
    CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    CDW11: NVME_CDW11_SECURITY_RECEIVE,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_FIRMWAREDOWNLOAD {
    CDW10: NVME_CDW10_FIRMWARE_DOWNLOAD,
    CDW11: NVME_CDW11_FIRMWARE_DOWNLOAD,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_FIRMWAREACTIVATE {
    CDW10: NVME_CDW10_FIRMWARE_ACTIVATE,
    CDW11: u32,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_FORMATNVM {
    CDW10: NVME_CDW10_FORMAT_NVM,
    CDW11: u32,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_DIRECTIVERECEIVE {
    CDW10: NVME_CDW10_DIRECTIVE_RECEIVE,
    CDW11: NVME_CDW11_DIRECTIVE_RECEIVE,
    CDW12: NVME_CDW12_DIRECTIVE_RECEIVE,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_DIRECTIVESEND {
    CDW10: NVME_CDW10_DIRECTIVE_SEND,
    CDW11: NVME_CDW11_DIRECTIVE_SEND,
    CDW12: NVME_CDW12_DIRECTIVE_SEND,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_SANITIZE {
    CDW10: NVME_CDW10_SANITIZE,
    CDW11: NVME_CDW11_SANITIZE,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_READWRITE {
    LBALOW: u32,
    LBAHIGH: u32,
    CDW12: NVME_CDW12_READ_WRITE,
    CDW13: NVME_CDW13_READ_WRITE,
    CDW14: u32,
    CDW15: NVME_CDW15_READ_WRITE,
}

#[repr(C)]
struct NVME_COMMAND_RESERVATIONACQUIRE {
    CDW10: NVME_CDW10_RESERVATION_ACQUIRE,
    CDW11: u32,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_RESERVATIONREGISTER {
    CDW10: NVME_CDW10_RESERVATION_REGISTER,
    CDW11: u32,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_RESERVATIONRELEASE {
    CDW10: NVME_CDW10_RESERVATION_RELEASE,
    CDW11: u32,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_RESERVATIONREPORT {
    CDW10: NVME_CDW10_RESERVATION_REPORT,
    CDW11: NVME_CDW11_RESERVATION_REPORT,
    CDW12: u32,
    CDW13: u32,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_ZONEMANAGEMENTSEND {
    CDW1011: NVME_CDW10_ZONE_MANAGEMENT_SEND,
    CDW12: u32,
    CDW13: NVME_CDW13_ZONE_MANAGEMENT_SEND,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_ZONEMANAGEMENTRECEIVE {
    CDW1011: NVME_CDW10_ZONE_MANAGEMENT_RECEIVE,
    DWORDCOUNT: u32,
    CDW13: NVME_CDW13_ZONE_MANAGEMENT_RECEIVE,
    CDW14: u32,
    CDW15: u32,
}

#[repr(C)]
struct NVME_COMMAND_ZONEAPPEND {
    CDW1011: NVME_CDW10_ZONE_APPEND,
    CDW12: NVME_CDW12_ZONE_APPEND,
    CDW13: u32,
    ILBRT: u32,
    CDW15: NVME_CDW15_ZONE_APPEND,
}
