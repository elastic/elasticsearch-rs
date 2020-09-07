Cross-cluster Replication APIs

[Enable replication of indices in remote clusters to a local cluster](https://www.elastic.co/guide/en/elasticsearch/reference/master/xpack-ccr.html).
This functionality can be used in some common production use cases:

- Disaster recovery in case a primary cluster fails. A secondary cluster can serve as a hot backup
- Geo-proximity so that reads can be served locally
