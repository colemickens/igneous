#![crate_id = "azure#0"]
#![desc = "Azure BlobStorage"]
#![license = "BSD"]
#![crate_type = "lib"]

#![feature(globs)]

extern crate time;
extern crate http;
extern crate rustc;

pub mod blobstorage {
  use std::io::IoResult;
  use time;
  use rustc::util::sha2::Sha256;
  use rustc::util::sha2::Digest;
  use http::client::RequestWriter;
  use http::headers::content_type::*;
  use http::headers::request::ExtensionHeader;
  use http::method::Post;

  use http::headers::test_utils::to_stream_into_str;
  
  pub struct BlobStorageConnection {
    accountName: String
  }
  
  impl BlobStorageConnection {
    pub fn newPutBlobRequest(&self, containerName: &str, blobName: &str) -> IoResult<RequestWriter> {
      let url = format!("https://{}.blob.core.windows.net/{}/{}", self.accountName, containerName, blobName);

      RequestWriter::new(
          Post,
          from_str(url.as_slice()).expect("Invalid URL :-(")
      )
    }

    pub fn signRequest(&self, rw: &mut RequestWriter) {
      let bs = || "".to_str();
      //let ref hdrs = rw.headers;
      let hdrs = &rw.headers;

      let strToSign = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        "POST".to_str(), // TODO(colemickens): fix
        hdrs.content_encoding.map_or(bs(), |ref ce| ce.clone()),
        hdrs.content_language.unwrap_or(bs()),
        hdrs.content_length.unwrap_or(0),
        hdrs.content_md5.unwrap_or(bs()),
        hdrs.content_type.map_or(bs(), |ref ct| to_stream_into_str::<MediaType>(ct)),
        //to_stream_into_str(&hdrs.date.unwrap_or(time::now())),
        hdrs.date.unwrap_or(time::now()),
        hdrs.if_modified_since.unwrap_or(time::now()),
        hdrs.if_match.unwrap_or(bs()),
        hdrs.if_none_match.unwrap_or(bs()),
        hdrs.if_unmodified_since.unwrap_or(time::now()),
        hdrs.range.unwrap_or(bs()),
        self.canonicalizedHeaders(rw),
        self.canonicalizedResource(rw),
      );

      let _ = hdrs;

      let mut s2 = Sha256::new();
      s2.input_str(strToSign.as_slice());

      rw.headers.insert(ExtensionHeader("Authorization".to_str(), format!("Shared-Key: {}", s2.result_str())));
    }

    pub fn canonicalizedHeaders(&self, rw: &RequestWriter) -> String {
      "".to_str()
    }

    pub fn canonicalizedResource(&self, rw: &RequestWriter) -> String {
      "".to_str()
    }
  }
}