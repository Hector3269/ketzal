use super::request::UploadedFile;
use std::collections::HashMap;
use crate::kernel::constants::{
    multipart::{BOUNDARY_PREFIX, CONTENT_DISPOSITION, CONTENT_TYPE_HEADER, NAME_PARAM, FILENAME_PARAM, DEFAULT_CONTENT_TYPE},
    protocol::CRLF
};

pub struct MultipartParser;

impl MultipartParser {
    pub fn parse(
        body: &[u8],
        boundary: &str,
    ) -> (HashMap<String, String>, HashMap<String, UploadedFile>) {
        let mut form_data = HashMap::new();
        let mut files = HashMap::new();

        let boundary_bytes = format!("{}{}", BOUNDARY_PREFIX, boundary).into_bytes();
        let end_boundary_bytes = format!("{}{}{}", BOUNDARY_PREFIX, boundary, BOUNDARY_PREFIX).into_bytes();

        let mut parts = Vec::new();
        let mut current_pos = 0;

        while current_pos < body.len() {
            if let Some(pos) = Self::find_subsequence(&body[current_pos..], &boundary_bytes) {
                let start = current_pos + pos + boundary_bytes.len();
                // Find next boundary
                if let Some(next_pos) = Self::find_subsequence(&body[start..], &boundary_bytes) {
                    parts.push(&body[start..start + next_pos]);
                    current_pos = start + next_pos;
                } else if let Some(end_pos) =
                    Self::find_subsequence(&body[start..], &end_boundary_bytes)
                {
                    parts.push(&body[start..start + end_pos]);
                    break;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        for part in parts {
            if part.is_empty() {
                continue;
            }

            // Trim leading \r\n
            let mut part = part;
            if part.starts_with(CRLF.as_bytes()) {
                part = &part[2..];
            }

            // Split into headers and body
            if let Some(header_end) = Self::find_subsequence(part, format!("{}{}", CRLF, CRLF).as_bytes()) {
                let headers_raw = &part[..header_end];
                let content = &part[header_end + 4..];

                // Trim trailing \r\n from content
                let mut final_content = content;
                if final_content.ends_with(CRLF.as_bytes()) {
                    final_content = &final_content[..final_content.len() - 2];
                }

                let headers_str = String::from_utf8_lossy(headers_raw);
                let mut name = String::new();
                let mut filename = None;
                let mut content_type = DEFAULT_CONTENT_TYPE.to_string();

                for line in headers_str.lines() {
                    if line.to_lowercase().starts_with(CONTENT_DISPOSITION) {
                        if let Some(n) = Self::capture_between(line, NAME_PARAM, "\"") {
                            name = n;
                        }
                        if let Some(f) = Self::capture_between(line, FILENAME_PARAM, "\"") {
                            filename = Some(f);
                        }
                    } else if line.to_lowercase().starts_with(CONTENT_TYPE_HEADER) {
                        content_type = line[CONTENT_TYPE_HEADER.len()..].trim().to_string();
                    }
                }

                if let Some(fname) = filename {
                    files.insert(
                        name,
                        UploadedFile::new(fname, content_type, final_content.to_vec()),
                    );
                } else {
                    form_data.insert(name, String::from_utf8_lossy(final_content).to_string());
                }
            }
        }

        (form_data, files)
    }

    fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
    }

    fn capture_between(s: &str, start: &str, end: &str) -> Option<String> {
        if let Some(start_pos) = s.find(start) {
            let start_idx = start_pos + start.len();
            if let Some(end_pos) = s[start_idx..].find(end) {
                return Some(s[start_idx..start_idx + end_pos].to_string());
            }
        }
        None
    }
}
