pub mod status_code {
    // 1xx Informational
    pub const CONTINUE: u16 = 100;
    pub const SWITCHING_PROTOCOLS: u16 = 101;
    pub const PROCESSING: u16 = 102;
    pub const EARLY_HINTS: u16 = 103;

    // 2xx Success
    pub const OK: u16 = 200;
    pub const CREATED: u16 = 201;
    pub const ACCEPTED: u16 = 202;
    pub const NON_AUTHORITATIVE_INFORMATION: u16 = 203;
    pub const NO_CONTENT: u16 = 204;
    pub const RESET_CONTENT: u16 = 205;
    pub const PARTIAL_CONTENT: u16 = 206;
    pub const MULTI_STATUS: u16 = 207;
    pub const ALREADY_REPORTED: u16 = 208;
    pub const IM_USED: u16 = 226;

    // 3xx Redirection
    pub const MULTIPLE_CHOICES: u16 = 300;
    pub const MOVED_PERMANENTLY: u16 = 301;
    pub const FOUND: u16 = 302;
    pub const SEE_OTHER: u16 = 303;
    pub const NOT_MODIFIED: u16 = 304;
    pub const USE_PROXY: u16 = 305;
    pub const TEMPORARY_REDIRECT: u16 = 307;
    pub const PERMANENT_REDIRECT: u16 = 308;

    // 4xx Client Errors
    pub const BAD_REQUEST: u16 = 400;
    pub const UNAUTHORIZED: u16 = 401;
    pub const PAYMENT_REQUIRED: u16 = 402;
    pub const FORBIDDEN: u16 = 403;
    pub const NOT_FOUND: u16 = 404;
    pub const METHOD_NOT_ALLOWED: u16 = 405;
    pub const NOT_ACCEPTABLE: u16 = 406;
    pub const PROXY_AUTHENTICATION_REQUIRED: u16 = 407;
    pub const REQUEST_TIMEOUT: u16 = 408;
    pub const CONFLICT: u16 = 409;
    pub const GONE: u16 = 410;
    pub const LENGTH_REQUIRED: u16 = 411;
    pub const PRECONDITION_FAILED: u16 = 412;
    pub const PAYLOAD_TOO_LARGE: u16 = 413;
    pub const URI_TOO_LONG: u16 = 414;
    pub const UNSUPPORTED_MEDIA_TYPE: u16 = 415;
    pub const RANGE_NOT_SATISFIABLE: u16 = 416;
    pub const EXPECTATION_FAILED: u16 = 417;
    pub const IM_A_TEAPOT: u16 = 418;
    pub const MISDIRECTED_REQUEST: u16 = 421;
    pub const UNPROCESSABLE_ENTITY: u16 = 422;
    pub const LOCKED: u16 = 423;
    pub const FAILED_DEPENDENCY: u16 = 424;
    pub const TOO_EARLY: u16 = 425;
    pub const UPGRADE_REQUIRED: u16 = 426;
    pub const PRECONDITION_REQUIRED: u16 = 428;
    pub const TOO_MANY_REQUESTS: u16 = 429;
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: u16 = 431;
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: u16 = 451;

    // 5xx Server Errors
    pub const INTERNAL_SERVER_ERROR: u16 = 500;
    pub const NOT_IMPLEMENTED: u16 = 501;
    pub const BAD_GATEWAY: u16 = 502;
    pub const SERVICE_UNAVAILABLE: u16 = 503;
    pub const GATEWAY_TIMEOUT: u16 = 504;
    pub const HTTP_VERSION_NOT_SUPPORTED: u16 = 505;
    pub const VARIANT_ALSO_NEGOTIATES: u16 = 506;
    pub const INSUFFICIENT_STORAGE: u16 = 507;
    pub const LOOP_DETECTED: u16 = 508;
    pub const NOT_EXTENDED: u16 = 510;
    pub const NETWORK_AUTHENTICATION_REQUIRED: u16 = 511;

    pub fn reason_phrase(code: u16) -> &'static str {
        match code {
            CONTINUE => "Continue",
            SWITCHING_PROTOCOLS => "Switching Protocols",
            PROCESSING => "Processing",
            EARLY_HINTS => "Early Hints",
            OK => "OK",
            CREATED => "Created",
            ACCEPTED => "Accepted",
            NON_AUTHORITATIVE_INFORMATION => "Non-Authoritative Information",
            NO_CONTENT => "No Content",
            RESET_CONTENT => "Reset Content",
            PARTIAL_CONTENT => "Partial Content",
            MULTI_STATUS => "Multi-Status",
            ALREADY_REPORTED => "Already Reported",
            IM_USED => "IM Used",
            MULTIPLE_CHOICES => "Multiple Choices",
            MOVED_PERMANENTLY => "Moved Permanently",
            FOUND => "Found",
            SEE_OTHER => "See Other",
            NOT_MODIFIED => "Not Modified",
            USE_PROXY => "Use Proxy",
            TEMPORARY_REDIRECT => "Temporary Redirect",
            PERMANENT_REDIRECT => "Permanent Redirect",
            BAD_REQUEST => "Bad Request",
            UNAUTHORIZED => "Unauthorized",
            PAYMENT_REQUIRED => "Payment Required",
            FORBIDDEN => "Forbidden",
            NOT_FOUND => "Not Found",
            METHOD_NOT_ALLOWED => "Method Not Allowed",
            NOT_ACCEPTABLE => "Not Acceptable",
            PROXY_AUTHENTICATION_REQUIRED => "Proxy Authentication Required",
            REQUEST_TIMEOUT => "Request Timeout",
            CONFLICT => "Conflict",
            GONE => "Gone",
            LENGTH_REQUIRED => "Length Required",
            PRECONDITION_FAILED => "Precondition Failed",
            PAYLOAD_TOO_LARGE => "Payload Too Large",
            URI_TOO_LONG => "URI Too Long",
            UNSUPPORTED_MEDIA_TYPE => "Unsupported Media Type",
            RANGE_NOT_SATISFIABLE => "Range Not Satisfiable",
            EXPECTATION_FAILED => "Expectation Failed",
            IM_A_TEAPOT => "I'm a teapot",
            MISDIRECTED_REQUEST => "Misdirected Request",
            UNPROCESSABLE_ENTITY => "Unprocessable Entity",
            LOCKED => "Locked",
            FAILED_DEPENDENCY => "Failed Dependency",
            TOO_EARLY => "Too Early",
            UPGRADE_REQUIRED => "Upgrade Required",
            PRECONDITION_REQUIRED => "Precondition Required",
            TOO_MANY_REQUESTS => "Too Many Requests",
            REQUEST_HEADER_FIELDS_TOO_LARGE => "Request Header Fields Too Large",
            UNAVAILABLE_FOR_LEGAL_REASONS => "Unavailable For Legal Reasons",
            INTERNAL_SERVER_ERROR => "Internal Server Error",
            NOT_IMPLEMENTED => "Not Implemented",
            BAD_GATEWAY => "Bad Gateway",
            SERVICE_UNAVAILABLE => "Service Unavailable",
            GATEWAY_TIMEOUT => "Gateway Timeout",
            HTTP_VERSION_NOT_SUPPORTED => "HTTP Version Not Supported",
            VARIANT_ALSO_NEGOTIATES => "Variant Also Negotiates",
            INSUFFICIENT_STORAGE => "Insufficient Storage",
            LOOP_DETECTED => "Loop Detected",
            NOT_EXTENDED => "Not Extended",
            NETWORK_AUTHENTICATION_REQUIRED => "Network Authentication Required",
            _ => "Unknown Status",
        }
    }
}
