Index Lifecycle Management APIs

Automate how [indices are managed over time](https://www.elastic.co/guide/en/elasticsearch/reference/master/index-lifecycle-management.html).
Rather than simply performing management actions on indices on a set schedule, actions can be based
on other factors such as shard size and performance requirements.

Control how indices are handled as they age by attaching a lifecycle policy to the index
template used to create them. Update the policy to modify the lifecycle of both new
and existing indices.
