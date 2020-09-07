Dangling Index APIs

If Elasticsearch encounters index data that is absent from the current cluster state,
those indices are considered to be _dangling_. For example, this can happen if you delete
more than `cluster.indices.tombstones.size` number of indices while an Elasticsearch node
is offline.

The dangling indices APIs can list, import and delete dangling indices.
