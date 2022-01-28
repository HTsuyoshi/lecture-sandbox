# What is hacking?

# Targeted Attackers

# Hacking cycle

* `Ressoinance`: Gather information about the target (Finding IP, Network, DNS records, etc...)
* `Scanning`: Use the information gathered and use tools (dialers, port scanners, network mappers, sweepers, vulnerability scanners, ...)
* `Gaining Access`: Design the target's network and use the data obtained in the Phase 1 and 2. This process is where we exploit the vulnerabilities
* `Maintaining Access`: Create backdoors to retain their access and do Post-Exploitations
* `Clearing Tracks`: Once they have access they need to clear their tracks and remove the evidence avoiding detection and legal actions

# So what is Privilege Escalation?

A `Privilege Escalation` is when a normal user gains access by impersonating the user to another user's account. Privilege escalations occur when a user tricks a system to grant permissions that are higher than those expected to be given to a typical user account by application developers or IT administrator. In any case, it is done with malicious intent to intensify.

## Types of Privilege Escalation

There are two types of Privilege Escalation:

* `Vertical`: This type of attack occur when an attacker adquires direct access to an account with the purpose of serving as that user (`Administrator`). The main aim is to an account to further spread an attack or access data.
* `Horizontal`: This attack allows the attacker to access the account credentials as well as elevate the permissions.

## Sniffing

Sometimes credentials are passed using `unencrypted protocols`. There are even offensive sniffers that will only target passwords for common protocols.

A such common tool is `dsniff`

### dsniff
You just need to be in the path of the traffic.
```sh
dsniff
```

Now just wait for the credentials to start coming in. You will see the `username` and the `password` coming along with the server and protocol the credential is for.

[Windows Privilege Escalation](./Windows_Privilege_Escalation.md)

[Linux Privilege Escalation](./Linux_Privilege_Escalation.md)
