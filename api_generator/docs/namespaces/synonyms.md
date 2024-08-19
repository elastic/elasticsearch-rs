Synonyms APIs

The synonyms management API provides a convenient way to define and manage synonyms in an internal system index. Related synonyms can be grouped in a "synonyms set". Create as many synonym sets as you need.

This provides an alternative to:

* Defining inline synonyms in an analyzer definition, which impacts mapping size and can lead to performance issues.
* Using synonyms files, which implies uploading and managing file consistency on all cluster nodes.

Synonyms sets can be used to configure synonym graph token filters and synonym token filters. These filters are applied as part of the analysis process by the search analyzer.

