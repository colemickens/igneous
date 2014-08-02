#![feature(globs)]

extern crate serialize;
extern crate http;
extern crate openssl;
extern crate time;

pub mod blobstorage {
  use http::client::RequestWriter;
  use http::headers::HeaderConvertible;
  use http::headers::HeaderEnum;
  use http::headers::test_utils::to_stream_into_str;
  use http::method::Get;
  use openssl::crypto::hash::*;
  use openssl::crypto::hmac::*;
  use std::ascii::OwnedStrAsciiExt;
  use std::path::BytesContainer;
  use serialize::base64::*;
  use serialize::base64::FromBase64;
  use time;

  pub struct BlobStorageClient {
    pub account_name: String,
    pub key: Vec<u8>
  }

  pub fn extract<T: HeaderConvertible>(element: Option<T>) -> String {
    match element {
      None => "".to_string(),
      Some(ref x) => to_stream_into_str(x)
    }
  }
  
  pub fn new_client(account_name: &str, account_key: &str) -> BlobStorageClient {
    let key = account_key.as_slice().from_base64().unwrap();
    BlobStorageClient{
      account_name: account_name.to_string(),
      key: key
    }
  }
  
  impl BlobStorageClient {
    pub fn new_list_blob_req(&self, containerName: &str) -> RequestWriter {
      let url = format!("https://{}.blob.core.windows.net/{}?restype=container", self.account_name, containerName);

      let mut rw = RequestWriter::new(
          Get,
          from_str(url.as_slice()).expect("Invalid URL :-(")
      ).unwrap();

      // TODO(colemick): this is common to all reqs, extract
      rw.headers.date = Some(time::now_utc());
      rw.headers.extensions.insert("x-ms-version".to_string(), "2009-09-19".to_string());
      rw
    }

    pub fn sign_request(&self, rw: &mut RequestWriter) {
      let hdrs = rw.headers.clone();

      let strToSign = format!("{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}",
        rw.method.to_string(),
        extract(hdrs.content_encoding),
        extract(hdrs.content_language),
        extract(hdrs.content_length),
        extract(hdrs.content_md5),
        extract(hdrs.content_type),
        extract(hdrs.date),
        extract(hdrs.if_modified_since),
        extract(hdrs.if_match),
        extract(hdrs.if_none_match),
        extract(hdrs.if_unmodified_since),
        extract(hdrs.range),
        self.canonicalized_headers(rw),
        self.canonicalized_resource(rw),
      );

      let _ = hdrs;

      let mut hmac = HMAC(SHA256, self.key.container_as_bytes());
      hmac.update(strToSign.as_bytes());
      let shared_key = hmac.final().as_slice().to_base64(STANDARD);

      rw.headers.authorization = Some(format!("SharedKey {:s}:{:s}", self.account_name, shared_key));
    }

    pub fn canonicalized_headers(&self, rw: &RequestWriter) -> String {
      let mut headers: Vec<String> = rw.headers.iter()
        .filter_map(|h| {
          let lower_key = h.header_name().into_ascii_lower();
          if lower_key.as_slice().starts_with("x-ms-".as_slice()) {
            let hdr_line = format!("{:s}:{:s}", lower_key, h.header_value());
            return Some(hdr_line)
          }
          return None
        })
        .collect();
      
      headers.sort();
      headers.connect("\n")
    }

    pub fn canonicalized_resource(&self, rw: &RequestWriter) -> String {
      // TODO(colemickens):
      // make this map the query params into a vec with the first line
      // collect() once, then sort & connect with ("\n") (like canoniclized_headers)

      let mut res_str = format!("/{:s}{:s}", self.account_name, rw.url.path.path);
      for &(ref k, ref v) in rw.url.path.query.iter() {
        let lower_key = k.to_string().into_ascii_lower();
        res_str = res_str.append(
          format!("\n{:s}:{:s}", lower_key, v.to_string()).as_slice()
        );
      }

      let mut lines: Vec<&str> = res_str.as_slice().split('\n').collect();
      lines.sort();
      lines.connect("\n")
    }
  }
}