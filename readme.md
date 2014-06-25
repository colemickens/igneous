Igneous
=======

Currently houses a crude attempt at an Azure Blob Storage client.


You'll need to get a copy of rust-http and rust-openssl into the third_party/ directory. This will do it:

```shell
make prepare
```


To run tests cleanly:

```shell
make clean
make deps
make runtest
```


To run the example app cleanly:

```shell
make clean
make deps
make run
```

This is the current output:

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