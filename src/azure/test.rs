extern crate azure;
extern crate http;
extern crate time;

use azure::blobstorage;
use http::headers::test_utils::to_stream_into_str;
use http::method;

#[test]
fn test_list_blobs() {
  let client = blobstorage::new_client("sampleAccount", "secretKey");
  let mut req = client.new_list_blob_req("samplecontainer");
  
  /*
  let mut req_date: time::Tm;
  req_date.tm_year = 2013-1900;
  req_date.tm_mon = 10;
  req_date.tm_mday = 2;
  req_date.tm_hour = 15;
  */
  
  let req_date = time::Tm{
    tm_sec: 00,
    tm_min: 00,
    tm_hour: 15,
    tm_mday: 2,
    tm_mon: 10,
    tm_year: 2013-1900, // lolwut
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_nsec: 0,
  };
  {
    // the following block is just stuff to fake it until we get the right signature
    // aka, I'm too lazy to generate a good signature to check, so I'm using the one known
    // good one and making my request match it first

    req.method = method::Put;

    req.headers.date = None;
    req.headers.content_length = Some(0);
    req.headers.extensions.insert("x-ms-date".to_str(), to_stream_into_str(&req_date));
  }
  
  client.sign_request(&mut req);

  assert_eq!(req.url.scheme, "https".to_str());
  assert_eq!(req.url.host, "sampleAccount.blob.core.windows.net".to_str())
  assert_eq!(req.url.path, "/samplecontainer".to_str())
  // TODO(colemickens): check query strings: { restype=container; comp=list; }

  /*
  assert_eq!(
    to_stream_into_str(&req.headers.date.expect("failed to get date")),
    "Sat, 02 Nov 2013 15:00:00 GMT".to_str()
  );
  */
  assert_eq!(req.headers.extensions.find(&"x-ms-version".to_str()), Some(&"2009-09-19".to_str()))
  assert_eq!(req.headers.authorization, Some("SharedKey sampleAccount:h0VRxbQipkWe0762ni41UQrKqV5h/j5gMlJDjb0tvys=".to_str()))
}