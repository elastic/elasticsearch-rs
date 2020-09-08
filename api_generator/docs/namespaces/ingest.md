Ingest APIs

[Manage ingest pipelines](https://www.elastic.co/guide/en/elasticsearch/reference/master/ingest-apis.html).
Ingest pipelines can be used on a node with the `ingest` role to
pre-process documents before indexing, to apply transformations and enrich data. Transformations are performed
by [processors](https://www.elastic.co/guide/en/elasticsearch/reference/master/ingest-processors.html)
in the pipeline, and can include such operations as

- add, remove and append fields within the document
- point documents to the right time-based index based on a timestamp within the document
- extract details from fields with known formats and add new fields with extracted data

and many more.

All nodes enable ingest by default, so any node can handle ingest tasks. Ingest pipelines can
be conditionally executed, and failures within pipelines can be explicitly handled by defining
processors to execute in the event of failure.
