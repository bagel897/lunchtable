# Query

```{graphql}
query get_status {
  getStatus(user: "e3e18c67-e0ca-40c6-9c62-3289db55677d") {
    __typename
  }
}

mutation set_status($kind: StatusKind!, $reason: Reason!, $duration: Duration!) {
  setStatus(user: "e3e18c67-e0ca-40c6-9c62-3289db55677d", kind: $kind, reason: $reason, duration: $duration) {
    duration
  }
}
```

# Variables

```{graphql}
{
  "kind": "FREE",
  "reason": "MANUAL",
  "duration": "2016-01-01T13:10:20Z"
}
```
