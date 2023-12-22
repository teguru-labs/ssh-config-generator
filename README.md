# generate-ssh-config

```bash
generate-ssh-config -u cloud_user \
    -i ~/.ssh/kimnguyen.ict \
    -h 192.168.48.1 \
    -h 192.168.48.2 \
    -h 192.168.48.3
```

Output:

```bash
# generated via `generate-ssh-config`:
Host node1
  Hostname 192.168.48.1
  User cloud_user
  IdentityFile ~/.ssh/kimnguyen.ict
Host node2
  Hostname 192.168.48.2
  User cloud_user
  IdentityFile ~/.ssh/kimnguyen.ict
Host node3
  Hostname 192.168.48.3
  User cloud_user
  IdentityFile ~/.ssh/kimnguyen.ict
```
