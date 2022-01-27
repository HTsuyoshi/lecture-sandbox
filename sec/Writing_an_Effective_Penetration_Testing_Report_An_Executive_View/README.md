# High-Level Security Assessment

## Vulnerability Assesment (VA) or Vulnerability Scanning

Objective: Identify weadknesses or vulnerabilities

Use: Automated systems (`nessus`, `eEye Retina` or `QualisysGuard`)

## Network Security Assesment (VA) or Vulnerability Scanning

Objective: Identify weadknesses or vulnerabilities

Use: Combination of automated systems and hands-on manual vulnerability identification and testing

## Penetration testing

Objective: Identify weadknesses or vulnerabilities

Use: Multiple attack vectors (`wireless testing`, `social engineering`, `client-side testing`, `war dialing`) to compromise the target environment

Methods:

* black-box (with no prior knowlege)
* white-box (with full knowlege)
* grey-box (with some knowlege)

## Onsite auditing

Provides the cleares picture of network security. Explore and identify anything untoward, including rootkits, backdoors, Trojans, weak passwords, weak permissions or policies, misconfigurations, and other issues

## Best pratices

* Network reconnaissance to identify networks or hosts
* Bulk network scannin and probing to identify potentially vulnerable hosts
* Vulnerability identification and investigation and futher probing (manually)
* Exploitation of vulnerabilities and circumvention of security mechanisms

### Tools

Knowing the capabilities and features of the tools is the key to successful security assessment.

Start by evaluating Open Source and commercial ones in terms of functions, features, and deliverables. (Do not waste your budget to purchase some commercial tools which you don't really want to use due to the lack of capabilities and features. Test the tools before buying them)

(Most of the tools show are avaliable on `BackTrack/Kali Linux` as well as `BackBox Linux` penetration testing distribution)

#### Network reconnaince & scanning

* `Nmap` or `ZenMap` (Open source)
* `Hping` (Open source)
* `NetDiscover` (Open source)
* `NBTStat` (Open source)

#### Vulnerability identification & investigation

* `Nmap` with `NSE` (Open source)
* `Nessus` (Commercial)
* `eEye Retina` (Commercial)
* `QualisysGuard` (Commercial)
* `OpenVAS` (Open source)

#### Exploitation of vulnerabilities

* `Metasploit Framework` (Open source)
* `ExploitPack` (Open source)
* `Core Impact` (Commercial)
* `Metasploit` Express and Pro (Commercial)
* `Immunity CANVAS` (Open source)

### Methodologies

* NIST SP 800-115, Technical Guide to Information Security Testing and Assessment
* OISSG ISSAF, Information Systems Security Assessment Framework
* ISECOM OSSTMM, Open Source Security Testing Methodology Manual
* OWASP Testing Guide, Open Web Application Security Project
* SANS Institute, Conducting a Penetration Test on a Organization
* PTES, Penetration Testing Execution Standard

## Penetration testing
