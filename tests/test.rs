extern crate azure;
extern crate http;
extern crate time;

use azure::blobstorage;
use http::headers::test_utils::to_stream_into_str;
use http::method;

// example values from here:
// http://msdn.microsoft.com/en-us/library/azure/hh225339.aspx

#[test]
fn test_list_blobs() {
    let client = blobstorage::new_client("myaccount", "SzlFqgzqhfdk594cFoveYqCyvl8v9EESAnOLcTCeBIo31p46rJJRZx/5vU/oY3ZsK/VdFNaVpm6G8YSD2K48Nw==");
    let mut req = client.new_list_blob_req("mycontainer");
        
    let req_date = time::Tm{
        tm_sec: 00,
        tm_min: 00,
        tm_hour: 00,
        tm_mday: 20,
        tm_mon: 3,
        tm_year: 2014-1900, // lolwut
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

        // TODO(colemick): Fix this now that we know we're signing correctly.
        // Ensure the date strat is okay throughout
        req.method = method::Put;

        req.headers.date = None;
        req.headers.content_length = Some(0);
        req.headers.extensions.insert("x-ms-date".to_str(), to_stream_into_str(&req_date));
    }
    
    client.sign_request(&mut req);

    assert_eq!(req.url.scheme, "https".to_str());
    assert_eq!(req.url.host, "myaccount.blob.core.windows.net".to_str())
    assert_eq!(req.url.path, "/mycontainer".to_str())
    // TODO(colemickens): check query strings: { restype=container; comp=list; }

    //assert_eq!(
    //  to_stream_into_str(&req.headers.date.expect("failed to get date")),
    //  "Sun, 20 Apr 2014 00:00:00 GMT".to_str()
    //);
    
    assert_eq!(
        req.headers.extensions.find(&"x-ms-date".to_str()).expect("failed to get date"),
        &"Sun, 20 Apr 2014 00:00:00 GMT".to_str()
    );
    assert_eq!(req.headers.extensions.find(&"x-ms-version".to_str()), Some(&"2009-09-19".to_str()))
    assert_eq!(req.headers.authorization, Some("SharedKey myaccount:+AecMf7mGS/AlP0Vw3GIj4YDe51vkrBCZg1bM0mhNTU=".to_str()))
}