# rust-azure-blob-storage-client

Currently houses a crude attempt at an Azure Blob Storage client.

## Building

(Uses [cargo](http://crates.io).)

```shell
git clone https://github.com/colemickens/igneous
cargo build
```

## Running

```shell
./bin/igneous
```

## Testing

```shell
cargo test
```

## Current Output

```
Request
=======

URL: https://camlidev.blob.core.windows.net/camlidev-test-1?restype=container
Remote address: Some(23.99.32.78:443)
Method: GET
Headers:
 - Date: Mon, 21 Jul 2014 09:11:14 GMT
 - Authorization: SharedKey camlidev:vT8HH86a/XZDQMy2ZxHJNeA8kKBrXgxAjRxdrYiu2JU=
 - Host: camlidev.blob.core.windows.net
 - x-ms-version: 2009-09-19

Response
========

Status: 505 The HTTP version specified is not supported for this operation by the server.
Headers:
 - Connection: close
 - Date: Mon, 21 Jul 2014 09:11:13 GMT
 - Server: Windows-Azure-Blob/1.0 Microsoft-HTTPAPI/2.0
 - Content-Length: 297
 - Content-Type: application/xml
 - Keep-Alive: true
 - X-Ms-Request-Id: 3c6e181f-c9fb-47d4-8602-97efa1d8eea2
 - X-Ms-Version: 2009-09-19
Body:
<?xml version="1.0" encoding="utf-8"?><Error><Code>UnsupportedHttpVersion</Code><Message>The HTTP version specified is not supported for this operation by the server.
RequestId:3c6e181f-c9fb-47d4-8602-97efa1d8eea2
Time:2014-07-21T09:11:13.6065834Z</Message><Version>1.0</Version><Via /></Error>
```