#![crate_id = "azure#0"]
#![desc = "Azure BlobStorage"]
#![license = "BSD"]
#![crate_type = "lib"]

#![feature(globs)]

extern crate serialize;
extern crate http;
extern crate openssl;

pub mod blobstorage {
  use std::io::IoResult;
  use std::path::BytesContainer;
  use serialize::base64::*;
  use http::client::RequestWriter;
  use http::headers::HeaderConvertible;
  use http::headers::request::ExtensionHeader;
  use http::headers::test_utils::to_stream_into_str;
  use http::method::Post;
  use openssl::crypto::hash::*;
  use openssl::crypto::hmac::*;

  pub struct BlobStorageClient {
    pub accountName: String,
    pub key: Vec<u8>
  }

  pub fn extract<T: HeaderConvertible>(element: Option<T>) -> String {
    match element {
      None => "".to_str(),
      Some(ref x) => to_stream_into_str(x)
    }
  }

  impl BlobStorageClient {
    pub fn new_upload_blob_ex_req(&self, containerName: &str, blobName: &str) -> IoResult<RequestWriter> {
      let url = format!("https://{}.blob.core.windows.net/{}/{}", self.accountName, containerName, blobName);

      RequestWriter::new(
          Post,
          from_str(url.as_slice()).expect("Invalid URL :-(")
      )
    }

    /*
    pub fn new_list_blob_ex_req(&self, containerName: &str, blobName: &str) -> IoResult<RequestWriter> {
      let url = format!("https://{}.blob.core.windows.net/{}/{}", self.accountName, containerName, blobName);

      RequestWriter::new(
          Post,
          from_str(url.as_slice()).expect("Invalid URL :-(")
      )
    }
    */

    pub fn sign_request(&self, rw: &mut RequestWriter) {
      let hdrs = rw.headers.clone();

      let strToSign = format!("{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}\n{:s}",
        "POST".to_str(), // TODO(colemickens): fix
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
      let shared_key = hmac.final().container_as_bytes().to_base64(STANDARD);

      rw.headers.insert(ExtensionHeader("Authorization".to_str(), format!("Shared-Key: {}", shared_key)));
    }

    pub fn canonicalized_headers(&self, rw: &RequestWriter) -> String {
      "".to_str()
    }

    pub fn canonicalized_resource(&self, rw: &RequestWriter) -> String {
      "".to_str()
    }
  }
}