# Igneous

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

(I don't understand why this isn't giving the same http ver error though)

```
Request
=======

URL: https://camlidev.blob.core.windows.net/camlidev-test-1?restype=container
Remote address: Some(23.99.32.78:443)
Method: GET
Headers:
 - Date: Mon, 21 Jul 2014 08:50:31 GMT
 - Authorization: SharedKey camlidev:LoKKnXaH1WapaTeIzt9tmPlHkXYwniUZhrpuIkqkCMg=
 - Host: camlidev.blob.core.windows.net
 - x-ms-version: 2009-09-19

Response
========

Bad header encountered. TODO: handle this better.
Status: 200 OK
Headers:
 - Date: Mon, 21 Jul 2014 08:50:30 GMT
 - Transfer-Encoding: chunked
 - Server: Windows-Azure-Blob/1.0 Microsoft-HTTPAPI/2.0
 - Last-Modified: Sun, 08 Jun 2014 00:16:03 GMT
 - X-Ms-Request-Id: 30e6712d-7e86-40d3-9c8b-4095f0e792de
 - X-Ms-Version: 2009-09-19
Body:
^C
```

## Old-ish Output

```
Request
=======

URL: https://camlidev.blob.core.windows.net/camlidev-test-1?restype=container
Remote address: Some(23.99.32.78:443)
Method: GET
Headers:
 - Date: Wed, 25 Jun 2014 08:55:58 GMT
 - Authorization: SharedKey camlidev:dZq/IkFUQecZpmNuQGTAbBkoDZA1cSR9yPuCS6KIQM8=
 - Host: camlidev.blob.core.windows.net
 - x-ms-version: 2009-09-19

Response
========

Status: 505 The HTTP version specified is not supported for this operation by the server.
Headers:
 - Connection: close
 - Date: Wed, 25 Jun 2014 08:55:57 GMT
 - Server: Windows-Azure-Blob/1.0 Microsoft-HTTPAPI/2.0
 - Content-Length: 297
 - Content-Type: application/xml
 - Keep-Alive: true
 - X-Ms-Request-Id: 44416623-57a8-4c59-ad4a-89913c015387
 - X-Ms-Version: 2009-09-19
Body:
<?xml version="1.0" encoding="utf-8"?>
<Error>
  <Code>UnsupportedHttpVersion</Code>
  <Message>
    The HTTP version specified is not supported for this operation by the server.
    RequestId:44416623-57a8-4c59-ad4a-89913c015387
    Time:2014-06-25T08:55:57.6240970Z
  </Message>
  <Version>1.0</Version>
  <Via />
</Error>
```