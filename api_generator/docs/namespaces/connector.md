Connector APIs

The connector and sync jobs APIs provide a convenient way to create and manage [Elastic connectors](https://www.elastic.co/guide/en/enterprise-search/master/connectors.html) and sync jobs in an internal index. To get started with Connector APIs, check out the tutorial.

Connectors are Elasticsearch integrations that bring content from third-party data sources, which can be deployed on Elastic Cloud or hosted on your own infrastructure:

* Native connectors are a managed service on Elastic Cloud
* Connector clients are self-managed on your infrastructure

This API provides an alternative to relying solely on Kibana UI for connector and sync job management. The API comes with a set of validations and assertions to ensure that the state representation in the internal index remains valid.
